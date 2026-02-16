use anyhow::{Result, anyhow};
use rayon::prelude::*;
use serde_json::{Value, json};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::build_order::{acquisition_level_map, optimize_build_order};
use crate::cache::{BlockingScoreCache, PersistentScoreCache};
use crate::engine::{
    ControlledChampionCombatSimulation, EnemyDerivedCombatStats, derive_enemy_combat_stats,
    simulate_controlled_champion_combat,
};
use crate::reporting::{
    write_controlled_champion_report_json, write_controlled_champion_report_markdown,
};
use crate::search::{
    adaptive_strategy_candidates, build_search_ranked, choose_best_build_by_stat,
    compute_build_metrics, generate_bleed_candidates, item_names, pareto_front_keys,
    portfolio_strategy_list, search_strategy_summary, select_diverse_top_builds,
    strategy_seed_elites,
};
use crate::status::{StatusReporter, deadline_reached};

use super::*;

fn format_repo_relative_path(path: &Path) -> String {
    if !path.is_absolute() {
        return path.display().to_string();
    }
    let repository_root = simulation_dir()
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(simulation_dir);
    path.strip_prefix(&repository_root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn parse_controlled_champion_config(
    scenario: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
) -> Result<(ChampionBase, Vec<String>, LoadoutSelection)> {
    let controlled_champion = scenario
        .get("controlled_champion")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("Missing controlled_champion object"))?;
    let champion_name = controlled_champion
        .get("champion")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Missing controlled_champion.champion"))?;
    let baseline_items = controlled_champion
        .get("baseline_items")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing controlled_champion.baseline_items"))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or_else(|| anyhow!("controlled_champion.baseline_items must be strings"))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let loadout_selection = parse_loadout_selection(controlled_champion.get("loadout"));
    let champion_base = lookup_champion_base(champion_bases, champion_name)?;
    Ok((champion_base, baseline_items, loadout_selection))
}

fn parse_opponent_encounters(
    scenario: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
) -> Result<Vec<(String, f64, Vec<EnemyConfig>)>> {
    let opponents = scenario
        .get("opponents")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("Missing opponents object"))?;
    let encounters = opponents
        .get("encounters")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing opponents.encounters"))?;
    if encounters.is_empty() {
        return Err(anyhow!(
            "opponents.encounters must include at least one encounter"
        ));
    }
    let mut parsed = Vec::with_capacity(encounters.len());
    for (index, encounter) in encounters.iter().enumerate() {
        let name = encounter
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Missing opponents.encounters[{index}].name"))?;
        let weight = encounter
            .get("weight")
            .and_then(Value::as_f64)
            .unwrap_or(1.0);
        if weight < 0.0 {
            return Err(anyhow!(
                "opponents.encounters[{index}].weight must be >= 0.0"
            ));
        }
        let actors = encounter
            .get("actors")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing opponents.encounters[{index}].actors"))?;
        if actors.is_empty() {
            return Err(anyhow!(
                "opponents.encounters[{index}].actors must include at least one actor"
            ));
        }
        let parsed_actors = actors
            .iter()
            .map(|actor| parse_enemy_config(actor, champion_bases))
            .collect::<Result<Vec<_>>>()?;
        parsed.push((name.to_string(), weight, parsed_actors));
    }
    Ok(parsed)
}

fn parse_opponent_shared_loadout_selection(scenario: &Value) -> LoadoutSelection {
    parse_loadout_selection(
        scenario
            .get("opponents")
            .and_then(Value::as_object)
            .and_then(|opponents| opponents.get("shared_loadout")),
    )
}

fn search_quality_profile_key(search_quality_profile: SearchQualityProfile) -> &'static str {
    match search_quality_profile {
        SearchQualityProfile::Fast => "fast",
        SearchQualityProfile::Balanced => "balanced",
        SearchQualityProfile::MaximumQuality => "maximum_quality",
    }
}

fn runtime_budget_key(max_runtime_seconds: Option<f64>) -> String {
    match max_runtime_seconds {
        Some(seconds) if seconds > 0.0 => {
            let rounded = seconds.round();
            if (seconds - rounded).abs() < 1e-9 {
                format!("{rounded:.0}s")
            } else {
                format!("{seconds:.1}s")
            }
        }
        _ => "unbounded".to_string(),
    }
}

fn default_run_output_directory(
    search_quality_profile: SearchQualityProfile,
    max_runtime_seconds: Option<f64>,
) -> PathBuf {
    simulation_dir()
        .join("output")
        .join("runs")
        .join("controlled_champion")
        .join(search_quality_profile_key(search_quality_profile))
        .join(runtime_budget_key(max_runtime_seconds))
}

pub(super) fn run_controlled_champion_scenario(
    scenario_path: &Path,
    options: &ControlledChampionRunOptions<'_>,
) -> Result<()> {
    let top_x = options.top_x;
    let min_item_diff = options.min_item_diff;
    let max_relative_gap_percent = options.max_relative_gap_percent;
    let report_path_override = options.report_path_override;
    let max_runtime_seconds = options.max_runtime_seconds;
    let status_every_seconds = options.status_every_seconds;
    let search_quality_profile = options.search_quality_profile;

    let run_start = Instant::now();
    let time_budget = max_runtime_seconds
        .filter(|s| *s > 0.0)
        .map(Duration::from_secs_f64);
    let deadline = time_budget.map(|d| run_start + d);
    let status_every = Duration::from_secs_f64(status_every_seconds.max(1.0));
    let mut status = StatusReporter::new(run_start, status_every);
    let timeout_flag = Arc::new(AtomicUsize::new(0));
    status.emit("initialization", None, None, Some("starting"), true);
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;
    status.emit("initialization", None, None, Some("core data loaded"), true);

    let sim = parse_simulation_config(
        scenario
            .get("simulation")
            .ok_or_else(|| anyhow!("Missing simulation"))?,
    )?;
    if deadline_reached(deadline) {
        timeout_flag.store(1, AtomicOrdering::Relaxed);
    }

    let (vlad_base_raw, baseline_fixed_names, vlad_loadout_selection) =
        parse_controlled_champion_config(&scenario, &champion_bases)?;
    let vlad_base = champion_at_level(&vlad_base_raw, sim.champion_level);
    let controlled_champion_name = vlad_base_raw.name.clone();
    let enemy_loadout_selection = parse_opponent_shared_loadout_selection(&scenario);

    let enemy_scenarios_raw = parse_opponent_encounters(&scenario, &champion_bases)?;
    let primary_enemy_raw = enemy_scenarios_raw
        .first()
        .map(|(_, _, v)| v.clone())
        .unwrap_or_default();
    let raw_enemy_bases = primary_enemy_raw
        .iter()
        .map(|e| (e.id.clone(), e.base.clone()))
        .collect::<HashMap<_, _>>();
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|(name, weight, enemies)| {
            let scaled = enemies
                .iter()
                .cloned()
                .map(|mut e| {
                    e.base = champion_at_level(&e.base, sim.champion_level);
                    e
                })
                .collect::<Vec<_>>();
            (name.clone(), *weight, scaled)
        })
        .collect::<Vec<_>>();

    let mut search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    apply_search_quality_profile(&mut search_cfg, search_quality_profile);
    let loadout_domain = Arc::new(build_loadout_domain());
    let loadout_eval_budget = loadout_eval_budget(&search_cfg, search_quality_profile);
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain)?;
    let enemy_loadout = resolve_loadout(&enemy_loadout_selection, sim.champion_level, false)?;
    let max_items = search_cfg.max_items;
    let item_pool = default_item_pool(&items);
    status.emit(
        "configuration",
        None,
        None,
        Some("search profile and enemy presets ready"),
        true,
    );

    let baseline_fixed_build = item_pool_from_names(&items, &baseline_fixed_names)?;

    let mut enemy_presets_used: HashMap<String, EnemyUrfPreset> = HashMap::new();
    let mut enemy_build_scenarios = Vec::new();
    for (name, weight, enemies) in &enemy_scenarios {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(&items, &preset.item_names)?;
            let preset_enemy_loadout = resolve_loadout(
                &enemy_loadout_from_preset(preset),
                sim.champion_level,
                false,
            )?;
            let mut enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            enemy_bonus_stats.add(&enemy_loadout.bonus_stats);
            enemy_presets_used.insert(preset_key, preset.clone());
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            enemy_with_loadout.loadout_masteries = preset.masteries.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }
    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, b)| b.clone())
        .unwrap_or_default();
    let enemy_derived_combat_stats = enemy_builds
        .iter()
        .map(|(enemy, build, bonus_stats)| {
            derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf)
        })
        .collect::<Vec<_>>();
    let enemy_similarity_notes = build_enemy_similarity_notes(&enemy_derived_combat_stats);
    status.emit(
        "enemy_setup",
        None,
        None,
        Some("enemy preset builds loaded"),
        true,
    );

    let vlad_base_loadout = resolve_loadout(&vlad_loadout_selection, sim.champion_level, true)?;
    let resolve_cache: Arc<Mutex<HashMap<String, ResolvedLoadout>>> =
        Arc::new(Mutex::new(HashMap::from([(
            loadout_selection_key(&vlad_loadout_selection),
            vlad_base_loadout.clone(),
        )])));
    let best_loadout_by_item: Arc<Mutex<BestLoadoutMap>> = Arc::new(Mutex::new(HashMap::new()));
    let best_outcome_by_item: Arc<Mutex<BestOutcomeMap>> = Arc::new(Mutex::new(HashMap::new()));

    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            simulate_controlled_champion_combat(
                &vlad_base,
                &baseline_fixed_build,
                &vlad_base_loadout.bonus_stats,
                Some(&vlad_loadout_selection),
                None,
                enemy_builds_s,
                &sim,
                &urf,
            )
        })
        .collect::<Vec<_>>();
    let objective_eval_ctx = ObjectiveEvalContext {
        controlled_champion_base: &vlad_base,
        enemy_build_scenarios: &enemy_build_scenarios,
        sim: &sim,
        urf: &urf,
        scenario_reference_outcomes: &scenario_reference_outcomes,
        weights: objective_component_weights,
        worst_case_weight: objective_worst_case_weight,
    };
    let evaluate_build_with_bonus =
        |build_items: &[Item],
         bonus_stats: &Stats,
         loadout_selection: Option<&LoadoutSelection>| {
            aggregate_objective_score_and_outcome_with_loadout_selection(
                &objective_eval_ctx,
                build_items,
                bonus_stats,
                loadout_selection,
            )
        };
    let score_build_with_bonus =
        |build_items: &[Item],
         bonus_stats: &Stats,
         loadout_selection: Option<&LoadoutSelection>| {
            evaluate_build_with_bonus(build_items, bonus_stats, loadout_selection).0
        };

    let loadout_candidates_count = loadout_eval_budget;
    let loadout_finalists_count = 1usize;
    let optimize_loadout_for_build = |build_key: &[usize], build_items: &[Item]| {
        let mut hasher = DefaultHasher::new();
        build_key.hash(&mut hasher);
        let mut seed = search_cfg.seed ^ hasher.finish();
        let mut seen = HashSet::new();

        let mut best_sel = vlad_loadout_selection.clone();
        let mut best_resolved = vlad_base_loadout.clone();
        let (mut best_score, mut best_outcome) =
            evaluate_build_with_bonus(build_items, &best_resolved.bonus_stats, Some(&best_sel));
        seen.insert(loadout_selection_key(&best_sel));

        let mut evaluated = 0usize;
        while evaluated < loadout_eval_budget {
            if deadline_reached(deadline) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                break;
            }
            let candidate =
                random_loadout_selection(&vlad_loadout_selection, &loadout_domain, &mut seed);
            let key = loadout_selection_key(&candidate);
            if !seen.insert(key.clone()) {
                continue;
            }

            let resolved = if let Ok(map) = resolve_cache.lock() {
                map.get(&key).cloned()
            } else {
                None
            }
            .or_else(|| {
                resolve_loadout(&candidate, sim.champion_level, true)
                    .ok()
                    .inspect(|resolved| {
                        if let Ok(mut map) = resolve_cache.lock() {
                            map.insert(key.clone(), resolved.clone());
                        }
                    })
            });

            let Some(resolved) = resolved else {
                continue;
            };
            let (score, outcome) =
                evaluate_build_with_bonus(build_items, &resolved.bonus_stats, Some(&candidate));
            if score > best_score {
                best_score = score;
                best_sel = candidate;
                best_resolved = resolved;
                best_outcome = outcome;
            }
            evaluated += 1;
        }
        (best_score, best_outcome, best_sel, best_resolved)
    };

    let full_eval_count = AtomicUsize::new(0);
    let full_cache = Arc::new(BlockingScoreCache::new());
    let mut scenario_hasher = DefaultHasher::new();
    scenario.to_string().hash(&mut scenario_hasher);
    search_strategy_summary(&search_cfg).hash(&mut scenario_hasher);
    search_cfg.seed.hash(&mut scenario_hasher);
    loadout_eval_budget.hash(&mut scenario_hasher);
    let persistent_full_cache_path = simulation_dir().join("output").join("cache").join(format!(
        "{}_full_scores_{:016x}.json",
        to_norm_key(&controlled_champion_name),
        scenario_hasher.finish()
    ));
    let persistent_full_cache = Arc::new(PersistentScoreCache::load(persistent_full_cache_path));
    let full_score_fn = |build_idx: &[usize]| {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            return f64::NEG_INFINITY;
        }
        let key = canonical_key(build_idx);
        if let Some(score) = persistent_full_cache.get(&key) {
            return score;
        }
        let cache = Arc::clone(&full_cache);
        cache.get_or_compute(key.clone(), || {
            if deadline_reached(deadline) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            if let Some(score) = persistent_full_cache.get(&key) {
                return score;
            }
            full_eval_count.fetch_add(1, AtomicOrdering::Relaxed);
            let build_items = build_from_indices(&item_pool, &key);
            let (score, outcome, best_sel, best_resolved) =
                optimize_loadout_for_build(&key, &build_items);
            if let Ok(mut map) = best_loadout_by_item.lock() {
                map.insert(key.clone(), (best_sel, best_resolved));
            }
            if let Ok(mut map) = best_outcome_by_item.lock() {
                map.insert(key.clone(), outcome);
            }
            if score.is_finite() {
                persistent_full_cache.insert(&key, score);
            }
            score
        })
    };

    let ensemble_seeds = search_cfg.ensemble_seeds.max(1);
    let active_strategies = portfolio_strategy_list(&search_cfg);
    status.emit(
        "seed_search",
        Some((0, ensemble_seeds)),
        None,
        Some("running ensemble seeds"),
        true,
    );
    let mut seed_ranked = Vec::new();
    for seed_idx in 0..ensemble_seeds {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        seed_ranked.push({
            let mut cfg = search_cfg.clone();
            cfg.seed = search_cfg.seed.wrapping_add(
                search_cfg
                    .ensemble_seed_stride
                    .wrapping_mul(seed_idx as u64),
            );
            cfg.ranked_limit = cfg.ranked_limit.max(64);
            build_search_ranked(&item_pool, max_items, &cfg, &full_score_fn, deadline)
        });
        status.emit(
            "seed_search",
            Some((seed_idx + 1, ensemble_seeds)),
            None,
            None,
            false,
        );
    }
    let strategy_elites = strategy_seed_elites(
        &item_pool,
        max_items,
        &search_cfg,
        &active_strategies,
        &full_score_fn,
        deadline,
    );
    let adaptive_candidates = adaptive_strategy_candidates(
        &item_pool,
        max_items,
        &search_cfg,
        &strategy_elites,
        &full_score_fn,
        deadline,
    );
    let bleed_candidates =
        generate_bleed_candidates(&item_pool, max_items, &strategy_elites, &search_cfg);
    status.emit(
        "candidate_merge",
        None,
        None,
        Some("merging strict candidates"),
        true,
    );
    let bleed_candidate_count = bleed_candidates.len();
    let adaptive_candidate_count = adaptive_candidates.len();

    let mut candidate_keys = Vec::new();
    let mut seed_top_sets = Vec::new();
    for ranked in &seed_ranked {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let seed_top = ranked
            .iter()
            .take(search_cfg.ensemble_seed_top_k.max(1))
            .map(|(k, _)| k.clone())
            .collect::<HashSet<_>>();
        seed_top_sets.push(seed_top);
        for (k, _) in ranked {
            candidate_keys.push(k.clone());
        }
    }
    for k in bleed_candidates {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        candidate_keys.push(k);
    }
    for k in adaptive_candidates {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        candidate_keys.push(k);
    }
    let candidate_keys_generated = candidate_keys.len();
    let mut unique_candidate_keys = candidate_keys
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_candidate_keys.sort_unstable();
    if unique_candidate_keys.is_empty() {
        let baseline_key = canonical_key(
            &baseline_fixed_build
                .iter()
                .filter_map(|item| item_pool.iter().position(|p| p.name == item.name))
                .collect::<Vec<_>>(),
        );
        unique_candidate_keys.push(baseline_key);
    }
    let candidate_duplicates_pruned =
        candidate_keys_generated.saturating_sub(unique_candidate_keys.len());

    let mut strict_scores = HashMap::<BuildKey, f64>::new();
    for ranked in &seed_ranked {
        for (k, s) in ranked {
            if !s.is_finite() {
                continue;
            }
            let entry = strict_scores.entry(k.clone()).or_insert(*s);
            if *s > *entry {
                *entry = *s;
            }
        }
    }

    let total_candidates = unique_candidate_keys.len();
    let strict_seed_scored_candidates = strict_scores.len().min(total_candidates);
    let mut processed_keys = strict_scores.keys().cloned().collect::<HashSet<_>>();
    let mut processed_candidates = processed_keys.len().min(total_candidates);
    let mut timed_out = timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    status.emit(
        "strict_full_ranking",
        Some((processed_candidates, total_candidates)),
        strict_scores
            .values()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
        Some("evaluating all generated candidates"),
        true,
    );
    let remaining_keys = unique_candidate_keys
        .iter()
        .filter(|key| !processed_keys.contains(*key))
        .cloned()
        .collect::<Vec<_>>();
    let strict_remaining_candidates = remaining_keys.len();
    let mut strict_non_finite_candidates = 0usize;
    let batch_size = 128usize;
    for batch in remaining_keys.chunks(batch_size) {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            timed_out = true;
            break;
        }
        let scored_batch = batch
            .par_iter()
            .map(|key| (key.clone(), full_score_fn(key)))
            .collect::<Vec<_>>();
        for (key, score) in scored_batch {
            if score.is_finite() {
                strict_scores.insert(key.clone(), score);
            } else {
                strict_non_finite_candidates += 1;
            }
            processed_keys.insert(key);
            processed_candidates += 1;
            status.emit(
                "strict_full_ranking",
                Some((processed_candidates, total_candidates)),
                strict_scores
                    .values()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
                None,
                false,
            );
        }
    }

    if strict_scores.is_empty() {
        let baseline_key = canonical_key(
            &baseline_fixed_build
                .iter()
                .filter_map(|item| item_pool.iter().position(|p| p.name == item.name))
                .collect::<Vec<_>>(),
        );
        let baseline_score = score_build_with_bonus(
            &baseline_fixed_build,
            &vlad_base_loadout.bonus_stats,
            Some(&vlad_loadout_selection),
        );
        strict_scores.insert(baseline_key, baseline_score);
    }

    let mut vlad_ranked = strict_scores.into_iter().collect::<Vec<_>>();
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    let strict_candidates_skipped_timeout =
        total_candidates.saturating_sub(processed_candidates.min(total_candidates));
    let strict_completion_percent = if total_candidates > 0 {
        100.0 * (processed_candidates.min(total_candidates) as f64) / (total_candidates as f64)
    } else {
        100.0
    };
    let outcome_map_for_tiebreak = best_outcome_by_item
        .lock()
        .map(|m| m.clone())
        .unwrap_or_default();
    vlad_ranked.sort_by(|a, b| {
        let by_score = b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal);
        if by_score != Ordering::Equal {
            return by_score;
        }
        let out_a = outcome_map_for_tiebreak.get(&a.0);
        let out_b = outcome_map_for_tiebreak.get(&b.0);
        let cap_a = out_a
            .map(|o| o.time_alive_seconds >= sim.max_time_seconds - 1e-6)
            .unwrap_or(false);
        let cap_b = out_b
            .map(|o| o.time_alive_seconds >= sim.max_time_seconds - 1e-6)
            .unwrap_or(false);
        if cap_a && cap_b {
            let combo_a = out_a
                .map(|o| {
                    objective_component_weights.damage * o.damage_dealt
                        + objective_component_weights.healing * o.healing_done
                })
                .unwrap_or(0.0);
            let combo_b = out_b
                .map(|o| {
                    objective_component_weights.damage * o.damage_dealt
                        + objective_component_weights.healing * o.healing_done
                })
                .unwrap_or(0.0);
            return combo_b.partial_cmp(&combo_a).unwrap_or(Ordering::Equal);
        }
        Ordering::Equal
    });

    let mut seed_best_scores = Vec::new();
    for ranked in &seed_ranked {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let best = ranked
            .iter()
            .take(search_cfg.ensemble_seed_top_k.max(1))
            .map(|(_, s)| *s)
            .fold(f64::NEG_INFINITY, |acc, v| acc.max(v));
        if best.is_finite() {
            seed_best_scores.push(best);
        }
    }

    let mut seed_hits_by_key: HashMap<BuildKey, usize> = HashMap::new();
    for top in &seed_top_sets {
        for key in top {
            *seed_hits_by_key.entry(key.clone()).or_insert(0) += 1;
        }
    }

    let vlad_best_indices = vlad_ranked
        .first()
        .map(|(build, _)| build.clone())
        .unwrap_or_default();
    let vlad_best_build = build_from_indices(&item_pool, &vlad_best_indices);
    let (vlad_runtime_loadout_selection, vlad_loadout) = best_loadout_by_item
        .lock()
        .ok()
        .and_then(|m| m.get(&vlad_best_indices).cloned())
        .unwrap_or_else(|| (vlad_loadout_selection.clone(), vlad_base_loadout.clone()));

    let baseline_fixed_indices = baseline_fixed_build
        .iter()
        .filter_map(|item| item_pool.iter().position(|p| p.name == item.name))
        .collect::<Vec<_>>();
    let baseline_fixed_score = if deadline_reached(deadline) {
        score_build_with_bonus(
            &baseline_fixed_build,
            &vlad_base_loadout.bonus_stats,
            Some(&vlad_loadout_selection),
        )
    } else {
        full_score_fn(&baseline_fixed_indices)
    };
    let baseline_fixed_key = canonical_key(&baseline_fixed_indices);
    let (baseline_runtime_loadout_selection, baseline_loadout) = best_loadout_by_item
        .lock()
        .ok()
        .and_then(|m| m.get(&baseline_fixed_key).cloned())
        .unwrap_or_else(|| (vlad_loadout_selection.clone(), vlad_base_loadout.clone()));
    let (_, baseline_fixed_outcome) = aggregate_objective_score_and_outcome_with_loadout_selection(
        &objective_eval_ctx,
        &baseline_fixed_build,
        &baseline_loadout.bonus_stats,
        Some(&baseline_runtime_loadout_selection),
    );
    let vlad_best_score = vlad_ranked.first().map(|(_, s)| *s).unwrap_or(0.0);
    let (_, vlad_best_outcome) = aggregate_objective_score_and_outcome_with_loadout_selection(
        &objective_eval_ctx,
        &vlad_best_build,
        &vlad_loadout.bonus_stats,
        Some(&vlad_runtime_loadout_selection),
    );
    let baseline_cap_survivor =
        baseline_fixed_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;
    let best_cap_survivor = vlad_best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;

    println!("Enemy builds (URF preset defaults):");
    for (enemy, build, _) in &enemy_builds {
        println!(
            "- {}: {}",
            enemy.name,
            build
                .iter()
                .map(|i| i.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );
        if let Some(preset) = enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            println!(
                "  source: {} (last checked {})",
                preset.source_url, preset.last_checked
            );
        }
    }
    println!("\nEnemy derived combat profiles:");
    for profile in &enemy_derived_combat_stats {
        println!(
            "- {}: HP {:.1}, Armor {:.1}, MR {:.1}, AD {:.1}, AS {:.3} (interval {:.3}s), range {:.0}, move speed {:.1}, hit physical {:.1}, hit ability {:.1}, burst phys/magic/true {:.1}/{:.1}/{:.1}",
            profile.champion,
            profile.max_health,
            profile.armor,
            profile.magic_resist,
            profile.attack_damage,
            profile.attack_speed,
            profile.attack_interval_seconds,
            profile.attack_range,
            profile.move_speed,
            profile.physical_hit_damage,
            profile.ability_hit_damage,
            profile.burst_physical_damage,
            profile.burst_magic_damage,
            profile.burst_true_damage
        );
    }
    for note in &enemy_similarity_notes {
        println!("- Warning: {}", note);
    }

    println!("\n{} baseline build (fixed):", controlled_champion_name);
    println!(
        "- Items: {}",
        baseline_fixed_build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Objective score: {:.4}", baseline_fixed_score);
    println!(
        "- Time alive / damage dealt / healing done / enemy kills: {:.2}s / {:.1} / {:.1} / {}",
        baseline_fixed_outcome.time_alive_seconds,
        baseline_fixed_outcome.damage_dealt,
        baseline_fixed_outcome.healing_done,
        baseline_fixed_outcome.enemy_kills
    );
    println!("- Cap survivor: {}", baseline_cap_survivor);

    println!(
        "\n{} best build (optimized for objective):",
        controlled_champion_name
    );
    println!(
        "- Search strategy: {}",
        search_strategy_summary(&search_cfg)
    );
    println!(
        "- Loadout candidates/finalists: {}/{}",
        loadout_candidates_count, loadout_finalists_count
    );
    println!(
        "- Candidate evaluations (full): {}",
        full_eval_count.load(AtomicOrdering::Relaxed)
    );
    println!("- Cache hits (full): {}", full_cache.hits());
    println!(
        "- Persistent full cache hits/entries: {}/{}",
        persistent_full_cache.hits(),
        persistent_full_cache.len()
    );
    println!("- Cache waits (full): {}", full_cache.waits());
    println!("- Ensemble seeds: {}", ensemble_seeds);
    println!(
        "- Enemy scenarios in objective: {}",
        enemy_build_scenarios.len()
    );
    println!(
        "- Objective weights (survival/damage/healing): {:.2}/{:.2}/{:.2}",
        objective_component_weights.survival,
        objective_component_weights.damage,
        objective_component_weights.healing
    );
    if let Some(budget) = time_budget {
        println!(
            "- Time budget: {:.1}s | elapsed: {:.1}s | timed_out: {} | progress: {}/{}",
            budget.as_secs_f64(),
            run_start.elapsed().as_secs_f64(),
            timed_out,
            processed_candidates,
            total_candidates
        );
    }
    println!(
        "- Unique strict candidates: {}",
        unique_candidate_keys.len()
    );
    println!(
        "- Candidate keys generated / duplicates pruned: {}/{}",
        candidate_keys_generated, candidate_duplicates_pruned
    );
    println!(
        "- Strict completion: {:.1}% (processed {}/{}, timeout-skipped {}, non-finite {})",
        strict_completion_percent,
        processed_candidates.min(total_candidates),
        total_candidates,
        strict_candidates_skipped_timeout,
        strict_non_finite_candidates
    );
    println!("- Bleed candidates injected: {}", bleed_candidate_count);
    println!(
        "- Adaptive candidates injected: {}",
        adaptive_candidate_count
    );
    println!(
        "- Items: {}",
        vlad_best_build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Objective score: {:.4}", vlad_best_score);
    println!(
        "- Time alive / damage dealt / healing done / enemy kills: {:.2}s / {:.1} / {:.1} / {}",
        vlad_best_outcome.time_alive_seconds,
        vlad_best_outcome.damage_dealt,
        vlad_best_outcome.healing_done,
        vlad_best_outcome.enemy_kills
    );
    println!("- Cap survivor: {}", best_cap_survivor);
    if !vlad_loadout.selection_labels.is_empty() {
        println!("\n{} runes/masteries:", controlled_champion_name);
        for s in &vlad_loadout.selection_labels {
            println!("- {}", s);
        }
    }
    if !enemy_loadout.selection_labels.is_empty() {
        println!("\nEnemy runes/masteries (applied to all enemies):");
        for s in &enemy_loadout.selection_labels {
            println!("- {}", s);
        }
    }

    let diverse_top_raw =
        select_diverse_top_builds(&vlad_ranked, top_x, min_item_diff, max_relative_gap_percent);
    let diverse_top_keys = diverse_top_raw
        .iter()
        .map(|(indices, _)| indices.clone())
        .collect::<Vec<_>>();
    let diverse_top_builds = diverse_top_raw
        .iter()
        .map(|(indices, score)| (build_from_indices(&item_pool, indices), *score))
        .collect::<Vec<_>>();
    let mut metrics_by_key = HashMap::new();
    for (key, score) in &vlad_ranked {
        metrics_by_key.insert(
            key.clone(),
            compute_build_metrics(
                key,
                &item_pool,
                &vlad_base,
                &vlad_loadout.bonus_stats,
                &sim,
                *score,
            ),
        );
    }
    let pareto_front = pareto_front_keys(&metrics_by_key);
    let build_confidence = vlad_ranked
        .iter()
        .map(|(key, _)| {
            let hits = seed_hits_by_key.get(key).copied().unwrap_or(0);
            let hit_rate = hits as f64 / ensemble_seeds as f64;
            let robustness = if hit_rate >= search_cfg.robust_min_seed_hit_rate {
                "robust".to_string()
            } else {
                "fragile".to_string()
            };
            BuildConfidence {
                key: key.clone(),
                seed_hits: hits,
                seed_hit_rate: hit_rate,
                robustness,
            }
        })
        .collect::<Vec<_>>();
    let diagnostics = SearchDiagnostics {
        strategy_summary: search_strategy_summary(&search_cfg),
        search_quality_profile: match search_quality_profile {
            SearchQualityProfile::Fast => "fast".to_string(),
            SearchQualityProfile::Balanced => "balanced".to_string(),
            SearchQualityProfile::MaximumQuality => "maximum_quality".to_string(),
        },
        ensemble_seeds,
        objective_survival_weight: objective_component_weights.survival,
        objective_damage_weight: objective_component_weights.damage,
        objective_healing_weight: objective_component_weights.healing,
        full_evaluations: full_eval_count.load(AtomicOrdering::Relaxed),
        full_cache_hits: full_cache.hits(),
        full_cache_misses: full_cache.misses(),
        full_cache_waits: full_cache.waits(),
        full_persistent_cache_hits: persistent_full_cache.hits(),
        full_persistent_cache_entries: persistent_full_cache.len(),
        candidate_keys_generated,
        candidate_duplicates_pruned,
        unique_candidate_builds: unique_candidate_keys.len(),
        bleed_candidates_injected: bleed_candidate_count,
        adaptive_candidates_injected: adaptive_candidate_count,
        scenario_count: enemy_build_scenarios.len(),
        loadout_candidates: loadout_candidates_count,
        loadout_finalists: loadout_finalists_count,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        time_budget_seconds: time_budget.map(|d| d.as_secs_f64()),
        elapsed_seconds: run_start.elapsed().as_secs_f64(),
        timed_out,
        processed_candidates,
        total_candidates,
        seed_best_scores,
    };
    let confidence_by_key = build_confidence
        .iter()
        .map(|c| (c.key.clone(), c.clone()))
        .collect::<HashMap<_, _>>();
    let mut order_input = diverse_top_builds
        .iter()
        .enumerate()
        .filter_map(|(idx, (build, _))| {
            let key = diverse_top_keys.get(idx)?;
            let robust = confidence_by_key
                .get(key)
                .map(|c| c.robustness == "robust")
                .unwrap_or(false);
            let pareto = pareto_front.contains(key);
            if robust || pareto {
                Some(build.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if order_input.is_empty() {
        order_input = diverse_top_builds
            .iter()
            .take(2)
            .map(|(b, _)| b.clone())
            .collect::<Vec<_>>();
    }
    let build_order_ctx = BuildOrderEvalContext {
        controlled_champion_base_raw: &vlad_base_raw,
        controlled_champion_bonus_stats: &vlad_loadout.bonus_stats,
        enemy_builds: &enemy_builds,
        raw_enemy_bases: &raw_enemy_bases,
        sim: &sim,
        urf: &urf,
        objective_weights: objective_component_weights,
    };
    let build_order_results = order_input
        .iter()
        .map(|build| optimize_build_order(build, &build_order_ctx))
        .collect::<Vec<_>>();
    let best_order_acquired_map = build_order_results
        .first()
        .map(|br| acquisition_level_map(&br.ordered_items, &br.acquired_levels));

    let best_effective_item_stats = compute_effective_item_stats_for_build(
        &vlad_base,
        &vlad_best_build,
        &vlad_loadout.bonus_stats,
        &sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
    );
    let vlad_end_stats = compute_champion_final_stats(&vlad_base, &best_effective_item_stats);
    let stack_notes = build_stack_notes(
        &vlad_best_build,
        &vlad_base,
        &sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
    );

    println!("\nTop diverse builds:");
    if diverse_top_builds.is_empty() {
        println!(
            "- None found (try increasing --max-relative-gap-percent or lowering --min-item-diff)."
        );
    } else {
        for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
            println!(
                "- #{:02} score {:.4}: {}",
                idx + 1,
                score,
                item_names(build)
            );
        }
    }
    if !build_order_results.is_empty() {
        println!("\nBuild order optimization (levels spread from 5 to 20):");
        for (idx, br) in build_order_results.iter().enumerate() {
            println!(
                "- Build #{:02} best order (cumulative {:.2}): {}",
                idx + 1,
                br.cumulative_score,
                item_names(&br.ordered_items)
            );
            for (stage_idx, lvl) in br.levels.iter().enumerate() {
                let surv = br.stage_survival.get(stage_idx).copied().unwrap_or(0.0);
                let dmg = br.stage_damage.get(stage_idx).copied().unwrap_or(0.0);
                let heal = br.stage_healing.get(stage_idx).copied().unwrap_or(0.0);
                let stage_objective = br
                    .stage_objective_scores
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                println!(
                    "  - Stage {} @ level {} -> objective {:.3} | time {:.2}s | damage {:.1} | healing {:.1}",
                    stage_idx + 1,
                    lvl,
                    stage_objective,
                    surv,
                    dmg,
                    heal
                );
            }
        }
    }

    let default_output_directory =
        default_run_output_directory(search_quality_profile, max_runtime_seconds);
    let report_path = report_path_override.map(PathBuf::from).unwrap_or_else(|| {
        default_output_directory.join(format!(
            "{}_run_report.md",
            to_norm_key(&controlled_champion_name)
        ))
    });
    let report_data = ControlledChampionReportData {
        scenario_path,
        controlled_champion_name: &controlled_champion_name,
        sim: &sim,
        controlled_champion_base_level: &vlad_base,
        controlled_champion_end_stats: &vlad_end_stats,
        stack_notes: &stack_notes,
        controlled_champion_loadout: &vlad_loadout,
        enemy_loadout: &enemy_loadout,
        baseline_build: &baseline_fixed_build,
        baseline_score: baseline_fixed_score,
        baseline_outcome: &baseline_fixed_outcome,
        best_build: &vlad_best_build,
        best_score: vlad_best_score,
        best_outcome: &vlad_best_outcome,
        enemy_builds: &enemy_builds,
        enemy_derived_combat_stats: &enemy_derived_combat_stats,
        enemy_similarity_notes: &enemy_similarity_notes,
        enemy_presets_used: &enemy_presets_used,
        diverse_top_builds: &diverse_top_builds,
        diverse_top_keys: &diverse_top_keys,
        build_confidence: &build_confidence,
        metrics_by_key: &metrics_by_key,
        pareto_front: &pareto_front,
        diagnostics: &diagnostics,
        build_orders: &build_order_results,
    };
    write_controlled_champion_report_markdown(&report_path, &report_data)?;
    let json_report_path = report_path.with_extension("json");
    write_controlled_champion_report_json(&json_report_path, &report_data)?;

    // Optional deterministic replay-style timeline for baseline and best runs.
    let trace_markdown_path = report_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!(
            "{}_event_trace.md",
            to_norm_key(&controlled_champion_name)
        ));
    let trace_json_path = trace_markdown_path.with_extension("json");
    let mut baseline_trace_sim =
        ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
            vlad_base.clone(),
            &baseline_fixed_build,
            &vlad_loadout.bonus_stats,
            Some(&baseline_runtime_loadout_selection),
            None,
            &enemy_builds,
            sim.clone(),
            urf.clone(),
        );
    baseline_trace_sim.enable_trace();
    while baseline_trace_sim.step(1) {}
    let baseline_trace = baseline_trace_sim.trace_events().to_vec();

    let mut best_trace_sim =
        ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
            vlad_base.clone(),
            &vlad_best_build,
            &vlad_loadout.bonus_stats,
            Some(&vlad_runtime_loadout_selection),
            best_order_acquired_map.as_ref(),
            &enemy_builds,
            sim.clone(),
            urf.clone(),
        );
    best_trace_sim.enable_trace();
    while best_trace_sim.step(1) {}
    let best_trace = best_trace_sim.trace_events().to_vec();

    let mut trace_md = String::new();
    trace_md.push_str(&format!("# {} Event Trace\n\n", controlled_champion_name));
    trace_md.push_str("## Baseline Build Trace\n");
    for line in &baseline_trace {
        trace_md.push_str("- ");
        trace_md.push_str(line);
        trace_md.push('\n');
    }
    trace_md.push_str("\n## Best Build Trace\n");
    for line in &best_trace {
        trace_md.push_str("- ");
        trace_md.push_str(line);
        trace_md.push('\n');
    }
    fs::write(&trace_markdown_path, trace_md)?;

    let trace_json = json!({
        "baseline": baseline_trace,
        "best": best_trace,
    });
    fs::write(&trace_json_path, serde_json::to_string_pretty(&trace_json)?)?;

    persistent_full_cache.flush()?;
    status.emit(
        "finalization",
        Some((processed_candidates, total_candidates)),
        Some(vlad_best_score),
        Some("reports, trace outputs, and persistent cache written"),
        true,
    );
    println!(
        "\nReport written: {}",
        format_repo_relative_path(&report_path)
    );
    println!(
        "Structured report written: {}",
        format_repo_relative_path(&json_report_path)
    );
    println!(
        "Trace report written: {}",
        format_repo_relative_path(&trace_markdown_path)
    );
    println!(
        "Trace json written: {}",
        format_repo_relative_path(&trace_json_path)
    );

    Ok(())
}

pub(super) fn run_controlled_champion_stepper(scenario_path: &Path, ticks: usize) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let sim_cfg = parse_simulation_config(
        scenario
            .get("simulation")
            .ok_or_else(|| anyhow!("Missing simulation"))?,
    )?;
    let (vlad_base_raw, baseline_fixed_names, vlad_loadout_selection) =
        parse_controlled_champion_config(&scenario, &champion_bases)?;
    let vlad_base = champion_at_level(&vlad_base_raw, sim_cfg.champion_level);

    let enemy_encounters = parse_opponent_encounters(&scenario, &champion_bases)?;
    let (selected_encounter_name, _, selected_enemies_raw) = enemy_encounters
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("opponents.encounters must include at least one encounter"))?;
    let enemies = selected_enemies_raw
        .into_iter()
        .map(|mut e| {
            e.base = champion_at_level(&e.base, sim_cfg.champion_level);
            e
        })
        .collect::<Vec<_>>();

    let enemy_loadout_selection = parse_opponent_shared_loadout_selection(&scenario);
    let vlad_loadout = resolve_loadout(&vlad_loadout_selection, sim_cfg.champion_level, true)?;
    let enemy_loadout = resolve_loadout(&enemy_loadout_selection, sim_cfg.champion_level, false)?;
    let loadout_domain = build_loadout_domain();
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain)?;

    let mut enemy_builds: Vec<(EnemyConfig, Vec<Item>, Stats)> = Vec::new();
    for enemy in &enemies {
        let key = to_norm_key(&enemy.name);
        let preset = enemy_presets.get(&key).ok_or_else(|| {
            anyhow!(
                "Missing URF preset for enemy champion '{}'. Add it to {}.",
                enemy.name,
                enemy_preset_data_path().display()
            )
        })?;
        let build = item_pool_from_names(&items, &preset.item_names)?;
        let mut bonus_stats = resolve_loadout(
            &enemy_loadout_from_preset(preset),
            sim_cfg.champion_level,
            false,
        )?
        .bonus_stats;
        bonus_stats.add(&enemy_loadout.bonus_stats);
        let mut enemy_with_loadout = enemy.clone();
        enemy_with_loadout.loadout_item_names = preset.item_names.clone();
        enemy_with_loadout.loadout_rune_names = preset.runes.clone();
        enemy_with_loadout.loadout_shards = preset.shards.clone();
        enemy_with_loadout.loadout_masteries = preset.masteries.clone();
        enemy_builds.push((enemy_with_loadout, build, bonus_stats));
    }
    let baseline_fixed_build = item_pool_from_names(&items, &baseline_fixed_names)?;

    let mut sim = ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
        vlad_base,
        &baseline_fixed_build,
        &vlad_loadout.bonus_stats,
        Some(&vlad_loadout_selection),
        None,
        &enemy_builds,
        sim_cfg.clone(),
        urf,
    );

    println!(
        "Server tick rate: {:.2} Hz ({:.5}s/tick)",
        sim_cfg.server_tick_rate_hz,
        sim.tick_seconds()
    );
    println!("Using opponent encounter: {}", selected_encounter_name);

    for tick in 0..ticks.max(1) {
        let alive = sim.step(1);
        let status = if alive { "alive" } else { "finished" };
        println!(
            "tick={} time={:.3}s health={:.2} targetable={} can_cast={} status={}",
            tick + 1,
            sim.current_time(),
            sim.current_health(),
            sim.is_targetable(),
            sim.can_cast(),
            status
        );
        if !alive {
            break;
        }
    }
    Ok(())
}

pub(super) fn run_stat_optimization(
    stat_key: &str,
    scenario_path: &Path,
    label: &str,
) -> Result<()> {
    let items = load_items()?;
    let scenario = load_json(scenario_path)?;
    let search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    let item_pool = default_item_pool(&items);

    let build_indices = choose_best_build_by_stat(
        &item_pool,
        stat_key,
        search_cfg.max_items,
        search_cfg.beam_width,
    );
    let build = build_from_indices(&item_pool, &build_indices);
    let stats = build_item_stats(&build);
    let value = stats.get_stat(stat_key);

    println!("Best build for {}:", label);
    println!(
        "- Items: {}",
        build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Total {}: {:.2}", label, value);
    Ok(())
}

fn build_enemy_similarity_notes(profiles: &[EnemyDerivedCombatStats]) -> Vec<String> {
    let mut pair_notes = Vec::new();
    for i in 0..profiles.len() {
        for j in (i + 1)..profiles.len() {
            let a = &profiles[i];
            let b = &profiles[j];
            let attack_damage_close = (a.attack_damage - b.attack_damage).abs() <= 8.0;
            let interval_close =
                (a.attack_interval_seconds - b.attack_interval_seconds).abs() <= 0.10;
            let range_close = (a.attack_range - b.attack_range).abs() <= 40.0;
            if attack_damage_close && interval_close && range_close {
                pair_notes.push(format!(
                    "{} and {} have very similar auto profiles (AD {:.1}/{:.1}, interval {:.3}/{:.3}, range {:.0}/{:.0}).",
                    a.champion,
                    b.champion,
                    a.attack_damage,
                    b.attack_damage,
                    a.attack_interval_seconds,
                    b.attack_interval_seconds,
                    a.attack_range,
                    b.attack_range
                ));
            }
        }
    }

    if pair_notes.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    out.push(format!(
        "Detected {} pair(s) of enemy auto profiles that are unusually similar; verify presets/loadout ingestion if this looks incorrect.",
        pair_notes.len()
    ));
    out.extend(pair_notes.into_iter().take(8));
    out
}

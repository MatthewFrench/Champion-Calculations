use anyhow::{Result, anyhow};
use rayon::prelude::*;
use serde_json::json;
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

use super::*;

pub(super) fn run_controlled_champion_fixed_loadout_rune_sweep_impl(
    scenario_path: &Path,
    options: &ControlledChampionFixedLoadoutOptions<'_>,
) -> Result<()> {
    #[derive(Debug, Clone)]
    struct RuneSweepEntry {
        keystone_name: String,
        loadout_selection: LoadoutSelection,
        objective_score: f64,
        outcome: CombatOutcome,
        objective_breakdown: ObjectiveScoreBreakdown,
        rune_proc_telemetry: Vec<crate::scripts::champions::ChampionRuneProcTelemetryEntry>,
        seed_repeat_scores: Vec<f64>,
        seed_repeat_values: Vec<u64>,
    }

    fn average_combat_outcomes(outcomes: &[CombatOutcome]) -> CombatOutcome {
        if outcomes.is_empty() {
            return CombatOutcome::default();
        }
        let n = outcomes.len() as f64;
        CombatOutcome {
            time_alive_seconds: outcomes
                .iter()
                .map(|outcome| outcome.time_alive_seconds)
                .sum::<f64>()
                / n,
            damage_dealt: outcomes
                .iter()
                .map(|outcome| outcome.damage_dealt)
                .sum::<f64>()
                / n,
            healing_done: outcomes
                .iter()
                .map(|outcome| outcome.healing_done)
                .sum::<f64>()
                / n,
            enemy_kills: ((outcomes
                .iter()
                .map(|outcome| outcome.enemy_kills as f64)
                .sum::<f64>()
                / n)
                .round()
                .max(0.0)) as usize,
            invulnerable_seconds: outcomes
                .iter()
                .map(|outcome| outcome.invulnerable_seconds)
                .sum::<f64>()
                / n,
        }
    }

    fn average_component_impacts(impacts: &[ObjectiveComponentImpact]) -> ObjectiveComponentImpact {
        if impacts.is_empty() {
            return ObjectiveComponentImpact::default();
        }
        let n = impacts.len() as f64;
        ObjectiveComponentImpact {
            weight: impacts.iter().map(|impact| impact.weight).sum::<f64>() / n,
            normalized_ratio: impacts
                .iter()
                .map(|impact| impact.normalized_ratio)
                .sum::<f64>()
                / n,
            contribution: impacts
                .iter()
                .map(|impact| impact.contribution)
                .sum::<f64>()
                / n,
            impact_percent: impacts
                .iter()
                .map(|impact| impact.impact_percent)
                .sum::<f64>()
                / n,
        }
    }

    fn average_objective_breakdowns(
        breakdowns: &[ObjectiveScoreBreakdown],
    ) -> ObjectiveScoreBreakdown {
        if breakdowns.is_empty() {
            return ObjectiveScoreBreakdown::default();
        }
        let n = breakdowns.len() as f64;
        ObjectiveScoreBreakdown {
            weighted_mean_score: breakdowns
                .iter()
                .map(|breakdown| breakdown.weighted_mean_score)
                .sum::<f64>()
                / n,
            worst_case_score: breakdowns
                .iter()
                .map(|breakdown| breakdown.worst_case_score)
                .sum::<f64>()
                / n,
            worst_case_weight: breakdowns
                .iter()
                .map(|breakdown| breakdown.worst_case_weight)
                .sum::<f64>()
                / n,
            final_score: breakdowns
                .iter()
                .map(|breakdown| breakdown.final_score)
                .sum::<f64>()
                / n,
            survival: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.survival)
                    .collect::<Vec<_>>(),
            ),
            damage: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.damage)
                    .collect::<Vec<_>>(),
            ),
            healing: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.healing)
                    .collect::<Vec<_>>(),
            ),
            enemy_kills: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.enemy_kills)
                    .collect::<Vec<_>>(),
            ),
            invulnerable_seconds: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.invulnerable_seconds)
                    .collect::<Vec<_>>(),
            ),
        }
    }

    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim = parse_simulation_config(simulation_config)?;
    sim.collect_rune_proc_telemetry = false;
    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim.champion_level;
    sim.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        simulation_config,
        simulation_level_before_controlled_override,
    );

    let mut controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim.champion_level);
    let controlled_champion_name = controlled_champion_base.name.clone();
    sim.controlled_champion_script = resolve_controlled_champion_script(&controlled_champion_name);

    let loadout_domain = build_loadout_domain();
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    if let Some(runes) = &options.fixed_rune_names {
        controlled_champion_loadout_selection.rune_names = runes.clone();
    }
    if let Some(shards) = &options.fixed_shard_stats {
        controlled_champion_loadout_selection.shard_stats = shards.clone();
    }
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    let fixed_build_items = item_pool_from_names(&items, &options.fixed_item_names)?;

    let baseline_keystone = controlled_champion_loadout_selection
        .rune_names
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("Controlled champion rune page must include a primary keystone"))?;
    let baseline_keystone_key = to_norm_key(&baseline_keystone);
    let primary_keystone_slot = loadout_domain
        .rune_paths
        .iter()
        .find_map(|path| {
            path.slot_runes.first().and_then(|slot| {
                slot.iter()
                    .any(|rune| to_norm_key(rune) == baseline_keystone_key)
                    .then_some(slot.clone())
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "Unable to resolve primary rune path for baseline keystone '{}'",
                baseline_keystone
            )
        })?;
    let mut keystone_candidates = primary_keystone_slot
        .into_iter()
        .map(|name| (to_norm_key(&name), name))
        .collect::<Vec<_>>();
    keystone_candidates.sort_by(|a, b| a.0.cmp(&b.0));
    keystone_candidates.dedup_by(|a, b| a.0 == b.0);
    let keystone_candidates = keystone_candidates
        .into_iter()
        .map(|(_, name)| name)
        .collect::<Vec<_>>();
    if keystone_candidates.is_empty() {
        return Err(anyhow!(
            "No keystone candidates found for baseline rune path '{}'",
            baseline_keystone
        ));
    }

    let enemy_scenarios_raw: Vec<ParsedOpponentEncounter> = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|encounter| {
            let scaled = encounter
                .actors
                .iter()
                .cloned()
                .map(|mut enemy| {
                    enemy.base = champion_at_level(&enemy.base, enemy.level);
                    enemy
                })
                .collect::<Vec<_>>();
            (encounter.name.clone(), encounter.weight, scaled)
        })
        .collect::<Vec<_>>();

    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;
    let mut enemy_build_scenarios = Vec::new();
    for (name, weight, enemies) in &enemy_scenarios {
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
            let preset_enemy_loadout =
                resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?;
            let enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }
    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, builds)| builds.clone())
        .unwrap_or_default();

    let mut search_cfg = parse_scenario_search_or_default(&scenario)?;
    apply_search_quality_profile(&mut search_cfg, options.search_quality_profile);
    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
        search_cfg.objective_enemy_kills_weight,
        search_cfg.objective_invulnerable_seconds_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            let damage_reference = enemy_builds_s
                .iter()
                .map(|(enemy, build, bonus_stats)| {
                    derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf).max_health
                })
                .sum::<f64>()
                .max(1.0);
            CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: damage_reference,
                healing_done: controlled_champion_base.base_health.max(1.0),
                enemy_kills: enemy_builds_s.len().max(1),
                invulnerable_seconds: sim.max_time_seconds.max(1.0),
            }
        })
        .collect::<Vec<_>>();
    let objective_eval_ctx = ObjectiveEvalContext {
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        enemy_build_scenarios: &enemy_build_scenarios,
        sim: &sim,
        urf: &urf,
        scenario_reference_outcomes: &scenario_reference_outcomes,
        weights: objective_component_weights,
        worst_case_weight: objective_worst_case_weight,
    };
    let sweep_seed_repeats = options.fixed_sweep_seed_repeats.max(1);
    let sweep_results_parallel = keystone_candidates
        .par_iter()
        .map(|keystone| -> Result<RuneSweepEntry> {
            let mut loadout_selection = controlled_champion_loadout_selection.clone();
            if let Some(primary_slot) = loadout_selection.rune_names.first_mut() {
                *primary_slot = keystone.clone();
            }
            loadout_selection =
                ensure_complete_loadout_selection(&loadout_selection, &loadout_domain)?;
            let resolved_loadout = resolve_loadout(&loadout_selection, sim.champion_level, true)?;
            let keystone_seed_base = fixed_sweep_keystone_seed_base(search_cfg.seed, keystone);
            let mut repeat_results = (0..sweep_seed_repeats)
                .into_par_iter()
                .map(|repeat_idx| {
                    let repeat_seed = fixed_sweep_repeat_seed(keystone_seed_base, repeat_idx);
                    let mut repeat_sim = sim.clone();
                    repeat_sim.combat_seed = Some(repeat_seed);
                    let repeat_objective_eval_ctx = ObjectiveEvalContext {
                        controlled_champion_base: objective_eval_ctx.controlled_champion_base,
                        controlled_champion_stack_overrides: objective_eval_ctx
                            .controlled_champion_stack_overrides,
                        enemy_build_scenarios: objective_eval_ctx.enemy_build_scenarios,
                        sim: &repeat_sim,
                        urf: objective_eval_ctx.urf,
                        scenario_reference_outcomes: objective_eval_ctx.scenario_reference_outcomes,
                        weights: objective_eval_ctx.weights,
                        worst_case_weight: objective_eval_ctx.worst_case_weight,
                    };
                    let (score, outcome, breakdown) =
                        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
                            &repeat_objective_eval_ctx,
                            &fixed_build_items,
                            &resolved_loadout.bonus_stats,
                            Some(&loadout_selection),
                        );
                    (repeat_idx, repeat_seed, score, outcome, breakdown)
                })
                .collect::<Vec<_>>();
            repeat_results.sort_by_key(|entry| entry.0);
            let seed_repeat_values = repeat_results
                .iter()
                .map(|entry| entry.1)
                .collect::<Vec<_>>();
            let seed_repeat_scores = repeat_results
                .iter()
                .map(|entry| entry.2)
                .collect::<Vec<_>>();
            let repeated_outcomes = repeat_results
                .iter()
                .map(|entry| entry.3)
                .collect::<Vec<_>>();
            let repeated_breakdowns = repeat_results
                .iter()
                .map(|entry| entry.4)
                .collect::<Vec<_>>();

            let objective_score =
                seed_repeat_scores.iter().sum::<f64>() / seed_repeat_scores.len().max(1) as f64;
            let outcome = average_combat_outcomes(&repeated_outcomes);
            let objective_breakdown = average_objective_breakdowns(&repeated_breakdowns);
            let mut trace_sim_cfg = sim.clone();
            trace_sim_cfg.collect_rune_proc_telemetry = true;
            if let Some(seed) = seed_repeat_values.first().copied() {
                trace_sim_cfg.combat_seed = Some(seed);
            }

            let mut trace_sim =
                ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
                    controlled_champion_base.clone(),
                    &fixed_build_items,
                    &resolved_loadout.bonus_stats,
                    Some(&loadout_selection),
                    None,
                    Some(&controlled_champion_stack_overrides),
                    &enemy_builds,
                    trace_sim_cfg,
                    urf.clone(),
                );
            while trace_sim.step(1) {}

            Ok(RuneSweepEntry {
                keystone_name: keystone.clone(),
                loadout_selection,
                objective_score,
                outcome,
                objective_breakdown,
                rune_proc_telemetry: trace_sim.controlled_champion_rune_proc_telemetry(),
                seed_repeat_scores,
                seed_repeat_values,
            })
        })
        .collect::<Vec<_>>();
    let mut sweep_results = sweep_results_parallel
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    sweep_results.sort_by(|a, b| {
        b.objective_score
            .partial_cmp(&a.objective_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| to_norm_key(&a.keystone_name).cmp(&to_norm_key(&b.keystone_name)))
    });

    let run_label = options
        .fixed_eval_label
        .as_deref()
        .unwrap_or("fixed_loadout_rune_sweep");
    let default_output_dir = default_fixed_loadout_rune_sweep_output_directory(
        options.search_quality_profile,
        run_label,
    );
    fs::create_dir_all(&default_output_dir)?;
    let controlled_champion_key = to_norm_key(&controlled_champion_name);
    let report_path = options
        .report_path_override
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            default_output_dir.join(format!(
                "{controlled_champion_key}_fixed_loadout_rune_sweep_report.md"
            ))
        });
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json_report_path = report_path.with_extension("json");

    let best_score = sweep_results
        .first()
        .map(|entry| entry.objective_score)
        .unwrap_or(0.0);
    let mut markdown = String::new();
    markdown.push_str("# Controlled Champion Fixed Loadout Rune Sweep\n\n");
    markdown.push_str(&format!("- Scenario: `{}`\n", scenario_path.display()));
    markdown.push_str(&format!(
        "- Search quality profile: `{}`\n",
        search_quality_profile_key(options.search_quality_profile)
    ));
    markdown.push_str(&format!(
        "- Controlled champion: `{}`\n",
        controlled_champion_name
    ));
    markdown.push_str(&format!(
        "- Build items: `{}`\n",
        fixed_build_items
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    markdown.push_str(&format!(
        "- Baseline runes: `{}`\n",
        controlled_champion_loadout_selection.rune_names.join(", ")
    ));
    markdown.push_str(&format!(
        "- Baseline shards: `{}`\n\n",
        controlled_champion_loadout_selection.shard_stats.join(", ")
    ));
    markdown.push_str(&format!(
        "- Seed repeats per keystone: `{}`\n\n",
        sweep_seed_repeats
    ));
    markdown.push_str(&format!("- Seed base: `{}`\n\n", search_cfg.seed));
    markdown.push_str("## Rune Sweep Ranking\n");
    for (idx, result) in sweep_results.iter().enumerate() {
        if sweep_seed_repeats > 1 {
            let (_, score_stddev) = mean_std(&result.seed_repeat_scores);
            let repeat_seeds = result
                .seed_repeat_values
                .iter()
                .map(|seed| seed.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            markdown.push_str(&format!(
                "{}. `{}`\n  - Score: `{:.4}`\n  - Delta: `{:+.4}`\n  - Seed stddev: `{:.4}`\n  - Outcome:\n    - Time alive: `{:.2}s`\n    - Damage dealt: `{:.1}`\n    - Healing done: `{:.1}`\n    - Enemy kills: `{}`\n    - Invulnerable seconds: `{:.2}s`\n  - Seeds: `[{}]`\n",
                idx + 1,
                result.keystone_name,
                result.objective_score,
                result.objective_score - best_score,
                score_stddev,
                result.outcome.time_alive_seconds,
                result.outcome.damage_dealt,
                result.outcome.healing_done,
                result.outcome.enemy_kills,
                result.outcome.invulnerable_seconds,
                repeat_seeds
            ));
        } else {
            markdown.push_str(&format!(
                "{}. `{}`\n  - Score: `{:.4}`\n  - Delta: `{:+.4}`\n  - Outcome:\n    - Time alive: `{:.2}s`\n    - Damage dealt: `{:.1}`\n    - Healing done: `{:.1}`\n    - Enemy kills: `{}`\n    - Invulnerable seconds: `{:.2}s`\n",
                idx + 1,
                result.keystone_name,
                result.objective_score,
                result.objective_score - best_score,
                result.outcome.time_alive_seconds,
                result.outcome.damage_dealt,
                result.outcome.healing_done,
                result.outcome.enemy_kills,
                result.outcome.invulnerable_seconds
            ));
        }
    }
    markdown.push('\n');
    markdown.push_str("## Rune Proc Telemetry\n");
    for result in &sweep_results {
        markdown.push_str(&format!("- {}:\n", result.keystone_name));
        append_rune_proc_telemetry_markdown_entries(
            &mut markdown,
            "  - ",
            "    ",
            &result.rune_proc_telemetry,
            result.outcome.damage_dealt,
            result.outcome.healing_done,
        );
    }
    fs::write(&report_path, markdown)?;

    let json_report = json!({
        "schema_version": FIXED_LOADOUT_RUNE_SWEEP_JSON_SCHEMA_VERSION,
        "scenario_path": scenario_path.display().to_string(),
        "search_quality_profile": search_quality_profile_key(options.search_quality_profile),
        "controlled_champion_name": controlled_champion_name,
        "items": fixed_build_items.iter().map(|item| item.name.clone()).collect::<Vec<_>>(),
        "baseline_rune_names": controlled_champion_loadout_selection.rune_names,
        "baseline_shard_stats": controlled_champion_loadout_selection.shard_stats,
        "seed_base": search_cfg.seed,
        "seed_repeats_per_keystone": sweep_seed_repeats,
        "results": sweep_results.iter().map(|result| {
            json!({
                "keystone_name": result.keystone_name,
                "loadout_selection": {
                    "rune_names": result.loadout_selection.rune_names,
                    "shard_stats": result.loadout_selection.shard_stats
                },
                "objective_score": result.objective_score,
                "outcome": {
                    "time_alive_seconds": result.outcome.time_alive_seconds,
                    "damage_dealt": result.outcome.damage_dealt,
                    "healing_done": result.outcome.healing_done,
                    "enemy_kills": result.outcome.enemy_kills,
                    "invulnerable_seconds": result.outcome.invulnerable_seconds
                },
                "objective_breakdown": {
                    "weighted_mean_score": result.objective_breakdown.weighted_mean_score,
                    "worst_case_score": result.objective_breakdown.worst_case_score,
                    "worst_case_weight": result.objective_breakdown.worst_case_weight,
                    "final_score": result.objective_breakdown.final_score,
                    "components": {
                        "survival": {
                            "weight": result.objective_breakdown.survival.weight,
                            "normalized_ratio": result.objective_breakdown.survival.normalized_ratio,
                            "contribution": result.objective_breakdown.survival.contribution,
                            "impact_percent": result.objective_breakdown.survival.impact_percent
                        },
                        "damage": {
                            "weight": result.objective_breakdown.damage.weight,
                            "normalized_ratio": result.objective_breakdown.damage.normalized_ratio,
                            "contribution": result.objective_breakdown.damage.contribution,
                            "impact_percent": result.objective_breakdown.damage.impact_percent
                        },
                        "healing": {
                            "weight": result.objective_breakdown.healing.weight,
                            "normalized_ratio": result.objective_breakdown.healing.normalized_ratio,
                            "contribution": result.objective_breakdown.healing.contribution,
                            "impact_percent": result.objective_breakdown.healing.impact_percent
                        },
                        "enemy_kills": {
                            "weight": result.objective_breakdown.enemy_kills.weight,
                            "normalized_ratio": result.objective_breakdown.enemy_kills.normalized_ratio,
                            "contribution": result.objective_breakdown.enemy_kills.contribution,
                            "impact_percent": result.objective_breakdown.enemy_kills.impact_percent
                        },
                        "invulnerable_seconds": {
                            "weight": result.objective_breakdown.invulnerable_seconds.weight,
                            "normalized_ratio": result.objective_breakdown.invulnerable_seconds.normalized_ratio,
                            "contribution": result.objective_breakdown.invulnerable_seconds.contribution,
                            "impact_percent": result.objective_breakdown.invulnerable_seconds.impact_percent
                        }
                    }
                },
                "seed_repeat_scores": result.seed_repeat_scores,
                "seed_repeat_values": result.seed_repeat_values,
                "rune_proc_telemetry": rune_proc_telemetry_json(
                    &result.rune_proc_telemetry,
                    result.outcome.damage_dealt,
                    result.outcome.healing_done,
                )
            })
        }).collect::<Vec<_>>()
    });
    fs::write(
        &json_report_path,
        serde_json::to_string_pretty(&json_report)?,
    )?;

    println!(
        "Fixed-loadout rune sweep report written: {}",
        format_repo_relative_path(&report_path)
    );
    println!(
        "Fixed-loadout rune sweep JSON written: {}",
        format_repo_relative_path(&json_report_path)
    );

    Ok(())
}

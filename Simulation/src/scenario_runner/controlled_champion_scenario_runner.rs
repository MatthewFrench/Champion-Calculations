use super::controlled_champion_scenario_setup::{
    ControlledChampionEnemyBuildSetup, ControlledChampionEnemyBuildSetupContext,
    ControlledChampionScenarioSearchSetup, ControlledChampionScenarioSearchSetupContext,
    prepare_controlled_champion_enemy_build_setup,
    prepare_controlled_champion_scenario_search_setup,
};
use super::controlled_champion_strict_ranking_finalization::{
    ControlledChampionStrictRankingFinalizationContext, finalize_controlled_champion_strict_ranking,
};
use super::*;

pub(super) fn run_controlled_champion_scenario_impl(
    scenario_path: &Path,
    options: &ControlledChampionRunOptions<'_>,
) -> Result<()> {
    let top_x = options.top_x;
    let min_item_diff = options.min_item_diff;
    let max_relative_gap_percent = options.max_relative_gap_percent;
    let report_path_override = options.report_path_override;
    let max_runtime_seconds = options.max_runtime_seconds;
    let popcorn_window_seconds = options.popcorn_window_seconds.filter(|s| *s > 0.0);
    let popcorn_window = popcorn_window_seconds.map(Duration::from_secs_f64);
    let popcorn_min_relative_improvement_percent =
        options.popcorn_min_relative_improvement_percent.max(0.0);
    let popcorn_min_relative_improvement = popcorn_min_relative_improvement_percent / 100.0;
    let status_every_seconds = options.status_every_seconds;
    let seed_override = options.seed_override;
    let search_quality_profile = if popcorn_window_seconds.is_some() {
        SearchQualityProfile::MaximumQuality
    } else {
        options.search_quality_profile
    };

    let run_start = Instant::now();
    let time_budget = max_runtime_seconds
        .filter(|s| *s > 0.0)
        .map(Duration::from_secs_f64);
    let defer_hard_budget_until_coverage =
        matches!(search_quality_profile, SearchQualityProfile::MaximumQuality);
    let hard_deadline_state = Arc::new(Mutex::new(None::<Instant>));
    let progress_state = Arc::new(Mutex::new(SignificantProgressState {
        best_overall_score: f64::NEG_INFINITY,
        best_significant_score: f64::NEG_INFINITY,
        significant_events: 0,
        last_significant_at: run_start,
    }));
    let hard_deadline_value = || hard_deadline_state.lock().ok().and_then(|state| *state);
    let coverage_stage_deadline = || hard_deadline_value();
    let current_deadline = || {
        let hard_deadline = hard_deadline_value();
        let progress_deadline = popcorn_window.map(|window| {
            let last_significant_at = progress_state
                .lock()
                .ok()
                .map(|state| state.last_significant_at)
                .unwrap_or(run_start);
            last_significant_at + window
        });
        match (hard_deadline, progress_deadline) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    };
    let deadline_for_search_type = |search_type: &str| {
        if search_type == "coverage_stage" {
            coverage_stage_deadline()
        } else {
            current_deadline()
        }
    };
    let record_score_progress = |score: f64| {
        if !score.is_finite() {
            return;
        }
        if let Ok(mut state) = progress_state.lock() {
            let previous_best_overall = state.best_overall_score;
            if score > state.best_overall_score {
                state.best_overall_score = score;
            }
            let significant = if !state.best_significant_score.is_finite() {
                true
            } else {
                let previous_best = previous_best_overall;
                let delta = if previous_best.is_finite() {
                    score - previous_best
                } else {
                    score - state.best_significant_score
                };
                if delta <= 0.0 {
                    false
                } else {
                    let threshold_base = if previous_best.is_finite() {
                        previous_best.abs().max(1e-9)
                    } else {
                        state.best_significant_score.abs().max(1e-9)
                    };
                    let threshold = threshold_base * popcorn_min_relative_improvement;
                    delta >= threshold
                }
            };
            if significant {
                state.best_significant_score = score;
                state.last_significant_at = Instant::now();
                state.significant_events += 1;
            }
        }
    };
    let status_every = Duration::from_secs_f64(status_every_seconds.max(1.0));
    let mut status = StatusReporter::new(run_start, status_every);
    let timeout_flag = Arc::new(AtomicUsize::new(0));
    status.emit("initialization", None, None, Some("starting"), true);
    let scenario_search_setup = prepare_controlled_champion_scenario_search_setup(
        ControlledChampionScenarioSearchSetupContext {
            scenario_path,
            search_quality_profile,
            seed_override,
            current_deadline: &current_deadline,
            timeout_flag: timeout_flag.as_ref(),
            status: &mut status,
        },
    )?;
    let ControlledChampionScenarioSearchSetup {
        items,
        urf,
        sim,
        controlled_champion_base,
        controlled_champion_base_raw,
        controlled_champion_name,
        controlled_champion_loadout_selection,
        controlled_champion_stack_overrides,
        raw_enemy_bases,
        enemy_scenarios,
        enemy_presets,
        search_cfg,
        active_strategies,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
        item_pool,
        max_items,
    } = scenario_search_setup;
    let enemy_loadout = ResolvedLoadout::default();

    let enemy_build_setup =
        prepare_controlled_champion_enemy_build_setup(ControlledChampionEnemyBuildSetupContext {
            enemy_scenarios: &enemy_scenarios,
            enemy_presets: &enemy_presets,
            items: &items,
            sim: &sim,
            urf: &urf,
            current_deadline: &current_deadline,
            timeout_flag: timeout_flag.as_ref(),
            status: &mut status,
        })?;
    let ControlledChampionEnemyBuildSetup {
        enemy_presets_used,
        enemy_build_scenarios,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
    } = enemy_build_setup;

    let controlled_champion_base_loadout = resolve_loadout(
        &controlled_champion_loadout_selection,
        sim.champion_level,
        true,
    )?;
    let resolve_cache: Arc<Mutex<HashMap<String, ResolvedLoadout>>> =
        Arc::new(Mutex::new(HashMap::new()));
    if let Ok(mut map) = resolve_cache.lock() {
        map.insert(
            loadout_selection_key(&controlled_champion_loadout_selection),
            controlled_champion_base_loadout.clone(),
        );
    }
    let best_loadout_by_candidate: Arc<Mutex<ResolvedByCandidateMap>> =
        Arc::new(Mutex::new(HashMap::new()));
    let best_outcome_by_candidate: Arc<Mutex<OutcomeByCandidateMap>> =
        Arc::new(Mutex::new(HashMap::new()));

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

    let resolve_loadout_for_selection = |selection: &LoadoutSelection| -> Option<ResolvedLoadout> {
        let key = loadout_selection_key(selection);
        if let Ok(map) = resolve_cache.lock()
            && let Some(existing) = map.get(&key).cloned()
        {
            return Some(existing);
        }
        let resolved = resolve_loadout(selection, sim.champion_level, true).ok()?;
        if let Ok(mut map) = resolve_cache.lock() {
            map.insert(key, resolved.clone());
        }
        Some(resolved)
    };
    let item_has_unmodeled_effect_by_index = item_pool
        .iter()
        .map(is_item_effect_unmodeled)
        .collect::<Vec<_>>();

    let full_eval_count = AtomicUsize::new(0);
    let unmodeled_rune_candidates_rejected = AtomicUsize::new(0);
    let unmodeled_rune_candidates_penalized = AtomicUsize::new(0);
    let unmodeled_item_effect_candidates_rejected = AtomicUsize::new(0);
    let unmodeled_item_effect_candidates_penalized = AtomicUsize::new(0);
    let full_cache = Arc::new(BlockingScoreCache::new());
    let unique_scored_candidate_keys = Arc::new(ShardedStringSet::new());
    let search_type_counters =
        initialize_search_type_counters(&active_strategies, &search_cfg.strategy);
    let full_score_for_search_type = |search_type: &str, candidate: &BuildKey| {
        increment_search_type_counter(search_type_counters.as_ref(), search_type, 1, 0);
        if deadline_reached(deadline_for_search_type(search_type)) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            return f64::NEG_INFINITY;
        }
        let key = canonical_build_candidate(candidate.clone());
        let is_full_candidate = key.item_indices.len() == max_items;
        let cache_key = build_key_cache_string(&key);
        let cache = Arc::clone(&full_cache);
        let search_type_owned = search_type.to_string();
        cache.get_or_compute(cache_key.clone(), || {
            if deadline_reached(deadline_for_search_type(&search_type_owned)) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            let Some(resolved_loadout) = resolve_loadout_for_selection(&key.loadout_selection)
            else {
                return f64::NEG_INFINITY;
            };
            let unmodeled_rune_count = resolved_loadout.unmodeled_rune_names.len();
            let unmodeled_item_effect_count = key
                .item_indices
                .iter()
                .filter(|item_idx| {
                    item_has_unmodeled_effect_by_index
                        .get(**item_idx)
                        .copied()
                        .unwrap_or(false)
                })
                .count();
            if unmodeled_rune_count > 0 && search_cfg.unmodeled_rune_hard_gate {
                unmodeled_rune_candidates_rejected.fetch_add(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            if unmodeled_item_effect_count > 0 && search_cfg.unmodeled_item_effect_hard_gate {
                unmodeled_item_effect_candidates_rejected.fetch_add(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            arm_time_budget_deadline_if_unset(
                &hard_deadline_state,
                time_budget,
                defer_hard_budget_until_coverage,
                &search_type_owned,
            );
            if is_full_candidate {
                full_eval_count.fetch_add(1, AtomicOrdering::Relaxed);
            }
            increment_search_type_counter(search_type_counters.as_ref(), &search_type_owned, 0, 1);
            let build_items = build_from_indices(&item_pool, &key.item_indices);
            let (score, outcome) = evaluate_build_with_bonus(
                &build_items,
                &resolved_loadout.bonus_stats,
                Some(&key.loadout_selection),
            );
            let mut score = score;
            if unmodeled_rune_count > 0 {
                unmodeled_rune_candidates_penalized.fetch_add(1, AtomicOrdering::Relaxed);
                score -= search_cfg.unmodeled_rune_penalty_per_rune.max(0.0)
                    * unmodeled_rune_count as f64;
            }
            if unmodeled_item_effect_count > 0 {
                unmodeled_item_effect_candidates_penalized.fetch_add(1, AtomicOrdering::Relaxed);
                score -= search_cfg.unmodeled_item_effect_penalty_per_item.max(0.0)
                    * unmodeled_item_effect_count as f64;
            }
            if is_full_candidate && score.is_finite() {
                unique_scored_candidate_keys.insert(cache_key.clone());
            }
            if is_full_candidate {
                if let Ok(mut map) = best_loadout_by_candidate.lock() {
                    map.insert(key.clone(), resolved_loadout);
                }
                if let Ok(mut map) = best_outcome_by_candidate.lock() {
                    map.insert(key.clone(), outcome);
                }
            }
            if is_full_candidate {
                record_score_progress(score);
            }
            score
        })
    };
    let evaluate_candidate_direct = |candidate: &BuildKey| {
        let key = canonical_build_candidate(candidate.clone());
        let resolved_loadout = resolve_loadout_for_selection(&key.loadout_selection)?;
        let unmodeled_rune_count = resolved_loadout.unmodeled_rune_names.len();
        let unmodeled_item_effect_count = key
            .item_indices
            .iter()
            .filter(|item_idx| {
                item_has_unmodeled_effect_by_index
                    .get(**item_idx)
                    .copied()
                    .unwrap_or(false)
            })
            .count();
        if unmodeled_rune_count > 0 && search_cfg.unmodeled_rune_hard_gate {
            return None;
        }
        if unmodeled_item_effect_count > 0 && search_cfg.unmodeled_item_effect_hard_gate {
            return None;
        }
        arm_time_budget_deadline_if_unset(
            &hard_deadline_state,
            time_budget,
            defer_hard_budget_until_coverage,
            "strict_fallback",
        );
        let build_items = build_from_indices(&item_pool, &key.item_indices);
        let (score, outcome) = evaluate_build_with_bonus(
            &build_items,
            &resolved_loadout.bonus_stats,
            Some(&key.loadout_selection),
        );
        let score = score
            - search_cfg.unmodeled_rune_penalty_per_rune.max(0.0) * unmodeled_rune_count as f64
            - search_cfg.unmodeled_item_effect_penalty_per_item.max(0.0)
                * unmodeled_item_effect_count as f64;
        if key.item_indices.len() == max_items && score.is_finite() {
            unique_scored_candidate_keys.insert(build_key_cache_string(&key));
        }
        Some((key, score, outcome, resolved_loadout))
    };

    let full_search_params = FullLoadoutSearchParams {
        item_pool: &item_pool,
        max_items,
        loadout_domain: search_loadout_domain.as_ref(),
        base_loadout: &controlled_champion_search_base_loadout_selection,
    };

    let coverage_stage = run_maximum_quality_coverage_stage(CoverageStageRunContext {
        search_quality_profile,
        search_cfg: &search_cfg,
        min_item_diff,
        item_pool: &item_pool,
        search_loadout_domain: search_loadout_domain.as_ref(),
        full_search_params,
        status: &mut status,
        timeout_flag: timeout_flag.as_ref(),
        coverage_stage_deadline: &coverage_stage_deadline,
        full_score_for_search_type: &full_score_for_search_type,
    });
    let coverage_stage_diagnostics = coverage_stage.diagnostics;
    let coverage_seed_candidates = coverage_stage.seed_candidates;

    if time_budget.is_some() && hard_deadline_value().is_none() {
        // Bootstrap one timed-phase simulation so staged search loops get a live deadline value.
        let mut bootstrap_seed = search_cfg.seed ^ 0xC0DE_DA7A_u64;
        let bootstrap_candidate = canonical_build_candidate(BuildKey {
            item_indices: random_valid_build(&item_pool, max_items, &mut bootstrap_seed),
            loadout_selection: controlled_champion_search_base_loadout_selection.clone(),
        });
        let bootstrap_search_type = format!("seed_search:{}", search_cfg.strategy);
        let _ = full_score_for_search_type(bootstrap_search_type.as_str(), &bootstrap_candidate);
        if hard_deadline_value().is_none() {
            arm_time_budget_deadline_if_unset(
                &hard_deadline_state,
                time_budget,
                defer_hard_budget_until_coverage,
                "seed_search:bootstrap",
            );
        }
    }

    let ensemble_seeds = search_cfg.ensemble_seeds.max(1);
    let seed_and_strict = run_seed_and_strict_ranking(SeedAndStrictRankingRunContext {
        search_cfg: &search_cfg,
        active_strategies: &active_strategies,
        item_pool: &item_pool,
        max_items,
        search_loadout_domain: search_loadout_domain.as_ref(),
        controlled_champion_search_base_loadout_selection:
            &controlled_champion_search_base_loadout_selection,
        full_search_params,
        coverage_seed_candidates: &coverage_seed_candidates,
        timeout_flag: timeout_flag.as_ref(),
        status: &mut status,
        current_deadline: &current_deadline,
        full_score_for_search_type: &full_score_for_search_type,
    });
    let seed_ranked = seed_and_strict.seed_ranked;
    let seed_top_sets = seed_and_strict.seed_top_sets;
    let best_seeded_candidate = seed_and_strict.best_seeded_candidate;
    let unique_candidate_keys = seed_and_strict.unique_candidate_keys;
    let strict_scores = seed_and_strict.strict_scores;
    let candidate_keys_generated = seed_and_strict.candidate_keys_generated;
    let candidate_duplicates_pruned = seed_and_strict.candidate_duplicates_pruned;
    let strict_seed_scored_candidates = seed_and_strict.strict_seed_scored_candidates;
    let strict_remaining_candidates = seed_and_strict.strict_remaining_candidates;
    let strict_non_finite_candidates = seed_and_strict.strict_non_finite_candidates;
    let strict_candidates_skipped_timeout = seed_and_strict.strict_candidates_skipped_timeout;
    let strict_completion_percent = seed_and_strict.strict_completion_percent;
    let strict_random_promotions_done = seed_and_strict.strict_random_promotions_done;
    let processed_candidates = seed_and_strict.processed_candidates;
    let total_candidates = seed_and_strict.total_candidates;
    let mut timed_out = seed_and_strict.timed_out;
    let bleed_candidate_count = seed_and_strict.bleed_candidate_count;
    let adaptive_candidate_count = seed_and_strict.adaptive_candidate_count;

    let strict_ranking_finalization = finalize_controlled_champion_strict_ranking(
        ControlledChampionStrictRankingFinalizationContext {
            strict_scores,
            best_seeded_candidate: &best_seeded_candidate,
            unique_candidate_keys: &unique_candidate_keys,
            search_seed: search_cfg.seed,
            item_pool: &item_pool,
            max_items,
            best_outcome_by_candidate: &best_outcome_by_candidate,
            best_loadout_by_candidate: &best_loadout_by_candidate,
            evaluate_candidate_direct: &evaluate_candidate_direct,
            sim_max_time_seconds: sim.max_time_seconds,
            objective_component_weights,
            timeout_flag: timeout_flag.as_ref(),
            current_deadline: &current_deadline,
            seed_ranked: &seed_ranked,
            seed_top_sets: &seed_top_sets,
            ensemble_seed_top_k: search_cfg.ensemble_seed_top_k,
        },
    );
    let controlled_champion_ranked = strict_ranking_finalization.controlled_champion_ranked;
    let seed_best_scores = strict_ranking_finalization.seed_best_scores;
    let seed_hits_by_key = strict_ranking_finalization.seed_hits_by_key;
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;

    if controlled_champion_ranked.is_empty() {
        return Err(anyhow!(
            "No valid full-build candidate remained after strict ranking. candidate_keys_generated={} unique_candidates={} strict_non_finite={} unmodeled_rune_rejected={} unmodeled_item_effect_rejected={} coverage_assets={}/{}. This run cannot produce a valid best build; adjust quality gates or increase modeled coverage.",
            candidate_keys_generated,
            total_candidates,
            strict_non_finite_candidates,
            unmodeled_rune_candidates_rejected.load(AtomicOrdering::Relaxed),
            unmodeled_item_effect_candidates_rejected.load(AtomicOrdering::Relaxed),
            coverage_stage_diagnostics.assets_covered,
            coverage_stage_diagnostics.assets_total
        ));
    }
    let controlled_champion_best_candidate = controlled_champion_ranked[0].0.clone();
    let controlled_champion_best_build =
        build_from_indices(&item_pool, &controlled_champion_best_candidate.item_indices);
    let controlled_champion_runtime_loadout_selection =
        controlled_champion_best_candidate.loadout_selection.clone();
    let controlled_champion_loadout = best_loadout_by_candidate
        .lock()
        .ok()
        .and_then(|m| m.get(&controlled_champion_best_candidate).cloned())
        .or_else(|| resolve_loadout_for_selection(&controlled_champion_runtime_loadout_selection))
        .unwrap_or_else(|| controlled_champion_base_loadout.clone());

    let controlled_champion_best_score = controlled_champion_ranked
        .first()
        .map(|(_, s)| *s)
        .unwrap_or(0.0);
    let controlled_champion_best_outcome = best_outcome_by_candidate
        .lock()
        .ok()
        .and_then(|m| m.get(&controlled_champion_best_candidate).copied())
        .unwrap_or_else(|| {
            aggregate_objective_score_and_outcome_with_loadout_selection(
                &objective_eval_ctx,
                &controlled_champion_best_build,
                &controlled_champion_loadout.bonus_stats,
                Some(&controlled_champion_runtime_loadout_selection),
            )
            .1
        });
    let (_, _, best_score_breakdown) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            &objective_eval_ctx,
            &controlled_champion_best_build,
            &controlled_champion_loadout.bonus_stats,
            Some(&controlled_champion_runtime_loadout_selection),
        );
    let best_cap_survivor =
        controlled_champion_best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    let progress_snapshot =
        progress_state
            .lock()
            .ok()
            .map(|state| *state)
            .unwrap_or(SignificantProgressState {
                best_overall_score: f64::NEG_INFINITY,
                best_significant_score: f64::NEG_INFINITY,
                significant_events: 0,
                last_significant_at: run_start,
            });
    let seconds_since_last_significant_improvement = Instant::now()
        .saturating_duration_since(progress_snapshot.last_significant_at)
        .as_secs_f64();
    let mut search_type_breakdown = snapshot_search_type_counters(search_type_counters.as_ref());
    search_type_breakdown.sort_by(|a, b| {
        b.new_simulations
            .cmp(&a.new_simulations)
            .then_with(|| b.score_requests.cmp(&a.score_requests))
            .then_with(|| a.name.cmp(&b.name))
    });
    let effective_threads = rayon::current_num_threads();
    let seed_orchestration_parallel = ensemble_seeds > 1;
    let portfolio_strategy_parallel =
        search_cfg.strategy == "portfolio" && active_strategies.len() > 1;
    let strategy_elites_parallel = active_strategies.len() > 1 || ensemble_seeds > 1;
    let estimated_total_candidate_space = {
        let item_space = estimated_legal_item_build_count(&item_pool, max_items);
        let loadout_space = estimated_legal_loadout_count(search_loadout_domain.as_ref());
        let total = item_space * loadout_space;
        (total.is_finite() && total > 0.0).then_some(total)
    };
    let unique_scored_candidates = unique_scored_candidate_keys.len();
    let estimated_run_space_coverage_percent = estimated_total_candidate_space
        .map(|total| ((unique_scored_candidates as f64) / total * 100.0).clamp(0.0, 100.0));
    let (estimated_close_to_optimal_probability, estimated_close_to_optimal_probability_note) =
        estimate_close_to_optimal_probability(
            unique_scored_candidates,
            estimated_total_candidate_space,
        );

    emit_controlled_champion_result_reporting(ControlledChampionResultReportingContext {
        scenario_path,
        controlled_champion_name: &controlled_champion_name,
        search_cfg: &search_cfg,
        search_quality_profile,
        max_runtime_seconds,
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
        report_path_override,
        enemy_builds: &enemy_builds,
        enemy_derived_combat_stats: &enemy_derived_combat_stats,
        enemy_similarity_notes: &enemy_similarity_notes,
        enemy_presets_used: &enemy_presets_used,
        enemy_build_scenarios: &enemy_build_scenarios,
        enemy_loadout: &enemy_loadout,
        raw_enemy_bases: &raw_enemy_bases,
        full_eval_count: &full_eval_count,
        full_cache: full_cache.as_ref(),
        ensemble_seeds,
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel,
        coverage_stage_diagnostics: &coverage_stage_diagnostics,
        candidate_keys_generated,
        candidate_duplicates_pruned,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_random_promotions_done,
        unique_candidate_keys: &unique_candidate_keys,
        unique_scored_candidates,
        bleed_candidate_count,
        adaptive_candidate_count,
        search_type_breakdown,
        seed_best_scores,
        seed_hits_by_key: &seed_hits_by_key,
        objective_component_weights,
        objective_worst_case_weight,
        run_start,
        time_budget,
        popcorn_window,
        progress_snapshot,
        seconds_since_last_significant_improvement,
        timed_out,
        processed_candidates,
        total_candidates,
        estimated_total_candidate_space,
        estimated_run_space_coverage_percent,
        estimated_close_to_optimal_probability,
        estimated_close_to_optimal_probability_note,
        unmodeled_rune_candidates_rejected: &unmodeled_rune_candidates_rejected,
        unmodeled_rune_candidates_penalized: &unmodeled_rune_candidates_penalized,
        unmodeled_item_effect_candidates_rejected: &unmodeled_item_effect_candidates_rejected,
        unmodeled_item_effect_candidates_penalized: &unmodeled_item_effect_candidates_penalized,
        controlled_champion_best_build: &controlled_champion_best_build,
        controlled_champion_best_score,
        controlled_champion_best_outcome,
        best_cap_survivor,
        controlled_champion_loadout: &controlled_champion_loadout,
        controlled_champion_runtime_loadout_selection:
            &controlled_champion_runtime_loadout_selection,
        controlled_champion_ranked: &controlled_champion_ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
        item_pool: &item_pool,
        best_loadout_by_candidate: &best_loadout_by_candidate,
        resolve_loadout_for_selection: &resolve_loadout_for_selection,
        controlled_champion_base_loadout: &controlled_champion_base_loadout,
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_base_raw: &controlled_champion_base_raw,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        sim: &sim,
        urf: &urf,
        best_score_breakdown,
        status: &mut status,
    })
}

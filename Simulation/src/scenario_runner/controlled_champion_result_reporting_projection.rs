use super::controlled_champion_result_reporting::{
    ControlledChampionResultReportingContext, emit_controlled_champion_result_reporting,
};
use super::*;

pub(super) struct ControlledChampionResultReportingProjectionContext<'a, 'ctx> {
    pub(super) scenario_path: &'a Path,
    pub(super) controlled_champion_name: &'a str,
    pub(super) search_cfg: &'a BuildSearchConfig,
    pub(super) search_quality_profile: SearchQualityProfile,
    pub(super) max_runtime_seconds: Option<f64>,
    pub(super) popcorn_window_seconds: Option<f64>,
    pub(super) popcorn_min_relative_improvement_percent: f64,
    pub(super) report_path_override: Option<&'a str>,
    pub(super) enemy_builds: &'a [EnemyBuildEntry],
    pub(super) enemy_derived_combat_stats: &'a [EnemyDerivedCombatStats],
    pub(super) enemy_similarity_notes: &'a [String],
    pub(super) enemy_presets_used: &'a HashMap<String, EnemyUrfPreset>,
    pub(super) enemy_build_scenarios: &'a [EnemyBuildScenario],
    pub(super) enemy_loadout: &'a ResolvedLoadout,
    pub(super) raw_enemy_bases: &'a HashMap<String, ChampionBase>,
    pub(super) full_eval_count: &'a AtomicUsize,
    pub(super) full_cache: &'a BlockingScoreCache,
    pub(super) ensemble_seeds: usize,
    pub(super) active_strategies: &'a [String],
    pub(super) coverage_stage_diagnostics: &'a CoverageStageDiagnostics,
    pub(super) candidate_keys_generated: usize,
    pub(super) candidate_duplicates_pruned: usize,
    pub(super) strict_seed_scored_candidates: usize,
    pub(super) strict_remaining_candidates: usize,
    pub(super) strict_non_finite_candidates: usize,
    pub(super) strict_candidates_skipped_timeout: usize,
    pub(super) strict_completion_percent: f64,
    pub(super) strict_random_promotions_done: usize,
    pub(super) unique_candidate_keys: &'a [BuildKey],
    pub(super) bleed_candidate_count: usize,
    pub(super) adaptive_candidate_count: usize,
    pub(super) seed_best_scores: Vec<f64>,
    pub(super) seed_hits_by_key: &'a HashMap<BuildKey, usize>,
    pub(super) objective_component_weights: ObjectiveComponentWeights,
    pub(super) objective_worst_case_weight: f64,
    pub(super) run_start: Instant,
    pub(super) time_budget: Option<Duration>,
    pub(super) popcorn_window: Option<Duration>,
    pub(super) timed_out: bool,
    pub(super) processed_candidates: usize,
    pub(super) total_candidates: usize,
    pub(super) unmodeled_rune_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_rune_candidates_penalized: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_penalized: &'a AtomicUsize,
    pub(super) controlled_champion_ranked: &'a [(BuildKey, f64)],
    pub(super) top_x: usize,
    pub(super) min_item_diff: usize,
    pub(super) max_relative_gap_percent: f64,
    pub(super) item_pool: &'a [Item],
    pub(super) max_items: usize,
    pub(super) search_loadout_domain: &'a crate::data::LoadoutDomain,
    pub(super) unique_scored_candidate_keys: &'a ShardedStringSet,
    pub(super) search_type_counters: &'a HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>,
    pub(super) best_loadout_by_candidate: &'a Arc<Mutex<ResolvedByCandidateMap>>,
    pub(super) best_outcome_by_candidate: &'a Arc<Mutex<OutcomeByCandidateMap>>,
    pub(super) resolve_loadout_for_selection:
        &'a dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout>,
    pub(super) controlled_champion_base_loadout: &'a ResolvedLoadout,
    pub(super) controlled_champion_base: &'a ChampionBase,
    pub(super) controlled_champion_base_raw: &'a ChampionBase,
    pub(super) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(super) objective_eval_ctx: &'a ObjectiveEvalContext<'ctx>,
    pub(super) sim: &'a SimulationConfig,
    pub(super) urf: &'a UrfBuffs,
    pub(super) progress_state: &'a Arc<Mutex<SignificantProgressState>>,
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) status: &'a mut StatusReporter,
}

pub(super) fn emit_controlled_champion_result_reporting_projection(
    context: ControlledChampionResultReportingProjectionContext<'_, '_>,
) -> Result<()> {
    let ControlledChampionResultReportingProjectionContext {
        scenario_path,
        controlled_champion_name,
        search_cfg,
        search_quality_profile,
        max_runtime_seconds,
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
        report_path_override,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
        enemy_presets_used,
        enemy_build_scenarios,
        enemy_loadout,
        raw_enemy_bases,
        full_eval_count,
        full_cache,
        ensemble_seeds,
        active_strategies,
        coverage_stage_diagnostics,
        candidate_keys_generated,
        candidate_duplicates_pruned,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_random_promotions_done,
        unique_candidate_keys,
        bleed_candidate_count,
        adaptive_candidate_count,
        seed_best_scores,
        seed_hits_by_key,
        objective_component_weights,
        objective_worst_case_weight,
        run_start,
        time_budget,
        popcorn_window,
        timed_out,
        processed_candidates,
        total_candidates,
        unmodeled_rune_candidates_rejected,
        unmodeled_rune_candidates_penalized,
        unmodeled_item_effect_candidates_rejected,
        unmodeled_item_effect_candidates_penalized,
        controlled_champion_ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
        item_pool,
        max_items,
        search_loadout_domain,
        unique_scored_candidate_keys,
        search_type_counters,
        best_loadout_by_candidate,
        best_outcome_by_candidate,
        resolve_loadout_for_selection,
        controlled_champion_base_loadout,
        controlled_champion_base,
        controlled_champion_base_raw,
        controlled_champion_stack_overrides,
        objective_eval_ctx,
        sim,
        urf,
        progress_state,
        timeout_flag,
        status,
    } = context;

    let controlled_champion_best_candidate = controlled_champion_ranked[0].0.clone();
    let controlled_champion_best_build =
        build_from_indices(item_pool, &controlled_champion_best_candidate.item_indices);
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
                objective_eval_ctx,
                &controlled_champion_best_build,
                &controlled_champion_loadout.bonus_stats,
                Some(&controlled_champion_runtime_loadout_selection),
            )
            .1
        });
    let (_, _, best_score_breakdown) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            objective_eval_ctx,
            &controlled_champion_best_build,
            &controlled_champion_loadout.bonus_stats,
            Some(&controlled_champion_runtime_loadout_selection),
        );
    let best_cap_survivor =
        controlled_champion_best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;
    let timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;
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
    let mut search_type_breakdown = snapshot_search_type_counters(search_type_counters);
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
        let item_space = estimated_legal_item_build_count(item_pool, max_items);
        let loadout_space = estimated_legal_loadout_count(search_loadout_domain);
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
        controlled_champion_name,
        search_cfg,
        search_quality_profile,
        max_runtime_seconds,
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
        report_path_override,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
        enemy_presets_used,
        enemy_build_scenarios,
        enemy_loadout,
        raw_enemy_bases,
        full_eval_count,
        full_cache,
        ensemble_seeds,
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel,
        coverage_stage_diagnostics,
        candidate_keys_generated,
        candidate_duplicates_pruned,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_random_promotions_done,
        unique_candidate_keys,
        unique_scored_candidates,
        bleed_candidate_count,
        adaptive_candidate_count,
        search_type_breakdown,
        seed_best_scores,
        seed_hits_by_key,
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
        unmodeled_rune_candidates_rejected,
        unmodeled_rune_candidates_penalized,
        unmodeled_item_effect_candidates_rejected,
        unmodeled_item_effect_candidates_penalized,
        controlled_champion_best_build: &controlled_champion_best_build,
        controlled_champion_best_score,
        controlled_champion_best_outcome,
        best_cap_survivor,
        controlled_champion_loadout: &controlled_champion_loadout,
        controlled_champion_runtime_loadout_selection:
            &controlled_champion_runtime_loadout_selection,
        controlled_champion_ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
        item_pool,
        best_loadout_by_candidate,
        resolve_loadout_for_selection,
        controlled_champion_base_loadout,
        controlled_champion_base,
        controlled_champion_base_raw,
        controlled_champion_stack_overrides,
        sim,
        urf,
        best_score_breakdown,
        status,
    })
}

use anyhow::anyhow;

use self::deadline_and_progress::ControlledChampionRuntimeDeadlines;
use self::projection_dispatch::{
    ControlledChampionProjectionDispatchContext, emit_controlled_champion_projection_dispatch,
};
use self::runtime_setup::{
    ControlledChampionRuntimeSetup, ControlledChampionRuntimeSetupContext,
    prepare_controlled_champion_runtime_setup,
};
use self::search_execution::{
    ControlledChampionSearchExecutionContext, ControlledChampionSearchExecutionResult,
    execute_controlled_champion_search_execution,
};
use crate::scenario_runner::*;

mod deadline_and_progress;
mod projection_dispatch;
mod runtime_setup;
mod search_execution;

pub(in crate::scenario_runner) fn run_controlled_champion_scenario_impl(
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
    let runtime_deadlines = ControlledChampionRuntimeDeadlines::new(
        run_start,
        popcorn_window,
        popcorn_min_relative_improvement,
    );
    let hard_deadline_state = runtime_deadlines.hard_deadline_state();
    let progress_state = runtime_deadlines.progress_state();
    let coverage_stage_deadline = || runtime_deadlines.coverage_stage_deadline();
    let current_deadline = || runtime_deadlines.current_deadline();
    let deadline_for_search_type =
        |search_type: &str| runtime_deadlines.deadline_for_search_type(search_type);
    let record_score_progress = |score: f64| runtime_deadlines.record_score_progress(score);
    let status_every = Duration::from_secs_f64(status_every_seconds.max(1.0));
    let mut status = StatusReporter::new(run_start, status_every);
    let timeout_flag = Arc::new(AtomicUsize::new(0));
    status.emit("initialization", None, None, Some("starting"), true);
    let runtime_setup =
        prepare_controlled_champion_runtime_setup(ControlledChampionRuntimeSetupContext {
            scenario_path,
            search_quality_profile,
            seed_override,
            current_deadline: &current_deadline,
            timeout_flag: timeout_flag.as_ref(),
            status: &mut status,
        })?;
    let ControlledChampionRuntimeSetup {
        urf,
        sim,
        controlled_champion_base,
        controlled_champion_base_raw,
        controlled_champion_name,
        controlled_champion_stack_overrides,
        raw_enemy_bases,
        enemy_presets_used,
        enemy_build_scenarios,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
        search_cfg,
        active_strategies,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
        item_pool,
        max_items,
        controlled_champion_base_loadout,
        resolve_cache,
        best_loadout_by_candidate,
        best_outcome_by_candidate,
        objective_worst_case_weight,
        objective_component_weights,
        scenario_reference_outcomes,
        item_has_unmodeled_effect_by_index,
        enemy_loadout,
    } = runtime_setup;

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

    let full_eval_count = AtomicUsize::new(0);
    let unmodeled_rune_candidates_rejected = AtomicUsize::new(0);
    let unmodeled_rune_candidates_penalized = AtomicUsize::new(0);
    let unmodeled_item_effect_candidates_rejected = AtomicUsize::new(0);
    let unmodeled_item_effect_candidates_penalized = AtomicUsize::new(0);
    let full_cache = Arc::new(BlockingScoreCache::new());
    let unique_scored_candidate_keys = Arc::new(ShardedStringSet::new());
    let search_type_counters =
        initialize_search_type_counters(&active_strategies, &search_cfg.strategy);
    let search_execution =
        execute_controlled_champion_search_execution(ControlledChampionSearchExecutionContext {
            search_quality_profile,
            search_cfg: &search_cfg,
            min_item_diff,
            item_pool: &item_pool,
            max_items,
            search_loadout_domain: search_loadout_domain.as_ref(),
            controlled_champion_search_base_loadout_selection:
                &controlled_champion_search_base_loadout_selection,
            active_strategies: &active_strategies,
            objective_component_weights,
            sim_max_time_seconds: sim.max_time_seconds,
            time_budget,
            defer_hard_budget_until_coverage,
            hard_deadline_state: &hard_deadline_state,
            current_deadline: &current_deadline,
            coverage_stage_deadline: &coverage_stage_deadline,
            deadline_for_search_type: &deadline_for_search_type,
            record_score_progress: &record_score_progress,
            resolve_loadout_for_selection: &resolve_loadout_for_selection,
            evaluate_build_with_bonus: &evaluate_build_with_bonus,
            item_has_unmodeled_effect_by_index: &item_has_unmodeled_effect_by_index,
            best_loadout_by_candidate: &best_loadout_by_candidate,
            best_outcome_by_candidate: &best_outcome_by_candidate,
            full_eval_count: &full_eval_count,
            unmodeled_rune_candidates_rejected: &unmodeled_rune_candidates_rejected,
            unmodeled_rune_candidates_penalized: &unmodeled_rune_candidates_penalized,
            unmodeled_item_effect_candidates_rejected: &unmodeled_item_effect_candidates_rejected,
            unmodeled_item_effect_candidates_penalized: &unmodeled_item_effect_candidates_penalized,
            full_cache: &full_cache,
            unique_scored_candidate_keys: &unique_scored_candidate_keys,
            search_type_counters: &search_type_counters,
            timeout_flag: timeout_flag.as_ref(),
            status: &mut status,
        });
    let ControlledChampionSearchExecutionResult {
        coverage_stage_diagnostics,
        ensemble_seeds,
        candidate_keys_generated,
        candidate_duplicates_pruned,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_random_promotions_done,
        unique_candidate_keys,
        controlled_champion_ranked,
        seed_best_scores,
        seed_hits_by_key,
        processed_candidates,
        total_candidates,
        timed_out,
        bleed_candidate_count,
        adaptive_candidate_count,
    } = search_execution;

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
    emit_controlled_champion_projection_dispatch(ControlledChampionProjectionDispatchContext {
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
        active_strategies: &active_strategies,
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
        bleed_candidate_count,
        adaptive_candidate_count,
        seed_best_scores,
        seed_hits_by_key: &seed_hits_by_key,
        objective_component_weights,
        objective_worst_case_weight,
        run_start,
        time_budget,
        popcorn_window,
        timed_out,
        processed_candidates,
        total_candidates,
        unmodeled_rune_candidates_rejected: &unmodeled_rune_candidates_rejected,
        unmodeled_rune_candidates_penalized: &unmodeled_rune_candidates_penalized,
        unmodeled_item_effect_candidates_rejected: &unmodeled_item_effect_candidates_rejected,
        unmodeled_item_effect_candidates_penalized: &unmodeled_item_effect_candidates_penalized,
        controlled_champion_ranked: &controlled_champion_ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
        item_pool: &item_pool,
        max_items,
        search_loadout_domain: search_loadout_domain.as_ref(),
        unique_scored_candidate_keys: unique_scored_candidate_keys.as_ref(),
        search_type_counters: search_type_counters.as_ref(),
        best_loadout_by_candidate: &best_loadout_by_candidate,
        best_outcome_by_candidate: &best_outcome_by_candidate,
        resolve_loadout_for_selection: &resolve_loadout_for_selection,
        controlled_champion_base_loadout: &controlled_champion_base_loadout,
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_base_raw: &controlled_champion_base_raw,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        objective_eval_ctx: &objective_eval_ctx,
        sim: &sim,
        urf: &urf,
        progress_state: &progress_state,
        timeout_flag: timeout_flag.as_ref(),
        status: &mut status,
    })
}

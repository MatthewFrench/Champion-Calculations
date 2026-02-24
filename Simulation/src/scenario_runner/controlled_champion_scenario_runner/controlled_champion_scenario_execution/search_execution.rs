use self::candidate_scoring_channels::{
    ControlledChampionCandidateScoringContext, evaluate_candidate_direct_for_strict_fallback,
    score_candidate_for_search_type,
};
use super::super::super::controlled_champion_strict_ranking_finalization::{
    ControlledChampionStrictRankingFinalizationContext, finalize_controlled_champion_strict_ranking,
};
use super::*;

pub(super) type EvaluateBuildWithBonusFn<'a> =
    dyn Fn(&[Item], &Stats, Option<&LoadoutSelection>) -> (f64, CombatOutcome) + Sync + 'a;

mod candidate_scoring_channels;

pub(super) struct ControlledChampionSearchExecutionContext<'a> {
    pub(super) search_quality_profile: SearchQualityProfile,
    pub(super) search_cfg: &'a BuildSearchConfig,
    pub(super) min_item_diff: usize,
    pub(super) item_pool: &'a [Item],
    pub(super) max_items: usize,
    pub(super) search_loadout_domain: &'a crate::data::LoadoutDomain,
    pub(super) controlled_champion_search_base_loadout_selection: &'a LoadoutSelection,
    pub(super) active_strategies: &'a [String],
    pub(super) objective_component_weights: ObjectiveComponentWeights,
    pub(super) sim_max_time_seconds: f64,
    pub(super) time_budget: Option<Duration>,
    pub(super) defer_hard_budget_until_coverage: bool,
    pub(super) hard_deadline_state: &'a Arc<Mutex<Option<Instant>>>,
    pub(super) current_deadline: &'a (dyn Fn() -> Option<Instant> + Sync),
    pub(super) coverage_stage_deadline: &'a (dyn Fn() -> Option<Instant> + Sync),
    pub(super) deadline_for_search_type: &'a (dyn Fn(&str) -> Option<Instant> + Sync),
    pub(super) record_score_progress: &'a (dyn Fn(f64) + Sync),
    pub(super) resolve_loadout_for_selection:
        &'a (dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout> + Sync),
    pub(super) evaluate_build_with_bonus: &'a EvaluateBuildWithBonusFn<'a>,
    pub(super) item_has_unmodeled_effect_by_index: &'a [bool],
    pub(super) best_loadout_by_candidate: &'a Arc<Mutex<ResolvedByCandidateMap>>,
    pub(super) best_outcome_by_candidate: &'a Arc<Mutex<OutcomeByCandidateMap>>,
    pub(super) full_eval_count: &'a AtomicUsize,
    pub(super) unmodeled_rune_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_rune_candidates_penalized: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_penalized: &'a AtomicUsize,
    pub(super) full_cache: &'a Arc<BlockingScoreCache>,
    pub(super) unique_scored_candidate_keys: &'a Arc<ShardedStringSet>,
    pub(super) search_type_counters: &'a Arc<HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>>,
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) status: &'a mut StatusReporter,
}

pub(super) struct ControlledChampionSearchExecutionResult {
    pub(super) coverage_stage_diagnostics: CoverageStageDiagnostics,
    pub(super) ensemble_seeds: usize,
    pub(super) candidate_keys_generated: usize,
    pub(super) candidate_duplicates_pruned: usize,
    pub(super) strict_seed_scored_candidates: usize,
    pub(super) strict_remaining_candidates: usize,
    pub(super) strict_non_finite_candidates: usize,
    pub(super) strict_candidates_skipped_timeout: usize,
    pub(super) strict_completion_percent: f64,
    pub(super) strict_random_promotions_done: usize,
    pub(super) unique_candidate_keys: Vec<BuildKey>,
    pub(super) controlled_champion_ranked: Vec<(BuildKey, f64)>,
    pub(super) seed_best_scores: Vec<f64>,
    pub(super) seed_hits_by_key: HashMap<BuildKey, usize>,
    pub(super) processed_candidates: usize,
    pub(super) total_candidates: usize,
    pub(super) timed_out: bool,
    pub(super) bleed_candidate_count: usize,
    pub(super) adaptive_candidate_count: usize,
}

pub(super) fn execute_controlled_champion_search_execution(
    context: ControlledChampionSearchExecutionContext<'_>,
) -> ControlledChampionSearchExecutionResult {
    let ControlledChampionSearchExecutionContext {
        search_quality_profile,
        search_cfg,
        min_item_diff,
        item_pool,
        max_items,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
        active_strategies,
        objective_component_weights,
        sim_max_time_seconds,
        time_budget,
        defer_hard_budget_until_coverage,
        hard_deadline_state,
        current_deadline,
        coverage_stage_deadline,
        deadline_for_search_type,
        record_score_progress,
        resolve_loadout_for_selection,
        evaluate_build_with_bonus,
        item_has_unmodeled_effect_by_index,
        best_loadout_by_candidate,
        best_outcome_by_candidate,
        full_eval_count,
        unmodeled_rune_candidates_rejected,
        unmodeled_rune_candidates_penalized,
        unmodeled_item_effect_candidates_rejected,
        unmodeled_item_effect_candidates_penalized,
        full_cache,
        unique_scored_candidate_keys,
        search_type_counters,
        timeout_flag,
        status,
    } = context;

    let candidate_scoring_context = ControlledChampionCandidateScoringContext {
        search_cfg,
        item_pool,
        max_items,
        time_budget,
        defer_hard_budget_until_coverage,
        hard_deadline_state,
        deadline_for_search_type,
        record_score_progress,
        resolve_loadout_for_selection,
        evaluate_build_with_bonus,
        item_has_unmodeled_effect_by_index,
        best_loadout_by_candidate,
        best_outcome_by_candidate,
        full_eval_count,
        unmodeled_rune_candidates_rejected,
        unmodeled_rune_candidates_penalized,
        unmodeled_item_effect_candidates_rejected,
        unmodeled_item_effect_candidates_penalized,
        full_cache,
        unique_scored_candidate_keys,
        search_type_counters,
        timeout_flag,
    };
    let full_score_for_search_type = |search_type: &str, candidate: &BuildKey| {
        score_candidate_for_search_type(search_type, candidate, &candidate_scoring_context)
    };
    let evaluate_candidate_direct = |candidate: &BuildKey| {
        evaluate_candidate_direct_for_strict_fallback(candidate, &candidate_scoring_context)
    };

    let full_search_params = FullLoadoutSearchParams {
        item_pool,
        max_items,
        loadout_domain: search_loadout_domain,
        base_loadout: controlled_champion_search_base_loadout_selection,
    };

    let coverage_stage = run_maximum_quality_coverage_stage(CoverageStageRunContext {
        search_quality_profile,
        search_cfg,
        min_item_diff,
        item_pool,
        search_loadout_domain,
        full_search_params,
        status,
        timeout_flag,
        coverage_stage_deadline,
        full_score_for_search_type: &full_score_for_search_type,
    });
    let coverage_stage_diagnostics = coverage_stage.diagnostics;
    let coverage_seed_candidates = coverage_stage.seed_candidates;

    let hard_deadline_value = || hard_deadline_state.lock().ok().and_then(|state| *state);
    if time_budget.is_some() && hard_deadline_value().is_none() {
        // Bootstrap one timed-phase simulation so staged search loops get a live deadline value.
        let mut bootstrap_seed = search_cfg.seed ^ 0xC0DE_DA7A_u64;
        let bootstrap_candidate = canonical_build_candidate(BuildKey {
            item_indices: random_valid_build(item_pool, max_items, &mut bootstrap_seed),
            loadout_selection: controlled_champion_search_base_loadout_selection.clone(),
        });
        let bootstrap_search_type = format!("seed_search:{}", search_cfg.strategy);
        let _ = full_score_for_search_type(bootstrap_search_type.as_str(), &bootstrap_candidate);
        if hard_deadline_value().is_none() {
            arm_time_budget_deadline_if_unset(
                hard_deadline_state,
                time_budget,
                defer_hard_budget_until_coverage,
                "seed_search:bootstrap",
            );
        }
    }

    let ensemble_seeds = search_cfg.ensemble_seeds.max(1);
    let seed_and_strict = run_seed_and_strict_ranking(SeedAndStrictRankingRunContext {
        search_cfg,
        active_strategies,
        item_pool,
        max_items,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
        full_search_params,
        coverage_seed_candidates: &coverage_seed_candidates,
        timeout_flag,
        status,
        current_deadline,
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
            item_pool,
            max_items,
            best_outcome_by_candidate,
            best_loadout_by_candidate,
            evaluate_candidate_direct: &evaluate_candidate_direct,
            sim_max_time_seconds,
            objective_component_weights,
            timeout_flag,
            current_deadline,
            seed_ranked: &seed_ranked,
            seed_top_sets: &seed_top_sets,
            ensemble_seed_top_k: search_cfg.ensemble_seed_top_k,
        },
    );
    let controlled_champion_ranked = strict_ranking_finalization.controlled_champion_ranked;
    let seed_best_scores = strict_ranking_finalization.seed_best_scores;
    let seed_hits_by_key = strict_ranking_finalization.seed_hits_by_key;
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;

    ControlledChampionSearchExecutionResult {
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
    }
}

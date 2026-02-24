use super::*;

pub(super) struct ControlledChampionSearchDiagnosticsProjection {
    pub(super) timed_out: bool,
    pub(super) progress_snapshot: SignificantProgressState,
    pub(super) seconds_since_last_significant_improvement: f64,
    pub(super) search_type_breakdown: Vec<SearchTypeBreakdown>,
    pub(super) effective_threads: usize,
    pub(super) seed_orchestration_parallel: bool,
    pub(super) portfolio_strategy_parallel: bool,
    pub(super) strategy_elites_parallel: bool,
    pub(super) estimated_total_candidate_space: Option<f64>,
    pub(super) unique_scored_candidates: usize,
    pub(super) estimated_run_space_coverage_percent: Option<f64>,
    pub(super) estimated_close_to_optimal_probability: Option<f64>,
    pub(super) estimated_close_to_optimal_probability_note: String,
}

pub(super) struct ControlledChampionSearchDiagnosticsProjectionContext<'a> {
    pub(super) timed_out: bool,
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) progress_state: &'a Arc<Mutex<SignificantProgressState>>,
    pub(super) run_start: Instant,
    pub(super) search_type_counters: &'a HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>,
    pub(super) ensemble_seeds: usize,
    pub(super) search_cfg: &'a BuildSearchConfig,
    pub(super) active_strategies: &'a [String],
    pub(super) item_pool: &'a [Item],
    pub(super) max_items: usize,
    pub(super) search_loadout_domain: &'a crate::data::LoadoutDomain,
    pub(super) unique_scored_candidate_keys: &'a ShardedStringSet,
}

pub(super) fn project_controlled_champion_search_diagnostics(
    context: ControlledChampionSearchDiagnosticsProjectionContext<'_>,
) -> ControlledChampionSearchDiagnosticsProjection {
    let ControlledChampionSearchDiagnosticsProjectionContext {
        timed_out,
        timeout_flag,
        progress_state,
        run_start,
        search_type_counters,
        ensemble_seeds,
        search_cfg,
        active_strategies,
        item_pool,
        max_items,
        search_loadout_domain,
        unique_scored_candidate_keys,
    } = context;

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

    ControlledChampionSearchDiagnosticsProjection {
        timed_out,
        progress_snapshot,
        seconds_since_last_significant_improvement,
        search_type_breakdown,
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel,
        estimated_total_candidate_space,
        unique_scored_candidates,
        estimated_run_space_coverage_percent,
        estimated_close_to_optimal_probability,
        estimated_close_to_optimal_probability_note,
    }
}

use super::*;

pub(super) struct ControlledChampionBuildAnalysisContext<'a> {
    pub(super) controlled_champion_ranked: &'a [(BuildKey, f64)],
    pub(super) top_x: usize,
    pub(super) min_item_diff: usize,
    pub(super) max_relative_gap_percent: f64,
    pub(super) item_pool: &'a [Item],
    pub(super) best_loadout_by_candidate: &'a Arc<Mutex<ResolvedByCandidateMap>>,
    pub(super) resolve_loadout_for_selection:
        &'a dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout>,
    pub(super) controlled_champion_base_loadout: &'a ResolvedLoadout,
    pub(super) controlled_champion_base: &'a ChampionBase,
    pub(super) controlled_champion_base_raw: &'a ChampionBase,
    pub(super) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(super) sim: &'a SimulationConfig,
    pub(super) urf: &'a UrfBuffs,
    pub(super) search_cfg: &'a BuildSearchConfig,
    pub(super) search_quality_profile: SearchQualityProfile,
    pub(super) objective_component_weights: ObjectiveComponentWeights,
    pub(super) objective_worst_case_weight: f64,
    pub(super) seed_hits_by_key: &'a HashMap<BuildKey, usize>,
    pub(super) ensemble_seeds: usize,
    pub(super) full_eval_count: &'a AtomicUsize,
    pub(super) full_cache: &'a BlockingScoreCache,
    pub(super) candidate_keys_generated: usize,
    pub(super) candidate_duplicates_pruned: usize,
    pub(super) unique_candidate_keys: &'a [BuildKey],
    pub(super) bleed_candidate_count: usize,
    pub(super) adaptive_candidate_count: usize,
    pub(super) strict_seed_scored_candidates: usize,
    pub(super) strict_remaining_candidates: usize,
    pub(super) strict_non_finite_candidates: usize,
    pub(super) strict_candidates_skipped_timeout: usize,
    pub(super) strict_completion_percent: f64,
    pub(super) strict_random_promotions_done: usize,
    pub(super) unmodeled_rune_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_rune_candidates_penalized: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_penalized: &'a AtomicUsize,
    pub(super) unique_scored_candidates: usize,
    pub(super) time_budget: Option<Duration>,
    pub(super) popcorn_window_seconds: Option<f64>,
    pub(super) popcorn_min_relative_improvement_percent: f64,
    pub(super) progress_snapshot: SignificantProgressState,
    pub(super) seconds_since_last_significant_improvement: f64,
    pub(super) search_type_breakdown: Vec<SearchTypeBreakdown>,
    pub(super) seed_best_scores: Vec<f64>,
    pub(super) estimated_total_candidate_space: Option<f64>,
    pub(super) estimated_run_space_coverage_percent: Option<f64>,
    pub(super) estimated_close_to_optimal_probability: Option<f64>,
    pub(super) estimated_close_to_optimal_probability_note: String,
    pub(super) coverage_stage_diagnostics: &'a CoverageStageDiagnostics,
    pub(super) run_start: Instant,
    pub(super) timed_out: bool,
    pub(super) processed_candidates: usize,
    pub(super) total_candidates: usize,
    pub(super) enemy_build_scenarios: &'a [EnemyBuildScenario],
    pub(super) raw_enemy_bases: &'a HashMap<String, ChampionBase>,
    pub(super) controlled_champion_best_build: &'a [Item],
    pub(super) controlled_champion_loadout: &'a ResolvedLoadout,
    pub(super) loadout_candidates_count: usize,
    pub(super) loadout_finalists_count: usize,
    pub(super) effective_threads: usize,
    pub(super) seed_orchestration_parallel: bool,
    pub(super) portfolio_strategy_parallel: bool,
    pub(super) strategy_elites_parallel: bool,
}

pub(super) struct ControlledChampionBuildAnalysisOutput {
    pub(super) diverse_top_builds: Vec<(Vec<Item>, f64)>,
    pub(super) diverse_top_keys: Vec<BuildKey>,
    pub(super) build_confidence: Vec<BuildConfidence>,
    pub(super) metrics_by_key: HashMap<BuildKey, BuildMetrics>,
    pub(super) pareto_front: HashSet<BuildKey>,
    pub(super) diagnostics: SearchDiagnostics,
    pub(super) build_order_results: Vec<BuildOrderResult>,
    pub(super) controlled_champion_end_stats: Stats,
    pub(super) stack_notes: Vec<String>,
}

fn candidate_bonus_stats_for_key(
    candidate: &BuildKey,
    resolved_by_candidate_snapshot: &ResolvedByCandidateMap,
    resolve_loadout_for_selection: &dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout>,
    controlled_champion_base_loadout: &ResolvedLoadout,
) -> Stats {
    resolved_by_candidate_snapshot
        .get(candidate)
        .map(|resolved| resolved.bonus_stats.clone())
        .or_else(|| {
            resolve_loadout_for_selection(&candidate.loadout_selection)
                .map(|resolved| resolved.bonus_stats)
        })
        .unwrap_or_else(|| controlled_champion_base_loadout.bonus_stats.clone())
}

pub(super) fn analyze_controlled_champion_build_results(
    context: ControlledChampionBuildAnalysisContext<'_>,
) -> ControlledChampionBuildAnalysisOutput {
    let ControlledChampionBuildAnalysisContext {
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
        search_cfg,
        search_quality_profile,
        objective_component_weights,
        objective_worst_case_weight,
        seed_hits_by_key,
        ensemble_seeds,
        full_eval_count,
        full_cache,
        candidate_keys_generated,
        candidate_duplicates_pruned,
        unique_candidate_keys,
        bleed_candidate_count,
        adaptive_candidate_count,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_random_promotions_done,
        unmodeled_rune_candidates_rejected,
        unmodeled_rune_candidates_penalized,
        unmodeled_item_effect_candidates_rejected,
        unmodeled_item_effect_candidates_penalized,
        unique_scored_candidates,
        time_budget,
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
        progress_snapshot,
        seconds_since_last_significant_improvement,
        search_type_breakdown,
        seed_best_scores,
        estimated_total_candidate_space,
        estimated_run_space_coverage_percent,
        estimated_close_to_optimal_probability,
        estimated_close_to_optimal_probability_note,
        coverage_stage_diagnostics,
        run_start,
        timed_out,
        processed_candidates,
        total_candidates,
        enemy_build_scenarios,
        raw_enemy_bases,
        controlled_champion_best_build,
        controlled_champion_loadout,
        loadout_candidates_count,
        loadout_finalists_count,
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel,
    } = context;

    let diverse_top_raw = select_diverse_top_candidates(
        controlled_champion_ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
    );
    let diverse_top_keys = diverse_top_raw
        .iter()
        .map(|(candidate, _)| candidate.clone())
        .collect::<Vec<_>>();
    let diverse_top_builds = diverse_top_raw
        .iter()
        .map(|(candidate, score)| {
            (
                build_from_indices(item_pool, &candidate.item_indices),
                *score,
            )
        })
        .collect::<Vec<_>>();
    let resolved_by_candidate_snapshot = best_loadout_by_candidate
        .lock()
        .map(|map| map.clone())
        .unwrap_or_default();

    let mut metrics_by_key = HashMap::new();
    for (candidate, score) in controlled_champion_ranked {
        let candidate_bonus_stats = candidate_bonus_stats_for_key(
            candidate,
            &resolved_by_candidate_snapshot,
            resolve_loadout_for_selection,
            controlled_champion_base_loadout,
        );
        metrics_by_key.insert(
            candidate.clone(),
            compute_build_metrics_for_candidate(
                candidate,
                item_pool,
                controlled_champion_base,
                &candidate_bonus_stats,
                controlled_champion_stack_overrides,
                sim,
                *score,
            ),
        );
    }
    let pareto_front = candidate_pareto_front_keys(&metrics_by_key);

    let build_confidence = controlled_champion_ranked
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
        strategy_summary: search_strategy_summary(search_cfg),
        search_quality_profile: match search_quality_profile {
            SearchQualityProfile::Fast => "fast".to_string(),
            SearchQualityProfile::Balanced => "balanced".to_string(),
            SearchQualityProfile::MaximumQuality => "maximum_quality".to_string(),
        },
        effective_seed: search_cfg.seed,
        ensemble_seeds,
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel,
        objective_survival_weight: objective_component_weights.survival,
        objective_damage_weight: objective_component_weights.damage,
        objective_healing_weight: objective_component_weights.healing,
        objective_enemy_kills_weight: objective_component_weights.enemy_kills,
        objective_invulnerable_seconds_weight: objective_component_weights.invulnerable_seconds,
        full_evaluations: full_eval_count.load(AtomicOrdering::Relaxed),
        full_cache_hits: full_cache.hits(),
        full_cache_misses: full_cache.misses(),
        full_cache_waits: full_cache.waits(),
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
        strict_heuristic_ordering_enabled: search_cfg.strict_ranking_enable_heuristic_ordering,
        strict_ranking_rune_signal_weight: search_cfg.strict_ranking_rune_signal_weight,
        strict_ranking_shard_signal_weight: search_cfg.strict_ranking_shard_signal_weight,
        strict_random_promotions_done,
        unmodeled_rune_hard_gate: search_cfg.unmodeled_rune_hard_gate,
        unmodeled_rune_penalty_per_rune: search_cfg.unmodeled_rune_penalty_per_rune,
        unmodeled_rune_candidates_rejected: unmodeled_rune_candidates_rejected
            .load(AtomicOrdering::Relaxed),
        unmodeled_rune_candidates_penalized: unmodeled_rune_candidates_penalized
            .load(AtomicOrdering::Relaxed),
        unmodeled_item_effect_hard_gate: search_cfg.unmodeled_item_effect_hard_gate,
        unmodeled_item_effect_penalty_per_item: search_cfg.unmodeled_item_effect_penalty_per_item,
        unmodeled_item_effect_candidates_rejected: unmodeled_item_effect_candidates_rejected
            .load(AtomicOrdering::Relaxed),
        unmodeled_item_effect_candidates_penalized: unmodeled_item_effect_candidates_penalized
            .load(AtomicOrdering::Relaxed),
        unique_scored_candidates,
        time_budget_seconds: time_budget.map(|d| d.as_secs_f64()),
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
        significant_improvement_events: progress_snapshot.significant_events,
        best_significant_score: progress_snapshot
            .best_significant_score
            .is_finite()
            .then_some(progress_snapshot.best_significant_score),
        seconds_since_last_significant_improvement: Some(
            seconds_since_last_significant_improvement,
        ),
        search_type_breakdown,
        estimated_total_candidate_space,
        estimated_run_space_coverage_percent,
        estimated_close_to_optimal_probability,
        estimated_close_to_optimal_probability_note,
        coverage_stage_enabled: coverage_stage_diagnostics.enabled,
        coverage_stage_elapsed_seconds: coverage_stage_diagnostics.elapsed_seconds,
        coverage_stage_assets_total: coverage_stage_diagnostics.assets_total,
        coverage_stage_assets_covered: coverage_stage_diagnostics.assets_covered,
        coverage_stage_seed_candidates: coverage_stage_diagnostics.seed_candidates,
        coverage_stage_seed_candidates_unique: coverage_stage_diagnostics.seed_candidates_unique,
        coverage_stage_incomplete: coverage_stage_diagnostics.coverage_incomplete,
        coverage_stage_warning: coverage_stage_diagnostics.coverage_warning.clone(),
        elapsed_seconds: run_start.elapsed().as_secs_f64(),
        total_run_seconds: 0.0,
        timed_out,
        processed_candidates,
        total_candidates,
        seed_best_scores,
    };

    let confidence_by_key = build_confidence
        .iter()
        .map(|c| (c.key.clone(), c.clone()))
        .collect::<HashMap<_, _>>();
    let mut order_input = diverse_top_raw
        .iter()
        .filter_map(|(candidate, _)| {
            let robust = confidence_by_key
                .get(candidate)
                .map(|c| c.robustness == "robust")
                .unwrap_or(false);
            let pareto = pareto_front.contains(candidate);
            if robust || pareto {
                Some((
                    candidate.clone(),
                    build_from_indices(item_pool, &candidate.item_indices),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if order_input.is_empty() {
        order_input = diverse_top_raw
            .iter()
            .take(2)
            .map(|(candidate, _)| {
                (
                    candidate.clone(),
                    build_from_indices(item_pool, &candidate.item_indices),
                )
            })
            .collect::<Vec<_>>();
    }

    let build_order_results = order_input
        .iter()
        .map(|(candidate, build)| {
            let candidate_bonus_stats = candidate_bonus_stats_for_key(
                candidate,
                &resolved_by_candidate_snapshot,
                resolve_loadout_for_selection,
                controlled_champion_base_loadout,
            );
            let build_order_ctx = BuildOrderEvalContext {
                controlled_champion_base_raw,
                controlled_champion_bonus_stats: &candidate_bonus_stats,
                controlled_champion_stack_overrides,
                enemy_build_scenarios,
                raw_enemy_bases,
                sim,
                urf,
                objective_weights: objective_component_weights,
                multi_scenario_worst_weight: objective_worst_case_weight,
            };
            optimize_build_order(build, &build_order_ctx)
        })
        .collect::<Vec<_>>();
    let best_order_acquired_map = build_order_results
        .first()
        .map(|br| acquisition_level_map(&br.ordered_items, &br.acquired_levels));

    let best_effective_item_stats = compute_effective_item_stats_for_build(
        controlled_champion_base,
        controlled_champion_best_build,
        &controlled_champion_loadout.bonus_stats,
        sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
        Some(controlled_champion_stack_overrides),
    );
    let controlled_champion_end_stats =
        compute_champion_final_stats(controlled_champion_base, &best_effective_item_stats);
    let stack_notes = build_stack_notes(
        controlled_champion_best_build,
        controlled_champion_base,
        sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
        Some(controlled_champion_stack_overrides),
    );

    ControlledChampionBuildAnalysisOutput {
        diverse_top_builds,
        diverse_top_keys,
        build_confidence,
        metrics_by_key,
        pareto_front,
        diagnostics,
        build_order_results,
        controlled_champion_end_stats,
        stack_notes,
    }
}

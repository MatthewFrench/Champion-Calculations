use super::controlled_champion_result_artifact_writing::{
    ControlledChampionResultArtifactWritingContext, write_controlled_champion_result_artifacts,
};
use super::controlled_champion_result_build_analysis::{
    ControlledChampionBuildAnalysisContext, analyze_controlled_champion_build_results,
};
use super::*;

pub(super) struct ControlledChampionResultReportingContext<'a> {
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
    pub(super) effective_threads: usize,
    pub(super) seed_orchestration_parallel: bool,
    pub(super) portfolio_strategy_parallel: bool,
    pub(super) strategy_elites_parallel: bool,
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
    pub(super) unique_scored_candidates: usize,
    pub(super) bleed_candidate_count: usize,
    pub(super) adaptive_candidate_count: usize,
    pub(super) search_type_breakdown: Vec<SearchTypeBreakdown>,
    pub(super) seed_best_scores: Vec<f64>,
    pub(super) seed_hits_by_key: &'a HashMap<BuildKey, usize>,
    pub(super) objective_component_weights: ObjectiveComponentWeights,
    pub(super) objective_worst_case_weight: f64,
    pub(super) run_start: Instant,
    pub(super) time_budget: Option<Duration>,
    pub(super) popcorn_window: Option<Duration>,
    pub(super) progress_snapshot: SignificantProgressState,
    pub(super) seconds_since_last_significant_improvement: f64,
    pub(super) timed_out: bool,
    pub(super) processed_candidates: usize,
    pub(super) total_candidates: usize,
    pub(super) estimated_total_candidate_space: Option<f64>,
    pub(super) estimated_run_space_coverage_percent: Option<f64>,
    pub(super) estimated_close_to_optimal_probability: Option<f64>,
    pub(super) estimated_close_to_optimal_probability_note: String,
    pub(super) unmodeled_rune_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_rune_candidates_penalized: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_penalized: &'a AtomicUsize,
    pub(super) controlled_champion_best_build: &'a [Item],
    pub(super) controlled_champion_best_score: f64,
    pub(super) controlled_champion_best_outcome: CombatOutcome,
    pub(super) best_cap_survivor: bool,
    pub(super) controlled_champion_loadout: &'a ResolvedLoadout,
    pub(super) controlled_champion_runtime_loadout_selection: &'a LoadoutSelection,
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
    pub(super) best_score_breakdown: ObjectiveScoreBreakdown,
    pub(super) status: &'a mut StatusReporter,
}

pub(super) fn emit_controlled_champion_result_reporting(
    context: ControlledChampionResultReportingContext<'_>,
) -> Result<()> {
    let ControlledChampionResultReportingContext {
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
        controlled_champion_best_build,
        controlled_champion_best_score,
        controlled_champion_best_outcome,
        best_cap_survivor,
        controlled_champion_loadout,
        controlled_champion_runtime_loadout_selection,
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
    } = context;

    println!("Enemy builds (URF preset defaults):");
    for (enemy, build, _) in enemy_builds {
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
    for profile in enemy_derived_combat_stats {
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
    for note in enemy_similarity_notes {
        println!("- Warning: {}", note);
    }

    println!(
        "\n{} best build (optimized for objective):",
        controlled_champion_name
    );
    println!("- Search strategy: {}", search_strategy_summary(search_cfg));
    let loadout_candidates_count = unique_loadout_selection_count(unique_candidate_keys);
    let loadout_finalists_count =
        unique_loadout_selection_count_from_ranked(controlled_champion_ranked);
    println!(
        "- Loadout candidates/finalists: {}/{}",
        loadout_candidates_count, loadout_finalists_count
    );
    println!("- Effective search seed: {}", search_cfg.seed);
    if coverage_stage_diagnostics.enabled {
        println!(
            "- Coverage stage (pre-budget): {:.2}s | assets covered {}/{} | seeded candidates {}/{}",
            coverage_stage_diagnostics.elapsed_seconds,
            coverage_stage_diagnostics.assets_covered,
            coverage_stage_diagnostics.assets_total,
            coverage_stage_diagnostics.seed_candidates_unique,
            coverage_stage_diagnostics.seed_candidates
        );
        if coverage_stage_diagnostics.coverage_incomplete
            && !coverage_stage_diagnostics.coverage_warning.is_empty()
        {
            println!(
                "- Coverage warning: {}",
                coverage_stage_diagnostics.coverage_warning
            );
        }
    }
    println!(
        "- Candidate evaluations (full): {}",
        full_eval_count.load(AtomicOrdering::Relaxed)
    );
    println!(
        "- In-memory full-evaluation cache (hits/misses/waits): {}/{}/{}",
        full_cache.hits(),
        full_cache.misses(),
        full_cache.waits()
    );
    println!("- Ensemble seeds: {}", ensemble_seeds);
    println!(
        "- Parallelism: threads {} | seed orchestration parallel {} | portfolio strategy parallel {} | strategy-elites parallel {}",
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel
    );
    println!(
        "- Enemy scenarios in objective: {}",
        enemy_build_scenarios.len()
    );
    println!(
        "- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): {:.2}/{:.2}/{:.2}/{:.2}/{:.2}",
        objective_component_weights.survival,
        objective_component_weights.damage,
        objective_component_weights.healing,
        objective_component_weights.enemy_kills,
        objective_component_weights.invulnerable_seconds
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
    if let Some(window) = popcorn_window {
        println!(
            "- Popcorn mode: window {:.1}s | significant threshold {:.2}% of last best score | significant events {} | seconds since last significant improvement {:.1}",
            window.as_secs_f64(),
            popcorn_min_relative_improvement_percent,
            progress_snapshot.significant_events,
            seconds_since_last_significant_improvement
        );
    }
    println!(
        "- Unique strict candidates: {}",
        unique_candidate_keys.len()
    );
    println!(
        "- Strict candidate ordering: heuristic {} (rune/shard weights {:.2}/{:.2}), exploration promotions {}",
        search_cfg.strict_ranking_enable_heuristic_ordering,
        search_cfg.strict_ranking_rune_signal_weight,
        search_cfg.strict_ranking_shard_signal_weight,
        strict_random_promotions_done
    );
    println!(
        "- Unmodeled rune gate: hard gate {} | penalty per rune {:.4} | rejected {} | penalized {}",
        search_cfg.unmodeled_rune_hard_gate,
        search_cfg.unmodeled_rune_penalty_per_rune,
        unmodeled_rune_candidates_rejected.load(AtomicOrdering::Relaxed),
        unmodeled_rune_candidates_penalized.load(AtomicOrdering::Relaxed)
    );
    println!(
        "- Unmodeled item-effect gate: hard gate {} | penalty per item {:.4} | rejected {} | penalized {}",
        search_cfg.unmodeled_item_effect_hard_gate,
        search_cfg.unmodeled_item_effect_penalty_per_item,
        unmodeled_item_effect_candidates_rejected.load(AtomicOrdering::Relaxed),
        unmodeled_item_effect_candidates_penalized.load(AtomicOrdering::Relaxed)
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
    println!(
        "- Unique scored candidates (all search stages): {}",
        unique_scored_candidates
    );
    if let Some(total) = estimated_total_candidate_space {
        println!("- Estimated total legal candidate space: {:.0}", total);
    }
    if let Some(run_coverage) = estimated_run_space_coverage_percent {
        println!(
            "- Estimated legal-space coverage (this run): {}",
            format_percent_display(run_coverage)
        );
    }
    if let Some(probability) = estimated_close_to_optimal_probability {
        println!(
            "- Estimated closeness probability (top 0.000001% heuristic): {:.2}% | {}",
            probability * 100.0,
            estimated_close_to_optimal_probability_note
        );
    }
    println!("- Bleed candidates injected: {}", bleed_candidate_count);
    println!(
        "- Adaptive candidates injected: {}",
        adaptive_candidate_count
    );
    if !search_type_breakdown.is_empty() {
        println!("- Search-type simulation breakdown:");
        for entry in &search_type_breakdown {
            println!(
                "  - {} => score requests {}, new simulations {}",
                entry.name, entry.score_requests, entry.new_simulations
            );
        }
    }
    println!(
        "- Items: {}",
        controlled_champion_best_build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Objective score: {:.4}", controlled_champion_best_score);
    println!(
        "- Time alive / damage dealt / healing done / enemy kills / invulnerable seconds: {:.2}s / {:.1} / {:.1} / {} / {:.2}",
        controlled_champion_best_outcome.time_alive_seconds,
        controlled_champion_best_outcome.damage_dealt,
        controlled_champion_best_outcome.healing_done,
        controlled_champion_best_outcome.enemy_kills,
        controlled_champion_best_outcome.invulnerable_seconds
    );
    println!("- Cap survivor: {}", best_cap_survivor);
    if !controlled_champion_loadout.selection_labels.is_empty() {
        println!("\n{} rune page:", controlled_champion_name);
        for s in &controlled_champion_loadout.selection_labels {
            println!("- {}", s);
        }
    }

    let mut analysis =
        analyze_controlled_champion_build_results(ControlledChampionBuildAnalysisContext {
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
        });

    write_controlled_champion_result_artifacts(
        ControlledChampionResultArtifactWritingContext {
            scenario_path,
            controlled_champion_name,
            search_quality_profile,
            max_runtime_seconds,
            popcorn_window_seconds,
            popcorn_min_relative_improvement_percent,
            report_path_override,
            controlled_champion_base,
            controlled_champion_best_build,
            controlled_champion_best_score,
            controlled_champion_best_outcome,
            controlled_champion_loadout,
            controlled_champion_runtime_loadout_selection,
            controlled_champion_stack_overrides,
            best_score_breakdown,
            enemy_builds,
            enemy_loadout,
            enemy_derived_combat_stats,
            enemy_similarity_notes,
            enemy_presets_used,
            sim,
            urf,
            run_start,
            status,
            processed_candidates,
            total_candidates,
        },
        &mut analysis,
    )
}

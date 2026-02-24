use super::*;

pub(in crate::scenario_runner) fn emit_controlled_champion_console_summary(
    context: &ControlledChampionResultReportingContext<'_>,
) -> (usize, usize) {
    println!("Enemy builds (URF preset defaults):");
    for (enemy, build, _) in context.enemy_builds {
        println!(
            "- {}: {}",
            enemy.name,
            build
                .iter()
                .map(|item| item.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );
        if let Some(preset) = context.enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            println!(
                "  source: {} (last checked {})",
                preset.source_url, preset.last_checked
            );
        }
    }
    println!("\nEnemy derived combat profiles:");
    for profile in context.enemy_derived_combat_stats {
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
    for note in context.enemy_similarity_notes {
        println!("- Warning: {}", note);
    }

    println!(
        "\n{} best build (optimized for objective):",
        context.controlled_champion_name
    );
    println!(
        "- Search strategy: {}",
        search_strategy_summary(context.search_cfg)
    );
    let loadout_candidates_count = unique_loadout_selection_count(context.unique_candidate_keys);
    let loadout_finalists_count =
        unique_loadout_selection_count_from_ranked(context.controlled_champion_ranked);
    println!(
        "- Loadout candidates/finalists: {}/{}",
        loadout_candidates_count, loadout_finalists_count
    );
    println!("- Effective search seed: {}", context.search_cfg.seed);
    if context.coverage_stage_diagnostics.enabled {
        println!(
            "- Coverage stage (pre-budget): {:.2}s | assets covered {}/{} | seeded candidates {}/{}",
            context.coverage_stage_diagnostics.elapsed_seconds,
            context.coverage_stage_diagnostics.assets_covered,
            context.coverage_stage_diagnostics.assets_total,
            context.coverage_stage_diagnostics.seed_candidates_unique,
            context.coverage_stage_diagnostics.seed_candidates
        );
        if context.coverage_stage_diagnostics.coverage_incomplete
            && !context
                .coverage_stage_diagnostics
                .coverage_warning
                .is_empty()
        {
            println!(
                "- Coverage warning: {}",
                context.coverage_stage_diagnostics.coverage_warning
            );
        }
    }
    println!(
        "- Candidate evaluations (full): {}",
        context.full_eval_count.load(AtomicOrdering::Relaxed)
    );
    println!(
        "- In-memory full-evaluation cache (hits/misses/waits): {}/{}/{}",
        context.full_cache.hits(),
        context.full_cache.misses(),
        context.full_cache.waits()
    );
    println!("- Ensemble seeds: {}", context.ensemble_seeds);
    println!(
        "- Parallelism: threads {} | seed orchestration parallel {} | portfolio strategy parallel {} | strategy-elites parallel {}",
        context.effective_threads,
        context.seed_orchestration_parallel,
        context.portfolio_strategy_parallel,
        context.strategy_elites_parallel
    );
    println!(
        "- Enemy scenarios in objective: {}",
        context.enemy_build_scenarios.len()
    );
    println!(
        "- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): {:.2}/{:.2}/{:.2}/{:.2}/{:.2}",
        context.objective_component_weights.survival,
        context.objective_component_weights.damage,
        context.objective_component_weights.healing,
        context.objective_component_weights.enemy_kills,
        context.objective_component_weights.invulnerable_seconds
    );
    if let Some(budget) = context.time_budget {
        println!(
            "- Time budget: {:.1}s | elapsed: {:.1}s | timed_out: {} | progress: {}/{}",
            budget.as_secs_f64(),
            context.run_start.elapsed().as_secs_f64(),
            context.timed_out,
            context.processed_candidates,
            context.total_candidates
        );
    }
    if let Some(window) = context.popcorn_window {
        println!(
            "- Popcorn mode: window {:.1}s | significant threshold {:.2}% of last best score | significant events {} | seconds since last significant improvement {:.1}",
            window.as_secs_f64(),
            context.popcorn_min_relative_improvement_percent,
            context.progress_snapshot.significant_events,
            context.seconds_since_last_significant_improvement
        );
    }
    println!(
        "- Unique strict candidates: {}",
        context.unique_candidate_keys.len()
    );
    println!(
        "- Strict candidate ordering: heuristic {} (rune/shard weights {:.2}/{:.2}), exploration promotions {}",
        context.search_cfg.strict_ranking_enable_heuristic_ordering,
        context.search_cfg.strict_ranking_rune_signal_weight,
        context.search_cfg.strict_ranking_shard_signal_weight,
        context.strict_random_promotions_done
    );
    println!(
        "- Unmodeled rune gate: hard gate {} | penalty per rune {:.4} | rejected {} | penalized {}",
        context.search_cfg.unmodeled_rune_hard_gate,
        context.search_cfg.unmodeled_rune_penalty_per_rune,
        context
            .unmodeled_rune_candidates_rejected
            .load(AtomicOrdering::Relaxed),
        context
            .unmodeled_rune_candidates_penalized
            .load(AtomicOrdering::Relaxed)
    );
    println!(
        "- Unmodeled item-effect gate: hard gate {} | penalty per item {:.4} | rejected {} | penalized {}",
        context.search_cfg.unmodeled_item_effect_hard_gate,
        context.search_cfg.unmodeled_item_effect_penalty_per_item,
        context
            .unmodeled_item_effect_candidates_rejected
            .load(AtomicOrdering::Relaxed),
        context
            .unmodeled_item_effect_candidates_penalized
            .load(AtomicOrdering::Relaxed)
    );
    println!(
        "- Candidate keys generated / duplicates pruned: {}/{}",
        context.candidate_keys_generated, context.candidate_duplicates_pruned
    );
    println!(
        "- Strict completion: {:.1}% (processed {}/{}, timeout-skipped {}, non-finite {})",
        context.strict_completion_percent,
        context.processed_candidates.min(context.total_candidates),
        context.total_candidates,
        context.strict_candidates_skipped_timeout,
        context.strict_non_finite_candidates
    );
    println!(
        "- Unique scored candidates (all search stages): {}",
        context.unique_scored_candidates
    );
    if let Some(total) = context.estimated_total_candidate_space {
        println!("- Estimated total legal candidate space: {:.0}", total);
    }
    if let Some(run_coverage) = context.estimated_run_space_coverage_percent {
        println!(
            "- Estimated legal-space coverage (this run): {}",
            format_percent_display(run_coverage)
        );
    }
    if let Some(probability) = context.estimated_close_to_optimal_probability {
        println!(
            "- Estimated closeness probability (top 0.000001% heuristic): {:.2}% | {}",
            probability * 100.0,
            context.estimated_close_to_optimal_probability_note
        );
    }
    println!(
        "- Bleed candidates injected: {}",
        context.bleed_candidate_count
    );
    println!(
        "- Adaptive candidates injected: {}",
        context.adaptive_candidate_count
    );
    if !context.search_type_breakdown.is_empty() {
        println!("- Search-type simulation breakdown:");
        for entry in &context.search_type_breakdown {
            println!(
                "  - {} => score requests {}, new simulations {}",
                entry.name, entry.score_requests, entry.new_simulations
            );
        }
    }
    println!(
        "- Items: {}",
        context
            .controlled_champion_best_build
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "- Objective score: {:.4}",
        context.controlled_champion_best_score
    );
    println!(
        "- Time alive / damage dealt / healing done / enemy kills / invulnerable seconds: {:.2}s / {:.1} / {:.1} / {} / {:.2}",
        context.controlled_champion_best_outcome.time_alive_seconds,
        context.controlled_champion_best_outcome.damage_dealt,
        context.controlled_champion_best_outcome.healing_done,
        context.controlled_champion_best_outcome.enemy_kills,
        context
            .controlled_champion_best_outcome
            .invulnerable_seconds
    );
    println!("- Cap survivor: {}", context.best_cap_survivor);
    if !context
        .controlled_champion_loadout
        .selection_labels
        .is_empty()
    {
        println!("\n{} rune page:", context.controlled_champion_name);
        for label in &context.controlled_champion_loadout.selection_labels {
            println!("- {}", label);
        }
    }

    (loadout_candidates_count, loadout_finalists_count)
}

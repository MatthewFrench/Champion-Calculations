use super::*;
use crate::SearchDiagnostics;

pub(super) fn append_search_diagnostics_section(
    content: &mut String,
    diagnostics: &SearchDiagnostics,
) {
    let (seed_mean, seed_std) = mean_std(&diagnostics.seed_best_scores);
    let processed_candidates = diagnostics
        .processed_candidates
        .min(diagnostics.total_candidates);
    let total_score_requests = diagnostics
        .search_type_breakdown
        .iter()
        .map(|breakdown| breakdown.score_requests)
        .sum::<usize>();
    content.push_str("## Search Diagnostics\n");
    content.push_str(&format!(
        "- Strategy: `{}`\n- Search quality profile: `{}`\n- Enemy scenarios: `{}`\n- Loadout:\n  - Candidates: `{}`\n  - Finalists: `{}`\n- Ensemble seeds: `{}`\n- Parallelism:\n  - Threads: `{}`\n  - Seed orchestration parallel: `{}`\n  - Portfolio parallel: `{}`\n  - Strategy-elites parallel: `{}`\n- Objective weights:\n  - survival: `{:.2}`\n  - damage: `{:.2}`\n  - healing: `{:.2}`\n  - enemy_kills: `{:.2}`\n  - invulnerable_seconds: `{:.2}`\n- Simulations executed (new full combat runs): `{}`\n- Unique scored candidates (all search stages): `{}`\n- Total score requests (all search stages): `{}`\n- In-memory full-evaluation cache:\n  - Hits: `{}`\n  - Misses: `{}`\n  - Waits: `{}`\n- Candidate key generation:\n  - Generated: `{}`\n  - Duplicate-pruned: `{}`\n  - Unique: `{}`\n- Strict candidate progression:\n  - Seed-scored: `{}`\n  - Remaining: `{}`\n  - Processed: `{}`\n- Strict stage:\n  - Non-finite: `{}`\n  - Timeout-skipped: `{}`\n  - Completion: `{:.1}%`\n- Strict ordering heuristic:\n  - Enabled: `{}`\n  - Rune signal weight: `{:.2}`\n  - Shard signal weight: `{:.2}`\n  - Exploration promotions: `{}`\n- Bleed candidates injected: `{}`\n- Adaptive candidates injected: `{}`\n- Seed-best stats:\n  - Mean: `{}`\n  - Stddev: `{}`\n- Search elapsed time: `{:.2}s`\n- Total run time (end-to-end): `{:.2}s`\n\n",
        diagnostics.strategy_summary,
        diagnostics.search_quality_profile,
        format_usize_with_commas(diagnostics.scenario_count),
        format_usize_with_commas(diagnostics.loadout_candidates),
        format_usize_with_commas(diagnostics.loadout_finalists),
        format_usize_with_commas(diagnostics.ensemble_seeds),
        format_usize_with_commas(diagnostics.effective_threads),
        diagnostics.seed_orchestration_parallel,
        diagnostics.portfolio_strategy_parallel,
        diagnostics.strategy_elites_parallel,
        diagnostics.objective_survival_weight,
        diagnostics.objective_damage_weight,
        diagnostics.objective_healing_weight,
        diagnostics.objective_enemy_kills_weight,
        diagnostics.objective_invulnerable_seconds_weight,
        format_usize_with_commas(diagnostics.full_evaluations),
        format_usize_with_commas(diagnostics.unique_scored_candidates),
        format_usize_with_commas(total_score_requests),
        format_usize_with_commas(diagnostics.full_cache_hits),
        format_usize_with_commas(diagnostics.full_cache_misses),
        format_usize_with_commas(diagnostics.full_cache_waits),
        format_usize_with_commas(diagnostics.candidate_keys_generated),
        format_usize_with_commas(diagnostics.candidate_duplicates_pruned),
        format_usize_with_commas(diagnostics.unique_candidate_builds),
        format_usize_with_commas(diagnostics.strict_seed_scored_candidates),
        format_usize_with_commas(diagnostics.strict_remaining_candidates),
        format_usize_with_commas(processed_candidates),
        format_usize_with_commas(diagnostics.strict_non_finite_candidates),
        format_usize_with_commas(diagnostics.strict_candidates_skipped_timeout),
        diagnostics.strict_completion_percent,
        diagnostics.strict_heuristic_ordering_enabled,
        diagnostics.strict_ranking_rune_signal_weight,
        diagnostics.strict_ranking_shard_signal_weight,
        format_usize_with_commas(diagnostics.strict_random_promotions_done),
        format_usize_with_commas(diagnostics.bleed_candidates_injected),
        format_usize_with_commas(diagnostics.adaptive_candidates_injected),
        format_f64_with_commas(seed_mean, 2),
        format_f64_with_commas(seed_std, 3),
        diagnostics.elapsed_seconds,
        diagnostics.total_run_seconds
    ));
    content.push_str(&format!(
        "- Effective seed: `{}`\n",
        diagnostics.effective_seed
    ));
    content.push_str(&format!(
        "- Unmodeled rune gate:\n  - Hard gate: `{}`\n  - Penalty per rune: `{:.4}`\n  - Rejected: `{}`\n  - Penalized: `{}`\n",
        diagnostics.unmodeled_rune_hard_gate,
        diagnostics.unmodeled_rune_penalty_per_rune,
        format_usize_with_commas(diagnostics.unmodeled_rune_candidates_rejected),
        format_usize_with_commas(diagnostics.unmodeled_rune_candidates_penalized)
    ));
    content.push_str(&format!(
        "- Unmodeled item-effect gate:\n  - Hard gate: `{}`\n  - Penalty per item: `{:.4}`\n  - Rejected: `{}`\n  - Penalized: `{}`\n",
        diagnostics.unmodeled_item_effect_hard_gate,
        diagnostics.unmodeled_item_effect_penalty_per_item,
        format_usize_with_commas(diagnostics.unmodeled_item_effect_candidates_rejected),
        format_usize_with_commas(diagnostics.unmodeled_item_effect_candidates_penalized)
    ));
    if diagnostics.coverage_stage_enabled {
        content.push_str(&format!(
            "- Coverage stage (pre-budget):\n  - Elapsed: `{:.2}s`\n  - Assets covered: `{}/{}`\n  - Seeded candidates (unique/raw): `{}/{}`\n",
            diagnostics.coverage_stage_elapsed_seconds,
            format_usize_with_commas(diagnostics.coverage_stage_assets_covered),
            format_usize_with_commas(diagnostics.coverage_stage_assets_total),
            format_usize_with_commas(diagnostics.coverage_stage_seed_candidates_unique),
            format_usize_with_commas(diagnostics.coverage_stage_seed_candidates)
        ));
        if diagnostics.coverage_stage_incomplete && !diagnostics.coverage_stage_warning.is_empty() {
            content.push_str(&format!(
                "- Coverage warning: {}\n",
                diagnostics.coverage_stage_warning
            ));
        }
    }
    if let Some(budget) = diagnostics.time_budget_seconds {
        let coverage_note = if diagnostics.coverage_stage_enabled {
            " (budget starts after pre-budget coverage stage)"
        } else {
            ""
        };
        content.push_str(&format!(
            "- Time budget:\n  - Budget: `{:.1}s`\n  - Timed out: `{}`\n  - Progress: `{}/{}` ({:.1}%){}\n\n",
            budget,
            diagnostics.timed_out,
            format_usize_with_commas(processed_candidates),
            format_usize_with_commas(diagnostics.total_candidates),
            diagnostics.strict_completion_percent,
            coverage_note
        ));
    } else {
        content.push_str(&format!(
            "- Progress: `{}/{}` ({:.1}%)\n\n",
            format_usize_with_commas(processed_candidates),
            format_usize_with_commas(diagnostics.total_candidates),
            diagnostics.strict_completion_percent
        ));
    }
    if let Some(window) = diagnostics.popcorn_window_seconds {
        content.push_str(&format!(
            "- Popcorn mode:\n  - Window: `{:.1}s`\n  - Significant threshold: `{:.2}% of last best score`\n  - Significant events: `{}`\n  - Seconds since last significant improvement: `{:.1}`\n\n",
            window,
            diagnostics.popcorn_min_relative_improvement_percent,
            format_usize_with_commas(diagnostics.significant_improvement_events),
            diagnostics
                .seconds_since_last_significant_improvement
                .unwrap_or(0.0)
        ));
    }
    if let Some(total) = diagnostics.estimated_total_candidate_space {
        content.push_str(&format!(
            "- Estimated total legal candidate space: `{}`\n",
            format_f64_with_commas(total, 0)
        ));
    }
    if let Some(run_coverage) = diagnostics.estimated_run_space_coverage_percent {
        content.push_str(&format!(
            "- Estimated legal-space coverage (this run): `{}`\n",
            format_percent_display(run_coverage)
        ));
    }
    if let Some(probability) = diagnostics.estimated_close_to_optimal_probability {
        content.push_str(&format!(
            "- Estimated closeness probability (top 0.000001% heuristic): `{:.2}%`\n",
            probability * 100.0
        ));
    }
    if !diagnostics
        .estimated_close_to_optimal_probability_note
        .is_empty()
    {
        content.push_str(&format!(
            "- Closeness probability note: {}\n",
            diagnostics.estimated_close_to_optimal_probability_note
        ));
    }
    if !diagnostics.search_type_breakdown.is_empty() {
        content.push_str("- Search-type simulation breakdown:\n");
        for breakdown in &diagnostics.search_type_breakdown {
            content.push_str(&format!(
                "  - {}: requests `{}`, new simulations `{}`\n",
                breakdown.name,
                format_usize_with_commas(breakdown.score_requests),
                format_usize_with_commas(breakdown.new_simulations)
            ));
        }
    }
    content.push('\n');
}

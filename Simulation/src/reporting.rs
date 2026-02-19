use anyhow::{Context, Result, bail};
use chrono::{DateTime, Local, Utc};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use std::time::SystemTime;

use crate::scripts::coverage::unmodeled_runtime_item_effect_names;
use crate::search::item_names;

use super::{
    ControlledChampionReportData, ObjectiveScoreBreakdown, mean_std, simulation_dir, to_norm_key,
};

const CONTROLLED_CHAMPION_RUN_REPORT_JSON_SCHEMA_VERSION: u32 = 2;

#[cfg(test)]
pub(super) fn default_report_path_for_champion(champion_name: &str) -> PathBuf {
    simulation_dir()
        .join("output")
        .join(format!("{}_run_report.md", to_norm_key(champion_name)))
}

fn format_repo_relative_path(path: &Path) -> String {
    if !path.is_absolute() {
        return path.display().to_string();
    }
    let repository_root = simulation_dir()
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(simulation_dir);
    path.strip_prefix(&repository_root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn comma_separated_digits(digits: &str) -> String {
    let len = digits.len();
    if len <= 3 {
        return digits.to_string();
    }
    let mut out = String::with_capacity(len + len / 3);
    for (idx, ch) in digits.chars().enumerate() {
        if idx > 0 && (len - idx).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

fn format_usize_with_commas(value: usize) -> String {
    comma_separated_digits(&value.to_string())
}

fn format_f64_with_commas(value: f64, decimals: usize) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let rendered = format!("{:.decimals$}", value.abs());
    if let Some((integer, fraction)) = rendered.split_once('.') {
        format!("{}{}.{}", sign, comma_separated_digits(integer), fraction)
    } else {
        format!("{}{}", sign, comma_separated_digits(&rendered))
    }
}

fn format_percent_display(percent: f64) -> String {
    if !percent.is_finite() {
        return percent.to_string();
    }
    if percent > 0.0 && percent < 0.000001 {
        format!("{percent:.3e}%")
    } else {
        format!("{percent:.6}%")
    }
}

pub(super) fn validate_controlled_champion_selection_labels(
    controlled_champion_name: &str,
    selection_labels: &[String],
) -> Result<()> {
    let rune_count = selection_labels
        .iter()
        .filter(|label| label.starts_with("Rune: "))
        .count();
    let shard_count = selection_labels
        .iter()
        .filter(|label| label.starts_with("Shard "))
        .count();
    if rune_count < 6 || shard_count < 3 {
        bail!(
            "Controlled champion '{}' report invariant violation: expected >=6 rune labels and >=3 shard labels, got runes={} shards={}.",
            controlled_champion_name,
            rune_count,
            shard_count
        );
    }
    Ok(())
}

fn append_objective_score_breakdown_block(
    content: &mut String,
    title: &str,
    breakdown: ObjectiveScoreBreakdown,
) {
    let push_component = |content: &mut String,
                          label: &str,
                          weight: f64,
                          normalized_ratio: f64,
                          contribution: f64,
                          impact_percent: f64| {
        let delta_vs_weight = impact_percent - weight * 100.0;
        content.push_str(&format!(
            "- {}: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%` | delta vs weight `{:+.2}pp`\n",
            label,
            weight,
            normalized_ratio,
            contribution,
            impact_percent,
            delta_vs_weight
        ));
    };

    content.push_str(&format!("### {}\n", title));
    content.push_str(&format!(
        "- Weighted-mean score: `{:.4}`\n- Worst-case scenario score: `{:.4}`\n- Worst-case blend weight: `{:.2}`\n- Final blended objective score: `{:.4}`\n",
        breakdown.weighted_mean_score,
        breakdown.worst_case_score,
        breakdown.worst_case_weight,
        breakdown.final_score
    ));
    push_component(
        content,
        "survival",
        breakdown.survival.weight,
        breakdown.survival.normalized_ratio,
        breakdown.survival.contribution,
        breakdown.survival.impact_percent,
    );
    push_component(
        content,
        "damage",
        breakdown.damage.weight,
        breakdown.damage.normalized_ratio,
        breakdown.damage.contribution,
        breakdown.damage.impact_percent,
    );
    push_component(
        content,
        "healing",
        breakdown.healing.weight,
        breakdown.healing.normalized_ratio,
        breakdown.healing.contribution,
        breakdown.healing.impact_percent,
    );
    push_component(
        content,
        "enemy_kills",
        breakdown.enemy_kills.weight,
        breakdown.enemy_kills.normalized_ratio,
        breakdown.enemy_kills.contribution,
        breakdown.enemy_kills.impact_percent,
    );
    push_component(
        content,
        "invulnerable_seconds",
        breakdown.invulnerable_seconds.weight,
        breakdown.invulnerable_seconds.normalized_ratio,
        breakdown.invulnerable_seconds.contribution,
        breakdown.invulnerable_seconds.impact_percent,
    );
    content.push('\n');
}

fn report_rune_proc_telemetry_json(
    entries: &[crate::scripts::champions::ChampionRuneProcTelemetryEntry],
    total_damage: f64,
    total_healing: f64,
) -> Vec<Value> {
    entries
        .iter()
        .map(|entry| {
            let damage_share = if total_damage > 0.0 {
                entry.bonus_damage.max(0.0) / total_damage
            } else {
                0.0
            };
            let healing_share = if total_healing > 0.0 {
                entry.bonus_healing.max(0.0) / total_healing
            } else {
                0.0
            };
            json!({
                "rune_name": entry.rune_name,
                "proc_count": entry.proc_count,
                "attempt_count": entry.attempt_count,
                "eligible_count": entry.eligible_count,
                "proc_attempt_rate": entry.proc_attempt_rate,
                "proc_eligible_rate": entry.proc_eligible_rate,
                "opportunity_count": entry.eligible_count,
                "proc_opportunity_rate": entry.proc_eligible_rate,
                "bonus_damage": entry.bonus_damage,
                "bonus_damage_share": damage_share,
                "bonus_healing": entry.bonus_healing,
                "bonus_healing_share": healing_share,
                "source_breakdown": entry.source_breakdown.iter().map(|source| {
                    json!({
                        "source": source.source,
                        "proc_count": source.proc_count,
                        "attempt_count": source.attempt_count,
                        "eligible_count": source.eligible_count,
                        "proc_attempt_rate": source.proc_attempt_rate,
                        "proc_eligible_rate": source.proc_eligible_rate,
                        "opportunity_count": source.eligible_count,
                        "proc_opportunity_rate": source.proc_eligible_rate,
                        "bonus_damage": source.bonus_damage,
                        "bonus_healing": source.bonus_healing
                    })
                }).collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>()
}

pub(super) fn write_controlled_champion_report_markdown(
    report_path: &Path,
    data: &ControlledChampionReportData<'_>,
) -> Result<()> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let scenario_path = data.scenario_path;
    let scenario_path_display = format_repo_relative_path(scenario_path);
    let controlled_champion_name = data.controlled_champion_name;
    let sim = data.sim;
    let controlled_champion_base_level = data.controlled_champion_base_level;
    let controlled_champion_end_stats = data.controlled_champion_end_stats;
    let stack_notes = data.stack_notes;
    let controlled_champion_loadout = data.controlled_champion_loadout;
    let enemy_loadout = data.enemy_loadout;
    let best_build = data.best_build;
    let best_score = data.best_score;
    let best_outcome = data.best_outcome;
    let best_rune_proc_telemetry = data.best_rune_proc_telemetry;
    let controlled_champion_unmodeled_item_effect_names =
        unmodeled_runtime_item_effect_names(best_build);
    let best_score_breakdown = data.best_score_breakdown;
    let enemy_builds = data.enemy_builds;
    let enemy_derived_combat_stats = data.enemy_derived_combat_stats;
    let enemy_similarity_notes = data.enemy_similarity_notes;
    let enemy_presets_used = data.enemy_presets_used;
    let diverse_top_builds = data.diverse_top_builds;
    let diverse_top_keys = data.diverse_top_keys;
    let build_confidence = data.build_confidence;
    let metrics_by_key = data.metrics_by_key;
    let pareto_front = data.pareto_front;
    let diagnostics = data.diagnostics;
    let build_orders = data.build_orders;

    validate_controlled_champion_selection_labels(
        controlled_champion_name,
        &controlled_champion_loadout.selection_labels,
    )?;

    let now = SystemTime::now();
    let generated_utc: DateTime<Utc> = now.into();
    let generated_local: DateTime<Local> = DateTime::from(now);

    let mut content = String::new();
    content.push_str(&format!(
        "# {} URF Run Report\n\n",
        controlled_champion_name
    ));
    content.push_str(&format!(
        "- Generated (local): `{}`\n",
        generated_local.format("%Y-%m-%d %H:%M:%S %Z")
    ));
    content.push_str(&format!(
        "- Generated (UTC): `{}`\n",
        generated_utc.to_rfc3339()
    ));
    content.push_str(&format!("- Scenario: `{}`\n\n", scenario_path_display));

    content.push_str("## Headline\n");
    let best_damage = format_f64_with_commas(best_outcome.damage_dealt, 1);
    let best_healing = format_f64_with_commas(best_outcome.healing_done, 1);
    let best_invulnerable_seconds = format_f64_with_commas(best_outcome.invulnerable_seconds, 2);
    content.push_str(&format!(
        "- Best objective score: **{:.4}**\n- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **{:.2}s / {} / {} / {} / {}s**\n- Best cap survivor: **{}**\n\n",
        best_score,
        best_outcome.time_alive_seconds,
        best_damage,
        best_healing,
        best_outcome.enemy_kills,
        best_invulnerable_seconds,
        best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
    ));

    content.push_str(&format!(
        "- Champion level assumption: **{}**\n\n",
        sim.champion_level
    ));
    content.push_str("## Objective Score Breakdown\n");
    append_objective_score_breakdown_block(&mut content, "Best Build", best_score_breakdown);
    content.push_str("## Rune Proc Telemetry (Best Trace)\n");
    if best_rune_proc_telemetry.is_empty() {
        content.push_str("- No rune procs were recorded during the best-trace replay.\n\n");
    } else {
        for entry in best_rune_proc_telemetry {
            let damage_share_percent = if best_outcome.damage_dealt > 0.0 {
                (entry.bonus_damage.max(0.0) / best_outcome.damage_dealt) * 100.0
            } else {
                0.0
            };
            let healing_share_percent = if best_outcome.healing_done > 0.0 {
                (entry.bonus_healing.max(0.0) / best_outcome.healing_done) * 100.0
            } else {
                0.0
            };
            content.push_str(&format!(
                "- {}: procs `{}` / attempts `{}` / eligible `{}` (proc/attempt {:.1}%, proc/eligible {:.1}%), bonus damage `{:.2}` ({:.2}% share), bonus healing `{:.2}` ({:.2}% share)\n",
                entry.rune_name,
                entry.proc_count,
                entry.attempt_count,
                entry.eligible_count,
                entry.proc_attempt_rate * 100.0,
                entry.proc_eligible_rate * 100.0,
                entry.bonus_damage,
                damage_share_percent,
                entry.bonus_healing,
                healing_share_percent
            ));
            if !entry.source_breakdown.is_empty() {
                let sources = entry
                    .source_breakdown
                    .iter()
                    .map(|source| {
                        format!(
                            "{} (procs {}, attempts {}, eligible {}, proc/attempt {:.1}%, proc/eligible {:.1}%, damage {:.2}, healing {:.2})",
                            source.source,
                            source.proc_count,
                            source.attempt_count,
                            source.eligible_count,
                            source.proc_attempt_rate * 100.0,
                            source.proc_eligible_rate * 100.0,
                            source.bonus_damage,
                            source.bonus_healing
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("; ");
                content.push_str(&format!("  - sources: {}\n", sources));
            }
        }
        content.push('\n');
    }

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
        "- Strategy: `{}`\n- Search quality profile: `{}`\n- Enemy scenarios: `{}`\n- Loadout candidates/finalists: `{}/{}`\n- Ensemble seeds: `{}`\n- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `{}` / `{}` / `{}` / `{}`\n- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `{:.2}/{:.2}/{:.2}/{:.2}/{:.2}`\n- Simulations executed (new full combat runs): `{}`\n- Unique scored candidates (all search stages): `{}`\n- Total score requests (all search stages): `{}`\n- Full evaluations cache hits/misses/waits: `{}/{}/{}`\n- Full persistent cache hits/entries: `{}/{}`\n- Candidate keys generated / duplicate-pruned / unique: `{}/{}/{}`\n- Strict candidates seed-scored / remaining / processed: `{}/{}/{}`\n- Strict non-finite / timeout-skipped: `{}/{}`\n- Strict completion: `{:.1}%`\n- Strict ordering heuristic (enabled / rune_weight / shard_weight / exploration_promotions): `{}` / `{:.2}` / `{:.2}` / `{}`\n- Bleed candidates injected: `{}`\n- Adaptive candidates injected: `{}`\n- Seed-best mean/stddev: `{}` / `{}`\n- Search elapsed time: `{:.2}s`\n- Total run time (end-to-end): `{:.2}s`\n\n",
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
        format_usize_with_commas(diagnostics.full_persistent_cache_hits),
        format_usize_with_commas(diagnostics.full_persistent_cache_entries),
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
        "- Unmodeled rune gate (hard_gate / penalty_per_rune / rejected / penalized): `{}` / `{:.4}` / `{}` / `{}`\n",
        diagnostics.unmodeled_rune_hard_gate,
        diagnostics.unmodeled_rune_penalty_per_rune,
        format_usize_with_commas(diagnostics.unmodeled_rune_candidates_rejected),
        format_usize_with_commas(diagnostics.unmodeled_rune_candidates_penalized)
    ));
    content.push_str(&format!(
        "- Unmodeled item-effect gate (hard_gate / penalty_per_item / rejected / penalized): `{}` / `{:.4}` / `{}` / `{}`\n",
        diagnostics.unmodeled_item_effect_hard_gate,
        diagnostics.unmodeled_item_effect_penalty_per_item,
        format_usize_with_commas(diagnostics.unmodeled_item_effect_candidates_rejected),
        format_usize_with_commas(diagnostics.unmodeled_item_effect_candidates_penalized)
    ));
    if diagnostics.coverage_stage_enabled {
        content.push_str(&format!(
            "- Coverage stage (pre-budget): `{:.2}s`; assets covered `{}/{}`; seeded candidates unique/raw `{}/{}`\n",
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
            "- Time budget: `{:.1}s`; timed_out: `{}`; progress: `{}/{}` ({:.1}%){}\n\n",
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
            "- Popcorn mode: window `{:.1}s`; significant threshold `{:.2}% of last best score`; significant events `{}`; seconds since last significant improvement `{:.1}`\n\n",
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
    if let Some(cache_coverage) = diagnostics.estimated_cache_space_coverage_percent {
        content.push_str(&format!(
            "- Estimated legal-space coverage (persistent cache): `{}`\n",
            format_percent_display(cache_coverage)
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
                "  - {}: requests `{}`, new simulations `{}`, persistent cache hits `{}`\n",
                breakdown.name,
                format_usize_with_commas(breakdown.score_requests),
                format_usize_with_commas(breakdown.new_simulations),
                format_usize_with_commas(breakdown.persistent_cache_hits)
            ));
        }
    }
    content.push('\n');

    content.push_str(&format!(
        "## {} Base Stats At Level\n",
        controlled_champion_name
    ));
    content.push_str(&format!(
        "- HP: {}, Armor: {}, MR: {}, AD: {}, AS: {}, MS: {}\n\n",
        format_f64_with_commas(controlled_champion_base_level.base_health, 1),
        format_f64_with_commas(controlled_champion_base_level.base_armor, 1),
        format_f64_with_commas(controlled_champion_base_level.base_magic_resist, 1),
        format_f64_with_commas(controlled_champion_base_level.base_attack_damage, 1),
        format_f64_with_commas(controlled_champion_base_level.base_attack_speed, 3),
        format_f64_with_commas(controlled_champion_base_level.base_move_speed, 1)
    ));

    content.push_str("## Selected Rune Page And Shards\n");
    content.push_str(&format!("- {}:\n", controlled_champion_name));
    for s in &controlled_champion_loadout.selection_labels {
        content.push_str(&format!("  - {}\n", s));
    }
    if enemy_loadout.selection_labels.is_empty() {
        content.push_str(
            "- Opponents: champion-specific preset rune pages are listed in Enemy Builds.\n\n",
        );
    } else {
        content.push_str("- Opponents (shared):\n");
        for s in &enemy_loadout.selection_labels {
            content.push_str(&format!("  - {}\n", s));
        }
        content.push('\n');
    }
    if !controlled_champion_loadout.applied_notes.is_empty()
        || !enemy_loadout.applied_notes.is_empty()
    {
        content.push_str("- Applied deterministic loadout effects:\n");
        for note in &controlled_champion_loadout.applied_notes {
            content.push_str(&format!("  - {}: {}\n", controlled_champion_name, note));
        }
        for note in &enemy_loadout.applied_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    if !controlled_champion_loadout.skipped_notes.is_empty()
        || !enemy_loadout.skipped_notes.is_empty()
    {
        content.push_str("- Skipped unsupported/non-deterministic effects:\n");
        for note in &controlled_champion_loadout.skipped_notes {
            content.push_str(&format!("  - {}: {}\n", controlled_champion_name, note));
        }
        for note in &enemy_loadout.skipped_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    if !controlled_champion_loadout.unmodeled_rune_names.is_empty() {
        content.push_str(
            "- Controlled champion runes with no modeled deterministic/runtime combat effect:\n",
        );
        for rune_name in &controlled_champion_loadout.unmodeled_rune_names {
            content.push_str(&format!("  - {}\n", rune_name));
        }
    }
    if !controlled_champion_unmodeled_item_effect_names.is_empty() {
        content.push_str(
            "- Controlled champion items with unmodeled passive/active/structured runtime effects:\n",
        );
        for item_name in &controlled_champion_unmodeled_item_effect_names {
            content.push_str(&format!("  - {}\n", item_name));
        }
    }
    content.push('\n');

    content.push_str("## Best Build\n");
    content.push_str(&format!("- {}\n\n", item_names(best_build)));

    content.push_str(&format!(
        "## {} End Stats (Best Build)\n",
        controlled_champion_name
    ));
    content.push_str(&format!(
        "- HP: {}, Armor: {}, MR: {}, AP: {}, AD: {}, Ability Haste: {}, Move Speed (flat bonus): {}, Move Speed (% bonus): {}\n\n",
        format_f64_with_commas(controlled_champion_end_stats.health, 1),
        format_f64_with_commas(controlled_champion_end_stats.armor, 1),
        format_f64_with_commas(controlled_champion_end_stats.magic_resist, 1),
        format_f64_with_commas(controlled_champion_end_stats.ability_power, 1),
        format_f64_with_commas(controlled_champion_end_stats.attack_damage, 1),
        format_f64_with_commas(controlled_champion_end_stats.ability_haste, 1),
        format_f64_with_commas(controlled_champion_end_stats.move_speed_flat, 1),
        format_f64_with_commas(controlled_champion_end_stats.move_speed_percent, 1)
    ));

    content.push_str("## Stack Overrides\n");
    if stack_notes.is_empty() {
        content
            .push_str("- No explicit stack overrides triggered for selected best build items.\n\n");
    } else {
        for note in stack_notes {
            content.push_str(&format!("- {}\n", note));
        }
        content.push('\n');
    }

    content.push_str("## Enemy Builds (URF Presets)\n");
    for (enemy, build, _) in enemy_builds {
        content.push_str(&format!("- {}: {}\n", enemy.name, item_names(build)));
        if let Some(preset) = enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            content.push_str(&format!(
                "  - Source: {} (last checked {})\n",
                preset.source_url, preset.last_checked
            ));
            content.push_str(&format!("  - Runes: {}\n", preset.runes.join(", ")));
            content.push_str(&format!("  - Shards: {}\n", preset.shards.join(", ")));
        }
    }
    content.push('\n');

    content.push_str("## Enemy Derived Combat Profiles\n");
    for profile in enemy_derived_combat_stats {
        content.push_str(&format!(
            "- {}: HP {:.1}, Armor {:.1}, MR {:.1}, AD {:.1}, AS {:.3} (interval {:.3}s), range {:.0}, projectile speed {:.0}, move speed {:.1}, desired combat range {:.0}, hit physical {:.1}, hit ability {:.1}, burst phys/magic/true {:.1}/{:.1}/{:.1}\n",
            profile.champion,
            profile.max_health,
            profile.armor,
            profile.magic_resist,
            profile.attack_damage,
            profile.attack_speed,
            profile.attack_interval_seconds,
            profile.attack_range,
            profile.attack_projectile_speed,
            profile.move_speed,
            profile.desired_combat_range,
            profile.physical_hit_damage,
            profile.ability_hit_damage,
            profile.burst_physical_damage,
            profile.burst_magic_damage,
            profile.burst_true_damage
        ));
    }
    if !enemy_similarity_notes.is_empty() {
        content.push_str("- Similarity checks:\n");
        for note in enemy_similarity_notes {
            content.push_str(&format!("  - {}\n", note));
        }
    }
    content.push('\n');

    content.push_str("## Diverse Top Builds\n");
    if diverse_top_builds.is_empty() {
        content.push_str("- No diverse builds found under current thresholds.\n\n");
    } else {
        let best = diverse_top_builds[0].1;
        for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
            let delta = score - best;
            let key = diverse_top_keys.get(idx);
            let confidence = key
                .and_then(|k| build_confidence.iter().find(|c| c.key == *k))
                .map(|c| {
                    format!(
                        " | seed hits: {}/{} ({:.0}%) {}",
                        c.seed_hits,
                        diagnostics.ensemble_seeds,
                        c.seed_hit_rate * 100.0,
                        c.robustness
                    )
                })
                .unwrap_or_default();
            let pareto = key.map(|k| pareto_front.contains(k)).unwrap_or(false);
            let pareto_tag = if pareto { " | Pareto-front" } else { "" };
            content.push_str(&format!(
                "{}. `score {:.4}` ({:+.4} vs top): {}{}{}\n",
                idx + 1,
                score,
                delta,
                item_names(build),
                confidence,
                pareto_tag
            ));
            if let Some(k) = key
                && let Some(m) = metrics_by_key.get(k)
            {
                content.push_str(&format!(
                    "   - metrics: EHP~{}, AP~{}, timing score {:+.2}, total cost {}\n",
                    format_f64_with_commas(m.ehp_mixed, 1),
                    format_f64_with_commas(m.ap, 1),
                    m.cost_timing,
                    format_f64_with_commas(m.total_cost, 0)
                ));
            }
        }
        content.push('\n');
    }

    content.push_str("## Build Order Optimization\n");
    if build_orders.is_empty() {
        content.push_str("- No build-order optimization results available.\n\n");
    } else {
        for (idx, br) in build_orders.iter().enumerate() {
            content.push_str(&format!(
                "{}. Cumulative score: `{:.2}` | Order: {}\n",
                idx + 1,
                br.cumulative_score,
                item_names(&br.ordered_items)
            ));
            for (stage_idx, lvl) in br.levels.iter().enumerate() {
                let surv = br.stage_survival.get(stage_idx).copied().unwrap_or(0.0);
                let dmg = br.stage_damage.get(stage_idx).copied().unwrap_or(0.0);
                let heal = br.stage_healing.get(stage_idx).copied().unwrap_or(0.0);
                let stage_objective = br
                    .stage_objective_scores
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                content.push_str(&format!(
                    "   - Stage {} (level {}): objective `{:.3}`, time alive `{:.2}s`, damage `{}`, healing `{}`\n",
                    stage_idx + 1,
                    format_usize_with_commas(*lvl),
                    stage_objective,
                    surv,
                    format_f64_with_commas(dmg, 1),
                    format_f64_with_commas(heal, 1)
                ));
            }
        }
        content.push('\n');
    }

    content.push_str("## Deeper Insights\n");
    if !diverse_top_builds.is_empty() {
        let mut item_counts: HashMap<String, usize> = HashMap::new();
        for (build, _) in diverse_top_builds {
            for item in build {
                *item_counts.entry(item.name.clone()).or_insert(0) += 1;
            }
        }
        let mut counts = item_counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let core_items = counts
            .iter()
            .filter(|(_, c)| *c == diverse_top_builds.len())
            .map(|(n, _)| n.clone())
            .collect::<Vec<_>>();
        let top_freq = counts
            .iter()
            .take(8)
            .map(|(n, c)| format!("{} ({}/{})", n, c, diverse_top_builds.len()))
            .collect::<Vec<_>>();

        if core_items.is_empty() {
            content.push_str("- No single item appears in every selected diverse top build.\n");
        } else {
            content.push_str(&format!(
                "- Common core across all selected top builds: {}.\n",
                core_items.join(", ")
            ));
        }
        content.push_str(&format!(
            "- Most frequent items in selected top set: {}.\n",
            top_freq.join(", ")
        ));
        content.push_str("- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.\n");
    } else {
        content.push_str("- Broaden thresholds (`--max-relative-gap-percent`) or lower diversity constraint (`--min-item-diff`) to surface more alternatives.\n");
    }

    fs::write(report_path, content)
        .with_context(|| format!("Failed writing report {}", report_path.display()))?;
    Ok(())
}

pub(super) fn write_controlled_champion_report_json(
    report_path: &Path,
    data: &ControlledChampionReportData<'_>,
) -> Result<()> {
    fn component_json(
        weight: f64,
        normalized_ratio: f64,
        contribution: f64,
        impact_percent: f64,
    ) -> Value {
        json!({
            "weight": weight,
            "normalized_ratio": normalized_ratio,
            "contribution": contribution,
            "impact_percent": impact_percent,
            "delta_vs_weight_percent_points": impact_percent - weight * 100.0
        })
    }
    fn objective_breakdown_json(breakdown: ObjectiveScoreBreakdown) -> Value {
        json!({
            "weighted_mean_score": breakdown.weighted_mean_score,
            "worst_case_score": breakdown.worst_case_score,
            "worst_case_weight": breakdown.worst_case_weight,
            "final_score": breakdown.final_score,
            "components": {
                "survival": component_json(
                    breakdown.survival.weight,
                    breakdown.survival.normalized_ratio,
                    breakdown.survival.contribution,
                    breakdown.survival.impact_percent
                ),
                "damage": component_json(
                    breakdown.damage.weight,
                    breakdown.damage.normalized_ratio,
                    breakdown.damage.contribution,
                    breakdown.damage.impact_percent
                ),
                "healing": component_json(
                    breakdown.healing.weight,
                    breakdown.healing.normalized_ratio,
                    breakdown.healing.contribution,
                    breakdown.healing.impact_percent
                ),
                "enemy_kills": component_json(
                    breakdown.enemy_kills.weight,
                    breakdown.enemy_kills.normalized_ratio,
                    breakdown.enemy_kills.contribution,
                    breakdown.enemy_kills.impact_percent
                ),
                "invulnerable_seconds": component_json(
                    breakdown.invulnerable_seconds.weight,
                    breakdown.invulnerable_seconds.normalized_ratio,
                    breakdown.invulnerable_seconds.contribution,
                    breakdown.invulnerable_seconds.impact_percent
                )
            }
        })
    }

    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let now = SystemTime::now();
    let generated_utc: DateTime<Utc> = now.into();
    let generated_local: DateTime<Local> = DateTime::from(now);
    let scenario_path = data.scenario_path;
    let scenario_path_display = format_repo_relative_path(scenario_path);
    let controlled_champion_name = data.controlled_champion_name;
    let sim = data.sim;
    let best_build = data.best_build;
    let best_score = data.best_score;
    let best_outcome = data.best_outcome;
    let best_rune_proc_telemetry = data.best_rune_proc_telemetry;
    let best_score_breakdown = data.best_score_breakdown;
    let controlled_champion_unmodeled_item_effect_names =
        unmodeled_runtime_item_effect_names(best_build);
    let controlled_champion_loadout = data.controlled_champion_loadout;
    let enemy_builds = data.enemy_builds;
    let enemy_derived_combat_stats = data.enemy_derived_combat_stats;
    let enemy_similarity_notes = data.enemy_similarity_notes;
    let enemy_presets_used = data.enemy_presets_used;
    let diverse_top_builds = data.diverse_top_builds;
    let diagnostics = data.diagnostics;
    let build_orders = data.build_orders;
    validate_controlled_champion_selection_labels(
        controlled_champion_name,
        &controlled_champion_loadout.selection_labels,
    )?;
    let total_score_requests = diagnostics
        .search_type_breakdown
        .iter()
        .map(|breakdown| breakdown.score_requests)
        .sum::<usize>();
    let json_value = json!({
        "schema_version": CONTROLLED_CHAMPION_RUN_REPORT_JSON_SCHEMA_VERSION,
        "generated_utc": generated_utc.to_rfc3339(),
        "generated_local": generated_local.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        "scenario_path": scenario_path_display,
        "controlled_champion_name": controlled_champion_name,
        "champion_level": sim.champion_level,
        "headline": {
            "best_objective_score": best_score,
            "best_outcome": {
                "time_alive_seconds": best_outcome.time_alive_seconds,
                "damage_dealt": best_outcome.damage_dealt,
                "healing_done": best_outcome.healing_done,
                "enemy_kills": best_outcome.enemy_kills,
                "invulnerable_seconds": best_outcome.invulnerable_seconds,
                "cap_survivor": best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
            },
            "best_objective_breakdown": objective_breakdown_json(best_score_breakdown),
        },
        "best_rune_proc_telemetry": report_rune_proc_telemetry_json(
            best_rune_proc_telemetry,
            best_outcome.damage_dealt,
            best_outcome.healing_done,
        ),
        "best_build": best_build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
        "controlled_champion_loadout_labels": controlled_champion_loadout.selection_labels,
        "controlled_champion_unmodeled_runes": controlled_champion_loadout.unmodeled_rune_names,
        "controlled_champion_unmodeled_item_effects": controlled_champion_unmodeled_item_effect_names,
        "enemy_presets": enemy_builds.iter().map(|(enemy, build, _)| {
            let key = to_norm_key(&enemy.name);
            let preset = enemy_presets_used.get(&key);
            json!({
                "champion": enemy.name,
                "items": build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                "runes": preset.map(|p| p.runes.clone()).unwrap_or_default(),
                "shards": preset.map(|p| p.shards.clone()).unwrap_or_default(),
                "source_url": preset.map(|p| p.source_url.clone()).unwrap_or_default(),
                "last_checked": preset.map(|p| p.last_checked.clone()).unwrap_or_default(),
            })
        }).collect::<Vec<_>>(),
        "enemy_derived_combat_stats": enemy_derived_combat_stats.iter().map(|profile| {
            json!({
                "champion": profile.champion,
                "max_health": profile.max_health,
                "armor": profile.armor,
                "magic_resist": profile.magic_resist,
                "attack_damage": profile.attack_damage,
                "attack_speed": profile.attack_speed,
                "attack_interval_seconds": profile.attack_interval_seconds,
                "attack_range": profile.attack_range,
                "attack_projectile_speed": profile.attack_projectile_speed,
                "move_speed": profile.move_speed,
                "desired_combat_range": profile.desired_combat_range,
                "physical_hit_damage": profile.physical_hit_damage,
                "ability_hit_damage": profile.ability_hit_damage,
                "burst_physical_damage": profile.burst_physical_damage,
                "burst_magic_damage": profile.burst_magic_damage,
                "burst_true_damage": profile.burst_true_damage
            })
        }).collect::<Vec<_>>(),
        "enemy_similarity_notes": enemy_similarity_notes,
        "diverse_top_builds": diverse_top_builds.iter().map(|(build, score)| {
            json!({
                "objective_score": score,
                "items": build.iter().map(|i| i.name.clone()).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>(),
        "build_orders": build_orders.iter().map(|order| {
            json!({
                "cumulative_score": order.cumulative_score,
                "ordered_items": order.ordered_items.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                "levels": order.levels,
                "stage_survival_seconds": order.stage_survival,
                "stage_damage": order.stage_damage,
                "stage_healing": order.stage_healing,
                "stage_objective_scores": order.stage_objective_scores,
            })
        }).collect::<Vec<_>>(),
        "diagnostics": {
            "strategy_summary": diagnostics.strategy_summary,
            "search_quality_profile": diagnostics.search_quality_profile,
            "effective_seed": diagnostics.effective_seed,
            "ensemble_seeds": diagnostics.ensemble_seeds,
            "effective_threads": diagnostics.effective_threads,
            "seed_orchestration_parallel": diagnostics.seed_orchestration_parallel,
            "portfolio_strategy_parallel": diagnostics.portfolio_strategy_parallel,
            "strategy_elites_parallel": diagnostics.strategy_elites_parallel,
            "objective_survival_weight": diagnostics.objective_survival_weight,
            "objective_damage_weight": diagnostics.objective_damage_weight,
            "objective_healing_weight": diagnostics.objective_healing_weight,
            "objective_enemy_kills_weight": diagnostics.objective_enemy_kills_weight,
            "objective_invulnerable_seconds_weight": diagnostics.objective_invulnerable_seconds_weight,
            "full_evaluations": diagnostics.full_evaluations,
            "full_cache_hits": diagnostics.full_cache_hits,
            "full_cache_misses": diagnostics.full_cache_misses,
            "full_cache_waits": diagnostics.full_cache_waits,
            "full_persistent_cache_hits": diagnostics.full_persistent_cache_hits,
            "full_persistent_cache_entries": diagnostics.full_persistent_cache_entries,
            "candidate_keys_generated": diagnostics.candidate_keys_generated,
            "candidate_duplicates_pruned": diagnostics.candidate_duplicates_pruned,
            "unique_candidate_builds": diagnostics.unique_candidate_builds,
            "bleed_candidates_injected": diagnostics.bleed_candidates_injected,
            "adaptive_candidates_injected": diagnostics.adaptive_candidates_injected,
            "scenario_count": diagnostics.scenario_count,
            "loadout_candidates": diagnostics.loadout_candidates,
            "loadout_finalists": diagnostics.loadout_finalists,
            "strict_seed_scored_candidates": diagnostics.strict_seed_scored_candidates,
            "strict_remaining_candidates": diagnostics.strict_remaining_candidates,
            "strict_non_finite_candidates": diagnostics.strict_non_finite_candidates,
            "strict_candidates_skipped_timeout": diagnostics.strict_candidates_skipped_timeout,
            "strict_completion_percent": diagnostics.strict_completion_percent,
            "strict_heuristic_ordering_enabled": diagnostics.strict_heuristic_ordering_enabled,
            "strict_ranking_rune_signal_weight": diagnostics.strict_ranking_rune_signal_weight,
            "strict_ranking_shard_signal_weight": diagnostics.strict_ranking_shard_signal_weight,
            "strict_random_promotions_done": diagnostics.strict_random_promotions_done,
            "unmodeled_rune_hard_gate": diagnostics.unmodeled_rune_hard_gate,
            "unmodeled_rune_penalty_per_rune": diagnostics.unmodeled_rune_penalty_per_rune,
            "unmodeled_rune_candidates_rejected": diagnostics.unmodeled_rune_candidates_rejected,
            "unmodeled_rune_candidates_penalized": diagnostics.unmodeled_rune_candidates_penalized,
            "unmodeled_item_effect_hard_gate": diagnostics.unmodeled_item_effect_hard_gate,
            "unmodeled_item_effect_penalty_per_item": diagnostics.unmodeled_item_effect_penalty_per_item,
            "unmodeled_item_effect_candidates_rejected": diagnostics.unmodeled_item_effect_candidates_rejected,
            "unmodeled_item_effect_candidates_penalized": diagnostics.unmodeled_item_effect_candidates_penalized,
            "unique_scored_candidates": diagnostics.unique_scored_candidates,
            "time_budget_seconds": diagnostics.time_budget_seconds,
            "popcorn_window_seconds": diagnostics.popcorn_window_seconds,
            "popcorn_min_relative_improvement_percent": diagnostics.popcorn_min_relative_improvement_percent,
            "significant_improvement_events": diagnostics.significant_improvement_events,
            "best_significant_score": diagnostics.best_significant_score,
            "seconds_since_last_significant_improvement": diagnostics.seconds_since_last_significant_improvement,
            "search_type_breakdown": diagnostics.search_type_breakdown.iter().map(|breakdown| {
                json!({
                    "name": breakdown.name.clone(),
                    "score_requests": breakdown.score_requests,
                    "new_simulations": breakdown.new_simulations,
                    "persistent_cache_hits": breakdown.persistent_cache_hits
                })
            }).collect::<Vec<_>>(),
            "estimated_total_candidate_space": diagnostics.estimated_total_candidate_space,
            "estimated_run_space_coverage_percent": diagnostics.estimated_run_space_coverage_percent,
            "estimated_cache_space_coverage_percent": diagnostics.estimated_cache_space_coverage_percent,
            "estimated_close_to_optimal_probability": diagnostics.estimated_close_to_optimal_probability,
            "estimated_close_to_optimal_probability_note": diagnostics.estimated_close_to_optimal_probability_note,
            "coverage_stage_enabled": diagnostics.coverage_stage_enabled,
            "coverage_stage_elapsed_seconds": diagnostics.coverage_stage_elapsed_seconds,
            "coverage_stage_assets_total": diagnostics.coverage_stage_assets_total,
            "coverage_stage_assets_covered": diagnostics.coverage_stage_assets_covered,
            "coverage_stage_seed_candidates": diagnostics.coverage_stage_seed_candidates,
            "coverage_stage_seed_candidates_unique": diagnostics.coverage_stage_seed_candidates_unique,
            "coverage_stage_incomplete": diagnostics.coverage_stage_incomplete,
            "coverage_stage_warning": diagnostics.coverage_stage_warning,
            "elapsed_seconds": diagnostics.elapsed_seconds,
            "total_run_seconds": diagnostics.total_run_seconds,
            "timed_out": diagnostics.timed_out,
            "processed_candidates": diagnostics.processed_candidates,
            "total_candidates": diagnostics.total_candidates,
            "simulation_counts": {
                "new_full_simulations": diagnostics.full_evaluations,
                "unique_scored_candidates": diagnostics.unique_scored_candidates,
                "total_score_requests": total_score_requests
            }
        }
    });
    fs::write(report_path, serde_json::to_string_pretty(&json_value)?)
        .with_context(|| format!("Failed writing JSON report {}", report_path.display()))?;
    Ok(())
}

#[cfg(test)]
#[path = "tests/reporting_tests.rs"]
mod tests;

use anyhow::Result;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

use super::*;

use super::result_aggregation::RuneSweepEntry;

pub(super) struct RuneSweepReportWriteInput<'a> {
    pub(super) scenario_path: &'a Path,
    pub(super) report_path_override: Option<&'a str>,
    pub(super) search_quality_profile: SearchQualityProfile,
    pub(super) fixed_eval_label: Option<&'a str>,
    pub(super) controlled_champion_name: &'a str,
    pub(super) fixed_build_items: &'a [Item],
    pub(super) controlled_champion_loadout_selection: &'a LoadoutSelection,
    pub(super) sweep_seed_repeats: usize,
    pub(super) seed_base: u64,
    pub(super) sweep_results: &'a [RuneSweepEntry],
}

fn resolve_rune_sweep_report_path(input: &RuneSweepReportWriteInput<'_>) -> Result<PathBuf> {
    let run_label = input.fixed_eval_label.unwrap_or("fixed_loadout_rune_sweep");
    let default_output_dir =
        default_fixed_loadout_rune_sweep_output_directory(input.search_quality_profile, run_label);
    fs::create_dir_all(&default_output_dir)?;
    let controlled_champion_key = to_norm_key(input.controlled_champion_name);
    let report_path = input
        .report_path_override
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            default_output_dir.join(format!(
                "{controlled_champion_key}_fixed_loadout_rune_sweep_report.md"
            ))
        });
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(report_path)
}

fn build_rune_sweep_report_markdown(input: &RuneSweepReportWriteInput<'_>) -> String {
    let best_score = input
        .sweep_results
        .first()
        .map(|entry| entry.objective_score)
        .unwrap_or(0.0);
    let mut markdown = String::new();
    markdown.push_str("# Controlled Champion Fixed Loadout Rune Sweep\n\n");
    markdown.push_str(&format!(
        "- Scenario: `{}`\n",
        input.scenario_path.display()
    ));
    markdown.push_str(&format!(
        "- Search quality profile: `{}`\n",
        search_quality_profile_key(input.search_quality_profile)
    ));
    markdown.push_str(&format!(
        "- Controlled champion: `{}`\n",
        input.controlled_champion_name
    ));
    markdown.push_str(&format!(
        "- Build items: `{}`\n",
        input
            .fixed_build_items
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    markdown.push_str(&format!(
        "- Baseline runes: `{}`\n",
        input
            .controlled_champion_loadout_selection
            .rune_names
            .join(", ")
    ));
    markdown.push_str(&format!(
        "- Baseline shards: `{}`\n\n",
        input
            .controlled_champion_loadout_selection
            .shard_stats
            .join(", ")
    ));
    markdown.push_str(&format!(
        "- Seed repeats per keystone: `{}`\n\n",
        input.sweep_seed_repeats
    ));
    markdown.push_str(&format!("- Seed base: `{}`\n\n", input.seed_base));
    markdown.push_str("## Rune Sweep Ranking\n");
    for (idx, result) in input.sweep_results.iter().enumerate() {
        if input.sweep_seed_repeats > 1 {
            let (_, score_stddev) = mean_std(&result.seed_repeat_scores);
            let repeat_seeds = result
                .seed_repeat_values
                .iter()
                .map(|seed| seed.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            markdown.push_str(&format!(
                "{}. `{}`\n  - Score: `{:.4}`\n  - Delta: `{:+.4}`\n  - Seed stddev: `{:.4}`\n  - Outcome:\n    - Time alive: `{:.2}s`\n    - Damage dealt: `{:.1}`\n    - Healing done: `{:.1}`\n    - Enemy kills: `{}`\n    - Invulnerable seconds: `{:.2}s`\n  - Seeds: `[{}]`\n",
                idx + 1,
                result.keystone_name,
                result.objective_score,
                result.objective_score - best_score,
                score_stddev,
                result.outcome.time_alive_seconds,
                result.outcome.damage_dealt,
                result.outcome.healing_done,
                result.outcome.enemy_kills,
                result.outcome.invulnerable_seconds,
                repeat_seeds
            ));
        } else {
            markdown.push_str(&format!(
                "{}. `{}`\n  - Score: `{:.4}`\n  - Delta: `{:+.4}`\n  - Outcome:\n    - Time alive: `{:.2}s`\n    - Damage dealt: `{:.1}`\n    - Healing done: `{:.1}`\n    - Enemy kills: `{}`\n    - Invulnerable seconds: `{:.2}s`\n",
                idx + 1,
                result.keystone_name,
                result.objective_score,
                result.objective_score - best_score,
                result.outcome.time_alive_seconds,
                result.outcome.damage_dealt,
                result.outcome.healing_done,
                result.outcome.enemy_kills,
                result.outcome.invulnerable_seconds
            ));
        }
    }
    markdown.push('\n');
    markdown.push_str("## Rune Proc Telemetry\n");
    for result in input.sweep_results {
        markdown.push_str(&format!("- {}:\n", result.keystone_name));
        append_rune_proc_telemetry_markdown_entries(
            &mut markdown,
            "  - ",
            "    ",
            &result.rune_proc_telemetry,
            result.outcome.damage_dealt,
            result.outcome.healing_done,
        );
    }
    markdown
}

fn build_rune_sweep_json_report(input: &RuneSweepReportWriteInput<'_>) -> serde_json::Value {
    json!({
        "schema_version": FIXED_LOADOUT_RUNE_SWEEP_JSON_SCHEMA_VERSION,
        "scenario_path": input.scenario_path.display().to_string(),
        "search_quality_profile": search_quality_profile_key(input.search_quality_profile),
        "controlled_champion_name": input.controlled_champion_name,
        "items": input.fixed_build_items.iter().map(|item| item.name.clone()).collect::<Vec<_>>(),
        "baseline_rune_names": input.controlled_champion_loadout_selection.rune_names,
        "baseline_shard_stats": input.controlled_champion_loadout_selection.shard_stats,
        "seed_base": input.seed_base,
        "seed_repeats_per_keystone": input.sweep_seed_repeats,
        "results": input.sweep_results.iter().map(|result| {
            json!({
                "keystone_name": result.keystone_name,
                "loadout_selection": {
                    "rune_names": result.loadout_selection.rune_names,
                    "shard_stats": result.loadout_selection.shard_stats
                },
                "objective_score": result.objective_score,
                "outcome": {
                    "time_alive_seconds": result.outcome.time_alive_seconds,
                    "damage_dealt": result.outcome.damage_dealt,
                    "healing_done": result.outcome.healing_done,
                    "enemy_kills": result.outcome.enemy_kills,
                    "invulnerable_seconds": result.outcome.invulnerable_seconds
                },
                "objective_breakdown": {
                    "weighted_mean_score": result.objective_breakdown.weighted_mean_score,
                    "worst_case_score": result.objective_breakdown.worst_case_score,
                    "worst_case_weight": result.objective_breakdown.worst_case_weight,
                    "final_score": result.objective_breakdown.final_score,
                    "components": {
                        "survival": {
                            "weight": result.objective_breakdown.survival.weight,
                            "normalized_ratio": result.objective_breakdown.survival.normalized_ratio,
                            "contribution": result.objective_breakdown.survival.contribution,
                            "impact_percent": result.objective_breakdown.survival.impact_percent
                        },
                        "damage": {
                            "weight": result.objective_breakdown.damage.weight,
                            "normalized_ratio": result.objective_breakdown.damage.normalized_ratio,
                            "contribution": result.objective_breakdown.damage.contribution,
                            "impact_percent": result.objective_breakdown.damage.impact_percent
                        },
                        "healing": {
                            "weight": result.objective_breakdown.healing.weight,
                            "normalized_ratio": result.objective_breakdown.healing.normalized_ratio,
                            "contribution": result.objective_breakdown.healing.contribution,
                            "impact_percent": result.objective_breakdown.healing.impact_percent
                        },
                        "enemy_kills": {
                            "weight": result.objective_breakdown.enemy_kills.weight,
                            "normalized_ratio": result.objective_breakdown.enemy_kills.normalized_ratio,
                            "contribution": result.objective_breakdown.enemy_kills.contribution,
                            "impact_percent": result.objective_breakdown.enemy_kills.impact_percent
                        },
                        "invulnerable_seconds": {
                            "weight": result.objective_breakdown.invulnerable_seconds.weight,
                            "normalized_ratio": result.objective_breakdown.invulnerable_seconds.normalized_ratio,
                            "contribution": result.objective_breakdown.invulnerable_seconds.contribution,
                            "impact_percent": result.objective_breakdown.invulnerable_seconds.impact_percent
                        }
                    }
                },
                "seed_repeat_scores": result.seed_repeat_scores,
                "seed_repeat_values": result.seed_repeat_values,
                "rune_proc_telemetry": rune_proc_telemetry_json(
                    &result.rune_proc_telemetry,
                    result.outcome.damage_dealt,
                    result.outcome.healing_done,
                )
            })
        }).collect::<Vec<_>>()
    })
}

pub(super) fn write_rune_sweep_reports(input: &RuneSweepReportWriteInput<'_>) -> Result<()> {
    let report_path = resolve_rune_sweep_report_path(input)?;
    let json_report_path = report_path.with_extension("json");

    let markdown = build_rune_sweep_report_markdown(input);
    fs::write(&report_path, markdown)?;

    let json_report = build_rune_sweep_json_report(input);
    fs::write(
        &json_report_path,
        serde_json::to_string_pretty(&json_report)?,
    )?;

    println!(
        "Fixed-loadout rune sweep report written: {}",
        format_repo_relative_path(&report_path)
    );
    println!(
        "Fixed-loadout rune sweep JSON written: {}",
        format_repo_relative_path(&json_report_path)
    );
    Ok(())
}

use anyhow::Result;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

use super::*;

pub(in crate::scenario_runner) struct FixedLoadoutReportWriteContext<'a> {
    pub(in crate::scenario_runner) scenario_path: &'a Path,
    pub(in crate::scenario_runner) options: &'a ControlledChampionFixedLoadoutOptions<'a>,
    pub(in crate::scenario_runner) controlled_champion_name: &'a str,
    pub(in crate::scenario_runner) controlled_champion_base: &'a ChampionBase,
    pub(in crate::scenario_runner) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(in crate::scenario_runner) controlled_champion_loadout_selection: &'a LoadoutSelection,
    pub(in crate::scenario_runner) controlled_champion_loadout: &'a ResolvedLoadout,
    pub(in crate::scenario_runner) fixed_build_items: &'a [Item],
    pub(in crate::scenario_runner) fixed_score: f64,
    pub(in crate::scenario_runner) fixed_outcome: &'a CombatOutcome,
    pub(in crate::scenario_runner) fixed_breakdown: &'a ObjectiveScoreBreakdown,
    pub(in crate::scenario_runner) scenario_reference_outcomes: &'a [CombatOutcome],
    pub(in crate::scenario_runner) enemy_builds: &'a [EnemyBuildEntry],
    pub(in crate::scenario_runner) sim: &'a SimulationConfig,
    pub(in crate::scenario_runner) urf: &'a UrfBuffs,
    pub(in crate::scenario_runner) trace_json_schema_version: u32,
    pub(in crate::scenario_runner) report_json_schema_version: u32,
}

pub(in crate::scenario_runner) fn write_fixed_loadout_reports(
    context: FixedLoadoutReportWriteContext<'_>,
) -> Result<()> {
    let FixedLoadoutReportWriteContext {
        scenario_path,
        options,
        controlled_champion_name,
        controlled_champion_base,
        controlled_champion_stack_overrides,
        controlled_champion_loadout_selection,
        controlled_champion_loadout,
        fixed_build_items,
        fixed_score,
        fixed_outcome,
        fixed_breakdown,
        scenario_reference_outcomes,
        enemy_builds,
        sim,
        urf,
        trace_json_schema_version,
        report_json_schema_version,
    } = context;

    let run_label = options
        .fixed_eval_label
        .as_deref()
        .unwrap_or("fixed_loadout");
    let default_output_dir =
        default_fixed_loadout_output_directory(options.search_quality_profile, run_label);
    fs::create_dir_all(&default_output_dir)?;
    let controlled_champion_key = to_norm_key(controlled_champion_name);
    let report_path = options
        .report_path_override
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            default_output_dir.join(format!("{controlled_champion_key}_fixed_loadout_report.md"))
        });
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json_report_path = report_path.with_extension("json");
    let trace_markdown_path =
        default_output_dir.join(format!("{controlled_champion_key}_fixed_loadout_trace.md"));
    let trace_json_path = default_output_dir.join(format!(
        "{controlled_champion_key}_fixed_loadout_trace.json"
    ));

    let mut trace_sim_cfg = sim.clone();
    trace_sim_cfg.collect_rune_proc_telemetry = true;
    let trace_replay_sim_cfg = trace_sim_cfg.clone();
    let mut trace_sim = ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
        controlled_champion_base.clone(),
        fixed_build_items,
        &controlled_champion_loadout.bonus_stats,
        Some(controlled_champion_loadout_selection),
        None,
        Some(controlled_champion_stack_overrides),
        enemy_builds,
        trace_sim_cfg,
        urf.clone(),
    );
    trace_sim.enable_trace();
    while trace_sim.step(1) {}
    let trace_events = trace_sim.trace_events();
    let rune_proc_telemetry = trace_sim.controlled_champion_rune_proc_telemetry();
    let trace_determinism = trace_sim.deterministic_replay_signature();
    let mut trace_replay_sim =
        ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
            controlled_champion_base.clone(),
            fixed_build_items,
            &controlled_champion_loadout.bonus_stats,
            Some(controlled_champion_loadout_selection),
            None,
            Some(controlled_champion_stack_overrides),
            enemy_builds,
            trace_replay_sim_cfg,
            urf.clone(),
        );
    while trace_replay_sim.step(1) {}
    let trace_replay_determinism = trace_replay_sim.deterministic_replay_signature();
    verify_deterministic_replay_signature_match(
        trace_determinism,
        trace_replay_determinism,
        "fixed-loadout trace replay",
    )?;
    let mut trace_markdown = String::new();
    trace_markdown.push_str("# Controlled Champion Fixed Loadout Trace\n\n");
    trace_markdown.push_str("## Rune Proc Telemetry\n");
    append_rune_proc_telemetry_markdown_entries(
        &mut trace_markdown,
        "- ",
        "  ",
        &rune_proc_telemetry,
        fixed_outcome.damage_dealt,
        fixed_outcome.healing_done,
    );
    trace_markdown.push_str("\n## Events\n");
    for entry in trace_events {
        trace_markdown.push_str("- ");
        trace_markdown.push_str(entry);
        trace_markdown.push('\n');
    }
    fs::write(&trace_markdown_path, trace_markdown)?;
    let trace_json = json!({
        "schema_version": trace_json_schema_version,
        "event_encoding": "structured",
        "determinism": deterministic_signature_json(trace_determinism),
        "rune_proc_telemetry": rune_proc_telemetry_json(
            &rune_proc_telemetry,
            fixed_outcome.damage_dealt,
            fixed_outcome.healing_done,
        ),
        "events": trace_events
            .iter()
            .map(|entry| structured_trace_event(entry))
            .collect::<Vec<_>>(),
    });
    fs::write(&trace_json_path, serde_json::to_string_pretty(&trace_json)?)?;

    let mut markdown_report = format!(
        "# Controlled Champion Fixed Loadout Evaluation\n\n- Scenario: `{}`\n- Search quality profile: `{}`\n- Controlled champion: `{}`\n- Build items: `{}`\n- Runes: `{}`\n- Shards: `{}`\n\n## Headline\n- Objective score: **{:.4}**\n- Outcome:\n  - Time alive: **{:.2}s**\n  - Damage dealt: **{:.1}**\n  - Healing done: **{:.1}**\n  - Enemy kills: **{}**\n  - Invulnerable seconds: **{:.2}s**\n\n## Objective Score Breakdown\n- Weighted-mean score: `{:.4}`\n- Worst-case scenario score: `{:.4}`\n- Worst-case blend weight: `{:.2}`\n- Final blended objective score: `{:.4}`\n- survival: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- damage: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- healing: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- enemy_kills: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- invulnerable_seconds: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n\n## Notes\n- This mode evaluates one fixed build and loadout directly; no candidate search or mutation is performed.\n- Trace markdown: `{}`\n- Trace json: `{}`\n",
        scenario_path.display(),
        search_quality_profile_key(options.search_quality_profile),
        controlled_champion_name,
        fixed_build_items
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        controlled_champion_loadout_selection.rune_names.join(", "),
        controlled_champion_loadout_selection.shard_stats.join(", "),
        fixed_score,
        fixed_outcome.time_alive_seconds,
        fixed_outcome.damage_dealt,
        fixed_outcome.healing_done,
        fixed_outcome.enemy_kills,
        fixed_outcome.invulnerable_seconds,
        fixed_breakdown.weighted_mean_score,
        fixed_breakdown.worst_case_score,
        fixed_breakdown.worst_case_weight,
        fixed_breakdown.final_score,
        fixed_breakdown.survival.weight,
        fixed_breakdown.survival.normalized_ratio,
        fixed_breakdown.survival.contribution,
        fixed_breakdown.survival.impact_percent,
        fixed_breakdown.damage.weight,
        fixed_breakdown.damage.normalized_ratio,
        fixed_breakdown.damage.contribution,
        fixed_breakdown.damage.impact_percent,
        fixed_breakdown.healing.weight,
        fixed_breakdown.healing.normalized_ratio,
        fixed_breakdown.healing.contribution,
        fixed_breakdown.healing.impact_percent,
        fixed_breakdown.enemy_kills.weight,
        fixed_breakdown.enemy_kills.normalized_ratio,
        fixed_breakdown.enemy_kills.contribution,
        fixed_breakdown.enemy_kills.impact_percent,
        fixed_breakdown.invulnerable_seconds.weight,
        fixed_breakdown.invulnerable_seconds.normalized_ratio,
        fixed_breakdown.invulnerable_seconds.contribution,
        fixed_breakdown.invulnerable_seconds.impact_percent,
        format_repo_relative_path(&trace_markdown_path),
        format_repo_relative_path(&trace_json_path),
    );
    markdown_report.push_str(&format!(
        "\n## Deterministic Replay Signature\n- Tick state checksum: `{:016x}`\n- Final state checksum: `{:016x}`\n- Queue checksum: `{:016x}`\n- Ticks executed: `{}`\n- Events processed: `{}`\n",
        trace_determinism.tick_state_checksum,
        trace_determinism.final_state_checksum,
        trace_determinism.queue_checksum,
        trace_determinism.ticks_executed,
        trace_determinism.events_processed,
    ));
    markdown_report.push_str("\n## Rune Proc Telemetry\n");
    if rune_proc_telemetry.is_empty() {
        markdown_report.push_str("- No rune procs were recorded.\n");
    } else {
        append_rune_proc_telemetry_markdown_entries(
            &mut markdown_report,
            "- ",
            "  ",
            &rune_proc_telemetry,
            fixed_outcome.damage_dealt,
            fixed_outcome.healing_done,
        );
    }
    markdown_report.push('\n');
    fs::write(&report_path, markdown_report)?;

    let structured_report = json!({
        "schema_version": report_json_schema_version,
        "scenario_path": scenario_path.display().to_string(),
        "search_quality_profile": search_quality_profile_key(options.search_quality_profile),
        "controlled_champion_name": controlled_champion_name,
        "items": fixed_build_items.iter().map(|item| item.name.clone()).collect::<Vec<_>>(),
        "runes": controlled_champion_loadout_selection.rune_names,
        "shards": controlled_champion_loadout_selection.shard_stats,
        "objective_score": fixed_score,
        "outcome": {
            "time_alive_seconds": fixed_outcome.time_alive_seconds,
            "damage_dealt": fixed_outcome.damage_dealt,
            "healing_done": fixed_outcome.healing_done,
            "enemy_kills": fixed_outcome.enemy_kills,
            "invulnerable_seconds": fixed_outcome.invulnerable_seconds
        },
        "objective_breakdown": {
            "weighted_mean_score": fixed_breakdown.weighted_mean_score,
            "worst_case_score": fixed_breakdown.worst_case_score,
            "worst_case_weight": fixed_breakdown.worst_case_weight,
            "final_score": fixed_breakdown.final_score,
            "components": {
                "survival": {
                    "weight": fixed_breakdown.survival.weight,
                    "normalized_ratio": fixed_breakdown.survival.normalized_ratio,
                    "contribution": fixed_breakdown.survival.contribution,
                    "impact_percent": fixed_breakdown.survival.impact_percent
                },
                "damage": {
                    "weight": fixed_breakdown.damage.weight,
                    "normalized_ratio": fixed_breakdown.damage.normalized_ratio,
                    "contribution": fixed_breakdown.damage.contribution,
                    "impact_percent": fixed_breakdown.damage.impact_percent
                },
                "healing": {
                    "weight": fixed_breakdown.healing.weight,
                    "normalized_ratio": fixed_breakdown.healing.normalized_ratio,
                    "contribution": fixed_breakdown.healing.contribution,
                    "impact_percent": fixed_breakdown.healing.impact_percent
                },
                "enemy_kills": {
                    "weight": fixed_breakdown.enemy_kills.weight,
                    "normalized_ratio": fixed_breakdown.enemy_kills.normalized_ratio,
                    "contribution": fixed_breakdown.enemy_kills.contribution,
                    "impact_percent": fixed_breakdown.enemy_kills.impact_percent
                },
                "invulnerable_seconds": {
                    "weight": fixed_breakdown.invulnerable_seconds.weight,
                    "normalized_ratio": fixed_breakdown.invulnerable_seconds.normalized_ratio,
                    "contribution": fixed_breakdown.invulnerable_seconds.contribution,
                    "impact_percent": fixed_breakdown.invulnerable_seconds.impact_percent
                }
            }
        },
        "reference_outcome": {
            "time_alive_seconds": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.time_alive_seconds)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64),
            "damage_dealt": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.damage_dealt)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64),
            "healing_done": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.healing_done)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64),
            "enemy_kills": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.enemy_kills)
                .sum::<usize>()
                / scenario_reference_outcomes.len().max(1),
            "invulnerable_seconds": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.invulnerable_seconds)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64)
        },
        "rune_proc_telemetry": rune_proc_telemetry_json(
            &rune_proc_telemetry,
            fixed_outcome.damage_dealt,
            fixed_outcome.healing_done,
        ),
        "determinism": deterministic_signature_json(trace_determinism),
        "notes": [
            "No search stage is run in controlled_champion_fixed_loadout mode.",
            "This report is intended for direct loadout-to-loadout comparisons."
        ]
    });
    fs::write(
        &json_report_path,
        serde_json::to_string_pretty(&structured_report)?,
    )?;

    println!(
        "Fixed-loadout report written: {}",
        format_repo_relative_path(&report_path)
    );
    println!(
        "Fixed-loadout JSON written: {}",
        format_repo_relative_path(&json_report_path)
    );
    println!(
        "Fixed-loadout trace written: {}",
        format_repo_relative_path(&trace_markdown_path)
    );
    println!(
        "Deterministic replay verification passed for fixed-loadout trace: tick={:016x} final={:016x} queue={:016x}",
        trace_determinism.tick_state_checksum,
        trace_determinism.final_state_checksum,
        trace_determinism.queue_checksum,
    );
    Ok(())
}

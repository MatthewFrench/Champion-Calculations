use anyhow::{Result, anyhow};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

use super::*;

pub(super) fn run_controlled_champion_fixed_loadout_evaluation_impl(
    scenario_path: &Path,
    options: &ControlledChampionFixedLoadoutOptions<'_>,
) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim = parse_simulation_config(simulation_config)?;
    sim.collect_rune_proc_telemetry = false;
    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim.champion_level;
    sim.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        simulation_config,
        simulation_level_before_controlled_override,
    );

    let mut controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim.champion_level);
    let controlled_champion_name = controlled_champion_base.name.clone();
    sim.controlled_champion_script = resolve_controlled_champion_script(&controlled_champion_name);

    let loadout_domain = build_loadout_domain();
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    if let Some(runes) = &options.fixed_rune_names {
        controlled_champion_loadout_selection.rune_names = runes.clone();
    }
    if let Some(shards) = &options.fixed_shard_stats {
        controlled_champion_loadout_selection.shard_stats = shards.clone();
    }
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    let controlled_champion_loadout = resolve_loadout(
        &controlled_champion_loadout_selection,
        sim.champion_level,
        true,
    )?;
    let fixed_build_items = item_pool_from_names(&items, &options.fixed_item_names)?;

    let enemy_scenarios_raw: Vec<ParsedOpponentEncounter> = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|encounter| {
            let scaled = encounter
                .actors
                .iter()
                .cloned()
                .map(|mut enemy| {
                    enemy.base = champion_at_level(&enemy.base, enemy.level);
                    enemy
                })
                .collect::<Vec<_>>();
            (encounter.name.clone(), encounter.weight, scaled)
        })
        .collect::<Vec<_>>();

    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;
    let mut enemy_build_scenarios = Vec::new();
    for (name, weight, enemies) in &enemy_scenarios {
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(&items, &preset.item_names)?;
            let preset_enemy_loadout =
                resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?;
            let enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }
    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, builds)| builds.clone())
        .unwrap_or_default();

    let mut search_cfg = parse_scenario_search_or_default(&scenario)?;
    apply_search_quality_profile(&mut search_cfg, options.search_quality_profile);
    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
        search_cfg.objective_enemy_kills_weight,
        search_cfg.objective_invulnerable_seconds_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            let damage_reference = enemy_builds_s
                .iter()
                .map(|(enemy, build, bonus_stats)| {
                    derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf).max_health
                })
                .sum::<f64>()
                .max(1.0);
            CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: damage_reference,
                healing_done: controlled_champion_base.base_health.max(1.0),
                enemy_kills: enemy_builds_s.len().max(1),
                invulnerable_seconds: sim.max_time_seconds.max(1.0),
            }
        })
        .collect::<Vec<_>>();
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
    let (fixed_score, fixed_outcome, fixed_breakdown) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            &objective_eval_ctx,
            &fixed_build_items,
            &controlled_champion_loadout.bonus_stats,
            Some(&controlled_champion_loadout_selection),
        );

    let run_label = options
        .fixed_eval_label
        .as_deref()
        .unwrap_or("fixed_loadout");
    let default_output_dir =
        default_fixed_loadout_output_directory(options.search_quality_profile, run_label);
    fs::create_dir_all(&default_output_dir)?;
    let controlled_champion_key = to_norm_key(&controlled_champion_name);
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
    let mut trace_sim = ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
        controlled_champion_base.clone(),
        &fixed_build_items,
        &controlled_champion_loadout.bonus_stats,
        Some(&controlled_champion_loadout_selection),
        None,
        Some(&controlled_champion_stack_overrides),
        &enemy_builds,
        trace_sim_cfg,
        urf.clone(),
    );
    trace_sim.enable_trace();
    while trace_sim.step(1) {}
    let trace_events = trace_sim.trace_events();
    let rune_proc_telemetry = trace_sim.controlled_champion_rune_proc_telemetry();
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
        "schema_version": FIXED_LOADOUT_TRACE_JSON_SCHEMA_VERSION,
        "event_encoding": "structured",
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
        "schema_version": FIXED_LOADOUT_REPORT_JSON_SCHEMA_VERSION,
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

    Ok(())
}

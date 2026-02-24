use anyhow::{Result, anyhow};
use std::path::Path;

use self::report_writing::{FixedLoadoutReportWriteContext, write_fixed_loadout_reports};
use super::controlled_champion_enemy_scenario_projection::{
    ControlledChampionEnemyBuildProjection, build_enemy_build_projection,
    build_scenario_reference_outcomes, parse_scaled_enemy_scenarios,
};
use super::*;

mod report_writing;

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
    sim.controlled_champion_script = Some(resolve_controlled_champion_script_or_error(
        &controlled_champion_name,
    )?);

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

    let enemy_scenarios = parse_scaled_enemy_scenarios(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    validate_world_positions_for_enemy_scenarios(&controlled_champion_name, &enemy_scenarios)?;

    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;
    let ControlledChampionEnemyBuildProjection {
        enemy_build_scenarios,
        enemy_builds,
    } = build_enemy_build_projection(&enemy_scenarios, &enemy_presets, &items)?;

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
    let scenario_reference_outcomes = build_scenario_reference_outcomes(
        &enemy_build_scenarios,
        &sim,
        &urf,
        &controlled_champion_base,
    );
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
    write_fixed_loadout_reports(FixedLoadoutReportWriteContext {
        scenario_path,
        options,
        controlled_champion_name: &controlled_champion_name,
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        controlled_champion_loadout_selection: &controlled_champion_loadout_selection,
        controlled_champion_loadout: &controlled_champion_loadout,
        fixed_build_items: &fixed_build_items,
        fixed_score,
        fixed_outcome: &fixed_outcome,
        fixed_breakdown: &fixed_breakdown,
        scenario_reference_outcomes: &scenario_reference_outcomes,
        enemy_builds: &enemy_builds,
        sim: &sim,
        urf: &urf,
        trace_json_schema_version: FIXED_LOADOUT_TRACE_JSON_SCHEMA_VERSION,
        report_json_schema_version: FIXED_LOADOUT_REPORT_JSON_SCHEMA_VERSION,
    })
}

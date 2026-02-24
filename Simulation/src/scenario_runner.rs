use anyhow::{Context, Result, anyhow};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::build_order::{acquisition_level_map, optimize_build_order};
use crate::cache::BlockingScoreCache;
use crate::data::{
    ensure_complete_loadout_selection, filter_loadout_domain_to_modeled_runes,
    is_legal_rune_page_selection,
};
use crate::defaults::protoplasm_lifeline_defaults;
use crate::engine::{
    ControlledChampionCombatSimulation, EnemyDerivedCombatStats, derive_enemy_combat_stats,
};
use crate::reporting::{
    write_controlled_champion_report_json, write_controlled_champion_report_markdown,
};
use crate::scripts::champions::{
    ChampionRuneProcTelemetryEntry, resolve_controlled_champion_script_result,
    supported_controlled_champion_script_keys,
};
use crate::scripts::coverage::is_item_effect_unmodeled;
use crate::search::{
    FullLoadoutSearchParams, adaptive_strategy_candidates_full_loadout,
    build_search_ranked_full_loadout, candidate_pareto_front_keys, choose_best_build_by_stat,
    compute_build_metrics_for_candidate, generate_bleed_candidates_full_loadout, item_names,
    portfolio_strategy_list, search_strategy_summary, select_diverse_top_candidates,
    strategy_seed_elites_full_loadout,
};
use crate::status::{StatusReporter, deadline_reached};

mod controlled_champion_candidate_search;
mod controlled_champion_enemy_scenario_projection;
mod controlled_champion_result_artifact_writing;
mod controlled_champion_result_build_analysis;
mod controlled_champion_result_reporting;
mod controlled_champion_result_reporting_projection;
mod controlled_champion_scenario_runner;
mod controlled_champion_scenario_setup;
mod controlled_champion_search_runtime_support;
mod controlled_champion_strict_ranking_finalization;
mod encounter_parsing;
mod fixed_loadout_runner;
mod progress_reporting;
mod run_output_paths;
mod rune_sweep_runner;
mod scenario_parsing;
mod search_space_estimation;
mod strict_ranking_ordering;

use self::controlled_champion_candidate_search::{
    CoverageStageRunContext, SeedAndStrictRankingRunContext, run_maximum_quality_coverage_stage,
    run_seed_and_strict_ranking,
};
use self::controlled_champion_scenario_runner::run_controlled_champion_scenario_impl;
use self::controlled_champion_search_runtime_support::{
    AtomicSearchTypeRuntimeCounter, CoverageStageDiagnostics, ShardedStringSet,
    SignificantProgressState, append_rune_proc_telemetry_markdown_entries,
    apply_level_scaled_sim_defaults_after_controlled_level_override,
    arm_time_budget_deadline_if_unset, build_enemy_similarity_notes,
    complete_partial_candidate_to_full, coverage_locked_assets,
    filter_item_pool_to_modeled_runtime_effects, fixed_sweep_keystone_seed_base,
    fixed_sweep_repeat_seed, max_legal_build_size, mutate_locked_candidate,
    partial_candidate_completion_seed, random_locked_candidate, rune_proc_telemetry_json,
    select_search_base_loadout_selection, structured_trace_event,
};
use self::encounter_parsing::{ParsedOpponentEncounter, parse_opponent_encounters};
use self::fixed_loadout_runner::run_controlled_champion_fixed_loadout_evaluation_impl;
use self::progress_reporting::{
    increment_search_type_counter, initialize_search_type_counters, snapshot_search_type_counters,
    unique_loadout_selection_count, unique_loadout_selection_count_from_ranked,
};
use self::run_output_paths::{
    default_fixed_loadout_output_directory, default_fixed_loadout_rune_sweep_output_directory,
    default_run_output_directory, format_repo_relative_path, search_quality_profile_key,
};
use self::rune_sweep_runner::run_controlled_champion_fixed_loadout_rune_sweep_impl;
use self::scenario_parsing::{parse_controlled_champion_config, parse_scenario_search_or_default};
use self::search_space_estimation::{
    estimate_close_to_optimal_probability, estimated_legal_item_build_count,
    estimated_legal_loadout_count, format_percent_display,
};
use self::strict_ranking_ordering::heuristic_sort_remaining_candidates_for_strict_ranking;
use super::*;

struct ControlledChampionScenarioConfig {
    base: ChampionBase,
    level: usize,
    loadout_selection: LoadoutSelection,
    stack_overrides: HashMap<String, f64>,
}

const FIXED_LOADOUT_TRACE_JSON_SCHEMA_VERSION: u32 = 3;
const FIXED_LOADOUT_REPORT_JSON_SCHEMA_VERSION: u32 = 2;
const FIXED_LOADOUT_RUNE_SWEEP_JSON_SCHEMA_VERSION: u32 = 2;
const CONTROLLED_CHAMPION_TRACE_JSON_SCHEMA_VERSION: u32 = 2;
const FIXED_SWEEP_REPEAT_SEED_STRIDE: u64 = 0x9E37_79B9_7F4A_7C15;

// Controlled-champion modes should fail fast when script coverage is missing so runs do not
// silently degrade into auto-attack-only behavior for unsupported champions.
pub(super) fn resolve_controlled_champion_script_or_error(
    controlled_champion_name: &str,
) -> Result<crate::scripts::champions::ControlledChampionScriptHandle> {
    let resolved_script = resolve_controlled_champion_script_result(controlled_champion_name)
        .with_context(|| {
            format!(
                "failed to initialize controlled-champion script for '{}'",
                controlled_champion_name
            )
        })?;
    resolved_script.ok_or_else(|| {
        let supported_champions = supported_controlled_champion_script_keys();
        let supported_champions = if supported_champions.is_empty() {
            "<none>".to_string()
        } else {
            supported_champions.join(", ")
        };
        anyhow!(
            "Controlled champion '{}' has no registered controlled-champion script. Supported controlled champions: {}.",
            controlled_champion_name,
            supported_champions
        )
    })
}

pub(super) fn validate_world_positions_for_enemy_scenarios(
    controlled_champion_name: &str,
    enemy_scenarios: &[(String, f64, Vec<EnemyConfig>)],
) -> Result<()> {
    for (encounter_name, _, enemies) in enemy_scenarios {
        crate::world::build_world_state_for_controlled_champion_encounter(
            controlled_champion_name,
            enemies,
        )
        .map_err(|err| {
            anyhow!(
                "Encounter '{}' has invalid world-position ownership data: {}",
                encounter_name,
                err
            )
        })?;
    }
    Ok(())
}

pub(super) fn run_controlled_champion_fixed_loadout_evaluation(
    scenario_path: &Path,
    options: &ControlledChampionFixedLoadoutOptions<'_>,
) -> Result<()> {
    run_controlled_champion_fixed_loadout_evaluation_impl(scenario_path, options)
}

pub(super) fn run_controlled_champion_fixed_loadout_rune_sweep(
    scenario_path: &Path,
    options: &ControlledChampionFixedLoadoutOptions<'_>,
) -> Result<()> {
    run_controlled_champion_fixed_loadout_rune_sweep_impl(scenario_path, options)
}

pub(super) fn run_controlled_champion_scenario(
    scenario_path: &Path,
    options: &ControlledChampionRunOptions<'_>,
) -> Result<()> {
    run_controlled_champion_scenario_impl(scenario_path, options)
}

pub(super) fn run_controlled_champion_stepper(scenario_path: &Path, ticks: usize) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim_cfg = parse_simulation_config(simulation_config)?;
    sim_cfg.collect_rune_proc_telemetry = false;
    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim_cfg.champion_level,
        &sim_cfg.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim_cfg.champion_level;
    sim_cfg.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim_cfg,
        simulation_config,
        simulation_level_before_controlled_override,
    );
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim_cfg.champion_level);
    sim_cfg.controlled_champion_script = Some(resolve_controlled_champion_script_or_error(
        &controlled_champion_base.name,
    )?);
    let controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;

    let enemy_encounters: Vec<ParsedOpponentEncounter> = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim_cfg.champion_level,
        &sim_cfg.stack_overrides,
    )?;
    let selected_encounter = enemy_encounters
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("opponents.encounters must include at least one encounter"))?;
    let selected_encounter_name = selected_encounter.name;
    let enemies = selected_encounter
        .actors
        .into_iter()
        .map(|mut e| {
            e.base = champion_at_level(&e.base, e.level);
            e
        })
        .collect::<Vec<_>>();
    crate::world::build_world_state_for_controlled_champion_encounter(
        &controlled_champion_base.name,
        &enemies,
    )
    .map_err(|err| {
        anyhow!(
            "Encounter '{}' has invalid world-position ownership data: {}",
            selected_encounter_name,
            err
        )
    })?;

    let loadout_domain = build_loadout_domain();
    let controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    let controlled_champion_loadout = resolve_loadout(
        &controlled_champion_loadout_selection,
        sim_cfg.champion_level,
        true,
    )?;
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;

    let mut enemy_builds: Vec<(EnemyConfig, Vec<Item>, Stats)> = Vec::new();
    for enemy in &enemies {
        let key = to_norm_key(&enemy.name);
        let preset = enemy_presets.get(&key).ok_or_else(|| {
            anyhow!(
                "Missing URF preset for enemy champion '{}'. Add it to {}.",
                enemy.name,
                enemy_preset_data_path().display()
            )
        })?;
        let build = item_pool_from_names(&items, &preset.item_names)?;
        let bonus_stats =
            resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?.bonus_stats;
        let mut enemy_with_loadout = enemy.clone();
        enemy_with_loadout.loadout_item_names = preset.item_names.clone();
        enemy_with_loadout.loadout_rune_names = preset.runes.clone();
        enemy_with_loadout.loadout_shards = preset.shards.clone();
        enemy_builds.push((enemy_with_loadout, build, bonus_stats));
    }
    let controlled_champion_items: Vec<Item> = Vec::new();

    let mut sim = ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
        controlled_champion_base,
        &controlled_champion_items,
        &controlled_champion_loadout.bonus_stats,
        Some(&controlled_champion_loadout_selection),
        None,
        Some(&controlled_champion_stack_overrides),
        &enemy_builds,
        sim_cfg.clone(),
        urf,
    );

    println!(
        "Server tick rate: {:.2} Hz ({:.5}s/tick)",
        sim_cfg.server_tick_rate_hz,
        sim.tick_seconds()
    );
    println!("Using opponent encounter: {}", selected_encounter_name);

    for tick in 0..ticks.max(1) {
        let alive = sim.step(1);
        let status = if alive { "alive" } else { "finished" };
        println!(
            "tick={} time={:.3}s health={:.2} targetable={} can_cast={} status={}",
            tick + 1,
            sim.current_time(),
            sim.current_health(),
            sim.is_targetable(),
            sim.can_cast(),
            status
        );
        if !alive {
            break;
        }
    }
    Ok(())
}

pub(super) fn run_stat_optimization(
    stat_key: &str,
    scenario_path: &Path,
    label: &str,
) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let scenario = load_json(scenario_path)?;
    let search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    let item_pool = default_item_pool(&items, &urf);

    let build_indices = choose_best_build_by_stat(
        &item_pool,
        stat_key,
        search_cfg.max_items,
        search_cfg.beam_width,
    );
    let build = build_from_indices(&item_pool, &build_indices);
    let stats = build_item_stats(&build);
    let value = stats.get_stat(stat_key);

    println!("Best build for {}:", label);
    println!(
        "- Items: {}",
        build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Total {}: {:.2}", label, value);
    Ok(())
}

#[cfg(test)]
#[path = "tests/scenario_runner_tests.rs"]
mod tests;

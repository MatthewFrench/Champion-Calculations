#![recursion_limit = "512"]

use anyhow::Result;
use clap::Parser;
use rayon::ThreadPoolBuilder;

mod build_order;
mod cache;
mod core;
mod data;
mod defaults;
mod engine;
mod reporting;
mod respawn;
mod scenario_runner;
mod scripts;
mod search;
mod simulation_contracts;
mod status;
mod world;

pub(crate) use crate::core::*;
pub(crate) use crate::data::{
    EnemyUrfPreset, apply_search_quality_profile, build_loadout_domain, default_item_pool,
    enemy_loadout_from_preset, enemy_preset_data_path, item_pool_from_names, load_champion_bases,
    load_enemy_urf_presets, load_items, load_json, load_urf_buffs, loadout_selection_key,
    lookup_champion_base, parse_build_search, parse_enemy_config, parse_loadout_selection,
    parse_simulation_config, random_loadout_selection, resolve_loadout, resolve_scenario_path,
    simulation_dir, to_norm_key, validate_enemy_urf_presets,
};
use crate::scenario_runner::{
    run_controlled_champion_fixed_loadout_evaluation,
    run_controlled_champion_fixed_loadout_rune_sweep, run_controlled_champion_scenario,
    run_controlled_champion_stepper, run_stat_optimization,
};
pub(crate) use crate::simulation_contracts::*;

const EXCLUDED_RANKS: &[&str] = &["CONSUMABLE", "TRINKET"];
const LEGENDARY_RANK: &str = "LEGENDARY";
const ITEM_EVOLUTION_REPLACEMENTS: &[(&str, &str)] = &[
    ("Manamune", "Muramana"),
    ("Archangel's Staff", "Seraph's Embrace"),
];

fn main() -> Result<()> {
    let cli = Cli::parse();
    let available = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let default_threads = available.saturating_sub(1).max(1);
    let threads = cli.threads.unwrap_or(default_threads).max(1);
    let _ = ThreadPoolBuilder::new().num_threads(threads).build_global();

    let scenario_path = resolve_scenario_path(&cli.scenario);
    match cli.mode {
        Mode::ControlledChampion => run_controlled_champion_scenario(
            &scenario_path,
            &ControlledChampionRunOptions {
                top_x: cli.top_x,
                min_item_diff: cli.min_item_diff,
                max_relative_gap_percent: cli.max_relative_gap_percent,
                report_path_override: cli.report_path.as_deref(),
                max_runtime_seconds: cli.max_runtime_seconds,
                popcorn_window_seconds: cli.popcorn_window_seconds,
                popcorn_min_relative_improvement_percent: cli
                    .popcorn_min_relative_improvement_percent,
                status_every_seconds: cli.status_every_seconds,
                search_quality_profile: cli.search_quality_profile,
                seed_override: cli.seed,
            },
        ),
        Mode::ControlledChampionFixedLoadout => {
            let fixed_item_names = parse_csv_arg(
                cli.fixed_item_names.as_deref(),
                "--fixed-item-names is required for --mode controlled_champion_fixed_loadout",
            )?;
            let fixed_rune_names = parse_optional_csv_arg(cli.fixed_rune_names.as_deref());
            let fixed_shard_stats = parse_optional_csv_arg(cli.fixed_shard_stats.as_deref());
            run_controlled_champion_fixed_loadout_evaluation(
                &scenario_path,
                &ControlledChampionFixedLoadoutOptions {
                    report_path_override: cli.report_path.as_deref(),
                    search_quality_profile: cli.search_quality_profile,
                    fixed_item_names,
                    fixed_rune_names,
                    fixed_shard_stats,
                    fixed_eval_label: cli.fixed_eval_label,
                    fixed_sweep_seed_repeats: cli.fixed_sweep_seed_repeats.max(1),
                },
            )
        }
        Mode::ControlledChampionFixedLoadoutRuneSweep => {
            let fixed_item_names = parse_csv_arg(
                cli.fixed_item_names.as_deref(),
                "--fixed-item-names is required for --mode controlled_champion_fixed_loadout_rune_sweep",
            )?;
            let fixed_rune_names = parse_optional_csv_arg(cli.fixed_rune_names.as_deref());
            let fixed_shard_stats = parse_optional_csv_arg(cli.fixed_shard_stats.as_deref());
            run_controlled_champion_fixed_loadout_rune_sweep(
                &scenario_path,
                &ControlledChampionFixedLoadoutOptions {
                    report_path_override: cli.report_path.as_deref(),
                    search_quality_profile: cli.search_quality_profile,
                    fixed_item_names,
                    fixed_rune_names,
                    fixed_shard_stats,
                    fixed_eval_label: cli.fixed_eval_label,
                    fixed_sweep_seed_repeats: cli.fixed_sweep_seed_repeats.max(1),
                },
            )
        }
        Mode::ControlledChampionStep => run_controlled_champion_stepper(&scenario_path, cli.ticks),
        Mode::TaricAs => {
            run_stat_optimization("attack_speed_percent", &scenario_path, "attack speed")
        }
        Mode::HecarimMs => run_stat_optimization("move_speed_flat", &scenario_path, "move speed"),
    }
}

fn parse_csv_arg(value: Option<&str>, missing_message: &str) -> Result<Vec<String>> {
    let raw = value.ok_or_else(|| anyhow::anyhow!(missing_message.to_string()))?;
    let values = raw
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if values.is_empty() {
        return Err(anyhow::anyhow!(missing_message.to_string()));
    }
    Ok(values)
}

fn parse_optional_csv_arg(value: Option<&str>) -> Option<Vec<String>> {
    value.and_then(|raw| {
        let values = raw
            .split(',')
            .map(str::trim)
            .filter(|entry| !entry.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        (!values.is_empty()).then_some(values)
    })
}

#[cfg(test)]
#[path = "tests/main_tests.rs"]
mod tests;

#![recursion_limit = "512"]

use anyhow::Result;
use clap::{Parser, ValueEnum};
use rayon::ThreadPoolBuilder;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::path::Path;

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
mod status;

pub(crate) use crate::core::*;
pub(crate) use crate::data::{
    EnemyUrfPreset, apply_search_quality_profile, build_loadout_domain, default_item_pool,
    enemy_loadout_from_preset, enemy_preset_data_path, item_pool_from_names, load_champion_bases,
    load_enemy_urf_presets, load_items, load_json, load_urf_buffs, loadout_selection_key,
    lookup_champion_base, parse_build_search, parse_enemy_config, parse_loadout_selection,
    parse_simulation_config, random_loadout_selection, resolve_loadout, resolve_scenario_path,
    simulation_dir, to_norm_key, validate_enemy_urf_presets,
};
use crate::engine::EnemyDerivedCombatStats;
use crate::scenario_runner::{
    run_controlled_champion_fixed_loadout_evaluation,
    run_controlled_champion_fixed_loadout_rune_sweep, run_controlled_champion_scenario,
    run_controlled_champion_stepper, run_stat_optimization,
};
use crate::scripts::champions::ChampionRuneProcTelemetryEntry;

const EXCLUDED_RANKS: &[&str] = &["CONSUMABLE", "TRINKET"];
const LEGENDARY_RANK: &str = "LEGENDARY";
const ITEM_EVOLUTION_REPLACEMENTS: &[(&str, &str)] = &[
    ("Manamune", "Muramana"),
    ("Archangel's Staff", "Seraph's Embrace"),
];

#[derive(Debug, Clone, Default)]
struct Stats {
    ability_power: f64,
    health: f64,
    armor: f64,
    magic_resist: f64,
    attack_damage: f64,
    attack_speed_percent: f64,
    ability_haste: f64,
    move_speed_flat: f64,
    move_speed_percent: f64,
    crit_chance_percent: f64,
}

impl Stats {
    fn add(&mut self, other: &Stats) {
        self.ability_power += other.ability_power;
        self.health += other.health;
        self.armor += other.armor;
        self.magic_resist += other.magic_resist;
        self.attack_damage += other.attack_damage;
        self.attack_speed_percent += other.attack_speed_percent;
        self.ability_haste += other.ability_haste;
        self.move_speed_flat += other.move_speed_flat;
        self.move_speed_percent += other.move_speed_percent;
        self.crit_chance_percent += other.crit_chance_percent;
    }

    fn get_stat(&self, key: &str) -> f64 {
        match key {
            "ability_power" => self.ability_power,
            "health" => self.health,
            "armor" => self.armor,
            "magic_resist" => self.magic_resist,
            "attack_damage" => self.attack_damage,
            "attack_speed_percent" => self.attack_speed_percent,
            "ability_haste" => self.ability_haste,
            "move_speed_flat" => self.move_speed_flat,
            "move_speed_percent" => self.move_speed_percent,
            "crit_chance_percent" => self.crit_chance_percent,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    stats: Stats,
    rank: Vec<String>,
    shop_purchasable: bool,
    total_cost: f64,
    passive_effects_text: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ChampionBase {
    name: String,
    base_health: f64,
    health_per_level: f64,
    base_armor: f64,
    armor_per_level: f64,
    base_magic_resist: f64,
    magic_resist_per_level: f64,
    base_attack_damage: f64,
    attack_damage_per_level: f64,
    base_attack_speed: f64,
    attack_speed_per_level_percent: f64,
    base_attack_range: f64,
    base_attack_projectile_speed: f64,
    base_move_speed: f64,
    is_melee: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum OpponentMovementMode {
    HoldPosition,
    #[default]
    MaintainCombatRange,
}

#[derive(Debug, Clone)]
struct EnemyConfig {
    id: String,
    name: String,
    level: usize,
    base: ChampionBase,
    spawn_position_xy: Option<(f64, f64)>,
    movement_mode: OpponentMovementMode,
    loadout_item_names: Vec<String>,
    loadout_rune_names: Vec<String>,
    loadout_shards: Vec<String>,
    stack_overrides: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
struct SimulationConfig {
    dt: f64,
    server_tick_rate_hz: f64,
    champion_level: usize,
    max_time_seconds: f64,
    combat_seed: Option<u64>,
    collect_rune_proc_telemetry: bool,
    controlled_champion_script: Option<crate::scripts::champions::ControlledChampionScriptHandle>,
    zhonya_duration_seconds: f64,
    zhonya_cooldown_seconds: f64,
    zhonya_trigger_health_percent: f64,
    ga_cooldown_seconds: f64,
    ga_revive_duration_seconds: f64,
    ga_revive_base_health_ratio: f64,
    protoplasm_trigger_health_percent: f64,
    protoplasm_bonus_health: f64,
    protoplasm_heal_total: f64,
    protoplasm_duration_seconds: f64,
    stack_overrides: HashMap<String, f64>,
    urf_respawn_flat_reduction_seconds: f64,
    urf_respawn_extrapolation_per_level: f64,
    urf_respawn_time_scaling_enabled: bool,
    urf_respawn_time_scaling_start_seconds: f64,
    urf_respawn_time_scaling_per_minute_seconds: f64,
    urf_respawn_time_scaling_cap_seconds: f64,
}

#[derive(Debug, Clone)]
struct UrfBuffs {
    ability_haste: f64,
    item_haste: f64,
    health_cost_multiplier: f64,
    bonus_attack_speed_multiplier_melee: f64,
    bonus_attack_speed_multiplier_ranged: f64,
    allowed_item_keys: HashSet<String>,
}

#[derive(Debug, Clone)]
struct BuildSearchConfig {
    strategy: String,
    beam_width: usize,
    max_items: usize,
    random_samples: usize,
    hill_climb_restarts: usize,
    hill_climb_steps: usize,
    hill_climb_neighbors: usize,
    genetic_population: usize,
    genetic_generations: usize,
    genetic_mutation_rate: f64,
    genetic_crossover_rate: f64,
    portfolio_strategies: Vec<String>,
    ranked_limit: usize,
    simulated_annealing_restarts: usize,
    simulated_annealing_iterations: usize,
    simulated_annealing_initial_temp: f64,
    simulated_annealing_cooling_rate: f64,
    mcts_iterations: usize,
    mcts_rollouts_per_expansion: usize,
    mcts_exploration: f64,
    ensemble_seeds: usize,
    ensemble_seed_stride: u64,
    ensemble_seed_top_k: usize,
    objective_survival_weight: f64,
    objective_damage_weight: f64,
    objective_healing_weight: f64,
    objective_enemy_kills_weight: f64,
    objective_invulnerable_seconds_weight: f64,
    robust_min_seed_hit_rate: f64,
    bleed_enabled: bool,
    bleed_budget: usize,
    bleed_mutation_rate: f64,
    multi_scenario_worst_weight: f64,
    strict_ranking_enable_heuristic_ordering: bool,
    strict_ranking_rune_signal_weight: f64,
    strict_ranking_shard_signal_weight: f64,
    strict_ranking_exploration_promotions: usize,
    unmodeled_rune_hard_gate: bool,
    unmodeled_rune_penalty_per_rune: f64,
    seed: u64,
}

#[derive(Debug, Clone)]
struct BuildMetrics {
    objective: f64,
    ehp_mixed: f64,
    ap: f64,
    cost_timing: f64,
    total_cost: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct CombatOutcome {
    time_alive_seconds: f64,
    damage_dealt: f64,
    healing_done: f64,
    enemy_kills: usize,
    invulnerable_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
struct ObjectiveComponentWeights {
    survival: f64,
    damage: f64,
    healing: f64,
    enemy_kills: f64,
    invulnerable_seconds: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct ObjectiveComponentImpact {
    weight: f64,
    normalized_ratio: f64,
    contribution: f64,
    impact_percent: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct ObjectiveScoreBreakdown {
    weighted_mean_score: f64,
    worst_case_score: f64,
    worst_case_weight: f64,
    final_score: f64,
    survival: ObjectiveComponentImpact,
    damage: ObjectiveComponentImpact,
    healing: ObjectiveComponentImpact,
    enemy_kills: ObjectiveComponentImpact,
    invulnerable_seconds: ObjectiveComponentImpact,
}

#[derive(Debug, Clone)]
struct BuildConfidence {
    key: BuildKey,
    seed_hits: usize,
    seed_hit_rate: f64,
    robustness: String,
}

#[derive(Debug, Clone)]
struct SearchTypeBreakdown {
    name: String,
    score_requests: usize,
    new_simulations: usize,
    persistent_cache_hits: usize,
}

#[derive(Debug, Clone)]
struct SearchDiagnostics {
    strategy_summary: String,
    search_quality_profile: String,
    effective_seed: u64,
    ensemble_seeds: usize,
    effective_threads: usize,
    seed_orchestration_parallel: bool,
    portfolio_strategy_parallel: bool,
    strategy_elites_parallel: bool,
    objective_survival_weight: f64,
    objective_damage_weight: f64,
    objective_healing_weight: f64,
    objective_enemy_kills_weight: f64,
    objective_invulnerable_seconds_weight: f64,
    full_evaluations: usize,
    full_cache_hits: usize,
    full_cache_misses: usize,
    full_cache_waits: usize,
    full_persistent_cache_hits: usize,
    full_persistent_cache_entries: usize,
    candidate_keys_generated: usize,
    candidate_duplicates_pruned: usize,
    unique_candidate_builds: usize,
    bleed_candidates_injected: usize,
    adaptive_candidates_injected: usize,
    scenario_count: usize,
    loadout_candidates: usize,
    loadout_finalists: usize,
    strict_seed_scored_candidates: usize,
    strict_remaining_candidates: usize,
    strict_non_finite_candidates: usize,
    strict_candidates_skipped_timeout: usize,
    strict_completion_percent: f64,
    strict_heuristic_ordering_enabled: bool,
    strict_ranking_rune_signal_weight: f64,
    strict_ranking_shard_signal_weight: f64,
    strict_random_promotions_done: usize,
    unmodeled_rune_hard_gate: bool,
    unmodeled_rune_penalty_per_rune: f64,
    unmodeled_rune_candidates_rejected: usize,
    unmodeled_rune_candidates_penalized: usize,
    unique_scored_candidates: usize,
    time_budget_seconds: Option<f64>,
    popcorn_window_seconds: Option<f64>,
    popcorn_min_relative_improvement_percent: f64,
    significant_improvement_events: usize,
    best_significant_score: Option<f64>,
    seconds_since_last_significant_improvement: Option<f64>,
    search_type_breakdown: Vec<SearchTypeBreakdown>,
    estimated_total_candidate_space: Option<f64>,
    estimated_run_space_coverage_percent: Option<f64>,
    estimated_cache_space_coverage_percent: Option<f64>,
    estimated_close_to_optimal_probability: Option<f64>,
    estimated_close_to_optimal_probability_note: String,
    coverage_stage_enabled: bool,
    coverage_stage_elapsed_seconds: f64,
    coverage_stage_assets_total: usize,
    coverage_stage_assets_covered: usize,
    coverage_stage_seed_candidates: usize,
    coverage_stage_seed_candidates_unique: usize,
    coverage_stage_incomplete: bool,
    coverage_stage_warning: String,
    elapsed_seconds: f64,
    total_run_seconds: f64,
    timed_out: bool,
    processed_candidates: usize,
    total_candidates: usize,
    seed_best_scores: Vec<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct LoadoutSelection {
    rune_names: Vec<String>,
    shard_stats: Vec<String>,
}

#[derive(Debug, Clone, Default)]
struct ResolvedLoadout {
    selection_labels: Vec<String>,
    bonus_stats: Stats,
    applied_notes: Vec<String>,
    skipped_notes: Vec<String>,
    unmodeled_rune_names: Vec<String>,
}

#[derive(Debug, Clone)]
struct BuildOrderResult {
    ordered_items: Vec<Item>,
    levels: Vec<usize>,
    acquired_levels: Vec<usize>,
    stage_survival: Vec<f64>,
    stage_damage: Vec<f64>,
    stage_healing: Vec<f64>,
    stage_objective_scores: Vec<f64>,
    cumulative_score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BuildCandidateKey {
    item_indices: Vec<usize>,
    loadout_selection: LoadoutSelection,
}

type BuildKey = BuildCandidateKey;
type EnemyBuildEntry = (EnemyConfig, Vec<Item>, Stats);
type EnemyBuildScenario = (String, f64, Vec<EnemyBuildEntry>);
type ResolvedByCandidateMap = HashMap<BuildKey, ResolvedLoadout>;
type OutcomeByCandidateMap = HashMap<BuildKey, CombatOutcome>;

struct ObjectiveEvalContext<'a> {
    controlled_champion_base: &'a ChampionBase,
    controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    enemy_build_scenarios: &'a [EnemyBuildScenario],
    sim: &'a SimulationConfig,
    urf: &'a UrfBuffs,
    scenario_reference_outcomes: &'a [CombatOutcome],
    weights: ObjectiveComponentWeights,
    worst_case_weight: f64,
}

struct BuildOrderEvalContext<'a> {
    controlled_champion_base_raw: &'a ChampionBase,
    controlled_champion_bonus_stats: &'a Stats,
    controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    enemy_builds: &'a [EnemyBuildEntry],
    raw_enemy_bases: &'a HashMap<String, ChampionBase>,
    sim: &'a SimulationConfig,
    urf: &'a UrfBuffs,
    objective_weights: ObjectiveComponentWeights,
}

struct ControlledChampionReportData<'a> {
    scenario_path: &'a Path,
    controlled_champion_name: &'a str,
    sim: &'a SimulationConfig,
    controlled_champion_base_level: &'a ChampionBase,
    controlled_champion_end_stats: &'a Stats,
    stack_notes: &'a [String],
    controlled_champion_loadout: &'a ResolvedLoadout,
    enemy_loadout: &'a ResolvedLoadout,
    best_build: &'a [Item],
    best_score: f64,
    best_outcome: &'a CombatOutcome,
    best_rune_proc_telemetry: &'a [ChampionRuneProcTelemetryEntry],
    best_score_breakdown: ObjectiveScoreBreakdown,
    enemy_builds: &'a [EnemyBuildEntry],
    enemy_derived_combat_stats: &'a [EnemyDerivedCombatStats],
    enemy_similarity_notes: &'a [String],
    enemy_presets_used: &'a HashMap<String, EnemyUrfPreset>,
    diverse_top_builds: &'a [(Vec<Item>, f64)],
    diverse_top_keys: &'a [BuildKey],
    build_confidence: &'a [BuildConfidence],
    metrics_by_key: &'a HashMap<BuildKey, BuildMetrics>,
    pareto_front: &'a HashSet<BuildKey>,
    diagnostics: &'a SearchDiagnostics,
    build_orders: &'a [BuildOrderResult],
}

#[derive(Debug, Clone, Parser)]
#[command(about = "URF controlled champion objective simulator")]
struct Cli {
    #[arg(
        long,
        help = "Scenario path or scenario name (resolved as Simulation/scenarios/<name>.json)"
    )]
    scenario: String,
    #[arg(long, value_enum, default_value_t = Mode::ControlledChampion)]
    mode: Mode,
    #[arg(long, default_value_t = 30)]
    ticks: usize,
    #[arg(long, default_value_t = 8)]
    top_x: usize,
    #[arg(long, default_value_t = 2)]
    min_item_diff: usize,
    #[arg(long, default_value_t = 5.0)]
    max_relative_gap_percent: f64,
    #[arg(long)]
    report_path: Option<String>,
    #[arg(long)]
    threads: Option<usize>,
    #[arg(long)]
    max_runtime_seconds: Option<f64>,
    #[arg(
        long,
        help = "Popcorn mode: continue running while significant objective improvements keep occurring within this window"
    )]
    popcorn_window_seconds: Option<f64>,
    #[arg(long, default_value_t = 1.0)]
    popcorn_min_relative_improvement_percent: f64,
    #[arg(long, default_value_t = 10.0)]
    status_every_seconds: f64,
    #[arg(long, value_enum, default_value_t = SearchQualityProfile::MaximumQuality)]
    search_quality_profile: SearchQualityProfile,
    #[arg(
        long,
        help = "Deterministic search seed override (default behavior is random)"
    )]
    seed: Option<u64>,
    #[arg(
        long,
        help = "Comma-separated fixed item names for controlled_champion_fixed_loadout mode"
    )]
    fixed_item_names: Option<String>,
    #[arg(
        long,
        help = "Optional comma-separated rune names override (six runes) for controlled_champion_fixed_loadout mode"
    )]
    fixed_rune_names: Option<String>,
    #[arg(
        long,
        help = "Optional comma-separated shard stat override (three shards) for controlled_champion_fixed_loadout mode"
    )]
    fixed_shard_stats: Option<String>,
    #[arg(
        long,
        help = "Optional report folder label for controlled_champion_fixed_loadout mode"
    )]
    fixed_eval_label: Option<String>,
    #[arg(
        long,
        default_value_t = 1,
        help = "Optional repeat count per keystone for controlled_champion_fixed_loadout_rune_sweep (enables multi-seed-ready aggregation)"
    )]
    fixed_sweep_seed_repeats: usize,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    #[value(name = "controlled_champion", alias = "vladimir")]
    ControlledChampion,
    #[value(name = "controlled_champion_fixed_loadout")]
    ControlledChampionFixedLoadout,
    #[value(name = "controlled_champion_fixed_loadout_rune_sweep")]
    ControlledChampionFixedLoadoutRuneSweep,
    #[value(name = "controlled_champion_step", alias = "vladimir_step")]
    ControlledChampionStep,
    #[value(name = "taric_as")]
    TaricAs,
    #[value(name = "hecarim_ms")]
    HecarimMs,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SearchQualityProfile {
    #[value(name = "fast")]
    Fast,
    #[value(name = "balanced")]
    Balanced,
    #[value(name = "maximum_quality")]
    MaximumQuality,
}

#[derive(Debug, Clone, Copy)]
struct ControlledChampionRunOptions<'a> {
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
    report_path_override: Option<&'a str>,
    max_runtime_seconds: Option<f64>,
    popcorn_window_seconds: Option<f64>,
    popcorn_min_relative_improvement_percent: f64,
    status_every_seconds: f64,
    search_quality_profile: SearchQualityProfile,
    seed_override: Option<u64>,
}

#[derive(Debug, Clone)]
struct ControlledChampionFixedLoadoutOptions<'a> {
    report_path_override: Option<&'a str>,
    search_quality_profile: SearchQualityProfile,
    fixed_item_names: Vec<String>,
    fixed_rune_names: Option<Vec<String>>,
    fixed_shard_stats: Option<Vec<String>>,
    fixed_eval_label: Option<String>,
    fixed_sweep_seed_repeats: usize,
}

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

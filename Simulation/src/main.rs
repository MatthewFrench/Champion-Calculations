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
    enemy_loadout_from_preset, enemy_preset_data_path, ensure_item_names_allowed_in_urf,
    item_pool_from_names, load_champion_bases, load_enemy_urf_presets, load_items, load_json,
    load_urf_buffs, loadout_selection_key, lookup_champion_base, parse_build_search,
    parse_enemy_config, parse_loadout_selection, parse_simulation_config, random_loadout_selection,
    resolve_loadout, resolve_scenario_path, simulation_dir, to_norm_key,
    validate_enemy_urf_presets,
};
use crate::engine::EnemyDerivedCombatStats;
use crate::scenario_runner::{
    run_controlled_champion_scenario, run_controlled_champion_stepper, run_stat_optimization,
};

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
    vlad_pool_rank: usize,
    vlad_pool_untargetable_seconds: f64,
    vlad_pool_cost_percent_current_health: f64,
    vlad_pool_heal_ratio_of_damage: f64,
    vlad_pool_base_damage_by_rank: Vec<f64>,
    vlad_pool_base_cooldown_seconds_by_rank: Vec<f64>,
    vlad_pool_bonus_health_ratio: f64,
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
    vlad_q_base_damage: f64,
    vlad_q_ap_ratio: f64,
    vlad_q_heal_ratio_of_damage: f64,
    vlad_q_base_cooldown_seconds: f64,
    vlad_e_base_damage: f64,
    vlad_e_ap_ratio: f64,
    vlad_e_base_cooldown_seconds: f64,
    vlad_r_base_damage: f64,
    vlad_r_ap_ratio: f64,
    vlad_r_base_cooldown_seconds: f64,
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
    ensemble_seeds: usize,
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
    baseline_build: &'a [Item],
    baseline_score: f64,
    baseline_outcome: &'a CombatOutcome,
    baseline_score_breakdown: ObjectiveScoreBreakdown,
    best_build: &'a [Item],
    best_score: f64,
    best_outcome: &'a CombatOutcome,
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
#[command(about = "URF Vladimir objective simulator")]
struct Cli {
    #[arg(
        long,
        help = "Scenario path or scenario name (resolved as Simulation/scenarios/<name>.json)"
    )]
    scenario: String,
    #[arg(long, value_enum, default_value_t = Mode::Vladimir)]
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
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    #[value(name = "vladimir")]
    Vladimir,
    #[value(name = "vladimir_step")]
    VladimirStep,
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
        Mode::Vladimir => run_controlled_champion_scenario(
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
            },
        ),
        Mode::VladimirStep => run_controlled_champion_stepper(&scenario_path, cli.ticks),
        Mode::TaricAs => {
            run_stat_optimization("attack_speed_percent", &scenario_path, "attack speed")
        }
        Mode::HecarimMs => run_stat_optimization("move_speed_flat", &scenario_path, "move speed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loadout_selection_key_is_order_sensitive() {
        let a = LoadoutSelection {
            rune_names: vec!["Triumph".to_string(), "Lethal Tempo".to_string()],
            shard_stats: vec!["adaptive".to_string(), "health".to_string()],
        };
        let b = LoadoutSelection {
            rune_names: vec!["Lethal Tempo".to_string(), "Triumph".to_string()],
            shard_stats: vec!["health".to_string(), "adaptive".to_string()],
        };
        assert_ne!(
            loadout_selection_key(&a),
            loadout_selection_key(&b),
            "rune and shard order must remain slot-aware"
        );
    }

    #[test]
    fn compute_vladimir_stats_does_not_recursively_reapply_conversions() {
        let base = ChampionBase {
            name: "Vladimir".to_string(),
            base_health: 1000.0,
            health_per_level: 0.0,
            base_armor: 30.0,
            armor_per_level: 0.0,
            base_magic_resist: 30.0,
            magic_resist_per_level: 0.0,
            base_attack_damage: 60.0,
            attack_damage_per_level: 0.0,
            base_attack_speed: 0.658,
            attack_speed_per_level_percent: 0.0,
            base_attack_range: 450.0,
            base_attack_projectile_speed: 1600.0,
            base_move_speed: 340.0,
            is_melee: false,
        };
        let item_stats = Stats {
            ability_power: 100.0,
            health: 200.0,
            ..Stats::default()
        };
        let out = compute_champion_final_stats(&base, &item_stats);
        let expected_ap = 100.0 + 0.033 * 200.0;
        let expected_health = 1000.0 + 200.0 + 1.6 * 100.0;
        assert!((out.ability_power - expected_ap).abs() < 1e-9);
        assert!((out.health - expected_health).abs() < 1e-9);
    }

    #[test]
    fn enemy_preset_data_validates_against_local_data() {
        let presets = load_enemy_urf_presets().expect("enemy presets should load");
        let items = load_items().expect("items should load");
        let domain = build_loadout_domain();
        let urf = load_urf_buffs().expect("urf config should load");
        validate_enemy_urf_presets(&presets, &items, &domain, &urf)
            .expect("enemy preset validation should pass");
    }

    #[test]
    fn random_loadout_generation_produces_legal_shapes() {
        let domain = build_loadout_domain();
        assert!(domain.rune_paths.len() >= 2);
        assert!(domain.shard_slots.iter().all(|s| !s.is_empty()));

        let base = LoadoutSelection::default();
        let mut seed = 1337u64;
        for _ in 0..64 {
            let sample = random_loadout_selection(&base, &domain, &mut seed);
            assert_eq!(sample.rune_names.len(), 6);
            assert_eq!(sample.shard_stats.len(), 3);
        }
    }

    #[test]
    fn objective_weights_and_scoring_are_normalized() {
        let w = normalized_objective_weights(0.50, 0.25, 0.15, 0.10, 0.05);
        assert!(
            (w.survival + w.damage + w.healing + w.enemy_kills + w.invulnerable_seconds - 1.0)
                .abs()
                < 1e-9
        );

        let reference = CombatOutcome {
            time_alive_seconds: 20.0,
            damage_dealt: 8000.0,
            healing_done: 2000.0,
            enemy_kills: 2,
            invulnerable_seconds: 1.0,
        };
        let baseline_score = objective_score_from_outcome(reference, reference, w);
        assert!((baseline_score - 1.0).abs() < 1e-9);

        let better = CombatOutcome {
            time_alive_seconds: 22.0,
            damage_dealt: 8800.0,
            healing_done: 2400.0,
            enemy_kills: 3,
            invulnerable_seconds: 2.0,
        };
        assert!(objective_score_from_outcome(better, reference, w) > baseline_score);

        let kills_only_upgrade = CombatOutcome {
            enemy_kills: reference.enemy_kills + 1,
            ..reference
        };
        assert!(objective_score_from_outcome(kills_only_upgrade, reference, w) > baseline_score);

        let invulnerable_only_upgrade = CombatOutcome {
            invulnerable_seconds: reference.invulnerable_seconds + 1.0,
            ..reference
        };
        assert!(
            objective_score_from_outcome(invulnerable_only_upgrade, reference, w) > baseline_score
        );
    }

    #[test]
    fn urf_respawn_timer_scales_with_level() {
        let tuning = respawn::UrfRespawnTuning {
            urf_flat_reduction_seconds: 3.0,
            extrapolation_per_level: 2.5,
            time_scaling_enabled: true,
            time_scaling_start_seconds: 300.0,
            time_scaling_per_minute_seconds: 0.4,
            time_scaling_cap_seconds: 20.0,
        };
        let mut prev = 0.0;
        for lvl in 1..=30 {
            let t = respawn::urf_respawn_delay_seconds(lvl, 600.0, tuning);
            assert!(t >= 1.0);
            assert!(t >= prev);
            prev = t;
        }
        let no_scale_tuning = respawn::UrfRespawnTuning {
            time_scaling_enabled: false,
            ..tuning
        };
        assert!((respawn::urf_respawn_delay_seconds(1, 0.0, no_scale_tuning) - 7.0).abs() < 1e-9);
    }

    #[test]
    fn urf_respawn_timer_increases_with_game_time_after_scaling_start() {
        let tuning = respawn::UrfRespawnTuning {
            urf_flat_reduction_seconds: 3.0,
            extrapolation_per_level: 2.5,
            time_scaling_enabled: true,
            time_scaling_start_seconds: 300.0,
            time_scaling_per_minute_seconds: 0.4,
            time_scaling_cap_seconds: 20.0,
        };
        let level = 16;
        let before = respawn::urf_respawn_delay_seconds(level, 240.0, tuning);
        let after = respawn::urf_respawn_delay_seconds(level, 1200.0, tuning);
        assert!(after > before);
    }
}

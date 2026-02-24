use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SimulationDefaults {
    pub server_tick_rate_hz: f64,
    pub dt_fallback_seconds: f64,
    pub champion_level: usize,
    pub time_limit_seconds: f64,
    #[serde(default)]
    pub stack_overrides: HashMap<String, f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SearchDefaults {
    pub beam_width: usize,
    pub max_items: usize,
    pub random_samples: usize,
    pub hill_climb_restarts: usize,
    pub hill_climb_steps: usize,
    pub hill_climb_neighbors: usize,
    pub genetic_population: usize,
    pub genetic_generations: usize,
    pub genetic_mutation_rate: f64,
    pub genetic_crossover_rate: f64,
    pub ranked_limit: usize,
    pub simulated_annealing_restarts: usize,
    pub simulated_annealing_iterations: usize,
    pub simulated_annealing_initial_temp: f64,
    pub simulated_annealing_cooling_rate: f64,
    pub mcts_iterations: usize,
    pub mcts_rollouts_per_expansion: usize,
    pub mcts_exploration: f64,
    pub ensemble_seeds: usize,
    pub ensemble_seed_stride: u64,
    pub ensemble_seed_top_k: usize,
    pub objective_survival_weight: f64,
    pub objective_damage_weight: f64,
    pub objective_healing_weight: f64,
    pub objective_enemy_kills_weight: f64,
    pub objective_invulnerable_seconds_weight: f64,
    pub robust_min_seed_hit_rate: f64,
    pub bleed_enabled: bool,
    pub bleed_budget: usize,
    pub bleed_mutation_rate: f64,
    pub multi_scenario_worst_weight: f64,
    pub strict_ranking_enable_heuristic_ordering: bool,
    pub strict_ranking_rune_signal_weight: f64,
    pub strict_ranking_shard_signal_weight: f64,
    pub strict_ranking_exploration_promotions: usize,
    pub unmodeled_rune_hard_gate: bool,
    pub unmodeled_rune_penalty_per_rune: f64,
    pub unmodeled_item_effect_hard_gate: bool,
    pub unmodeled_item_effect_penalty_per_item: f64,
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct SearchQualityProfilePreset {
    pub beam_width: usize,
    pub random_samples: usize,
    pub hill_climb_restarts: usize,
    pub hill_climb_steps: usize,
    pub hill_climb_neighbors: usize,
    pub genetic_population: usize,
    pub genetic_generations: usize,
    pub simulated_annealing_restarts: usize,
    pub simulated_annealing_iterations: usize,
    pub mcts_iterations: usize,
    pub mcts_rollouts_per_expansion: usize,
    pub ensemble_seeds: usize,
    pub ensemble_seed_top_k: usize,
    pub ranked_limit: usize,
    pub bleed_budget: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SearchQualityProfileDefaults {
    pub fast: SearchQualityProfilePreset,
    pub balanced: SearchQualityProfilePreset,
    pub maximum_quality_minimums: SearchQualityProfilePreset,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EngineDefaults {
    pub default_champion_hitbox_radius: f64,
    pub champion_hitbox_radius_overrides: HashMap<String, f64>,
    pub controlled_champion_controller_vision_radius: f64,
    pub controlled_champion_request_fixed_tick_delay: u64,
    pub melee_spawn_attack_range_threshold: f64,
    pub melee_spawn_radius: f64,
    pub ranged_spawn_radius_multiplier: f64,
    pub ranged_spawn_radius_min: f64,
    pub ranged_spawn_radius_max: f64,
    pub minimum_attack_speed: f64,
    pub world_lifecycle: WorldLifecycleDefaults,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct WorldLifecycleDefaults {
    pub minion_wave_start_seconds: f64,
    pub minion_wave_interval_seconds: f64,
    pub minion_units_per_team_per_wave: usize,
    pub minion_lifetime_seconds: f64,
    pub dragon_initial_spawn_seconds: f64,
    pub dragon_respawn_seconds: f64,
    pub baron_initial_spawn_seconds: f64,
    pub baron_respawn_seconds: f64,
}

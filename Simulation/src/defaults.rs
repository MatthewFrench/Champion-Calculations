use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SimulationDefaults {
    pub server_tick_rate_hz: f64,
    pub dt_fallback_seconds: f64,
    pub champion_level: usize,
    pub heartsteel_assumed_stacks_at_8m: f64,
    pub enemy_uptime_model_enabled: bool,
    pub urf_respawn_flat_reduction_seconds: f64,
    pub urf_respawn_extrapolation_per_level: f64,
    pub urf_respawn_time_scaling_enabled: bool,
    pub urf_respawn_time_scaling_start_seconds: f64,
    pub urf_respawn_time_scaling_per_minute_seconds: f64,
    pub urf_respawn_time_scaling_cap_seconds: f64,
    pub vlad_q_base_damage: f64,
    pub vlad_q_ap_ratio: f64,
    pub vlad_q_heal_ratio_of_damage: f64,
    pub vlad_q_base_cooldown_seconds: f64,
    pub vlad_e_base_damage: f64,
    pub vlad_e_ap_ratio: f64,
    pub vlad_e_base_cooldown_seconds: f64,
    pub vlad_r_base_damage: f64,
    pub vlad_r_ap_ratio: f64,
    pub vlad_r_base_cooldown_seconds: f64,
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
    pub robust_min_seed_hit_rate: f64,
    pub bleed_enabled: bool,
    pub bleed_budget: usize,
    pub bleed_mutation_rate: f64,
    pub multi_scenario_worst_weight: f64,
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
    pub melee_spawn_attack_range_threshold: f64,
    pub melee_spawn_radius: f64,
    pub ranged_spawn_radius_multiplier: f64,
    pub ranged_spawn_radius_min: f64,
    pub ranged_spawn_radius_max: f64,
    pub minimum_attack_speed: f64,
    pub emergency_shield_item_cooldown_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AbilitySystemDefaults {
    pub champion_default_slot_bindings: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VladimirCastProfileDefaults {
    pub q_ability_id: String,
    pub e_ability_id: String,
    pub r_ability_id: String,
    pub pool_ability_id: String,
    pub q_range: f64,
    pub q_windup_seconds: f64,
    pub q_projectile_speed: f64,
    pub q_effect_hitbox_radius: f64,
    pub e_range: f64,
    pub e_windup_seconds: f64,
    pub e_projectile_speed: f64,
    pub e_effect_hitbox_radius: f64,
    pub r_range: f64,
    pub r_windup_seconds: f64,
    pub r_projectile_speed: f64,
    pub r_effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionBehaviorDefaults {
    pub default_melee: ChampionBehaviorDefaultsEntry,
    pub default_ranged: ChampionBehaviorDefaultsEntry,
    pub overrides: HashMap<String, ChampionBehaviorDefaultsEntry>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBehaviorDefaultsEntry {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub attack_effect_hitbox_radius: f64,
    pub ability_windup_seconds: f64,
    pub ability_projectile_speed: f64,
    pub ability_effect_hitbox_radius: f64,
    pub burst_windup_seconds: f64,
    pub burst_projectile_speed: f64,
    pub burst_effect_hitbox_radius: f64,
    pub desired_combat_range: f64,
    pub movement_speed_scale: f64,
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct EventScheduleDefaults {
    pub start_offset_seconds: f64,
    pub interval_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct WarwickScriptDefaults {
    pub infinite_duress_schedule: EventScheduleDefaults,
    pub infinite_duress_cast_range: f64,
    pub infinite_duress_hitbox_radius: f64,
    pub infinite_duress_physical_attack_damage_ratio: f64,
    pub infinite_duress_magic_base_damage: f64,
    pub infinite_duress_magic_attack_damage_ratio: f64,
    pub infinite_duress_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct VayneScriptDefaults {
    pub tumble_empower_schedule: EventScheduleDefaults,
    pub tumble_bonus_physical_attack_damage_ratio: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct MorganaScriptDefaults {
    pub dark_binding_schedule: EventScheduleDefaults,
    pub dark_binding_cast_range: f64,
    pub dark_binding_magic_base_damage: f64,
    pub dark_binding_magic_ability_power_ratio: f64,
    pub dark_binding_hitbox_radius: f64,
    pub dark_binding_stun_duration_seconds: f64,
    pub soul_shackles_schedule: EventScheduleDefaults,
    pub soul_shackles_cast_range: f64,
    pub soul_shackles_hitbox_radius: f64,
    pub soul_shackles_initial_magic_damage: f64,
    pub soul_shackles_detonate_delay_seconds: f64,
    pub soul_shackles_detonate_priority: i32,
    pub soul_shackles_detonate_cast_range: f64,
    pub soul_shackles_detonate_magic_damage: f64,
    pub soul_shackles_detonate_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SonaScriptDefaults {
    pub crescendo_schedule: EventScheduleDefaults,
    pub crescendo_cast_range: f64,
    pub crescendo_magic_base_damage: f64,
    pub crescendo_magic_ability_power_ratio: f64,
    pub crescendo_hitbox_radius: f64,
    pub crescendo_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct DoctorMundoScriptDefaults {
    pub infected_cleaver_schedule: EventScheduleDefaults,
    pub infected_cleaver_cast_range: f64,
    pub infected_cleaver_current_health_ratio: f64,
    pub infected_cleaver_min_magic_damage: f64,
    pub infected_cleaver_max_magic_damage: f64,
    pub infected_cleaver_flat_magic_damage: f64,
    pub infected_cleaver_hitbox_radius: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct YasuoScriptDefaults {
    pub wind_wall_schedule: EventScheduleDefaults,
    pub wind_wall_forward_offset: f64,
    pub wind_wall_half_length: f64,
    pub wind_wall_block_half_width: f64,
    pub wind_wall_duration_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionScriptDefaults {
    pub warwick: WarwickScriptDefaults,
    pub vayne: VayneScriptDefaults,
    pub morgana: MorganaScriptDefaults,
    pub sona: SonaScriptDefaults,
    pub doctor_mundo: DoctorMundoScriptDefaults,
    pub yasuo: YasuoScriptDefaults,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LoadoutGenerationDefaults {
    pub mastery_primary_points: usize,
    pub mastery_secondary_points: usize,
    pub mastery_keystone_requirement: usize,
    pub mastery_tier_points_available_fallback: usize,
    pub random_tree_attempts: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SimulatorDefaults {
    pub simulation_defaults: SimulationDefaults,
    pub search_defaults: SearchDefaults,
    pub search_quality_profile_defaults: SearchQualityProfileDefaults,
    pub engine_defaults: EngineDefaults,
    pub ability_system_defaults: AbilitySystemDefaults,
    pub vladimir_cast_profile_defaults: VladimirCastProfileDefaults,
    pub champion_behavior_defaults: ChampionBehaviorDefaults,
    pub champion_script_defaults: ChampionScriptDefaults,
    pub loadout_generation_defaults: LoadoutGenerationDefaults,
}

static SIMULATOR_DEFAULTS: OnceLock<SimulatorDefaults> = OnceLock::new();

fn normalize_key(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn load_defaults_from_disk() -> Result<SimulatorDefaults> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("simulator_defaults.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading simulator defaults: {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing simulator defaults: {}", path.display()))
}

pub(crate) fn simulator_defaults() -> &'static SimulatorDefaults {
    SIMULATOR_DEFAULTS
        .get_or_init(|| load_defaults_from_disk().unwrap_or_else(|err| panic!("{}", err)))
}

pub(crate) fn champion_slot_bindings(champion_name: &str) -> HashMap<String, String> {
    let key = normalize_key(champion_name);
    simulator_defaults()
        .ability_system_defaults
        .champion_default_slot_bindings
        .get(&key)
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn champion_hitbox_radius(champion_name: &str) -> f64 {
    let defaults = simulator_defaults();
    let key = normalize_key(champion_name);
    defaults
        .engine_defaults
        .champion_hitbox_radius_overrides
        .get(&key)
        .copied()
        .unwrap_or(defaults.engine_defaults.default_champion_hitbox_radius)
}

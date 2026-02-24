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

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct LevelScalingRange {
    pub min: f64,
    pub max: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct LethalTempoRuneDefaults {
    pub max_stacks: usize,
    pub attack_speed_per_stack: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct GraspOfTheUndyingRuneDefaults {
    pub cooldown_seconds: f64,
    pub base_magic_damage: f64,
    pub target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct SecondWindRuneDefaults {
    pub base_regen_max_health_ratio_per_second: f64,
    pub low_health_bonus_regen_max_health_ratio_per_second: f64,
    pub low_health_threshold_ratio: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct PressTheAttackRuneDefaults {
    pub vulnerability_true_damage_ratio: f64,
    pub burst_magic_damage_by_level: LevelScalingRange,
    pub stack_window_seconds: f64,
    pub vulnerability_duration_seconds: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct FleetFootworkRuneDefaults {
    pub heal_by_level: LevelScalingRange,
    pub attack_damage_ratio: f64,
    pub cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ConquerorRuneDefaults {
    pub max_stacks: usize,
    pub stack_duration_seconds: f64,
    pub melee_basic_attack_stacks: usize,
    pub ranged_basic_attack_stacks: usize,
    pub ability_hit_stacks: usize,
    pub adaptive_ability_power_per_stack_by_level: LevelScalingRange,
    pub melee_heal_ratio: f64,
    pub ranged_heal_ratio: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct AftershockRuneDefaults {
    pub cooldown_seconds: f64,
    pub active_duration_seconds: f64,
    pub shockwave_magic_damage_by_level: LevelScalingRange,
    pub shockwave_bonus_health_ratio: f64,
    pub resist_base: f64,
    pub resist_bonus_ratio: f64,
    pub resist_cap_by_level: LevelScalingRange,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ElectrocuteRuneDefaults {
    pub hits_to_proc: usize,
    pub hit_window_seconds: f64,
    pub cooldown_by_level: LevelScalingRange,
    pub proc_magic_damage_by_level: LevelScalingRange,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct FirstStrikeRuneDefaults {
    pub bonus_true_damage_ratio: f64,
    pub window_duration_seconds: f64,
    pub cooldown_by_level: LevelScalingRange,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct PhaseRushRuneDefaults {
    pub hits_to_proc: usize,
    pub hit_window_seconds: f64,
    pub cooldown_seconds: f64,
    pub active_duration_seconds: f64,
    pub movement_speed_bonus_ratio_by_level: LevelScalingRange,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ArcaneCometRuneDefaults {
    pub proc_magic_damage_by_level: LevelScalingRange,
    pub ability_power_ratio: f64,
    pub bonus_attack_damage_ratio: f64,
    pub cooldown_by_level: LevelScalingRange,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct SummonAeryRuneDefaults {
    pub proc_magic_damage_by_level: LevelScalingRange,
    pub ability_power_ratio: f64,
    pub bonus_attack_damage_ratio: f64,
    pub cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct HailOfBladesRuneDefaults {
    pub bonus_attack_speed_ratio_melee: f64,
    pub bonus_attack_speed_ratio_ranged: f64,
    pub empowered_attack_count: usize,
    pub active_duration_seconds: f64,
    pub cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct DarkHarvestRuneDefaults {
    pub trigger_health_ratio: f64,
    pub cooldown_seconds: f64,
    pub base_magic_damage: f64,
    pub soul_magic_damage: f64,
    pub ability_power_ratio: f64,
    pub bonus_attack_damage_ratio: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct TriumphRuneDefaults {
    pub heal_max_health_ratio: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct GatheringStormRuneDefaults {
    pub interval_seconds: f64,
    pub ability_power_by_interval: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RuneRuntimeDefaults {
    pub lethal_tempo: LethalTempoRuneDefaults,
    pub grasp_of_the_undying: GraspOfTheUndyingRuneDefaults,
    pub second_wind: SecondWindRuneDefaults,
    pub press_the_attack: PressTheAttackRuneDefaults,
    pub fleet_footwork: FleetFootworkRuneDefaults,
    pub conqueror: ConquerorRuneDefaults,
    pub aftershock: AftershockRuneDefaults,
    pub electrocute: ElectrocuteRuneDefaults,
    pub first_strike: FirstStrikeRuneDefaults,
    pub phase_rush: PhaseRushRuneDefaults,
    pub arcane_comet: ArcaneCometRuneDefaults,
    pub summon_aery: SummonAeryRuneDefaults,
    pub hail_of_blades: HailOfBladesRuneDefaults,
    pub dark_harvest: DarkHarvestRuneDefaults,
    pub triumph: TriumphRuneDefaults,
    pub gathering_storm: GatheringStormRuneDefaults,
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
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBehaviorDefaultsEntry {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub attack_effect_hitbox_radius: f64,
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorOverrideEntry {
    #[serde(default)]
    pub attack_range: Option<f64>,
    #[serde(default)]
    pub attack_windup_seconds: Option<f64>,
    #[serde(default)]
    pub attack_projectile_speed: Option<f64>,
    #[serde(default)]
    pub attack_effect_hitbox_radius: Option<f64>,
    #[serde(default)]
    pub on_hit_magic_flat: Option<f64>,
    #[serde(default)]
    pub on_hit_magic_ad_ratio: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_every: Option<usize>,
    #[serde(default)]
    pub periodic_true_hit_base: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_ad_ratio: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_target_max_health_ratio: Option<f64>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirOffensiveAbilityDefaults {
    pub q_base_damage: f64,
    pub q_ap_ratio: f64,
    pub q_heal_ratio_of_damage: f64,
    pub q_base_cooldown_seconds: f64,
    pub e_base_damage: f64,
    pub e_ap_ratio: f64,
    pub e_base_cooldown_seconds: f64,
    pub r_base_damage: f64,
    pub r_ap_ratio: f64,
    pub r_base_cooldown_seconds: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct VladimirSanguinePoolDefaults {
    pub base_cooldown_seconds_by_rank: Vec<f64>,
    pub default_rank: usize,
    pub effect_range: f64,
    pub untargetable_seconds: f64,
    pub damage_tick_interval_seconds: f64,
    pub cost_percent_current_health: f64,
    pub heal_ratio_of_damage: f64,
    pub damage_per_tick_by_rank: Vec<f64>,
    pub damage_per_tick_bonus_health_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirDefensiveAbilityTwoPolicyDefaults {
    pub prioritize_offensive_ultimate_before_defensive_ability_two: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ZhonyaTimeStopDefaults {
    pub duration_seconds: f64,
    pub cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GuardianAngelRebirthDefaults {
    pub cooldown_seconds: f64,
    pub revive_duration_seconds: f64,
    pub revive_base_health_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProtoplasmLifelineDefaults {
    pub trigger_health_percent: f64,
    pub bonus_health_min: f64,
    pub bonus_health_max: f64,
    pub heal_total_min: f64,
    pub heal_total_max: f64,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ControlledChampionDefensiveItemPolicyDefaults {
    pub stasis_trigger_health_percent: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionAiDefaults {
    pub script_poll_interval_seconds: f64,
    pub movement_speed_scale: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) struct ChampionAiProfileOverrideEntry {
    #[serde(default)]
    pub desired_combat_range: Option<f64>,
    #[serde(default)]
    pub movement_speed_scale: Option<f64>,
    #[serde(default)]
    pub script_poll_interval_seconds: Option<f64>,
    #[serde(default)]
    pub script_priority_overrides: HashMap<String, i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionAiProfilesFile {
    pub(crate) defaults: ChampionAiDefaults,
    pub(crate) controlled_champion_defaults: ControlledChampionDefensiveItemPolicyDefaults,
    #[serde(default)]
    pub(crate) champions: HashMap<String, ChampionAiProfileOverrideEntry>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct AbilityExecutionDefaultsEntry {
    #[serde(default)]
    pub cast_windup_seconds: f64,
    #[serde(default)]
    pub projectile_speed: f64,
    #[serde(default)]
    pub effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AbilityExecutionDefaultsByRole {
    pub(crate) melee: AbilityExecutionDefaultsEntry,
    pub(crate) ranged: AbilityExecutionDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct AbilityExecutionOverrideEntry {
    #[serde(default)]
    pub cast_windup_seconds: Option<f64>,
    #[serde(default)]
    pub projectile_speed: Option<f64>,
    #[serde(default)]
    pub effect_hitbox_radius: Option<f64>,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub(crate) struct ChampionAbilityExecutionData {
    pub(crate) is_melee: bool,
    pub(crate) abilities: HashMap<String, AbilityExecutionOverrideEntry>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionAiProfile {
    pub desired_combat_range: f64,
    pub movement_speed_scale: f64,
    pub script_poll_interval_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AbilityExecutionProfile {
    pub cast_windup_seconds: f64,
    pub projectile_speed: f64,
    pub effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WarwickInfiniteDuressAbilityDefaults {
    pub infinite_duress_cast_range: f64,
    pub infinite_duress_cooldown_seconds: f64,
    pub infinite_duress_execution: AbilityExecutionProfile,
    pub infinite_duress_physical_attack_damage_ratio: f64,
    pub infinite_duress_magic_base_damage: f64,
    pub infinite_duress_magic_attack_damage_ratio: f64,
    pub infinite_duress_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WarwickEternalHungerPassiveDefaults {
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VayneTumbleAbilityDefaults {
    pub tumble_cooldown_seconds: f64,
    pub tumble_bonus_physical_attack_damage_ratio: f64,
    pub tumble_bonus_physical_ability_power_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VayneSilverBoltsAbilityDefaults {
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MorganaBindingAndSoulShacklesAbilityDefaults {
    pub dark_binding_cast_range: f64,
    pub dark_binding_cooldown_seconds: f64,
    pub dark_binding_execution: AbilityExecutionProfile,
    pub dark_binding_magic_base_damage: f64,
    pub dark_binding_magic_ability_power_ratio: f64,
    pub dark_binding_stun_duration_seconds: f64,
    pub soul_shackles_cast_range: f64,
    pub soul_shackles_cooldown_seconds: f64,
    pub soul_shackles_execution: AbilityExecutionProfile,
    pub soul_shackles_detonate_delay_seconds: f64,
    pub soul_shackles_initial_magic_damage: f64,
    pub soul_shackles_initial_magic_ability_power_ratio: f64,
    pub soul_shackles_detonate_magic_damage: f64,
    pub soul_shackles_detonate_magic_ability_power_ratio: f64,
    pub soul_shackles_detonate_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SonaCrescendoAbilityDefaults {
    pub crescendo_cast_range: f64,
    pub crescendo_cooldown_seconds: f64,
    pub crescendo_execution: AbilityExecutionProfile,
    pub crescendo_magic_base_damage: f64,
    pub crescendo_magic_ability_power_ratio: f64,
    pub crescendo_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DoctorMundoInfectedBonesawAbilityDefaults {
    pub cast_range: f64,
    pub cooldown_seconds: f64,
    pub infected_bonesaw_execution: AbilityExecutionProfile,
    pub current_health_ratio: f64,
    pub minimum_magic_damage: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct ChampionSimulationData {
    #[serde(default)]
    pub behavior: Option<ChampionBehaviorOverrideEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionFileEnvelope {
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) behavior: Option<ChampionBehaviorOverrideEntry>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBaseStatsDefaultsEntry {
    #[serde(default)]
    pub(crate) attack_range: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBaseStatsDefaultsByRole {
    pub(crate) melee: ChampionBaseStatsDefaultsEntry,
    pub(crate) ranged: ChampionBaseStatsDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBasicAttackRawTimingDefaultsEntry {
    #[serde(default)]
    pub(crate) gameplay_radius: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBasicAttackDefaultsEntry {
    #[serde(default)]
    pub(crate) base_windup_seconds: f64,
    #[serde(default)]
    pub(crate) missile_speed: f64,
    #[serde(default)]
    pub(crate) raw_timing_stats: ChampionBasicAttackRawTimingDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBasicAttackDefaultsByRole {
    pub(crate) melee: ChampionBasicAttackDefaultsEntry,
    pub(crate) ranged: ChampionBasicAttackDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorOnHitDefaultsEntry {
    #[serde(default)]
    pub(crate) magic_flat: f64,
    #[serde(default)]
    pub(crate) magic_ad_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorPeriodicTrueHitDefaultsEntry {
    #[serde(default)]
    pub(crate) every: usize,
    #[serde(default)]
    pub(crate) base: f64,
    #[serde(default)]
    pub(crate) ad_ratio: f64,
    #[serde(default)]
    pub(crate) target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorModifiersEntry {
    #[serde(default)]
    pub(crate) on_hit: ChampionBehaviorOnHitDefaultsEntry,
    #[serde(default)]
    pub(crate) periodic_true_hit: ChampionBehaviorPeriodicTrueHitDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorModifiersByRole {
    #[serde(default)]
    pub(crate) melee: ChampionBehaviorModifiersEntry,
    #[serde(default)]
    pub(crate) ranged: ChampionBehaviorModifiersEntry,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionAbilitiesDefaults {
    pub(crate) execution_defaults: AbilityExecutionDefaultsByRole,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionDefaultsFile {
    pub(crate) base_stats: ChampionBaseStatsDefaultsByRole,
    pub(crate) basic_attack: ChampionBasicAttackDefaultsByRole,
    pub(crate) abilities: ChampionAbilitiesDefaults,
    #[serde(default)]
    pub(crate) behavior: ChampionBehaviorModifiersByRole,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionBehaviorDefaults {
    pub(crate) melee: ChampionBehaviorDefaultsEntry,
    pub(crate) ranged: ChampionBehaviorDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct UrfRespawnDefaults {
    pub flat_reduction_seconds: f64,
    pub extrapolation_per_level: f64,
    pub time_scaling_enabled: bool,
    pub time_scaling_start_seconds: f64,
    pub time_scaling_per_minute_seconds: f64,
    pub time_scaling_cap_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct UrfFileEnvelope {
    #[serde(default)]
    pub(crate) respawn: Option<UrfRespawnDefaults>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SimulatorDefaults {
    pub simulation_defaults: SimulationDefaults,
    pub search_defaults: SearchDefaults,
    pub search_quality_profile_defaults: SearchQualityProfileDefaults,
    pub engine_defaults: EngineDefaults,
    pub rune_runtime_defaults: RuneRuntimeDefaults,
}

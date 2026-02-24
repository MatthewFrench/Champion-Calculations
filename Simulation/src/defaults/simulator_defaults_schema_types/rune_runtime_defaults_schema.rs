use serde::Deserialize;

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

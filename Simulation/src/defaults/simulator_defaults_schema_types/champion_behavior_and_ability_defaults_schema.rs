use serde::Deserialize;

use super::champion_ai_and_execution_schema::AbilityExecutionProfile;

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

#[derive(Debug, Clone)]
pub(crate) struct ChampionBehaviorDefaults {
    pub(crate) melee: ChampionBehaviorDefaultsEntry,
    pub(crate) ranged: ChampionBehaviorDefaultsEntry,
}

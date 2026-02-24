use super::super::champion_ai_and_execution_schema::AbilityExecutionProfile;

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

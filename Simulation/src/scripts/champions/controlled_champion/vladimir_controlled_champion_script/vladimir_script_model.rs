use crate::scripts::champions::vladimir;

use super::super::{ControlledChampionAbilityTuning, ControlledChampionCastProfile};

#[derive(Debug, Clone)]
pub(super) struct VladimirControlledChampionScript {
    pub(super) cast_profile: ControlledChampionCastProfile,
    pub(super) offensive_tuning: ControlledChampionAbilityTuning,
    pub(super) defensive_ability_two_rank: usize,
    pub(super) defensive_ability_two_duration_seconds: f64,
    pub(super) defensive_ability_two_effect_range: f64,
    pub(super) defensive_ability_two_damage_tick_interval_seconds: f64,
    pub(super) defensive_ability_two_cost_percent_current_health: f64,
    pub(super) defensive_ability_two_heal_ratio_of_damage: f64,
    pub(super) defensive_ability_two_damage_per_tick_by_rank: Vec<f64>,
    pub(super) defensive_ability_two_base_cooldown_seconds_by_rank: Vec<f64>,
    pub(super) defensive_ability_two_damage_per_tick_bonus_health_ratio: f64,
    pub(super) prioritize_offensive_ultimate_before_defensive_ability_two: bool,
}

pub(super) fn to_vladimir_tuning(
    tuning: ControlledChampionAbilityTuning,
) -> vladimir::VladimirAbilityTuning {
    vladimir::VladimirAbilityTuning {
        q_base_damage: tuning.offensive_primary_base_damage,
        q_ap_ratio: tuning.offensive_primary_ap_ratio,
        q_base_cooldown_seconds: tuning.offensive_primary_base_cooldown_seconds,
        e_base_damage: tuning.offensive_secondary_base_damage,
        e_ap_ratio: tuning.offensive_secondary_ap_ratio,
        e_base_cooldown_seconds: tuning.offensive_secondary_base_cooldown_seconds,
        r_base_damage: tuning.offensive_ultimate_base_damage,
        r_ap_ratio: tuning.offensive_ultimate_ap_ratio,
        r_base_cooldown_seconds: tuning.offensive_ultimate_base_cooldown_seconds,
    }
}

pub(super) fn from_vladimir_cast_profile(
    profile: vladimir::VladimirCastProfile,
) -> ControlledChampionCastProfile {
    ControlledChampionCastProfile {
        offensive_primary_ability_id: profile.q_ability_id,
        defensive_ability_two_id: profile.pool_ability_id,
        offensive_secondary_ability_id: profile.e_ability_id,
        offensive_ultimate_ability_id: profile.r_ability_id,
        offensive_primary_range: profile.q_range,
        offensive_primary_windup_seconds: profile.q_windup_seconds,
        offensive_primary_projectile_speed: profile.q_projectile_speed,
        offensive_primary_effect_hitbox_radius: profile.q_effect_hitbox_radius,
        offensive_secondary_range: profile.e_range,
        offensive_secondary_windup_seconds: profile.e_windup_seconds,
        offensive_secondary_projectile_speed: profile.e_projectile_speed,
        offensive_secondary_effect_hitbox_radius: profile.e_effect_hitbox_radius,
        offensive_ultimate_range: profile.r_range,
        offensive_ultimate_windup_seconds: profile.r_windup_seconds,
        offensive_ultimate_projectile_speed: profile.r_projectile_speed,
        offensive_ultimate_effect_hitbox_radius: profile.r_effect_hitbox_radius,
    }
}

pub(super) fn to_vladimir_cast_profile(
    profile: ControlledChampionCastProfile,
) -> vladimir::VladimirCastProfile {
    vladimir::VladimirCastProfile {
        q_ability_id: profile.offensive_primary_ability_id,
        pool_ability_id: profile.defensive_ability_two_id,
        e_ability_id: profile.offensive_secondary_ability_id,
        r_ability_id: profile.offensive_ultimate_ability_id,
        q_range: profile.offensive_primary_range,
        q_windup_seconds: profile.offensive_primary_windup_seconds,
        q_projectile_speed: profile.offensive_primary_projectile_speed,
        q_effect_hitbox_radius: profile.offensive_primary_effect_hitbox_radius,
        e_range: profile.offensive_secondary_range,
        e_windup_seconds: profile.offensive_secondary_windup_seconds,
        e_projectile_speed: profile.offensive_secondary_projectile_speed,
        e_effect_hitbox_radius: profile.offensive_secondary_effect_hitbox_radius,
        r_range: profile.offensive_ultimate_range,
        r_windup_seconds: profile.offensive_ultimate_windup_seconds,
        r_projectile_speed: profile.offensive_ultimate_projectile_speed,
        r_effect_hitbox_radius: profile.offensive_ultimate_effect_hitbox_radius,
    }
}

pub(super) fn resolve_ranked_value(values: &[f64], rank: usize, fallback: f64) -> f64 {
    values
        .get(rank.saturating_sub(1))
        .copied()
        .or_else(|| values.last().copied())
        .unwrap_or(fallback)
}

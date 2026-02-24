use std::sync::Arc;

use crate::defaults::{
    vladimir_cast_profile_defaults, vladimir_defensive_ability_two_policy_defaults,
    vladimir_offensive_ability_defaults, vladimir_sanguine_pool_defaults,
};
use crate::scripts::champions::vladimir;

use super::super::{ControlledChampionAbilityTuning, ControlledChampionScriptHandle};
use super::vladimir_script_model::{VladimirControlledChampionScript, from_vladimir_cast_profile};

pub(crate) fn build_vladimir_script() -> ControlledChampionScriptHandle {
    let cast_profile = vladimir_cast_profile_defaults("vladimir")
        .map(|defaults| {
            from_vladimir_cast_profile(vladimir::VladimirCastProfile {
                q_ability_id: defaults.q_ability_id,
                pool_ability_id: defaults.pool_ability_id,
                e_ability_id: defaults.e_ability_id,
                r_ability_id: defaults.r_ability_id,
                q_range: defaults.q_range,
                q_windup_seconds: defaults.q_windup_seconds,
                q_projectile_speed: defaults.q_projectile_speed,
                q_effect_hitbox_radius: defaults.q_effect_hitbox_radius,
                e_range: defaults.e_range,
                e_windup_seconds: defaults.e_windup_seconds,
                e_projectile_speed: defaults.e_projectile_speed,
                e_effect_hitbox_radius: defaults.e_effect_hitbox_radius,
                r_range: defaults.r_range,
                r_windup_seconds: defaults.r_windup_seconds,
                r_projectile_speed: defaults.r_projectile_speed,
                r_effect_hitbox_radius: defaults.r_effect_hitbox_radius,
            })
        })
        .unwrap_or_default();
    let offensive_tuning = vladimir_offensive_ability_defaults("vladimir")
        .map(|defaults| ControlledChampionAbilityTuning {
            offensive_primary_base_damage: defaults.q_base_damage,
            offensive_primary_ap_ratio: defaults.q_ap_ratio,
            offensive_primary_heal_ratio_of_damage: defaults.q_heal_ratio_of_damage,
            offensive_primary_base_cooldown_seconds: defaults.q_base_cooldown_seconds,
            offensive_secondary_base_damage: defaults.e_base_damage,
            offensive_secondary_ap_ratio: defaults.e_ap_ratio,
            offensive_secondary_base_cooldown_seconds: defaults.e_base_cooldown_seconds,
            offensive_ultimate_base_damage: defaults.r_base_damage,
            offensive_ultimate_ap_ratio: defaults.r_ap_ratio,
            offensive_ultimate_base_cooldown_seconds: defaults.r_base_cooldown_seconds,
        })
        .unwrap_or_default();
    let pool_defaults = vladimir_sanguine_pool_defaults("vladimir").unwrap_or_else(|| {
        panic!("missing Vladimir Sanguine Pool defaults in canonical champion data")
    });
    let defensive_ability_two_policy_defaults =
        vladimir_defensive_ability_two_policy_defaults("vladimir").unwrap_or_else(|| {
            panic!(
                "missing Vladimir defensive ability two policy defaults in champion simulation data"
            )
        });

    Arc::new(VladimirControlledChampionScript {
        cast_profile,
        offensive_tuning,
        defensive_ability_two_rank: pool_defaults.default_rank,
        defensive_ability_two_duration_seconds: pool_defaults.untargetable_seconds,
        defensive_ability_two_effect_range: pool_defaults.effect_range,
        defensive_ability_two_damage_tick_interval_seconds: pool_defaults
            .damage_tick_interval_seconds,
        defensive_ability_two_cost_percent_current_health: pool_defaults
            .cost_percent_current_health,
        defensive_ability_two_heal_ratio_of_damage: pool_defaults.heal_ratio_of_damage,
        defensive_ability_two_damage_per_tick_by_rank: pool_defaults.damage_per_tick_by_rank,
        defensive_ability_two_base_cooldown_seconds_by_rank: pool_defaults
            .base_cooldown_seconds_by_rank,
        defensive_ability_two_damage_per_tick_bonus_health_ratio: pool_defaults
            .damage_per_tick_bonus_health_ratio,
        prioritize_offensive_ultimate_before_defensive_ability_two:
            defensive_ability_two_policy_defaults
                .prioritize_offensive_ultimate_before_defensive_ability_two,
    })
}

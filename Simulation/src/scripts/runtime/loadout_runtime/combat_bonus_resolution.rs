mod ability_bonus_damage_resolution;
mod on_hit_bonus_damage_resolution;
mod projection_helpers;
mod rune_proc_state_mutations;

use super::{LoadoutRuntimeState, OnHitEffectProfile};

pub(super) fn decay_expired_conqueror_stacks(runtime: &mut LoadoutRuntimeState, now: f64) {
    rune_proc_state_mutations::decay_expired_conqueror_stacks(runtime, now);
}

#[allow(clippy::too_many_arguments)]
pub(super) fn calculate_on_hit_bonus_damage_impl(
    profile: OnHitEffectProfile,
    runtime: &mut LoadoutRuntimeState,
    attack_damage: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64, f64) {
    on_hit_bonus_damage_resolution::calculate_on_hit_bonus_damage_impl(
        profile,
        runtime,
        attack_damage,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        target_current_health,
        target_max_health,
        attacker_max_health,
        now,
        target_id,
        attacker_level,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn calculate_ability_bonus_damage_impl(
    runtime: &mut LoadoutRuntimeState,
    ability_raw_damage: f64,
    ability_ap_ratio: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64) {
    ability_bonus_damage_resolution::calculate_ability_bonus_damage_impl(
        runtime,
        ability_raw_damage,
        ability_ap_ratio,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        target_current_health,
        target_max_health,
        now,
        target_id,
        attacker_level,
    )
}

mod combat_bonus_resolution;
mod rune_proc_telemetry;
mod runtime_effect_mutations;
mod runtime_stat_projections;
mod runtime_state_initialization;
mod runtime_state_reporting;
mod runtime_state_schema;

pub(crate) type RuneProcTelemetryEntry = self::rune_proc_telemetry::RuneProcTelemetryEntry;
#[cfg(test)]
pub(crate) type RuneProcTelemetrySourceEntry =
    self::rune_proc_telemetry::RuneProcTelemetrySourceEntry;

use self::runtime_state_schema::{HitWindowTargetState, level_scaled_range_value};
pub(crate) use self::runtime_state_schema::{LoadoutRuntimeState, OnHitEffectProfile};

#[cfg(test)]
use self::rune_proc_telemetry::{MODELED_RUNE_TELEMETRY_KEYS, rune_telemetry_index};

use self::combat_bonus_resolution::{
    calculate_ability_bonus_damage_impl, calculate_on_hit_bonus_damage_impl,
};
use self::rune_proc_telemetry::build_rune_proc_telemetry_entries;
use self::runtime_effect_mutations::{
    on_enemy_kill_heal_impl, on_outgoing_damage_heal_impl, trigger_immobilize_rune_damage_impl,
};
use self::runtime_stat_projections::{
    loadout_attack_speed_multiplier_impl, loadout_incoming_damage_multipliers_impl,
    loadout_movement_speed_multiplier_impl, tick_loadout_regeneration_impl,
};
use self::runtime_state_initialization::{
    build_loadout_runtime_state_with_telemetry_impl, reset_transient_loadout_state_impl,
};
use self::runtime_state_reporting::{
    describe_runtime_cooldowns_impl, describe_runtime_stacks_impl,
};

#[cfg(test)]
pub(crate) fn build_loadout_runtime_state(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
) -> LoadoutRuntimeState {
    build_loadout_runtime_state_with_telemetry(
        item_names,
        rune_names,
        item_haste,
        owner_is_melee,
        true,
    )
}

pub(crate) fn build_loadout_runtime_state_with_telemetry(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
    rune_proc_telemetry_enabled: bool,
) -> LoadoutRuntimeState {
    build_loadout_runtime_state_with_telemetry_impl(
        item_names,
        rune_names,
        item_haste,
        owner_is_melee,
        rune_proc_telemetry_enabled,
    )
}

pub(crate) fn loadout_attack_speed_multiplier(runtime: &LoadoutRuntimeState, now: f64) -> f64 {
    loadout_attack_speed_multiplier_impl(runtime, now)
}

pub(crate) fn reset_transient_loadout_state(runtime: &mut LoadoutRuntimeState) {
    reset_transient_loadout_state_impl(runtime);
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn calculate_on_hit_bonus_damage(
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
    calculate_on_hit_bonus_damage_impl(
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
pub(crate) fn calculate_ability_bonus_damage(
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
    calculate_ability_bonus_damage_impl(
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

pub(crate) fn on_outgoing_damage_heal(
    runtime: &mut LoadoutRuntimeState,
    damage_dealt: f64,
    now: f64,
) -> f64 {
    on_outgoing_damage_heal_impl(runtime, damage_dealt, now)
}

pub(crate) fn on_enemy_kill_heal(runtime: &mut LoadoutRuntimeState, max_health: f64) -> f64 {
    on_enemy_kill_heal_impl(runtime, max_health)
}

pub(crate) fn trigger_immobilize_rune_damage(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    actor_bonus_health: f64,
) -> f64 {
    trigger_immobilize_rune_damage_impl(runtime, now, actor_level, actor_bonus_health)
}

pub(crate) fn loadout_incoming_damage_multipliers(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    current_armor: f64,
    current_magic_resist: f64,
    bonus_armor: f64,
    bonus_magic_resist: f64,
) -> (f64, f64) {
    loadout_incoming_damage_multipliers_impl(
        runtime,
        now,
        actor_level,
        current_armor,
        current_magic_resist,
        bonus_armor,
        bonus_magic_resist,
    )
}

pub(crate) fn loadout_movement_speed_multiplier(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
) -> f64 {
    loadout_movement_speed_multiplier_impl(runtime, now, actor_level)
}

pub(crate) fn rune_proc_telemetry(runtime: &LoadoutRuntimeState) -> Vec<RuneProcTelemetryEntry> {
    build_rune_proc_telemetry_entries(runtime)
}

pub(crate) fn tick_loadout_regeneration(
    runtime: &LoadoutRuntimeState,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    tick_loadout_regeneration_impl(runtime, current_health, max_health, dt)
}

pub(crate) fn describe_runtime_cooldowns(runtime: &LoadoutRuntimeState, now: f64) -> Vec<String> {
    describe_runtime_cooldowns_impl(runtime, now)
}

pub(crate) fn describe_runtime_stacks(runtime: &LoadoutRuntimeState) -> Vec<String> {
    describe_runtime_stacks_impl(runtime)
}

#[cfg(test)]
#[path = "tests/loadout_runtime_tests.rs"]
mod tests;

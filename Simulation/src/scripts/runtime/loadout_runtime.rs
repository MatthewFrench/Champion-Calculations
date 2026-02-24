use crate::defaults::LevelScalingRange;
use std::collections::HashMap;

mod combat_bonus_resolution;
mod rune_proc_telemetry;
mod runtime_effect_mutations;
mod runtime_stat_projections;
mod runtime_state_initialization;
mod runtime_state_reporting;

pub(crate) type RuneProcTelemetryEntry = self::rune_proc_telemetry::RuneProcTelemetryEntry;
#[cfg(test)]
pub(crate) type RuneProcTelemetrySourceEntry =
    self::rune_proc_telemetry::RuneProcTelemetrySourceEntry;

#[cfg(test)]
use self::rune_proc_telemetry::{MODELED_RUNE_TELEMETRY_KEYS, rune_telemetry_index};

use self::combat_bonus_resolution::{
    calculate_ability_bonus_damage_impl, calculate_on_hit_bonus_damage_impl,
};
use self::rune_proc_telemetry::{
    MODELED_RUNE_TELEMETRY_KEY_COUNT, RuneProcTelemetryTotals, build_rune_proc_telemetry_entries,
};
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct OnHitEffectProfile {
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct PressTheAttackTargetState {
    stacks: usize,
    stack_expires_at: f64,
    vulnerable_until: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct HitWindowTargetState {
    stacks: usize,
    expires_at: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutRuntimeState {
    has_lethal_tempo: bool,
    has_grasp: bool,
    has_kraken: bool,
    has_blade_of_the_ruined_king: bool,
    has_heartsteel: bool,
    has_liandry: bool,
    has_luden: bool,
    has_guinsoo: bool,
    has_second_wind: bool,
    has_press_the_attack: bool,
    has_fleet_footwork: bool,
    has_conqueror: bool,
    has_aftershock: bool,
    has_electrocute: bool,
    has_first_strike: bool,
    has_phase_rush: bool,
    has_arcane_comet: bool,
    has_summon_aery: bool,
    has_hail_of_blades: bool,
    has_dark_harvest: bool,
    has_triumph: bool,
    has_gathering_storm: bool,
    owner_is_melee: bool,
    rune_proc_telemetry_enabled: bool,

    pub attacks_landed: usize,
    pub lethal_tempo_stacks: usize,
    pub hail_of_blades_remaining_attacks: usize,
    pub guinsoo_stacks: usize,
    pub conqueror_stacks: usize,
    pub dark_harvest_souls: usize,
    pub grasp_cooldown_seconds: f64,
    pub heartsteel_cooldown_seconds: f64,
    pub luden_cooldown_seconds: f64,
    pub grasp_ready_at: f64,
    pub heartsteel_ready_at: f64,
    pub luden_ready_at: f64,
    pub conqueror_expires_at: f64,
    pub fleet_ready_at: f64,
    pub aftershock_ready_at: f64,
    pub aftershock_active_until: f64,
    pub electrocute_ready_at: f64,
    pub first_strike_ready_at: f64,
    pub first_strike_window_until: f64,
    pub phase_rush_ready_at: f64,
    pub phase_rush_active_until: f64,
    pub arcane_comet_ready_at: f64,
    pub summon_aery_ready_at: f64,
    pub hail_of_blades_ready_at: f64,
    pub hail_of_blades_expires_at: f64,
    pub dark_harvest_ready_at: f64,
    pub pending_fleet_heal: f64,
    press_the_attack_targets: HashMap<usize, PressTheAttackTargetState>,
    electrocute_targets: HashMap<usize, HitWindowTargetState>,
    phase_rush_targets: HashMap<usize, HitWindowTargetState>,
    rune_proc_telemetry_totals: [RuneProcTelemetryTotals; MODELED_RUNE_TELEMETRY_KEY_COUNT],
}

impl Default for LoadoutRuntimeState {
    fn default() -> Self {
        Self {
            has_lethal_tempo: false,
            has_grasp: false,
            has_kraken: false,
            has_blade_of_the_ruined_king: false,
            has_heartsteel: false,
            has_liandry: false,
            has_luden: false,
            has_guinsoo: false,
            has_second_wind: false,
            has_press_the_attack: false,
            has_fleet_footwork: false,
            has_conqueror: false,
            has_aftershock: false,
            has_electrocute: false,
            has_first_strike: false,
            has_phase_rush: false,
            has_arcane_comet: false,
            has_summon_aery: false,
            has_hail_of_blades: false,
            has_dark_harvest: false,
            has_triumph: false,
            has_gathering_storm: false,
            owner_is_melee: false,
            rune_proc_telemetry_enabled: true,
            attacks_landed: 0,
            lethal_tempo_stacks: 0,
            hail_of_blades_remaining_attacks: 0,
            guinsoo_stacks: 0,
            conqueror_stacks: 0,
            dark_harvest_souls: 0,
            grasp_cooldown_seconds: 4.0,
            heartsteel_cooldown_seconds: 0.0,
            luden_cooldown_seconds: 0.0,
            grasp_ready_at: 0.0,
            heartsteel_ready_at: 0.0,
            luden_ready_at: 0.0,
            conqueror_expires_at: 0.0,
            fleet_ready_at: 0.0,
            aftershock_ready_at: 0.0,
            aftershock_active_until: 0.0,
            electrocute_ready_at: 0.0,
            first_strike_ready_at: 0.0,
            first_strike_window_until: 0.0,
            phase_rush_ready_at: 0.0,
            phase_rush_active_until: 0.0,
            arcane_comet_ready_at: 0.0,
            summon_aery_ready_at: 0.0,
            hail_of_blades_ready_at: 0.0,
            hail_of_blades_expires_at: 0.0,
            dark_harvest_ready_at: 0.0,
            pending_fleet_heal: 0.0,
            press_the_attack_targets: HashMap::new(),
            electrocute_targets: HashMap::new(),
            phase_rush_targets: HashMap::new(),
            rune_proc_telemetry_totals: [RuneProcTelemetryTotals::default();
                MODELED_RUNE_TELEMETRY_KEY_COUNT],
        }
    }
}

fn level_scaled_value(level: usize, min: f64, max: f64) -> f64 {
    let clamped_level = level.clamp(1, 18);
    let t = (clamped_level as f64 - 1.0) / 17.0;
    min + (max - min) * t
}

fn level_scaled_range_value(level: usize, range: LevelScalingRange) -> f64 {
    level_scaled_value(level, range.min, range.max)
}

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

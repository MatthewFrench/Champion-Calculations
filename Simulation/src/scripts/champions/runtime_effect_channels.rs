use crate::scripts::runtime::loadout_runtime::{
    LoadoutRuntimeState, OnHitEffectProfile, build_loadout_runtime_state_with_telemetry,
    calculate_ability_bonus_damage, calculate_on_hit_bonus_damage, describe_runtime_cooldowns,
    describe_runtime_stacks, loadout_attack_speed_multiplier, loadout_incoming_damage_multipliers,
    loadout_movement_speed_multiplier, on_enemy_kill_heal, on_outgoing_damage_heal,
    reset_transient_loadout_state, rune_proc_telemetry, tick_loadout_regeneration,
    trigger_immobilize_rune_damage,
};

use super::ChampionBehaviorProfile;

pub(crate) type ChampionLoadoutRuntime = LoadoutRuntimeState;
pub(crate) type ChampionRuneProcTelemetryEntry =
    crate::scripts::runtime::loadout_runtime::RuneProcTelemetryEntry;

pub(crate) fn build_champion_loadout_runtime(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
    rune_proc_telemetry_enabled: bool,
) -> ChampionLoadoutRuntime {
    build_loadout_runtime_state_with_telemetry(
        item_names,
        rune_names,
        item_haste,
        owner_is_melee,
        rune_proc_telemetry_enabled,
    )
}

pub(crate) fn attack_speed_multiplier(runtime: &ChampionLoadoutRuntime, now: f64) -> f64 {
    loadout_attack_speed_multiplier(runtime, now)
}

pub(crate) fn clear_transient_combat_state(runtime: &mut ChampionLoadoutRuntime) {
    reset_transient_loadout_state(runtime)
}

fn on_hit_effect_profile(profile: ChampionBehaviorProfile) -> OnHitEffectProfile {
    OnHitEffectProfile {
        on_hit_magic_flat: profile.on_hit_magic_flat,
        on_hit_magic_ad_ratio: profile.on_hit_magic_ad_ratio,
        periodic_true_hit_every: profile.periodic_true_hit_every,
        periodic_true_hit_base: profile.periodic_true_hit_base,
        periodic_true_hit_ad_ratio: profile.periodic_true_hit_ad_ratio,
        periodic_true_hit_target_max_health_ratio: profile
            .periodic_true_hit_target_max_health_ratio,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn on_hit_bonus_damage(
    profile: ChampionBehaviorProfile,
    runtime: &mut ChampionLoadoutRuntime,
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
    calculate_on_hit_bonus_damage(
        on_hit_effect_profile(profile),
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
pub(crate) fn on_ability_bonus_damage(
    runtime: &mut ChampionLoadoutRuntime,
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
    calculate_ability_bonus_damage(
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

pub(crate) fn on_immobilize_rune_damage(
    runtime: &mut ChampionLoadoutRuntime,
    now: f64,
    actor_level: usize,
    actor_bonus_health: f64,
) -> f64 {
    trigger_immobilize_rune_damage(runtime, now, actor_level, actor_bonus_health)
}

pub(crate) fn outgoing_damage_heal(
    runtime: &mut ChampionLoadoutRuntime,
    damage_dealt: f64,
    now: f64,
) -> f64 {
    on_outgoing_damage_heal(runtime, damage_dealt, now)
}

pub(crate) fn enemy_kill_heal(runtime: &mut ChampionLoadoutRuntime, max_health: f64) -> f64 {
    on_enemy_kill_heal(runtime, max_health)
}

pub(crate) fn tick_regen_heal(
    runtime: &ChampionLoadoutRuntime,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    tick_loadout_regeneration(runtime, current_health, max_health, dt)
}

pub(crate) fn describe_runtime_effect_cooldowns(
    runtime: &ChampionLoadoutRuntime,
    now: f64,
) -> Vec<String> {
    describe_runtime_cooldowns(runtime, now)
}

pub(crate) fn describe_runtime_effect_stacks(runtime: &ChampionLoadoutRuntime) -> Vec<String> {
    describe_runtime_stacks(runtime)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn incoming_damage_multipliers(
    runtime: &ChampionLoadoutRuntime,
    now: f64,
    actor_level: usize,
    current_armor: f64,
    current_magic_resist: f64,
    bonus_armor: f64,
    bonus_magic_resist: f64,
) -> (f64, f64) {
    loadout_incoming_damage_multipliers(
        runtime,
        now,
        actor_level,
        current_armor,
        current_magic_resist,
        bonus_armor,
        bonus_magic_resist,
    )
}

pub(crate) fn movement_speed_multiplier(
    runtime: &ChampionLoadoutRuntime,
    now: f64,
    actor_level: usize,
) -> f64 {
    loadout_movement_speed_multiplier(runtime, now, actor_level)
}

pub(crate) fn describe_rune_proc_telemetry(
    runtime: &ChampionLoadoutRuntime,
) -> Vec<ChampionRuneProcTelemetryEntry> {
    rune_proc_telemetry(runtime)
}

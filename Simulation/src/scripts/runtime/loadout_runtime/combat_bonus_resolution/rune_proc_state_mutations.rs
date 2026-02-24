use super::super::rune_proc_telemetry::{
    RuneProcTriggerSource, record_rune_proc, record_rune_proc_attempt, record_rune_proc_eligibility,
};
use super::super::{HitWindowTargetState, LoadoutRuntimeState, level_scaled_range_value};
use crate::defaults::rune_runtime_defaults;
use std::collections::HashMap;

fn accumulate_window_stacks(
    stacks_by_target: &mut HashMap<usize, HitWindowTargetState>,
    target_idx: usize,
    now: f64,
    window_seconds: f64,
    max_stacks: usize,
) -> usize {
    let state = stacks_by_target.entry(target_idx).or_default();
    if now > state.expires_at {
        state.stacks = 0;
    }
    state.stacks = (state.stacks + 1).min(max_stacks);
    state.expires_at = now + window_seconds.max(0.0);
    state.stacks
}

pub(super) fn decay_expired_conqueror_stacks(runtime: &mut LoadoutRuntimeState, now: f64) {
    if runtime.has_conqueror && now > runtime.conqueror_expires_at {
        runtime.conqueror_stacks = 0;
    }
}

pub(super) fn add_conqueror_stacks(runtime: &mut LoadoutRuntimeState, stacks: usize, now: f64) {
    if !runtime.has_conqueror || stacks == 0 {
        return;
    }
    let defaults = &rune_runtime_defaults().conqueror;
    decay_expired_conqueror_stacks(runtime, now);
    runtime.conqueror_stacks = (runtime.conqueror_stacks + stacks).min(defaults.max_stacks.max(1));
    runtime.conqueror_expires_at = now + defaults.stack_duration_seconds.max(0.0);
}

pub(super) fn maybe_apply_first_strike(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    raw_damage: f64,
    attacker_level: usize,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().first_strike;
    if !runtime.has_first_strike {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "firststrike", source);
    if now >= runtime.first_strike_ready_at && now > runtime.first_strike_window_until {
        runtime.first_strike_window_until = now + defaults.window_duration_seconds.max(0.0);
        runtime.first_strike_ready_at =
            now + level_scaled_range_value(attacker_level, defaults.cooldown_by_level).max(0.0);
    }
    if now > runtime.first_strike_window_until {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "firststrike", source);
    let bonus_true = defaults.bonus_true_damage_ratio.max(0.0) * raw_damage.max(0.0);
    if bonus_true > 0.0 {
        record_rune_proc(runtime, "firststrike", source, bonus_true, 0.0);
    }
    bonus_true
}

pub(super) fn maybe_apply_electrocute(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().electrocute;
    if !runtime.has_electrocute {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "electrocute", source);
    if now < runtime.electrocute_ready_at {
        return 0.0;
    }
    let Some(target_idx) = target_id else {
        return 0.0;
    };
    let stacks = accumulate_window_stacks(
        &mut runtime.electrocute_targets,
        target_idx,
        now,
        defaults.hit_window_seconds,
        defaults.hits_to_proc.max(1),
    );
    if stacks < defaults.hits_to_proc.max(1) {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "electrocute", source);
    runtime.electrocute_ready_at =
        now + level_scaled_range_value(attacker_level, defaults.cooldown_by_level).max(0.0);
    if let Some(state) = runtime.electrocute_targets.get_mut(&target_idx) {
        state.stacks = 0;
        state.expires_at = now;
    }
    let damage = level_scaled_range_value(attacker_level, defaults.proc_magic_damage_by_level);
    record_rune_proc(runtime, "electrocute", source, damage, 0.0);
    damage
}

pub(super) fn maybe_apply_phase_rush(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    target_id: Option<usize>,
) {
    let defaults = &rune_runtime_defaults().phase_rush;
    if !runtime.has_phase_rush {
        return;
    }
    record_rune_proc_attempt(
        runtime,
        "phaserush",
        RuneProcTriggerSource::RuntimeActivation,
    );
    if now < runtime.phase_rush_ready_at {
        return;
    }
    let Some(target_idx) = target_id else {
        return;
    };
    let stacks = accumulate_window_stacks(
        &mut runtime.phase_rush_targets,
        target_idx,
        now,
        defaults.hit_window_seconds,
        defaults.hits_to_proc.max(1),
    );
    if stacks < defaults.hits_to_proc.max(1) {
        return;
    }
    record_rune_proc_eligibility(
        runtime,
        "phaserush",
        RuneProcTriggerSource::RuntimeActivation,
    );
    runtime.phase_rush_ready_at = now + defaults.cooldown_seconds.max(0.0);
    runtime.phase_rush_active_until = now + defaults.active_duration_seconds.max(0.0);
    if let Some(state) = runtime.phase_rush_targets.get_mut(&target_idx) {
        state.stacks = 0;
        state.expires_at = now;
    }
    record_rune_proc(
        runtime,
        "phaserush",
        RuneProcTriggerSource::RuntimeActivation,
        0.0,
        0.0,
    );
}

pub(super) fn update_hail_of_blades_state(runtime: &mut LoadoutRuntimeState, now: f64) {
    if !runtime.has_hail_of_blades {
        return;
    }
    let defaults = &rune_runtime_defaults().hail_of_blades;
    if runtime.hail_of_blades_remaining_attacks > 0 && now > runtime.hail_of_blades_expires_at {
        runtime.hail_of_blades_remaining_attacks = 0;
        runtime.hail_of_blades_expires_at = 0.0;
        runtime.hail_of_blades_ready_at = now + defaults.cooldown_seconds.max(0.0);
    }
    if runtime.hail_of_blades_remaining_attacks == 0 && now >= runtime.hail_of_blades_ready_at {
        runtime.hail_of_blades_remaining_attacks = defaults.empowered_attack_count.max(1);
        runtime.hail_of_blades_expires_at = now + defaults.active_duration_seconds.max(0.0);
    }
    if runtime.hail_of_blades_remaining_attacks > 0 {
        runtime.hail_of_blades_remaining_attacks -= 1;
        runtime.hail_of_blades_expires_at = now + defaults.active_duration_seconds.max(0.0);
        if runtime.hail_of_blades_remaining_attacks == 0 {
            runtime.hail_of_blades_ready_at = now + defaults.cooldown_seconds.max(0.0);
        }
    }
}

pub(super) fn maybe_apply_arcane_comet(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    attacker_level: usize,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().arcane_comet;
    if !runtime.has_arcane_comet {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "arcanecomet", RuneProcTriggerSource::Ability);
    if now < runtime.arcane_comet_ready_at {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "arcanecomet", RuneProcTriggerSource::Ability);
    runtime.arcane_comet_ready_at =
        now + level_scaled_range_value(attacker_level, defaults.cooldown_by_level).max(0.0);
    let damage = level_scaled_range_value(attacker_level, defaults.proc_magic_damage_by_level)
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    if damage > 0.0 {
        record_rune_proc(
            runtime,
            "arcanecomet",
            RuneProcTriggerSource::Ability,
            damage,
            0.0,
        );
    }
    damage
}

pub(super) fn maybe_apply_summon_aery(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    attacker_level: usize,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().summon_aery;
    if !runtime.has_summon_aery {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "summonaery", source);
    if now < runtime.summon_aery_ready_at {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "summonaery", source);
    runtime.summon_aery_ready_at = now + defaults.cooldown_seconds.max(0.0);
    let damage = level_scaled_range_value(attacker_level, defaults.proc_magic_damage_by_level)
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    if damage > 0.0 {
        record_rune_proc(runtime, "summonaery", source, damage, 0.0);
    }
    damage
}

pub(super) fn maybe_apply_dark_harvest(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().dark_harvest;
    if !runtime.has_dark_harvest {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "darkharvest", source);
    if now < runtime.dark_harvest_ready_at || target_max_health <= 0.0 {
        return 0.0;
    }
    let health_ratio = (target_current_health / target_max_health).clamp(0.0, 1.0);
    if health_ratio > defaults.trigger_health_ratio {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "darkharvest", source);
    runtime.dark_harvest_ready_at = now + defaults.cooldown_seconds.max(0.0);
    let damage = defaults.base_magic_damage.max(0.0)
        + defaults.soul_magic_damage.max(0.0) * runtime.dark_harvest_souls as f64
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    runtime.dark_harvest_souls = runtime.dark_harvest_souls.saturating_add(1);
    if damage > 0.0 {
        record_rune_proc(runtime, "darkharvest", source, damage, 0.0);
    }
    damage
}

use crate::defaults::rune_runtime_defaults;
use crate::scripts::runtime::stat_resolution::{
    RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
};

use super::super::rune_proc_telemetry::{
    RuneProcTriggerSource, record_rune_proc, record_rune_proc_attempt, record_rune_proc_eligibility,
};
use super::super::{LoadoutRuntimeState, level_scaled_range_value};
use super::projection_helpers::{
    gathering_storm_bonus_ability_power, press_the_attack_damage_multiplier,
};
use super::rune_proc_state_mutations::{
    add_conqueror_stacks, maybe_apply_arcane_comet, maybe_apply_dark_harvest,
    maybe_apply_electrocute, maybe_apply_first_strike, maybe_apply_phase_rush,
    maybe_apply_summon_aery,
};

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
    let defaults = rune_runtime_defaults();
    super::decay_expired_conqueror_stacks(runtime, now);
    let mut extra_magic = 0.0;
    let mut extra_true = 0.0_f64;
    maybe_apply_phase_rush(runtime, now, target_id);
    extra_true += maybe_apply_first_strike(
        runtime,
        now,
        ability_raw_damage,
        attacker_level,
        RuneProcTriggerSource::Ability,
    );
    if runtime.has_press_the_attack {
        record_rune_proc_attempt(runtime, "presstheattack", RuneProcTriggerSource::Ability);
    }
    let pta_multiplier = press_the_attack_damage_multiplier(runtime, target_id, now);
    if pta_multiplier > 0.0 {
        record_rune_proc_eligibility(runtime, "presstheattack", RuneProcTriggerSource::Ability);
        let pta_bonus_true = pta_multiplier * ability_raw_damage.max(0.0);
        extra_true += pta_bonus_true;
        if pta_bonus_true > 0.0 {
            record_rune_proc(
                runtime,
                "presstheattack",
                RuneProcTriggerSource::Ability,
                pta_bonus_true,
                0.0,
            );
        }
    }
    if runtime.has_conqueror {
        record_rune_proc_attempt(runtime, "conqueror", RuneProcTriggerSource::Ability);
        add_conqueror_stacks(runtime, defaults.conqueror.ability_hit_stacks, now);
        record_rune_proc_eligibility(runtime, "conqueror", RuneProcTriggerSource::Ability);
        let adaptive_ability_power = level_scaled_range_value(
            attacker_level,
            defaults.conqueror.adaptive_ability_power_per_stack_by_level,
        ) * runtime.conqueror_stacks as f64;
        let conqueror_damage = adaptive_ability_power * ability_ap_ratio.max(0.0);
        extra_magic += conqueror_damage;
        if conqueror_damage > 0.0 {
            record_rune_proc(
                runtime,
                "conqueror",
                RuneProcTriggerSource::Ability,
                conqueror_damage,
                0.0,
            );
        }
    }
    extra_magic += ability_ap_ratio.max(0.0) * gathering_storm_bonus_ability_power(runtime, now);

    if runtime.has_liandry {
        extra_magic += 0.04 * target_max_health.max(0.0);
    }

    if runtime.has_luden && now >= runtime.luden_ready_at {
        extra_magic += 90.0 + 0.10 * ability_raw_damage.max(0.0);
        runtime.luden_ready_at = now + runtime.luden_cooldown_seconds;
    }
    let electrocute_damage = maybe_apply_electrocute(
        runtime,
        now,
        target_id,
        attacker_level,
        RuneProcTriggerSource::Ability,
    );
    extra_magic += electrocute_damage;
    extra_magic += maybe_apply_arcane_comet(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
    );
    extra_magic += maybe_apply_summon_aery(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::Ability,
    );
    extra_magic += maybe_apply_dark_harvest(
        runtime,
        now,
        target_current_health,
        target_max_health,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::Ability,
    );

    (
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_magic,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_true,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
    )
}

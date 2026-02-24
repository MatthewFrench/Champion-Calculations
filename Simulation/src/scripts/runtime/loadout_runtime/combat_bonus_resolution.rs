mod projection_helpers;
mod rune_proc_state_mutations;

use self::projection_helpers::{
    gathering_storm_bonus_ability_power, press_the_attack_damage_multiplier,
};
use self::rune_proc_state_mutations::{
    add_conqueror_stacks, maybe_apply_arcane_comet, maybe_apply_dark_harvest,
    maybe_apply_electrocute, maybe_apply_first_strike, maybe_apply_phase_rush,
    maybe_apply_summon_aery, update_hail_of_blades_state,
};
use super::rune_proc_telemetry::{
    RuneProcTriggerSource, record_rune_proc, record_rune_proc_attempt, record_rune_proc_eligibility,
};
use super::{LoadoutRuntimeState, OnHitEffectProfile, level_scaled_range_value};
use crate::defaults::rune_runtime_defaults;
use crate::scripts::runtime::stat_resolution::{
    RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
};

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
    let defaults = rune_runtime_defaults();
    decay_expired_conqueror_stacks(runtime, now);
    runtime.attacks_landed += 1;
    if runtime.has_lethal_tempo {
        runtime.lethal_tempo_stacks =
            (runtime.lethal_tempo_stacks + 1).min(defaults.lethal_tempo.max_stacks.max(1));
    }
    if runtime.has_guinsoo {
        runtime.guinsoo_stacks = (runtime.guinsoo_stacks + 1).min(8);
    }
    update_hail_of_blades_state(runtime, now);

    let magic = profile.on_hit_magic_flat + profile.on_hit_magic_ad_ratio * attack_damage;
    let mut extra_physical = 0.0;
    let mut extra_magic = magic.max(0.0);
    let mut extra_true = 0.0;

    maybe_apply_phase_rush(runtime, now, target_id);
    extra_true += maybe_apply_first_strike(
        runtime,
        now,
        attack_damage,
        attacker_level,
        RuneProcTriggerSource::OnHit,
    );
    if runtime.has_press_the_attack {
        record_rune_proc_attempt(runtime, "presstheattack", RuneProcTriggerSource::OnHit);
    }
    let pta_multiplier = press_the_attack_damage_multiplier(runtime, target_id, now);
    if pta_multiplier > 0.0 {
        record_rune_proc_eligibility(runtime, "presstheattack", RuneProcTriggerSource::OnHit);
        let pta_bonus_true = pta_multiplier * attack_damage.max(0.0);
        extra_true += pta_bonus_true;
        if pta_bonus_true > 0.0 {
            record_rune_proc(
                runtime,
                "presstheattack",
                RuneProcTriggerSource::OnHit,
                pta_bonus_true,
                0.0,
            );
        }
    }

    if profile.periodic_true_hit_every > 0
        && runtime
            .attacks_landed
            .is_multiple_of(profile.periodic_true_hit_every)
    {
        extra_true += profile.periodic_true_hit_base
            + profile.periodic_true_hit_ad_ratio * attack_damage
            + profile.periodic_true_hit_target_max_health_ratio * target_max_health;
    }

    if runtime.has_blade_of_the_ruined_king {
        extra_physical += 0.06 * target_current_health.max(0.0);
    }

    if runtime.has_kraken && runtime.attacks_landed.is_multiple_of(3) {
        extra_true += 65.0 + 0.45 * attack_damage;
    }

    if runtime.has_grasp {
        record_rune_proc_attempt(runtime, "graspoftheundying", RuneProcTriggerSource::OnHit);
    }
    if runtime.has_grasp && now >= runtime.grasp_ready_at {
        record_rune_proc_eligibility(runtime, "graspoftheundying", RuneProcTriggerSource::OnHit);
        let grasp_damage = defaults.grasp_of_the_undying.base_magic_damage.max(0.0)
            + defaults
                .grasp_of_the_undying
                .target_max_health_ratio
                .max(0.0)
                * target_max_health.max(0.0);
        extra_magic += grasp_damage;
        runtime.grasp_ready_at = now + runtime.grasp_cooldown_seconds;
        record_rune_proc(
            runtime,
            "graspoftheundying",
            RuneProcTriggerSource::OnHit,
            grasp_damage,
            0.0,
        );
    }

    if runtime.has_heartsteel && now >= runtime.heartsteel_ready_at {
        extra_physical += 70.0 + 0.06 * attacker_max_health.max(0.0);
        runtime.heartsteel_ready_at = now + runtime.heartsteel_cooldown_seconds;
    }
    if runtime.has_fleet_footwork {
        record_rune_proc_attempt(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
    }
    if runtime.has_fleet_footwork && now >= runtime.fleet_ready_at {
        record_rune_proc_eligibility(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
        runtime.pending_fleet_heal +=
            level_scaled_range_value(attacker_level, defaults.fleet_footwork.heal_by_level)
                + defaults.fleet_footwork.attack_damage_ratio.max(0.0) * attack_damage.max(0.0);
        runtime.fleet_ready_at = now + defaults.fleet_footwork.cooldown_seconds.max(0.0);
    }
    if runtime.has_press_the_attack
        && let Some(target_idx) = target_id
    {
        let state = runtime
            .press_the_attack_targets
            .entry(target_idx)
            .or_default();
        if now > state.stack_expires_at {
            state.stacks = 0;
        }
        state.stacks = (state.stacks + 1).min(3);
        state.stack_expires_at = now + defaults.press_the_attack.stack_window_seconds.max(0.0);
        if state.stacks >= 3 {
            let pta_burst_damage = level_scaled_range_value(
                attacker_level,
                defaults.press_the_attack.burst_magic_damage_by_level,
            );
            extra_magic += pta_burst_damage;
            state.stacks = 0;
            state.vulnerable_until = now
                + defaults
                    .press_the_attack
                    .vulnerability_duration_seconds
                    .max(0.0);
            record_rune_proc_eligibility(runtime, "presstheattack", RuneProcTriggerSource::OnHit);
            record_rune_proc(
                runtime,
                "presstheattack",
                RuneProcTriggerSource::OnHit,
                pta_burst_damage,
                0.0,
            );
        }
    }
    if runtime.has_conqueror {
        let basic_attack_stacks = if runtime.owner_is_melee {
            defaults.conqueror.melee_basic_attack_stacks
        } else {
            defaults.conqueror.ranged_basic_attack_stacks
        };
        add_conqueror_stacks(runtime, basic_attack_stacks, now);
    }
    let electrocute_damage = maybe_apply_electrocute(
        runtime,
        now,
        target_id,
        attacker_level,
        RuneProcTriggerSource::OnHit,
    );
    extra_magic += electrocute_damage;
    extra_magic += maybe_apply_summon_aery(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::OnHit,
    );
    extra_magic += maybe_apply_dark_harvest(
        runtime,
        now,
        target_current_health,
        target_max_health,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::OnHit,
    );

    (
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_physical,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
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
    decay_expired_conqueror_stacks(runtime, now);
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

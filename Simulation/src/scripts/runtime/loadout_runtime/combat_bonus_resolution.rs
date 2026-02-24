use super::rune_proc_telemetry::{
    RuneProcTriggerSource, record_rune_proc, record_rune_proc_attempt, record_rune_proc_eligibility,
};
use super::{
    HitWindowTargetState, LoadoutRuntimeState, OnHitEffectProfile, level_scaled_range_value,
};
use crate::defaults::rune_runtime_defaults;
use crate::scripts::runtime::stat_resolution::{
    RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
};
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

fn add_conqueror_stacks(runtime: &mut LoadoutRuntimeState, stacks: usize, now: f64) {
    if !runtime.has_conqueror || stacks == 0 {
        return;
    }
    let defaults = &rune_runtime_defaults().conqueror;
    decay_expired_conqueror_stacks(runtime, now);
    runtime.conqueror_stacks = (runtime.conqueror_stacks + stacks).min(defaults.max_stacks.max(1));
    runtime.conqueror_expires_at = now + defaults.stack_duration_seconds.max(0.0);
}

fn maybe_apply_first_strike(
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

fn maybe_apply_electrocute(
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

fn maybe_apply_phase_rush(runtime: &mut LoadoutRuntimeState, now: f64, target_id: Option<usize>) {
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

fn gathering_storm_bonus_ability_power(runtime: &LoadoutRuntimeState, now: f64) -> f64 {
    if !runtime.has_gathering_storm {
        return 0.0;
    }
    let defaults = &rune_runtime_defaults().gathering_storm;
    let interval = defaults.interval_seconds.max(1.0);
    let intervals_completed = (now / interval).floor().max(0.0) as usize;
    if intervals_completed == 0 {
        return 0.0;
    }
    defaults
        .ability_power_by_interval
        .get(intervals_completed - 1)
        .copied()
        .or_else(|| defaults.ability_power_by_interval.last().copied())
        .unwrap_or(0.0)
}

fn update_hail_of_blades_state(runtime: &mut LoadoutRuntimeState, now: f64) {
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

fn maybe_apply_arcane_comet(
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

fn maybe_apply_summon_aery(
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

fn maybe_apply_dark_harvest(
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

fn press_the_attack_damage_multiplier(
    runtime: &LoadoutRuntimeState,
    target_id: Option<usize>,
    now: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().press_the_attack;
    if !runtime.has_press_the_attack {
        return 0.0;
    }
    target_id
        .and_then(|idx| runtime.press_the_attack_targets.get(&idx))
        .filter(|state| now <= state.vulnerable_until)
        .map(|_| defaults.vulnerability_true_damage_ratio.max(0.0))
        .unwrap_or(0.0)
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

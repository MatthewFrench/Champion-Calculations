use super::rune_proc_telemetry::{
    RuneProcTriggerSource, record_rune_proc, record_rune_proc_attempt, record_rune_proc_eligibility,
};
use super::{LoadoutRuntimeState, level_scaled_range_value};
use crate::defaults::rune_runtime_defaults;
use crate::scripts::runtime::stat_resolution::{
    RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
};

pub(super) fn on_outgoing_damage_heal_impl(
    runtime: &mut LoadoutRuntimeState,
    damage_dealt: f64,
    now: f64,
) -> f64 {
    let defaults = rune_runtime_defaults();
    super::combat_bonus_resolution::decay_expired_conqueror_stacks(runtime, now);
    if runtime.has_fleet_footwork {
        record_rune_proc_attempt(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
    }
    if runtime.has_conqueror {
        record_rune_proc_attempt(runtime, "conqueror", RuneProcTriggerSource::OnHit);
    }
    let mut heal = runtime.pending_fleet_heal.max(0.0);
    runtime.pending_fleet_heal = 0.0;
    if heal > 0.0 {
        record_rune_proc_eligibility(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
        record_rune_proc(
            runtime,
            "fleetfootwork",
            RuneProcTriggerSource::OnHit,
            0.0,
            heal,
        );
    }
    if runtime.has_conqueror
        && runtime.conqueror_stacks >= defaults.conqueror.max_stacks.max(1)
        && now <= runtime.conqueror_expires_at
        && damage_dealt > 0.0
    {
        record_rune_proc_eligibility(runtime, "conqueror", RuneProcTriggerSource::OnHit);
        let conqueror_heal_ratio = if runtime.owner_is_melee {
            defaults.conqueror.melee_heal_ratio.max(0.0)
        } else {
            defaults.conqueror.ranged_heal_ratio.max(0.0)
        };
        let conqueror_heal = damage_dealt.max(0.0) * conqueror_heal_ratio;
        heal += conqueror_heal;
        if conqueror_heal > 0.0 {
            record_rune_proc(
                runtime,
                "conqueror",
                RuneProcTriggerSource::OnHit,
                0.0,
                conqueror_heal,
            );
        }
    }
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: heal,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(super) fn on_enemy_kill_heal_impl(runtime: &mut LoadoutRuntimeState, max_health: f64) -> f64 {
    let defaults = rune_runtime_defaults();
    if !runtime.has_triumph || max_health <= 0.0 {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "triumph", RuneProcTriggerSource::EnemyKill);
    let heal = defaults.triumph.heal_max_health_ratio.max(0.0) * max_health.max(0.0);
    if heal <= 0.0 {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "triumph", RuneProcTriggerSource::EnemyKill);
    record_rune_proc(
        runtime,
        "triumph",
        RuneProcTriggerSource::EnemyKill,
        0.0,
        heal,
    );
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: heal,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(super) fn trigger_immobilize_rune_damage_impl(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    actor_bonus_health: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().aftershock;
    if !runtime.has_aftershock {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "aftershock", RuneProcTriggerSource::Immobilize);
    if now < runtime.aftershock_ready_at {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "aftershock", RuneProcTriggerSource::Immobilize);
    runtime.aftershock_ready_at = now + defaults.cooldown_seconds.max(0.0);
    runtime.aftershock_active_until = now + defaults.active_duration_seconds.max(0.0);
    let shockwave_magic =
        level_scaled_range_value(actor_level, defaults.shockwave_magic_damage_by_level)
            + defaults.shockwave_bonus_health_ratio.max(0.0) * actor_bonus_health.max(0.0);
    record_rune_proc(
        runtime,
        "aftershock",
        RuneProcTriggerSource::Immobilize,
        shockwave_magic,
        0.0,
    );
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: shockwave_magic,
            source: ScalarMetricSource::OutgoingAbilityDamage,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

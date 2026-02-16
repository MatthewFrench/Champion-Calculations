use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule, ScriptedEffectHitbox,
    on_ability_bonus_damage,
};
use crate::defaults::simulator_defaults;

pub(crate) const CHAMPION_KEY: &str = "morgana";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    let defaults = &simulator_defaults().champion_script_defaults.morgana;
    vec![
        ChampionScriptSchedule {
            event: ChampionScriptEvent::MorganaDarkBinding,
            start_offset_seconds: defaults.dark_binding_schedule.start_offset_seconds,
            interval_seconds: defaults.dark_binding_schedule.interval_seconds,
        },
        ChampionScriptSchedule {
            event: ChampionScriptEvent::MorganaSoulShackles,
            start_offset_seconds: defaults.soul_shackles_schedule.start_offset_seconds,
            interval_seconds: defaults.soul_shackles_schedule.interval_seconds,
        },
    ]
}

pub(crate) fn execute_dark_binding(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.morgana;
    if input.distance_to_target > defaults.dark_binding_cast_range {
        return Vec::new();
    }
    let raw_magic = defaults.dark_binding_magic_base_damage
        + defaults.dark_binding_magic_ability_power_ratio * input.burst_magic_damage.max(0.0);
    let (extra_magic, extra_true) =
        on_ability_bonus_damage(runtime, raw_magic, input.target_max_health, input.now);
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: input.ability_projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: defaults.dark_binding_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: defaults.dark_binding_stun_duration_seconds,
    }]
}

pub(crate) fn execute_soul_shackles(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.morgana;
    if input.distance_to_target > defaults.soul_shackles_cast_range {
        return Vec::new();
    }
    vec![
        ChampionScriptAction::ApplyDamage {
            source: input.actor_position,
            projectile_speed: 0.0,
            hitbox: ScriptedEffectHitbox::Circle {
                radius: defaults.soul_shackles_hitbox_radius,
            },
            physical: 0.0,
            magic: defaults.soul_shackles_initial_magic_damage,
            true_damage: 0.0,
            stun_duration: 0.0,
        },
        ChampionScriptAction::ScheduleFollowup {
            delay_seconds: defaults.soul_shackles_detonate_delay_seconds,
            priority: defaults.soul_shackles_detonate_priority,
            event: ChampionScriptEvent::MorganaSoulShacklesDetonate,
        },
    ]
}

pub(crate) fn execute_soul_shackles_detonate(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.morgana;
    if input.distance_to_target > defaults.soul_shackles_detonate_cast_range {
        return Vec::new();
    }
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: 0.0,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: defaults.soul_shackles_hitbox_radius,
        },
        physical: 0.0,
        magic: defaults.soul_shackles_detonate_magic_damage,
        true_damage: 0.0,
        stun_duration: defaults.soul_shackles_detonate_stun_duration_seconds,
    }]
}

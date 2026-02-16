use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule, ScriptedEffectHitbox,
    on_ability_bonus_damage,
};

pub(crate) const CHAMPION_KEY: &str = "morgana";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    profile.attack_range = 450.0;
    profile.attack_windup_seconds = 0.20;
    profile.attack_projectile_speed = 1800.0;
    profile.ability_windup_seconds = 0.25;
    profile.ability_projectile_speed = 1600.0;
    profile.burst_windup_seconds = 0.20;
    profile.burst_projectile_speed = 1500.0;
    profile.desired_combat_range = 500.0;
    profile.movement_speed_scale = 0.95;
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    vec![
        ChampionScriptSchedule {
            event: ChampionScriptEvent::MorganaDarkBinding,
            start_offset_seconds: 3.0,
            interval_seconds: 10.0,
        },
        ChampionScriptSchedule {
            event: ChampionScriptEvent::MorganaSoulShackles,
            start_offset_seconds: 8.0,
            interval_seconds: 22.0,
        },
    ]
}

pub(crate) fn execute_dark_binding(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    if input.distance_to_target > 1300.0 {
        return Vec::new();
    }
    let raw_magic = 140.0 + 0.25 * input.burst_magic_damage.max(0.0);
    let (extra_magic, extra_true) =
        on_ability_bonus_damage(runtime, raw_magic, input.target_max_health, input.now);
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: input.ability_projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle { radius: 80.0 },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 2.0,
    }]
}

pub(crate) fn execute_soul_shackles(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    if input.distance_to_target > 650.0 {
        return Vec::new();
    }
    vec![
        ChampionScriptAction::ApplyDamage {
            source: input.actor_position,
            projectile_speed: 0.0,
            hitbox: ScriptedEffectHitbox::Circle { radius: 340.0 },
            physical: 0.0,
            magic: 70.0,
            true_damage: 0.0,
            stun_duration: 0.0,
        },
        ChampionScriptAction::ScheduleFollowup {
            delay_seconds: 2.5,
            priority: 11,
            event: ChampionScriptEvent::MorganaSoulShacklesDetonate,
        },
    ]
}

pub(crate) fn execute_soul_shackles_detonate(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    if input.distance_to_target > 700.0 {
        return Vec::new();
    }
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: 0.0,
        hitbox: ScriptedEffectHitbox::Circle { radius: 340.0 },
        physical: 0.0,
        magic: 170.0,
        true_damage: 0.0,
        stun_duration: 1.5,
    }]
}

use super::{
    EnemyBehaviorProfile, EnemyLoadoutRuntime, EnemyScriptAction, EnemyScriptEvent,
    EnemyScriptExecutionInput, EnemyScriptSchedule, on_ability_bonus_damage,
};

pub(crate) const CHAMPION_KEY: &str = "morgana";

pub(crate) fn apply_behavior(profile: &mut EnemyBehaviorProfile) {
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

pub(crate) fn schedules() -> Vec<EnemyScriptSchedule> {
    vec![
        EnemyScriptSchedule {
            event: EnemyScriptEvent::MorganaDarkBinding,
            start_offset_seconds: 3.0,
            interval_seconds: 10.0,
        },
        EnemyScriptSchedule {
            event: EnemyScriptEvent::MorganaSoulShackles,
            start_offset_seconds: 8.0,
            interval_seconds: 22.0,
        },
    ]
}

pub(crate) fn execute_dark_binding(
    input: EnemyScriptExecutionInput,
    runtime: &mut EnemyLoadoutRuntime,
) -> Vec<EnemyScriptAction> {
    if input.enemy_distance_to_controlled_champion > 1300.0 {
        return Vec::new();
    }
    let raw_magic = 140.0 + 0.25 * input.enemy_burst_magic_damage.max(0.0);
    let (extra_magic, extra_true) = on_ability_bonus_damage(
        runtime,
        raw_magic,
        input.controlled_champion_max_health,
        input.now,
    );
    vec![EnemyScriptAction::ApplyDamage {
        source: input.enemy_position,
        projectile_speed: input.enemy_ability_projectile_speed,
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 2.0,
    }]
}

pub(crate) fn execute_soul_shackles(input: EnemyScriptExecutionInput) -> Vec<EnemyScriptAction> {
    if input.enemy_distance_to_controlled_champion > 650.0 {
        return Vec::new();
    }
    vec![
        EnemyScriptAction::ApplyDamage {
            source: input.enemy_position,
            projectile_speed: 0.0,
            physical: 0.0,
            magic: 70.0,
            true_damage: 0.0,
            stun_duration: 0.0,
        },
        EnemyScriptAction::ScheduleFollowup {
            delay_seconds: 2.5,
            priority: 11,
            event: EnemyScriptEvent::MorganaSoulShacklesDetonate,
        },
    ]
}

pub(crate) fn execute_soul_shackles_detonate(
    input: EnemyScriptExecutionInput,
) -> Vec<EnemyScriptAction> {
    if input.enemy_distance_to_controlled_champion > 700.0 {
        return Vec::new();
    }
    vec![EnemyScriptAction::ApplyDamage {
        source: input.enemy_position,
        projectile_speed: 0.0,
        physical: 0.0,
        magic: 170.0,
        true_damage: 0.0,
        stun_duration: 1.5,
    }]
}

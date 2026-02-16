use super::{
    EnemyBehaviorProfile, EnemyScriptAction, EnemyScriptEvent, EnemyScriptExecutionInput,
    EnemyScriptSchedule,
};

pub(crate) const CHAMPION_KEY: &str = "warwick";

pub(crate) fn apply_behavior(profile: &mut EnemyBehaviorProfile) {
    profile.attack_range = 175.0;
    profile.attack_windup_seconds = 0.22;
    profile.attack_projectile_speed = 0.0;
    profile.on_hit_magic_flat = 18.0;
    profile.on_hit_magic_ad_ratio = 0.12;
    profile.ability_windup_seconds = 0.08;
    profile.desired_combat_range = 130.0;
    profile.movement_speed_scale = 1.08;
}

pub(crate) fn schedules() -> Vec<EnemyScriptSchedule> {
    vec![EnemyScriptSchedule {
        event: EnemyScriptEvent::WarwickInfiniteDuress,
        start_offset_seconds: 7.0,
        interval_seconds: 15.0,
    }]
}

pub(crate) fn execute_infinite_duress(input: EnemyScriptExecutionInput) -> Vec<EnemyScriptAction> {
    if input.enemy_distance_to_controlled_champion > 700.0 {
        return Vec::new();
    }
    vec![EnemyScriptAction::ApplyDamage {
        source: input.enemy_position,
        projectile_speed: 0.0,
        physical: input.enemy_physical_hit_damage * 1.8,
        magic: 80.0 + 0.25 * input.enemy_physical_hit_damage,
        true_damage: 0.0,
        stun_duration: 1.4,
    }]
}

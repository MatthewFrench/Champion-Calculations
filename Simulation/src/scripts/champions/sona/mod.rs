use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule, on_ability_bonus_damage,
};

pub(crate) const CHAMPION_KEY: &str = "sona";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    profile.attack_range = 550.0;
    profile.attack_windup_seconds = 0.18;
    profile.attack_projectile_speed = 1900.0;
    profile.ability_windup_seconds = 0.15;
    profile.ability_projectile_speed = 1800.0;
    profile.burst_windup_seconds = 0.12;
    profile.burst_projectile_speed = 1800.0;
    profile.desired_combat_range = 520.0;
    profile.movement_speed_scale = 1.02;
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::SonaCrescendo,
        start_offset_seconds: 9.0,
        interval_seconds: 20.0,
    }]
}

pub(crate) fn execute_crescendo(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    if input.distance_to_target > 1000.0 {
        return Vec::new();
    }
    let raw_magic = 190.0 + 0.20 * input.burst_magic_damage.max(0.0);
    let (extra_magic, extra_true) =
        on_ability_bonus_damage(runtime, raw_magic, input.target_max_health, input.now);
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: input.burst_projectile_speed,
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 1.5,
    }]
}

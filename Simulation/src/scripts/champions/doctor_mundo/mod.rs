use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule, on_ability_bonus_damage,
};

pub(crate) const CHAMPION_KEY: &str = "drmundo";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    profile.attack_range = 175.0;
    profile.attack_windup_seconds = 0.24;
    profile.attack_projectile_speed = 0.0;
    profile.ability_windup_seconds = 0.20;
    profile.ability_projectile_speed = 2000.0;
    profile.burst_windup_seconds = 0.15;
    profile.burst_projectile_speed = 1500.0;
    profile.desired_combat_range = 140.0;
    profile.movement_speed_scale = 0.98;
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::DoctorMundoInfectedCleaver,
        start_offset_seconds: 2.0,
        interval_seconds: 4.0,
    }]
}

pub(crate) fn execute_infected_cleaver(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    if input.distance_to_target > 1050.0 {
        return Vec::new();
    }
    let raw_magic = (0.15 * input.target_current_health.max(0.0)).clamp(80.0, 320.0) + 20.0;
    let (extra_magic, extra_true) =
        on_ability_bonus_damage(runtime, raw_magic, input.target_max_health, input.now);
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: input.ability_projectile_speed,
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

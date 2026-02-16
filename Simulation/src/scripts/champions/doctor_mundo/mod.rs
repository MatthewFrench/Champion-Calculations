use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule, ScriptedEffectHitbox,
    on_ability_bonus_damage,
};
use crate::defaults::simulator_defaults;

pub(crate) const CHAMPION_KEY: &str = "drmundo";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    let defaults = &simulator_defaults().champion_script_defaults.doctor_mundo;
    let schedule = defaults.infected_cleaver_schedule;
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::DoctorMundoInfectedCleaver,
        start_offset_seconds: schedule.start_offset_seconds,
        interval_seconds: schedule.interval_seconds,
    }]
}

pub(crate) fn execute_infected_cleaver(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.doctor_mundo;
    if input.distance_to_target > defaults.infected_cleaver_cast_range {
        return Vec::new();
    }
    let raw_magic = (defaults.infected_cleaver_current_health_ratio
        * input.target_current_health.max(0.0))
    .clamp(
        defaults.infected_cleaver_min_magic_damage,
        defaults.infected_cleaver_max_magic_damage,
    ) + defaults.infected_cleaver_flat_magic_damage;
    let (extra_magic, extra_true) =
        on_ability_bonus_damage(runtime, raw_magic, input.target_max_health, input.now);
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: input.ability_projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: defaults.infected_cleaver_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

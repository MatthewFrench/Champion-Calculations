use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule, ScriptedEffectHitbox,
    on_ability_bonus_damage,
};
use crate::defaults::simulator_defaults;

pub(crate) const CHAMPION_KEY: &str = "sona";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    let defaults = &simulator_defaults().champion_script_defaults.sona;
    let schedule = defaults.crescendo_schedule;
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::SonaCrescendo,
        start_offset_seconds: schedule.start_offset_seconds,
        interval_seconds: schedule.interval_seconds,
    }]
}

pub(crate) fn execute_crescendo(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.sona;
    if input.distance_to_target > defaults.crescendo_cast_range {
        return Vec::new();
    }
    let raw_magic = defaults.crescendo_magic_base_damage
        + defaults.crescendo_magic_ability_power_ratio * input.burst_magic_damage.max(0.0);
    let (extra_magic, extra_true) =
        on_ability_bonus_damage(runtime, raw_magic, input.target_max_health, input.now);
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: input.burst_projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: defaults.crescendo_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: defaults.crescendo_stun_duration_seconds,
    }]
}

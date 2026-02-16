use super::{
    ChampionBehaviorProfile, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule, ScriptedEffectHitbox,
};
use crate::defaults::simulator_defaults;

pub(crate) const CHAMPION_KEY: &str = "warwick";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    let defaults = &simulator_defaults().champion_script_defaults.warwick;
    let schedule = defaults.infinite_duress_schedule;
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::WarwickInfiniteDuress,
        start_offset_seconds: schedule.start_offset_seconds,
        interval_seconds: schedule.interval_seconds,
    }]
}

pub(crate) fn execute_infinite_duress(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.warwick;
    if input.distance_to_target > defaults.infinite_duress_cast_range {
        return Vec::new();
    }
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: 0.0,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: defaults.infinite_duress_hitbox_radius,
        },
        physical: input.physical_hit_damage * defaults.infinite_duress_physical_attack_damage_ratio,
        magic: defaults.infinite_duress_magic_base_damage
            + defaults.infinite_duress_magic_attack_damage_ratio * input.physical_hit_damage,
        true_damage: 0.0,
        stun_duration: defaults.infinite_duress_stun_duration_seconds,
    }]
}

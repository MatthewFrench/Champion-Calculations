use super::{
    ChampionBehaviorProfile, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule,
};
use crate::defaults::simulator_defaults;

pub(crate) const CHAMPION_KEY: &str = "vayne";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    let defaults = &simulator_defaults().champion_script_defaults.vayne;
    let schedule = defaults.tumble_empower_schedule;
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::VayneTumbleEmpower,
        start_offset_seconds: schedule.start_offset_seconds,
        interval_seconds: schedule.interval_seconds,
    }]
}

pub(crate) fn execute_tumble_empower(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.vayne;
    vec![ChampionScriptAction::AddNextAttackBonusPhysical {
        amount: defaults.tumble_bonus_physical_attack_damage_ratio * input.physical_hit_damage,
        trace_message: "empowered next attack",
    }]
}

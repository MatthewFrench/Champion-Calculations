mod controlled_champion;
pub(crate) mod vladimir;

mod champion_behavior_profile_channels;
mod champion_script_effect_types;
mod champion_script_event_channels;
mod doctor_mundo;
mod morgana;
mod runtime_effect_channels;
mod sona;
mod vayne;
mod warwick;

pub(crate) use self::champion_behavior_profile_channels::{
    ChampionBehaviorProfile, apply_behavior_override, behavior_profile,
};
pub(crate) use self::champion_script_effect_types::{
    ChampionScriptAction, ChampionScriptEvent, ChampionScriptExecutionInput, ChampionScriptPoint,
    ScriptedEffectHitbox,
};
pub(crate) use self::champion_script_event_channels::{
    champion_script_event_cast_range, champion_script_event_cooldown_seconds,
    champion_script_event_for_ability_id, champion_script_event_label,
    execute_champion_script_event, scripted_champion_events,
};
pub(crate) use self::runtime_effect_channels::{
    ChampionLoadoutRuntime, ChampionRuneProcTelemetryEntry, attack_speed_multiplier,
    build_champion_loadout_runtime, clear_transient_combat_state, describe_rune_proc_telemetry,
    describe_runtime_effect_cooldowns, describe_runtime_effect_stacks, enemy_kill_heal,
    incoming_damage_multipliers, movement_speed_multiplier, on_ability_bonus_damage,
    on_hit_bonus_damage, on_immobilize_rune_damage, outgoing_damage_heal, tick_regen_heal,
};
pub(crate) use controlled_champion::*;

#[cfg(test)]
#[path = "champions/tests/champions_tests.rs"]
mod tests;

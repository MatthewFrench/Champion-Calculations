use crate::to_norm_key;

use super::{
    ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, doctor_mundo, morgana, sona, vayne, warwick,
};

pub(crate) fn scripted_champion_events(champion_name: &str) -> Vec<ChampionScriptEvent> {
    match to_norm_key(champion_name).as_str() {
        warwick::CHAMPION_KEY => vec![ChampionScriptEvent::WarwickInfiniteDuress],
        vayne::CHAMPION_KEY => vec![ChampionScriptEvent::VayneTumbleEmpower],
        morgana::CHAMPION_KEY => vec![
            ChampionScriptEvent::MorganaDarkBinding,
            ChampionScriptEvent::MorganaSoulShackles,
        ],
        sona::CHAMPION_KEY => vec![ChampionScriptEvent::SonaCrescendo],
        doctor_mundo::CHAMPION_KEY => vec![ChampionScriptEvent::DoctorMundoInfectedBonesaw],
        _ => Vec::new(),
    }
}

pub(crate) fn champion_script_event_cooldown_seconds(
    champion_name: &str,
    event: ChampionScriptEvent,
) -> Option<f64> {
    match to_norm_key(champion_name).as_str() {
        warwick::CHAMPION_KEY => warwick::event_cooldown_seconds(event),
        vayne::CHAMPION_KEY => vayne::event_cooldown_seconds(event),
        morgana::CHAMPION_KEY => morgana::event_cooldown_seconds(event),
        sona::CHAMPION_KEY => sona::event_cooldown_seconds(event),
        doctor_mundo::CHAMPION_KEY => doctor_mundo::event_cooldown_seconds(event),
        _ => None,
    }
}

pub(crate) fn champion_script_event_label(event: ChampionScriptEvent) -> &'static str {
    match event {
        ChampionScriptEvent::WarwickInfiniteDuress => "Infinite Duress",
        ChampionScriptEvent::VayneTumbleEmpower => "Tumble Empower",
        ChampionScriptEvent::MorganaDarkBinding => "Dark Binding",
        ChampionScriptEvent::MorganaSoulShackles => "Soul Shackles",
        ChampionScriptEvent::MorganaSoulShacklesDetonate => "Soul Shackles Detonate",
        ChampionScriptEvent::SonaCrescendo => "Crescendo",
        ChampionScriptEvent::DoctorMundoInfectedBonesaw => "Infected Bonesaw",
    }
}

pub(crate) fn execute_champion_script_event(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    match input.event {
        ChampionScriptEvent::WarwickInfiniteDuress => warwick::execute_infinite_duress(input),
        ChampionScriptEvent::VayneTumbleEmpower => vayne::execute_tumble_empower(input),
        ChampionScriptEvent::MorganaDarkBinding => morgana::execute_dark_binding(input, runtime),
        ChampionScriptEvent::MorganaSoulShackles => morgana::execute_soul_shackles(input),
        ChampionScriptEvent::MorganaSoulShacklesDetonate => {
            morgana::execute_soul_shackles_detonate(input)
        }
        ChampionScriptEvent::SonaCrescendo => sona::execute_crescendo(input, runtime),
        ChampionScriptEvent::DoctorMundoInfectedBonesaw => {
            doctor_mundo::execute_infected_bonesaw(input, runtime)
        }
    }
}

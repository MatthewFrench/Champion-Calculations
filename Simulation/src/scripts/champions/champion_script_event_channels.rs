use crate::defaults::{
    doctor_mundo_infected_bonesaw_ability_defaults,
    morgana_binding_and_soul_shackles_ability_defaults, sona_crescendo_ability_defaults,
    warwick_infinite_duress_ability_defaults,
};
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

pub(crate) fn champion_script_event_for_ability_id(
    champion_name: &str,
    ability_id: &str,
) -> Option<ChampionScriptEvent> {
    match (
        to_norm_key(champion_name).as_str(),
        to_norm_key(ability_id).as_str(),
    ) {
        (warwick::CHAMPION_KEY, "warwickinfiniteduress" | "ultimate") => {
            Some(ChampionScriptEvent::WarwickInfiniteDuress)
        }
        (vayne::CHAMPION_KEY, "vaynetumble" | "basicability1") => {
            Some(ChampionScriptEvent::VayneTumbleEmpower)
        }
        (morgana::CHAMPION_KEY, "morganadarkbinding" | "basicability1") => {
            Some(ChampionScriptEvent::MorganaDarkBinding)
        }
        (morgana::CHAMPION_KEY, "morganasoulshackles" | "ultimate") => {
            Some(ChampionScriptEvent::MorganaSoulShackles)
        }
        (sona::CHAMPION_KEY, "sonacrescendo" | "ultimate") => {
            Some(ChampionScriptEvent::SonaCrescendo)
        }
        (doctor_mundo::CHAMPION_KEY, "drmundoinfectedbonesaw" | "basicability1") => {
            Some(ChampionScriptEvent::DoctorMundoInfectedBonesaw)
        }
        _ => None,
    }
}

pub(crate) fn champion_script_event_cast_range(
    champion_name: &str,
    event: ChampionScriptEvent,
) -> Option<f64> {
    match to_norm_key(champion_name).as_str() {
        warwick::CHAMPION_KEY => {
            let defaults = warwick_infinite_duress_ability_defaults(warwick::CHAMPION_KEY)?;
            if event == ChampionScriptEvent::WarwickInfiniteDuress {
                Some(defaults.infinite_duress_cast_range.max(0.0))
            } else {
                None
            }
        }
        vayne::CHAMPION_KEY => {
            if event == ChampionScriptEvent::VayneTumbleEmpower {
                // Tumble is a self-buff script event; no explicit target range requirement.
                Some(0.0)
            } else {
                None
            }
        }
        morgana::CHAMPION_KEY => {
            let defaults =
                morgana_binding_and_soul_shackles_ability_defaults(morgana::CHAMPION_KEY)?;
            match event {
                ChampionScriptEvent::MorganaDarkBinding => {
                    Some(defaults.dark_binding_cast_range.max(0.0))
                }
                ChampionScriptEvent::MorganaSoulShackles => {
                    Some(defaults.soul_shackles_cast_range.max(0.0))
                }
                _ => None,
            }
        }
        sona::CHAMPION_KEY => {
            let defaults = sona_crescendo_ability_defaults(sona::CHAMPION_KEY)?;
            if event == ChampionScriptEvent::SonaCrescendo {
                Some(defaults.crescendo_cast_range.max(0.0))
            } else {
                None
            }
        }
        doctor_mundo::CHAMPION_KEY => {
            let defaults =
                doctor_mundo_infected_bonesaw_ability_defaults(doctor_mundo::CHAMPION_KEY)?;
            if event == ChampionScriptEvent::DoctorMundoInfectedBonesaw {
                Some(defaults.cast_range.max(0.0))
            } else {
                None
            }
        }
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

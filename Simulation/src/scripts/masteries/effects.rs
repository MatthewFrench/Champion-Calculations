use crate::{MasterySelection, to_norm_key};

use crate::scripts::runtime::controlled_champion_loadout::ControlledChampionLoadoutRuntime;

pub(crate) const DYNAMIC_MASTERY_KEYS: &[&str] = &[
    "fervorofbattle",
    "thunderlordsdecree",
    "windspeakersblessing",
    "legendaryguardian",
    "perseverance",
];

pub(crate) fn apply_mastery_runtime_flag(
    runtime: &mut ControlledChampionLoadoutRuntime,
    mastery: &MasterySelection,
) {
    match to_norm_key(&mastery.name).as_str() {
        "fervorofbattle" => runtime.has_fervor = true,
        "thunderlordsdecree" => runtime.has_thunderlords = true,
        "windspeakersblessing" => runtime.has_windspeakers_blessing = true,
        "legendaryguardian" => runtime.has_legendary_guardian = true,
        "perseverance" => runtime.has_perseverance = true,
        _ => {}
    }
}

pub(crate) fn has_dynamic_mastery_effect(mastery_name: &str) -> bool {
    let key = to_norm_key(mastery_name);
    DYNAMIC_MASTERY_KEYS.contains(&key.as_str())
}

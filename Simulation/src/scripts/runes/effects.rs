use crate::to_norm_key;

use crate::scripts::runtime::controlled_champion_loadout::ControlledChampionLoadoutRuntime;

pub(crate) const DYNAMIC_RUNE_KEYS: &[&str] = &[
    "graspoftheundying",
    "lethaltempo",
    "arcanecomet",
    "summonaery",
    "secondwind",
    "triumph",
    "gatheringstorm",
];

pub(crate) fn apply_rune_runtime_flag(
    runtime: &mut ControlledChampionLoadoutRuntime,
    rune_name: &str,
) {
    match to_norm_key(rune_name).as_str() {
        "arcanecomet" => runtime.has_arcane_comet = true,
        "summonaery" => runtime.has_summon_aery = true,
        "secondwind" => runtime.has_second_wind = true,
        "triumph" => runtime.has_triumph = true,
        "gatheringstorm" => runtime.has_gathering_storm = true,
        _ => {}
    }
}

pub(crate) fn has_dynamic_rune_effect(rune_name: &str) -> bool {
    let key = to_norm_key(rune_name);
    DYNAMIC_RUNE_KEYS.contains(&key.as_str())
}

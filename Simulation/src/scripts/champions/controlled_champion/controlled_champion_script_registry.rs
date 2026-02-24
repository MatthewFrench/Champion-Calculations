use anyhow::Result;

use crate::to_norm_key;

use super::ControlledChampionScriptHandle;
use super::sona_controlled_champion_script::build_sona_script;
use super::vladimir_controlled_champion_script::build_vladimir_script;

type ControlledChampionScriptFactory = fn() -> Result<ControlledChampionScriptHandle>;

struct ControlledChampionScriptRegistryEntry {
    champion_key: &'static str,
    build: ControlledChampionScriptFactory,
}

const CONTROLLED_CHAMPION_SCRIPT_REGISTRY: &[ControlledChampionScriptRegistryEntry] = &[
    ControlledChampionScriptRegistryEntry {
        champion_key: "vladimir",
        build: build_vladimir_script,
    },
    ControlledChampionScriptRegistryEntry {
        champion_key: "sona",
        build: build_sona_script,
    },
];

pub(crate) fn supported_controlled_champion_script_keys() -> Vec<&'static str> {
    CONTROLLED_CHAMPION_SCRIPT_REGISTRY
        .iter()
        .map(|entry| entry.champion_key)
        .collect()
}

pub(crate) fn resolve_controlled_champion_script_result(
    champion_name: &str,
) -> Result<Option<ControlledChampionScriptHandle>> {
    let champion_key = to_norm_key(champion_name);
    let Some(entry) = CONTROLLED_CHAMPION_SCRIPT_REGISTRY
        .iter()
        .find(|entry| entry.champion_key == champion_key.as_str())
    else {
        return Ok(None);
    };
    Ok(Some((entry.build)()?))
}

#[cfg(test)]
pub(crate) fn resolve_controlled_champion_script(
    champion_name: &str,
) -> Option<ControlledChampionScriptHandle> {
    resolve_controlled_champion_script_result(champion_name)
        .ok()
        .flatten()
}

use crate::to_norm_key;

use super::ControlledChampionScriptHandle;
use super::vladimir_controlled_champion_script::build_vladimir_script;

type ControlledChampionScriptFactory = fn() -> ControlledChampionScriptHandle;

struct ControlledChampionScriptRegistryEntry {
    champion_key: &'static str,
    build: ControlledChampionScriptFactory,
}

const CONTROLLED_CHAMPION_SCRIPT_REGISTRY: &[ControlledChampionScriptRegistryEntry] =
    &[ControlledChampionScriptRegistryEntry {
        champion_key: "vladimir",
        build: build_vladimir_script,
    }];

pub(crate) fn resolve_controlled_champion_script(
    champion_name: &str,
) -> Option<ControlledChampionScriptHandle> {
    let champion_key = to_norm_key(champion_name);
    CONTROLLED_CHAMPION_SCRIPT_REGISTRY
        .iter()
        .find(|entry| entry.champion_key == champion_key.as_str())
        .map(|entry| (entry.build)())
}

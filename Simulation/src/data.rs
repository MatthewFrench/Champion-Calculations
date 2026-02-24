use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::defaults::{
    SearchQualityProfilePreset, controlled_champion_stasis_trigger_health_percent_default,
    guardian_angel_rebirth_defaults, protoplasm_lifeline_defaults, simulator_defaults,
    urf_respawn_defaults, zhonya_time_stop_defaults,
};
use crate::scripts::registry::hooks::{LoadoutHookContext, resolve_loadout_with_hooks};
use crate::scripts::runes::effects::has_dynamic_rune_effect;

use super::{
    BuildSearchConfig, ChampionBase, EXCLUDED_RANKS, EnemyConfig, ITEM_EVOLUTION_REPLACEMENTS,
    Item, LEGENDARY_RANK, LoadoutSelection, OpponentMovementMode, ResolvedLoadout,
    SearchQualityProfile, SimulationConfig, Stats, UrfBuffs, rand_index, shuffle_usize,
};

pub(crate) fn simulation_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

pub(crate) fn scenarios_dir() -> PathBuf {
    simulation_dir().join("scenarios")
}

pub(crate) fn resolve_scenario_path(raw: &str) -> PathBuf {
    let direct_candidate = PathBuf::from(raw);
    let looks_like_path = direct_candidate.is_absolute()
        || direct_candidate.exists()
        || raw.contains(std::path::MAIN_SEPARATOR)
        || raw.contains('/')
        || raw.contains('\\');
    if looks_like_path {
        return direct_candidate;
    }
    let by_name = scenarios_dir().join(&direct_candidate);
    if by_name.extension().is_some() {
        by_name
    } else {
        by_name.with_extension("json")
    }
}

pub(crate) fn items_dir() -> PathBuf {
    simulation_dir().join("..").join("Items")
}

pub(crate) fn game_mode_dir() -> PathBuf {
    simulation_dir().join("..").join("Game Mode")
}

pub(crate) fn characters_dir() -> PathBuf {
    simulation_dir().join("..").join("Characters")
}

pub(crate) fn rune_data_dir() -> PathBuf {
    simulation_dir().join("..").join("Masteries")
}

pub(crate) fn simulation_data_dir() -> PathBuf {
    simulation_dir().join("data")
}

pub(crate) fn load_json(path: &Path) -> Result<Value> {
    let text =
        fs::read_to_string(path).with_context(|| format!("Failed reading {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("Failed parsing {}", path.display()))
}

pub(crate) fn to_norm_key(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

mod loadout_effect_resolution;

pub(crate) use self::loadout_effect_resolution::{apply_structured_effect, resolve_loadout};

mod simulation_search_configuration_parsing;

pub(crate) use self::simulation_search_configuration_parsing::{
    apply_search_quality_profile, loadout_selection_key, parse_build_search, parse_enemy_config,
    parse_loadout_selection, parse_simulation_config, parse_stack_overrides_map,
};

mod loadout_domain_modeling;

pub(crate) use self::loadout_domain_modeling::{
    LoadoutDomain, build_loadout_domain, ensure_complete_loadout_selection,
    filter_loadout_domain_to_modeled_runes, is_legal_rune_page_selection, random_loadout_selection,
    validate_rune_page_selection,
};

#[cfg(test)]
pub(crate) use self::loadout_domain_modeling::RunePathDomain;

mod champion_item_preset_data_loading;

pub(crate) use self::champion_item_preset_data_loading::{
    EnemyUrfPreset, default_item_pool, enemy_loadout_from_preset, enemy_preset_data_path,
    item_pool_from_names, load_champion_bases, load_enemy_urf_presets, load_items, load_urf_buffs,
    lookup_champion_base, validate_enemy_urf_presets,
};

#[cfg(test)]
pub(crate) use self::champion_item_preset_data_loading::normalize_name;

#[cfg(test)]
#[path = "tests/data_tests.rs"]
mod tests;

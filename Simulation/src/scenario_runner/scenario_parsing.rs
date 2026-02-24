use anyhow::{Result, anyhow};
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::data::parse_stack_overrides_map;

use super::*;

pub(super) fn parse_controlled_champion_config(
    scenario: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
    default_level: usize,
    default_stack_overrides: &HashMap<String, f64>,
) -> Result<ControlledChampionScenarioConfig> {
    let controlled_champion = scenario
        .get("controlled_champion")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("Missing controlled_champion object"))?;
    let champion_name = controlled_champion
        .get("champion")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Missing controlled_champion.champion"))?;
    if controlled_champion.get("baseline_items").is_some() {
        return Err(anyhow!(
            "controlled_champion.baseline_items is no longer supported."
        ));
    }
    let loadout_selection = parse_loadout_selection(controlled_champion.get("loadout"))?;
    let champion_base = lookup_champion_base(champion_bases, champion_name)?;
    let level = controlled_champion
        .get("level")
        .and_then(Value::as_u64)
        .unwrap_or(default_level as u64)
        .max(1) as usize;
    if controlled_champion.get("assumptions").is_some() {
        return Err(anyhow!(
            "controlled_champion.assumptions is no longer supported. Use controlled_champion.stack_overrides."
        ));
    }
    if controlled_champion.get("item_stacks_at_level_20").is_some() {
        return Err(anyhow!(
            "controlled_champion.item_stacks_at_level_20 is no longer supported. Use controlled_champion.stack_overrides."
        ));
    }
    let mut stack_overrides = default_stack_overrides.clone();
    stack_overrides.extend(parse_stack_overrides_map(
        controlled_champion.get("stack_overrides"),
    )?);
    Ok(ControlledChampionScenarioConfig {
        base: champion_base,
        level,
        loadout_selection,
        stack_overrides,
    })
}

pub(super) fn parse_scenario_search_or_default(scenario: &Value) -> Result<BuildSearchConfig> {
    if let Some(search) = scenario.get("search") {
        return parse_build_search(search);
    }
    parse_build_search(&json!({ "strategy": "portfolio" }))
}

use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;

use crate::data::parse_stack_overrides_map;

use super::*;

#[derive(Debug, Clone)]
pub(super) struct ParsedOpponentEncounter {
    pub(super) name: String,
    pub(super) weight: f64,
    pub(super) actors: Vec<EnemyConfig>,
}

pub(super) fn parse_opponent_encounters(
    scenario: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
    default_level: usize,
    default_stack_overrides: &HashMap<String, f64>,
) -> Result<Vec<ParsedOpponentEncounter>> {
    let opponents = scenario
        .get("opponents")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("Missing opponents object"))?;
    let opponent_default_level = opponents
        .get("default_level")
        .and_then(Value::as_u64)
        .unwrap_or(default_level as u64)
        .max(1) as usize;
    if opponents.get("assumptions").is_some() {
        return Err(anyhow!(
            "opponents.assumptions is no longer supported. Use opponents.stack_overrides."
        ));
    }
    if opponents.get("item_stacks_at_level_20").is_some() {
        return Err(anyhow!(
            "opponents.item_stacks_at_level_20 is no longer supported. Use opponents.stack_overrides."
        ));
    }
    if opponents.get("shared_loadout").is_some() {
        return Err(anyhow!(
            "opponents.shared_loadout is no longer supported. Enemy champions always use their own preset rune pages and shard selections."
        ));
    }
    if opponents.get("uptime_windows_enabled").is_some() {
        return Err(anyhow!(
            "opponents.uptime_windows_enabled is no longer supported. Enemy combat windows are modeled by champion scripts and runtime state."
        ));
    }
    let mut opponent_default_stack_overrides = default_stack_overrides.clone();
    opponent_default_stack_overrides
        .extend(parse_stack_overrides_map(opponents.get("stack_overrides"))?);
    let encounters = opponents
        .get("encounters")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing opponents.encounters"))?;
    if encounters.is_empty() {
        return Err(anyhow!(
            "opponents.encounters must include at least one encounter"
        ));
    }
    let mut parsed = Vec::with_capacity(encounters.len());
    let mut total_weight = 0.0;
    let mut positive_weight_count = 0usize;
    let mut seen_actor_ids = HashMap::<String, String>::new();
    for (index, encounter) in encounters.iter().enumerate() {
        let name = encounter
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Missing opponents.encounters[{index}].name"))?;
        let weight = encounter
            .get("weight")
            .and_then(Value::as_f64)
            .unwrap_or(1.0);
        if weight < 0.0 {
            return Err(anyhow!(
                "opponents.encounters[{index}].weight must be >= 0.0"
            ));
        }
        total_weight += weight;
        if weight > 0.0 {
            positive_weight_count += 1;
        }
        let actors = encounter
            .get("actors")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing opponents.encounters[{index}].actors"))?;
        if actors.is_empty() {
            return Err(anyhow!(
                "opponents.encounters[{index}].actors must include at least one actor"
            ));
        }
        let parsed_actors = actors
            .iter()
            .map(|actor| {
                parse_enemy_config(
                    actor,
                    champion_bases,
                    opponent_default_level,
                    &opponent_default_stack_overrides,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        for actor in &parsed_actors {
            let actor_champion_key = to_norm_key(&actor.name);
            if let Some(previous_champion_key) = seen_actor_ids.get(&actor.id)
                && previous_champion_key != &actor_champion_key
            {
                return Err(anyhow!(
                    "opponents.encounters actor IDs must map to a single champion identity across encounters. Duplicate actor id '{}' is assigned to multiple champions (encounter '{}').",
                    actor.id,
                    name
                ));
            }
            seen_actor_ids.insert(actor.id.clone(), actor_champion_key);
        }
        parsed.push(ParsedOpponentEncounter {
            name: name.to_string(),
            weight,
            actors: parsed_actors,
        });
    }
    if positive_weight_count == 0 || total_weight <= 0.0 {
        return Err(anyhow!(
            "opponents.encounters must include at least one encounter with weight > 0.0"
        ));
    }
    Ok(parsed)
}

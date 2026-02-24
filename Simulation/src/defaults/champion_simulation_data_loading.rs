use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;

use super::defaults_path_key_and_effect_helpers::{
    is_character_support_file, normalize_key, normalize_snake_key, repository_root_dir,
};
use super::*;

pub(super) fn load_champion_simulation_data() -> Result<HashMap<String, ChampionSimulationData>> {
    let mut profiles = HashMap::new();
    let characters_dir = repository_root_dir().join("Characters");
    for entry in std::fs::read_dir(&characters_dir)
        .with_context(|| format!("Failed reading {}", characters_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid champion filename: {}", path.display()))?;
        if is_character_support_file(stem) {
            continue;
        }

        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
        let champion_file: ChampionFileEnvelope = serde_json::from_str(&text)
            .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;

        let ChampionFileEnvelope { name, behavior } = champion_file;

        if behavior.is_none() {
            continue;
        }

        let profile = ChampionSimulationData { behavior };

        profiles.insert(normalize_key(stem), profile.clone());
        if !name.trim().is_empty() {
            profiles.insert(normalize_key(&name), profile);
        }
    }
    Ok(profiles)
}

fn derive_ability_identifier(
    champion_name: &str,
    ability_key: &str,
    ability_data: &Value,
) -> String {
    if let Some(explicit_identifier) = ability_data
        .get("ability_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|identifier| !identifier.is_empty())
    {
        return explicit_identifier.to_string();
    }
    let champion_prefix = normalize_key(champion_name);
    let ability_name = ability_data
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(ability_key);
    let mut ability_suffix = normalize_snake_key(ability_name);
    if ability_suffix.is_empty() {
        ability_suffix = normalize_snake_key(ability_key);
    }
    if ability_suffix.is_empty() {
        champion_prefix
    } else {
        format!("{}_{}", champion_prefix, ability_suffix)
    }
}

pub(super) fn load_champion_slot_bindings() -> Result<HashMap<String, HashMap<String, String>>> {
    let mut bindings_by_champion = HashMap::new();
    let characters_dir = repository_root_dir().join("Characters");
    for entry in std::fs::read_dir(&characters_dir)
        .with_context(|| format!("Failed reading {}", characters_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid champion filename: {}", path.display()))?;
        if is_character_support_file(stem) {
            continue;
        }

        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
        let champion_data: Value = serde_json::from_str(&text)
            .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;

        let champion_name = champion_data
            .get("name")
            .and_then(Value::as_str)
            .filter(|name| !name.trim().is_empty())
            .unwrap_or(stem);
        let mut slot_bindings = HashMap::new();
        if let Some(abilities) = champion_data.get("abilities").and_then(Value::as_object) {
            for (ability_key, ability_data) in abilities {
                let slot = ability_data
                    .get("slot")
                    .and_then(Value::as_str)
                    .or_else(|| {
                        ability_data
                            .get("default_keybinding")
                            .and_then(Value::as_str)
                    })
                    .map(str::trim)
                    .filter(|slot| !slot.is_empty())
                    .map(|slot| slot.to_ascii_uppercase());
                let Some(slot) = slot else {
                    continue;
                };
                let ability_identifier =
                    derive_ability_identifier(champion_name, ability_key, ability_data);
                slot_bindings.insert(slot, ability_identifier);
            }
        }
        if slot_bindings.is_empty() {
            continue;
        }

        bindings_by_champion.insert(normalize_key(stem), slot_bindings.clone());
        bindings_by_champion.insert(normalize_key(champion_name), slot_bindings);
    }
    Ok(bindings_by_champion)
}

pub(super) fn load_champion_behavior_defaults() -> Result<ChampionBehaviorDefaults> {
    let path = repository_root_dir()
        .join("Characters")
        .join("ChampionDefaults.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion defaults file: {}", path.display()))?;
    let defaults_file: ChampionDefaultsFile = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion defaults file: {}", path.display()))?;
    let to_entry =
        |base_stats: ChampionBaseStatsDefaultsEntry,
         basic_attack: ChampionBasicAttackDefaultsEntry,
         behavior: ChampionBehaviorModifiersEntry| ChampionBehaviorDefaultsEntry {
            attack_range: base_stats.attack_range,
            attack_windup_seconds: basic_attack.base_windup_seconds,
            attack_projectile_speed: basic_attack.missile_speed,
            attack_effect_hitbox_radius: basic_attack.raw_timing_stats.gameplay_radius,
            on_hit_magic_flat: behavior.on_hit.magic_flat,
            on_hit_magic_ad_ratio: behavior.on_hit.magic_ad_ratio,
            periodic_true_hit_every: behavior.periodic_true_hit.every,
            periodic_true_hit_base: behavior.periodic_true_hit.base,
            periodic_true_hit_ad_ratio: behavior.periodic_true_hit.ad_ratio,
            periodic_true_hit_target_max_health_ratio: behavior
                .periodic_true_hit
                .target_max_health_ratio,
        };
    Ok(ChampionBehaviorDefaults {
        melee: to_entry(
            defaults_file.base_stats.melee,
            defaults_file.basic_attack.melee,
            defaults_file.behavior.melee,
        ),
        ranged: to_entry(
            defaults_file.base_stats.ranged,
            defaults_file.basic_attack.ranged,
            defaults_file.behavior.ranged,
        ),
    })
}

pub(super) fn load_champion_ability_execution_defaults() -> Result<AbilityExecutionDefaultsByRole> {
    let path = repository_root_dir()
        .join("Characters")
        .join("ChampionDefaults.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion defaults file: {}", path.display()))?;
    let defaults_file: ChampionDefaultsFile = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion defaults file: {}", path.display()))?;
    Ok(defaults_file.abilities.execution_defaults)
}

pub(super) fn load_champion_ai_profiles() -> Result<ChampionAiProfilesFile> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("champion_ai_profiles.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion AI profiles: {}", path.display()))?;
    let mut profiles: ChampionAiProfilesFile = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion AI profiles: {}", path.display()))?;
    let mut normalized = HashMap::new();
    for (champion_key, mut entry) in profiles.champions {
        let mut normalized_script_priority_overrides = HashMap::new();
        for (event_key, priority) in entry.script_priority_overrides {
            normalized_script_priority_overrides.insert(normalize_key(&event_key), priority);
        }
        entry.script_priority_overrides = normalized_script_priority_overrides;
        normalized.insert(normalize_key(&champion_key), entry);
    }
    profiles.champions = normalized;
    Ok(profiles)
}

#[allow(dead_code)]
pub(super) fn load_champion_ability_execution_data()
-> Result<HashMap<String, ChampionAbilityExecutionData>> {
    let mut data = HashMap::new();
    let characters_dir = repository_root_dir().join("Characters");
    for entry in std::fs::read_dir(&characters_dir)
        .with_context(|| format!("Failed reading {}", characters_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid champion filename: {}", path.display()))?;
        if is_character_support_file(stem) {
            continue;
        }

        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
        let champion_data: Value = serde_json::from_str(&text)
            .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;

        let mut abilities = HashMap::new();
        if let Some(ability_object) = champion_data.get("abilities").and_then(Value::as_object) {
            for (ability_key, ability) in ability_object {
                let execution = ability
                    .get("execution")
                    .cloned()
                    .and_then(|value| {
                        serde_json::from_value::<AbilityExecutionOverrideEntry>(value).ok()
                    })
                    .unwrap_or_default();
                if execution.cast_windup_seconds.is_some()
                    || execution.projectile_speed.is_some()
                    || execution.effect_hitbox_radius.is_some()
                {
                    abilities.insert(ability_key.clone(), execution);
                }
            }
        }

        let profile = ChampionAbilityExecutionData {
            is_melee: champion_is_melee_from_data(&champion_data),
            abilities,
        };
        data.insert(normalize_key(stem), profile.clone());
        if let Some(name) = champion_data.get("name").and_then(Value::as_str)
            && !name.trim().is_empty()
        {
            data.insert(normalize_key(name), profile);
        }
    }

    Ok(data)
}

pub(super) fn load_urf_respawn_defaults() -> Result<UrfRespawnDefaults> {
    let path = repository_root_dir().join("Game Mode").join("URF.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading URF mode file: {}", path.display()))?;
    let urf: UrfFileEnvelope = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing URF mode file: {}", path.display()))?;
    let respawn = urf
        .respawn
        .ok_or_else(|| anyhow!("Missing respawn in {}", path.display()))?;
    Ok(respawn)
}

fn champion_is_melee_from_data(champion_data: &Value) -> bool {
    if let Some(attack_type) = champion_data
        .get("basic_attack")
        .and_then(|basic| basic.get("attack_type"))
        .and_then(Value::as_str)
    {
        return attack_type.eq_ignore_ascii_case("melee");
    }
    champion_data
        .pointer("/base_stats/attack_range")
        .and_then(Value::as_f64)
        .map(|range| range <= 200.0)
        .unwrap_or(false)
}

pub(super) fn champion_ability_execution_defaults_for_role_internal(
    is_melee: bool,
) -> AbilityExecutionDefaultsEntry {
    let defaults = CHAMPION_ABILITY_EXECUTION_DEFAULTS.get_or_init(|| {
        load_champion_ability_execution_defaults().unwrap_or_else(|err| panic!("{}", err))
    });
    if is_melee {
        defaults.melee
    } else {
        defaults.ranged
    }
}

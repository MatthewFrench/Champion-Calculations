use super::item_pool_loading::item_is_allowed_in_urf;
use super::*;

#[derive(Debug, Clone)]
pub(crate) struct EnemyUrfPreset {
    pub(crate) champion: String,
    pub(crate) source_url: String,
    pub(crate) last_checked: String,
    pub(crate) item_names: Vec<String>,
    pub(crate) runes: Vec<String>,
    pub(crate) shards: Vec<String>,
}

pub(crate) fn enemy_preset_data_path() -> PathBuf {
    simulation_data_dir().join("enemy_urf_presets.json")
}

pub(crate) fn load_enemy_urf_presets() -> Result<HashMap<String, EnemyUrfPreset>> {
    let data = load_json(&enemy_preset_data_path())?;
    let defaults = data.get("defaults").and_then(Value::as_object);
    let default_source_url = defaults
        .and_then(|o| o.get("source_url"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let default_last_checked = defaults
        .and_then(|o| o.get("last_checked"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let presets = data
        .get("presets")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing presets array in {}",
                enemy_preset_data_path().display()
            )
        })?;

    let mut by_champion = HashMap::new();
    for preset in presets {
        let champion = preset
            .get("champion")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Missing preset champion field"))?
            .to_string();
        let item_names = preset
            .get("item_names")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing item_names for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let runes = preset
            .get("runes")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing runes for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let shards = preset
            .get("shards")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing shards for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let source_url = preset
            .get("source_url")
            .and_then(Value::as_str)
            .unwrap_or(&default_source_url)
            .to_string();
        let last_checked = preset
            .get("last_checked")
            .and_then(Value::as_str)
            .unwrap_or(&default_last_checked)
            .to_string();

        let loaded = EnemyUrfPreset {
            champion: champion.clone(),
            source_url,
            last_checked,
            item_names,
            runes,
            shards,
        };
        by_champion.insert(to_norm_key(&champion), loaded);
    }
    Ok(by_champion)
}

pub(crate) fn validate_enemy_urf_presets(
    presets: &HashMap<String, EnemyUrfPreset>,
    items: &HashMap<String, Item>,
    loadout_domain: &LoadoutDomain,
    urf: &UrfBuffs,
) -> Result<()> {
    let all_runes = loadout_domain
        .rune_paths
        .iter()
        .flat_map(|p| p.slot_runes.iter())
        .flat_map(|slot| slot.iter())
        .map(|s| to_norm_key(s))
        .collect::<HashSet<_>>();
    for preset in presets.values() {
        if preset.item_names.len() != 6 {
            bail!(
                "Enemy preset for {} must contain exactly six full items",
                preset.champion
            );
        }
        if preset.runes.len() != 6 {
            bail!(
                "Enemy preset for {} must contain exactly six runes",
                preset.champion
            );
        }
        if preset.shards.len() != 3 {
            bail!(
                "Enemy preset for {} must contain exactly three shards",
                preset.champion
            );
        }
        for item_name in &preset.item_names {
            if !items.contains_key(item_name) {
                bail!(
                    "Enemy preset item '{}' for {} is not present in Items/",
                    item_name,
                    preset.champion
                );
            }
            if !item_is_allowed_in_urf(item_name, urf) {
                bail!(
                    "Enemy preset item '{}' for {} is not present in Game Mode/URF.json allowed_items.",
                    item_name,
                    preset.champion
                );
            }
        }
        for rune_name in &preset.runes {
            if !all_runes.contains(&to_norm_key(rune_name)) {
                bail!(
                    "Enemy preset rune '{}' for {} is not present in RunesReforged",
                    rune_name,
                    preset.champion
                );
            }
        }
        validate_rune_page_selection(
            &LoadoutSelection {
                rune_names: preset.runes.clone(),
                shard_stats: preset.shards.clone(),
            },
            loadout_domain,
        )?;
        for (idx, shard) in preset.shards.iter().enumerate() {
            let valid = loadout_domain
                .shard_slots
                .get(idx)
                .map(|slot| slot.iter().any(|s| to_norm_key(s) == to_norm_key(shard)))
                .unwrap_or(false);
            if !valid {
                bail!(
                    "Enemy preset shard '{}' in slot {} for {} is invalid",
                    shard,
                    idx + 1,
                    preset.champion
                );
            }
        }
    }
    Ok(())
}

pub(crate) fn enemy_loadout_from_preset(preset: &EnemyUrfPreset) -> LoadoutSelection {
    LoadoutSelection {
        rune_names: preset.runes.clone(),
        shard_stats: preset.shards.clone(),
    }
}

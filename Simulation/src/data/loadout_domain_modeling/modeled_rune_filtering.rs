use super::*;

fn rune_has_modeled_static_or_dynamic_effect(
    rune: &Value,
    level: usize,
    for_controlled_champion: bool,
) -> Result<bool> {
    let rune_name = rune.get("name").and_then(Value::as_str).unwrap_or("");
    if !rune_name.is_empty() && has_dynamic_rune_effect(rune_name) {
        return Ok(true);
    }

    let effects = rune
        .get("effects_structured")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    for effect in effects {
        let mut stats = Stats::default();
        if apply_structured_effect(&effect, 1, level, for_controlled_champion, &mut stats)? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn filter_loadout_domain_to_modeled_runes(
    domain: &LoadoutDomain,
    level: usize,
    for_controlled_champion: bool,
) -> Result<LoadoutDomain> {
    let runes_data = load_json(&rune_data_dir().join("RunesReforged.json"))?;
    let mut modeled_rune_keys = HashSet::<String>::new();
    if let Some(paths) = runes_data.get("paths").and_then(Value::as_array) {
        for path in paths {
            let Some(slots) = path.get("slots").and_then(Value::as_array) else {
                continue;
            };
            for slot in slots {
                let Some(runes) = slot.get("runes").and_then(Value::as_array) else {
                    continue;
                };
                for rune in runes {
                    let Some(name) = rune.get("name").and_then(Value::as_str) else {
                        continue;
                    };
                    if rune_has_modeled_static_or_dynamic_effect(
                        rune,
                        level,
                        for_controlled_champion,
                    )? {
                        modeled_rune_keys.insert(to_norm_key(name));
                    }
                }
            }
        }
    }

    let rune_paths = domain
        .rune_paths
        .iter()
        .filter_map(|path| {
            if path.slot_runes.len() < 4 {
                return None;
            }
            let slot_runes = path
                .slot_runes
                .iter()
                .map(|slot| {
                    slot.iter()
                        .filter(|rune| modeled_rune_keys.contains(&to_norm_key(rune)))
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            let usable_as_primary = slot_runes.iter().take(4).all(|slot| !slot.is_empty());
            let usable_as_secondary = (1..=3)
                .filter(|slot| {
                    slot_runes
                        .get(*slot)
                        .map(|runes| !runes.is_empty())
                        .unwrap_or(false)
                })
                .count()
                >= 2;
            if !usable_as_primary && !usable_as_secondary {
                return None;
            }
            Some(RunePathDomain { slot_runes })
        })
        .collect::<Vec<_>>();

    Ok(LoadoutDomain {
        rune_paths,
        shard_slots: domain.shard_slots.clone(),
    })
}

use super::*;

#[derive(Debug, Clone)]
pub(crate) struct RunePathDomain {
    pub(crate) slot_runes: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutDomain {
    pub(crate) rune_paths: Vec<RunePathDomain>,
    pub(crate) shard_slots: [Vec<String>; 3],
}

pub(crate) fn build_loadout_domain() -> LoadoutDomain {
    let runes_data = load_json(&rune_data_dir().join("RunesReforged.json")).unwrap_or(Value::Null);

    let rune_paths = runes_data
        .get("paths")
        .and_then(Value::as_array)
        .map(|paths| {
            paths
                .iter()
                .filter_map(|path| {
                    let slots = path.get("slots").and_then(Value::as_array)?;
                    let slot_runes = slots
                        .iter()
                        .map(|slot| {
                            slot.get("runes")
                                .and_then(Value::as_array)
                                .map(|runes| {
                                    runes
                                        .iter()
                                        .filter_map(|r| r.get("name").and_then(Value::as_str))
                                        .map(ToOwned::to_owned)
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default()
                        })
                        .collect::<Vec<_>>();
                    if slot_runes.len() >= 4 {
                        Some(RunePathDomain { slot_runes })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let shard_slots = {
        let slots = runes_data
            .get("stat_shards")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let read_slot = |idx: usize| -> Vec<String> {
            slots
                .get(idx)
                .and_then(|s| s.get("options"))
                .and_then(Value::as_array)
                .map(|options| {
                    options
                        .iter()
                        .filter_map(|o| o.get("stat").and_then(Value::as_str))
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        [read_slot(0), read_slot(1), read_slot(2)]
    };

    LoadoutDomain {
        rune_paths,
        shard_slots,
    }
}

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

fn is_rune_name_in_slot(slot_runes: &[String], rune_name: &str) -> bool {
    let key = to_norm_key(rune_name);
    slot_runes.iter().any(|r| to_norm_key(r) == key)
}

fn is_shard_name_in_slot(slot_shards: &[String], shard_name: &str) -> bool {
    let key = to_norm_key(shard_name);
    slot_shards.iter().any(|s| to_norm_key(s) == key)
}

pub(crate) fn is_legal_rune_page_selection(
    selection: &LoadoutSelection,
    loadout_domain: &LoadoutDomain,
) -> bool {
    if selection.rune_names.len() != 6 || selection.shard_stats.len() != 3 {
        return false;
    }
    if loadout_domain.rune_paths.len() < 2 {
        return false;
    }
    for (idx, shard_name) in selection.shard_stats.iter().enumerate() {
        let valid = loadout_domain
            .shard_slots
            .get(idx)
            .map(|slot| is_shard_name_in_slot(slot, shard_name))
            .unwrap_or(false);
        if !valid {
            return false;
        }
    }

    let primary_keystone = &selection.rune_names[0];
    let primary_minor_1 = &selection.rune_names[1];
    let primary_minor_2 = &selection.rune_names[2];
    let primary_minor_3 = &selection.rune_names[3];
    let secondary_minor_1 = &selection.rune_names[4];
    let secondary_minor_2 = &selection.rune_names[5];

    for (primary_idx, primary_path) in loadout_domain.rune_paths.iter().enumerate() {
        if primary_path.slot_runes.len() < 4 {
            continue;
        }
        if !is_rune_name_in_slot(&primary_path.slot_runes[0], primary_keystone)
            || !is_rune_name_in_slot(&primary_path.slot_runes[1], primary_minor_1)
            || !is_rune_name_in_slot(&primary_path.slot_runes[2], primary_minor_2)
            || !is_rune_name_in_slot(&primary_path.slot_runes[3], primary_minor_3)
        {
            continue;
        }

        for (secondary_idx, secondary_path) in loadout_domain.rune_paths.iter().enumerate() {
            if secondary_idx == primary_idx || secondary_path.slot_runes.len() < 4 {
                continue;
            }
            let secondary_minor_1_slot = (1..=3).find(|slot| {
                is_rune_name_in_slot(&secondary_path.slot_runes[*slot], secondary_minor_1)
            });
            let secondary_minor_2_slot = (1..=3).find(|slot| {
                is_rune_name_in_slot(&secondary_path.slot_runes[*slot], secondary_minor_2)
            });
            if let (Some(slot_a), Some(slot_b)) = (secondary_minor_1_slot, secondary_minor_2_slot)
                && slot_a < slot_b
            {
                return true;
            }
        }
    }

    false
}

fn deterministic_default_loadout_selection(domain: &LoadoutDomain) -> Option<LoadoutSelection> {
    if domain.rune_paths.len() < 2 || domain.shard_slots.iter().any(Vec::is_empty) {
        return None;
    }

    for (primary_idx, primary_path) in domain.rune_paths.iter().enumerate() {
        if primary_path.slot_runes.len() < 4
            || primary_path.slot_runes[..4].iter().any(Vec::is_empty)
        {
            continue;
        }

        for (secondary_idx, secondary_path) in domain.rune_paths.iter().enumerate() {
            if secondary_idx == primary_idx || secondary_path.slot_runes.len() < 4 {
                continue;
            }

            let secondary_slots = (1..=3)
                .filter(|slot| {
                    secondary_path
                        .slot_runes
                        .get(*slot)
                        .map(|runes| !runes.is_empty())
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();
            if secondary_slots.len() < 2 {
                continue;
            }

            let slot_a = secondary_slots[0];
            let slot_b = secondary_slots[1];
            return Some(LoadoutSelection {
                rune_names: vec![
                    primary_path.slot_runes[0][0].clone(),
                    primary_path.slot_runes[1][0].clone(),
                    primary_path.slot_runes[2][0].clone(),
                    primary_path.slot_runes[3][0].clone(),
                    secondary_path.slot_runes[slot_a][0].clone(),
                    secondary_path.slot_runes[slot_b][0].clone(),
                ],
                shard_stats: domain
                    .shard_slots
                    .iter()
                    .map(|slot| slot[0].clone())
                    .collect::<Vec<_>>(),
            });
        }
    }

    None
}

pub(crate) fn validate_rune_page_selection(
    selection: &LoadoutSelection,
    loadout_domain: &LoadoutDomain,
) -> Result<()> {
    if is_legal_rune_page_selection(selection, loadout_domain) {
        return Ok(());
    }
    bail!(
        "Invalid rune page selection. Provide ordered runes_reforged.rune_names with six runes [primary keystone, primary slot2, primary slot3, primary slot4, secondary slot2/3/4 rune A, secondary higher-slot rune B], plus ordered shard_stats for the three shard slots."
    );
}

pub(crate) fn ensure_complete_loadout_selection(
    selection: &LoadoutSelection,
    loadout_domain: &LoadoutDomain,
) -> Result<LoadoutSelection> {
    if selection.rune_names.is_empty() && selection.shard_stats.is_empty() {
        let generated = deterministic_default_loadout_selection(loadout_domain).ok_or_else(|| {
            anyhow!(
                "Unable to derive a default rune page and shard selection from loaded rune domain data."
            )
        })?;
        validate_rune_page_selection(&generated, loadout_domain)?;
        return Ok(generated);
    }
    validate_rune_page_selection(selection, loadout_domain)?;
    Ok(selection.clone())
}

pub(crate) fn random_loadout_selection(
    base: &LoadoutSelection,
    domain: &LoadoutDomain,
    seed: &mut u64,
) -> LoadoutSelection {
    let mut out = base.clone();

    if domain.rune_paths.len() >= 2
        && domain.shard_slots.iter().all(|s| !s.is_empty())
        && domain.rune_paths.iter().all(|p| p.slot_runes.len() >= 4)
    {
        let primary_choices = domain
            .rune_paths
            .iter()
            .enumerate()
            .filter_map(|(idx, path)| {
                path.slot_runes
                    .iter()
                    .take(4)
                    .all(|slot| !slot.is_empty())
                    .then_some(idx)
            })
            .collect::<Vec<_>>();
        if !primary_choices.is_empty() {
            let primary_idx = primary_choices[rand_index(seed, primary_choices.len())];
            let secondary_choices = domain
                .rune_paths
                .iter()
                .enumerate()
                .filter_map(|(idx, path)| {
                    if idx == primary_idx {
                        return None;
                    }
                    let secondary_slot_count = (1..=3)
                        .filter(|slot| {
                            path.slot_runes
                                .get(*slot)
                                .map(|runes| !runes.is_empty())
                                .unwrap_or(false)
                        })
                        .count();
                    (secondary_slot_count >= 2).then_some(idx)
                })
                .collect::<Vec<_>>();
            if !secondary_choices.is_empty() {
                let secondary_idx = secondary_choices[rand_index(seed, secondary_choices.len())];
                let primary = &domain.rune_paths[primary_idx];
                let secondary = &domain.rune_paths[secondary_idx];
                let secondary_slots = (1..=3)
                    .filter(|slot| {
                        secondary
                            .slot_runes
                            .get(*slot)
                            .map(|r| !r.is_empty())
                            .unwrap_or(false)
                    })
                    .collect::<Vec<_>>();
                if secondary_slots.len() >= 2 {
                    let mut picks = secondary_slots.clone();
                    shuffle_usize(&mut picks, seed);
                    let (sa, sb) = if picks[0] <= picks[1] {
                        (picks[0], picks[1])
                    } else {
                        (picks[1], picks[0])
                    };
                    out.rune_names = vec![
                        primary.slot_runes[0][rand_index(seed, primary.slot_runes[0].len())]
                            .clone(),
                        primary.slot_runes[1][rand_index(seed, primary.slot_runes[1].len())]
                            .clone(),
                        primary.slot_runes[2][rand_index(seed, primary.slot_runes[2].len())]
                            .clone(),
                        primary.slot_runes[3][rand_index(seed, primary.slot_runes[3].len())]
                            .clone(),
                        secondary.slot_runes[sa][rand_index(seed, secondary.slot_runes[sa].len())]
                            .clone(),
                        secondary.slot_runes[sb][rand_index(seed, secondary.slot_runes[sb].len())]
                            .clone(),
                    ];
                    out.shard_stats = domain
                        .shard_slots
                        .iter()
                        .map(|slot| slot[rand_index(seed, slot.len())].clone())
                        .collect::<Vec<_>>();
                }
            }
        }
    }

    out
}

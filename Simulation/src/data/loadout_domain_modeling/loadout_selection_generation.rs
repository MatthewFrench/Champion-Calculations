use super::*;

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

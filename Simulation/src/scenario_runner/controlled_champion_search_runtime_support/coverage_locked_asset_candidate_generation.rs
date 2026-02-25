use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub(in crate::scenario_runner) enum CoverageLockedAsset {
    Item(usize),
    Rune(String),
    Shard { slot: usize, stat: String },
}

impl CoverageLockedAsset {
    pub(in crate::scenario_runner) fn display_label(&self, item_pool: &[Item]) -> String {
        match self {
            Self::Item(item_idx) => item_pool
                .get(*item_idx)
                .map(|item| format!("item:{}", item.name))
                .unwrap_or_else(|| format!("item_index:{item_idx}")),
            Self::Rune(name) => format!("rune:{name}"),
            Self::Shard { slot, stat } => format!("shard_slot_{}:{}", slot + 1, stat),
        }
    }
}

pub(in crate::scenario_runner) fn coverage_locked_assets(
    item_pool: &[Item],
    loadout_domain: &crate::data::LoadoutDomain,
) -> Vec<CoverageLockedAsset> {
    let mut out = Vec::new();
    for item_idx in 0..item_pool.len() {
        out.push(CoverageLockedAsset::Item(item_idx));
    }

    let mut rune_by_key = HashMap::<String, String>::new();
    let primary_path_indices = loadout_domain
        .rune_paths
        .iter()
        .enumerate()
        .filter_map(|(idx, path)| {
            (path.slot_runes.len() >= 4
                && path.slot_runes.iter().take(4).all(|slot| !slot.is_empty()))
            .then_some(idx)
        })
        .collect::<Vec<_>>();
    for &primary_idx in &primary_path_indices {
        let primary_path = &loadout_domain.rune_paths[primary_idx];
        let secondary_path_indices = loadout_domain
            .rune_paths
            .iter()
            .enumerate()
            .filter_map(|(idx, path)| {
                if idx == primary_idx || path.slot_runes.len() < 4 {
                    return None;
                }
                let secondary_slot_count = (1..=3)
                    .filter(|slot| {
                        path.slot_runes
                            .get(*slot)
                            .map(|slot_runes| !slot_runes.is_empty())
                            .unwrap_or(false)
                    })
                    .count();
                (secondary_slot_count >= 2).then_some(idx)
            })
            .collect::<Vec<_>>();
        if secondary_path_indices.is_empty() {
            continue;
        }

        for slot in primary_path.slot_runes.iter().take(4) {
            for rune_name in slot {
                let key = to_norm_key(rune_name);
                rune_by_key.entry(key).or_insert_with(|| rune_name.clone());
            }
        }
        for secondary_idx in secondary_path_indices {
            let secondary_path = &loadout_domain.rune_paths[secondary_idx];
            for slot_idx in 1..=3 {
                let Some(slot) = secondary_path.slot_runes.get(slot_idx) else {
                    continue;
                };
                if slot.is_empty() {
                    continue;
                }
                for rune_name in slot {
                    let key = to_norm_key(rune_name);
                    rune_by_key.entry(key).or_insert_with(|| rune_name.clone());
                }
            }
        }
    }
    let mut rune_values = rune_by_key.into_values().collect::<Vec<_>>();
    rune_values.sort_by_key(|name| to_norm_key(name));
    out.extend(rune_values.into_iter().map(CoverageLockedAsset::Rune));

    for (slot_idx, slot_stats) in loadout_domain.shard_slots.iter().enumerate() {
        for stat in slot_stats {
            out.push(CoverageLockedAsset::Shard {
                slot: slot_idx,
                stat: stat.clone(),
            });
        }
    }
    out
}

pub(in crate::scenario_runner) fn filter_item_pool_to_modeled_runtime_effects(
    item_pool: &[Item],
) -> Vec<Item> {
    item_pool
        .iter()
        .filter(|item| !is_item_effect_unmodeled(item))
        .cloned()
        .collect::<Vec<_>>()
}

pub(in crate::scenario_runner) fn max_legal_build_size(item_pool: &[Item]) -> usize {
    let boots_count = item_pool.iter().filter(|item| is_boots(item)).count();
    let non_boots_count = item_pool.len().saturating_sub(boots_count);
    non_boots_count + usize::from(boots_count > 0)
}

pub(in crate::scenario_runner) fn complete_partial_candidate_to_full(
    partial: &BuildKey,
    item_pool: &[Item],
    max_items: usize,
    seed: &mut u64,
) -> BuildKey {
    let mut candidate = canonical_build_candidate(partial.clone());
    if candidate.item_indices.len() >= max_items {
        candidate.item_indices.truncate(max_items);
        candidate.item_indices = canonical_key(&candidate.item_indices);
        return canonical_build_candidate(candidate);
    }

    while candidate.item_indices.len() < max_items {
        let options = (0..item_pool.len())
            .filter(|idx| can_add_item_to_build(item_pool, &candidate.item_indices, *idx))
            .collect::<Vec<_>>();
        if options.is_empty() {
            break;
        }
        let pick = options[rand_index(seed, options.len())];
        candidate.item_indices.push(pick);
        candidate.item_indices = canonical_key(&candidate.item_indices);
    }

    if candidate.item_indices.len() < max_items {
        candidate.item_indices = random_valid_build(item_pool, max_items, seed);
    }
    canonical_build_candidate(candidate)
}

fn candidate_matches_locked_asset(candidate: &BuildKey, asset: &CoverageLockedAsset) -> bool {
    match asset {
        CoverageLockedAsset::Item(item_idx) => candidate.item_indices.contains(item_idx),
        CoverageLockedAsset::Rune(name) => {
            let target = to_norm_key(name);
            candidate
                .loadout_selection
                .rune_names
                .iter()
                .any(|rune| to_norm_key(rune) == target)
        }
        CoverageLockedAsset::Shard { slot, stat } => candidate
            .loadout_selection
            .shard_stats
            .get(*slot)
            .map(|value| to_norm_key(value) == to_norm_key(stat))
            .unwrap_or(false),
    }
}

#[derive(Debug, Clone, Copy)]
enum LockedRunePlacement {
    Primary(usize),
    Secondary(usize),
}

#[derive(Debug, Clone, Copy)]
struct LockedRuneSelectionOption {
    primary_idx: usize,
    secondary_idx: usize,
    secondary_slot_a: usize,
    secondary_slot_b: usize,
    placement: LockedRunePlacement,
}

fn norm_equals(value: &str, target_key: &str) -> bool {
    to_norm_key(value) == target_key
}

fn canonical_slot_value_by_key(values: &[String], target_key: &str) -> Option<String> {
    values
        .iter()
        .find(|value| norm_equals(value, target_key))
        .cloned()
}

fn legal_primary_path_indices(loadout_domain: &crate::data::LoadoutDomain) -> Vec<usize> {
    loadout_domain
        .rune_paths
        .iter()
        .enumerate()
        .filter_map(|(idx, path)| {
            (path.slot_runes.len() >= 4
                && path.slot_runes.iter().take(4).all(|slot| !slot.is_empty()))
            .then_some(idx)
        })
        .collect::<Vec<_>>()
}

fn legal_secondary_slot_indices(slot_runes: &[Vec<String>]) -> Vec<usize> {
    (1..=3)
        .filter(|slot| {
            slot_runes
                .get(*slot)
                .map(|runes| !runes.is_empty())
                .unwrap_or(false)
        })
        .collect::<Vec<_>>()
}

fn random_legal_loadout_selection(
    base_loadout: &LoadoutSelection,
    loadout_domain: &crate::data::LoadoutDomain,
    seed: &mut u64,
) -> Option<LoadoutSelection> {
    let sampled = random_loadout_selection(base_loadout, loadout_domain, seed);
    if is_legal_rune_page_selection(&sampled, loadout_domain) {
        return Some(sampled);
    }
    ensure_complete_loadout_selection(&LoadoutSelection::default(), loadout_domain).ok()
}

fn build_locked_rune_selection_options(
    loadout_domain: &crate::data::LoadoutDomain,
    target_rune_key: &str,
) -> Vec<LockedRuneSelectionOption> {
    let mut options = Vec::new();
    for primary_idx in legal_primary_path_indices(loadout_domain) {
        let primary_path = &loadout_domain.rune_paths[primary_idx];
        let primary_matching_slots = (0..4)
            .filter(|slot| {
                primary_path
                    .slot_runes
                    .get(*slot)
                    .map(|slot_runes| {
                        slot_runes
                            .iter()
                            .any(|rune_name| norm_equals(rune_name, target_rune_key))
                    })
                    .unwrap_or(false)
            })
            .collect::<Vec<_>>();
        for (secondary_idx, secondary_path) in loadout_domain.rune_paths.iter().enumerate() {
            if secondary_idx == primary_idx || secondary_path.slot_runes.len() < 4 {
                continue;
            }
            let secondary_slots = legal_secondary_slot_indices(&secondary_path.slot_runes);
            if secondary_slots.len() < 2 {
                continue;
            }
            for slot_a_index in 0..secondary_slots.len() {
                for slot_b_index in (slot_a_index + 1)..secondary_slots.len() {
                    let secondary_slot_a = secondary_slots[slot_a_index];
                    let secondary_slot_b = secondary_slots[slot_b_index];
                    for primary_slot in &primary_matching_slots {
                        options.push(LockedRuneSelectionOption {
                            primary_idx,
                            secondary_idx,
                            secondary_slot_a,
                            secondary_slot_b,
                            placement: LockedRunePlacement::Primary(*primary_slot),
                        });
                    }
                    for secondary_slot in [secondary_slot_a, secondary_slot_b] {
                        let has_target = secondary_path
                            .slot_runes
                            .get(secondary_slot)
                            .map(|slot_runes| {
                                slot_runes
                                    .iter()
                                    .any(|rune_name| norm_equals(rune_name, target_rune_key))
                            })
                            .unwrap_or(false);
                        if has_target {
                            options.push(LockedRuneSelectionOption {
                                primary_idx,
                                secondary_idx,
                                secondary_slot_a,
                                secondary_slot_b,
                                placement: LockedRunePlacement::Secondary(secondary_slot),
                            });
                        }
                    }
                }
            }
        }
    }
    options
}

fn construct_locked_rune_loadout_selection(
    loadout_domain: &crate::data::LoadoutDomain,
    rune_name: &str,
    seed: &mut u64,
) -> Option<LoadoutSelection> {
    let target_rune_key = to_norm_key(rune_name);
    let options = build_locked_rune_selection_options(loadout_domain, &target_rune_key);
    if options.is_empty() {
        return None;
    }
    let option = options[rand_index(seed, options.len())];
    let primary_path = &loadout_domain.rune_paths[option.primary_idx];
    let secondary_path = &loadout_domain.rune_paths[option.secondary_idx];

    let mut rune_names = Vec::with_capacity(6);
    for primary_slot in 0..4 {
        let slot_runes = primary_path.slot_runes.get(primary_slot)?;
        let selected_rune = match option.placement {
            LockedRunePlacement::Primary(locked_slot) if locked_slot == primary_slot => {
                canonical_slot_value_by_key(slot_runes, &target_rune_key)?
            }
            _ => slot_runes[rand_index(seed, slot_runes.len())].clone(),
        };
        rune_names.push(selected_rune);
    }
    for secondary_slot in [option.secondary_slot_a, option.secondary_slot_b] {
        let slot_runes = secondary_path.slot_runes.get(secondary_slot)?;
        let selected_rune = match option.placement {
            LockedRunePlacement::Secondary(locked_slot) if locked_slot == secondary_slot => {
                canonical_slot_value_by_key(slot_runes, &target_rune_key)?
            }
            _ => slot_runes[rand_index(seed, slot_runes.len())].clone(),
        };
        rune_names.push(selected_rune);
    }

    let shard_stats = loadout_domain
        .shard_slots
        .iter()
        .map(|slot_stats| slot_stats[rand_index(seed, slot_stats.len())].clone())
        .collect::<Vec<_>>();
    let selection = LoadoutSelection {
        rune_names,
        shard_stats,
    };
    is_legal_rune_page_selection(&selection, loadout_domain).then_some(selection)
}

fn construct_locked_shard_loadout_selection(
    base_loadout: &LoadoutSelection,
    loadout_domain: &crate::data::LoadoutDomain,
    slot: usize,
    stat: &str,
    seed: &mut u64,
) -> Option<LoadoutSelection> {
    let target_shard_key = to_norm_key(stat);
    let canonical_shard_stat = loadout_domain
        .shard_slots
        .get(slot)
        .and_then(|slot_stats| canonical_slot_value_by_key(slot_stats, &target_shard_key))?;
    let mut selection = random_legal_loadout_selection(base_loadout, loadout_domain, seed)?;
    if slot >= selection.shard_stats.len() {
        return None;
    }
    selection.shard_stats[slot] = canonical_shard_stat;
    is_legal_rune_page_selection(&selection, loadout_domain).then_some(selection)
}

fn enforce_locked_item(
    item_pool: &[Item],
    max_items: usize,
    item_idx: usize,
    build: &mut Vec<usize>,
    seed: &mut u64,
) -> bool {
    if build.contains(&item_idx) {
        return true;
    }
    if build.len() < max_items && can_add_item_to_build(item_pool, build, item_idx) {
        build.push(item_idx);
        *build = canonical_key(build);
        return true;
    }

    let mut slots = (0..build.len()).collect::<Vec<_>>();
    shuffle_usize(&mut slots, seed);
    for slot in slots {
        let mut trial = build.clone();
        trial[slot] = item_idx;
        repair_build(item_pool, &mut trial, max_items, seed);
        if trial.contains(&item_idx) {
            *build = canonical_key(&trial);
            return true;
        }
    }

    let mut fallback = vec![item_idx];
    repair_build(item_pool, &mut fallback, max_items, seed);
    if fallback.contains(&item_idx) {
        *build = canonical_key(&fallback);
        return true;
    }
    false
}

fn random_loadout_matching_asset(
    base_loadout: &LoadoutSelection,
    loadout_domain: &crate::data::LoadoutDomain,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> Option<LoadoutSelection> {
    // Coverage stage must touch every locked asset reliably; direct construction avoids
    // rejection-heavy random sampling loops for rune/shard locks.
    match asset {
        CoverageLockedAsset::Rune(rune_name) => {
            construct_locked_rune_loadout_selection(loadout_domain, rune_name, seed)
        }
        CoverageLockedAsset::Shard { slot, stat } => construct_locked_shard_loadout_selection(
            base_loadout,
            loadout_domain,
            *slot,
            stat,
            seed,
        ),
        CoverageLockedAsset::Item(_) => None,
    }
}

fn enforce_locked_asset(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &mut BuildKey,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> bool {
    match asset {
        CoverageLockedAsset::Item(item_idx) => {
            if !enforce_locked_item(
                params.item_pool,
                params.max_items,
                *item_idx,
                &mut candidate.item_indices,
                seed,
            ) {
                return false;
            }
        }
        CoverageLockedAsset::Rune(_) | CoverageLockedAsset::Shard { .. } => {
            if let Some(selection) = random_loadout_matching_asset(
                params.base_loadout,
                params.loadout_domain,
                asset,
                seed,
            ) {
                candidate.loadout_selection = selection;
            } else {
                return false;
            }
        }
    }
    candidate.item_indices = canonical_key(&candidate.item_indices);
    candidate_matches_locked_asset(candidate, asset)
}

pub(in crate::scenario_runner) fn random_locked_candidate(
    params: &FullLoadoutSearchParams<'_>,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> Option<BuildKey> {
    let mut candidate = BuildKey {
        item_indices: random_valid_build(params.item_pool, params.max_items, seed),
        loadout_selection: random_loadout_selection(
            params.base_loadout,
            params.loadout_domain,
            seed,
        ),
    };
    if !enforce_locked_asset(params, &mut candidate, asset, seed) {
        return None;
    }
    Some(canonical_build_candidate(candidate))
}

pub(in crate::scenario_runner) fn mutate_locked_candidate(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &BuildKey,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> Option<BuildKey> {
    let mut out = candidate.clone();

    if !out.item_indices.is_empty() && rand_f64(seed) < 0.85 {
        let slot = rand_index(seed, out.item_indices.len());
        for _ in 0..params.item_pool.len().max(1) {
            let replacement = rand_index(seed, params.item_pool.len());
            if out.item_indices[slot] == replacement {
                continue;
            }
            out.item_indices[slot] = replacement;
            repair_build(
                params.item_pool,
                &mut out.item_indices,
                params.max_items,
                seed,
            );
            if out.item_indices.contains(&replacement) {
                break;
            }
        }
    }

    if rand_f64(seed) < 0.85 {
        out.loadout_selection =
            random_loadout_selection(&out.loadout_selection, params.loadout_domain, seed);
    }

    if !enforce_locked_asset(params, &mut out, asset, seed) {
        return None;
    }
    Some(canonical_build_candidate(out))
}

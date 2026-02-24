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
    let attempts = 4096usize;
    for _ in 0..attempts {
        let selection = random_loadout_selection(base_loadout, loadout_domain, seed);
        let candidate = BuildKey {
            item_indices: Vec::new(),
            loadout_selection: selection.clone(),
        };
        if candidate_matches_locked_asset(&candidate, asset) {
            return Some(selection);
        }
    }
    None
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

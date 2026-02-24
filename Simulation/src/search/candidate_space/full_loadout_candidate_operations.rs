use std::collections::HashSet;

use super::super::{
    BuildKey, FullLoadoutSearchParams, LoadoutSelection, canonical_build_candidate, canonical_key,
    rand_f64, random_loadout_selection, repair_build,
};
use super::item_candidate_operations::{crossover_builds, mutate_build};
use crate::loadout_selection_key;
use crate::random_valid_build;

pub(in crate::search) fn candidate_order_key(candidate: &BuildKey) -> String {
    format!(
        "i={}|l={}",
        candidate
            .item_indices
            .iter()
            .map(|idx| idx.to_string())
            .collect::<Vec<_>>()
            .join(","),
        loadout_selection_key(&candidate.loadout_selection)
    )
}

pub(in crate::search) fn random_full_candidate(
    params: &FullLoadoutSearchParams<'_>,
    seed: &mut u64,
) -> BuildKey {
    canonical_build_candidate(BuildKey {
        item_indices: random_valid_build(params.item_pool, params.max_items, seed),
        loadout_selection: random_loadout_selection(
            params.base_loadout,
            params.loadout_domain,
            seed,
        ),
    })
}

pub(in crate::search) fn candidate_loadout_variants(
    anchor: &LoadoutSelection,
    params: &FullLoadoutSearchParams<'_>,
    seed: &mut u64,
    random_samples: usize,
) -> Vec<LoadoutSelection> {
    let mut variants = Vec::new();
    let mut seen = HashSet::<String>::new();
    for base in [anchor.clone(), params.base_loadout.clone()] {
        let key = loadout_selection_key(&base);
        if seen.insert(key) {
            variants.push(base);
        }
    }
    for _ in 0..random_samples {
        let sampled = random_loadout_selection(anchor, params.loadout_domain, seed);
        let key = loadout_selection_key(&sampled);
        if seen.insert(key) {
            variants.push(sampled);
        }
    }
    if variants.is_empty() {
        variants.push(params.base_loadout.clone());
    }
    variants
}

pub(in crate::search) fn repair_full_candidate(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &mut BuildKey,
    seed: &mut u64,
) {
    repair_build(
        params.item_pool,
        &mut candidate.item_indices,
        params.max_items,
        seed,
    );
    candidate.item_indices = canonical_key(&candidate.item_indices);
    if candidate.loadout_selection.rune_names.len() != 6
        || candidate.loadout_selection.shard_stats.len() != 3
    {
        candidate.loadout_selection =
            random_loadout_selection(params.base_loadout, params.loadout_domain, seed);
    }
}

pub(in crate::search) fn mutate_full_candidate(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &mut BuildKey,
    mutation_rate: f64,
    seed: &mut u64,
) {
    let rate = mutation_rate.clamp(0.0, 1.0);
    if rand_f64(seed) <= rate {
        mutate_build(
            &mut candidate.item_indices,
            params.item_pool,
            params.max_items,
            rate,
            seed,
        );
    }
    if rand_f64(seed) <= rate {
        candidate.loadout_selection =
            random_loadout_selection(&candidate.loadout_selection, params.loadout_domain, seed);
    }
    repair_full_candidate(params, candidate, seed);
}

pub(in crate::search) fn crossover_full_candidates(
    parent_a: &BuildKey,
    parent_b: &BuildKey,
    params: &FullLoadoutSearchParams<'_>,
    seed: &mut u64,
) -> BuildKey {
    let item_indices = crossover_builds(
        &parent_a.item_indices,
        &parent_b.item_indices,
        params.item_pool,
        params.max_items,
        seed,
    );
    let mut loadout_selection = if rand_f64(seed) < 0.5 {
        parent_a.loadout_selection.clone()
    } else {
        parent_b.loadout_selection.clone()
    };
    if rand_f64(seed) < 0.25 {
        loadout_selection =
            random_loadout_selection(&loadout_selection, params.loadout_domain, seed);
    }
    let mut child = BuildKey {
        item_indices,
        loadout_selection,
    };
    repair_full_candidate(params, &mut child, seed);
    canonical_build_candidate(child)
}

#[cfg(test)]
#[path = "tests/full_loadout_candidate_operations_tests.rs"]
mod full_loadout_candidate_operations_tests;

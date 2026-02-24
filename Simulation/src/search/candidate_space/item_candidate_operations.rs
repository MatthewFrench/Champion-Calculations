use super::super::{Item, can_add_item_to_build, rand_f64, rand_index, repair_build};
use crate::shuffle_usize;

pub(in crate::search) fn tournament_parent(
    scored_population: &[(Vec<usize>, f64)],
    seed: &mut u64,
    tournament_size: usize,
) -> Vec<usize> {
    let mut best_idx = rand_index(seed, scored_population.len());
    for _ in 1..tournament_size.max(1) {
        let idx = rand_index(seed, scored_population.len());
        if scored_population[idx].1 > scored_population[best_idx].1 {
            best_idx = idx;
        }
    }
    scored_population[best_idx].0.clone()
}

pub(in crate::search) fn crossover_builds(
    parent_a: &[usize],
    parent_b: &[usize],
    item_pool: &[Item],
    max_items: usize,
    seed: &mut u64,
) -> Vec<usize> {
    let mut merged = parent_a.to_vec();
    for &idx in parent_b {
        if !merged.contains(&idx) {
            merged.push(idx);
        }
    }
    shuffle_usize(&mut merged, seed);
    let mut child = Vec::with_capacity(max_items);
    for idx in merged {
        if child.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &child, idx) {
            child.push(idx);
        }
    }
    repair_build(item_pool, &mut child, max_items, seed);
    child
}

pub(in crate::search) fn mutate_build(
    build: &mut Vec<usize>,
    item_pool: &[Item],
    max_items: usize,
    mutation_rate: f64,
    seed: &mut u64,
) {
    if build.is_empty() || rand_f64(seed) > mutation_rate.clamp(0.0, 1.0) {
        return;
    }
    let slot = rand_index(seed, build.len());
    let mut tries = 0usize;
    while tries < item_pool.len() {
        let candidate = rand_index(seed, item_pool.len());
        if candidate != build[slot] {
            let old = build[slot];
            build[slot] = candidate;
            if can_add_item_to_build(item_pool, &build[..slot], build[slot])
                && !build[(slot + 1)..].contains(&build[slot])
            {
                repair_build(item_pool, build, max_items, seed);
                return;
            }
            build[slot] = old;
        }
        tries += 1;
    }
    repair_build(item_pool, build, max_items, seed);
}

#[cfg(test)]
#[path = "tests/item_candidate_operations_tests.rs"]
mod item_candidate_operations_tests;

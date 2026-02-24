use std::collections::{HashMap, HashSet};

use super::super::candidate_space::full_loadout_candidate_operations::{
    candidate_order_key, crossover_full_candidates, mutate_full_candidate, repair_full_candidate,
};
use super::super::{BuildKey, BuildSearchConfig, canonical_build_candidate, rand_index};
use super::FullLoadoutSearchParams;

pub(super) fn generate_bleed_candidates_full_loadout(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<BuildKey>>,
) -> Vec<BuildKey> {
    if !search.bleed_enabled {
        return Vec::new();
    }

    let mut seed = search.seed ^ 0xB1EED_u64;
    let mut out = Vec::new();
    let mut seen = HashSet::<BuildKey>::new();
    let mut strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    strategies.sort_unstable();
    let mut elite_pool = Vec::new();

    for strategy in &strategies {
        if let Some(candidates) = strategy_elites.get(strategy) {
            for candidate in candidates.iter().take(search.ensemble_seed_top_k.max(1)) {
                let canonical = canonical_build_candidate(candidate.clone());
                if seen.insert(canonical.clone()) {
                    out.push(canonical.clone());
                    elite_pool.push(canonical);
                }
            }
        }
    }

    if elite_pool.is_empty() {
        return out;
    }

    let bleed_budget = if search.bleed_budget > 0 {
        search.bleed_budget
    } else {
        search.ranked_limit.max(800)
    };
    let cross_budget = bleed_budget / 2;
    let mutate_budget = bleed_budget - cross_budget;
    let mutation_rate = search.bleed_mutation_rate.clamp(0.0, 1.0);

    for _ in 0..cross_budget {
        let parent_a = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        let child = if strategies.len() >= 2 {
            let strategy_a = rand_index(&mut seed, strategies.len());
            let mut strategy_b = rand_index(&mut seed, strategies.len());
            if strategy_b == strategy_a {
                strategy_b = (strategy_b + 1) % strategies.len();
            }

            let list_a = strategy_elites
                .get(&strategies[strategy_a])
                .unwrap_or(&elite_pool);
            let list_b = strategy_elites
                .get(&strategies[strategy_b])
                .unwrap_or(&elite_pool);
            let parent_a_candidate = list_a
                .get(rand_index(&mut seed, list_a.len()))
                .cloned()
                .unwrap_or(parent_a.clone());
            let parent_b_candidate = list_b
                .get(rand_index(&mut seed, list_b.len()))
                .cloned()
                .unwrap_or(parent_a.clone());
            let mut child = crossover_full_candidates(
                &parent_a_candidate,
                &parent_b_candidate,
                params,
                &mut seed,
            );
            mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
            canonical_build_candidate(child)
        } else {
            let mut child = parent_a.clone();
            mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
            canonical_build_candidate(child)
        };
        if seen.insert(child.clone()) {
            out.push(child);
        }
    }

    for _ in 0..mutate_budget {
        let mut child = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
        repair_full_candidate(params, &mut child, &mut seed);
        let canonical = canonical_build_candidate(child);
        if seen.insert(canonical.clone()) {
            out.push(canonical);
        }
    }

    out.sort_by_key(candidate_order_key);
    out
}

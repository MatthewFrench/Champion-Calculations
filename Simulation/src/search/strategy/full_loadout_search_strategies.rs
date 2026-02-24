use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::candidate_space::full_loadout_candidate_operations::{
    candidate_loadout_variants, crossover_full_candidates, mutate_full_candidate,
    random_full_candidate, repair_full_candidate,
};
use super::super::candidate_space::full_loadout_candidate_scoring::{
    score_full_candidates, unique_ranked_full_candidates,
};
use super::super::{
    BuildKey, FullLoadoutSearchParams, GeneticSearchConfig, HillClimbSearchConfig,
    MctsSearchConfig, SimulatedAnnealingSearchConfig, canonical_build_candidate, canonical_key,
    is_boots, rand_f64, rand_index, random_loadout_selection,
};

pub(in crate::search) fn beam_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    beam_width: usize,
    seed: u64,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = seed;
    let mut candidates: Vec<BuildKey> = vec![BuildKey {
        item_indices: Vec::new(),
        loadout_selection: params.base_loadout.clone(),
    }];
    let mut final_scored = Vec::new();

    for _ in 0..params.max_items {
        if deadline_reached(deadline) {
            break;
        }
        let mut next_candidates = Vec::new();
        for candidate in &candidates {
            let has_boots = candidate
                .item_indices
                .iter()
                .any(|&idx| is_boots(&params.item_pool[idx]));
            let used = candidate
                .item_indices
                .iter()
                .copied()
                .collect::<HashSet<_>>();
            for (item_idx, item) in params.item_pool.iter().enumerate() {
                if used.contains(&item_idx) {
                    continue;
                }
                if is_boots(item) && has_boots {
                    continue;
                }
                let mut next = candidate.clone();
                next.item_indices.push(item_idx);
                next.item_indices = canonical_key(&next.item_indices);
                let loadout_variants =
                    candidate_loadout_variants(&next.loadout_selection, params, &mut local_seed, 1);
                for loadout_selection in loadout_variants {
                    let mut variant = next.clone();
                    variant.loadout_selection = loadout_selection;
                    next_candidates.push(canonical_build_candidate(variant));
                }
            }
        }
        let scored = score_full_candidates(next_candidates, score_fn, deadline);
        candidates = scored
            .iter()
            .take(beam_width.max(1))
            .map(|(candidate, _)| candidate.clone())
            .collect::<Vec<_>>();
        final_scored = scored;
    }
    final_scored
}

pub(in crate::search) fn random_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    random_samples: usize,
    seed: u64,
    limit: usize,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = seed;
    let mut candidates = Vec::with_capacity(random_samples.max(1));
    for _ in 0..random_samples.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        candidates.push(random_full_candidate(params, &mut local_seed));
    }
    unique_ranked_full_candidates(candidates, score_fn, limit, deadline)
}

pub(in crate::search) fn hill_climb_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    config: &HillClimbSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = config.seed;
    let mut candidates = Vec::new();
    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_full_candidate(params, &mut local_seed);
        let mut current_score = score_fn(&current);
        candidates.push(current.clone());

        for _ in 0..config.steps {
            if deadline_reached(deadline) {
                break;
            }
            let mut neighbor_candidates = Vec::new();
            for _ in 0..config.neighbors_per_step.max(1) {
                let mut neighbor = current.clone();
                mutate_full_candidate(params, &mut neighbor, 1.0, &mut local_seed);
                neighbor_candidates.push(neighbor);
            }
            if neighbor_candidates.is_empty() {
                break;
            }
            let ranked_neighbors = unique_ranked_full_candidates(
                neighbor_candidates,
                score_fn,
                config.neighbors_per_step.max(1),
                deadline,
            );
            let Some((best_neighbor, best_score)) = ranked_neighbors.first().cloned() else {
                break;
            };
            if best_score > current_score {
                current = best_neighbor;
                current_score = best_score;
                candidates.push(current.clone());
            } else {
                break;
            }
        }
    }
    unique_ranked_full_candidates(candidates, score_fn, config.limit, deadline)
}

fn tournament_parent_full(
    scored_population: &[(BuildKey, f64)],
    seed: &mut u64,
    tournament_size: usize,
) -> BuildKey {
    let mut best_idx = rand_index(seed, scored_population.len());
    for _ in 1..tournament_size.max(1) {
        let idx = rand_index(seed, scored_population.len());
        if scored_population[idx].1 > scored_population[best_idx].1 {
            best_idx = idx;
        }
    }
    scored_population[best_idx].0.clone()
}

pub(in crate::search) fn genetic_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    config: &GeneticSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let population_size = config.population_size.max(8);
    let mut local_seed = config.seed;
    let mut population = Vec::with_capacity(population_size);
    for _ in 0..population_size {
        if deadline_reached(deadline) {
            break;
        }
        population.push(random_full_candidate(params, &mut local_seed));
    }

    let mut all_seen = population.clone();
    for _ in 0..config.generations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut scored =
            unique_ranked_full_candidates(population.clone(), score_fn, population_size, deadline);
        if scored.is_empty() {
            break;
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        let elite_count = (population_size / 8).max(1).min(scored.len());
        let mut next_population = scored
            .iter()
            .take(elite_count)
            .map(|(candidate, _)| candidate.clone())
            .collect::<Vec<_>>();

        while next_population.len() < population_size {
            if deadline_reached(deadline) {
                break;
            }
            let parent_a = tournament_parent_full(&scored, &mut local_seed, 3);
            let parent_b = tournament_parent_full(&scored, &mut local_seed, 3);
            let mut child = if rand_f64(&mut local_seed) <= config.crossover_rate.clamp(0.0, 1.0) {
                crossover_full_candidates(&parent_a, &parent_b, params, &mut local_seed)
            } else {
                parent_a
            };
            mutate_full_candidate(params, &mut child, config.mutation_rate, &mut local_seed);
            repair_full_candidate(params, &mut child, &mut local_seed);
            next_population.push(child);
        }

        all_seen.extend(next_population.clone());
        population = next_population;
    }

    unique_ranked_full_candidates(all_seen, score_fn, config.limit, deadline)
}

pub(in crate::search) fn simulated_annealing_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    config: &SimulatedAnnealingSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = config.seed;
    let mut candidates = Vec::new();

    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_full_candidate(params, &mut local_seed);
        let mut current_score = score_fn(&current);
        let mut best = current.clone();
        let mut best_score = current_score;
        let mut temp = config.initial_temp.max(0.0001);
        candidates.push(current.clone());

        for _ in 0..config.iterations.max(1) {
            if deadline_reached(deadline) {
                break;
            }
            let mut next = current.clone();
            mutate_full_candidate(params, &mut next, 1.0, &mut local_seed);
            let next_score = score_fn(&next);
            let delta = next_score - current_score;
            let accept = delta >= 0.0 || rand_f64(&mut local_seed) < (delta / temp).exp();
            if accept {
                current = next;
                current_score = next_score;
                candidates.push(current.clone());
                if current_score > best_score {
                    best_score = current_score;
                    best = current.clone();
                }
            }
            temp = (temp * config.cooling_rate.clamp(0.8, 0.9999)).max(0.0001);
        }
        candidates.push(best);
    }

    unique_ranked_full_candidates(candidates, score_fn, config.limit, deadline)
}

#[derive(Debug, Clone)]
struct MctsFullNode {
    candidate: BuildKey,
    parent: Option<usize>,
    children: Vec<usize>,
    visits: usize,
    value_sum: f64,
}

pub(in crate::search) fn mcts_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    config: &MctsSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = config.seed;
    let root = BuildKey {
        item_indices: Vec::new(),
        loadout_selection: params.base_loadout.clone(),
    };
    let mut nodes = vec![MctsFullNode {
        candidate: root,
        parent: None,
        children: Vec::new(),
        visits: 0,
        value_sum: 0.0,
    }];
    let mut seen = HashSet::<BuildKey>::new();
    let mut rollout_keys = Vec::new();

    for _ in 0..config.iterations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut node_idx = 0usize;
        while !nodes[node_idx].children.is_empty() {
            let parent_visits = nodes[node_idx].visits.max(1) as f64;
            let mut best_child = nodes[node_idx].children[0];
            let mut best_uct = f64::NEG_INFINITY;
            for &child_idx in &nodes[node_idx].children {
                let child = &nodes[child_idx];
                let exploit = if child.visits == 0 {
                    0.0
                } else {
                    child.value_sum / child.visits as f64
                };
                let explore = config.exploration
                    * ((parent_visits.ln() / (child.visits.max(1) as f64)).sqrt());
                let uct = exploit + explore;
                if uct > best_uct {
                    best_uct = uct;
                    best_child = child_idx;
                }
            }
            node_idx = best_child;
        }

        let mut expanded = nodes[node_idx].candidate.clone();
        if expanded.item_indices.len() < params.max_items && rand_f64(&mut local_seed) < 0.6 {
            let has_boots = expanded
                .item_indices
                .iter()
                .any(|&idx| is_boots(&params.item_pool[idx]));
            let mut options = (0..params.item_pool.len())
                .filter(|idx| {
                    !(expanded.item_indices.contains(idx)
                        || has_boots && is_boots(&params.item_pool[*idx]))
                })
                .collect::<Vec<_>>();
            if !options.is_empty() {
                let pick = options.swap_remove(rand_index(&mut local_seed, options.len()));
                expanded.item_indices.push(pick);
            }
        } else {
            expanded.loadout_selection = random_loadout_selection(
                &expanded.loadout_selection,
                params.loadout_domain,
                &mut local_seed,
            );
        }
        repair_full_candidate(params, &mut expanded, &mut local_seed);

        if seen.insert(expanded.clone()) {
            let child_idx = nodes.len();
            nodes.push(MctsFullNode {
                candidate: expanded.clone(),
                parent: Some(node_idx),
                children: Vec::new(),
                visits: 0,
                value_sum: 0.0,
            });
            nodes[node_idx].children.push(child_idx);
            node_idx = child_idx;
        }

        let rollouts = config.rollouts_per_expansion.max(1);
        let mut scores = Vec::new();
        for _ in 0..rollouts {
            if deadline_reached(deadline) {
                break;
            }
            let mut rollout = nodes[node_idx].candidate.clone();
            mutate_full_candidate(params, &mut rollout, 1.0, &mut local_seed);
            let score = score_fn(&rollout);
            scores.push(score);
            rollout_keys.push(rollout);
        }
        if scores.is_empty() {
            break;
        }
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let mut back = Some(node_idx);
        while let Some(idx) = back {
            nodes[idx].visits += 1;
            nodes[idx].value_sum += mean;
            back = nodes[idx].parent;
        }
    }

    unique_ranked_full_candidates(rollout_keys, score_fn, config.limit, deadline)
}

#[cfg(test)]
#[path = "tests/full_loadout_search_strategies_tests.rs"]
mod full_loadout_search_strategies_tests;

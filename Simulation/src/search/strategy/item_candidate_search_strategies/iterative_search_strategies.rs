use std::cmp::Ordering;
use std::time::Instant;

use crate::random_valid_build;
use crate::status::deadline_reached;

use super::super::super::candidate_space::item_candidate_operations::{
    crossover_builds, mutate_build, tournament_parent,
};
use super::super::super::candidate_space::item_candidate_scoring::unique_ranked_from_candidates;
use super::super::super::{
    GeneticSearchConfig, HillClimbSearchConfig, Item, SimulatedAnnealingSearchConfig,
    can_add_item_to_build, canonical_key, rand_f64, rand_index, repair_build,
};

pub(super) fn random_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    random_samples: usize,
    seed: u64,
    limit: usize,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = seed;
    let mut candidates = Vec::with_capacity(random_samples.max(1));
    for _ in 0..random_samples.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        candidates.push(random_valid_build(item_pool, max_items, &mut s));
    }
    unique_ranked_from_candidates(candidates, score_fn, limit, deadline)
}

pub(super) fn hill_climb_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &HillClimbSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = config.seed;
    let mut candidates = Vec::new();

    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_valid_build(item_pool, max_items, &mut s);
        let mut current_score = score_fn(&canonical_key(&current));
        candidates.push(current.clone());

        for _ in 0..config.steps {
            if deadline_reached(deadline) {
                break;
            }
            let mut neighbor_builds = Vec::new();
            for _ in 0..config.neighbors_per_step.max(1) {
                if current.is_empty() {
                    break;
                }
                let mut neighbor = current.clone();
                let swap_idx = rand_index(&mut s, neighbor.len());
                let mut proposed = rand_index(&mut s, item_pool.len());
                let mut tries = 0usize;
                while tries < item_pool.len()
                    && (!can_add_item_to_build(item_pool, &neighbor, proposed)
                        || proposed == neighbor[swap_idx])
                {
                    proposed = rand_index(&mut s, item_pool.len());
                    tries += 1;
                }
                if tries < item_pool.len() {
                    neighbor[swap_idx] = proposed;
                    repair_build(item_pool, &mut neighbor, max_items, &mut s);
                    neighbor_builds.push(neighbor);
                }
            }
            if neighbor_builds.is_empty() {
                break;
            }
            let ranked_neighbors = unique_ranked_from_candidates(
                neighbor_builds,
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

    unique_ranked_from_candidates(candidates, score_fn, config.limit, deadline)
}

pub(super) fn genetic_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &GeneticSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let pop_n = config.population_size.max(8);
    let mut s = config.seed;
    let mut population = Vec::with_capacity(pop_n);
    for _ in 0..pop_n {
        if deadline_reached(deadline) {
            break;
        }
        population.push(random_valid_build(item_pool, max_items, &mut s));
    }

    let mut all_seen = population.clone();
    for _ in 0..config.generations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut scored =
            unique_ranked_from_candidates(population.clone(), score_fn, pop_n, deadline);
        if scored.is_empty() {
            break;
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        let elite_count = (pop_n / 8).max(1).min(scored.len());
        let mut next_population = scored
            .iter()
            .take(elite_count)
            .map(|(b, _)| b.clone())
            .collect::<Vec<_>>();

        while next_population.len() < pop_n {
            if deadline_reached(deadline) {
                break;
            }
            let parent_a = tournament_parent(&scored, &mut s, 3);
            let parent_b = tournament_parent(&scored, &mut s, 3);
            let mut child = if rand_f64(&mut s) <= config.crossover_rate.clamp(0.0, 1.0) {
                crossover_builds(&parent_a, &parent_b, item_pool, max_items, &mut s)
            } else {
                parent_a
            };
            mutate_build(
                &mut child,
                item_pool,
                max_items,
                config.mutation_rate,
                &mut s,
            );
            repair_build(item_pool, &mut child, max_items, &mut s);
            next_population.push(child);
        }

        all_seen.extend(next_population.clone());
        population = next_population;
    }

    unique_ranked_from_candidates(all_seen, score_fn, config.limit, deadline)
}

pub(super) fn simulated_annealing_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &SimulatedAnnealingSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = config.seed;
    let mut candidates = Vec::new();

    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_valid_build(item_pool, max_items, &mut s);
        let mut current_score = score_fn(&canonical_key(&current));
        let mut best = current.clone();
        let mut best_score = current_score;
        let mut temp = config.initial_temp.max(0.0001);
        candidates.push(current.clone());

        for _ in 0..config.iterations.max(1) {
            if deadline_reached(deadline) {
                break;
            }
            let mut next = current.clone();
            if !next.is_empty() {
                let slot = rand_index(&mut s, next.len());
                let candidate = rand_index(&mut s, item_pool.len());
                next[slot] = candidate;
                repair_build(item_pool, &mut next, max_items, &mut s);
                let next_key = canonical_key(&next);
                let next_score = score_fn(&next_key);
                let delta = next_score - current_score;
                let accept = delta >= 0.0 || rand_f64(&mut s) < (delta / temp).exp();
                if accept {
                    current = next;
                    current_score = next_score;
                    candidates.push(current.clone());
                    if current_score > best_score {
                        best_score = current_score;
                        best = current.clone();
                    }
                }
            }
            temp = (temp * config.cooling_rate.clamp(0.8, 0.9999)).max(0.0001);
        }
        candidates.push(best);
    }

    unique_ranked_from_candidates(candidates, score_fn, config.limit, deadline)
}

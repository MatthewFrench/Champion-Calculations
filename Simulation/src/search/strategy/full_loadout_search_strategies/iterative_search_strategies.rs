use std::cmp::Ordering;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::super::candidate_space::full_loadout_candidate_operations::{
    crossover_full_candidates, mutate_full_candidate, random_full_candidate, repair_full_candidate,
};
use super::super::super::candidate_space::full_loadout_candidate_scoring::unique_ranked_full_candidates;
use super::super::super::{
    BuildKey, FullLoadoutSearchParams, GeneticSearchConfig, HillClimbSearchConfig,
    SimulatedAnnealingSearchConfig, rand_f64,
};

pub(super) fn random_search_ranked_full<F>(
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

pub(super) fn hill_climb_search_ranked_full<F>(
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
    let mut best_idx = super::super::super::rand_index(seed, scored_population.len());
    for _ in 1..tournament_size.max(1) {
        let idx = super::super::super::rand_index(seed, scored_population.len());
        if scored_population[idx].1 > scored_population[best_idx].1 {
            best_idx = idx;
        }
    }
    scored_population[best_idx].0.clone()
}

pub(super) fn genetic_search_ranked_full<F>(
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
            let mut child = if rand_f64(&mut local_seed) <= config.crossover_rate.clamp(0.0, 1.0)
            {
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

pub(super) fn simulated_annealing_search_ranked_full<F>(
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

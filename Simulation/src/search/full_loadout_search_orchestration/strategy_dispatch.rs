use rayon::prelude::*;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::candidate_space::full_loadout_candidate_operations::candidate_loadout_variants;
use super::super::candidate_space::full_loadout_candidate_scoring::unique_ranked_full_candidates;
use super::super::strategy::full_loadout_search_strategies::{
    beam_search_ranked_full, genetic_search_ranked_full, hill_climb_search_ranked_full,
    mcts_search_ranked_full, random_search_ranked_full, simulated_annealing_search_ranked_full,
};
use super::super::{
    BuildKey, BuildSearchConfig, GeneticSearchConfig, HillClimbSearchConfig, MctsSearchConfig,
    SimulatedAnnealingSearchConfig, can_add_item_to_build, canonical_build_candidate,
    canonical_key,
};
use super::FullLoadoutSearchParams;

fn greedy_search_ranked_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut seed = search.seed;
    let mut candidate = BuildKey {
        item_indices: Vec::new(),
        loadout_selection: params.base_loadout.clone(),
    };
    for _ in 0..params.max_items {
        if deadline_reached(deadline) {
            break;
        }
        let mut best: Option<BuildKey> = None;
        let mut best_score = f64::NEG_INFINITY;
        for item_idx in 0..params.item_pool.len() {
            if !can_add_item_to_build(params.item_pool, &candidate.item_indices, item_idx) {
                continue;
            }
            let mut next = candidate.clone();
            next.item_indices.push(item_idx);
            next.item_indices = canonical_key(&next.item_indices);
            let loadout_variants =
                candidate_loadout_variants(&next.loadout_selection, params, &mut seed, 4);
            for loadout_selection in loadout_variants {
                let mut probe = next.clone();
                probe.loadout_selection = loadout_selection;
                probe = canonical_build_candidate(probe);
                let score = score_fn(&probe);
                if score > best_score {
                    best_score = score;
                    best = Some(probe);
                }
            }
        }
        if let Some(next) = best {
            candidate = next;
        } else {
            break;
        }
    }
    vec![(candidate.clone(), score_fn(&candidate))]
}

fn portfolio_search_ranked_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let strategies = super::super::portfolio_strategy_list(search);
    let mut ranked_sets = strategies
        .par_iter()
        .enumerate()
        .map(|(idx, strategy)| {
            if deadline_reached(deadline) {
                return (idx, Vec::new());
            }
            let mut cfg = search.clone();
            cfg.strategy = strategy.clone();
            cfg.seed = search.seed.wrapping_add((idx as u64 + 1) * 1_000_003);
            (
                idx,
                build_search_ranked_full_loadout(params, &cfg, score_fn, deadline),
            )
        })
        .collect::<Vec<_>>();
    ranked_sets.sort_by_key(|(idx, _)| *idx);
    let merged = ranked_sets
        .into_iter()
        .flat_map(|(_, ranked)| ranked.into_iter().map(|(candidate, _)| candidate))
        .collect::<Vec<_>>();
    unique_ranked_full_candidates(merged, score_fn, search.ranked_limit, deadline)
}

pub(super) fn build_search_ranked_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    if deadline_reached(deadline) {
        return Vec::new();
    }

    match search.strategy.as_str() {
        "greedy" => greedy_search_ranked_full_loadout(params, search, score_fn, deadline),
        "beam" => {
            beam_search_ranked_full(params, search.beam_width, search.seed, score_fn, deadline)
        }
        "random" => random_search_ranked_full(
            params,
            search.random_samples,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "hill_climb" => hill_climb_search_ranked_full(
            params,
            &HillClimbSearchConfig {
                restarts: search.hill_climb_restarts,
                steps: search.hill_climb_steps,
                neighbors_per_step: search.hill_climb_neighbors,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "genetic" => genetic_search_ranked_full(
            params,
            &GeneticSearchConfig {
                population_size: search.genetic_population,
                generations: search.genetic_generations,
                mutation_rate: search.genetic_mutation_rate,
                crossover_rate: search.genetic_crossover_rate,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "simulated_annealing" => simulated_annealing_search_ranked_full(
            params,
            &SimulatedAnnealingSearchConfig {
                restarts: search.simulated_annealing_restarts,
                iterations: search.simulated_annealing_iterations,
                initial_temp: search.simulated_annealing_initial_temp,
                cooling_rate: search.simulated_annealing_cooling_rate,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "mcts" => mcts_search_ranked_full(
            params,
            &MctsSearchConfig {
                iterations: search.mcts_iterations,
                rollouts_per_expansion: search.mcts_rollouts_per_expansion,
                exploration: search.mcts_exploration,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "portfolio" => portfolio_search_ranked_full_loadout(params, search, score_fn, deadline),
        _ => Vec::new(),
    }
}

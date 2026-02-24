use std::time::Instant;

use super::super::{
    BuildKey, FullLoadoutSearchParams, GeneticSearchConfig, HillClimbSearchConfig,
    MctsSearchConfig, SimulatedAnnealingSearchConfig,
};

mod beam_search_strategy;
mod iterative_search_strategies;
mod mcts_search_strategy;

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
    beam_search_strategy::beam_search_ranked_full(params, beam_width, seed, score_fn, deadline)
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
    iterative_search_strategies::random_search_ranked_full(
        params,
        random_samples,
        seed,
        limit,
        score_fn,
        deadline,
    )
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
    iterative_search_strategies::hill_climb_search_ranked_full(params, config, score_fn, deadline)
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
    iterative_search_strategies::genetic_search_ranked_full(params, config, score_fn, deadline)
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
    iterative_search_strategies::simulated_annealing_search_ranked_full(
        params, config, score_fn, deadline,
    )
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
    mcts_search_strategy::mcts_search_ranked_full(params, config, score_fn, deadline)
}

#[cfg(test)]
#[path = "tests/full_loadout_search_strategies_tests.rs"]
mod full_loadout_search_strategies_tests;

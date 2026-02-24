use std::time::Instant;

use super::super::{
    GeneticSearchConfig, HillClimbSearchConfig, Item, MctsSearchConfig,
    SimulatedAnnealingSearchConfig,
};
mod beam_search_strategy;
mod iterative_search_strategies;
mod mcts_search_strategy;

#[cfg(test)]
use self::mcts_search_strategy::{available_actions, rollout_completion};

pub(in crate::search) fn beam_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    beam_width: usize,
    score_fn: F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    beam_search_strategy::beam_search_ranked(item_pool, max_items, beam_width, score_fn, deadline)
}

pub(in crate::search) fn random_search_ranked<F>(
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
    iterative_search_strategies::random_search_ranked(
        item_pool,
        max_items,
        random_samples,
        seed,
        limit,
        score_fn,
        deadline,
    )
}

pub(in crate::search) fn hill_climb_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &HillClimbSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    iterative_search_strategies::hill_climb_search_ranked(
        item_pool, max_items, config, score_fn, deadline,
    )
}

pub(in crate::search) fn genetic_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &GeneticSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    iterative_search_strategies::genetic_search_ranked(
        item_pool, max_items, config, score_fn, deadline,
    )
}

pub(in crate::search) fn simulated_annealing_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &SimulatedAnnealingSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    iterative_search_strategies::simulated_annealing_search_ranked(
        item_pool, max_items, config, score_fn, deadline,
    )
}

pub(in crate::search) fn mcts_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &MctsSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    mcts_search_strategy::mcts_search_ranked(item_pool, max_items, config, score_fn, deadline)
}

#[cfg(test)]
#[path = "tests/item_candidate_search_strategies_tests.rs"]
mod item_candidate_search_strategies_tests;

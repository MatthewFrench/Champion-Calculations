use std::collections::HashMap;
use std::time::Instant;

use super::{BuildKey, BuildSearchConfig, Item, LoadoutSelection};
use crate::data::LoadoutDomain;

mod adaptive_candidate_generation;
mod bleed_candidate_generation;
mod seed_elite_generation;
mod strategy_dispatch;

#[derive(Debug, Clone, Copy)]
pub(crate) struct FullLoadoutSearchParams<'a> {
    pub item_pool: &'a [Item],
    pub max_items: usize,
    pub loadout_domain: &'a LoadoutDomain,
    pub base_loadout: &'a LoadoutSelection,
}

pub(crate) fn build_search_ranked_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    strategy_dispatch::build_search_ranked_full_loadout(params, search, score_fn, deadline)
}

pub(crate) fn strategy_seed_elites_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategies: &[String],
    score_fn: &F,
    deadline: Option<Instant>,
) -> HashMap<String, Vec<BuildKey>>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    seed_elite_generation::strategy_seed_elites_full_loadout(
        params, search, strategies, score_fn, deadline,
    )
}

pub(crate) fn adaptive_strategy_candidates_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<BuildKey>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<BuildKey>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    adaptive_candidate_generation::adaptive_strategy_candidates_full_loadout(
        params,
        search,
        strategy_elites,
        score_fn,
        deadline,
    )
}

pub(crate) fn generate_bleed_candidates_full_loadout(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<BuildKey>>,
) -> Vec<BuildKey> {
    bleed_candidate_generation::generate_bleed_candidates_full_loadout(
        params,
        search,
        strategy_elites,
    )
}

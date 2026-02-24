use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::candidate_space::full_loadout_candidate_operations::candidate_order_key;
use super::super::{BuildKey, BuildSearchConfig};
use super::FullLoadoutSearchParams;
use super::strategy_dispatch::build_search_ranked_full_loadout;

pub(super) fn adaptive_strategy_candidates_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<BuildKey>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<BuildKey>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    if strategy_elites.is_empty() {
        return Vec::new();
    }

    let mut strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    strategies.sort_unstable();

    let contributions = strategies
        .iter()
        .map(|strategy| {
            let contribution = strategy_elites
                .get(strategy)
                .map(|candidates| candidates.len().max(1) as f64)
                .unwrap_or(1.0);
            (strategy.clone(), contribution)
        })
        .collect::<Vec<_>>();

    let total_contribution = contributions
        .iter()
        .map(|(_, contribution)| *contribution)
        .sum::<f64>()
        .max(1.0);
    let extra_runs_total = (search.ensemble_seeds.max(1) * strategies.len()).max(8);
    let per_strategy = contributions
        .into_iter()
        .map(|(strategy, contribution)| {
            let share = contribution / total_contribution;
            let runs = ((extra_runs_total as f64) * share).round() as usize;
            (strategy, runs.max(1))
        })
        .collect::<Vec<_>>();

    let mut out = HashSet::<BuildKey>::new();
    let gathered = per_strategy
        .par_iter()
        .enumerate()
        .map(|(strategy_idx, (strategy, runs))| {
            (0..*runs)
                .into_par_iter()
                .flat_map_iter(|run_idx| {
                    if deadline_reached(deadline) {
                        return Vec::<BuildKey>::new().into_iter();
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strategy.clone();
                    cfg.seed = search.seed.wrapping_add(
                        ((strategy_idx as u64 + 1) * 131 + run_idx as u64 + 1)
                            * search.ensemble_seed_stride,
                    );
                    cfg.ranked_limit = (search.ensemble_seed_top_k.max(1) * 2).max(50);
                    let ranked = build_search_ranked_full_loadout(params, &cfg, score_fn, deadline);
                    ranked
                        .into_iter()
                        .take(search.ensemble_seed_top_k.max(1))
                        .map(|(candidate, _)| candidate)
                        .collect::<Vec<_>>()
                        .into_iter()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for candidates in gathered {
        for candidate in candidates {
            out.insert(candidate);
        }
    }

    let mut out_vec = out.into_iter().collect::<Vec<_>>();
    out_vec.sort_by_key(candidate_order_key);
    out_vec
}

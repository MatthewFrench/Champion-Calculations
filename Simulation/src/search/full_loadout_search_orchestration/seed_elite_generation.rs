use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::{BuildKey, BuildSearchConfig};
use super::FullLoadoutSearchParams;
use super::strategy_dispatch::build_search_ranked_full_loadout;

pub(super) fn strategy_seed_elites_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategies: &[String],
    score_fn: &F,
    deadline: Option<Instant>,
) -> HashMap<String, Vec<BuildKey>>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let ensemble = search.ensemble_seeds.max(1);
    let top_k = search.ensemble_seed_top_k.max(1);

    let mut grouped = strategies
        .par_iter()
        .enumerate()
        .map(|(strategy_index, strategy)| {
            let mut aggregate = HashMap::<BuildKey, f64>::new();
            let seed_ranked = (0..ensemble)
                .into_par_iter()
                .map(|seed_idx| {
                    if deadline_reached(deadline) {
                        return Vec::new();
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strategy.clone();
                    cfg.seed = search.seed.wrapping_add(
                        ((strategy_index as u64 + 1) * 31 + seed_idx as u64 + 1)
                            * search.ensemble_seed_stride,
                    );
                    cfg.ranked_limit = top_k.max(64);
                    build_search_ranked_full_loadout(params, &cfg, score_fn, deadline)
                })
                .collect::<Vec<_>>();

            for ranked in seed_ranked {
                for (candidate, score) in ranked.into_iter().take(top_k) {
                    let entry = aggregate.entry(candidate).or_insert(score);
                    if score > *entry {
                        *entry = score;
                    }
                }
            }

            let mut entries = aggregate.into_iter().collect::<Vec<_>>();
            entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            let elites = entries
                .into_iter()
                .map(|(candidate, _)| candidate)
                .collect::<Vec<_>>();
            (strategy_index, strategy.clone(), elites)
        })
        .collect::<Vec<_>>();

    grouped.sort_by_key(|(idx, _, _)| *idx);
    grouped
        .into_iter()
        .map(|(_, strategy, elites)| (strategy, elites))
        .collect::<HashMap<_, _>>()
}

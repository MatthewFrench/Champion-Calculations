use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::AtomicSearchTypeRuntimeCounter;
use super::*;

pub(super) fn initialize_search_type_counters(
    active_strategies: &[String],
    configured_strategy: &str,
) -> Arc<HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>> {
    let mut keys = vec![
        "coverage_stage".to_string(),
        "strategy_elites".to_string(),
        "adaptive_search".to_string(),
        "strict_full_ranking".to_string(),
        format!("seed_search:{}", configured_strategy),
    ];
    for strategy in active_strategies {
        keys.push(format!("seed_search:{strategy}"));
    }
    keys.sort();
    keys.dedup();

    Arc::new(
        keys.into_iter()
            .map(|key| (key, Arc::new(AtomicSearchTypeRuntimeCounter::default())))
            .collect::<HashMap<_, _>>(),
    )
}

pub(super) fn increment_search_type_counter(
    counters: &HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>,
    search_type: &str,
    score_requests: usize,
    new_simulations: usize,
) {
    if let Some(counter) = counters.get(search_type) {
        counter.add(score_requests, new_simulations);
    }
}

pub(super) fn snapshot_search_type_counters(
    counters: &HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>,
) -> Vec<SearchTypeBreakdown> {
    counters
        .iter()
        .filter_map(|(name, counter)| {
            let snapshot = counter.snapshot();
            let touched = snapshot.score_requests > 0 || snapshot.new_simulations > 0;
            touched.then(|| SearchTypeBreakdown {
                name: name.clone(),
                score_requests: snapshot.score_requests,
                new_simulations: snapshot.new_simulations,
            })
        })
        .collect::<Vec<_>>()
}

pub(super) fn unique_loadout_selection_count(candidates: &[BuildKey]) -> usize {
    candidates
        .iter()
        .map(|candidate| loadout_selection_key(&candidate.loadout_selection))
        .collect::<HashSet<_>>()
        .len()
}

pub(super) fn unique_loadout_selection_count_from_ranked(ranked: &[(BuildKey, f64)]) -> usize {
    ranked
        .iter()
        .map(|(candidate, _)| loadout_selection_key(&candidate.loadout_selection))
        .collect::<HashSet<_>>()
        .len()
}

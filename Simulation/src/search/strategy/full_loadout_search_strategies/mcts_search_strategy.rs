use std::collections::HashSet;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::super::candidate_space::full_loadout_candidate_operations::{
    mutate_full_candidate, repair_full_candidate,
};
use super::super::super::candidate_space::full_loadout_candidate_scoring::unique_ranked_full_candidates;
use super::super::super::{
    BuildKey, FullLoadoutSearchParams, MctsSearchConfig, is_boots, rand_f64, rand_index,
    random_loadout_selection,
};

#[derive(Debug, Clone)]
struct MctsFullNode {
    candidate: BuildKey,
    parent: Option<usize>,
    children: Vec<usize>,
    visits: usize,
    value_sum: f64,
}

pub(super) fn mcts_search_ranked_full<F>(
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

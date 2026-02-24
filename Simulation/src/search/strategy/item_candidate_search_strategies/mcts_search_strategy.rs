use std::time::Instant;

use crate::shuffle_usize;
use crate::status::deadline_reached;

use super::super::super::candidate_space::item_candidate_scoring::unique_ranked_from_candidates;
use super::super::super::{
    Item, MctsSearchConfig, can_add_item_to_build, canonical_key, rand_index, repair_build,
};

#[derive(Debug, Clone)]
struct MctsNode {
    build: Vec<usize>,
    parent: Option<usize>,
    action_from_parent: Option<usize>,
    children: Vec<usize>,
    untried_actions: Vec<usize>,
    visits: usize,
    value_sum: f64,
}

pub(super) fn available_actions(item_pool: &[Item], build: &[usize]) -> Vec<usize> {
    (0..item_pool.len())
        .filter(|&idx| can_add_item_to_build(item_pool, build, idx))
        .collect()
}

pub(super) fn rollout_completion<F>(
    item_pool: &[Item],
    max_items: usize,
    start_build: &[usize],
    seed: &mut u64,
    score_fn: &F,
    deadline: Option<Instant>,
) -> (Vec<usize>, f64)
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut build = start_build.to_vec();
    let mut actions = available_actions(item_pool, &build);
    shuffle_usize(&mut actions, seed);
    for action in actions {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &build, action) {
            build.push(action);
        }
    }
    repair_build(item_pool, &mut build, max_items, seed);
    let key = canonical_key(&build);
    let score = if deadline_reached(deadline) {
        f64::NEG_INFINITY
    } else {
        score_fn(&key)
    };
    (key, score)
}

pub(super) fn mcts_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    config: &MctsSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = config.seed;
    let mut nodes = vec![MctsNode {
        build: vec![],
        parent: None,
        action_from_parent: None,
        children: vec![],
        untried_actions: available_actions(item_pool, &[]),
        visits: 0,
        value_sum: 0.0,
    }];
    let mut all_rollout_keys = Vec::new();

    for _ in 0..config.iterations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut node_idx = 0usize;
        loop {
            if nodes[node_idx].build.len() >= max_items {
                break;
            }
            if !nodes[node_idx].untried_actions.is_empty() {
                break;
            }
            if nodes[node_idx].children.is_empty() {
                break;
            }
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

        if !nodes[node_idx].untried_actions.is_empty() && nodes[node_idx].build.len() < max_items {
            let action_pos = rand_index(&mut s, nodes[node_idx].untried_actions.len());
            let action = nodes[node_idx].untried_actions.swap_remove(action_pos);
            let mut child_build = nodes[node_idx].build.clone();
            child_build.push(action);
            repair_build(item_pool, &mut child_build, max_items, &mut s);
            let child_idx = nodes.len();
            nodes.push(MctsNode {
                build: child_build.clone(),
                parent: Some(node_idx),
                action_from_parent: Some(action),
                children: vec![],
                untried_actions: available_actions(item_pool, &child_build),
                visits: 0,
                value_sum: 0.0,
            });
            nodes[node_idx].children.push(child_idx);
            node_idx = child_idx;
        }

        let mut rollout_scores = Vec::new();
        let rollouts = config.rollouts_per_expansion.max(1);
        for _ in 0..rollouts {
            if deadline_reached(deadline) {
                break;
            }
            let (key, score) = rollout_completion(
                item_pool,
                max_items,
                &nodes[node_idx].build,
                &mut s,
                score_fn,
                deadline,
            );
            all_rollout_keys.push(key);
            rollout_scores.push(score);
        }
        if rollout_scores.is_empty() {
            break;
        }
        let mean_score = rollout_scores.iter().sum::<f64>() / rollout_scores.len() as f64;

        let mut back = Some(node_idx);
        while let Some(idx) = back {
            nodes[idx].visits += 1;
            nodes[idx].value_sum += mean_score;
            back = nodes[idx].parent;
        }
    }

    let _used_actions = nodes
        .iter()
        .filter_map(|node| node.action_from_parent)
        .count();
    unique_ranked_from_candidates(all_rollout_keys, score_fn, config.limit, deadline)
}

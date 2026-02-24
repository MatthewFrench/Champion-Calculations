use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::candidate_space::item_candidate_operations::{
    crossover_builds, mutate_build, tournament_parent,
};
use super::super::candidate_space::item_candidate_scoring::{
    score_candidates, unique_ranked_from_candidates,
};
use super::super::{
    GeneticSearchConfig, HillClimbSearchConfig, Item, MctsSearchConfig,
    SimulatedAnnealingSearchConfig, can_add_item_to_build, canonical_key, is_boots, rand_f64,
    rand_index, repair_build,
};
use crate::{random_valid_build, shuffle_usize};

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
    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    let mut final_scored: Vec<(Vec<usize>, Vec<usize>, f64)> = vec![];

    for _ in 0..max_items {
        if deadline_reached(deadline) {
            break;
        }
        let mut next_candidates = Vec::new();
        for build in &candidates {
            let has_boots = build.iter().any(|&i| is_boots(&item_pool[i]));
            let used = build.iter().copied().collect::<HashSet<_>>();
            for (item_idx, item) in item_pool.iter().enumerate() {
                if used.contains(&item_idx) {
                    continue;
                }
                if is_boots(item) && has_boots {
                    continue;
                }
                let mut next = build.clone();
                next.push(item_idx);
                next_candidates.push(next);
            }
        }

        let scored = score_candidates(next_candidates, &score_fn, deadline);
        candidates = scored
            .iter()
            .take(beam_width)
            .map(|(candidate, _, _)| candidate.clone())
            .collect();
        final_scored = scored;
    }

    let mut ranked = Vec::new();
    let mut seen = HashSet::new();
    for (_, key, score) in final_scored {
        if seen.insert(key.clone()) {
            ranked.push((key, score));
        }
    }
    ranked
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
    let mut s = seed;
    let mut candidates = Vec::with_capacity(random_samples.max(1));
    for _ in 0..random_samples.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        candidates.push(random_valid_build(item_pool, max_items, &mut s));
    }
    unique_ranked_from_candidates(candidates, score_fn, limit, deadline)
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
    let mut s = config.seed;
    let mut candidates = Vec::new();

    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_valid_build(item_pool, max_items, &mut s);
        let mut current_score = score_fn(&canonical_key(&current));
        candidates.push(current.clone());

        for _ in 0..config.steps {
            if deadline_reached(deadline) {
                break;
            }
            let mut neighbor_builds = Vec::new();
            for _ in 0..config.neighbors_per_step.max(1) {
                if current.is_empty() {
                    break;
                }
                let mut neighbor = current.clone();
                let swap_idx = rand_index(&mut s, neighbor.len());
                let mut proposed = rand_index(&mut s, item_pool.len());
                let mut tries = 0usize;
                while tries < item_pool.len()
                    && (!can_add_item_to_build(item_pool, &neighbor, proposed)
                        || proposed == neighbor[swap_idx])
                {
                    proposed = rand_index(&mut s, item_pool.len());
                    tries += 1;
                }
                if tries < item_pool.len() {
                    neighbor[swap_idx] = proposed;
                    repair_build(item_pool, &mut neighbor, max_items, &mut s);
                    neighbor_builds.push(neighbor);
                }
            }
            if neighbor_builds.is_empty() {
                break;
            }
            let ranked_neighbors = unique_ranked_from_candidates(
                neighbor_builds,
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

    unique_ranked_from_candidates(candidates, score_fn, config.limit, deadline)
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
    let pop_n = config.population_size.max(8);
    let mut s = config.seed;
    let mut population = Vec::with_capacity(pop_n);
    for _ in 0..pop_n {
        if deadline_reached(deadline) {
            break;
        }
        population.push(random_valid_build(item_pool, max_items, &mut s));
    }

    let mut all_seen = population.clone();
    for _ in 0..config.generations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut scored =
            unique_ranked_from_candidates(population.clone(), score_fn, pop_n, deadline);
        if scored.is_empty() {
            break;
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        let elite_count = (pop_n / 8).max(1).min(scored.len());
        let mut next_population = scored
            .iter()
            .take(elite_count)
            .map(|(b, _)| b.clone())
            .collect::<Vec<_>>();

        while next_population.len() < pop_n {
            if deadline_reached(deadline) {
                break;
            }
            let parent_a = tournament_parent(&scored, &mut s, 3);
            let parent_b = tournament_parent(&scored, &mut s, 3);
            let mut child = if rand_f64(&mut s) <= config.crossover_rate.clamp(0.0, 1.0) {
                crossover_builds(&parent_a, &parent_b, item_pool, max_items, &mut s)
            } else {
                parent_a
            };
            mutate_build(
                &mut child,
                item_pool,
                max_items,
                config.mutation_rate,
                &mut s,
            );
            repair_build(item_pool, &mut child, max_items, &mut s);
            next_population.push(child);
        }

        all_seen.extend(next_population.clone());
        population = next_population;
    }

    unique_ranked_from_candidates(all_seen, score_fn, config.limit, deadline)
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
    let mut s = config.seed;
    let mut candidates = Vec::new();

    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_valid_build(item_pool, max_items, &mut s);
        let mut current_score = score_fn(&canonical_key(&current));
        let mut best = current.clone();
        let mut best_score = current_score;
        let mut temp = config.initial_temp.max(0.0001);
        candidates.push(current.clone());

        for _ in 0..config.iterations.max(1) {
            if deadline_reached(deadline) {
                break;
            }
            let mut next = current.clone();
            if !next.is_empty() {
                let slot = rand_index(&mut s, next.len());
                let candidate = rand_index(&mut s, item_pool.len());
                next[slot] = candidate;
                repair_build(item_pool, &mut next, max_items, &mut s);
                let next_key = canonical_key(&next);
                let next_score = score_fn(&next_key);
                let delta = next_score - current_score;
                let accept = delta >= 0.0 || rand_f64(&mut s) < (delta / temp).exp();
                if accept {
                    current = next;
                    current_score = next_score;
                    candidates.push(current.clone());
                    if current_score > best_score {
                        best_score = current_score;
                        best = current.clone();
                    }
                }
            }
            temp = (temp * config.cooling_rate.clamp(0.8, 0.9999)).max(0.0001);
        }
        candidates.push(best);
    }

    unique_ranked_from_candidates(candidates, score_fn, config.limit, deadline)
}

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

fn available_actions(item_pool: &[Item], build: &[usize]) -> Vec<usize> {
    (0..item_pool.len())
        .filter(|&idx| can_add_item_to_build(item_pool, build, idx))
        .collect()
}

fn rollout_completion<F>(
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

#[cfg(test)]
#[path = "tests/item_candidate_search_strategies_tests.rs"]
mod item_candidate_search_strategies_tests;

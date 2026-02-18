use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crate::data::LoadoutDomain;
use crate::status::deadline_reached;

use super::{
    BuildKey, BuildMetrics, BuildSearchConfig, ChampionBase, Item, LoadoutSelection,
    SimulationConfig, Stats, build_from_indices, build_item_stats, can_add_item_to_build,
    canonical_build_candidate, canonical_key, compute_champion_final_stats,
    compute_effective_item_stats_for_build, is_boots, loadout_selection_key, rand_f64, rand_index,
    random_loadout_selection, random_valid_build, repair_build, shuffle_usize,
};

#[allow(dead_code)]
fn unique_ranked_from_candidates<F>(
    candidates: Vec<Vec<usize>>,
    score_fn: &F,
    limit: usize,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let scored = score_candidates(candidates, score_fn, deadline);
    let mut ranked = Vec::new();
    let mut seen = HashSet::new();
    for (_, key, score) in scored {
        if !score.is_finite() {
            continue;
        }
        if seen.insert(key.clone()) {
            ranked.push((key, score));
            if ranked.len() >= limit.max(1) {
                break;
            }
        }
    }
    ranked
}

#[allow(dead_code)]
fn score_candidates<F>(
    candidates: Vec<Vec<usize>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    if candidates.is_empty() || deadline_reached(deadline) {
        return Vec::new();
    }
    let unique_keys: HashSet<Vec<usize>> = candidates.iter().map(|c| canonical_key(c)).collect();
    let mut key_list = unique_keys.into_iter().collect::<Vec<_>>();
    key_list.sort_unstable();

    let score_pairs = key_list
        .par_iter()
        .map(|key| {
            if deadline_reached(deadline) {
                (key.clone(), f64::NEG_INFINITY)
            } else {
                (key.clone(), score_fn(key))
            }
        })
        .collect::<Vec<_>>();
    let score_map = score_pairs
        .into_iter()
        .collect::<HashMap<Vec<usize>, f64>>();

    let mut scored = candidates
        .into_iter()
        .map(|candidate| {
            let key = canonical_key(&candidate);
            let score = score_map.get(&key).copied().unwrap_or(f64::NEG_INFINITY);
            (candidate, key, score)
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| {
        b.2.partial_cmp(&a.2)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.0.cmp(&b.0))
    });
    scored
}

#[allow(dead_code)]
fn beam_search_ranked<F>(
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

#[allow(dead_code)]
fn random_search_ranked<F>(
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

struct HillClimbSearchConfig {
    restarts: usize,
    steps: usize,
    neighbors_per_step: usize,
    seed: u64,
    limit: usize,
}

#[allow(dead_code)]
fn hill_climb_search_ranked<F>(
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

#[allow(dead_code)]
fn tournament_parent(
    scored_population: &[(Vec<usize>, f64)],
    seed: &mut u64,
    tournament_size: usize,
) -> Vec<usize> {
    let mut best_idx = rand_index(seed, scored_population.len());
    for _ in 1..tournament_size.max(1) {
        let idx = rand_index(seed, scored_population.len());
        if scored_population[idx].1 > scored_population[best_idx].1 {
            best_idx = idx;
        }
    }
    scored_population[best_idx].0.clone()
}

fn crossover_builds(
    parent_a: &[usize],
    parent_b: &[usize],
    item_pool: &[Item],
    max_items: usize,
    seed: &mut u64,
) -> Vec<usize> {
    let mut merged = parent_a.to_vec();
    for &idx in parent_b {
        if !merged.contains(&idx) {
            merged.push(idx);
        }
    }
    shuffle_usize(&mut merged, seed);
    let mut child = Vec::with_capacity(max_items);
    for idx in merged {
        if child.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &child, idx) {
            child.push(idx);
        }
    }
    repair_build(item_pool, &mut child, max_items, seed);
    child
}

fn mutate_build(
    build: &mut Vec<usize>,
    item_pool: &[Item],
    max_items: usize,
    mutation_rate: f64,
    seed: &mut u64,
) {
    if build.is_empty() || rand_f64(seed) > mutation_rate.clamp(0.0, 1.0) {
        return;
    }
    let slot = rand_index(seed, build.len());
    let mut tries = 0usize;
    while tries < item_pool.len() {
        let candidate = rand_index(seed, item_pool.len());
        if candidate != build[slot] {
            let old = build[slot];
            build[slot] = candidate;
            if can_add_item_to_build(item_pool, &build[..slot], build[slot])
                && !build[(slot + 1)..].contains(&build[slot])
            {
                repair_build(item_pool, build, max_items, seed);
                return;
            }
            build[slot] = old;
        }
        tries += 1;
    }
    repair_build(item_pool, build, max_items, seed);
}

struct GeneticSearchConfig {
    population_size: usize,
    generations: usize,
    mutation_rate: f64,
    crossover_rate: f64,
    seed: u64,
    limit: usize,
}

#[allow(dead_code)]
fn genetic_search_ranked<F>(
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

struct SimulatedAnnealingSearchConfig {
    restarts: usize,
    iterations: usize,
    initial_temp: f64,
    cooling_rate: f64,
    seed: u64,
    limit: usize,
}

#[allow(dead_code)]
fn simulated_annealing_search_ranked<F>(
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn available_actions(item_pool: &[Item], build: &[usize]) -> Vec<usize> {
    (0..item_pool.len())
        .filter(|&idx| can_add_item_to_build(item_pool, build, idx))
        .collect()
}

#[allow(dead_code)]
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

struct MctsSearchConfig {
    iterations: usize,
    rollouts_per_expansion: usize,
    exploration: f64,
    seed: u64,
    limit: usize,
}

#[allow(dead_code)]
fn mcts_search_ranked<F>(
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

    let _used_actions = nodes.iter().filter_map(|n| n.action_from_parent).count();
    unique_ranked_from_candidates(all_rollout_keys, score_fn, config.limit, deadline)
}

#[allow(dead_code)]
fn symmetric_diff_count(a: &[usize], b: &[usize]) -> usize {
    let sa = a.iter().copied().collect::<HashSet<_>>();
    let sb = b.iter().copied().collect::<HashSet<_>>();
    sa.symmetric_difference(&sb).count()
}

#[allow(dead_code)]
pub(super) fn select_diverse_top_builds(
    ranked: &[(Vec<usize>, f64)],
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
) -> Vec<(Vec<usize>, f64)> {
    if ranked.is_empty() || top_x == 0 {
        return vec![];
    }

    let best_score = ranked[0].1;
    let min_allowed = best_score * (1.0 - (max_relative_gap_percent / 100.0));

    let mut selected: Vec<(Vec<usize>, f64)> = Vec::new();
    for (build, score) in ranked {
        if *score < min_allowed {
            continue;
        }
        if selected
            .iter()
            .all(|(chosen, _)| symmetric_diff_count(chosen, build) >= min_item_diff)
        {
            selected.push((build.clone(), *score));
            if selected.len() >= top_x {
                break;
            }
        }
    }
    selected
}

pub(super) fn item_names(items: &[Item]) -> String {
    items
        .iter()
        .map(|i| i.name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn choose_best_build_by_stat(
    item_pool: &[Item],
    stat_key: &str,
    max_items: usize,
    beam_width: usize,
) -> Vec<usize> {
    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    for _ in 0..max_items {
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
        next_candidates.sort_by(|a, b| {
            let sa = build_item_stats(&build_from_indices(item_pool, a)).get_stat(stat_key);
            let sb = build_item_stats(&build_from_indices(item_pool, b)).get_stat(stat_key);
            sb.partial_cmp(&sa).unwrap_or(Ordering::Equal)
        });
        next_candidates.truncate(beam_width);
        candidates = next_candidates;
    }
    candidates.into_iter().next().unwrap_or_default()
}

#[allow(dead_code)]
pub(super) fn build_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    search: &BuildSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    if deadline_reached(deadline) {
        return Vec::new();
    }
    match search.strategy.as_str() {
        "greedy" => {
            let mut build = Vec::new();
            for _ in 0..max_items {
                if deadline_reached(deadline) {
                    break;
                }
                let mut best: Option<usize> = None;
                let mut best_score = f64::NEG_INFINITY;
                for item_idx in 0..item_pool.len() {
                    if !can_add_item_to_build(item_pool, &build, item_idx) {
                        continue;
                    }
                    let mut candidate = build.clone();
                    candidate.push(item_idx);
                    let score = score_fn(&canonical_key(&candidate));
                    if score > best_score {
                        best_score = score;
                        best = Some(item_idx);
                    }
                }
                if let Some(item_idx) = best {
                    build.push(item_idx);
                } else {
                    break;
                }
            }
            let key = canonical_key(&build);
            vec![(key.clone(), score_fn(&key))]
        }
        "beam" => beam_search_ranked(item_pool, max_items, search.beam_width, score_fn, deadline),
        "random" => random_search_ranked(
            item_pool,
            max_items,
            search.random_samples,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "hill_climb" => hill_climb_search_ranked(
            item_pool,
            max_items,
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
        "genetic" => genetic_search_ranked(
            item_pool,
            max_items,
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
        "simulated_annealing" => simulated_annealing_search_ranked(
            item_pool,
            max_items,
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
        "mcts" => mcts_search_ranked(
            item_pool,
            max_items,
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
        "portfolio" => {
            let strategies = portfolio_strategy_list(search);
            let mut ranked_sets = strategies
                .par_iter()
                .enumerate()
                .map(|(idx, strat)| {
                    if deadline_reached(deadline) {
                        return (idx, Vec::new());
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strat.clone();
                    cfg.seed = search.seed.wrapping_add((idx as u64 + 1) * 1_000_003);
                    (
                        idx,
                        build_search_ranked(item_pool, max_items, &cfg, score_fn, deadline),
                    )
                })
                .collect::<Vec<_>>();
            ranked_sets.sort_by_key(|(idx, _)| *idx);
            let mut merged_candidates = Vec::new();
            for (_, ranked) in ranked_sets {
                for (build, _) in ranked {
                    merged_candidates.push(build);
                }
            }
            unique_ranked_from_candidates(
                merged_candidates,
                score_fn,
                search.ranked_limit,
                deadline,
            )
        }
        _ => vec![],
    }
}

pub(super) fn portfolio_strategy_list(search: &BuildSearchConfig) -> Vec<String> {
    if search.strategy != "portfolio" {
        return vec![search.strategy.clone()];
    }
    let mut strategies = if search.portfolio_strategies.is_empty() {
        vec![
            "beam".to_string(),
            "hill_climb".to_string(),
            "genetic".to_string(),
            "simulated_annealing".to_string(),
            "mcts".to_string(),
            "random".to_string(),
            "greedy".to_string(),
        ]
    } else {
        search.portfolio_strategies.clone()
    };
    strategies.retain(|s| s != "portfolio");
    if strategies.is_empty() {
        strategies.push("beam".to_string());
    }
    strategies
}

pub(super) fn search_strategy_summary(search: &BuildSearchConfig) -> String {
    if search.strategy == "portfolio" {
        let strategies = portfolio_strategy_list(search);
        format!("portfolio({})", strategies.join(", "))
    } else {
        search.strategy.clone()
    }
}

#[allow(dead_code)]
pub(super) fn strategy_seed_elites<F>(
    item_pool: &[Item],
    max_items: usize,
    search: &BuildSearchConfig,
    strategies: &[String],
    score_fn: &F,
    deadline: Option<Instant>,
) -> HashMap<String, Vec<Vec<usize>>>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let ensemble = search.ensemble_seeds.max(1);
    let top_k = search.ensemble_seed_top_k.max(1);

    let grouped = strategies
        .par_iter()
        .enumerate()
        .map(|(sidx, strategy)| {
            let mut aggregate = HashMap::<Vec<usize>, f64>::new();
            let seed_ranked = (0..ensemble)
                .into_par_iter()
                .map(|seed_idx| {
                    if deadline_reached(deadline) {
                        return Vec::new();
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strategy.clone();
                    cfg.seed = search.seed.wrapping_add(
                        ((sidx as u64 + 1) * 31 + seed_idx as u64 + 1)
                            * search.ensemble_seed_stride,
                    );
                    cfg.ranked_limit = top_k.max(64);
                    build_search_ranked(item_pool, max_items, &cfg, score_fn, deadline)
                })
                .collect::<Vec<_>>();
            for ranked in seed_ranked {
                for (key, score) in ranked.into_iter().take(top_k) {
                    let e = aggregate.entry(key).or_insert(score);
                    if score > *e {
                        *e = score;
                    }
                }
            }
            let mut items = aggregate.into_iter().collect::<Vec<_>>();
            items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            let keys = items.into_iter().map(|(k, _)| k).collect::<Vec<_>>();
            (sidx, strategy.clone(), keys)
        })
        .collect::<Vec<_>>();

    let mut ordered = grouped;
    ordered.sort_by_key(|(idx, _, _)| *idx);
    ordered
        .into_iter()
        .map(|(_, strategy, keys)| (strategy, keys))
        .collect::<HashMap<_, _>>()
}

#[allow(dead_code)]
pub(super) fn generate_bleed_candidates(
    item_pool: &[Item],
    max_items: usize,
    strategy_elites: &HashMap<String, Vec<Vec<usize>>>,
    search: &BuildSearchConfig,
) -> Vec<Vec<usize>> {
    if !search.bleed_enabled {
        return Vec::new();
    }
    let mut seed = search.seed ^ 0xB1EEDu64;
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    strategies.sort_unstable();
    let mut elite_pool = Vec::new();

    for strategy in &strategies {
        if let Some(builds) = strategy_elites.get(strategy) {
            for key in builds.iter().take(search.ensemble_seed_top_k.max(1)) {
                let canon = canonical_key(key);
                if seen.insert(canon.clone()) {
                    out.push(canon.clone());
                    elite_pool.push(canon);
                }
            }
        }
    }
    if elite_pool.is_empty() {
        return out;
    }

    let bleed_budget = if search.bleed_budget > 0 {
        search.bleed_budget
    } else {
        // Max-quality default: at least ranked candidate pool size, with a reasonable floor.
        search.ranked_limit.max(800)
    };
    let cross_budget = bleed_budget / 2;
    let mutate_budget = bleed_budget - cross_budget;
    let mutation_rate = search.bleed_mutation_rate.clamp(0.0, 1.0);

    for _ in 0..cross_budget {
        let a = rand_index(&mut seed, elite_pool.len());
        let b = if strategies.len() >= 2 {
            let sa = rand_index(&mut seed, strategies.len());
            let mut sb = rand_index(&mut seed, strategies.len());
            if sb == sa {
                sb = (sb + 1) % strategies.len();
            }
            let list_a = strategy_elites.get(&strategies[sa]).unwrap_or(&elite_pool);
            let list_b = strategy_elites.get(&strategies[sb]).unwrap_or(&elite_pool);
            let pa = list_a
                .get(rand_index(&mut seed, list_a.len()))
                .cloned()
                .unwrap_or_else(|| elite_pool[a].clone());
            let pb = list_b
                .get(rand_index(&mut seed, list_b.len()))
                .cloned()
                .unwrap_or_else(|| elite_pool[a].clone());
            let mut child = crossover_builds(&pa, &pb, item_pool, max_items, &mut seed);
            mutate_build(&mut child, item_pool, max_items, mutation_rate, &mut seed);
            canonical_key(&child)
        } else {
            let mut child = elite_pool[a].clone();
            mutate_build(&mut child, item_pool, max_items, mutation_rate, &mut seed);
            canonical_key(&child)
        };
        if seen.insert(b.clone()) {
            out.push(b);
        }
    }

    for _ in 0..mutate_budget {
        let mut child = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        mutate_build(&mut child, item_pool, max_items, mutation_rate, &mut seed);
        repair_build(item_pool, &mut child, max_items, &mut seed);
        let key = canonical_key(&child);
        if seen.insert(key.clone()) {
            out.push(key);
        }
    }

    out.sort_unstable();
    out
}

#[allow(dead_code)]
pub(super) fn adaptive_strategy_candidates<F>(
    item_pool: &[Item],
    max_items: usize,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<Vec<usize>>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<Vec<usize>>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    if strategy_elites.is_empty() {
        return Vec::new();
    }
    let mut strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    strategies.sort_unstable();
    let contributions = strategies
        .iter()
        .map(|s| {
            let c = strategy_elites
                .get(s)
                .map(|v| v.len().max(1) as f64)
                .unwrap_or(1.0);
            (s.clone(), c)
        })
        .collect::<Vec<_>>();
    let total_contrib = contributions.iter().map(|(_, c)| *c).sum::<f64>().max(1.0);
    let extra_runs_total = (search.ensemble_seeds.max(1) * strategies.len()).max(8);
    let per_strategy = contributions
        .into_iter()
        .map(|(s, c)| {
            let share = c / total_contrib;
            let runs = ((extra_runs_total as f64) * share).round() as usize;
            (s, runs.max(1))
        })
        .collect::<Vec<_>>();

    let gathered = per_strategy
        .par_iter()
        .enumerate()
        .map(|(sidx, (strategy, runs))| {
            (0..*runs)
                .into_par_iter()
                .flat_map_iter(|ridx| {
                    if deadline_reached(deadline) {
                        return Vec::<Vec<usize>>::new().into_iter();
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strategy.clone();
                    cfg.seed = search.seed.wrapping_add(
                        ((sidx as u64 + 1) * 131 + ridx as u64 + 1) * search.ensemble_seed_stride,
                    );
                    cfg.ranked_limit = (search.ensemble_seed_top_k.max(1) * 2).max(50);
                    let ranked =
                        build_search_ranked(item_pool, max_items, &cfg, score_fn, deadline);
                    ranked
                        .into_iter()
                        .take(search.ensemble_seed_top_k.max(1))
                        .map(|(k, _)| k)
                        .collect::<Vec<_>>()
                        .into_iter()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut out = gathered
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    out.sort_unstable();
    out
}

fn effective_hp_mixed(health: f64, armor: f64, magic_resist: f64) -> f64 {
    let phys_mult = 1.0 + armor.max(0.0) / 100.0;
    let magic_mult = 1.0 + magic_resist.max(0.0) / 100.0;
    health.max(1.0) * 0.5 * (phys_mult + magic_mult)
}

fn build_cost_timing_score(build: &[Item]) -> f64 {
    if build.is_empty() {
        return 0.0;
    }
    let mut weighted = 0.0;
    let mut total = 0.0;
    for (idx, item) in build.iter().enumerate() {
        let w = 1.0 / (1.0 + idx as f64);
        weighted += w * item.total_cost.max(0.0);
        total += item.total_cost.max(0.0);
    }
    // Higher is better. Penalize expensive early spikes more.
    -weighted - 0.1 * total
}

#[allow(dead_code)]
pub(super) fn compute_build_metrics(
    key: &[usize],
    item_pool: &[Item],
    controlled_champion_base: &ChampionBase,
    controlled_champion_bonus_stats: &Stats,
    sim: &SimulationConfig,
    objective: f64,
) -> BuildMetrics {
    let build = build_from_indices(item_pool, key);
    let item_stats = compute_effective_item_stats_for_build(
        controlled_champion_base,
        &build,
        controlled_champion_bonus_stats,
        sim,
        sim.champion_level,
        None,
        None,
    );
    let stats = compute_champion_final_stats(controlled_champion_base, &item_stats);
    let ehp = effective_hp_mixed(stats.health, stats.armor, stats.magic_resist);
    let total_cost = build.iter().map(|i| i.total_cost).sum::<f64>();
    BuildMetrics {
        objective,
        ehp_mixed: ehp,
        ap: stats.ability_power,
        cost_timing: build_cost_timing_score(&build),
        total_cost,
    }
}

fn dominates(a: &BuildMetrics, b: &BuildMetrics) -> bool {
    let ge = a.objective >= b.objective
        && a.ehp_mixed >= b.ehp_mixed
        && a.ap >= b.ap
        && a.cost_timing >= b.cost_timing;
    let gt = a.objective > b.objective
        || a.ehp_mixed > b.ehp_mixed
        || a.ap > b.ap
        || a.cost_timing > b.cost_timing;
    ge && gt
}

#[allow(dead_code)]
pub(super) fn pareto_front_keys(
    metrics_by_key: &HashMap<Vec<usize>, BuildMetrics>,
) -> HashSet<Vec<usize>> {
    let keys = metrics_by_key.keys().cloned().collect::<Vec<_>>();
    let mut front = HashSet::new();
    for key_a in &keys {
        let Some(a) = metrics_by_key.get(key_a) else {
            continue;
        };
        let dominated = keys.iter().any(|key_b| {
            if key_a == key_b {
                return false;
            }
            let Some(b) = metrics_by_key.get(key_b) else {
                return false;
            };
            dominates(b, a)
        });
        if !dominated {
            front.insert(key_a.clone());
        }
    }
    front
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FullLoadoutSearchParams<'a> {
    pub item_pool: &'a [Item],
    pub max_items: usize,
    pub loadout_domain: &'a LoadoutDomain,
    pub base_loadout: &'a LoadoutSelection,
}

fn candidate_order_key(candidate: &BuildKey) -> String {
    format!(
        "i={}|l={}",
        candidate
            .item_indices
            .iter()
            .map(|idx| idx.to_string())
            .collect::<Vec<_>>()
            .join(","),
        loadout_selection_key(&candidate.loadout_selection)
    )
}

fn random_full_candidate(params: &FullLoadoutSearchParams<'_>, seed: &mut u64) -> BuildKey {
    canonical_build_candidate(BuildKey {
        item_indices: random_valid_build(params.item_pool, params.max_items, seed),
        loadout_selection: random_loadout_selection(
            params.base_loadout,
            params.loadout_domain,
            seed,
        ),
    })
}

fn candidate_loadout_variants(
    anchor: &LoadoutSelection,
    params: &FullLoadoutSearchParams<'_>,
    seed: &mut u64,
    random_samples: usize,
) -> Vec<LoadoutSelection> {
    let mut variants = Vec::new();
    let mut seen = HashSet::<String>::new();
    for base in [anchor.clone(), params.base_loadout.clone()] {
        let key = loadout_selection_key(&base);
        if seen.insert(key) {
            variants.push(base);
        }
    }
    for _ in 0..random_samples {
        let sampled = random_loadout_selection(anchor, params.loadout_domain, seed);
        let key = loadout_selection_key(&sampled);
        if seen.insert(key) {
            variants.push(sampled);
        }
    }
    if variants.is_empty() {
        variants.push(params.base_loadout.clone());
    }
    variants
}

fn repair_full_candidate(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &mut BuildKey,
    seed: &mut u64,
) {
    repair_build(
        params.item_pool,
        &mut candidate.item_indices,
        params.max_items,
        seed,
    );
    candidate.item_indices = canonical_key(&candidate.item_indices);
    if candidate.loadout_selection.rune_names.len() != 6
        || candidate.loadout_selection.shard_stats.len() != 3
    {
        candidate.loadout_selection =
            random_loadout_selection(params.base_loadout, params.loadout_domain, seed);
    }
}

fn mutate_full_candidate(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &mut BuildKey,
    mutation_rate: f64,
    seed: &mut u64,
) {
    let rate = mutation_rate.clamp(0.0, 1.0);
    if rand_f64(seed) <= rate {
        mutate_build(
            &mut candidate.item_indices,
            params.item_pool,
            params.max_items,
            rate,
            seed,
        );
    }
    if rand_f64(seed) <= rate {
        candidate.loadout_selection =
            random_loadout_selection(&candidate.loadout_selection, params.loadout_domain, seed);
    }
    repair_full_candidate(params, candidate, seed);
}

fn crossover_full_candidates(
    parent_a: &BuildKey,
    parent_b: &BuildKey,
    params: &FullLoadoutSearchParams<'_>,
    seed: &mut u64,
) -> BuildKey {
    let item_indices = crossover_builds(
        &parent_a.item_indices,
        &parent_b.item_indices,
        params.item_pool,
        params.max_items,
        seed,
    );
    let mut loadout_selection = if rand_f64(seed) < 0.5 {
        parent_a.loadout_selection.clone()
    } else {
        parent_b.loadout_selection.clone()
    };
    if rand_f64(seed) < 0.25 {
        loadout_selection =
            random_loadout_selection(&loadout_selection, params.loadout_domain, seed);
    }
    let mut child = BuildKey {
        item_indices,
        loadout_selection,
    };
    repair_full_candidate(params, &mut child, seed);
    canonical_build_candidate(child)
}

fn score_full_candidates<F>(
    candidates: Vec<BuildKey>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    if candidates.is_empty() {
        return Vec::new();
    }
    let unique = candidates
        .into_iter()
        .map(canonical_build_candidate)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut scored = unique
        .par_iter()
        .map(|candidate| {
            let score = if deadline_reached(deadline) {
                f64::NEG_INFINITY
            } else {
                score_fn(candidate)
            };
            (candidate.clone(), score)
        })
        .collect::<Vec<_>>();
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| candidate_order_key(&a.0).cmp(&candidate_order_key(&b.0)))
    });
    scored
}

fn unique_ranked_full_candidates<F>(
    candidates: Vec<BuildKey>,
    score_fn: &F,
    limit: usize,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    score_full_candidates(candidates, score_fn, deadline)
        .into_iter()
        .take(limit.max(1))
        .collect::<Vec<_>>()
}

fn beam_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    beam_width: usize,
    seed: u64,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = seed;
    let mut candidates: Vec<BuildKey> = vec![BuildKey {
        item_indices: Vec::new(),
        loadout_selection: params.base_loadout.clone(),
    }];
    let mut final_scored = Vec::new();

    for _ in 0..params.max_items {
        if deadline_reached(deadline) {
            break;
        }
        let mut next_candidates = Vec::new();
        for candidate in &candidates {
            let has_boots = candidate
                .item_indices
                .iter()
                .any(|&idx| is_boots(&params.item_pool[idx]));
            let used = candidate
                .item_indices
                .iter()
                .copied()
                .collect::<HashSet<_>>();
            for (item_idx, item) in params.item_pool.iter().enumerate() {
                if used.contains(&item_idx) {
                    continue;
                }
                if is_boots(item) && has_boots {
                    continue;
                }
                let mut next = candidate.clone();
                next.item_indices.push(item_idx);
                next.item_indices = canonical_key(&next.item_indices);
                let loadout_variants =
                    candidate_loadout_variants(&next.loadout_selection, params, &mut local_seed, 1);
                for loadout_selection in loadout_variants {
                    let mut variant = next.clone();
                    variant.loadout_selection = loadout_selection;
                    next_candidates.push(canonical_build_candidate(variant));
                }
            }
        }
        let scored = score_full_candidates(next_candidates, score_fn, deadline);
        candidates = scored
            .iter()
            .take(beam_width.max(1))
            .map(|(candidate, _)| candidate.clone())
            .collect::<Vec<_>>();
        final_scored = scored;
    }
    final_scored
}

fn random_search_ranked_full<F>(
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
    let mut local_seed = seed;
    let mut candidates = Vec::with_capacity(random_samples.max(1));
    for _ in 0..random_samples.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        candidates.push(random_full_candidate(params, &mut local_seed));
    }
    unique_ranked_full_candidates(candidates, score_fn, limit, deadline)
}

fn hill_climb_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    config: &HillClimbSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = config.seed;
    let mut candidates = Vec::new();
    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_full_candidate(params, &mut local_seed);
        let mut current_score = score_fn(&current);
        candidates.push(current.clone());

        for _ in 0..config.steps {
            if deadline_reached(deadline) {
                break;
            }
            let mut neighbor_candidates = Vec::new();
            for _ in 0..config.neighbors_per_step.max(1) {
                let mut neighbor = current.clone();
                mutate_full_candidate(params, &mut neighbor, 1.0, &mut local_seed);
                neighbor_candidates.push(neighbor);
            }
            if neighbor_candidates.is_empty() {
                break;
            }
            let ranked_neighbors = unique_ranked_full_candidates(
                neighbor_candidates,
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
    unique_ranked_full_candidates(candidates, score_fn, config.limit, deadline)
}

fn tournament_parent_full(
    scored_population: &[(BuildKey, f64)],
    seed: &mut u64,
    tournament_size: usize,
) -> BuildKey {
    let mut best_idx = rand_index(seed, scored_population.len());
    for _ in 1..tournament_size.max(1) {
        let idx = rand_index(seed, scored_population.len());
        if scored_population[idx].1 > scored_population[best_idx].1 {
            best_idx = idx;
        }
    }
    scored_population[best_idx].0.clone()
}

fn genetic_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    config: &GeneticSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let population_size = config.population_size.max(8);
    let mut local_seed = config.seed;
    let mut population = Vec::with_capacity(population_size);
    for _ in 0..population_size {
        if deadline_reached(deadline) {
            break;
        }
        population.push(random_full_candidate(params, &mut local_seed));
    }

    let mut all_seen = population.clone();
    for _ in 0..config.generations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut scored =
            unique_ranked_full_candidates(population.clone(), score_fn, population_size, deadline);
        if scored.is_empty() {
            break;
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        let elite_count = (population_size / 8).max(1).min(scored.len());
        let mut next_population = scored
            .iter()
            .take(elite_count)
            .map(|(candidate, _)| candidate.clone())
            .collect::<Vec<_>>();

        while next_population.len() < population_size {
            if deadline_reached(deadline) {
                break;
            }
            let parent_a = tournament_parent_full(&scored, &mut local_seed, 3);
            let parent_b = tournament_parent_full(&scored, &mut local_seed, 3);
            let mut child = if rand_f64(&mut local_seed) <= config.crossover_rate.clamp(0.0, 1.0) {
                crossover_full_candidates(&parent_a, &parent_b, params, &mut local_seed)
            } else {
                parent_a
            };
            mutate_full_candidate(params, &mut child, config.mutation_rate, &mut local_seed);
            repair_full_candidate(params, &mut child, &mut local_seed);
            next_population.push(child);
        }

        all_seen.extend(next_population.clone());
        population = next_population;
    }

    unique_ranked_full_candidates(all_seen, score_fn, config.limit, deadline)
}

fn simulated_annealing_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    config: &SimulatedAnnealingSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = config.seed;
    let mut candidates = Vec::new();

    for _ in 0..config.restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_full_candidate(params, &mut local_seed);
        let mut current_score = score_fn(&current);
        let mut best = current.clone();
        let mut best_score = current_score;
        let mut temp = config.initial_temp.max(0.0001);
        candidates.push(current.clone());

        for _ in 0..config.iterations.max(1) {
            if deadline_reached(deadline) {
                break;
            }
            let mut next = current.clone();
            mutate_full_candidate(params, &mut next, 1.0, &mut local_seed);
            let next_score = score_fn(&next);
            let delta = next_score - current_score;
            let accept = delta >= 0.0 || rand_f64(&mut local_seed) < (delta / temp).exp();
            if accept {
                current = next;
                current_score = next_score;
                candidates.push(current.clone());
                if current_score > best_score {
                    best_score = current_score;
                    best = current.clone();
                }
            }
            temp = (temp * config.cooling_rate.clamp(0.8, 0.9999)).max(0.0001);
        }
        candidates.push(best);
    }

    unique_ranked_full_candidates(candidates, score_fn, config.limit, deadline)
}

#[derive(Debug, Clone)]
struct MctsFullNode {
    candidate: BuildKey,
    parent: Option<usize>,
    children: Vec<usize>,
    visits: usize,
    value_sum: f64,
}

fn mcts_search_ranked_full<F>(
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
        "greedy" => {
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
        "portfolio" => {
            let strategies = portfolio_strategy_list(search);
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
        _ => Vec::new(),
    }
}

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

pub(super) fn generate_bleed_candidates_full_loadout(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<BuildKey>>,
) -> Vec<BuildKey> {
    if !search.bleed_enabled {
        return Vec::new();
    }
    let mut seed = search.seed ^ 0xB1EED_u64;
    let mut out = Vec::new();
    let mut seen = HashSet::<BuildKey>::new();
    let mut strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    strategies.sort_unstable();
    let mut elite_pool = Vec::new();

    for strategy in &strategies {
        if let Some(candidates) = strategy_elites.get(strategy) {
            for candidate in candidates.iter().take(search.ensemble_seed_top_k.max(1)) {
                let canonical = canonical_build_candidate(candidate.clone());
                if seen.insert(canonical.clone()) {
                    out.push(canonical.clone());
                    elite_pool.push(canonical);
                }
            }
        }
    }
    if elite_pool.is_empty() {
        return out;
    }

    let bleed_budget = if search.bleed_budget > 0 {
        search.bleed_budget
    } else {
        search.ranked_limit.max(800)
    };
    let cross_budget = bleed_budget / 2;
    let mutate_budget = bleed_budget - cross_budget;
    let mutation_rate = search.bleed_mutation_rate.clamp(0.0, 1.0);

    for _ in 0..cross_budget {
        let parent_a = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        let child = if strategies.len() >= 2 {
            let strategy_a = rand_index(&mut seed, strategies.len());
            let mut strategy_b = rand_index(&mut seed, strategies.len());
            if strategy_b == strategy_a {
                strategy_b = (strategy_b + 1) % strategies.len();
            }
            let list_a = strategy_elites
                .get(&strategies[strategy_a])
                .unwrap_or(&elite_pool);
            let list_b = strategy_elites
                .get(&strategies[strategy_b])
                .unwrap_or(&elite_pool);
            let parent_a_candidate = list_a
                .get(rand_index(&mut seed, list_a.len()))
                .cloned()
                .unwrap_or(parent_a.clone());
            let parent_b_candidate = list_b
                .get(rand_index(&mut seed, list_b.len()))
                .cloned()
                .unwrap_or(parent_a.clone());
            let mut child = crossover_full_candidates(
                &parent_a_candidate,
                &parent_b_candidate,
                params,
                &mut seed,
            );
            mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
            canonical_build_candidate(child)
        } else {
            let mut child = parent_a.clone();
            mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
            canonical_build_candidate(child)
        };
        if seen.insert(child.clone()) {
            out.push(child);
        }
    }

    for _ in 0..mutate_budget {
        let mut child = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
        repair_full_candidate(params, &mut child, &mut seed);
        let canonical = canonical_build_candidate(child);
        if seen.insert(canonical.clone()) {
            out.push(canonical);
        }
    }
    out.sort_by_key(candidate_order_key);
    out
}

fn symmetric_item_diff_count(a: &BuildKey, b: &BuildKey) -> usize {
    let set_a = a.item_indices.iter().copied().collect::<HashSet<_>>();
    let set_b = b.item_indices.iter().copied().collect::<HashSet<_>>();
    set_a.symmetric_difference(&set_b).count()
}

pub(super) fn select_diverse_top_candidates(
    ranked: &[(BuildKey, f64)],
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
) -> Vec<(BuildKey, f64)> {
    if ranked.is_empty() || top_x == 0 {
        return Vec::new();
    }
    let best_score = ranked[0].1;
    let min_allowed = best_score * (1.0 - (max_relative_gap_percent / 100.0));

    let mut selected = Vec::new();
    for (candidate, score) in ranked {
        if *score < min_allowed {
            continue;
        }
        if selected
            .iter()
            .all(|(chosen, _)| symmetric_item_diff_count(chosen, candidate) >= min_item_diff)
        {
            selected.push((candidate.clone(), *score));
            if selected.len() >= top_x {
                break;
            }
        }
    }
    selected
}

pub(super) fn compute_build_metrics_for_candidate(
    candidate: &BuildKey,
    item_pool: &[Item],
    controlled_champion_base: &ChampionBase,
    controlled_champion_bonus_stats: &Stats,
    controlled_champion_stack_overrides: &HashMap<String, f64>,
    sim: &SimulationConfig,
    objective: f64,
) -> BuildMetrics {
    let build = build_from_indices(item_pool, &candidate.item_indices);
    let item_stats = compute_effective_item_stats_for_build(
        controlled_champion_base,
        &build,
        controlled_champion_bonus_stats,
        sim,
        sim.champion_level,
        None,
        Some(controlled_champion_stack_overrides),
    );
    let stats = compute_champion_final_stats(controlled_champion_base, &item_stats);
    let ehp = effective_hp_mixed(stats.health, stats.armor, stats.magic_resist);
    let total_cost = build.iter().map(|item| item.total_cost).sum::<f64>();
    BuildMetrics {
        objective,
        ehp_mixed: ehp,
        ap: stats.ability_power,
        cost_timing: build_cost_timing_score(&build),
        total_cost,
    }
}

pub(super) fn candidate_pareto_front_keys(
    metrics_by_key: &HashMap<BuildKey, BuildMetrics>,
) -> HashSet<BuildKey> {
    let keys = metrics_by_key.keys().cloned().collect::<Vec<_>>();
    let mut front = HashSet::new();
    for key_a in &keys {
        let Some(metrics_a) = metrics_by_key.get(key_a) else {
            continue;
        };
        let dominated = keys.iter().any(|key_b| {
            if key_a == key_b {
                return false;
            }
            let Some(metrics_b) = metrics_by_key.get(key_b) else {
                return false;
            };
            dominates(metrics_b, metrics_a)
        });
        if !dominated {
            front.insert(key_a.clone());
        }
    }
    front
}

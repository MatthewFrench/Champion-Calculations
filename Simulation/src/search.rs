use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crate::data::LoadoutDomain;
use crate::status::deadline_reached;

use super::{
    BuildKey, BuildMetrics, BuildSearchConfig, ChampionBase, Item, LoadoutSelection,
    SimulationConfig, Stats, can_add_item_to_build, canonical_build_candidate, canonical_key,
    is_boots, rand_f64, rand_index, random_loadout_selection, repair_build,
};

mod candidate_space;
mod full_loadout_search_orchestration;
mod scoring;
mod strategy;

use self::candidate_space::full_loadout_candidate_operations::{
    candidate_loadout_variants, candidate_order_key, crossover_full_candidates,
    mutate_full_candidate, repair_full_candidate,
};
use self::candidate_space::full_loadout_candidate_scoring::unique_ranked_full_candidates;
use self::candidate_space::item_candidate_operations::{crossover_builds, mutate_build};
use self::candidate_space::item_candidate_scoring::unique_ranked_from_candidates;
pub(crate) use self::full_loadout_search_orchestration::FullLoadoutSearchParams;
pub(crate) use self::full_loadout_search_orchestration::{
    adaptive_strategy_candidates_full_loadout, build_search_ranked_full_loadout,
    generate_bleed_candidates_full_loadout, strategy_seed_elites_full_loadout,
};
use self::strategy::full_loadout_search_strategies::{
    beam_search_ranked_full, genetic_search_ranked_full, hill_climb_search_ranked_full,
    mcts_search_ranked_full, random_search_ranked_full, simulated_annealing_search_ranked_full,
};
use self::strategy::item_candidate_search_strategies::{
    beam_search_ranked, genetic_search_ranked, hill_climb_search_ranked, mcts_search_ranked,
    random_search_ranked, simulated_annealing_search_ranked,
};

struct HillClimbSearchConfig {
    restarts: usize,
    steps: usize,
    neighbors_per_step: usize,
    seed: u64,
    limit: usize,
}

struct GeneticSearchConfig {
    population_size: usize,
    generations: usize,
    mutation_rate: f64,
    crossover_rate: f64,
    seed: u64,
    limit: usize,
}

struct SimulatedAnnealingSearchConfig {
    restarts: usize,
    iterations: usize,
    initial_temp: f64,
    cooling_rate: f64,
    seed: u64,
    limit: usize,
}

struct MctsSearchConfig {
    iterations: usize,
    rollouts_per_expansion: usize,
    exploration: f64,
    seed: u64,
    limit: usize,
}

#[allow(dead_code)]
pub(super) fn select_diverse_top_builds(
    ranked: &[(Vec<usize>, f64)],
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
) -> Vec<(Vec<usize>, f64)> {
    scoring::item_build_scoring_and_diversity::select_diverse_top_builds(
        ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
    )
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
    scoring::item_build_scoring_and_diversity::compute_build_metrics(
        key,
        item_pool,
        controlled_champion_base,
        controlled_champion_bonus_stats,
        sim,
        objective,
    )
}

#[allow(dead_code)]
pub(super) fn pareto_front_keys(
    metrics_by_key: &HashMap<Vec<usize>, BuildMetrics>,
) -> HashSet<Vec<usize>> {
    scoring::item_build_scoring_and_diversity::pareto_front_keys(metrics_by_key)
}

pub(super) fn item_names(items: &[Item]) -> String {
    scoring::item_name_list_formatting::format_item_name_list_comma_separated(items)
}

pub(super) fn choose_best_build_by_stat(
    item_pool: &[Item],
    stat_key: &str,
    max_items: usize,
    beam_width: usize,
) -> Vec<usize> {
    scoring::stat_key_build_selection::choose_best_build_by_stat(
        item_pool, stat_key, max_items, beam_width,
    )
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

pub(super) fn select_diverse_top_candidates(
    ranked: &[(BuildKey, f64)],
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
) -> Vec<(BuildKey, f64)> {
    scoring::full_loadout_scoring_and_diversity::select_diverse_top_candidates(
        ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
    )
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
    scoring::full_loadout_scoring_and_diversity::compute_build_metrics_for_candidate(
        candidate,
        item_pool,
        controlled_champion_base,
        controlled_champion_bonus_stats,
        controlled_champion_stack_overrides,
        sim,
        objective,
    )
}

pub(super) fn candidate_pareto_front_keys(
    metrics_by_key: &HashMap<BuildKey, BuildMetrics>,
) -> HashSet<BuildKey> {
    scoring::full_loadout_scoring_and_diversity::candidate_pareto_front_keys(metrics_by_key)
}

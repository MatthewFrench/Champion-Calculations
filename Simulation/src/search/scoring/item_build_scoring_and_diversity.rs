use std::collections::{HashMap, HashSet};

use super::metric_scoring_helpers::{
    build_cost_timing_score, build_metrics_dominates, effective_health_points_mixed,
};
use crate::{
    BuildMetrics, ChampionBase, Item, SimulationConfig, Stats, build_from_indices,
    compute_champion_final_stats, compute_effective_item_stats_for_build,
};

fn symmetric_item_difference_count(a: &[usize], b: &[usize]) -> usize {
    let set_a = a.iter().copied().collect::<HashSet<_>>();
    let set_b = b.iter().copied().collect::<HashSet<_>>();
    set_a.symmetric_difference(&set_b).count()
}

#[allow(dead_code)]
pub(in crate::search) fn select_diverse_top_builds(
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
            .all(|(chosen, _)| symmetric_item_difference_count(chosen, build) >= min_item_diff)
        {
            selected.push((build.clone(), *score));
            if selected.len() >= top_x {
                break;
            }
        }
    }
    selected
}

#[allow(dead_code)]
pub(in crate::search) fn compute_build_metrics(
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
    let effective_health_points =
        effective_health_points_mixed(stats.health, stats.armor, stats.magic_resist);
    let total_cost = build.iter().map(|item| item.total_cost).sum::<f64>();
    BuildMetrics {
        objective,
        ehp_mixed: effective_health_points,
        ap: stats.ability_power,
        cost_timing: build_cost_timing_score(&build),
        total_cost,
    }
}

#[allow(dead_code)]
pub(in crate::search) fn pareto_front_keys(
    metrics_by_key: &HashMap<Vec<usize>, BuildMetrics>,
) -> HashSet<Vec<usize>> {
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
            build_metrics_dominates(metrics_b, metrics_a)
        });
        if !dominated {
            front.insert(key_a.clone());
        }
    }
    front
}

#[cfg(test)]
#[path = "tests/item_build_scoring_and_diversity_tests.rs"]
mod item_build_scoring_and_diversity_tests;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use crate::engine::simulate_controlled_champion_combat;

use super::{
    BuildOrderEvalContext, BuildOrderResult, ChampionBase, CombatOutcome, EnemyConfig, Item, Stats,
    champion_at_level, objective_score_from_outcome,
};

fn build_level_milestones(item_count: usize, start_level: usize, end_level: usize) -> Vec<usize> {
    if item_count == 0 {
        return vec![];
    }
    if item_count == 1 {
        return vec![end_level.max(start_level)];
    }
    let start = start_level as f64;
    let end = end_level as f64;
    let denom = (item_count - 1) as f64;
    (0..item_count)
        .map(|i| {
            let t = (i as f64) / denom;
            (start + (end - start) * t).round().max(1.0) as usize
        })
        .collect()
}

pub(super) fn acquisition_level_map(items: &[Item], levels: &[usize]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (item, lvl) in items.iter().zip(levels.iter()) {
        map.insert(item.name.clone(), *lvl);
    }
    map
}

fn level_scaled_enemy_builds(
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    raw_enemy_bases: &HashMap<String, ChampionBase>,
) -> Vec<(EnemyConfig, Vec<Item>, Stats)> {
    enemy_builds
        .iter()
        .map(|(enemy_cfg, build, bonus_stats)| {
            let raw_base = raw_enemy_bases
                .get(&enemy_cfg.id)
                .cloned()
                .unwrap_or_else(|| enemy_cfg.base.clone());
            let mut scaled_cfg = enemy_cfg.clone();
            scaled_cfg.base = champion_at_level(&raw_base, enemy_cfg.level);
            (scaled_cfg, build.clone(), bonus_stats.clone())
        })
        .collect()
}

fn normalized_encounter_weights(ctx: &BuildOrderEvalContext<'_>) -> Vec<f64> {
    let raw = ctx
        .enemy_build_scenarios
        .iter()
        .map(|(_, weight, _)| (*weight).max(0.0))
        .collect::<Vec<_>>();
    let sum = raw.iter().sum::<f64>();
    if sum > 0.0 {
        raw.into_iter()
            .map(|weight| weight / sum)
            .collect::<Vec<_>>()
    } else if !raw.is_empty() {
        let uniform = 1.0 / raw.len() as f64;
        raw.into_iter().map(|_| uniform).collect::<Vec<_>>()
    } else {
        Vec::new()
    }
}

fn simulate_build_order_stage_outcomes_for_scenario(
    ordered_items: &[Item],
    levels: &[usize],
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    ctx: &BuildOrderEvalContext<'_>,
) -> Vec<CombatOutcome> {
    let mut stage_outcomes = Vec::with_capacity(levels.len());
    for (idx, level) in levels.iter().enumerate() {
        let prefix = &ordered_items[..=idx];
        let prefix_levels = &levels[..=idx];
        let acquired_map = acquisition_level_map(prefix, prefix_levels);
        let controlled_champion_base_level =
            champion_at_level(ctx.controlled_champion_base_raw, *level);
        let enemy_level_builds = level_scaled_enemy_builds(enemy_builds, ctx.raw_enemy_bases);
        let mut sim_at_level = ctx.sim.clone();
        sim_at_level.champion_level = *level;
        let outcome = simulate_controlled_champion_combat(
            &controlled_champion_base_level,
            prefix,
            ctx.controlled_champion_bonus_stats,
            None,
            Some(&acquired_map),
            Some(ctx.controlled_champion_stack_overrides),
            &enemy_level_builds,
            &sim_at_level,
            ctx.urf,
        );
        stage_outcomes.push(outcome);
    }
    stage_outcomes
}

fn score_build_order(
    ordered_items: &[Item],
    levels: &[usize],
    reference_outcomes_by_scenario: &[Vec<CombatOutcome>],
    ctx: &BuildOrderEvalContext<'_>,
) -> BuildOrderResult {
    let encounter_weights = normalized_encounter_weights(ctx);
    let stage_outcomes_by_scenario = ctx
        .enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds)| {
            simulate_build_order_stage_outcomes_for_scenario(
                ordered_items,
                levels,
                enemy_builds,
                ctx,
            )
        })
        .collect::<Vec<_>>();
    let scenario_count = stage_outcomes_by_scenario.len();

    let mut stage_survival = Vec::new();
    let mut stage_damage = Vec::new();
    let mut stage_healing = Vec::new();
    let mut stage_objective_scores = Vec::new();
    let mut cumulative_score = 0.0;
    for idx in 0..levels.len() {
        let stage_level = levels.get(idx).copied().unwrap_or(ctx.sim.champion_level);
        let mut sim_at_level = ctx.sim.clone();
        sim_at_level.champion_level = stage_level;
        let default_reference = CombatOutcome {
            time_alive_seconds: sim_at_level.max_time_seconds.max(1.0),
            damage_dealt: 1.0,
            healing_done: 1.0,
            enemy_kills: 0,
            invulnerable_seconds: 0.0,
        };
        let mut weighted_outcome = CombatOutcome::default();
        let mut weighted_enemy_kills = 0.0;
        let mut weighted_mean_score = 0.0;
        let mut worst_case_score = f64::INFINITY;

        for scenario_idx in 0..scenario_count {
            let weight = encounter_weights.get(scenario_idx).copied().unwrap_or(0.0);
            let outcome = stage_outcomes_by_scenario
                .get(scenario_idx)
                .and_then(|stages| stages.get(idx))
                .copied()
                .unwrap_or_default();
            let reference = reference_outcomes_by_scenario
                .get(scenario_idx)
                .and_then(|stages| stages.get(idx))
                .copied()
                .unwrap_or(default_reference);
            let stage_score =
                objective_score_from_outcome(outcome, reference, ctx.objective_weights);
            weighted_mean_score += stage_score * weight;
            worst_case_score = worst_case_score.min(stage_score);
            weighted_outcome.time_alive_seconds += outcome.time_alive_seconds * weight;
            weighted_outcome.damage_dealt += outcome.damage_dealt * weight;
            weighted_outcome.healing_done += outcome.healing_done * weight;
            weighted_outcome.invulnerable_seconds += outcome.invulnerable_seconds * weight;
            weighted_enemy_kills += outcome.enemy_kills as f64 * weight;
        }
        weighted_outcome.enemy_kills = weighted_enemy_kills.round().max(0.0) as usize;
        let stage_score = if scenario_count > 1 {
            weighted_mean_score * (1.0 - ctx.multi_scenario_worst_weight)
                + worst_case_score * ctx.multi_scenario_worst_weight
        } else {
            weighted_mean_score
        };
        stage_survival.push(weighted_outcome.time_alive_seconds);
        stage_damage.push(weighted_outcome.damage_dealt);
        stage_healing.push(weighted_outcome.healing_done);
        stage_objective_scores.push(stage_score);
        cumulative_score += stage_score;
    }
    BuildOrderResult {
        ordered_items: ordered_items.to_vec(),
        levels: levels.to_vec(),
        acquired_levels: levels.to_vec(),
        stage_survival,
        stage_damage,
        stage_healing,
        stage_objective_scores,
        cumulative_score,
    }
}

pub(super) fn optimize_build_order(
    build_items: &[Item],
    ctx: &BuildOrderEvalContext<'_>,
) -> BuildOrderResult {
    let levels = build_level_milestones(build_items.len(), 5, 20);
    let reference_outcomes_by_scenario = ctx
        .enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds)| {
            simulate_build_order_stage_outcomes_for_scenario(
                build_items,
                &levels,
                enemy_builds,
                ctx,
            )
        })
        .collect::<Vec<_>>();
    let mut best = score_build_order(build_items, &levels, &reference_outcomes_by_scenario, ctx);
    if build_items.len() <= 1 {
        return best;
    }

    let beam_width = 40usize;
    let mut frontier = vec![Vec::<usize>::new()];
    for depth in 0..build_items.len() {
        let mut expanded = Vec::<(Vec<usize>, f64)>::new();
        for partial in &frontier {
            let mut used = partial.iter().copied().collect::<HashSet<_>>();
            for idx in 0..build_items.len() {
                if used.contains(&idx) {
                    continue;
                }
                let mut candidate = partial.clone();
                candidate.push(idx);
                used.insert(idx);
                let ordered = candidate
                    .iter()
                    .map(|i| build_items[*i].clone())
                    .collect::<Vec<_>>();
                let partial_levels = levels[..candidate.len()].to_vec();
                let current = score_build_order(
                    &ordered,
                    &partial_levels,
                    &reference_outcomes_by_scenario,
                    ctx,
                );
                let optimistic_upper_bound = current.cumulative_score
                    + (build_items.len() - candidate.len()) as f64 * ctx.sim.max_time_seconds;
                expanded.push((candidate, optimistic_upper_bound));
            }
        }
        expanded.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        frontier = expanded
            .into_iter()
            .take(beam_width)
            .map(|(candidate, _)| candidate)
            .collect::<Vec<_>>();
        if frontier.is_empty() {
            break;
        }
        if depth + 1 == build_items.len() {
            for order_idx in &frontier {
                let ordered = order_idx
                    .iter()
                    .map(|i| build_items[*i].clone())
                    .collect::<Vec<_>>();
                let scored =
                    score_build_order(&ordered, &levels, &reference_outcomes_by_scenario, ctx);
                if scored.cumulative_score > best.cumulative_score {
                    best = scored;
                }
            }
        }
    }

    best
}

#[cfg(test)]
#[path = "tests/build_order_tests.rs"]
mod tests;

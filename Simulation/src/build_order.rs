use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use super::{
    BuildOrderEvalContext, BuildOrderResult, ChampionBase, CombatOutcome, EnemyConfig, Item, Stats,
    champion_at_level, objective_score_from_outcome, simulate_vlad_combat,
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
    level: usize,
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    raw_enemy_bases: &HashMap<String, ChampionBase>,
) -> Vec<(EnemyConfig, Vec<Item>, Stats)> {
    enemy_builds
        .iter()
        .map(|(enemy_cfg, build, bonus_stats)| {
            let raw_base = raw_enemy_bases
                .get(&enemy_cfg.name)
                .cloned()
                .unwrap_or_else(|| enemy_cfg.base.clone());
            let mut scaled_cfg = enemy_cfg.clone();
            scaled_cfg.base = champion_at_level(&raw_base, level);
            (scaled_cfg, build.clone(), bonus_stats.clone())
        })
        .collect()
}

fn simulate_build_order_stage_outcomes(
    ordered_items: &[Item],
    levels: &[usize],
    ctx: &BuildOrderEvalContext<'_>,
) -> Vec<CombatOutcome> {
    let mut stage_outcomes = Vec::with_capacity(levels.len());
    for (idx, level) in levels.iter().enumerate() {
        let prefix = &ordered_items[..=idx];
        let prefix_levels = &levels[..=idx];
        let acquired_map = acquisition_level_map(prefix, prefix_levels);
        let vlad_base_level = champion_at_level(ctx.vlad_base_raw, *level);
        let enemy_level_builds =
            level_scaled_enemy_builds(*level, ctx.enemy_builds, ctx.raw_enemy_bases);
        let mut sim_at_level = ctx.sim.clone();
        sim_at_level.champion_level = *level;
        let outcome = simulate_vlad_combat(
            &vlad_base_level,
            prefix,
            ctx.vlad_bonus_stats,
            Some(&acquired_map),
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
    reference_outcomes: &[CombatOutcome],
    ctx: &BuildOrderEvalContext<'_>,
) -> BuildOrderResult {
    let stage_outcomes = simulate_build_order_stage_outcomes(ordered_items, levels, ctx);
    let mut stage_survival = Vec::new();
    let mut stage_damage = Vec::new();
    let mut stage_healing = Vec::new();
    let mut stage_objective_scores = Vec::new();
    let mut cumulative_score = 0.0;
    for (idx, outcome) in stage_outcomes.iter().enumerate() {
        let stage_level = levels.get(idx).copied().unwrap_or(ctx.sim.champion_level);
        let mut sim_at_level = ctx.sim.clone();
        sim_at_level.champion_level = stage_level;
        let reference = reference_outcomes
            .get(idx)
            .copied()
            .unwrap_or(CombatOutcome {
                time_alive_seconds: sim_at_level.max_time_seconds.max(1.0),
                damage_dealt: 1.0,
                healing_done: 1.0,
                enemy_kills: 0,
            });
        let stage_score = objective_score_from_outcome(*outcome, reference, ctx.objective_weights);
        stage_survival.push(outcome.time_alive_seconds);
        stage_damage.push(outcome.damage_dealt);
        stage_healing.push(outcome.healing_done);
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
    let reference_outcomes = simulate_build_order_stage_outcomes(build_items, &levels, ctx);
    let mut best = score_build_order(build_items, &levels, &reference_outcomes, ctx);
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
                let reference_partial = &reference_outcomes[..candidate.len()];
                let current = score_build_order(&ordered, &partial_levels, reference_partial, ctx);
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
                let scored = score_build_order(&ordered, &levels, &reference_outcomes, ctx);
                if scored.cumulative_score > best.cumulative_score {
                    best = scored;
                }
            }
        }
    }

    best
}

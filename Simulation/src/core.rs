use std::collections::HashMap;

use crate::engine::simulate_vlad_combat;
use crate::scripts::hooks::{
    ChampionStatContext, ItemAssumptionContext, StackNoteContext, apply_item_assumption_hooks,
    finalize_champion_stats_with_hooks, stack_notes_from_hooks,
};

use super::{
    ChampionBase, CombatOutcome, Item, ObjectiveComponentWeights, ObjectiveEvalContext,
    SimulationConfig, Stats,
};

pub(crate) fn is_boots(item: &Item) -> bool {
    item.rank.iter().any(|r| r == "BOOTS")
}

pub(crate) fn cooldown_after_haste(base_seconds: f64, haste: f64) -> f64 {
    base_seconds * (100.0 / (100.0 + haste))
}

pub(crate) fn champion_at_level(base: &ChampionBase, level: usize) -> ChampionBase {
    let lvl = level.max(1) as f64;
    let growth_levels = (lvl - 1.0).max(0.0);
    ChampionBase {
        name: base.name.clone(),
        base_health: base.base_health + base.health_per_level * growth_levels,
        health_per_level: base.health_per_level,
        base_armor: base.base_armor + base.armor_per_level * growth_levels,
        armor_per_level: base.armor_per_level,
        base_magic_resist: base.base_magic_resist + base.magic_resist_per_level * growth_levels,
        magic_resist_per_level: base.magic_resist_per_level,
        base_attack_damage: base.base_attack_damage + base.attack_damage_per_level * growth_levels,
        attack_damage_per_level: base.attack_damage_per_level,
        base_attack_speed: base.base_attack_speed
            * (1.0 + (base.attack_speed_per_level_percent / 100.0) * growth_levels),
        attack_speed_per_level_percent: base.attack_speed_per_level_percent,
        base_move_speed: base.base_move_speed,
        is_melee: base.is_melee,
    }
}

pub(crate) fn apply_item_assumptions(
    stats: &mut Stats,
    base: &ChampionBase,
    build_items: &[Item],
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
) {
    let ctx = ItemAssumptionContext {
        champion: base,
        build_items,
        sim,
        current_level,
        acquired_levels,
    };
    apply_item_assumption_hooks(&ctx, stats);
}

pub(crate) fn compute_effective_item_stats_for_build(
    base: &ChampionBase,
    build_items: &[Item],
    bonus_stats: &Stats,
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
) -> Stats {
    let mut stats = build_item_stats(build_items);
    stats.add(bonus_stats);
    apply_item_assumptions(
        &mut stats,
        base,
        build_items,
        sim,
        current_level,
        acquired_levels,
    );
    stats
}

pub(crate) fn build_stack_notes(
    build_items: &[Item],
    base: &ChampionBase,
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
) -> Vec<String> {
    let mut notes = Vec::new();
    for item in build_items {
        let hook_ctx = StackNoteContext {
            champion: base,
            build_items,
            item,
            sim,
            current_level,
            acquired_levels,
        };
        let hook_notes = stack_notes_from_hooks(&hook_ctx);
        let has_explicit_item_note = !hook_notes.is_empty();
        notes.extend(hook_notes);

        if has_explicit_item_note {
            continue;
        }

        let has_stack_text = item
            .passive_effects_text
            .iter()
            .any(|t| t.to_ascii_lowercase().contains("stack"));
        if has_stack_text {
            notes.push(format!(
                "{} has stack-based passive text in item data; currently treated as baseline/implicit unless explicitly modeled.",
                item.name
            ));
        }
    }
    notes
}

pub(crate) fn compute_vlad_stats(base: &ChampionBase, item_stats: &Stats) -> Stats {
    let mut stats = item_stats.clone();
    let hook_ctx = ChampionStatContext {
        champion: base,
        item_stats,
    };
    finalize_champion_stats_with_hooks(&hook_ctx, &mut stats);
    stats.health += base.base_health;
    stats.armor += base.base_armor;
    stats.magic_resist += base.base_magic_resist;
    stats
}

pub(crate) fn normalized_objective_weights(
    survival: f64,
    damage: f64,
    healing: f64,
) -> ObjectiveComponentWeights {
    let mut s = survival.max(0.0);
    let mut d = damage.max(0.0);
    let mut h = healing.max(0.0);
    let sum = s + d + h;
    if sum <= 0.0 {
        s = 1.0;
        d = 0.0;
        h = 0.0;
    } else {
        s /= sum;
        d /= sum;
        h /= sum;
    }
    ObjectiveComponentWeights {
        survival: s,
        damage: d,
        healing: h,
    }
}

pub(crate) fn objective_score_from_outcome(
    outcome: CombatOutcome,
    reference: CombatOutcome,
    weights: ObjectiveComponentWeights,
) -> f64 {
    let survival_ref = reference.time_alive_seconds.max(0.01);
    let damage_ref = reference.damage_dealt.max(1.0);
    let healing_ref = reference.healing_done.max(1.0);
    weights.survival * (outcome.time_alive_seconds / survival_ref)
        + weights.damage * (outcome.damage_dealt / damage_ref)
        + weights.healing * (outcome.healing_done / healing_ref)
}

pub(crate) fn aggregate_objective_score_and_outcome(
    ctx: &ObjectiveEvalContext<'_>,
    build_items: &[Item],
    bonus_stats: &Stats,
) -> (f64, CombatOutcome) {
    let mut weighted_score_sum = 0.0;
    let mut weighted_time_sum = 0.0;
    let mut weighted_damage_sum = 0.0;
    let mut weighted_healing_sum = 0.0;
    let mut weighted_kills_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut worst = f64::INFINITY;

    for (idx, (_, weight, enemy_builds_s)) in ctx.enemy_build_scenarios.iter().enumerate() {
        let w = (*weight).max(0.0);
        if w <= 0.0 {
            continue;
        }
        let outcome = simulate_vlad_combat(
            ctx.vlad_base,
            build_items,
            bonus_stats,
            None,
            enemy_builds_s,
            ctx.sim,
            ctx.urf,
        );
        let reference =
            ctx.scenario_reference_outcomes
                .get(idx)
                .copied()
                .unwrap_or(CombatOutcome {
                    time_alive_seconds: ctx.sim.max_time_seconds.max(1.0),
                    damage_dealt: 1.0,
                    healing_done: 1.0,
                    enemy_kills: 0,
                });
        let scenario_score = objective_score_from_outcome(outcome, reference, ctx.weights);
        weighted_score_sum += w * scenario_score;
        weighted_time_sum += w * outcome.time_alive_seconds;
        weighted_damage_sum += w * outcome.damage_dealt;
        weighted_healing_sum += w * outcome.healing_done;
        weighted_kills_sum += w * outcome.enemy_kills as f64;
        weight_sum += w;
        worst = worst.min(scenario_score);
    }

    if weight_sum <= 0.0 {
        return (0.0, CombatOutcome::default());
    }

    let mean_score = weighted_score_sum / weight_sum;
    let blended_score = if worst.is_finite() {
        let ww = ctx.worst_case_weight.clamp(0.0, 1.0);
        (1.0 - ww) * mean_score + ww * worst
    } else {
        mean_score
    };
    let mean_outcome = CombatOutcome {
        time_alive_seconds: weighted_time_sum / weight_sum,
        damage_dealt: weighted_damage_sum / weight_sum,
        healing_done: weighted_healing_sum / weight_sum,
        enemy_kills: (weighted_kills_sum / weight_sum).round() as usize,
    };
    (blended_score, mean_outcome)
}

pub(crate) fn build_item_stats(items: &[Item]) -> Stats {
    let mut stats = Stats::default();
    for item in items {
        stats.add(&item.stats);
    }
    stats
}

pub(crate) fn build_from_indices(item_pool: &[Item], build: &[usize]) -> Vec<Item> {
    build.iter().map(|&idx| item_pool[idx].clone()).collect()
}

pub(crate) fn canonical_key(build: &[usize]) -> Vec<usize> {
    let mut key = build.to_vec();
    key.sort_unstable();
    key
}

pub(crate) fn next_u64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}

pub(crate) fn rand_index(seed: &mut u64, upper: usize) -> usize {
    if upper <= 1 {
        return 0;
    }
    (next_u64(seed) as usize) % upper
}

pub(crate) fn rand_f64(seed: &mut u64) -> f64 {
    let bits = next_u64(seed) >> 11;
    (bits as f64) / ((1u64 << 53) as f64)
}

pub(crate) fn shuffle_usize(slice: &mut [usize], seed: &mut u64) {
    if slice.len() <= 1 {
        return;
    }
    for i in (1..slice.len()).rev() {
        let j = rand_index(seed, i + 1);
        slice.swap(i, j);
    }
}

pub(crate) fn can_add_item_to_build(item_pool: &[Item], build: &[usize], item_idx: usize) -> bool {
    if build.contains(&item_idx) {
        return false;
    }
    if is_boots(&item_pool[item_idx]) && build.iter().any(|&i| is_boots(&item_pool[i])) {
        return false;
    }
    true
}

pub(crate) fn random_valid_build(
    item_pool: &[Item],
    max_items: usize,
    seed: &mut u64,
) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..item_pool.len()).collect();
    shuffle_usize(&mut indices, seed);
    let mut build = Vec::with_capacity(max_items);
    for item_idx in indices {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &build, item_idx) {
            build.push(item_idx);
        }
    }
    build
}

pub(crate) fn repair_build(
    item_pool: &[Item],
    build: &mut Vec<usize>,
    max_items: usize,
    seed: &mut u64,
) {
    let mut deduped = Vec::with_capacity(max_items);
    for &item_idx in build.iter() {
        if deduped.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &deduped, item_idx) {
            deduped.push(item_idx);
        }
    }
    *build = deduped;

    if build.len() >= max_items {
        return;
    }
    let mut all_indices: Vec<usize> = (0..item_pool.len()).collect();
    shuffle_usize(&mut all_indices, seed);
    for item_idx in all_indices {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, build, item_idx) {
            build.push(item_idx);
        }
    }
}

pub(crate) fn mean_std(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|v| {
            let d = *v - mean;
            d * d
        })
        .sum::<f64>()
        / values.len() as f64;
    (mean, var.sqrt())
}

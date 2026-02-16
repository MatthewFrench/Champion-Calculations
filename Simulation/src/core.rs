use std::collections::HashMap;

use crate::engine::simulate_vlad_combat;

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

pub(crate) fn assumed_heartsteel_bonus_health(base_max_health: f64, stacks_at_8m: f64) -> f64 {
    if stacks_at_8m <= 0.0 {
        return 0.0;
    }
    // Approximate permanent health gained by repeatedly proccing Heartsteel:
    // per proc ~= 8% * (70 + 6% max_health) = 5.6 + 0.0048 * max_health.
    // Use an iterative approximation because max_health grows as stacks are gained.
    let procs = stacks_at_8m.max(0.0).round() as usize;
    let mut max_health = base_max_health;
    let mut gained = 0.0;
    for _ in 0..procs {
        let delta = 5.6 + 0.0048 * max_health;
        gained += delta;
        max_health += delta;
    }
    gained
}

pub(crate) fn assumed_heartsteel_stacks_by_level(
    full_stacks_at_level_20: f64,
    acquired_level: usize,
    current_level: usize,
) -> f64 {
    let ref_start: f64 = 5.0;
    let ref_end: f64 = 20.0;
    let acquired = acquired_level as f64;
    let current = current_level as f64;
    let elapsed = (current - acquired).max(0.0);
    let reference_window = (ref_end - ref_start).max(1.0_f64);
    (full_stacks_at_level_20 * (elapsed / reference_window)).clamp(0.0, full_stacks_at_level_20)
}

pub(crate) fn get_item_acquired_level(
    build_items: &[Item],
    item_name: &str,
    acquired_levels: Option<&HashMap<String, usize>>,
    default_level: usize,
) -> usize {
    if build_items.iter().any(|i| i.name == item_name) {
        if let Some(map) = acquired_levels
            && let Some(level) = map.get(item_name)
        {
            return *level;
        }
        return default_level;
    }
    default_level
}

pub(crate) fn apply_item_assumptions(
    stats: &mut Stats,
    base: &ChampionBase,
    build_items: &[Item],
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
) {
    if build_items.iter().any(|i| i.name == "Heartsteel") {
        let acquired_level = get_item_acquired_level(build_items, "Heartsteel", acquired_levels, 5);
        let stacks = assumed_heartsteel_stacks_by_level(
            sim.heartsteel_assumed_stacks_at_8m,
            acquired_level,
            current_level,
        );
        let base_max_health = base.base_health + stats.health;
        stats.health += assumed_heartsteel_bonus_health(base_max_health, stacks);
    }
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
        if item.name == "Heartsteel" {
            let mut base_stats = build_item_stats(build_items);
            // Remove Heartsteel's own flat health so the estimate is anchored to pre-heartsteel max HP.
            base_stats.health -= item.stats.health;
            let base_max_hp = base.base_health + base_stats.health.max(0.0);
            let acquired_level =
                get_item_acquired_level(build_items, "Heartsteel", acquired_levels, 5);
            let stacks = assumed_heartsteel_stacks_by_level(
                sim.heartsteel_assumed_stacks_at_8m,
                acquired_level,
                current_level,
            );
            let bonus = assumed_heartsteel_bonus_health(base_max_hp, stacks);
            notes.push(format!(
                "Heartsteel estimated stacks by level {}: {:.1} (acquired at level {}, reference full-at-20 stack target {:.0}, estimated permanent bonus health: +{:.1}).",
                current_level, stacks, acquired_level, sim.heartsteel_assumed_stacks_at_8m, bonus
            ));
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
    let ap_items = item_stats.ability_power;
    let bonus_health_items = item_stats.health;
    // Crimson Pact should not self-recursively amplify:
    // - AP gained from bonus health does not grant extra health again
    // - Health gained from AP does not grant extra AP again
    let bonus_health = bonus_health_items + 1.6 * ap_items;
    let ability_power = ap_items + 0.033 * bonus_health_items;

    let mut stats = Stats {
        ability_power,
        health: bonus_health,
        armor: item_stats.armor,
        magic_resist: item_stats.magic_resist,
        attack_damage: item_stats.attack_damage,
        attack_speed_percent: item_stats.attack_speed_percent,
        ability_haste: item_stats.ability_haste,
        move_speed_flat: item_stats.move_speed_flat,
        move_speed_percent: item_stats.move_speed_percent,
        crit_chance_percent: item_stats.crit_chance_percent,
    };
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

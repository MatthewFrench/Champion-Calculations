use std::collections::HashMap;

use crate::scripts::registry::hooks::{
    ChampionStatContext, ItemAssumptionContext, StackNoteContext, apply_item_assumption_hooks,
    finalize_champion_stats_with_hooks, stack_notes_from_hooks,
};

use super::{
    BuildKey, ChampionBase, CombatOutcome, Item, LoadoutSelection, ObjectiveComponentImpact,
    ObjectiveComponentWeights, ObjectiveEvalContext, ObjectiveScoreBreakdown, SimulationConfig,
    Stats, loadout_selection_key,
};

mod build_candidate_random_helpers;
mod combat_primitives_state;
mod objective_scoring_math;

#[cfg(test)]
pub(crate) use self::combat_primitives_state::{CastLockPhase, CastLockState, StatusEffectSet};
pub(crate) use self::combat_primitives_state::{
    CastLockWindow, CombatPrimitivesState, StatusDuration, StatusEffect, StatusEffectKind,
    StatusPersistence,
};

pub(crate) use self::build_candidate_random_helpers::{
    build_from_indices, build_item_stats, build_key_cache_string, can_add_item_to_build,
    canonical_build_candidate, canonical_key, is_boots, mean_std, rand_f64, rand_index,
    random_valid_build, repair_build, runtime_random_seed, shuffle_usize,
};
pub(crate) use self::objective_scoring_math::{
    aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection,
    aggregate_objective_score_and_outcome_with_loadout_selection, normalized_objective_weights,
    objective_score_from_outcome,
};

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
        base_attack_range: base.base_attack_range,
        base_attack_projectile_speed: base.base_attack_projectile_speed,
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
    stack_overrides: Option<&HashMap<String, f64>>,
) {
    let ctx = ItemAssumptionContext {
        champion: base,
        build_items,
        sim,
        current_level,
        acquired_levels,
        stack_overrides,
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
    stack_overrides: Option<&HashMap<String, f64>>,
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
        stack_overrides,
    );
    stats
}

pub(crate) fn build_stack_notes(
    build_items: &[Item],
    base: &ChampionBase,
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
    stack_overrides: Option<&HashMap<String, f64>>,
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
            stack_overrides,
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
                "{} has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.",
                item.name
            ));
        }
    }
    notes
}

pub(crate) fn compute_champion_final_stats(base: &ChampionBase, item_stats: &Stats) -> Stats {
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

#[cfg(test)]
#[path = "tests/core_tests.rs"]
mod tests;

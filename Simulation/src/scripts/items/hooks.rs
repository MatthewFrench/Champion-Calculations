use crate::{Item, Stats, to_norm_key};

use crate::scripts::registry::hooks::{ItemAssumptionContext, ScriptHook, StackNoteContext};

pub(crate) struct ItemHook;

pub(crate) const ITEM_HOOK: ItemHook = ItemHook;
const HEARTSTEEL_STACK_IDENTIFIER: &str = "heartsteel";

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionDefensiveItemCapabilities {
    pub has_stasis_item: bool,
    pub has_revive_item: bool,
    pub has_emergency_shield_item: bool,
}

pub(crate) fn controlled_champion_defensive_item_capabilities(
    build_items: &[Item],
) -> ControlledChampionDefensiveItemCapabilities {
    let mut capabilities = ControlledChampionDefensiveItemCapabilities::default();
    for item in build_items {
        match item.name.as_str() {
            "Zhonya's Hourglass" => capabilities.has_stasis_item = true,
            "Guardian Angel" => capabilities.has_revive_item = true,
            "Protoplasm Harness" => capabilities.has_emergency_shield_item = true,
            _ => {}
        }
    }
    capabilities
}

impl ScriptHook for ItemHook {
    fn apply_item_assumptions(&self, ctx: &ItemAssumptionContext<'_>, stats: &mut Stats) {
        if ctx.build_items.iter().any(|i| i.name == "Heartsteel") {
            let acquired_level =
                get_item_acquired_level(ctx.build_items, "Heartsteel", ctx.acquired_levels, 5);
            let Some(reference_full_stacks) =
                stack_override_value(ctx, HEARTSTEEL_STACK_IDENTIFIER)
            else {
                return;
            };
            let stacks = heartsteel_stacks_by_level(
                reference_full_stacks,
                acquired_level,
                ctx.current_level,
            );
            let base_max_health = ctx.champion.base_health + stats.health;
            stats.health += heartsteel_bonus_health(base_max_health, stacks);
        }
    }

    fn stack_note_for(&self, ctx: &StackNoteContext<'_>) -> Option<String> {
        if ctx.item.name != "Heartsteel" {
            return None;
        }

        let mut pre_heartsteel_health = 0.0;
        for build_item in ctx.build_items {
            if build_item.name == "Heartsteel" {
                continue;
            }
            pre_heartsteel_health += build_item.stats.health;
        }

        let base_max_hp = ctx.champion.base_health + pre_heartsteel_health.max(0.0);
        let acquired_level =
            get_item_acquired_level(ctx.build_items, "Heartsteel", ctx.acquired_levels, 5);
        let reference_full_stacks =
            stack_override_value_for_note(ctx, HEARTSTEEL_STACK_IDENTIFIER)?;
        let stacks =
            heartsteel_stacks_by_level(reference_full_stacks, acquired_level, ctx.current_level);
        let bonus = heartsteel_bonus_health(base_max_hp, stacks);

        Some(format!(
            "Heartsteel estimated stacks by level {}: {:.1} (acquired at level {}, reference full-at-20 stack target {:.0}, estimated permanent bonus health: +{:.1}).",
            ctx.current_level, stacks, acquired_level, reference_full_stacks, bonus
        ))
    }
}

fn stack_assumption_from_map(
    map: &std::collections::HashMap<String, f64>,
    stack_identifier: &str,
) -> Option<f64> {
    if let Some(value) = map.get(stack_identifier) {
        return Some((*value).max(0.0));
    }
    let key = to_norm_key(stack_identifier);
    map.iter()
        .find(|(name, _)| to_norm_key(name) == key)
        .map(|(_, value)| (*value).max(0.0))
}

fn stack_override_value(ctx: &ItemAssumptionContext<'_>, stack_identifier: &str) -> Option<f64> {
    if let Some(overrides) = ctx.stack_overrides
        && let Some(value) = stack_assumption_from_map(overrides, stack_identifier)
    {
        return Some(value);
    }
    stack_assumption_from_map(&ctx.sim.stack_overrides, stack_identifier)
}

fn stack_override_value_for_note(
    ctx: &StackNoteContext<'_>,
    stack_identifier: &str,
) -> Option<f64> {
    if let Some(overrides) = ctx.stack_overrides
        && let Some(value) = stack_assumption_from_map(overrides, stack_identifier)
    {
        return Some(value);
    }
    stack_assumption_from_map(&ctx.sim.stack_overrides, stack_identifier)
}

fn get_item_acquired_level(
    build_items: &[Item],
    item_name: &str,
    acquired_levels: Option<&std::collections::HashMap<String, usize>>,
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

fn heartsteel_bonus_health(base_max_health: f64, stacks_at_8m: f64) -> f64 {
    if stacks_at_8m <= 0.0 {
        return 0.0;
    }
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

fn heartsteel_stacks_by_level(
    reference_full_stacks: f64,
    acquired_level: usize,
    current_level: usize,
) -> f64 {
    let reference_start: f64 = 5.0;
    let reference_end: f64 = 20.0;
    let acquired = acquired_level as f64;
    let current = current_level as f64;
    let elapsed = (current - acquired).max(0.0);
    let reference_window = (reference_end - reference_start).max(1.0_f64);
    (reference_full_stacks * (elapsed / reference_window)).clamp(0.0, reference_full_stacks)
}

#[cfg(test)]
#[path = "tests/hooks_tests.rs"]
mod tests;

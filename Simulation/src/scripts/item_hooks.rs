use crate::{Item, Stats};

use super::hooks::{ItemAssumptionContext, ScriptHook, StackNoteContext};

pub(crate) struct ItemHook;

pub(crate) const ITEM_HOOK: ItemHook = ItemHook;

impl ScriptHook for ItemHook {
    fn apply_item_assumptions(&self, ctx: &ItemAssumptionContext<'_>, stats: &mut Stats) {
        if ctx.build_items.iter().any(|i| i.name == "Heartsteel") {
            let acquired_level =
                get_item_acquired_level(ctx.build_items, "Heartsteel", ctx.acquired_levels, 5);
            let stacks = heartsteel_stacks_by_level(
                ctx.sim.heartsteel_assumed_stacks_at_8m,
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
        let stacks = heartsteel_stacks_by_level(
            ctx.sim.heartsteel_assumed_stacks_at_8m,
            acquired_level,
            ctx.current_level,
        );
        let bonus = heartsteel_bonus_health(base_max_hp, stacks);

        Some(format!(
            "Heartsteel estimated stacks by level {}: {:.1} (acquired at level {}, reference full-at-20 stack target {:.0}, estimated permanent bonus health: +{:.1}).",
            ctx.current_level,
            stacks,
            acquired_level,
            ctx.sim.heartsteel_assumed_stacks_at_8m,
            bonus
        ))
    }
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
    full_stacks_at_level_20: f64,
    acquired_level: usize,
    current_level: usize,
) -> f64 {
    let reference_start: f64 = 5.0;
    let reference_end: f64 = 20.0;
    let acquired = acquired_level as f64;
    let current = current_level as f64;
    let elapsed = (current - acquired).max(0.0);
    let reference_window = (reference_end - reference_start).max(1.0_f64);
    (full_stacks_at_level_20 * (elapsed / reference_window)).clamp(0.0, full_stacks_at_level_20)
}

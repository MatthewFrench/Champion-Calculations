use anyhow::Result;
use std::collections::HashMap;

use crate::scripts::champions::vladimir::VLADIMIR_HOOK;
use crate::scripts::items::hooks::ITEM_HOOK;
use crate::scripts::runtime::controlled_champion_loadout::CONTROLLED_CHAMPION_LOADOUT_HOOK;
use crate::{ChampionBase, Item, LoadoutSelection, ResolvedLoadout, SimulationConfig, Stats};

pub(crate) struct ItemAssumptionContext<'a> {
    pub champion: &'a ChampionBase,
    pub build_items: &'a [Item],
    pub sim: &'a SimulationConfig,
    pub current_level: usize,
    pub acquired_levels: Option<&'a HashMap<String, usize>>,
    pub stack_overrides: Option<&'a HashMap<String, f64>>,
}

pub(crate) struct StackNoteContext<'a> {
    pub champion: &'a ChampionBase,
    pub build_items: &'a [Item],
    pub item: &'a Item,
    pub sim: &'a SimulationConfig,
    pub current_level: usize,
    pub acquired_levels: Option<&'a HashMap<String, usize>>,
    pub stack_overrides: Option<&'a HashMap<String, f64>>,
}

pub(crate) struct ChampionStatContext<'a> {
    pub champion: &'a ChampionBase,
    pub item_stats: &'a Stats,
}

pub(crate) struct LoadoutHookContext<'a> {
    pub selection: &'a LoadoutSelection,
    pub level: usize,
    pub for_controlled_champion: bool,
}

pub(crate) trait ScriptHook: Sync {
    fn apply_item_assumptions(&self, _ctx: &ItemAssumptionContext<'_>, _stats: &mut Stats) {}

    fn stack_note_for(&self, _ctx: &StackNoteContext<'_>) -> Option<String> {
        None
    }

    fn finalize_champion_stats(&self, _ctx: &ChampionStatContext<'_>, _stats: &mut Stats) {}

    fn resolve_loadout(
        &self,
        _ctx: &LoadoutHookContext<'_>,
        _resolved: &mut ResolvedLoadout,
    ) -> Result<()> {
        Ok(())
    }
}

fn registry() -> [&'static dyn ScriptHook; 3] {
    [
        &VLADIMIR_HOOK,
        &ITEM_HOOK,
        &CONTROLLED_CHAMPION_LOADOUT_HOOK,
    ]
}

pub(crate) fn apply_item_assumption_hooks(ctx: &ItemAssumptionContext<'_>, stats: &mut Stats) {
    for hook in registry() {
        hook.apply_item_assumptions(ctx, stats);
    }
}

pub(crate) fn stack_notes_from_hooks(ctx: &StackNoteContext<'_>) -> Vec<String> {
    let mut notes = Vec::new();
    for hook in registry() {
        if let Some(note) = hook.stack_note_for(ctx)
            && !note.is_empty()
        {
            notes.push(note);
        }
    }
    notes
}

pub(crate) fn finalize_champion_stats_with_hooks(ctx: &ChampionStatContext<'_>, stats: &mut Stats) {
    for hook in registry() {
        hook.finalize_champion_stats(ctx, stats);
    }
}

pub(crate) fn resolve_loadout_with_hooks(
    ctx: &LoadoutHookContext<'_>,
    resolved: &mut ResolvedLoadout,
) -> Result<()> {
    for hook in registry() {
        hook.resolve_loadout(ctx, resolved)?;
    }
    Ok(())
}

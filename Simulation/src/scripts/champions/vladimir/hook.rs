use anyhow::Result;

use crate::{ResolvedLoadout, Stats, to_norm_key};

use crate::scripts::registry::hooks::{ChampionStatContext, LoadoutHookContext, ScriptHook};

pub(crate) struct VladimirHook;

pub(crate) const VLADIMIR_HOOK: VladimirHook = VladimirHook;

impl ScriptHook for VladimirHook {
    fn finalize_champion_stats(&self, ctx: &ChampionStatContext<'_>, stats: &mut Stats) {
        if to_norm_key(&ctx.champion.name) != "vladimir" {
            return;
        }

        let ability_power_from_items = ctx.item_stats.ability_power;
        let bonus_health_from_items = ctx.item_stats.health;

        // Crimson Pact conversion applied once (no recursive feedback loop).
        stats.health = bonus_health_from_items + 1.6 * ability_power_from_items;
        stats.ability_power = ability_power_from_items + 0.033 * bonus_health_from_items;
    }

    fn resolve_loadout(
        &self,
        ctx: &LoadoutHookContext<'_>,
        resolved: &mut ResolvedLoadout,
    ) -> Result<()> {
        if !ctx.for_controlled_champion {
            return Ok(());
        }

        for rune_name in &ctx.selection.rune_names {
            let rune_key = to_norm_key(rune_name);
            if rune_key == "phase rush" || rune_key == "phaserush" {
                resolved.skipped_notes.push(
                    "Phase Rush movement scripting is not yet modeled in combat-time movement."
                        .to_string(),
                );
            }
        }

        Ok(())
    }
}

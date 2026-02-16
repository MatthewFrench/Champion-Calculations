use anyhow::Result;

use crate::to_norm_key;

use super::hooks::{LoadoutHookContext, ScriptHook};

pub(crate) struct LoadoutHook;

pub(crate) const LOADOUT_HOOK: LoadoutHook = LoadoutHook;

impl ScriptHook for LoadoutHook {
    fn resolve_loadout(
        &self,
        ctx: &LoadoutHookContext<'_>,
        resolved: &mut crate::ResolvedLoadout,
    ) -> Result<()> {
        let dynamic_runes = [
            "graspoftheundying",
            "lethaltempo",
            "arcanecomet",
            "summonaery",
            "triumph",
            "gatheringstorm",
        ];
        let dynamic_masteries = [
            "fervorofbattle",
            "thunderlordsdecree",
            "windspeakersblessing",
            "legendaryguardian",
            "perseverance",
        ];

        for rune in &ctx.selection.rune_names {
            let key = to_norm_key(rune);
            if dynamic_runes.contains(&key.as_str()) {
                resolved.skipped_notes.push(format!(
                    "Rune '{}' has a combat-time script effect and is not fully represented as static pre-fight stats at level {}.",
                    rune,
                    ctx.level
                ));
            }
        }

        for mastery in &ctx.selection.masteries {
            let key = to_norm_key(&mastery.name);
            if dynamic_masteries.contains(&key.as_str()) {
                resolved.skipped_notes.push(format!(
                    "Mastery '{}' has a combat-time script effect and is not fully represented as static pre-fight stats at level {}.",
                    mastery.name,
                    ctx.level
                ));
            }
        }

        Ok(())
    }
}

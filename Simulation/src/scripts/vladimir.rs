use anyhow::Result;

use crate::{ResolvedLoadout, Stats, cooldown_after_haste, to_norm_key};

use super::hooks::{ChampionStatContext, LoadoutHookContext, ScriptHook};

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirAbilityTuning {
    pub q_base_damage: f64,
    pub q_ap_ratio: f64,
    pub q_heal_ratio_of_damage: f64,
    pub q_base_cooldown_seconds: f64,
    pub e_base_damage: f64,
    pub e_ap_ratio: f64,
    pub e_base_cooldown_seconds: f64,
    pub r_base_damage: f64,
    pub r_ap_ratio: f64,
    pub r_base_cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirAbilityCooldowns {
    pub q_seconds: f64,
    pub e_seconds: f64,
    pub r_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirCastProfile {
    pub q_range: f64,
    pub q_windup_seconds: f64,
    pub q_projectile_speed: f64,
    pub e_range: f64,
    pub e_windup_seconds: f64,
    pub e_projectile_speed: f64,
    pub r_range: f64,
    pub r_windup_seconds: f64,
    pub r_projectile_speed: f64,
}

pub(crate) fn offensive_cooldowns_after_haste(
    tuning: VladimirAbilityTuning,
    ability_haste: f64,
) -> VladimirAbilityCooldowns {
    VladimirAbilityCooldowns {
        q_seconds: cooldown_after_haste(tuning.q_base_cooldown_seconds, ability_haste),
        e_seconds: cooldown_after_haste(tuning.e_base_cooldown_seconds, ability_haste),
        r_seconds: cooldown_after_haste(tuning.r_base_cooldown_seconds, ability_haste),
    }
}

pub(crate) fn default_cast_profile() -> VladimirCastProfile {
    VladimirCastProfile {
        q_range: 600.0,
        q_windup_seconds: 0.20,
        q_projectile_speed: 0.0,
        e_range: 600.0,
        e_windup_seconds: 0.30,
        e_projectile_speed: 0.0,
        r_range: 700.0,
        r_windup_seconds: 0.25,
        r_projectile_speed: 0.0,
    }
}

pub(crate) fn q_damage_raw(tuning: VladimirAbilityTuning, ability_power: f64) -> f64 {
    tuning.q_base_damage + tuning.q_ap_ratio * ability_power
}

pub(crate) fn e_damage_raw(tuning: VladimirAbilityTuning, ability_power: f64) -> f64 {
    tuning.e_base_damage + tuning.e_ap_ratio * ability_power
}

pub(crate) fn r_damage_raw(tuning: VladimirAbilityTuning, ability_power: f64) -> f64 {
    tuning.r_base_damage + tuning.r_ap_ratio * ability_power
}

pub(crate) struct VladimirHook;

pub(crate) const VLADIMIR_HOOK: VladimirHook = VladimirHook;

impl ScriptHook for VladimirHook {
    fn finalize_champion_stats(&self, ctx: &ChampionStatContext<'_>, stats: &mut Stats) {
        if to_norm_key(&ctx.champion.name) != "vladimir" {
            return;
        }

        let ap_items = ctx.item_stats.ability_power;
        let bonus_health_items = ctx.item_stats.health;

        // Crimson Pact conversion applied once (no recursive feedback loop).
        stats.health = bonus_health_items + 1.6 * ap_items;
        stats.ability_power = ap_items + 0.033 * bonus_health_items;
    }

    fn resolve_loadout(
        &self,
        ctx: &LoadoutHookContext<'_>,
        resolved: &mut ResolvedLoadout,
    ) -> Result<()> {
        if !ctx.for_vlad {
            return Ok(());
        }

        for rune in &ctx.selection.rune_names {
            let key = to_norm_key(rune);
            if key == "phase rush" || key == "phaserush" {
                resolved.skipped_notes.push(
                    "Phase Rush movement scripting is not yet modeled in combat-time movement."
                        .to_string(),
                );
            }
        }

        Ok(())
    }
}

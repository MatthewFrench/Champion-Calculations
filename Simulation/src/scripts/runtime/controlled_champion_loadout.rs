use anyhow::Result;

use crate::LoadoutSelection;

use crate::scripts::masteries::effects::{apply_mastery_runtime_flag, has_dynamic_mastery_effect};
use crate::scripts::registry::hooks::{LoadoutHookContext, ScriptHook};
use crate::scripts::runes::effects::{apply_rune_runtime_flag, has_dynamic_rune_effect};

pub(crate) struct ControlledChampionLoadoutHook;

pub(crate) const CONTROLLED_CHAMPION_LOADOUT_HOOK: ControlledChampionLoadoutHook =
    ControlledChampionLoadoutHook;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ControlledChampionAbilityRuntimeInput {
    pub raw_magic_damage: f64,
    pub ability_power: f64,
    pub ability_ap_ratio: f64,
    pub now_seconds: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionAbilityRuntimeBonus {
    pub extra_magic_damage: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionLoadoutRuntime {
    pub(crate) has_arcane_comet: bool,
    pub(crate) has_summon_aery: bool,
    pub(crate) has_triumph: bool,
    pub(crate) has_gathering_storm: bool,
    pub(crate) has_fervor: bool,
    pub(crate) has_thunderlords: bool,
    pub(crate) has_windspeakers_blessing: bool,
    pub(crate) has_legendary_guardian: bool,
    pub(crate) has_perseverance: bool,

    pub fervor_stacks: usize,
    pub thunderlords_stacks: usize,
    pub arcane_comet_ready_at: f64,
    pub aery_ready_at: f64,
}

pub(crate) fn build_controlled_champion_loadout_runtime(
    selection: &LoadoutSelection,
) -> ControlledChampionLoadoutRuntime {
    let mut runtime = ControlledChampionLoadoutRuntime::default();
    for rune_name in &selection.rune_names {
        apply_rune_runtime_flag(&mut runtime, rune_name);
    }
    for mastery in &selection.masteries {
        apply_mastery_runtime_flag(&mut runtime, mastery);
    }
    runtime
}

pub(crate) fn on_controlled_champion_ability_bonus(
    runtime: &mut ControlledChampionLoadoutRuntime,
    input: ControlledChampionAbilityRuntimeInput,
) -> ControlledChampionAbilityRuntimeBonus {
    let mut extra_magic_damage = 0.0;

    if runtime.has_fervor {
        runtime.fervor_stacks = (runtime.fervor_stacks + 1).min(8);
        extra_magic_damage += 0.01 * runtime.fervor_stacks as f64 * input.raw_magic_damage.max(0.0);
    }

    if runtime.has_thunderlords {
        runtime.thunderlords_stacks += 1;
        if runtime.thunderlords_stacks >= 3 {
            extra_magic_damage += 35.0 + 0.22 * input.ability_power.max(0.0);
            runtime.thunderlords_stacks = 0;
        }
    }

    if runtime.has_arcane_comet && input.now_seconds >= runtime.arcane_comet_ready_at {
        extra_magic_damage += 30.0 + 0.20 * input.ability_power.max(0.0);
        runtime.arcane_comet_ready_at = input.now_seconds + 9.0;
    }

    if runtime.has_summon_aery && input.now_seconds >= runtime.aery_ready_at {
        extra_magic_damage += 12.0 + 0.10 * input.ability_power.max(0.0);
        runtime.aery_ready_at = input.now_seconds + 2.0;
    }

    if runtime.has_gathering_storm {
        // Approximate Gathering Storm as a deterministic AP step every 10 minutes.
        let steps = (input.now_seconds / 600.0).floor().max(0.0);
        let bonus_ability_power = 8.0 * steps;
        extra_magic_damage += input.ability_ap_ratio.max(0.0) * bonus_ability_power;
    }

    ControlledChampionAbilityRuntimeBonus {
        extra_magic_damage: extra_magic_damage.max(0.0),
    }
}

pub(crate) fn on_controlled_champion_enemy_kill(
    runtime: &mut ControlledChampionLoadoutRuntime,
    max_health: f64,
) -> f64 {
    if !runtime.has_triumph || max_health <= 0.0 {
        return 0.0;
    }
    0.08 * max_health
}

pub(crate) fn tick_controlled_champion_regen_heal(
    runtime: &ControlledChampionLoadoutRuntime,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    if !runtime.has_perseverance || max_health <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let health_ratio = (current_health / max_health).clamp(0.0, 1.0);
    let base_regen = 0.0012 * max_health * dt;
    let missing_health_bonus = if health_ratio <= 0.35 {
        0.0026 * max_health * dt
    } else {
        0.0
    };
    base_regen + missing_health_bonus
}

pub(crate) fn controlled_champion_heal_multiplier(
    runtime: &ControlledChampionLoadoutRuntime,
) -> f64 {
    if runtime.has_windspeakers_blessing {
        1.10
    } else {
        1.0
    }
}

pub(crate) fn controlled_champion_damage_taken_multiplier(
    runtime: &ControlledChampionLoadoutRuntime,
    nearby_enemies: usize,
) -> f64 {
    if !runtime.has_legendary_guardian {
        return 1.0;
    }
    let enemies = nearby_enemies.min(5) as f64;
    (1.0 - 0.015 * enemies).clamp(0.85, 1.0)
}

impl ScriptHook for ControlledChampionLoadoutHook {
    fn resolve_loadout(
        &self,
        ctx: &LoadoutHookContext<'_>,
        resolved: &mut crate::ResolvedLoadout,
    ) -> Result<()> {
        if !ctx.for_controlled_champion {
            return Ok(());
        }

        for rune in &ctx.selection.rune_names {
            if has_dynamic_rune_effect(rune) {
                resolved.skipped_notes.push(format!(
                    "Rune '{}' has a combat-time script effect and is not fully represented as static pre-fight stats at level {}.",
                    rune,
                    ctx.level
                ));
            }
        }

        for mastery in &ctx.selection.masteries {
            if has_dynamic_mastery_effect(&mastery.name) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MasterySelection;

    fn selection_with(rune_names: &[&str], mastery_names: &[&str]) -> LoadoutSelection {
        LoadoutSelection {
            rune_ids: Vec::new(),
            rune_names: rune_names.iter().map(|s| (*s).to_string()).collect(),
            shard_stats: Vec::new(),
            masteries: mastery_names
                .iter()
                .map(|name| MasterySelection {
                    name: (*name).to_string(),
                    rank: 1,
                })
                .collect(),
        }
    }

    #[test]
    fn controlled_champion_runtime_parses_dynamic_runes_and_masteries() {
        let runtime = build_controlled_champion_loadout_runtime(&selection_with(
            &["Arcane Comet", "Summon Aery", "Triumph"],
            &["Fervor of Battle", "Perseverance"],
        ));
        assert!(runtime.has_arcane_comet);
        assert!(runtime.has_summon_aery);
        assert!(runtime.has_triumph);
        assert!(runtime.has_fervor);
        assert!(runtime.has_perseverance);
    }

    #[test]
    fn arcane_comet_and_aery_respect_runtime_cooldowns() {
        let mut runtime = build_controlled_champion_loadout_runtime(&selection_with(
            &["Arcane Comet", "Summon Aery"],
            &[],
        ));

        let first = on_controlled_champion_ability_bonus(
            &mut runtime,
            ControlledChampionAbilityRuntimeInput {
                raw_magic_damage: 200.0,
                ability_power: 300.0,
                ability_ap_ratio: 0.6,
                now_seconds: 1.0,
            },
        );
        let second = on_controlled_champion_ability_bonus(
            &mut runtime,
            ControlledChampionAbilityRuntimeInput {
                raw_magic_damage: 200.0,
                ability_power: 300.0,
                ability_ap_ratio: 0.6,
                now_seconds: 1.5,
            },
        );
        assert!(first.extra_magic_damage > second.extra_magic_damage);
    }

    #[test]
    fn perseverance_regen_gives_more_heal_at_low_health() {
        let runtime =
            build_controlled_champion_loadout_runtime(&selection_with(&[], &["Perseverance"]));
        let high = tick_controlled_champion_regen_heal(&runtime, 1800.0, 2000.0, 1.0);
        let low = tick_controlled_champion_regen_heal(&runtime, 500.0, 2000.0, 1.0);
        assert!(low > high);
    }
}

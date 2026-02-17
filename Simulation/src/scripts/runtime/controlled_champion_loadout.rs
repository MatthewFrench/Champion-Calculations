use anyhow::Result;

use crate::LoadoutSelection;

use crate::scripts::registry::hooks::{LoadoutHookContext, ScriptHook};
use crate::scripts::runes::effects::{apply_rune_runtime_flag, has_dynamic_rune_effect};
use crate::scripts::runtime::stat_resolution::{
    RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
};

pub(crate) struct ControlledChampionLoadoutHook;

pub(crate) const CONTROLLED_CHAMPION_LOADOUT_HOOK: ControlledChampionLoadoutHook =
    ControlledChampionLoadoutHook;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ControlledChampionAbilityRuntimeInput {
    pub ability_power: f64,
    pub ability_ap_ratio: f64,
    pub now_seconds: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionAbilityRuntimeBonus {
    pub extra_magic_damage: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DefensiveItemActivationInput {
    pub now_seconds: f64,
    pub can_cast: bool,
    pub health: f64,
    pub max_health: f64,
    pub stasis_available: bool,
    pub stasis_ready_at: f64,
    pub stasis_trigger_health_percent: f64,
    pub untargetable_active_until: f64,
    pub revive_lock_active_until: f64,
    pub emergency_shield_available: bool,
    pub emergency_shield_ready_at: f64,
    pub emergency_shield_trigger_health_percent: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct DefensiveItemActivationDecisions {
    pub activate_stasis: bool,
    pub activate_emergency_shield: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ReviveEffectDecisionInput {
    pub available: bool,
    pub now_seconds: f64,
    pub ready_at: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionLoadoutRuntime {
    pub(crate) has_arcane_comet: bool,
    pub(crate) has_summon_aery: bool,
    pub(crate) has_triumph: bool,
    pub(crate) has_gathering_storm: bool,
    pub(crate) has_second_wind: bool,

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
    runtime
}

pub(crate) fn on_controlled_champion_ability_bonus(
    runtime: &mut ControlledChampionLoadoutRuntime,
    input: ControlledChampionAbilityRuntimeInput,
) -> ControlledChampionAbilityRuntimeBonus {
    let mut extra_magic_damage = 0.0;

    if runtime.has_arcane_comet && input.now_seconds >= runtime.arcane_comet_ready_at {
        extra_magic_damage += 30.0 + 0.20 * input.ability_power.max(0.0);
        runtime.arcane_comet_ready_at = input.now_seconds + 9.0;
    }

    if runtime.has_summon_aery && input.now_seconds >= runtime.aery_ready_at {
        extra_magic_damage += 12.0 + 0.10 * input.ability_power.max(0.0);
        runtime.aery_ready_at = input.now_seconds + 2.0;
    }

    if runtime.has_gathering_storm {
        // Approximate Gathering Storm as deterministic AP growth every 10 minutes.
        let steps = (input.now_seconds / 600.0).floor().max(0.0);
        let bonus_ability_power = 8.0 * steps;
        extra_magic_damage += input.ability_ap_ratio.max(0.0) * bonus_ability_power;
    }

    ControlledChampionAbilityRuntimeBonus {
        extra_magic_damage: resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_magic_damage,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
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

pub(crate) fn decide_defensive_item_activations(
    input: DefensiveItemActivationInput,
) -> DefensiveItemActivationDecisions {
    DefensiveItemActivationDecisions {
        activate_stasis: input.can_cast
            && input.stasis_available
            && input.now_seconds >= input.stasis_ready_at
            && input.health <= input.max_health * input.stasis_trigger_health_percent
            && input.now_seconds >= input.untargetable_active_until
            && input.now_seconds >= input.revive_lock_active_until,
        activate_emergency_shield: input.emergency_shield_available
            && input.now_seconds >= input.emergency_shield_ready_at
            && input.health <= input.max_health * input.emergency_shield_trigger_health_percent,
    }
}

pub(crate) fn should_trigger_revive_effect(input: ReviveEffectDecisionInput) -> bool {
    input.available && input.now_seconds >= input.ready_at
}

pub(crate) fn tick_controlled_champion_regen_heal(
    runtime: &ControlledChampionLoadoutRuntime,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    if !runtime.has_second_wind || max_health <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let health_ratio = (current_health / max_health).clamp(0.0, 1.0);
    let base_regen = 0.0012 * max_health * dt;
    let missing_health_bonus = if health_ratio <= 0.35 {
        0.0026 * max_health * dt
    } else {
        0.0
    };
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: base_regen + missing_health_bonus,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn controlled_champion_heal_multiplier(
    _runtime: &ControlledChampionLoadoutRuntime,
) -> f64 {
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: 1.0,
            source: ScalarMetricSource::Neutral,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn controlled_champion_damage_taken_multiplier(
    _runtime: &ControlledChampionLoadoutRuntime,
    _nearby_enemies: usize,
) -> f64 {
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: 1.0,
            source: ScalarMetricSource::Neutral,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn selection_with(rune_names: &[&str]) -> LoadoutSelection {
        LoadoutSelection {
            rune_names: rune_names.iter().map(|s| (*s).to_string()).collect(),
            shard_stats: Vec::new(),
        }
    }

    #[test]
    fn controlled_champion_runtime_parses_dynamic_runes() {
        let runtime = build_controlled_champion_loadout_runtime(&selection_with(&[
            "Arcane Comet",
            "Summon Aery",
            "Triumph",
            "Second Wind",
        ]));
        assert!(runtime.has_arcane_comet);
        assert!(runtime.has_summon_aery);
        assert!(runtime.has_triumph);
        assert!(runtime.has_second_wind);
    }

    #[test]
    fn arcane_comet_and_aery_respect_runtime_cooldowns() {
        let mut runtime = build_controlled_champion_loadout_runtime(&selection_with(&[
            "Arcane Comet",
            "Summon Aery",
        ]));

        let first = on_controlled_champion_ability_bonus(
            &mut runtime,
            ControlledChampionAbilityRuntimeInput {
                ability_power: 300.0,
                ability_ap_ratio: 0.6,
                now_seconds: 1.0,
            },
        );
        let second = on_controlled_champion_ability_bonus(
            &mut runtime,
            ControlledChampionAbilityRuntimeInput {
                ability_power: 300.0,
                ability_ap_ratio: 0.6,
                now_seconds: 1.5,
            },
        );
        assert!(first.extra_magic_damage > second.extra_magic_damage);
    }

    #[test]
    fn second_wind_regen_gives_more_heal_at_low_health() {
        let runtime = build_controlled_champion_loadout_runtime(&selection_with(&["Second Wind"]));
        let high = tick_controlled_champion_regen_heal(&runtime, 1800.0, 2000.0, 1.0);
        let low = tick_controlled_champion_regen_heal(&runtime, 500.0, 2000.0, 1.0);
        assert!(low > high);
    }

    #[test]
    fn defensive_item_decisions_require_health_and_readiness() {
        let decisions = decide_defensive_item_activations(DefensiveItemActivationInput {
            now_seconds: 10.0,
            can_cast: true,
            health: 320.0,
            max_health: 1000.0,
            stasis_available: true,
            stasis_ready_at: 7.0,
            stasis_trigger_health_percent: 0.35,
            untargetable_active_until: 10.0,
            revive_lock_active_until: 9.0,
            emergency_shield_available: true,
            emergency_shield_ready_at: 6.0,
            emergency_shield_trigger_health_percent: 0.40,
        });
        assert!(decisions.activate_stasis);
        assert!(decisions.activate_emergency_shield);
    }

    #[test]
    fn defensive_item_decisions_block_stasis_during_lock_windows() {
        let decisions = decide_defensive_item_activations(DefensiveItemActivationInput {
            now_seconds: 10.0,
            can_cast: true,
            health: 200.0,
            max_health: 1000.0,
            stasis_available: true,
            stasis_ready_at: 0.0,
            stasis_trigger_health_percent: 0.50,
            untargetable_active_until: 11.0,
            revive_lock_active_until: 0.0,
            emergency_shield_available: false,
            emergency_shield_ready_at: 0.0,
            emergency_shield_trigger_health_percent: 0.0,
        });
        assert!(!decisions.activate_stasis);

        let blocked_by_revive = decide_defensive_item_activations(DefensiveItemActivationInput {
            now_seconds: 10.0,
            can_cast: true,
            health: 200.0,
            max_health: 1000.0,
            stasis_available: true,
            stasis_ready_at: 0.0,
            stasis_trigger_health_percent: 0.50,
            untargetable_active_until: 0.0,
            revive_lock_active_until: 11.0,
            emergency_shield_available: false,
            emergency_shield_ready_at: 0.0,
            emergency_shield_trigger_health_percent: 0.0,
        });
        assert!(!blocked_by_revive.activate_stasis);
    }

    #[test]
    fn revive_effect_trigger_checks_cooldown_and_availability() {
        assert!(should_trigger_revive_effect(ReviveEffectDecisionInput {
            available: true,
            now_seconds: 120.0,
            ready_at: 120.0,
        }));
        assert!(!should_trigger_revive_effect(ReviveEffectDecisionInput {
            available: true,
            now_seconds: 119.9,
            ready_at: 120.0,
        }));
        assert!(!should_trigger_revive_effect(ReviveEffectDecisionInput {
            available: false,
            now_seconds: 120.0,
            ready_at: 0.0,
        }));
    }
}

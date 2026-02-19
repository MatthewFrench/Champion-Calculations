use anyhow::Result;

use crate::scripts::registry::hooks::{LoadoutHookContext, ScriptHook};
use crate::scripts::runes::effects::has_dynamic_rune_effect;
use crate::scripts::runtime::stat_resolution::{
    RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
};

pub(crate) struct ControlledChampionLoadoutHook;

pub(crate) const CONTROLLED_CHAMPION_LOADOUT_HOOK: ControlledChampionLoadoutHook =
    ControlledChampionLoadoutHook;

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

pub(crate) fn describe_controlled_champion_runtime_cooldowns(_now: f64) -> Vec<String> {
    vec!["none".to_string()]
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

pub(crate) fn controlled_champion_heal_multiplier() -> f64 {
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: 1.0,
            source: ScalarMetricSource::Neutral,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn controlled_champion_damage_taken_multiplier(_nearby_enemies: usize) -> f64 {
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
#[path = "tests/controlled_champion_loadout_tests.rs"]
mod tests;

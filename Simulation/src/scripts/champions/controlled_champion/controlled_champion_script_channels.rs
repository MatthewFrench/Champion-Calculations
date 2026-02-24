use std::collections::HashMap;

use crate::scripts::runtime::ability_slots::{AbilitySlotKey, ActorAbilityLoadout};
use crate::{ChampionBase, Stats};

use super::{
    ControlledChampionCastProfile, ControlledChampionDefensiveAbilityDecisionInput,
    ControlledChampionDefensiveAbilityDecisions, ControlledChampionDefensiveAbilityTwoConfig,
    ControlledChampionOffensiveAbility, ControlledChampionOffensiveCastDecisions,
    ControlledChampionScriptHandle,
};

pub(crate) fn controlled_champion_script_enabled(
    script: Option<&ControlledChampionScriptHandle>,
) -> bool {
    script.is_some()
}

pub(crate) fn controlled_champion_default_cast_profile(
    script: Option<&ControlledChampionScriptHandle>,
) -> ControlledChampionCastProfile {
    script
        .map(|script| script.default_cast_profile())
        .unwrap_or_default()
}

pub(crate) fn controlled_champion_offensive_cooldowns_after_haste(
    script: Option<&ControlledChampionScriptHandle>,
    ability_haste: f64,
) -> super::ControlledChampionAbilityCooldowns {
    script
        .map(|script| script.offensive_cooldowns_after_haste(ability_haste))
        .unwrap_or_default()
}

pub(crate) fn controlled_champion_defensive_ability_two_config(
    script: Option<&ControlledChampionScriptHandle>,
    ability_haste: f64,
) -> ControlledChampionDefensiveAbilityTwoConfig {
    script
        .map(|script| script.defensive_ability_two_config(ability_haste))
        .unwrap_or_default()
}

pub(crate) fn initialize_controlled_champion_ability_slots(
    script: Option<&ControlledChampionScriptHandle>,
    cast_profile: &ControlledChampionCastProfile,
    ability_loadout: &mut ActorAbilityLoadout,
    ability_ready_at: &mut HashMap<String, f64>,
) {
    if !controlled_champion_script_enabled(script) {
        return;
    }
    if ability_loadout
        .slot_for_ability(&cast_profile.offensive_primary_ability_id)
        .is_none()
    {
        ability_loadout.assign_ability_to_slot(
            cast_profile.offensive_primary_ability_id.clone(),
            AbilitySlotKey::Q,
        );
    }
    if ability_loadout
        .slot_for_ability(&cast_profile.defensive_ability_two_id)
        .is_none()
    {
        ability_loadout.assign_ability_to_slot(
            cast_profile.defensive_ability_two_id.clone(),
            AbilitySlotKey::W,
        );
    }
    if ability_loadout
        .slot_for_ability(&cast_profile.offensive_secondary_ability_id)
        .is_none()
    {
        ability_loadout.assign_ability_to_slot(
            cast_profile.offensive_secondary_ability_id.clone(),
            AbilitySlotKey::E,
        );
    }
    if ability_loadout
        .slot_for_ability(&cast_profile.offensive_ultimate_ability_id)
        .is_none()
    {
        ability_loadout.assign_ability_to_slot(
            cast_profile.offensive_ultimate_ability_id.clone(),
            AbilitySlotKey::R,
        );
    }
    ability_ready_at.insert(cast_profile.offensive_primary_ability_id.clone(), 0.0);
    ability_ready_at.insert(cast_profile.defensive_ability_two_id.clone(), 0.0);
    ability_ready_at.insert(cast_profile.offensive_secondary_ability_id.clone(), 0.0);
    ability_ready_at.insert(cast_profile.offensive_ultimate_ability_id.clone(), 0.0);
}

pub(crate) fn decide_controlled_champion_defensive_ability_activations(
    script: Option<&ControlledChampionScriptHandle>,
    input: ControlledChampionDefensiveAbilityDecisionInput,
) -> ControlledChampionDefensiveAbilityDecisions {
    script
        .map(|script| script.decide_defensive_ability_activations(input))
        .unwrap_or_default()
}

pub(crate) fn decide_controlled_champion_offensive_casts(
    script: Option<&ControlledChampionScriptHandle>,
    input: super::ControlledChampionOffensiveDecisionInput,
) -> ControlledChampionOffensiveCastDecisions {
    script
        .map(|script| script.decide_offensive_casts(input))
        .unwrap_or_default()
}

pub(crate) fn controlled_champion_offensive_raw_damage(
    script: Option<&ControlledChampionScriptHandle>,
    ability: ControlledChampionOffensiveAbility,
    ability_power: f64,
) -> f64 {
    script
        .map(|script| script.offensive_raw_damage(ability, ability_power))
        .unwrap_or(0.0)
}

pub(crate) fn controlled_champion_offensive_ap_ratio(
    script: Option<&ControlledChampionScriptHandle>,
    ability: ControlledChampionOffensiveAbility,
) -> f64 {
    script
        .map(|script| script.offensive_ap_ratio(ability))
        .unwrap_or(0.0)
}

pub(crate) fn controlled_champion_offensive_primary_heal_ratio(
    script: Option<&ControlledChampionScriptHandle>,
) -> f64 {
    script
        .map(|script| script.offensive_primary_heal_ratio())
        .unwrap_or(0.0)
}

pub(crate) fn controlled_champion_defensive_ability_two_raw_damage(
    script: Option<&ControlledChampionScriptHandle>,
    config: ControlledChampionDefensiveAbilityTwoConfig,
    controlled_champion_stats: &Stats,
    controlled_champion_base: &ChampionBase,
) -> f64 {
    script
        .map(|script| {
            script.defensive_ability_two_raw_damage(
                config,
                controlled_champion_stats,
                controlled_champion_base,
            )
        })
        .unwrap_or(0.0)
}

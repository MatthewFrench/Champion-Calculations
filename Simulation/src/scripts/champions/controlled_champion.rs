use std::collections::HashMap;
use std::sync::Arc;

use crate::scripts::runtime::ability_slots::{AbilitySlotKey, ActorAbilityLoadout};
use crate::{ChampionBase, Stats, to_norm_key};

mod vladimir_controlled_champion_script;

use self::vladimir_controlled_champion_script::build_vladimir_script;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ControlledChampionOffensiveAbility {
    Primary,
    Secondary,
    Ultimate,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionAbilityTuning {
    pub offensive_primary_base_damage: f64,
    pub offensive_primary_ap_ratio: f64,
    pub offensive_primary_heal_ratio_of_damage: f64,
    pub offensive_primary_base_cooldown_seconds: f64,
    pub offensive_secondary_base_damage: f64,
    pub offensive_secondary_ap_ratio: f64,
    pub offensive_secondary_base_cooldown_seconds: f64,
    pub offensive_ultimate_base_damage: f64,
    pub offensive_ultimate_ap_ratio: f64,
    pub offensive_ultimate_base_cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionAbilityCooldowns {
    pub offensive_primary_seconds: f64,
    pub offensive_secondary_seconds: f64,
    pub offensive_ultimate_seconds: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ControlledChampionCastProfile {
    pub offensive_primary_ability_id: String,
    pub defensive_ability_two_id: String,
    pub offensive_secondary_ability_id: String,
    pub offensive_ultimate_ability_id: String,
    pub offensive_primary_range: f64,
    pub offensive_primary_windup_seconds: f64,
    pub offensive_primary_projectile_speed: f64,
    pub offensive_primary_effect_hitbox_radius: f64,
    pub offensive_secondary_range: f64,
    pub offensive_secondary_windup_seconds: f64,
    pub offensive_secondary_projectile_speed: f64,
    pub offensive_secondary_effect_hitbox_radius: f64,
    pub offensive_ultimate_range: f64,
    pub offensive_ultimate_windup_seconds: f64,
    pub offensive_ultimate_projectile_speed: f64,
    pub offensive_ultimate_effect_hitbox_radius: f64,
}

impl Default for ControlledChampionCastProfile {
    fn default() -> Self {
        Self {
            offensive_primary_ability_id: String::new(),
            defensive_ability_two_id: String::new(),
            offensive_secondary_ability_id: String::new(),
            offensive_ultimate_ability_id: String::new(),
            offensive_primary_range: 0.0,
            offensive_primary_windup_seconds: 0.0,
            offensive_primary_projectile_speed: 0.0,
            offensive_primary_effect_hitbox_radius: 0.0,
            offensive_secondary_range: 0.0,
            offensive_secondary_windup_seconds: 0.0,
            offensive_secondary_projectile_speed: 0.0,
            offensive_secondary_effect_hitbox_radius: 0.0,
            offensive_ultimate_range: 0.0,
            offensive_ultimate_windup_seconds: 0.0,
            offensive_ultimate_projectile_speed: 0.0,
            offensive_ultimate_effect_hitbox_radius: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionTargetSnapshot {
    pub target_index: usize,
    pub distance: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ControlledChampionOffensiveDecisionInput {
    pub now_seconds: f64,
    pub can_cast: bool,
    pub offensive_primary_ready_at: f64,
    pub offensive_secondary_ready_at: f64,
    pub offensive_ultimate_ready_at: f64,
    pub cooldowns: ControlledChampionAbilityCooldowns,
    pub cast_profile: ControlledChampionCastProfile,
    pub offensive_primary_target: Option<ControlledChampionTargetSnapshot>,
    pub offensive_secondary_max_distance: Option<f64>,
    pub offensive_ultimate_max_distance: Option<f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct ControlledChampionSingleTargetCastDecision {
    pub ability_id: String,
    pub target_index: usize,
    pub impact_delay_seconds: f64,
    pub next_ready_at: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ControlledChampionAreaCastDecision {
    pub ability_id: String,
    pub impact_delay_seconds: f64,
    pub next_ready_at: f64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ControlledChampionOffensiveCastDecisions {
    pub offensive_primary: Option<ControlledChampionSingleTargetCastDecision>,
    pub offensive_secondary: Option<ControlledChampionAreaCastDecision>,
    pub offensive_ultimate: Option<ControlledChampionAreaCastDecision>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ControlledChampionDefensiveAbilityDecisionInput {
    pub now_seconds: f64,
    pub can_cast: bool,
    pub defensive_ability_two_ready_at: f64,
    pub offensive_ultimate_ready_at: f64,
    pub offensive_ultimate_has_viable_targets: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionDefensiveAbilityDecisions {
    pub cast_defensive_ability_two: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ControlledChampionDefensiveAbilityTwoConfig {
    pub cooldown_seconds: f64,
    pub duration_seconds: f64,
    pub effect_range: f64,
    pub damage_tick_interval_seconds: f64,
    pub cost_percent_current_health: f64,
    pub damage_per_tick: f64,
    pub damage_per_tick_bonus_health_ratio: f64,
    pub heal_ratio_of_damage: f64,
}

pub(crate) trait ControlledChampionScriptCapability: std::fmt::Debug + Send + Sync {
    fn default_cast_profile(&self) -> ControlledChampionCastProfile;
    fn offensive_cooldowns_after_haste(
        &self,
        ability_haste: f64,
    ) -> ControlledChampionAbilityCooldowns;
    fn defensive_ability_two_config(
        &self,
        ability_haste: f64,
    ) -> ControlledChampionDefensiveAbilityTwoConfig;
    fn decide_defensive_ability_activations(
        &self,
        input: ControlledChampionDefensiveAbilityDecisionInput,
    ) -> ControlledChampionDefensiveAbilityDecisions;
    fn decide_offensive_casts(
        &self,
        input: ControlledChampionOffensiveDecisionInput,
    ) -> ControlledChampionOffensiveCastDecisions;
    fn offensive_raw_damage(
        &self,
        ability: ControlledChampionOffensiveAbility,
        ability_power: f64,
    ) -> f64;
    fn offensive_ap_ratio(&self, ability: ControlledChampionOffensiveAbility) -> f64;
    fn offensive_primary_heal_ratio(&self) -> f64;
    fn defensive_ability_two_raw_damage(
        &self,
        config: ControlledChampionDefensiveAbilityTwoConfig,
        controlled_champion_stats: &Stats,
        controlled_champion_base: &ChampionBase,
    ) -> f64;
}

pub(crate) type ControlledChampionScriptHandle = Arc<dyn ControlledChampionScriptCapability>;

type ControlledChampionScriptFactory = fn() -> ControlledChampionScriptHandle;

struct ControlledChampionScriptRegistryEntry {
    champion_key: &'static str,
    build: ControlledChampionScriptFactory,
}

const CONTROLLED_CHAMPION_SCRIPT_REGISTRY: &[ControlledChampionScriptRegistryEntry] =
    &[ControlledChampionScriptRegistryEntry {
        champion_key: "vladimir",
        build: build_vladimir_script,
    }];

pub(crate) fn resolve_controlled_champion_script(
    champion_name: &str,
) -> Option<ControlledChampionScriptHandle> {
    let champion_key = to_norm_key(champion_name);
    CONTROLLED_CHAMPION_SCRIPT_REGISTRY
        .iter()
        .find(|entry| entry.champion_key == champion_key.as_str())
        .map(|entry| (entry.build)())
}

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
) -> ControlledChampionAbilityCooldowns {
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
    input: ControlledChampionOffensiveDecisionInput,
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

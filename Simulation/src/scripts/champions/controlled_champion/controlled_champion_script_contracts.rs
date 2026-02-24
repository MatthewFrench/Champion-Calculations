use std::sync::Arc;

use crate::{ChampionBase, Stats};

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

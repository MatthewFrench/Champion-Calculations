use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ControlledChampionDefensiveItemPolicyDefaults {
    pub stasis_trigger_health_percent: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionAiDefaults {
    pub script_poll_interval_seconds: f64,
    pub movement_speed_scale: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub(crate) struct ChampionAiProfileOverrideEntry {
    #[serde(default)]
    pub desired_combat_range: Option<f64>,
    #[serde(default)]
    pub movement_speed_scale: Option<f64>,
    #[serde(default)]
    pub script_poll_interval_seconds: Option<f64>,
    #[serde(default)]
    pub script_priority_overrides: HashMap<String, i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionAiProfilesFile {
    pub(crate) defaults: ChampionAiDefaults,
    pub(crate) controlled_champion_defaults: ControlledChampionDefensiveItemPolicyDefaults,
    #[serde(default)]
    pub(crate) champions: HashMap<String, ChampionAiProfileOverrideEntry>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct AbilityExecutionDefaultsEntry {
    #[serde(default)]
    pub cast_windup_seconds: f64,
    #[serde(default)]
    pub projectile_speed: f64,
    #[serde(default)]
    pub effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AbilityExecutionDefaultsByRole {
    pub(crate) melee: AbilityExecutionDefaultsEntry,
    pub(crate) ranged: AbilityExecutionDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct AbilityExecutionOverrideEntry {
    #[serde(default)]
    pub cast_windup_seconds: Option<f64>,
    #[serde(default)]
    pub projectile_speed: Option<f64>,
    #[serde(default)]
    pub effect_hitbox_radius: Option<f64>,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub(crate) struct ChampionAbilityExecutionData {
    pub(crate) is_melee: bool,
    pub(crate) abilities: HashMap<String, AbilityExecutionOverrideEntry>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionAiProfile {
    pub desired_combat_range: f64,
    pub movement_speed_scale: f64,
    pub script_poll_interval_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AbilityExecutionProfile {
    pub cast_windup_seconds: f64,
    pub projectile_speed: f64,
    pub effect_hitbox_radius: f64,
}

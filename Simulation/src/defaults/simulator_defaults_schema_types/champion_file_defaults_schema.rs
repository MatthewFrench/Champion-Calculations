use serde::Deserialize;

use super::champion_ai_and_execution_schema::AbilityExecutionDefaultsByRole;
use super::champion_behavior_and_ability_defaults_schema::ChampionBehaviorOverrideEntry;

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct ChampionSimulationData {
    #[serde(default)]
    pub behavior: Option<ChampionBehaviorOverrideEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionFileEnvelope {
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) behavior: Option<ChampionBehaviorOverrideEntry>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBaseStatsDefaultsEntry {
    #[serde(default)]
    pub(crate) attack_range: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBaseStatsDefaultsByRole {
    pub(crate) melee: ChampionBaseStatsDefaultsEntry,
    pub(crate) ranged: ChampionBaseStatsDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBasicAttackRawTimingDefaultsEntry {
    #[serde(default)]
    pub(crate) gameplay_radius: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBasicAttackDefaultsEntry {
    #[serde(default)]
    pub(crate) base_windup_seconds: f64,
    #[serde(default)]
    pub(crate) missile_speed: f64,
    #[serde(default)]
    pub(crate) raw_timing_stats: ChampionBasicAttackRawTimingDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBasicAttackDefaultsByRole {
    pub(crate) melee: ChampionBasicAttackDefaultsEntry,
    pub(crate) ranged: ChampionBasicAttackDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorOnHitDefaultsEntry {
    #[serde(default)]
    pub(crate) magic_flat: f64,
    #[serde(default)]
    pub(crate) magic_ad_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorPeriodicTrueHitDefaultsEntry {
    #[serde(default)]
    pub(crate) every: usize,
    #[serde(default)]
    pub(crate) base: f64,
    #[serde(default)]
    pub(crate) ad_ratio: f64,
    #[serde(default)]
    pub(crate) target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorModifiersEntry {
    #[serde(default)]
    pub(crate) on_hit: ChampionBehaviorOnHitDefaultsEntry,
    #[serde(default)]
    pub(crate) periodic_true_hit: ChampionBehaviorPeriodicTrueHitDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorModifiersByRole {
    #[serde(default)]
    pub(crate) melee: ChampionBehaviorModifiersEntry,
    #[serde(default)]
    pub(crate) ranged: ChampionBehaviorModifiersEntry,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionAbilitiesDefaults {
    pub(crate) execution_defaults: AbilityExecutionDefaultsByRole,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ChampionDefaultsFile {
    pub(crate) base_stats: ChampionBaseStatsDefaultsByRole,
    pub(crate) basic_attack: ChampionBasicAttackDefaultsByRole,
    pub(crate) abilities: ChampionAbilitiesDefaults,
    #[serde(default)]
    pub(crate) behavior: ChampionBehaviorModifiersByRole,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct UrfRespawnDefaults {
    pub flat_reduction_seconds: f64,
    pub extrapolation_per_level: f64,
    pub time_scaling_enabled: bool,
    pub time_scaling_start_seconds: f64,
    pub time_scaling_per_minute_seconds: f64,
    pub time_scaling_cap_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct UrfFileEnvelope {
    #[serde(default)]
    pub(crate) respawn: Option<UrfRespawnDefaults>,
}

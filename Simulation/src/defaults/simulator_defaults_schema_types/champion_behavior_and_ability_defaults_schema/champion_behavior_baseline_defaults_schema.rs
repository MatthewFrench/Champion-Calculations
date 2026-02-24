use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBehaviorDefaultsEntry {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub attack_effect_hitbox_radius: f64,
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorOverrideEntry {
    #[serde(default)]
    pub attack_range: Option<f64>,
    #[serde(default)]
    pub attack_windup_seconds: Option<f64>,
    #[serde(default)]
    pub attack_projectile_speed: Option<f64>,
    #[serde(default)]
    pub attack_effect_hitbox_radius: Option<f64>,
    #[serde(default)]
    pub on_hit_magic_flat: Option<f64>,
    #[serde(default)]
    pub on_hit_magic_ad_ratio: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_every: Option<usize>,
    #[serde(default)]
    pub periodic_true_hit_base: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_ad_ratio: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_target_max_health_ratio: Option<f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionBehaviorDefaults {
    pub(crate) melee: ChampionBehaviorDefaultsEntry,
    pub(crate) ranged: ChampionBehaviorDefaultsEntry,
}

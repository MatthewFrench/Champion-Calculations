#[derive(Debug, Clone)]
pub(crate) struct VladimirCastProfileDefaults {
    pub q_ability_id: String,
    pub e_ability_id: String,
    pub r_ability_id: String,
    pub pool_ability_id: String,
    pub q_range: f64,
    pub q_windup_seconds: f64,
    pub q_projectile_speed: f64,
    pub q_effect_hitbox_radius: f64,
    pub e_range: f64,
    pub e_windup_seconds: f64,
    pub e_projectile_speed: f64,
    pub e_effect_hitbox_radius: f64,
    pub r_range: f64,
    pub r_windup_seconds: f64,
    pub r_projectile_speed: f64,
    pub r_effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirOffensiveAbilityDefaults {
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

#[derive(Debug, Clone)]
pub(crate) struct VladimirSanguinePoolDefaults {
    pub base_cooldown_seconds_by_rank: Vec<f64>,
    pub default_rank: usize,
    pub effect_range: f64,
    pub untargetable_seconds: f64,
    pub damage_tick_interval_seconds: f64,
    pub cost_percent_current_health: f64,
    pub heal_ratio_of_damage: f64,
    pub damage_per_tick_by_rank: Vec<f64>,
    pub damage_per_tick_bonus_health_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirDefensiveAbilityTwoPolicyDefaults {
    pub prioritize_offensive_ultimate_before_defensive_ability_two: bool,
}

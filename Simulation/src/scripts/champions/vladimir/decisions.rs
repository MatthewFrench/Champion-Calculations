use super::abilities::VladimirAbilityCooldowns;
use crate::defaults::simulator_defaults;

#[derive(Debug, Clone)]
pub(crate) struct VladimirCastProfile {
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
pub(crate) struct VladimirTargetSnapshot {
    pub target_index: usize,
    pub distance: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct VladimirOffensiveDecisionInput {
    pub now_seconds: f64,
    pub can_cast: bool,
    pub q_ready_at: f64,
    pub e_ready_at: f64,
    pub r_ready_at: f64,
    pub cooldowns: VladimirAbilityCooldowns,
    pub cast_profile: VladimirCastProfile,
    pub q_target: Option<VladimirTargetSnapshot>,
    pub e_max_distance: Option<f64>,
    pub r_max_distance: Option<f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct VladimirSingleTargetCastDecision {
    pub ability_id: String,
    pub target_index: usize,
    pub impact_delay_seconds: f64,
    pub next_ready_at: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct VladimirAreaCastDecision {
    pub ability_id: String,
    pub impact_delay_seconds: f64,
    pub next_ready_at: f64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct VladimirOffensiveCastDecisions {
    pub q: Option<VladimirSingleTargetCastDecision>,
    pub e: Option<VladimirAreaCastDecision>,
    pub r: Option<VladimirAreaCastDecision>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirDefensiveAbilityDecisionInput {
    pub now_seconds: f64,
    pub can_cast: bool,
    pub pool_ready_at: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct VladimirDefensiveAbilityDecisions {
    pub cast_pool: bool,
}

pub(crate) fn default_cast_profile() -> VladimirCastProfile {
    let defaults = &simulator_defaults().vladimir_cast_profile_defaults;
    VladimirCastProfile {
        q_ability_id: defaults.q_ability_id.clone(),
        e_ability_id: defaults.e_ability_id.clone(),
        r_ability_id: defaults.r_ability_id.clone(),
        pool_ability_id: defaults.pool_ability_id.clone(),
        q_range: defaults.q_range,
        q_windup_seconds: defaults.q_windup_seconds,
        q_projectile_speed: defaults.q_projectile_speed,
        q_effect_hitbox_radius: defaults.q_effect_hitbox_radius,
        e_range: defaults.e_range,
        e_windup_seconds: defaults.e_windup_seconds,
        e_projectile_speed: defaults.e_projectile_speed,
        e_effect_hitbox_radius: defaults.e_effect_hitbox_radius,
        r_range: defaults.r_range,
        r_windup_seconds: defaults.r_windup_seconds,
        r_projectile_speed: defaults.r_projectile_speed,
        r_effect_hitbox_radius: defaults.r_effect_hitbox_radius,
    }
}

fn projectile_travel_seconds(distance: f64, speed: f64) -> f64 {
    if speed <= 0.0 {
        0.0
    } else {
        (distance / speed).max(0.0)
    }
}

pub(crate) fn decide_offensive_casts(
    input: VladimirOffensiveDecisionInput,
) -> VladimirOffensiveCastDecisions {
    let mut decisions = VladimirOffensiveCastDecisions::default();
    if !input.can_cast {
        return decisions;
    }

    if input.now_seconds >= input.q_ready_at
        && let Some(target) = input.q_target
    {
        let travel =
            projectile_travel_seconds(target.distance, input.cast_profile.q_projectile_speed);
        decisions.q = Some(VladimirSingleTargetCastDecision {
            ability_id: input.cast_profile.q_ability_id.clone(),
            target_index: target.target_index,
            impact_delay_seconds: input.cast_profile.q_windup_seconds + travel,
            next_ready_at: input.now_seconds + input.cooldowns.q_seconds,
        });
    }

    if input.now_seconds >= input.e_ready_at
        && let Some(max_distance) = input.e_max_distance
    {
        let travel = projectile_travel_seconds(max_distance, input.cast_profile.e_projectile_speed);
        decisions.e = Some(VladimirAreaCastDecision {
            ability_id: input.cast_profile.e_ability_id.clone(),
            impact_delay_seconds: input.cast_profile.e_windup_seconds + travel,
            next_ready_at: input.now_seconds + input.cooldowns.e_seconds,
        });
    }

    if input.now_seconds >= input.r_ready_at
        && let Some(max_distance) = input.r_max_distance
    {
        let travel = projectile_travel_seconds(max_distance, input.cast_profile.r_projectile_speed);
        decisions.r = Some(VladimirAreaCastDecision {
            ability_id: input.cast_profile.r_ability_id.clone(),
            impact_delay_seconds: input.cast_profile.r_windup_seconds + travel,
            next_ready_at: input.now_seconds + input.cooldowns.r_seconds,
        });
    }

    decisions
}

pub(crate) fn decide_defensive_ability_activations(
    input: VladimirDefensiveAbilityDecisionInput,
) -> VladimirDefensiveAbilityDecisions {
    VladimirDefensiveAbilityDecisions {
        cast_pool: input.can_cast && input.now_seconds >= input.pool_ready_at,
    }
}

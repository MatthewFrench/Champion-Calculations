use super::abilities::VladimirAbilityCooldowns;

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirCastProfile {
    pub q_range: f64,
    pub q_windup_seconds: f64,
    pub q_projectile_speed: f64,
    pub e_range: f64,
    pub e_windup_seconds: f64,
    pub e_projectile_speed: f64,
    pub r_range: f64,
    pub r_windup_seconds: f64,
    pub r_projectile_speed: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirTargetSnapshot {
    pub target_index: usize,
    pub distance: f64,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirSingleTargetCastDecision {
    pub target_index: usize,
    pub impact_delay_seconds: f64,
    pub next_ready_at: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirAreaCastDecision {
    pub impact_delay_seconds: f64,
    pub next_ready_at: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct VladimirOffensiveCastDecisions {
    pub q: Option<VladimirSingleTargetCastDecision>,
    pub e: Option<VladimirAreaCastDecision>,
    pub r: Option<VladimirAreaCastDecision>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirDefensiveDecisionInput {
    pub now_seconds: f64,
    pub can_cast: bool,
    pub health: f64,
    pub max_health: f64,
    pub pool_ready_at: f64,
    pub zhonya_available: bool,
    pub zhonya_ready_at: f64,
    pub zhonya_trigger_health_percent: f64,
    pub pool_active_until: f64,
    pub ga_revive_active_until: f64,
    pub protoplasm_available: bool,
    pub protoplasm_ready_at: f64,
    pub protoplasm_trigger_health_percent: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct VladimirDefensiveDecisions {
    pub cast_pool: bool,
    pub activate_zhonya: bool,
    pub activate_protoplasm: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirGuardianAngelDecisionInput {
    pub available: bool,
    pub now_seconds: f64,
    pub ready_at: f64,
}

pub(crate) fn default_cast_profile() -> VladimirCastProfile {
    VladimirCastProfile {
        q_range: 600.0,
        q_windup_seconds: 0.20,
        q_projectile_speed: 0.0,
        e_range: 600.0,
        e_windup_seconds: 0.30,
        e_projectile_speed: 0.0,
        r_range: 700.0,
        r_windup_seconds: 0.25,
        r_projectile_speed: 0.0,
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
            impact_delay_seconds: input.cast_profile.e_windup_seconds + travel,
            next_ready_at: input.now_seconds + input.cooldowns.e_seconds,
        });
    }

    if input.now_seconds >= input.r_ready_at
        && let Some(max_distance) = input.r_max_distance
    {
        let travel = projectile_travel_seconds(max_distance, input.cast_profile.r_projectile_speed);
        decisions.r = Some(VladimirAreaCastDecision {
            impact_delay_seconds: input.cast_profile.r_windup_seconds + travel,
            next_ready_at: input.now_seconds + input.cooldowns.r_seconds,
        });
    }

    decisions
}

pub(crate) fn decide_defensive_activations(
    input: VladimirDefensiveDecisionInput,
) -> VladimirDefensiveDecisions {
    VladimirDefensiveDecisions {
        cast_pool: input.can_cast && input.now_seconds >= input.pool_ready_at,
        activate_zhonya: input.zhonya_available
            && input.now_seconds >= input.zhonya_ready_at
            && input.health <= input.max_health * input.zhonya_trigger_health_percent
            && input.now_seconds >= input.pool_active_until
            && input.now_seconds >= input.ga_revive_active_until,
        activate_protoplasm: input.protoplasm_available
            && input.now_seconds >= input.protoplasm_ready_at
            && input.health <= input.max_health * input.protoplasm_trigger_health_percent,
    }
}

pub(crate) fn should_trigger_guardian_angel(input: VladimirGuardianAngelDecisionInput) -> bool {
    input.available && input.now_seconds >= input.ready_at
}

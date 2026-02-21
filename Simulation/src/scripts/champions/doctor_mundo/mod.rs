use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ScriptedEffectHitbox, on_ability_bonus_damage,
};
use crate::defaults::{
    doctor_mundo_blunt_force_trauma_ability_defaults,
    doctor_mundo_infected_bonesaw_ability_defaults,
};

pub(crate) const CHAMPION_KEY: &str = "drmundo";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn event_cooldown_seconds(event: ChampionScriptEvent) -> Option<f64> {
    match event {
        ChampionScriptEvent::DoctorMundoInfectedBonesaw => {
            Some(doctor_mundo_infected_bonesaw_ability_defaults().cooldown_seconds)
        }
        ChampionScriptEvent::DoctorMundoBluntForceTrauma => Some(
            doctor_mundo_blunt_force_trauma_ability_defaults().blunt_force_trauma_cooldown_seconds,
        ),
        _ => None,
    }
}

pub(crate) fn execute_infected_bonesaw(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = doctor_mundo_infected_bonesaw_ability_defaults();
    if input.distance_to_target > ability_defaults.cast_range {
        return Vec::new();
    }
    let raw_magic = (ability_defaults.current_health_ratio * input.target_current_health.max(0.0))
        .max(ability_defaults.minimum_magic_damage);
    let (extra_magic, extra_true) = on_ability_bonus_damage(
        runtime,
        raw_magic,
        0.0,
        input.actor_ability_power,
        input.actor_bonus_attack_damage,
        input.target_current_health,
        input.target_max_health,
        input.now,
        Some(0),
        input.actor_level,
    );
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: ability_defaults.infected_bonesaw_execution.projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults
                .infected_bonesaw_execution
                .effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

pub(crate) fn execute_blunt_force_trauma(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = doctor_mundo_blunt_force_trauma_ability_defaults();
    if input.distance_to_target > ability_defaults.blunt_force_trauma_cast_range {
        return Vec::new();
    }
    vec![ChampionScriptAction::AddNextAttackBonusPhysical {
        amount: ability_defaults.blunt_force_trauma_bonus_physical_min_base_damage,
        trace_message: "Blunt Force Trauma empowered next attack",
    }]
}

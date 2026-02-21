use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ScriptedEffectHitbox, on_ability_bonus_damage,
};
use crate::defaults::{
    champion_ai_script_priority_override, morgana_binding_and_soul_shackles_ability_defaults,
    morgana_tormented_shadow_ability_defaults,
};

pub(crate) const CHAMPION_KEY: &str = "morgana";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn event_cooldown_seconds(event: ChampionScriptEvent) -> Option<f64> {
    let defaults = morgana_binding_and_soul_shackles_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Morgana.json abilities"));
    match event {
        ChampionScriptEvent::MorganaDarkBinding => Some(defaults.dark_binding_cooldown_seconds),
        ChampionScriptEvent::MorganaTormentedShadow => Some(
            morgana_tormented_shadow_ability_defaults(CHAMPION_KEY)
                .unwrap_or_else(|| panic!("Missing Characters/Morgana.json abilities"))
                .tormented_shadow_cooldown_seconds,
        ),
        ChampionScriptEvent::MorganaSoulShackles => Some(defaults.soul_shackles_cooldown_seconds),
        _ => None,
    }
}

pub(crate) fn execute_dark_binding(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = morgana_binding_and_soul_shackles_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Morgana.json abilities"));
    if input.distance_to_target > ability_defaults.dark_binding_cast_range {
        return Vec::new();
    }
    let raw_magic = ability_defaults.dark_binding_magic_base_damage
        + ability_defaults.dark_binding_magic_ability_power_ratio
            * input.actor_ability_power.max(0.0);
    let (extra_magic, extra_true) = on_ability_bonus_damage(
        runtime,
        raw_magic,
        ability_defaults.dark_binding_magic_ability_power_ratio,
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
        projectile_speed: ability_defaults.dark_binding_execution.projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults.dark_binding_execution.effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: ability_defaults.dark_binding_stun_duration_seconds,
    }]
}

pub(crate) fn execute_soul_shackles(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = morgana_binding_and_soul_shackles_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Morgana.json abilities"));
    if input.distance_to_target > ability_defaults.soul_shackles_cast_range {
        return Vec::new();
    }
    vec![
        ChampionScriptAction::ApplyDamage {
            source: input.actor_position,
            projectile_speed: ability_defaults.soul_shackles_execution.projectile_speed,
            hitbox: ScriptedEffectHitbox::Circle {
                radius: ability_defaults
                    .soul_shackles_execution
                    .effect_hitbox_radius,
            },
            physical: 0.0,
            magic: ability_defaults.soul_shackles_initial_magic_damage
                + ability_defaults.soul_shackles_initial_magic_ability_power_ratio
                    * input.actor_ability_power.max(0.0),
            true_damage: 0.0,
            stun_duration: 0.0,
        },
        ChampionScriptAction::ScheduleFollowup {
            delay_seconds: ability_defaults.soul_shackles_detonate_delay_seconds,
            priority: champion_ai_script_priority_override(CHAMPION_KEY, "soul_shackles_detonate")
                .unwrap_or(11),
            event: ChampionScriptEvent::MorganaSoulShacklesDetonate,
        },
    ]
}

pub(crate) fn execute_soul_shackles_detonate(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = morgana_binding_and_soul_shackles_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Morgana.json abilities"));
    if input.distance_to_target > ability_defaults.soul_shackles_cast_range {
        return Vec::new();
    }
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: ability_defaults.soul_shackles_execution.projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults
                .soul_shackles_execution
                .effect_hitbox_radius,
        },
        physical: 0.0,
        magic: ability_defaults.soul_shackles_detonate_magic_damage
            + ability_defaults.soul_shackles_detonate_magic_ability_power_ratio
                * input.actor_ability_power.max(0.0),
        true_damage: 0.0,
        stun_duration: ability_defaults.soul_shackles_detonate_stun_duration_seconds,
    }]
}

pub(crate) fn execute_tormented_shadow(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = morgana_tormented_shadow_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Morgana.json abilities.basic_ability_2"));
    if input.distance_to_target > ability_defaults.tormented_shadow_cast_range {
        return Vec::new();
    }
    let raw_magic = ability_defaults.tormented_shadow_total_magic_base_damage
        + ability_defaults.tormented_shadow_total_magic_ability_power_ratio
            * input.actor_ability_power.max(0.0);
    let (extra_magic, extra_true) = on_ability_bonus_damage(
        runtime,
        raw_magic,
        ability_defaults.tormented_shadow_total_magic_ability_power_ratio,
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
        projectile_speed: ability_defaults.tormented_shadow_execution.projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults
                .tormented_shadow_execution
                .effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

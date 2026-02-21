use crate::defaults::{vladimir_cast_profile_defaults, vladimir_offensive_ability_defaults};
use crate::scripts::champions::{
    ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ScriptedEffectHitbox, on_ability_bonus_damage,
};

pub(crate) const CHAMPION_KEY: &str = "vladimir";

pub(crate) fn event_cooldown_seconds(event: ChampionScriptEvent) -> Option<f64> {
    let defaults = vladimir_offensive_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vladimir.json abilities"));
    match event {
        ChampionScriptEvent::VladimirTransfusion => Some(defaults.q_base_cooldown_seconds),
        ChampionScriptEvent::VladimirTidesOfBlood => Some(defaults.e_base_cooldown_seconds),
        ChampionScriptEvent::VladimirHemoplague => Some(defaults.r_base_cooldown_seconds),
        _ => None,
    }
}

pub(crate) fn execute_transfusion(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let cast_profile = vladimir_cast_profile_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vladimir.json cast profile defaults"));
    if input.distance_to_target > cast_profile.q_range {
        return Vec::new();
    }
    let defaults = vladimir_offensive_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vladimir.json abilities"));
    let raw_magic =
        defaults.q_base_damage + defaults.q_ap_ratio * input.actor_ability_power.max(0.0);
    let (extra_magic, extra_true) = on_ability_bonus_damage(
        runtime,
        raw_magic,
        defaults.q_ap_ratio,
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
        projectile_speed: cast_profile.q_projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: cast_profile.q_effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

pub(crate) fn execute_tides_of_blood(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let cast_profile = vladimir_cast_profile_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vladimir.json cast profile defaults"));
    if input.distance_to_target > cast_profile.e_range {
        return Vec::new();
    }
    let defaults = vladimir_offensive_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vladimir.json abilities"));
    let raw_magic =
        defaults.e_base_damage + defaults.e_ap_ratio * input.actor_ability_power.max(0.0);
    let (extra_magic, extra_true) = on_ability_bonus_damage(
        runtime,
        raw_magic,
        defaults.e_ap_ratio,
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
        projectile_speed: cast_profile.e_projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: cast_profile.e_effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

pub(crate) fn execute_hemoplague(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let cast_profile = vladimir_cast_profile_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vladimir.json cast profile defaults"));
    if input.distance_to_target > cast_profile.r_range {
        return Vec::new();
    }
    let defaults = vladimir_offensive_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vladimir.json abilities"));
    let raw_magic =
        defaults.r_base_damage + defaults.r_ap_ratio * input.actor_ability_power.max(0.0);
    let (extra_magic, extra_true) = on_ability_bonus_damage(
        runtime,
        raw_magic,
        defaults.r_ap_ratio,
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
        projectile_speed: cast_profile.r_projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: cast_profile.r_effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

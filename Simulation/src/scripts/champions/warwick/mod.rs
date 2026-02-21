use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ScriptedEffectHitbox, on_ability_bonus_damage,
};
use crate::defaults::{
    warwick_eternal_hunger_passive_defaults, warwick_infinite_duress_ability_defaults,
    warwick_jaws_of_the_beast_ability_defaults,
};

pub(crate) const CHAMPION_KEY: &str = "warwick";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    let passive_defaults = warwick_eternal_hunger_passive_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Warwick.json abilities.passive"));
    profile.on_hit_magic_flat = passive_defaults.on_hit_magic_flat;
    profile.on_hit_magic_ad_ratio = passive_defaults.on_hit_magic_ad_ratio;
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn event_cooldown_seconds(event: ChampionScriptEvent) -> Option<f64> {
    match event {
        ChampionScriptEvent::WarwickInfiniteDuress => Some(
            warwick_infinite_duress_ability_defaults(CHAMPION_KEY)
                .unwrap_or_else(|| panic!("Missing Characters/Warwick.json abilities.ultimate"))
                .infinite_duress_cooldown_seconds,
        ),
        ChampionScriptEvent::WarwickJawsOfTheBeast => Some(
            warwick_jaws_of_the_beast_ability_defaults(CHAMPION_KEY)
                .unwrap_or_else(|| {
                    panic!("Missing Characters/Warwick.json abilities.basic_ability_1")
                })
                .jaws_of_the_beast_cooldown_seconds,
        ),
        _ => None,
    }
}

pub(crate) fn execute_infinite_duress(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = warwick_infinite_duress_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Warwick.json abilities.ultimate"));
    if input.distance_to_target > ability_defaults.infinite_duress_cast_range {
        return Vec::new();
    }
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: ability_defaults.infinite_duress_execution.projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults
                .infinite_duress_execution
                .effect_hitbox_radius,
        },
        physical: input.physical_hit_damage
            * ability_defaults.infinite_duress_physical_attack_damage_ratio,
        magic: ability_defaults.infinite_duress_magic_base_damage
            + ability_defaults.infinite_duress_magic_attack_damage_ratio
                * input.physical_hit_damage,
        true_damage: 0.0,
        stun_duration: ability_defaults.infinite_duress_stun_duration_seconds,
    }]
}

pub(crate) fn execute_jaws_of_the_beast(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = warwick_jaws_of_the_beast_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Warwick.json abilities.basic_ability_1"));
    if input.distance_to_target > ability_defaults.jaws_of_the_beast_cast_range {
        return Vec::new();
    }
    let raw_magic = ability_defaults.jaws_of_the_beast_target_max_health_ratio
        * input.target_max_health.max(0.0)
        + ability_defaults.jaws_of_the_beast_attack_damage_ratio
            * input.physical_hit_damage.max(0.0)
        + ability_defaults.jaws_of_the_beast_ability_power_ratio
            * input.actor_ability_power.max(0.0);
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
        projectile_speed: ability_defaults
            .jaws_of_the_beast_execution
            .projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults
                .jaws_of_the_beast_execution
                .effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: 0.0,
    }]
}

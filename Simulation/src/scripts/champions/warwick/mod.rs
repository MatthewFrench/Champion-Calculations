use super::{
    ChampionBehaviorProfile, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ScriptedEffectHitbox,
};
use crate::defaults::{
    warwick_eternal_hunger_passive_defaults, warwick_infinite_duress_ability_defaults,
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
    if event != ChampionScriptEvent::WarwickInfiniteDuress {
        return None;
    }
    Some(
        warwick_infinite_duress_ability_defaults(CHAMPION_KEY)
            .unwrap_or_else(|| panic!("Missing Characters/Warwick.json abilities.ultimate"))
            .infinite_duress_cooldown_seconds,
    )
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

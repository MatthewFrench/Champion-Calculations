use super::{
    ChampionBehaviorProfile, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ScriptedEffectHitbox,
};
use crate::defaults::{
    vayne_condemn_ability_defaults, vayne_silver_bolts_ability_defaults,
    vayne_tumble_ability_defaults,
};

pub(crate) const CHAMPION_KEY: &str = "vayne";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    let silver_bolts = vayne_silver_bolts_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vayne.json abilities.basic_ability_2"));
    profile.periodic_true_hit_every = silver_bolts.periodic_true_hit_every;
    profile.periodic_true_hit_base = silver_bolts.periodic_true_hit_base;
    profile.periodic_true_hit_ad_ratio = 0.0;
    profile.periodic_true_hit_target_max_health_ratio =
        silver_bolts.periodic_true_hit_target_max_health_ratio;
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn event_cooldown_seconds(event: ChampionScriptEvent) -> Option<f64> {
    match event {
        ChampionScriptEvent::VayneTumbleEmpower => Some(
            vayne_tumble_ability_defaults(CHAMPION_KEY)
                .unwrap_or_else(|| {
                    panic!("Missing Characters/Vayne.json abilities.basic_ability_1")
                })
                .tumble_cooldown_seconds,
        ),
        ChampionScriptEvent::VayneCondemn => Some(
            vayne_condemn_ability_defaults(CHAMPION_KEY)
                .unwrap_or_else(|| {
                    panic!("Missing Characters/Vayne.json abilities.basic_ability_3")
                })
                .condemn_cooldown_seconds,
        ),
        _ => None,
    }
}

pub(crate) fn execute_tumble_empower(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = vayne_tumble_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vayne.json abilities.basic_ability_1"));
    vec![ChampionScriptAction::AddNextAttackBonusPhysical {
        amount: ability_defaults.tumble_bonus_physical_attack_damage_ratio
            * input.physical_hit_damage
            + ability_defaults.tumble_bonus_physical_ability_power_ratio
                * input.actor_ability_power.max(0.0),
        trace_message: "empowered next attack",
    }]
}

pub(crate) fn execute_condemn(input: ChampionScriptExecutionInput) -> Vec<ChampionScriptAction> {
    let ability_defaults = vayne_condemn_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vayne.json abilities.basic_ability_3"));
    if input.distance_to_target > ability_defaults.condemn_cast_range {
        return Vec::new();
    }
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: ability_defaults.condemn_execution.projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults.condemn_execution.effect_hitbox_radius,
        },
        physical: ability_defaults.condemn_physical_base_damage
            + ability_defaults.condemn_bonus_attack_damage_ratio
                * input.actor_bonus_attack_damage.max(0.0),
        magic: 0.0,
        true_damage: 0.0,
        stun_duration: 0.0,
    }]
}

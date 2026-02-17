use super::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ScriptedEffectHitbox, on_ability_bonus_damage,
};
use crate::defaults::sona_crescendo_ability_defaults;

pub(crate) const CHAMPION_KEY: &str = "sona";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn event_cooldown_seconds(event: ChampionScriptEvent) -> Option<f64> {
    if event != ChampionScriptEvent::SonaCrescendo {
        return None;
    }
    Some(
        sona_crescendo_ability_defaults(CHAMPION_KEY)
            .unwrap_or_else(|| panic!("Missing Characters/Sona.json abilities.ultimate"))
            .crescendo_cooldown_seconds,
    )
}

pub(crate) fn execute_crescendo(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = sona_crescendo_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Sona.json abilities.ultimate"));
    if input.distance_to_target > ability_defaults.crescendo_cast_range {
        return Vec::new();
    }
    let raw_magic = ability_defaults.crescendo_magic_base_damage
        + ability_defaults.crescendo_magic_ability_power_ratio * input.actor_ability_power.max(0.0);
    let (extra_magic, extra_true) =
        on_ability_bonus_damage(runtime, raw_magic, input.target_max_health, input.now);
    vec![ChampionScriptAction::ApplyDamage {
        source: input.actor_position,
        projectile_speed: ability_defaults.crescendo_execution.projectile_speed,
        hitbox: ScriptedEffectHitbox::Circle {
            radius: ability_defaults.crescendo_execution.effect_hitbox_radius,
        },
        physical: 0.0,
        magic: raw_magic + extra_magic,
        true_damage: extra_true,
        stun_duration: ability_defaults.crescendo_stun_duration_seconds,
    }]
}

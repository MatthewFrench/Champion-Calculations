use crate::defaults::{
    ChampionBehaviorDefaultsEntry, ChampionBehaviorOverrideEntry, champion_ai_profile,
    champion_behavior_defaults_for_role, champion_behavior_override,
};
use crate::to_norm_key;

use super::{doctor_mundo, morgana, sona, vayne, warwick};

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionBehaviorProfile {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub attack_effect_hitbox_radius: f64,
    pub desired_combat_range: f64,
    pub movement_speed_scale: f64,
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

impl ChampionBehaviorProfile {
    pub(crate) fn default_for(
        is_melee: bool,
        base_attack_range: f64,
        base_attack_projectile_speed: f64,
    ) -> Self {
        let mut profile = profile_from_defaults(champion_behavior_defaults_for_role(is_melee));
        if base_attack_range > 0.0 {
            profile.attack_range = base_attack_range;
        }
        if base_attack_projectile_speed > 0.0 {
            profile.attack_projectile_speed = base_attack_projectile_speed;
        }
        profile.desired_combat_range = profile.attack_range.max(75.0);
        profile.movement_speed_scale = 1.0;
        profile
    }
}

fn profile_from_defaults(source: ChampionBehaviorDefaultsEntry) -> ChampionBehaviorProfile {
    ChampionBehaviorProfile {
        attack_range: source.attack_range,
        attack_windup_seconds: source.attack_windup_seconds,
        attack_projectile_speed: source.attack_projectile_speed,
        attack_effect_hitbox_radius: source.attack_effect_hitbox_radius,
        desired_combat_range: source.attack_range.max(75.0),
        movement_speed_scale: 1.0,
        on_hit_magic_flat: source.on_hit_magic_flat,
        on_hit_magic_ad_ratio: source.on_hit_magic_ad_ratio,
        periodic_true_hit_every: source.periodic_true_hit_every,
        periodic_true_hit_base: source.periodic_true_hit_base,
        periodic_true_hit_ad_ratio: source.periodic_true_hit_ad_ratio,
        periodic_true_hit_target_max_health_ratio: source.periodic_true_hit_target_max_health_ratio,
    }
}

pub(crate) fn apply_behavior_override(champion_key: &str, profile: &mut ChampionBehaviorProfile) {
    fn apply_f64(slot: &mut f64, value: Option<f64>) {
        if let Some(v) = value {
            *slot = v;
        }
    }

    fn apply_usize(slot: &mut usize, value: Option<usize>) {
        if let Some(v) = value {
            *slot = v;
        }
    }

    if let Some(override_entry) = champion_behavior_override(champion_key) {
        let ChampionBehaviorOverrideEntry {
            attack_range,
            attack_windup_seconds,
            attack_projectile_speed,
            attack_effect_hitbox_radius,
            on_hit_magic_flat,
            on_hit_magic_ad_ratio,
            periodic_true_hit_every,
            periodic_true_hit_base,
            periodic_true_hit_ad_ratio,
            periodic_true_hit_target_max_health_ratio,
        } = override_entry;

        apply_f64(&mut profile.attack_range, attack_range);
        apply_f64(&mut profile.attack_windup_seconds, attack_windup_seconds);
        apply_f64(
            &mut profile.attack_projectile_speed,
            attack_projectile_speed,
        );
        apply_f64(
            &mut profile.attack_effect_hitbox_radius,
            attack_effect_hitbox_radius,
        );
        apply_f64(&mut profile.on_hit_magic_flat, on_hit_magic_flat);
        apply_f64(&mut profile.on_hit_magic_ad_ratio, on_hit_magic_ad_ratio);
        apply_usize(
            &mut profile.periodic_true_hit_every,
            periodic_true_hit_every,
        );
        apply_f64(&mut profile.periodic_true_hit_base, periodic_true_hit_base);
        apply_f64(
            &mut profile.periodic_true_hit_ad_ratio,
            periodic_true_hit_ad_ratio,
        );
        apply_f64(
            &mut profile.periodic_true_hit_target_max_health_ratio,
            periodic_true_hit_target_max_health_ratio,
        );
    }
}

pub(crate) fn behavior_profile(
    champion_name: &str,
    is_melee: bool,
    base_attack_range: f64,
    base_attack_projectile_speed: f64,
) -> ChampionBehaviorProfile {
    let mut profile = ChampionBehaviorProfile::default_for(
        is_melee,
        base_attack_range,
        base_attack_projectile_speed,
    );
    match to_norm_key(champion_name).as_str() {
        warwick::CHAMPION_KEY => warwick::apply_behavior(&mut profile),
        vayne::CHAMPION_KEY => vayne::apply_behavior(&mut profile),
        morgana::CHAMPION_KEY => morgana::apply_behavior(&mut profile),
        sona::CHAMPION_KEY => sona::apply_behavior(&mut profile),
        doctor_mundo::CHAMPION_KEY => doctor_mundo::apply_behavior(&mut profile),
        _ => {}
    }
    let ai_profile = champion_ai_profile(champion_name, profile.attack_range);
    profile.desired_combat_range = ai_profile.desired_combat_range;
    profile.movement_speed_scale = ai_profile.movement_speed_scale;
    profile
}

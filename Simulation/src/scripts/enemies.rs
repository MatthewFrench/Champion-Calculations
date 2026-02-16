use crate::to_norm_key;

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemyBehaviorProfile {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub ability_windup_seconds: f64,
    pub ability_projectile_speed: f64,
    pub burst_windup_seconds: f64,
    pub burst_projectile_speed: f64,
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

impl EnemyBehaviorProfile {
    pub(crate) fn default_for(is_melee: bool) -> Self {
        if is_melee {
            Self {
                attack_range: 175.0,
                attack_windup_seconds: 0.24,
                attack_projectile_speed: 0.0,
                ability_windup_seconds: 0.10,
                ability_projectile_speed: 0.0,
                burst_windup_seconds: 0.10,
                burst_projectile_speed: 0.0,
                on_hit_magic_flat: 0.0,
                on_hit_magic_ad_ratio: 0.0,
                periodic_true_hit_every: 0,
                periodic_true_hit_base: 0.0,
                periodic_true_hit_ad_ratio: 0.0,
                periodic_true_hit_target_max_health_ratio: 0.0,
            }
        } else {
            Self {
                attack_range: 550.0,
                attack_windup_seconds: 0.20,
                attack_projectile_speed: 2000.0,
                ability_windup_seconds: 0.10,
                ability_projectile_speed: 1800.0,
                burst_windup_seconds: 0.10,
                burst_projectile_speed: 1800.0,
                on_hit_magic_flat: 0.0,
                on_hit_magic_ad_ratio: 0.0,
                periodic_true_hit_every: 0,
                periodic_true_hit_base: 0.0,
                periodic_true_hit_ad_ratio: 0.0,
                periodic_true_hit_target_max_health_ratio: 0.0,
            }
        }
    }
}

pub(crate) fn behavior_profile(champion_name: &str, is_melee: bool) -> EnemyBehaviorProfile {
    let mut profile = EnemyBehaviorProfile::default_for(is_melee);
    match to_norm_key(champion_name).as_str() {
        "warwick" => {
            profile.attack_range = 175.0;
            profile.attack_windup_seconds = 0.22;
            profile.attack_projectile_speed = 0.0;
            profile.on_hit_magic_flat = 18.0;
            profile.on_hit_magic_ad_ratio = 0.12;
            profile.ability_windup_seconds = 0.08;
        }
        "vayne" => {
            profile.attack_range = 550.0;
            profile.attack_windup_seconds = 0.17;
            profile.attack_projectile_speed = 2500.0;
            profile.periodic_true_hit_every = 3;
            profile.periodic_true_hit_base = 40.0;
            profile.periodic_true_hit_ad_ratio = 0.25;
            profile.periodic_true_hit_target_max_health_ratio = 0.04;
            profile.ability_projectile_speed = 2200.0;
            profile.burst_projectile_speed = 2200.0;
        }
        "morgana" => {
            profile.attack_range = 450.0;
            profile.attack_windup_seconds = 0.20;
            profile.attack_projectile_speed = 1800.0;
            profile.ability_windup_seconds = 0.25;
            profile.ability_projectile_speed = 1600.0;
            profile.burst_windup_seconds = 0.20;
            profile.burst_projectile_speed = 1500.0;
        }
        "sona" => {
            profile.attack_range = 550.0;
            profile.attack_windup_seconds = 0.18;
            profile.attack_projectile_speed = 1900.0;
            profile.ability_windup_seconds = 0.15;
            profile.ability_projectile_speed = 1800.0;
            profile.burst_windup_seconds = 0.12;
            profile.burst_projectile_speed = 1800.0;
        }
        "drmundo" => {
            profile.attack_range = 175.0;
            profile.attack_windup_seconds = 0.24;
            profile.attack_projectile_speed = 0.0;
            profile.ability_windup_seconds = 0.20;
            profile.ability_projectile_speed = 2000.0;
            profile.burst_windup_seconds = 0.15;
            profile.burst_projectile_speed = 1500.0;
        }
        _ => {}
    }
    profile
}

pub(crate) fn on_hit_bonus_damage(
    profile: EnemyBehaviorProfile,
    attack_number: usize,
    attack_damage: f64,
    target_max_health: f64,
) -> (f64, f64) {
    let magic = profile.on_hit_magic_flat + profile.on_hit_magic_ad_ratio * attack_damage;
    if profile.periodic_true_hit_every == 0 {
        return (magic.max(0.0), 0.0);
    }

    let true_damage =
        if attack_number > 0 && attack_number.is_multiple_of(profile.periodic_true_hit_every) {
            profile.periodic_true_hit_base
                + profile.periodic_true_hit_ad_ratio * attack_damage
                + profile.periodic_true_hit_target_max_health_ratio * target_max_health
        } else {
            0.0
        };

    (magic.max(0.0), true_damage.max(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vayne_every_third_hit_adds_true_damage() {
        let profile = behavior_profile("Vayne", false);
        let (_, true_a) = on_hit_bonus_damage(profile, 1, 200.0, 3000.0);
        let (_, true_b) = on_hit_bonus_damage(profile, 2, 200.0, 3000.0);
        let (_, true_c) = on_hit_bonus_damage(profile, 3, 200.0, 3000.0);
        assert_eq!(true_a, 0.0);
        assert_eq!(true_b, 0.0);
        assert!(true_c > 0.0);
    }

    #[test]
    fn melee_defaults_spawn_with_short_range_profile() {
        let melee = EnemyBehaviorProfile::default_for(true);
        let ranged = EnemyBehaviorProfile::default_for(false);
        assert!(melee.attack_range < ranged.attack_range);
        assert_eq!(melee.attack_projectile_speed, 0.0);
        assert!(ranged.attack_projectile_speed > 0.0);
    }
}

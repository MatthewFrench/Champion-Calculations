use crate::{MasterySelection, to_norm_key};

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemyBehaviorProfile {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub ability_windup_seconds: f64,
    pub ability_projectile_speed: f64,
    pub burst_windup_seconds: f64,
    pub burst_projectile_speed: f64,
    pub desired_combat_range: f64,
    pub movement_speed_scale: f64,
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
                desired_combat_range: 135.0,
                movement_speed_scale: 1.0,
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
                desired_combat_range: 500.0,
                movement_speed_scale: 1.0,
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
            profile.desired_combat_range = 130.0;
            profile.movement_speed_scale = 1.08;
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
            profile.desired_combat_range = 520.0;
            profile.movement_speed_scale = 1.10;
        }
        "morgana" => {
            profile.attack_range = 450.0;
            profile.attack_windup_seconds = 0.20;
            profile.attack_projectile_speed = 1800.0;
            profile.ability_windup_seconds = 0.25;
            profile.ability_projectile_speed = 1600.0;
            profile.burst_windup_seconds = 0.20;
            profile.burst_projectile_speed = 1500.0;
            profile.desired_combat_range = 500.0;
            profile.movement_speed_scale = 0.95;
        }
        "sona" => {
            profile.attack_range = 550.0;
            profile.attack_windup_seconds = 0.18;
            profile.attack_projectile_speed = 1900.0;
            profile.ability_windup_seconds = 0.15;
            profile.ability_projectile_speed = 1800.0;
            profile.burst_windup_seconds = 0.12;
            profile.burst_projectile_speed = 1800.0;
            profile.desired_combat_range = 520.0;
            profile.movement_speed_scale = 1.02;
        }
        "drmundo" => {
            profile.attack_range = 175.0;
            profile.attack_windup_seconds = 0.24;
            profile.attack_projectile_speed = 0.0;
            profile.ability_windup_seconds = 0.20;
            profile.ability_projectile_speed = 2000.0;
            profile.burst_windup_seconds = 0.15;
            profile.burst_projectile_speed = 1500.0;
            profile.desired_combat_range = 140.0;
            profile.movement_speed_scale = 0.98;
        }
        _ => {}
    }
    profile
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EnemyScriptEvent {
    WarwickInfiniteDuress,
    VayneTumbleEmpower,
    MorganaDarkBinding,
    MorganaSoulShackles,
    MorganaSoulShacklesDetonate,
    SonaCrescendo,
    DrMundoInfectedCleaver,
    YasuoWindWall,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemyScriptSchedule {
    pub event: EnemyScriptEvent,
    pub start_offset_seconds: f64,
    pub interval_seconds: f64,
}

pub(crate) fn scripted_event_schedules(champion_name: &str) -> Vec<EnemyScriptSchedule> {
    match to_norm_key(champion_name).as_str() {
        "warwick" => vec![EnemyScriptSchedule {
            event: EnemyScriptEvent::WarwickInfiniteDuress,
            start_offset_seconds: 7.0,
            interval_seconds: 15.0,
        }],
        "vayne" => vec![EnemyScriptSchedule {
            event: EnemyScriptEvent::VayneTumbleEmpower,
            start_offset_seconds: 2.2,
            interval_seconds: 5.0,
        }],
        "morgana" => vec![
            EnemyScriptSchedule {
                event: EnemyScriptEvent::MorganaDarkBinding,
                start_offset_seconds: 3.0,
                interval_seconds: 10.0,
            },
            EnemyScriptSchedule {
                event: EnemyScriptEvent::MorganaSoulShackles,
                start_offset_seconds: 8.0,
                interval_seconds: 22.0,
            },
        ],
        "sona" => vec![EnemyScriptSchedule {
            event: EnemyScriptEvent::SonaCrescendo,
            start_offset_seconds: 9.0,
            interval_seconds: 20.0,
        }],
        "drmundo" => vec![EnemyScriptSchedule {
            event: EnemyScriptEvent::DrMundoInfectedCleaver,
            start_offset_seconds: 2.0,
            interval_seconds: 4.0,
        }],
        "yasuo" => vec![EnemyScriptSchedule {
            event: EnemyScriptEvent::YasuoWindWall,
            start_offset_seconds: 5.0,
            interval_seconds: 18.0,
        }],
        _ => Vec::new(),
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EnemyLoadoutRuntime {
    has_lethal_tempo: bool,
    has_grasp: bool,
    has_kraken: bool,
    has_blade_of_the_ruined_king: bool,
    has_heartsteel: bool,
    has_liandry: bool,
    has_luden: bool,
    has_guinsoo: bool,
    has_fervor: bool,
    has_thunderlords: bool,
    has_perseverance: bool,

    pub attacks_landed: usize,
    pub lethal_tempo_stacks: usize,
    pub guinsoo_stacks: usize,
    pub fervor_stacks: usize,
    pub thunderlords_stacks: usize,
    pub grasp_ready_at: f64,
    pub heartsteel_ready_at: f64,
    pub luden_ready_at: f64,
}

impl Default for EnemyLoadoutRuntime {
    fn default() -> Self {
        Self {
            has_lethal_tempo: false,
            has_grasp: false,
            has_kraken: false,
            has_blade_of_the_ruined_king: false,
            has_heartsteel: false,
            has_liandry: false,
            has_luden: false,
            has_guinsoo: false,
            has_fervor: false,
            has_thunderlords: false,
            has_perseverance: false,
            attacks_landed: 0,
            lethal_tempo_stacks: 0,
            guinsoo_stacks: 0,
            fervor_stacks: 0,
            thunderlords_stacks: 0,
            grasp_ready_at: 0.0,
            heartsteel_ready_at: 0.0,
            luden_ready_at: 0.0,
        }
    }
}

pub(crate) fn build_enemy_loadout_runtime(
    item_names: &[String],
    rune_names: &[String],
    masteries: &[MasterySelection],
) -> EnemyLoadoutRuntime {
    let mut runtime = EnemyLoadoutRuntime::default();

    for item in item_names {
        match to_norm_key(item).as_str() {
            "krakenslayer" => runtime.has_kraken = true,
            "bladeoftheruinedking" => runtime.has_blade_of_the_ruined_king = true,
            "heartsteel" => runtime.has_heartsteel = true,
            "liandrystorment" => runtime.has_liandry = true,
            "ludensecho" => runtime.has_luden = true,
            "guinsoosrageblade" => runtime.has_guinsoo = true,
            _ => {}
        }
    }

    for rune in rune_names {
        match to_norm_key(rune).as_str() {
            "lethaltempo" => runtime.has_lethal_tempo = true,
            "graspoftheundying" => runtime.has_grasp = true,
            _ => {}
        }
    }

    for mastery in masteries {
        match to_norm_key(&mastery.name).as_str() {
            "fervorofbattle" => runtime.has_fervor = true,
            "thunderlordsdecree" => runtime.has_thunderlords = true,
            "perseverance" => runtime.has_perseverance = true,
            _ => {}
        }
    }

    runtime
}

pub(crate) fn attack_speed_multiplier(runtime: &EnemyLoadoutRuntime) -> f64 {
    let lethal_tempo_bonus = if runtime.has_lethal_tempo {
        0.04 * runtime.lethal_tempo_stacks as f64
    } else {
        0.0
    };
    let guinsoo_bonus = if runtime.has_guinsoo {
        0.02 * runtime.guinsoo_stacks as f64
    } else {
        0.0
    };
    1.0 + lethal_tempo_bonus + guinsoo_bonus
}

pub(crate) fn clear_transient_combat_state(runtime: &mut EnemyLoadoutRuntime) {
    runtime.attacks_landed = 0;
    runtime.lethal_tempo_stacks = 0;
    runtime.guinsoo_stacks = 0;
    runtime.fervor_stacks = 0;
    runtime.thunderlords_stacks = 0;
}

pub(crate) fn on_hit_bonus_damage(
    profile: EnemyBehaviorProfile,
    runtime: &mut EnemyLoadoutRuntime,
    attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_max_health: f64,
    now: f64,
) -> (f64, f64, f64) {
    runtime.attacks_landed += 1;
    if runtime.has_lethal_tempo {
        runtime.lethal_tempo_stacks = (runtime.lethal_tempo_stacks + 1).min(6);
    }
    if runtime.has_guinsoo {
        runtime.guinsoo_stacks = (runtime.guinsoo_stacks + 1).min(8);
    }
    if runtime.has_fervor {
        runtime.fervor_stacks = (runtime.fervor_stacks + 1).min(8);
    }

    let fervor_bonus_ad = if runtime.has_fervor {
        2.0 * runtime.fervor_stacks as f64
    } else {
        0.0
    };

    let magic = profile.on_hit_magic_flat
        + profile.on_hit_magic_ad_ratio * (attack_damage + fervor_bonus_ad);
    let mut extra_physical = 0.0;
    let mut extra_magic = magic.max(0.0);
    let mut extra_true = 0.0;

    if profile.periodic_true_hit_every > 0
        && runtime
            .attacks_landed
            .is_multiple_of(profile.periodic_true_hit_every)
    {
        extra_true += profile.periodic_true_hit_base
            + profile.periodic_true_hit_ad_ratio * (attack_damage + fervor_bonus_ad)
            + profile.periodic_true_hit_target_max_health_ratio * target_max_health;
    }

    if runtime.has_blade_of_the_ruined_king {
        extra_physical += 0.06 * target_current_health.max(0.0);
    }

    if runtime.has_kraken && runtime.attacks_landed.is_multiple_of(3) {
        extra_true += 65.0 + 0.45 * (attack_damage + fervor_bonus_ad);
    }

    if runtime.has_grasp && now >= runtime.grasp_ready_at {
        extra_magic += 8.0 + 0.035 * target_max_health.max(0.0);
        runtime.grasp_ready_at = now + 4.0;
    }

    if runtime.has_heartsteel && now >= runtime.heartsteel_ready_at {
        extra_physical += 70.0 + 0.06 * attacker_max_health.max(0.0);
        runtime.heartsteel_ready_at = now + 5.0;
    }

    if runtime.has_thunderlords {
        runtime.thunderlords_stacks += 1;
        if runtime.thunderlords_stacks >= 3 {
            extra_magic += 30.0 + 0.30 * (attack_damage + fervor_bonus_ad);
            runtime.thunderlords_stacks = 0;
        }
    }

    (
        extra_physical.max(0.0),
        extra_magic.max(0.0),
        extra_true.max(0.0),
    )
}

pub(crate) fn on_ability_bonus_damage(
    runtime: &mut EnemyLoadoutRuntime,
    ability_raw_damage: f64,
    target_max_health: f64,
    now: f64,
) -> (f64, f64) {
    let mut extra_magic = 0.0;
    let mut extra_true = 0.0;

    if runtime.has_liandry {
        extra_magic += 0.04 * target_max_health.max(0.0);
    }

    if runtime.has_luden && now >= runtime.luden_ready_at {
        extra_magic += 90.0 + 0.10 * ability_raw_damage.max(0.0);
        runtime.luden_ready_at = now + 8.0;
    }

    if runtime.has_thunderlords {
        runtime.thunderlords_stacks += 1;
        if runtime.thunderlords_stacks >= 3 {
            extra_magic += 40.0 + 0.20 * ability_raw_damage.max(0.0);
            runtime.thunderlords_stacks = 0;
        }
    }

    if runtime.has_fervor {
        extra_true += 0.02 * runtime.fervor_stacks as f64 * ability_raw_damage.max(0.0);
    }

    (extra_magic.max(0.0), extra_true.max(0.0))
}

pub(crate) fn tick_regen_heal(
    runtime: &EnemyLoadoutRuntime,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    if !runtime.has_perseverance || max_health <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let health_ratio = (current_health / max_health).clamp(0.0, 1.0);
    let base_regen = 0.0015 * max_health * dt;
    let bonus = if health_ratio <= 0.35 {
        0.0030 * max_health * dt
    } else {
        0.0
    };
    base_regen + bonus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vayne_every_third_hit_adds_true_damage() {
        let profile = behavior_profile("Vayne", false);
        let mut runtime = EnemyLoadoutRuntime::default();
        let (_, _, true_a) =
            on_hit_bonus_damage(profile, &mut runtime, 200.0, 2200.0, 3000.0, 1800.0, 1.0);
        let (_, _, true_b) =
            on_hit_bonus_damage(profile, &mut runtime, 200.0, 2200.0, 3000.0, 1800.0, 2.0);
        let (_, _, true_c) =
            on_hit_bonus_damage(profile, &mut runtime, 200.0, 2200.0, 3000.0, 1800.0, 3.0);
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

    #[test]
    fn lethal_tempo_and_kraken_stack_runtime() {
        let runtime = build_enemy_loadout_runtime(
            &["Kraken Slayer".to_string()],
            &["Lethal Tempo".to_string()],
            &[],
        );
        assert!(attack_speed_multiplier(&runtime) >= 1.0);

        let mut runtime = runtime;
        let profile = behavior_profile("Vayne", false);
        let _ = on_hit_bonus_damage(profile, &mut runtime, 180.0, 2500.0, 3000.0, 1600.0, 1.0);
        let _ = on_hit_bonus_damage(profile, &mut runtime, 180.0, 2500.0, 3000.0, 1600.0, 2.0);
        let (_, _, true_damage) =
            on_hit_bonus_damage(profile, &mut runtime, 180.0, 2500.0, 3000.0, 1600.0, 3.0);
        assert!(runtime.lethal_tempo_stacks > 0);
        assert!(true_damage > 0.0);
    }

    #[test]
    fn clear_transient_state_resets_stack_counters() {
        let mut runtime = build_enemy_loadout_runtime(
            &[
                "Kraken Slayer".to_string(),
                "Guinsoo's Rageblade".to_string(),
            ],
            &["Lethal Tempo".to_string()],
            &[MasterySelection {
                name: "Fervor of Battle".to_string(),
                rank: 1,
            }],
        );
        runtime.attacks_landed = 7;
        runtime.lethal_tempo_stacks = 6;
        runtime.guinsoo_stacks = 8;
        runtime.fervor_stacks = 8;
        runtime.thunderlords_stacks = 2;
        clear_transient_combat_state(&mut runtime);
        assert_eq!(runtime.attacks_landed, 0);
        assert_eq!(runtime.lethal_tempo_stacks, 0);
        assert_eq!(runtime.guinsoo_stacks, 0);
        assert_eq!(runtime.fervor_stacks, 0);
        assert_eq!(runtime.thunderlords_stacks, 0);
    }
}

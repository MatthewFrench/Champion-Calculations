use crate::{MasterySelection, to_norm_key};

use crate::scripts::runtime::loadout_runtime::{
    LoadoutRuntimeState, OnHitEffectProfile, build_loadout_runtime_state,
    calculate_ability_bonus_damage, calculate_on_hit_bonus_damage, loadout_attack_speed_multiplier,
    reset_transient_loadout_state, tick_loadout_regeneration,
};

mod doctor_mundo;
mod morgana;
mod sona;
mod vayne;
mod warwick;
mod yasuo;

pub(crate) type EnemyLoadoutRuntime = LoadoutRuntimeState;

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
        warwick::CHAMPION_KEY => warwick::apply_behavior(&mut profile),
        vayne::CHAMPION_KEY => vayne::apply_behavior(&mut profile),
        morgana::CHAMPION_KEY => morgana::apply_behavior(&mut profile),
        sona::CHAMPION_KEY => sona::apply_behavior(&mut profile),
        doctor_mundo::CHAMPION_KEY => doctor_mundo::apply_behavior(&mut profile),
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
    DoctorMundoInfectedCleaver,
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
        warwick::CHAMPION_KEY => warwick::schedules(),
        vayne::CHAMPION_KEY => vayne::schedules(),
        morgana::CHAMPION_KEY => morgana::schedules(),
        sona::CHAMPION_KEY => sona::schedules(),
        doctor_mundo::CHAMPION_KEY => doctor_mundo::schedules(),
        yasuo::CHAMPION_KEY => yasuo::schedules(),
        _ => Vec::new(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EnemyScriptPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct EnemyScriptExecutionInput {
    pub event: EnemyScriptEvent,
    pub enemy_position: EnemyScriptPoint,
    pub controlled_champion_position: EnemyScriptPoint,
    pub enemy_distance_to_controlled_champion: f64,
    pub enemy_physical_hit_damage: f64,
    pub enemy_burst_magic_damage: f64,
    pub enemy_ability_projectile_speed: f64,
    pub enemy_burst_projectile_speed: f64,
    pub controlled_champion_current_health: f64,
    pub controlled_champion_max_health: f64,
    pub now: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum EnemyScriptAction {
    AddNextAttackBonusPhysical {
        amount: f64,
        trace_message: &'static str,
    },
    ApplyDamage {
        source: EnemyScriptPoint,
        projectile_speed: f64,
        physical: f64,
        magic: f64,
        true_damage: f64,
        stun_duration: f64,
    },
    ScheduleFollowup {
        delay_seconds: f64,
        priority: i32,
        event: EnemyScriptEvent,
    },
    CreateProjectileBlockZone {
        start: EnemyScriptPoint,
        end: EnemyScriptPoint,
        duration_seconds: f64,
    },
}

pub(crate) fn execute_enemy_script_event(
    input: EnemyScriptExecutionInput,
    runtime: &mut EnemyLoadoutRuntime,
) -> Vec<EnemyScriptAction> {
    match input.event {
        EnemyScriptEvent::WarwickInfiniteDuress => warwick::execute_infinite_duress(input),
        EnemyScriptEvent::VayneTumbleEmpower => vayne::execute_tumble_empower(input),
        EnemyScriptEvent::MorganaDarkBinding => morgana::execute_dark_binding(input, runtime),
        EnemyScriptEvent::MorganaSoulShackles => morgana::execute_soul_shackles(input),
        EnemyScriptEvent::MorganaSoulShacklesDetonate => {
            morgana::execute_soul_shackles_detonate(input)
        }
        EnemyScriptEvent::SonaCrescendo => sona::execute_crescendo(input, runtime),
        EnemyScriptEvent::DoctorMundoInfectedCleaver => {
            doctor_mundo::execute_infected_cleaver(input, runtime)
        }
        EnemyScriptEvent::YasuoWindWall => yasuo::execute_wind_wall(input),
    }
}

pub(crate) fn build_enemy_loadout_runtime(
    item_names: &[String],
    rune_names: &[String],
    masteries: &[MasterySelection],
) -> EnemyLoadoutRuntime {
    build_loadout_runtime_state(item_names, rune_names, masteries)
}

pub(crate) fn attack_speed_multiplier(runtime: &EnemyLoadoutRuntime) -> f64 {
    loadout_attack_speed_multiplier(runtime)
}

pub(crate) fn clear_transient_combat_state(runtime: &mut EnemyLoadoutRuntime) {
    reset_transient_loadout_state(runtime)
}

fn on_hit_effect_profile(profile: EnemyBehaviorProfile) -> OnHitEffectProfile {
    OnHitEffectProfile {
        on_hit_magic_flat: profile.on_hit_magic_flat,
        on_hit_magic_ad_ratio: profile.on_hit_magic_ad_ratio,
        periodic_true_hit_every: profile.periodic_true_hit_every,
        periodic_true_hit_base: profile.periodic_true_hit_base,
        periodic_true_hit_ad_ratio: profile.periodic_true_hit_ad_ratio,
        periodic_true_hit_target_max_health_ratio: profile
            .periodic_true_hit_target_max_health_ratio,
    }
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
    calculate_on_hit_bonus_damage(
        on_hit_effect_profile(profile),
        runtime,
        attack_damage,
        target_current_health,
        target_max_health,
        attacker_max_health,
        now,
    )
}

pub(crate) fn on_ability_bonus_damage(
    runtime: &mut EnemyLoadoutRuntime,
    ability_raw_damage: f64,
    target_max_health: f64,
    now: f64,
) -> (f64, f64) {
    calculate_ability_bonus_damage(runtime, ability_raw_damage, target_max_health, now)
}

pub(crate) fn tick_regen_heal(
    runtime: &EnemyLoadoutRuntime,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    tick_loadout_regeneration(runtime, current_health, max_health, dt)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_script_input(event: EnemyScriptEvent) -> EnemyScriptExecutionInput {
        EnemyScriptExecutionInput {
            event,
            enemy_position: EnemyScriptPoint { x: 100.0, y: 0.0 },
            controlled_champion_position: EnemyScriptPoint { x: 0.0, y: 0.0 },
            enemy_distance_to_controlled_champion: 100.0,
            enemy_physical_hit_damage: 200.0,
            enemy_burst_magic_damage: 300.0,
            enemy_ability_projectile_speed: 1500.0,
            enemy_burst_projectile_speed: 1600.0,
            controlled_champion_current_health: 1800.0,
            controlled_champion_max_health: 2500.0,
            now: 4.0,
        }
    }

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
    fn warwick_script_skips_when_target_is_out_of_range() {
        let mut runtime = EnemyLoadoutRuntime::default();
        let mut input = base_script_input(EnemyScriptEvent::WarwickInfiniteDuress);
        input.enemy_distance_to_controlled_champion = 701.0;
        assert!(execute_enemy_script_event(input, &mut runtime).is_empty());
    }

    #[test]
    fn morgana_soul_shackles_emits_damage_and_followup() {
        let mut runtime = EnemyLoadoutRuntime::default();
        let actions = execute_enemy_script_event(
            base_script_input(EnemyScriptEvent::MorganaSoulShackles),
            &mut runtime,
        );
        assert_eq!(actions.len(), 2);
        match actions[0] {
            EnemyScriptAction::ApplyDamage {
                magic,
                stun_duration,
                ..
            } => {
                assert_eq!(magic, 70.0);
                assert_eq!(stun_duration, 0.0);
            }
            _ => panic!("expected damage action"),
        }
        match actions[1] {
            EnemyScriptAction::ScheduleFollowup {
                delay_seconds,
                priority,
                event,
            } => {
                assert_eq!(delay_seconds, 2.5);
                assert_eq!(priority, 11);
                assert_eq!(event, EnemyScriptEvent::MorganaSoulShacklesDetonate);
            }
            _ => panic!("expected followup action"),
        }
    }

    #[test]
    fn yasuo_wind_wall_emits_block_zone_geometry() {
        let mut runtime = EnemyLoadoutRuntime::default();
        let actions = execute_enemy_script_event(
            base_script_input(EnemyScriptEvent::YasuoWindWall),
            &mut runtime,
        );
        assert_eq!(actions.len(), 1);
        match actions[0] {
            EnemyScriptAction::CreateProjectileBlockZone {
                start,
                end,
                duration_seconds,
            } => {
                assert_eq!(duration_seconds, 4.0);
                assert_ne!(start, end);
            }
            _ => panic!("expected wind wall action"),
        }
    }
}

use crate::{MasterySelection, to_norm_key};

use crate::scripts::runtime::loadout_runtime::{
    LoadoutRuntimeState, OnHitEffectProfile, build_loadout_runtime_state,
    calculate_ability_bonus_damage, calculate_on_hit_bonus_damage, loadout_attack_speed_multiplier,
    reset_transient_loadout_state, tick_loadout_regeneration,
};

pub(crate) mod vladimir;

mod doctor_mundo;
mod morgana;
mod sona;
mod vayne;
mod warwick;
mod yasuo;

pub(crate) type ChampionLoadoutRuntime = LoadoutRuntimeState;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionBehaviorProfile {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub attack_effect_hitbox_radius: f64,
    pub ability_windup_seconds: f64,
    pub ability_projectile_speed: f64,
    pub ability_effect_hitbox_radius: f64,
    pub burst_windup_seconds: f64,
    pub burst_projectile_speed: f64,
    pub burst_effect_hitbox_radius: f64,
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
    pub(crate) fn default_for(is_melee: bool) -> Self {
        if is_melee {
            Self {
                attack_range: 175.0,
                attack_windup_seconds: 0.24,
                attack_projectile_speed: 0.0,
                attack_effect_hitbox_radius: 80.0,
                ability_windup_seconds: 0.10,
                ability_projectile_speed: 0.0,
                ability_effect_hitbox_radius: 90.0,
                burst_windup_seconds: 0.10,
                burst_projectile_speed: 0.0,
                burst_effect_hitbox_radius: 120.0,
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
                attack_effect_hitbox_radius: 45.0,
                ability_windup_seconds: 0.10,
                ability_projectile_speed: 1800.0,
                ability_effect_hitbox_radius: 70.0,
                burst_windup_seconds: 0.10,
                burst_projectile_speed: 1800.0,
                burst_effect_hitbox_radius: 100.0,
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

pub(crate) fn behavior_profile(champion_name: &str, is_melee: bool) -> ChampionBehaviorProfile {
    let mut profile = ChampionBehaviorProfile::default_for(is_melee);
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
pub(crate) enum ChampionScriptEvent {
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
pub(crate) struct ChampionScriptSchedule {
    pub event: ChampionScriptEvent,
    pub start_offset_seconds: f64,
    pub interval_seconds: f64,
}

pub(crate) fn scripted_champion_event_schedules(
    champion_name: &str,
) -> Vec<ChampionScriptSchedule> {
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
pub(crate) struct ChampionScriptPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ScriptedEffectHitbox {
    Circle { radius: f64 },
}

impl ScriptedEffectHitbox {
    pub(crate) fn radius(self) -> f64 {
        match self {
            Self::Circle { radius } => radius.max(0.0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionScriptExecutionInput {
    pub event: ChampionScriptEvent,
    pub actor_position: ChampionScriptPoint,
    pub target_position: ChampionScriptPoint,
    pub distance_to_target: f64,
    pub physical_hit_damage: f64,
    pub burst_magic_damage: f64,
    pub ability_projectile_speed: f64,
    pub burst_projectile_speed: f64,
    pub target_current_health: f64,
    pub target_max_health: f64,
    pub now: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum ChampionScriptAction {
    AddNextAttackBonusPhysical {
        amount: f64,
        trace_message: &'static str,
    },
    ApplyDamage {
        source: ChampionScriptPoint,
        projectile_speed: f64,
        hitbox: ScriptedEffectHitbox,
        physical: f64,
        magic: f64,
        true_damage: f64,
        stun_duration: f64,
    },
    ScheduleFollowup {
        delay_seconds: f64,
        priority: i32,
        event: ChampionScriptEvent,
    },
    CreateProjectileBlockZone {
        start: ChampionScriptPoint,
        end: ChampionScriptPoint,
        half_width: f64,
        duration_seconds: f64,
    },
}

pub(crate) fn execute_champion_script_event(
    input: ChampionScriptExecutionInput,
    runtime: &mut ChampionLoadoutRuntime,
) -> Vec<ChampionScriptAction> {
    match input.event {
        ChampionScriptEvent::WarwickInfiniteDuress => warwick::execute_infinite_duress(input),
        ChampionScriptEvent::VayneTumbleEmpower => vayne::execute_tumble_empower(input),
        ChampionScriptEvent::MorganaDarkBinding => morgana::execute_dark_binding(input, runtime),
        ChampionScriptEvent::MorganaSoulShackles => morgana::execute_soul_shackles(input),
        ChampionScriptEvent::MorganaSoulShacklesDetonate => {
            morgana::execute_soul_shackles_detonate(input)
        }
        ChampionScriptEvent::SonaCrescendo => sona::execute_crescendo(input, runtime),
        ChampionScriptEvent::DoctorMundoInfectedCleaver => {
            doctor_mundo::execute_infected_cleaver(input, runtime)
        }
        ChampionScriptEvent::YasuoWindWall => yasuo::execute_wind_wall(input),
    }
}

pub(crate) fn build_champion_loadout_runtime(
    item_names: &[String],
    rune_names: &[String],
    masteries: &[MasterySelection],
) -> ChampionLoadoutRuntime {
    build_loadout_runtime_state(item_names, rune_names, masteries)
}

pub(crate) fn attack_speed_multiplier(runtime: &ChampionLoadoutRuntime) -> f64 {
    loadout_attack_speed_multiplier(runtime)
}

pub(crate) fn clear_transient_combat_state(runtime: &mut ChampionLoadoutRuntime) {
    reset_transient_loadout_state(runtime)
}

fn on_hit_effect_profile(profile: ChampionBehaviorProfile) -> OnHitEffectProfile {
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
    profile: ChampionBehaviorProfile,
    runtime: &mut ChampionLoadoutRuntime,
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
    runtime: &mut ChampionLoadoutRuntime,
    ability_raw_damage: f64,
    target_max_health: f64,
    now: f64,
) -> (f64, f64) {
    calculate_ability_bonus_damage(runtime, ability_raw_damage, target_max_health, now)
}

pub(crate) fn tick_regen_heal(
    runtime: &ChampionLoadoutRuntime,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    tick_loadout_regeneration(runtime, current_health, max_health, dt)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_script_input(event: ChampionScriptEvent) -> ChampionScriptExecutionInput {
        ChampionScriptExecutionInput {
            event,
            actor_position: ChampionScriptPoint { x: 100.0, y: 0.0 },
            target_position: ChampionScriptPoint { x: 0.0, y: 0.0 },
            distance_to_target: 100.0,
            physical_hit_damage: 200.0,
            burst_magic_damage: 300.0,
            ability_projectile_speed: 1500.0,
            burst_projectile_speed: 1600.0,
            target_current_health: 1800.0,
            target_max_health: 2500.0,
            now: 4.0,
        }
    }

    #[test]
    fn vayne_every_third_hit_adds_true_damage() {
        let profile = behavior_profile("Vayne", false);
        let mut runtime = ChampionLoadoutRuntime::default();
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
        let melee = ChampionBehaviorProfile::default_for(true);
        let ranged = ChampionBehaviorProfile::default_for(false);
        assert!(melee.attack_range < ranged.attack_range);
        assert_eq!(melee.attack_projectile_speed, 0.0);
        assert!(ranged.attack_projectile_speed > 0.0);
    }

    #[test]
    fn warwick_script_skips_when_target_is_out_of_range() {
        let mut runtime = ChampionLoadoutRuntime::default();
        let mut input = base_script_input(ChampionScriptEvent::WarwickInfiniteDuress);
        input.distance_to_target = 701.0;
        assert!(execute_champion_script_event(input, &mut runtime).is_empty());
    }

    #[test]
    fn morgana_soul_shackles_emits_damage_and_followup() {
        let mut runtime = ChampionLoadoutRuntime::default();
        let actions = execute_champion_script_event(
            base_script_input(ChampionScriptEvent::MorganaSoulShackles),
            &mut runtime,
        );
        assert_eq!(actions.len(), 2);
        match actions[0] {
            ChampionScriptAction::ApplyDamage {
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
            ChampionScriptAction::ScheduleFollowup {
                delay_seconds,
                priority,
                event,
            } => {
                assert_eq!(delay_seconds, 2.5);
                assert_eq!(priority, 11);
                assert_eq!(event, ChampionScriptEvent::MorganaSoulShacklesDetonate);
            }
            _ => panic!("expected followup action"),
        }
    }

    #[test]
    fn yasuo_wind_wall_emits_block_zone_geometry() {
        let mut runtime = ChampionLoadoutRuntime::default();
        let actions = execute_champion_script_event(
            base_script_input(ChampionScriptEvent::YasuoWindWall),
            &mut runtime,
        );
        assert_eq!(actions.len(), 1);
        match actions[0] {
            ChampionScriptAction::CreateProjectileBlockZone {
                start,
                end,
                half_width,
                duration_seconds,
            } => {
                assert_eq!(half_width, 40.0);
                assert_eq!(duration_seconds, 4.0);
                assert_ne!(start, end);
            }
            _ => panic!("expected wind wall action"),
        }
    }
}

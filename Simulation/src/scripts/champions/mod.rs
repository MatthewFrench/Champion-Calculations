use crate::defaults::{
    ChampionBehaviorDefaultsEntry, ChampionBehaviorOverrideEntry, champion_ai_profile,
    champion_behavior_defaults_for_role, champion_behavior_override,
};
use crate::to_norm_key;

use crate::scripts::runtime::loadout_runtime::{
    LoadoutRuntimeState, OnHitEffectProfile, build_loadout_runtime_state,
    calculate_ability_bonus_damage, calculate_on_hit_bonus_damage, describe_runtime_cooldowns,
    describe_runtime_stacks, loadout_attack_speed_multiplier, reset_transient_loadout_state,
    tick_loadout_regeneration,
};

pub(crate) mod vladimir;

mod doctor_mundo;
mod morgana;
mod sona;
mod vayne;
mod warwick;

pub(crate) type ChampionLoadoutRuntime = LoadoutRuntimeState;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ChampionScriptEvent {
    WarwickInfiniteDuress,
    VayneTumbleEmpower,
    MorganaDarkBinding,
    MorganaSoulShackles,
    MorganaSoulShacklesDetonate,
    SonaCrescendo,
    DoctorMundoInfectedBonesaw,
}

pub(crate) fn scripted_champion_events(champion_name: &str) -> Vec<ChampionScriptEvent> {
    match to_norm_key(champion_name).as_str() {
        warwick::CHAMPION_KEY => vec![ChampionScriptEvent::WarwickInfiniteDuress],
        vayne::CHAMPION_KEY => vec![ChampionScriptEvent::VayneTumbleEmpower],
        morgana::CHAMPION_KEY => vec![
            ChampionScriptEvent::MorganaDarkBinding,
            ChampionScriptEvent::MorganaSoulShackles,
        ],
        sona::CHAMPION_KEY => vec![ChampionScriptEvent::SonaCrescendo],
        doctor_mundo::CHAMPION_KEY => vec![ChampionScriptEvent::DoctorMundoInfectedBonesaw],
        _ => Vec::new(),
    }
}

pub(crate) fn champion_script_event_cooldown_seconds(
    champion_name: &str,
    event: ChampionScriptEvent,
) -> Option<f64> {
    match to_norm_key(champion_name).as_str() {
        warwick::CHAMPION_KEY => warwick::event_cooldown_seconds(event),
        vayne::CHAMPION_KEY => vayne::event_cooldown_seconds(event),
        morgana::CHAMPION_KEY => morgana::event_cooldown_seconds(event),
        sona::CHAMPION_KEY => sona::event_cooldown_seconds(event),
        doctor_mundo::CHAMPION_KEY => doctor_mundo::event_cooldown_seconds(event),
        _ => None,
    }
}

pub(crate) fn champion_script_event_label(event: ChampionScriptEvent) -> &'static str {
    match event {
        ChampionScriptEvent::WarwickInfiniteDuress => "Infinite Duress",
        ChampionScriptEvent::VayneTumbleEmpower => "Tumble Empower",
        ChampionScriptEvent::MorganaDarkBinding => "Dark Binding",
        ChampionScriptEvent::MorganaSoulShackles => "Soul Shackles",
        ChampionScriptEvent::MorganaSoulShacklesDetonate => "Soul Shackles Detonate",
        ChampionScriptEvent::SonaCrescendo => "Crescendo",
        ChampionScriptEvent::DoctorMundoInfectedBonesaw => "Infected Bonesaw",
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
    pub distance_to_target: f64,
    pub physical_hit_damage: f64,
    pub actor_ability_power: f64,
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
        ChampionScriptEvent::DoctorMundoInfectedBonesaw => {
            doctor_mundo::execute_infected_bonesaw(input, runtime)
        }
    }
}

pub(crate) fn build_champion_loadout_runtime(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
) -> ChampionLoadoutRuntime {
    build_loadout_runtime_state(item_names, rune_names, item_haste)
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

pub(crate) fn describe_runtime_effect_cooldowns(
    runtime: &ChampionLoadoutRuntime,
    now: f64,
) -> Vec<String> {
    describe_runtime_cooldowns(runtime, now)
}

pub(crate) fn describe_runtime_effect_stacks(runtime: &ChampionLoadoutRuntime) -> Vec<String> {
    describe_runtime_stacks(runtime)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_script_input(event: ChampionScriptEvent) -> ChampionScriptExecutionInput {
        ChampionScriptExecutionInput {
            event,
            actor_position: ChampionScriptPoint { x: 100.0, y: 0.0 },
            distance_to_target: 100.0,
            physical_hit_damage: 200.0,
            actor_ability_power: 300.0,
            target_current_health: 1800.0,
            target_max_health: 2500.0,
            now: 4.0,
        }
    }

    #[test]
    fn vayne_every_third_hit_adds_true_damage() {
        let profile = behavior_profile("Vayne", false, 550.0, 2000.0);
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
        let melee = ChampionBehaviorProfile::default_for(true, 125.0, 0.0);
        let ranged = ChampionBehaviorProfile::default_for(false, 550.0, 2000.0);
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
                assert!((magic - 590.0).abs() < 1e-9);
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
                assert_eq!(delay_seconds, 3.0);
                assert_eq!(priority, 11);
                assert_eq!(event, ChampionScriptEvent::MorganaSoulShacklesDetonate);
            }
            _ => panic!("expected followup action"),
        }
    }
}

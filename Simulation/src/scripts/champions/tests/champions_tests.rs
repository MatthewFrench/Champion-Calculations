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

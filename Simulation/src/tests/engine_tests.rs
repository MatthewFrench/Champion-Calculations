use super::*;

#[test]
fn projectile_travel_time_handles_instant_and_ranged() {
    assert_eq!(projectile_travel_seconds(400.0, 0.0), 0.0);
    assert!((projectile_travel_seconds(500.0, 2000.0) - 0.25).abs() < 1e-9);
}

#[test]
fn spawn_positions_keep_melee_closer_than_ranged() {
    let melee = ChampionBehaviorProfile::default_for(true, 125.0, 0.0);
    let ranged = ChampionBehaviorProfile::default_for(false, 550.0, 2000.0);
    let melee_pos = enemy_spawn_position(0, 5, melee);
    let ranged_pos = enemy_spawn_position(0, 5, ranged);
    let origin = Vec2 { x: 0.0, y: 0.0 };
    assert!(melee_pos.distance_to(origin) < ranged_pos.distance_to(origin));
}

#[test]
fn projectile_path_intersection_detects_blocks() {
    let source = Vec2 { x: 0.0, y: 0.0 };
    let target = Vec2 { x: 1000.0, y: 0.0 };
    let wall_start = Vec2 { x: 500.0, y: 200.0 };
    let wall_end = Vec2 {
        x: 500.0,
        y: -200.0,
    };
    assert!(line_segments_intersect(
        source, target, wall_start, wall_end
    ));

    let miss_start = Vec2 { x: 500.0, y: 300.0 };
    let miss_end = Vec2 { x: 500.0, y: 600.0 };
    assert!(!line_segments_intersect(
        source, target, miss_start, miss_end
    ));

    let colinear_disjoint_start = Vec2 { x: 1200.0, y: 0.0 };
    let colinear_disjoint_end = Vec2 { x: 1400.0, y: 0.0 };
    assert!(!line_segments_intersect(
        source,
        target,
        colinear_disjoint_start,
        colinear_disjoint_end
    ));
}

#[test]
fn path_hits_circle_respects_effect_and_target_hitbox() {
    let source = Vec2 { x: 0.0, y: 0.0 };
    let aim = Vec2 { x: 1000.0, y: 0.0 };
    let near_target = Vec2 { x: 1000.0, y: 70.0 };
    let far_target = Vec2 {
        x: 1000.0,
        y: 120.0,
    };
    assert!(path_hits_circle(source, aim, near_target, 65.0, 10.0));
    assert!(!path_hits_circle(source, aim, far_target, 65.0, 10.0));
}

fn test_controlled_champion_base() -> ChampionBase {
    ChampionBase {
        name: "Vladimir".to_string(),
        base_health: 2000.0,
        health_per_level: 0.0,
        base_armor: 45.0,
        armor_per_level: 0.0,
        base_magic_resist: 45.0,
        magic_resist_per_level: 0.0,
        base_attack_damage: 60.0,
        attack_damage_per_level: 0.0,
        base_attack_speed: 0.658,
        attack_speed_per_level_percent: 0.0,
        base_attack_range: 450.0,
        base_attack_projectile_speed: 1600.0,
        base_move_speed: 335.0,
        is_melee: false,
    }
}

fn test_enemy_base(name: &str) -> ChampionBase {
    ChampionBase {
        name: name.to_string(),
        base_health: 2200.0,
        health_per_level: 0.0,
        base_armor: 35.0,
        armor_per_level: 0.0,
        base_magic_resist: 35.0,
        magic_resist_per_level: 0.0,
        base_attack_damage: 80.0,
        attack_damage_per_level: 0.0,
        base_attack_speed: 0.70,
        attack_speed_per_level_percent: 0.0,
        base_attack_range: 550.0,
        base_attack_projectile_speed: 1800.0,
        base_move_speed: 330.0,
        is_melee: false,
    }
}

fn test_enemy_base_with_role(name: &str, is_melee: bool) -> ChampionBase {
    let mut base = test_enemy_base(name);
    base.is_melee = is_melee;
    if is_melee {
        base.base_attack_range = 125.0;
        base.base_attack_projectile_speed = 0.0;
    }
    base
}

fn test_enemy(name: &str) -> EnemyConfig {
    EnemyConfig {
        id: name.to_string(),
        name: name.to_string(),
        level: 20,
        base: test_enemy_base(name),
        spawn_position_xy: None,
        movement_mode: OpponentMovementMode::MaintainCombatRange,
        loadout_item_names: Vec::new(),
        loadout_rune_names: Vec::new(),
        loadout_shards: Vec::new(),
        stack_overrides: HashMap::new(),
    }
}

fn test_enemy_with_role(name: &str, is_melee: bool) -> EnemyConfig {
    let mut enemy = test_enemy(name);
    enemy.base = test_enemy_base_with_role(name, is_melee);
    enemy
}

fn test_simulation(
    max_time_seconds: f64,
    controlled_champion_script_enabled: bool,
) -> SimulationConfig {
    SimulationConfig {
        dt: 1.0 / 30.0,
        server_tick_rate_hz: 30.0,
        champion_level: 20,
        max_time_seconds,
        combat_seed: None,
        controlled_champion_script: if controlled_champion_script_enabled {
            crate::scripts::champions::resolve_controlled_champion_script("Vladimir")
        } else {
            None
        },
        zhonya_duration_seconds: 2.5,
        zhonya_cooldown_seconds: 120.0,
        zhonya_trigger_health_percent: 0.0,
        ga_cooldown_seconds: 300.0,
        ga_revive_duration_seconds: 4.0,
        ga_revive_base_health_ratio: 0.3,
        protoplasm_trigger_health_percent: 0.0,
        protoplasm_bonus_health: 0.0,
        protoplasm_heal_total: 0.0,
        protoplasm_duration_seconds: 0.0,
        stack_overrides: HashMap::new(),
        urf_respawn_flat_reduction_seconds: 3.0,
        urf_respawn_extrapolation_per_level: 2.5,
        urf_respawn_time_scaling_enabled: true,
        urf_respawn_time_scaling_start_seconds: 300.0,
        urf_respawn_time_scaling_per_minute_seconds: 0.4,
        urf_respawn_time_scaling_cap_seconds: 20.0,
    }
}

fn test_urf() -> UrfBuffs {
    UrfBuffs {
        ability_haste: 0.0,
        item_haste: 0.0,
        health_cost_multiplier: 1.0,
        bonus_attack_speed_multiplier_melee: 1.0,
        bonus_attack_speed_multiplier_ranged: 1.0,
        allowed_item_keys: Default::default(),
    }
}

fn scheduled_enemy_attack_time(runner: &ControlledChampionCombatSimulation, idx: usize) -> f64 {
    runner
        .event_queue
        .iter()
        .find_map(|event| match event.kind {
            EventType::Attack(event_idx) if event_idx == idx => Some(event.time),
            _ => None,
        })
        .expect("enemy auto attack should be scheduled")
}

#[test]
fn controlled_champion_loadout_runtime_increases_spell_damage_when_selected() {
    let base = test_controlled_champion_base();
    let enemy = test_enemy("Target Dummy");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let bonus_stats = Stats {
        ability_power: 250.0,
        ..Stats::default()
    };
    let sim = test_simulation(4.0, true);
    let urf = test_urf();
    let outcome_without_runtime = simulate_controlled_champion_combat(
        &base,
        &[],
        &bonus_stats,
        None,
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    let arcane_comet_selection = LoadoutSelection {
        rune_names: vec!["Arcane Comet".to_string()],
        shard_stats: Vec::new(),
    };
    let outcome_with_runtime = simulate_controlled_champion_combat(
        &base,
        &[],
        &bonus_stats,
        Some(&arcane_comet_selection),
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    assert!(outcome_with_runtime.damage_dealt > outcome_without_runtime.damage_dealt);
}

#[test]
fn combat_seed_changes_enemy_attack_jitter_deterministically() {
    let controlled_champion = test_controlled_champion_base();
    let enemies = vec![(test_enemy("Sona"), Vec::new(), Stats::default())];
    let urf = test_urf();

    let mut sim_a = test_simulation(2.0, false);
    sim_a.combat_seed = Some(7);
    let runner_a = ControlledChampionCombatSimulation::new(
        controlled_champion.clone(),
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim_a.clone(),
        urf.clone(),
    );
    let attack_time_a = scheduled_enemy_attack_time(&runner_a, 0);

    let mut sim_b = test_simulation(2.0, false);
    sim_b.combat_seed = Some(11);
    let runner_b = ControlledChampionCombatSimulation::new(
        controlled_champion.clone(),
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim_b,
        urf.clone(),
    );
    let attack_time_b = scheduled_enemy_attack_time(&runner_b, 0);

    let runner_a_repeat = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim_a,
        urf,
    );
    let attack_time_a_repeat = scheduled_enemy_attack_time(&runner_a_repeat, 0);

    assert_ne!(attack_time_a, attack_time_b);
    assert!((attack_time_a - attack_time_a_repeat).abs() < 1e-12);
}

#[test]
fn controlled_champion_second_wind_runtime_adds_regeneration_ticks() {
    let base = test_controlled_champion_base();
    let enemy = test_enemy("Sona");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let sim = test_simulation(12.0, false);
    let urf = test_urf();
    let outcome_without_runtime = simulate_controlled_champion_combat(
        &base,
        &[],
        &Stats::default(),
        None,
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    let second_wind_selection = LoadoutSelection {
        rune_names: vec!["Second Wind".to_string()],
        shard_stats: Vec::new(),
    };
    let outcome_with_runtime = simulate_controlled_champion_combat(
        &base,
        &[],
        &Stats::default(),
        Some(&second_wind_selection),
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    assert!(outcome_with_runtime.healing_done > outcome_without_runtime.healing_done);
    assert!(outcome_with_runtime.time_alive_seconds >= outcome_without_runtime.time_alive_seconds);
}

#[test]
fn trace_emits_initial_state_snapshot_with_checkpoint_zero() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy("Sona");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(1.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();

    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[state_snapshot] checkpoint 0.0s"))
    );
}

#[test]
fn trace_emits_periodic_snapshots_every_five_seconds() {
    let controlled_champion = test_controlled_champion_base();
    let enemies: Vec<(EnemyConfig, Vec<Item>, Stats)> = Vec::new();
    let simulation = test_simulation(12.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    while runner.step(1) {}

    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[state_snapshot] checkpoint 5.0s"))
    );
    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[state_snapshot] checkpoint 10.0s"))
    );
}

#[test]
fn damage_trace_includes_source_champion_and_ability() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy("Sona");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(3.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    for _ in 0..120 {
        if !runner.step(1) {
            break;
        }
    }

    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[damage_in]") && entry.contains("Auto Attack ->")),
        "expected damage_in trace to include source ability context"
    );
}

#[test]
fn offensive_ultimate_is_prioritized_before_defensive_ability_two_when_both_ready() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Sona");
    enemy.spawn_position_xy = Some((200.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(1.0, true);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );

    let defensive_ability_two_id = runner.cast_profile.defensive_ability_two_id.clone();
    let offensive_ultimate_id = runner.cast_profile.offensive_ultimate_ability_id.clone();

    let _ = runner.step(1);

    assert!(
        runner.controlled_champion_ability_ready_at(&offensive_ultimate_id) > 0.0,
        "offensive ultimate should be cast when ready and target is in range"
    );
    assert_eq!(
        runner.controlled_champion_ability_ready_at(&defensive_ability_two_id),
        0.0,
        "defensive ability two should be delayed when offensive ultimate is ready this tick"
    );
}

#[test]
fn miss_trace_includes_reason_text() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy("Sona");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(1.0, true);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    runner.schedule_event(
        0.0,
        34,
        EventType::ControlledChampionOffensivePrimaryHit {
            idx: 0,
            source: Vec2 { x: 0.0, y: 0.0 },
            target_at_cast: Vec2 {
                x: 4000.0,
                y: 4000.0,
            },
            projectile_speed: 1200.0,
            effect_hitbox_radius: 0.0,
        },
        None,
    );
    let _ = runner.step(1);

    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[controlled_champion_primary_miss]")
                && entry.contains("target outside hitbox path")),
        "expected miss trace to include miss reason"
    );
}

#[test]
fn melee_attack_is_cancelled_when_attacker_is_stunned_during_windup() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy_with_role("Melee Tester", true);
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    runner.schedule_event(0.0, 30, EventType::Attack(0), None);
    let _ = runner.step(1);
    runner.enemy_state[0].stunned_until = runner.current_time() + 1.0;
    for _ in 0..30 {
        let _ = runner.step(1);
    }
    assert_eq!(runner.current_health(), runner.max_health);
    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("cancelled during windup"))
    );
}

#[test]
fn projectile_impact_on_stasis_is_nullified() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy("Sona");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(1.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    runner.schedule_event(0.0, 30, EventType::Attack(0), None);
    for _ in 0..8 {
        let _ = runner.step(1);
    }
    let health_before_stasis = runner.current_health();
    runner.stasis_until = runner.current_time() + 0.8;
    for _ in 0..16 {
        let _ = runner.step(1);
    }
    assert_eq!(runner.current_health(), health_before_stasis);
    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[impact_nullified]"))
    );
}

#[test]
fn enemy_cannot_auto_attack_while_invulnerable() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy("Sona");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(1.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    runner.enemy_state[0].invulnerable_until = runner.current_time() + 0.8;
    runner.schedule_event(0.0, 30, EventType::Attack(0), None);
    for _ in 0..24 {
        let _ = runner.step(1);
    }
    assert_eq!(runner.current_health(), runner.max_health);
    assert!(
        !runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("begins auto attack"))
    );
    assert!(
        !runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[attack_hit]"))
    );
}

#[test]
fn press_the_attack_runtime_increases_auto_attack_damage() {
    let base = test_controlled_champion_base();
    let mut enemy = test_enemy("Target Dummy");
    enemy.spawn_position_xy = Some((250.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let sim = test_simulation(8.0, false);
    let urf = test_urf();

    let baseline = simulate_controlled_champion_combat(
        &base,
        &[],
        &Stats::default(),
        None,
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    let press_the_attack = LoadoutSelection {
        rune_names: vec!["Press the Attack".to_string()],
        shard_stats: Vec::new(),
    };
    let with_press_the_attack = simulate_controlled_champion_combat(
        &base,
        &[],
        &Stats::default(),
        Some(&press_the_attack),
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    assert!(with_press_the_attack.damage_dealt > baseline.damage_dealt);
}

#[test]
fn fleet_footwork_runtime_adds_healing_in_combat() {
    let base = test_controlled_champion_base();
    let mut enemy = test_enemy("Target Dummy");
    enemy.spawn_position_xy = Some((250.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let sim = test_simulation(12.0, false);
    let urf = test_urf();

    let baseline = simulate_controlled_champion_combat(
        &base,
        &[],
        &Stats::default(),
        None,
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    let fleet_footwork = LoadoutSelection {
        rune_names: vec!["Fleet Footwork".to_string()],
        shard_stats: Vec::new(),
    };
    let with_fleet = simulate_controlled_champion_combat(
        &base,
        &[],
        &Stats::default(),
        Some(&fleet_footwork),
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    assert!(with_fleet.healing_done > baseline.healing_done);
}

#[test]
fn conqueror_runtime_increases_damage_and_healing_over_extended_fight() {
    let base = test_controlled_champion_base();
    let mut enemy = test_enemy("Target Dummy");
    enemy.spawn_position_xy = Some((250.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let sim = test_simulation(12.0, true);
    let urf = test_urf();
    let bonus_stats = Stats {
        ability_power: 280.0,
        ..Stats::default()
    };

    let baseline = simulate_controlled_champion_combat(
        &base,
        &[],
        &bonus_stats,
        None,
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    let conqueror = LoadoutSelection {
        rune_names: vec!["Conqueror".to_string()],
        shard_stats: Vec::new(),
    };
    let with_conqueror = simulate_controlled_champion_combat(
        &base,
        &[],
        &bonus_stats,
        Some(&conqueror),
        None,
        None,
        &enemies,
        &sim,
        &urf,
    );
    assert!(with_conqueror.damage_dealt > baseline.damage_dealt);
    assert!(with_conqueror.healing_done >= baseline.healing_done);
}

#[test]
fn pool_ticks_hit_all_enemies_in_range_with_expected_total_damage() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemies = Vec::new();
    for idx in 0..5 {
        let mut enemy = test_enemy(&format!("Target {}", idx + 1));
        enemy.spawn_position_xy = Some((120.0 + idx as f64 * 20.0, 0.0));
        enemy.movement_mode = OpponentMovementMode::HoldPosition;
        enemies.push((enemy, Vec::new(), Stats::default()));
    }
    let simulation = test_simulation(3.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    runner.controlled_champion_defensive_ability_two_damage_per_tick = 100.0;
    runner.controlled_champion_defensive_ability_two_heal_ratio_of_damage = 0.0;
    runner.pool_effect_range = 500.0;
    runner.pool_damage_tick_interval_seconds = 0.5;
    runner.pool_damage_until = 1.5;
    runner.pool_next_damage_tick_at = 0.5;
    runner.apply_hot_effects(1.6);

    let tick_hit_count = runner
        .trace_events()
        .iter()
        .filter(|entry| entry.contains("[controlled_champion_pool_tick]"))
        .filter(|entry| entry.contains("to 5 enemies in range"))
        .count();
    assert_eq!(tick_hit_count, 3);

    let enemy_magic_multiplier = runner.enemy_state[0].magic_multiplier;
    let expected_damage = 3.0 * 5.0 * 100.0 * enemy_magic_multiplier;
    assert!((runner.damage_dealt_total - expected_damage).abs() < 1e-6);

    let expected_enemy_health =
        runner.enemy_state[0].max_health - (3.0 * 100.0 * enemy_magic_multiplier);
    for enemy in &runner.enemy_state {
        assert!((enemy.health - expected_enemy_health).abs() < 1e-6);
    }
}

#[test]
fn pool_tick_hits_enemy_exactly_on_range_boundary() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Boundary Target");
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    runner.controlled_champion_defensive_ability_two_damage_per_tick = 75.0;
    runner.controlled_champion_defensive_ability_two_heal_ratio_of_damage = 0.0;
    runner.pool_effect_range = 200.0;
    runner.pool_damage_tick_interval_seconds = 0.5;
    runner.pool_damage_until = 0.5;
    runner.pool_next_damage_tick_at = 0.5;
    let boundary_distance = runner.pool_effect_range
        + runner.controlled_champion_hitbox_radius
        + runner.enemy_state[0].hitbox_radius;
    runner.target_position = Vec2 { x: 0.0, y: 0.0 };
    runner.enemy_state[0].position = Vec2 {
        x: boundary_distance,
        y: 0.0,
    };

    runner.apply_hot_effects(0.6);

    assert!(runner.damage_dealt_total > 0.0);
    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[controlled_champion_pool_tick]")
                && entry.contains("to 1 enemies in range"))
    );
}

#[test]
fn pool_tick_misses_enemy_just_outside_range_boundary() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Boundary Miss");
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );
    runner.enable_trace();
    runner.controlled_champion_defensive_ability_two_damage_per_tick = 75.0;
    runner.controlled_champion_defensive_ability_two_heal_ratio_of_damage = 0.0;
    runner.pool_effect_range = 200.0;
    runner.pool_damage_tick_interval_seconds = 0.5;
    runner.pool_damage_until = 0.5;
    runner.pool_next_damage_tick_at = 0.5;
    let boundary_distance = runner.pool_effect_range
        + runner.controlled_champion_hitbox_radius
        + runner.enemy_state[0].hitbox_radius;
    runner.target_position = Vec2 { x: 0.0, y: 0.0 };
    runner.enemy_state[0].position = Vec2 {
        x: boundary_distance + 1e-3,
        y: 0.0,
    };

    runner.apply_hot_effects(0.6);

    assert_eq!(runner.damage_dealt_total, 0.0);
    assert!(
        runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[controlled_champion_pool_tick]")
                && entry.contains("to 0 enemies in range"))
    );
}

#[test]
fn aftershock_runtime_triggers_from_enemy_immobilize_script() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Sona");
    enemy.spawn_position_xy = Some((200.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let baseline_enemies = vec![(enemy.clone(), Vec::new(), Stats::default())];
    enemy.loadout_rune_names = vec!["Aftershock".to_string()];
    let aftershock_enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(3.0, false);
    let urf = test_urf();

    let mut baseline_runner = ControlledChampionCombatSimulation::new(
        controlled_champion.clone(),
        &[],
        &Stats::default(),
        None,
        None,
        &baseline_enemies,
        simulation.clone(),
        urf.clone(),
    );
    let baseline_epoch = baseline_runner.enemy_state[0].script_epoch;
    baseline_runner.schedule_event(
        0.0,
        12,
        EventType::ChampionScript(0, ChampionScriptEvent::SonaCrescendo, baseline_epoch),
        None,
    );
    while baseline_runner.step(1) {}
    let baseline_health = baseline_runner.current_health();

    let mut aftershock_runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &aftershock_enemies,
        simulation,
        urf,
    );
    aftershock_runner.enable_trace();
    let aftershock_epoch = aftershock_runner.enemy_state[0].script_epoch;
    aftershock_runner.schedule_event(
        0.0,
        12,
        EventType::ChampionScript(0, ChampionScriptEvent::SonaCrescendo, aftershock_epoch),
        None,
    );
    while aftershock_runner.step(1) {}
    let aftershock_health = aftershock_runner.current_health();

    assert!(aftershock_health < baseline_health);
    assert!(
        aftershock_runner
            .trace_events()
            .iter()
            .any(|entry| entry.contains("[aftershock_hit]")),
        "expected aftershock trace event when immobilize lands"
    );
}

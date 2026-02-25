use super::*;

#[derive(Debug)]
struct OneShotMoveControllerPolicy {
    emitted: bool,
}

impl crate::champion_control_harness::ChampionActionDecisionPolicy for OneShotMoveControllerPolicy {
    fn choose_action(
        &mut self,
        _view: &crate::champion_control_harness::ChampionPerspectiveView,
    ) -> Option<crate::champion_control_harness::ChampionActionRequest> {
        if self.emitted {
            return None;
        }
        self.emitted = true;
        Some(
            crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
                target_position: crate::world::WorldActorPosition { x: 1500.0, y: 0.0 },
            },
        )
    }
}

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
fn out_of_bounds_enemy_queries_return_safe_defaults() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy("Fallback Target");
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(1.0, false);
    let urf = test_urf();

    let runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );

    let invalid_index = 999;
    assert!(
        runner.distance_to_target(invalid_index).is_infinite(),
        "invalid enemy index should not panic and should resolve to +infinity distance"
    );
    assert!(
        !runner.enemy_in_attack_range(invalid_index),
        "invalid enemy index should be treated as not in attack range"
    );
}

#[test]
fn enemy_spawn_positions_are_clamped_to_world_bounds() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Out Of Bounds Target");
    enemy.spawn_position_xy = Some((12_000.0, -12_000.0));
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let simulation = test_simulation(1.0, false);
    let urf = test_urf();

    let runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        simulation,
        urf,
    );

    let Some(enemy_position) = runner.enemy_position(0) else {
        panic!("enemy should be present");
    };
    let map_bounds = crate::world::default_urf_world_map_state().bounds;
    assert!(
        map_bounds.contains(enemy_position.x, enemy_position.y),
        "enemy spawn position should be clamped into world bounds"
    );
}

#[test]
fn world_lifecycle_step_spawns_minion_wave_actors_during_runtime() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(70.0, false);
    let urf = test_urf();

    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );
    while runner.step(1) {}

    assert!(
        runner
            .world_actor_position("minion:blue:wave_1:unit_1")
            .is_some(),
        "first blue minion wave actor should be present after wave start timer"
    );
    assert!(
        runner
            .world_actor_position("minion:red:wave_1:unit_1")
            .is_some(),
        "first red minion wave actor should be present after wave start timer"
    );
    assert_eq!(
        runner.world_actor_count_by_class_and_allegiance(
            crate::world::WorldActorClass::Structure,
            crate::world::WorldActorAllegiance::ControlledChampionTeam
        ),
        1
    );
}

#[test]
fn controller_move_command_steps_controlled_champion_position() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );

    let action_status = runner.queue_controlled_champion_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
            target_position: crate::world::WorldActorPosition { x: 1200.0, y: 0.0 },
        },
    );
    assert!(matches!(
        action_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));

    for _ in 0..8 {
        runner.step(1);
    }

    let controlled_position = runner
        .world_actor_position("controlled_champion")
        .expect("controlled champion world actor should exist");
    assert!(
        controlled_position.x > 0.0,
        "move command should advance controlled champion along x-axis"
    );
}

#[test]
fn controller_request_queue_preserves_sequence_order_per_tick() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );

    let controller_identity = crate::champion_control_harness::ChampionControllerIdentity {
        controller_id: "human_player_test".to_string(),
        controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
    };
    let first = runner.queue_controlled_champion_action_request(
        controller_identity.clone(),
        crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
            target_position: crate::world::WorldActorPosition { x: 1500.0, y: 0.0 },
        },
    );
    let second = runner.queue_controlled_champion_action_request(
        controller_identity,
        crate::champion_control_harness::ChampionActionRequest::StopCurrentAction,
    );
    assert!(matches!(
        first.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    assert!(matches!(
        second.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));

    runner.step(1);

    assert!(
        runner
            .controlled_champion_pending_move_target_position()
            .is_none(),
        "stop request should clear move command when applied after move in same tick"
    );
}

#[test]
fn unsupported_controller_cast_action_returns_explicit_rejection_status() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );

    let action_status = runner.queue_controlled_champion_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        crate::champion_control_harness::ChampionActionRequest::CastAbilityBySlot {
            ability_slot: crate::scripts::runtime::ability_slots::AbilitySlotKey::Q,
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(
        matches!(
            action_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedUnsupportedAction { .. }
        ),
        "unsupported cast channel should reject with explicit status"
    );
}

#[test]
fn controller_policy_requests_respect_fixed_tick_delay_before_execution() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );

    runner.set_controlled_champion_controller_policy(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "ai_controller_test".to_string(),
            controller_kind:
                crate::champion_control_harness::ChampionControllerKind::ArtificialIntelligence,
        },
        Box::new(OneShotMoveControllerPolicy { emitted: false }),
    );

    runner.step(1);
    assert!(
        runner
            .controlled_champion_pending_move_target_position()
            .is_none(),
        "policy request should not execute in the same tick it is sampled"
    );

    runner.step(1);
    assert!(
        runner
            .controlled_champion_pending_move_target_position()
            .is_some(),
        "policy request should execute after configured fixed tick delay"
    );
}

#[test]
fn controller_perspective_uses_data_owned_vision_radius_default() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );
    assert_eq!(
        runner.controlled_champion_controller_vision_radius,
        crate::defaults::controlled_champion_controller_vision_radius_default(),
    );
}

#[test]
fn clearing_controller_policy_keeps_manual_control_mode_enabled() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );

    runner.set_controlled_champion_controller_policy(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "policy_toggle_test".to_string(),
            controller_kind:
                crate::champion_control_harness::ChampionControllerKind::ArtificialIntelligence,
        },
        Box::new(OneShotMoveControllerPolicy { emitted: true }),
    );
    runner.clear_controlled_champion_controller_policy();

    assert!(
        runner.controlled_champion_manual_control_mode_enabled(),
        "manual control mode should remain enabled once harness control has been activated"
    );
}

#[test]
fn enemy_actor_move_command_steps_position_under_manual_control() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Enemy Manual Move");
    enemy.id = "enemy_manual_move_actor".to_string();
    enemy.spawn_position_xy = Some((900.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_move_actor",
        crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
            target_position: crate::world::WorldActorPosition { x: -600.0, y: 0.0 },
        },
    );
    assert!(matches!(
        action_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));

    let initial_position = runner
        .world_actor_position("enemy_manual_move_actor")
        .expect("enemy actor should be present");
    runner.step(1);
    let position_after_first_tick = runner
        .world_actor_position("enemy_manual_move_actor")
        .expect("enemy actor should still be present");
    assert!(
        position_after_first_tick.x < initial_position.x,
        "enemy actor should move toward queued command target on first eligible execution tick"
    );

    runner.step(1);
    let position_after_second_tick = runner
        .world_actor_position("enemy_manual_move_actor")
        .expect("enemy actor should still be present");
    assert!(
        position_after_second_tick.x < position_after_first_tick.x,
        "enemy actor should continue stepping toward queued command target under manual control"
    );
}

#[test]
fn enemy_actor_cast_request_returns_explicit_unsupported_status() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Vladimir");
    enemy.id = "enemy_unsupported_cast_actor".to_string();
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_unsupported_cast_actor",
        crate::champion_control_harness::ChampionActionRequest::CastAbilityBySlot {
            ability_slot: crate::scripts::runtime::ability_slots::AbilitySlotKey::Q,
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(
        matches!(
            action_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedUnsupportedAction { .. }
        ),
        "enemy cast action should return explicit unsupported status when no mapped script-cast channel exists"
    );
}

#[test]
fn enemy_manual_control_disables_autonomous_script_casts_without_manual_cast_command() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_manual_script_disabled_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_script_disabled_actor",
        crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
            target_position: crate::world::WorldActorPosition { x: 300.0, y: 0.0 },
        },
    );
    assert!(matches!(
        status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));

    let health_before = runner.health;
    for _ in 0..180 {
        runner.step(1);
    }
    assert!(
        (runner.health - health_before).abs() < 1e-9,
        "manual-control enemies should not execute autonomous script cadence without explicit cast commands"
    );
}

#[test]
fn enemy_actor_script_cast_request_accepts_supported_slot_and_applies_damage() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_manual_script_cast_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_script_cast_actor",
        crate::champion_control_harness::ChampionActionRequest::CastAbilityBySlot {
            ability_slot: crate::scripts::runtime::ability_slots::AbilitySlotKey::Q,
            target_actor_id: Some("controlled_champion".to_string()),
            target_position: None,
        },
    );
    assert!(matches!(
        action_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));

    let health_before = runner.health;
    for _ in 0..120 {
        runner.step(1);
    }
    assert!(
        runner.health < health_before,
        "manual script-cast command should execute a mapped enemy script event and deal damage"
    );
}

#[test]
fn enemy_actor_script_cast_request_reports_cooldown_after_successful_cast() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_manual_script_cooldown_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let first_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_script_cooldown_actor",
        crate::champion_control_harness::ChampionActionRequest::CastAbilityBySlot {
            ability_slot: crate::scripts::runtime::ability_slots::AbilitySlotKey::Q,
            target_actor_id: Some("controlled_champion".to_string()),
            target_position: None,
        },
    );
    assert!(matches!(
        first_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    for _ in 0..60 {
        runner.step(1);
    }

    let second_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_script_cooldown_actor",
        crate::champion_control_harness::ChampionActionRequest::CastAbilityBySlot {
            ability_slot: crate::scripts::runtime::ability_slots::AbilitySlotKey::Q,
            target_actor_id: Some("controlled_champion".to_string()),
            target_position: None,
        },
    );
    assert!(
        matches!(
            second_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedAbilityOnCooldown { .. }
        ),
        "second command should report explicit cooldown rejection while the mapped script cast is cooling down"
    );
}

#[test]
fn enemy_actor_item_active_stasis_request_rejects_when_enemy_has_no_stasis_item() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_no_stasis_item_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_no_stasis_item_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "stasis_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(
        matches!(
            action_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedUnknownItemActive { .. }
        ),
        "enemy item-active command should reject when the active is not available in runtime readiness"
    );
}

#[test]
fn enemy_actor_item_active_stasis_request_accepts_and_blocks_pool_tick_damage() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_stasis_item_actor".to_string();
    enemy.spawn_position_xy = Some((120.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemy_build = vec![test_item("Zhonya's Hourglass")];
    let enemies = vec![(enemy, enemy_build, Stats::default())];
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

    let first_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_stasis_item_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "stasis_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(matches!(
        first_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    runner.step(2);
    assert!(
        runner.enemy_state[0].stasis_until > runner.time,
        "enemy stasis item command should set active stasis window"
    );

    let health_before = runner.enemy_state[0].health;
    runner.controlled_champion_defensive_ability_two_damage_per_tick = 100.0;
    runner.controlled_champion_defensive_ability_two_heal_ratio_of_damage = 0.0;
    runner.pool_effect_range = 500.0;
    runner.pool_damage_tick_interval_seconds = 0.5;
    runner.pool_damage_until = runner.time + 0.5;
    runner.pool_next_damage_tick_at = runner.time + 0.5;
    runner.apply_hot_effects(0.6);
    assert!(
        (runner.enemy_state[0].health - health_before).abs() < 1e-9,
        "enemy stasis should nullify incoming controlled-champion pool tick damage during the stasis window"
    );
}

#[test]
fn enemy_actor_item_active_stasis_request_reports_cooldown_after_activation() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_stasis_cooldown_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemy_build = vec![test_item("Zhonya's Hourglass")];
    let enemies = vec![(enemy, enemy_build, Stats::default())];
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

    let first_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_stasis_cooldown_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "stasis_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(matches!(
        first_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    runner.step(2);
    for _ in 0..120 {
        runner.step(1);
    }

    let second_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_stasis_cooldown_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "stasis_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(
        matches!(
            second_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedItemActiveOnCooldown { .. }
        ),
        "second enemy stasis-item command should report explicit cooldown rejection"
    );
}

#[test]
fn enemy_actor_item_active_emergency_shield_request_rejects_when_enemy_has_no_item() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_no_emergency_shield_item_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_no_emergency_shield_item_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "emergency_shield_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(
        matches!(
            action_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedUnknownItemActive { .. }
        ),
        "enemy emergency-shield command should reject when the active is not available in runtime readiness"
    );
}

#[test]
fn enemy_actor_item_active_emergency_shield_request_accepts_and_absorbs_damage() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_emergency_shield_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemy_build = vec![test_item("Protoplasm Harness")];
    let enemies = vec![(enemy, enemy_build, Stats::default())];
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
    runner.sim.protoplasm_bonus_health = 300.0;
    runner.sim.protoplasm_heal_total = 0.0;
    runner.sim.protoplasm_duration_seconds = 0.0;

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_emergency_shield_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "emergency_shield_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(matches!(
        action_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    runner.step(2);

    let health_before = runner.enemy_state[0].health;
    let dealt = runner.apply_incoming_magic_damage_to_enemy(0, 100.0);
    assert!(
        dealt <= 1e-9,
        "enemy emergency shield should absorb incoming damage while shield amount remains"
    );
    assert!(
        (runner.enemy_state[0].health - health_before).abs() < 1e-9,
        "enemy health should remain unchanged when incoming damage is fully absorbed by emergency shield"
    );
}

#[test]
fn enemy_actor_item_active_emergency_shield_request_reports_cooldown_after_activation() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_emergency_shield_cooldown_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemy_build = vec![test_item("Protoplasm Harness")];
    let enemies = vec![(enemy, enemy_build, Stats::default())];
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

    let first_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_emergency_shield_cooldown_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "emergency_shield_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(matches!(
        first_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    runner.step(2);

    let second_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_emergency_shield_cooldown_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "emergency_shield_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(
        matches!(
            second_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedItemActiveOnCooldown { .. }
        ),
        "second enemy emergency-shield command should report explicit cooldown rejection"
    );
}

#[test]
fn enemy_actor_emergency_shield_item_applies_heal_over_time() {
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_emergency_shield_heal_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(
        enemy.clone(),
        vec![test_item("Protoplasm Harness")],
        Stats::default(),
    )];

    let mut baseline_runner = ControlledChampionCombatSimulation::new(
        test_controlled_champion_base(),
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        test_simulation(8.0, false),
        test_urf(),
    );
    baseline_runner.sim.protoplasm_bonus_health = 0.0;
    baseline_runner.sim.protoplasm_heal_total = 300.0;
    baseline_runner.sim.protoplasm_duration_seconds = 3.0;
    baseline_runner.enemy_state[0].health =
        (baseline_runner.enemy_state[0].max_health - 500.0).max(1.0);
    for _ in 0..30 {
        baseline_runner.step(1);
    }
    let baseline_health_after = baseline_runner.enemy_state[0].health;

    let mut activated_runner = ControlledChampionCombatSimulation::new(
        test_controlled_champion_base(),
        &[],
        &Stats::default(),
        None,
        None,
        &[(
            enemy,
            vec![test_item("Protoplasm Harness")],
            Stats::default(),
        )],
        test_simulation(8.0, false),
        test_urf(),
    );
    activated_runner.sim.protoplasm_bonus_health = 0.0;
    activated_runner.sim.protoplasm_heal_total = 300.0;
    activated_runner.sim.protoplasm_duration_seconds = 3.0;
    activated_runner.enemy_state[0].health =
        (activated_runner.enemy_state[0].max_health - 500.0).max(1.0);

    let status = activated_runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_emergency_shield_heal_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "emergency_shield_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(matches!(
        status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    for _ in 0..30 {
        activated_runner.step(1);
    }
    assert!(
        activated_runner.enemy_state[0].health > baseline_health_after,
        "enemy emergency shield item should increase heal-over-time beyond baseline regen-only healing"
    );
}

#[test]
fn enemy_manual_move_command_does_not_step_position_during_stasis_window() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Morgana");
    enemy.id = "enemy_stasis_move_lock_actor".to_string();
    enemy.spawn_position_xy = Some((900.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemy_build = vec![test_item("Zhonya's Hourglass")];
    let enemies = vec![(enemy, enemy_build, Stats::default())];
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

    let stasis_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_stasis_move_lock_actor",
        crate::champion_control_harness::ChampionActionRequest::UseItemActive {
            item_active_id: "stasis_item".to_string(),
            target_actor_id: None,
            target_position: None,
        },
    );
    assert!(matches!(
        stasis_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));
    runner.step(2);
    let position_before = runner
        .world_actor_position("enemy_stasis_move_lock_actor")
        .expect("enemy actor should be present");

    let move_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_stasis_move_lock_actor",
        crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
            target_position: crate::world::WorldActorPosition { x: -700.0, y: 0.0 },
        },
    );
    assert!(
        matches!(
            move_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedMovementLocked { .. }
        ),
        "enemy movement commands should return explicit movement-lock rejection while stasis is active"
    );
    runner.step(2);

    let position_after = runner
        .world_actor_position("enemy_stasis_move_lock_actor")
        .expect("enemy actor should be present");
    assert!(
        (position_after.x - position_before.x).abs() < 1e-9,
        "enemy movement should remain locked while stasis is active"
    );
}

#[test]
fn enemy_manual_control_without_attack_target_prevents_auto_attack_hits() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Enemy Manual No Attack");
    enemy.id = "enemy_manual_no_attack_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_no_attack_actor",
        crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
            target_position: crate::world::WorldActorPosition { x: 300.0, y: 0.0 },
        },
    );
    assert!(matches!(
        action_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));

    let health_before = runner.health;
    for _ in 0..120 {
        runner.step(1);
    }
    assert!(
        (runner.health - health_before).abs() < 1e-9,
        "manually controlled enemy should not auto-attack without an explicit StartBasicAttack target command"
    );
}

#[test]
fn enemy_actor_start_basic_attack_request_accepts_controlled_target_and_deals_damage() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(6.0, false);
    let urf = test_urf();
    let mut enemy = test_enemy("Enemy Manual Attack");
    enemy.id = "enemy_manual_attack_actor".to_string();
    enemy.spawn_position_xy = Some((300.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_attack_actor",
        crate::champion_control_harness::ChampionActionRequest::StartBasicAttack {
            target_actor_id: "controlled_champion".to_string(),
        },
    );
    assert!(matches!(
        action_status.status,
        crate::champion_control_harness::ChampionActionStatus::AcceptedQueued
    ));

    let health_before = runner.health;
    for _ in 0..120 {
        runner.step(1);
    }
    assert!(
        runner.health < health_before,
        "enemy start basic attack command should enable manual attack execution against controlled champion"
    );
}

#[test]
fn enemy_actor_start_basic_attack_rejects_non_controlled_target() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut enemy_a = test_enemy("Enemy Manual Attack A");
    enemy_a.id = "enemy_manual_attack_actor_a".to_string();
    enemy_a.spawn_position_xy = Some((300.0, 0.0));
    let mut enemy_b = test_enemy("Enemy Manual Attack B");
    enemy_b.id = "enemy_manual_attack_actor_b".to_string();
    enemy_b.spawn_position_xy = Some((350.0, 0.0));
    let enemies = vec![
        (enemy_a, Vec::new(), Stats::default()),
        (enemy_b, Vec::new(), Stats::default()),
    ];
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

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "enemy_manual_attack_actor_a",
        crate::champion_control_harness::ChampionActionRequest::StartBasicAttack {
            target_actor_id: "enemy_manual_attack_actor_b".to_string(),
        },
    );
    assert!(
        matches!(
            action_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedTargetInvalidForAction { .. }
        ),
        "enemy basic attack requests should reject non-controlled-champion targets"
    );
}

#[test]
fn unknown_controlled_actor_request_returns_explicit_not_found_status() {
    let controlled_champion = test_controlled_champion_base();
    let simulation = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &[],
        simulation,
        urf,
    );

    let action_status = runner.queue_actor_action_request(
        crate::champion_control_harness::ChampionControllerIdentity {
            controller_id: "human_player_test".to_string(),
            controller_kind: crate::champion_control_harness::ChampionControllerKind::HumanPlayer,
        },
        "unknown_actor_for_control",
        crate::champion_control_harness::ChampionActionRequest::MoveToPosition {
            target_position: crate::world::WorldActorPosition { x: 100.0, y: 100.0 },
        },
    );
    assert!(
        matches!(
            action_status.status,
            crate::champion_control_harness::ChampionActionStatus::RejectedControlledActorNotFound { .. }
        ),
        "unknown controlled actor requests should be rejected explicitly"
    );
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

fn test_item(name: &str) -> Item {
    Item {
        name: name.to_string(),
        stats: Stats::default(),
        rank: Vec::new(),
        shop_purchasable: true,
        total_cost: 0.0,
        passive_effects_text: Vec::new(),
        has_active_effect: false,
        structured_effect_count: 0,
    }
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
        collect_rune_proc_telemetry: true,
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
        .enemy_next_attack_ready_at(idx)
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
fn rune_proc_telemetry_can_be_disabled_for_search_time_simulations() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Telemetry Target");
    enemy.spawn_position_xy = Some((250.0, 0.0));
    enemy.movement_mode = OpponentMovementMode::HoldPosition;
    let enemies = vec![(enemy, Vec::new(), Stats::default())];
    let bonus_stats = Stats {
        ability_power: 300.0,
        ..Stats::default()
    };
    let loadout_selection = LoadoutSelection {
        rune_names: vec!["Summon Aery".to_string()],
        shard_stats: Vec::new(),
    };
    let urf = test_urf();

    let mut sim_with_telemetry = test_simulation(4.0, true);
    sim_with_telemetry.collect_rune_proc_telemetry = true;
    let mut runner_with_telemetry =
        ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
            controlled_champion.clone(),
            &[],
            &bonus_stats,
            Some(&loadout_selection),
            None,
            None,
            &enemies,
            sim_with_telemetry,
            urf.clone(),
        );
    while runner_with_telemetry.step(1) {}
    let with_telemetry = runner_with_telemetry.controlled_champion_rune_proc_telemetry();
    assert!(
        with_telemetry
            .iter()
            .any(|entry| entry.rune_name == "Summon Aery"),
        "expected summon aery telemetry when collection is enabled"
    );

    let mut sim_without_telemetry = test_simulation(4.0, true);
    sim_without_telemetry.collect_rune_proc_telemetry = false;
    let mut runner_without_telemetry =
        ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
            controlled_champion,
            &[],
            &bonus_stats,
            Some(&loadout_selection),
            None,
            None,
            &enemies,
            sim_without_telemetry,
            urf,
        );
    while runner_without_telemetry.step(1) {}
    assert!(
        runner_without_telemetry
            .controlled_champion_rune_proc_telemetry()
            .is_empty(),
        "search-time runs should skip full rune telemetry collection"
    );
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
fn enemy_attack_sequence_owner_methods_advance_and_invalidate_old_tokens() {
    let controlled_champion = test_controlled_champion_base();
    let enemies = vec![(test_enemy("Sona"), Vec::new(), Stats::default())];
    let sim = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim,
        urf,
    );

    let first_token = runner
        .begin_enemy_attack_sequence(0)
        .expect("enemy index should be valid");
    assert!(runner.enemy_attack_sequence_matches(0, first_token));

    let second_token = runner
        .begin_enemy_attack_sequence(0)
        .expect("enemy index should be valid");
    assert_ne!(first_token, second_token);
    assert!(!runner.enemy_attack_sequence_matches(0, first_token));
    assert!(runner.enemy_attack_sequence_matches(0, second_token));
}

#[test]
fn enemy_attack_bonus_physical_is_consumed_once_and_resets_after_hit() {
    let controlled_champion = test_controlled_champion_base();
    let enemies = vec![(test_enemy("Sona"), Vec::new(), Stats::default())];
    let sim = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim,
        urf,
    );

    let bonus = 42.0;
    runner
        .apply_enemy_next_attack_bonus_physical(0, bonus)
        .expect("enemy index should be valid");

    let first = runner
        .consume_enemy_attack_damage_with_on_hit(0, 1000.0, 1000.0)
        .expect("enemy index should be valid");
    let second = runner
        .consume_enemy_attack_damage_with_on_hit(0, 1000.0, 1000.0)
        .expect("enemy index should be valid");

    assert!((first.0 - second.0 - bonus).abs() < 1e-9);
}

#[test]
fn enemy_script_epoch_and_ready_queries_read_owner_state() {
    let controlled_champion = test_controlled_champion_base();
    let enemies = vec![(test_enemy("Sona"), Vec::new(), Stats::default())];
    let sim = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim,
        urf,
    );

    assert!(runner.enemy_script_epoch_matches(0, 0));
    assert_eq!(
        runner.enemy_script_event_ready_at_or_zero(0, ChampionScriptEvent::SonaCrescendo),
        0.0
    );

    runner.set_enemy_script_event_ready_at(0, ChampionScriptEvent::SonaCrescendo, 3.25);
    assert_eq!(
        runner.enemy_script_event_ready_at_or_zero(0, ChampionScriptEvent::SonaCrescendo),
        3.25
    );
}

#[test]
fn enemy_script_execution_owner_method_generates_actions_for_in_range_event() {
    let controlled_champion = test_controlled_champion_base();
    let enemies = vec![(test_enemy("Sona"), Vec::new(), Stats::default())];
    let sim = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim,
        urf,
    );

    let actions = runner.execute_enemy_script_event_actions(
        0,
        ChampionScriptEvent::SonaCrescendo,
        100.0,
        1200.0,
        2000.0,
        0.0,
    );
    assert!(!actions.is_empty());
}

#[test]
fn enemy_aftershock_owner_method_is_zero_without_aftershock_rune() {
    let controlled_champion = test_controlled_champion_base();
    let enemies = vec![(test_enemy("Sona"), Vec::new(), Stats::default())];
    let sim = test_simulation(2.0, false);
    let urf = test_urf();
    let mut runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim,
        urf,
    );

    let aftershock_damage = runner.enemy_aftershock_magic_damage_on_immobilize(0);
    assert_eq!(aftershock_damage, 0.0);
}

#[test]
fn enemy_read_projection_owner_queries_return_expected_shapes() {
    let controlled_champion = test_controlled_champion_base();
    let enemies = vec![(test_enemy("Sona"), Vec::new(), Stats::default())];
    let sim = test_simulation(2.0, false);
    let urf = test_urf();
    let runner = ControlledChampionCombatSimulation::new(
        controlled_champion,
        &[],
        &Stats::default(),
        None,
        None,
        &enemies,
        sim,
        urf,
    );

    assert_eq!(runner.enemy_name(0).as_deref(), Some("Sona"));
    assert!(runner.enemy_position(0).is_some());
    assert!(
        runner
            .enemy_attack_interval_seconds(0, runner.tick_seconds(), 0.1)
            .expect("enemy index should be valid")
            > 0.0
    );
    assert!(
        runner
            .enemy_hitbox_radius(0)
            .expect("enemy index should be valid")
            > 0.0
    );
    assert!(
        runner
            .enemy_attack_range(0)
            .expect("enemy index should be valid")
            > 0.0
    );
    assert!(
        runner
            .enemy_attack_windup_seconds(0)
            .expect("enemy index should be valid")
            >= 0.0
    );
    assert!(
        runner
            .enemy_attack_projectile_speed(0)
            .expect("enemy index should be valid")
            >= 0.0
    );
    assert!(
        runner
            .enemy_attack_effect_hitbox_radius(0)
            .expect("enemy index should be valid")
            >= 0.0
    );
    let snapshot = runner
        .enemy_trace_snapshot_at(0, 0.0)
        .expect("enemy index should be valid");
    assert_eq!(snapshot.name, "Sona");
    assert!(snapshot.attack_speed > 0.0);
    assert!(snapshot.attack_interval_seconds > 0.0);
    assert!(!snapshot.scripted_ability_cooldowns.is_empty());
    assert!(runner.enemy_trace_snapshot_at(99, 0.0).is_none());
    assert_eq!(
        runner.enemy_target_health_snapshot_or_defaults(99),
        (0.0, 1.0)
    );
    assert_eq!(
        runner.enemy_status_lines_at(99, 0.0),
        vec!["none".to_string()]
    );
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
fn pool_tick_moving_ranged_target_can_exit_range_before_tick() {
    let controlled_champion = test_controlled_champion_base();
    let enemy = test_enemy("Moving Boundary");
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
    runner.controlled_champion_defensive_ability_two_damage_per_tick = 90.0;
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
        x: boundary_distance - 1.0,
        y: 0.0,
    };

    runner.apply_enemy_movement_step(0.5);
    assert!(
        runner.distance_to_target(0) > boundary_distance,
        "target should have moved outside pool range before tick"
    );
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
fn pool_tick_hits_enemy_on_diagonal_range_boundary() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Diagonal Boundary");
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
    let diagonal_component = boundary_distance / 2.0_f64.sqrt();
    runner.target_position = Vec2 { x: 0.0, y: 0.0 };
    runner.enemy_state[0].position = Vec2 {
        x: diagonal_component,
        y: diagonal_component,
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
fn controlled_champion_range_checks_respect_effect_hitbox_radius() {
    let controlled_champion = test_controlled_champion_base();
    let mut enemy = test_enemy("Hitbox Radius");
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
    runner.pool_effect_range = 200.0;
    let boundary_distance = runner.pool_effect_range
        + runner.controlled_champion_hitbox_radius
        + runner.enemy_state[0].hitbox_radius;
    runner.target_position = Vec2 { x: 0.0, y: 0.0 };
    runner.enemy_state[0].position = Vec2 {
        x: boundary_distance + 8.0,
        y: 0.0,
    };

    let (_, without_effect_hitbox_hits) = runner
        .apply_incoming_magic_damage_to_enemies_in_controlled_champion_range(40.0, 200.0, 0.0);
    let (_, with_effect_hitbox_hits) = runner
        .apply_incoming_magic_damage_to_enemies_in_controlled_champion_range(40.0, 200.0, 10.0);

    assert_eq!(without_effect_hitbox_hits, 0);
    assert_eq!(with_effect_hitbox_hits, 1);
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

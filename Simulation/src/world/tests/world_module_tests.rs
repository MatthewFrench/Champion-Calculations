use super::world_actor_lifecycle_channels::{NeutralObjective, WorldLifecycleState};
use super::world_actor_position_channels::{
    WorldActorAllegiance, WorldActorClass, WorldActorPosition, WorldState,
};
use super::world_encounter_state_builder::build_world_state_for_controlled_champion_encounter;
use super::world_map_model::default_urf_world_map_state;
use crate::defaults::WorldLifecycleDefaults;

#[test]
fn world_state_registers_actor_positions_and_computes_distances() {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    world_state
        .register_actor_position_with_allegiance(
            "controlled_champion",
            WorldActorClass::Champion,
            WorldActorAllegiance::ControlledChampionTeam,
            WorldActorPosition { x: 0.0, y: 0.0 },
        )
        .expect("controlled champion registration should succeed");
    world_state
        .register_actor_position_with_allegiance(
            "enemy_frontline",
            WorldActorClass::Champion,
            WorldActorAllegiance::OpponentTeam,
            WorldActorPosition { x: 300.0, y: 400.0 },
        )
        .expect("enemy registration should succeed");

    let distance = world_state
        .distance_between("controlled_champion", "enemy_frontline")
        .expect("distance should exist for both registered actors");
    assert!(
        (distance - 500.0).abs() < 1e-9,
        "unexpected distance value: {}",
        distance
    );
}

#[test]
fn world_state_rejects_actor_positions_outside_bounds() {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    let err = world_state
        .register_actor_position(
            "enemy_out_of_bounds",
            WorldActorClass::Champion,
            WorldActorPosition { x: 9000.0, y: 0.0 },
        )
        .expect_err("out-of-bounds actor positions should fail");
    assert!(
        err.to_string().contains("outside map bounds"),
        "unexpected error: {}",
        err
    );
}

#[test]
fn world_state_rejects_duplicate_actor_ids() {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    world_state
        .register_actor_position_with_allegiance(
            "shared_actor_id",
            WorldActorClass::Champion,
            WorldActorAllegiance::OpponentTeam,
            WorldActorPosition { x: 0.0, y: 0.0 },
        )
        .expect("initial registration should succeed");
    let err = world_state
        .register_actor_position_with_allegiance(
            "shared_actor_id",
            WorldActorClass::Champion,
            WorldActorAllegiance::OpponentTeam,
            WorldActorPosition { x: 100.0, y: 50.0 },
        )
        .expect_err("duplicate actor IDs should fail");
    assert!(
        err.to_string().contains("already registered"),
        "unexpected error: {}",
        err
    );
}

#[test]
fn world_state_upsert_clamps_positions_to_map_bounds() {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    world_state
        .register_actor_position_with_allegiance(
            "enemy",
            WorldActorClass::Champion,
            WorldActorAllegiance::OpponentTeam,
            WorldActorPosition { x: 0.0, y: 0.0 },
        )
        .expect("initial enemy registration should succeed");

    let clamped = world_state.upsert_actor_position_clamped(
        "enemy",
        WorldActorClass::Champion,
        WorldActorAllegiance::OpponentTeam,
        WorldActorPosition {
            x: 99_999.0,
            y: -99_999.0,
        },
    );
    assert!(
        default_urf_world_map_state()
            .bounds
            .contains(clamped.x, clamped.y),
        "clamped world position should remain map-bounded"
    );
}

#[test]
fn encounter_world_state_includes_non_champion_ecology_anchors() {
    let world_state = build_world_state_for_controlled_champion_encounter("Vladimir", &[])
        .expect("encounter world-state build should succeed");

    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Structure,
            WorldActorAllegiance::ControlledChampionTeam
        ),
        1
    );
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Structure,
            WorldActorAllegiance::OpponentTeam
        ),
        1
    );
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Monster,
            WorldActorAllegiance::NeutralWorld
        ),
        2
    );
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Minion,
            WorldActorAllegiance::ControlledChampionTeam
        ),
        1
    );
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Minion,
            WorldActorAllegiance::OpponentTeam
        ),
        1
    );
}

fn test_world_lifecycle_defaults() -> WorldLifecycleDefaults {
    WorldLifecycleDefaults {
        minion_wave_start_seconds: 10.0,
        minion_wave_interval_seconds: 30.0,
        minion_units_per_team_per_wave: 2,
        minion_lifetime_seconds: 20.0,
        dragon_initial_spawn_seconds: 5.0,
        dragon_respawn_seconds: 300.0,
        baron_initial_spawn_seconds: 20.0,
        baron_respawn_seconds: 360.0,
    }
}

#[test]
fn world_lifecycle_spawns_and_despawns_minion_wave_actors() {
    let mut world_state = build_world_state_for_controlled_champion_encounter("Vladimir", &[])
        .expect("encounter world-state build should succeed");
    let mut lifecycle = WorldLifecycleState::new(test_world_lifecycle_defaults());

    lifecycle.advance_to_time(&mut world_state, 9.0);
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Minion,
            WorldActorAllegiance::ControlledChampionTeam
        ),
        1
    );

    lifecycle.advance_to_time(&mut world_state, 10.0);
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Minion,
            WorldActorAllegiance::ControlledChampionTeam
        ),
        3
    );
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Minion,
            WorldActorAllegiance::OpponentTeam
        ),
        3
    );

    lifecycle.advance_to_time(&mut world_state, 31.0);
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Minion,
            WorldActorAllegiance::ControlledChampionTeam
        ),
        1
    );
    assert_eq!(
        world_state.actor_count_by_class_and_allegiance(
            WorldActorClass::Minion,
            WorldActorAllegiance::OpponentTeam
        ),
        1
    );
}

#[test]
fn world_lifecycle_neutral_objectives_respawn_after_defeat() {
    let mut world_state = build_world_state_for_controlled_champion_encounter("Vladimir", &[])
        .expect("encounter world-state build should succeed");
    let mut lifecycle = WorldLifecycleState::new(test_world_lifecycle_defaults());

    lifecycle.advance_to_time(&mut world_state, 6.0);
    assert!(
        world_state
            .actor_position("monster:dragon_objective")
            .is_some(),
        "dragon should spawn after initial objective timer"
    );

    assert!(
        lifecycle.mark_neutral_objective_defeated(
            &mut world_state,
            NeutralObjective::DragonObjective,
            6.0
        ),
        "defeat channel should remove active dragon objective actor"
    );
    assert!(
        world_state
            .actor_position("monster:dragon_objective")
            .is_none(),
        "dragon objective should be removed immediately on defeat"
    );

    lifecycle.advance_to_time(&mut world_state, 305.0);
    assert!(
        world_state
            .actor_position("monster:dragon_objective")
            .is_none(),
        "dragon should remain inactive before respawn timer"
    );
    lifecycle.advance_to_time(&mut world_state, 306.0);
    assert!(
        world_state
            .actor_position("monster:dragon_objective")
            .is_some(),
        "dragon objective should respawn after its configured timer"
    );
}

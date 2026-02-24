use super::world_actor_position_channels::{WorldActorClass, WorldActorPosition, WorldState};
use super::world_map_model::default_urf_world_map_state;

#[test]
fn world_state_registers_actor_positions_and_computes_distances() {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    world_state
        .register_actor_position(
            "controlled_champion",
            WorldActorClass::Champion,
            WorldActorPosition { x: 0.0, y: 0.0 },
        )
        .expect("controlled champion registration should succeed");
    world_state
        .register_actor_position(
            "enemy_frontline",
            WorldActorClass::Champion,
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
        .register_actor_position(
            "shared_actor_id",
            WorldActorClass::Champion,
            WorldActorPosition { x: 0.0, y: 0.0 },
        )
        .expect("initial registration should succeed");
    let err = world_state
        .register_actor_position(
            "shared_actor_id",
            WorldActorClass::Champion,
            WorldActorPosition { x: 100.0, y: 50.0 },
        )
        .expect_err("duplicate actor IDs should fail");
    assert!(
        err.to_string().contains("already registered"),
        "unexpected error: {}",
        err
    );
}

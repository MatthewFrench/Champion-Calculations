mod world_actor_lifecycle_channels;
mod world_actor_position_channels;
mod world_encounter_state_builder;
mod world_map_model;

pub(crate) use self::world_actor_lifecycle_channels::WorldLifecycleState;
pub(crate) use self::world_actor_position_channels::{
    WorldActorAllegiance, WorldActorClass, WorldActorPosition, WorldState,
};
pub(crate) use self::world_encounter_state_builder::{
    build_world_state_for_controlled_champion_encounter, seed_static_world_ecology_anchors,
};
pub(crate) use self::world_map_model::default_urf_world_map_state;

#[cfg(test)]
#[path = "world/tests/world_module_tests.rs"]
mod tests;

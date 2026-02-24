mod world_actor_position_channels;
mod world_encounter_state_builder;
mod world_map_model;

pub(crate) use self::world_encounter_state_builder::build_world_state_for_controlled_champion_encounter;

#[cfg(test)]
#[path = "world/tests/world_module_tests.rs"]
mod tests;

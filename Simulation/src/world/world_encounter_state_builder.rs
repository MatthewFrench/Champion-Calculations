use anyhow::{Result, anyhow};

use crate::EnemyConfig;

use super::world_actor_position_channels::{WorldActorClass, WorldActorPosition, WorldState};
use super::world_map_model::default_urf_world_map_state;

// Scenario world-state assembly is intentionally explicit and deterministic so future map/pathing
// systems can replace combat-only placement assumptions without changing scenario ownership flow.
pub(crate) fn build_world_state_for_controlled_champion_encounter(
    controlled_champion_name: &str,
    enemy_configs: &[EnemyConfig],
) -> Result<WorldState> {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    world_state.register_actor_position(
        controlled_champion_name,
        WorldActorClass::Champion,
        WorldActorPosition { x: 0.0, y: 0.0 },
    )?;

    for enemy in enemy_configs {
        let Some((x, y)) = enemy.spawn_position_xy else {
            continue;
        };
        world_state.register_actor_position(
            &enemy.id,
            WorldActorClass::Champion,
            WorldActorPosition { x, y },
        )?;
    }

    if world_state
        .actor_position(controlled_champion_name)
        .is_none()
    {
        return Err(anyhow!(
            "controlled champion '{}' is missing from world encounter state",
            controlled_champion_name
        ));
    }
    if world_state
        .actor_snapshot(controlled_champion_name)
        .map(|snapshot| snapshot.actor_class != WorldActorClass::Champion)
        .unwrap_or(true)
    {
        return Err(anyhow!(
            "controlled champion '{}' must be registered as world actor class Champion",
            controlled_champion_name
        ));
    }

    Ok(world_state)
}

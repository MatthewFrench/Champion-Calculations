use anyhow::{Result, anyhow};

use crate::EnemyConfig;

use super::world_actor_position_channels::{
    WorldActorAllegiance, WorldActorClass, WorldActorPosition, WorldState,
};
use super::world_map_model::default_urf_world_map_state;

const STATIC_WORLD_ECOLOGY_ANCHORS: &[(
    &str,
    WorldActorClass,
    WorldActorAllegiance,
    WorldActorPosition,
)] = &[
    (
        "structure:blue_nexus",
        WorldActorClass::Structure,
        WorldActorAllegiance::ControlledChampionTeam,
        WorldActorPosition {
            x: -6500.0,
            y: -6500.0,
        },
    ),
    (
        "structure:red_nexus",
        WorldActorClass::Structure,
        WorldActorAllegiance::OpponentTeam,
        WorldActorPosition {
            x: 6500.0,
            y: 6500.0,
        },
    ),
    (
        "monster:baron_pit_anchor",
        WorldActorClass::Monster,
        WorldActorAllegiance::NeutralWorld,
        WorldActorPosition {
            x: 4500.0,
            y: 6200.0,
        },
    ),
    (
        "monster:dragon_pit_anchor",
        WorldActorClass::Monster,
        WorldActorAllegiance::NeutralWorld,
        WorldActorPosition {
            x: -4200.0,
            y: 5200.0,
        },
    ),
    (
        "minion:blue_lane_spawn_anchor",
        WorldActorClass::Minion,
        WorldActorAllegiance::ControlledChampionTeam,
        WorldActorPosition {
            x: -6200.0,
            y: -6200.0,
        },
    ),
    (
        "minion:red_lane_spawn_anchor",
        WorldActorClass::Minion,
        WorldActorAllegiance::OpponentTeam,
        WorldActorPosition {
            x: 6200.0,
            y: 6200.0,
        },
    ),
];

pub(crate) fn seed_static_world_ecology_anchors(world_state: &mut WorldState) {
    for (actor_id, actor_class, actor_allegiance, actor_position) in STATIC_WORLD_ECOLOGY_ANCHORS {
        world_state.upsert_actor_position_clamped(
            actor_id,
            *actor_class,
            *actor_allegiance,
            *actor_position,
        );
    }
}

// Scenario world-state assembly is intentionally explicit and deterministic so future map/pathing
// systems can replace combat-only placement assumptions without changing scenario ownership flow.
pub(crate) fn build_world_state_for_controlled_champion_encounter(
    controlled_champion_name: &str,
    enemy_configs: &[EnemyConfig],
) -> Result<WorldState> {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    seed_static_world_ecology_anchors(&mut world_state);

    world_state.register_actor_position_with_allegiance(
        controlled_champion_name,
        WorldActorClass::Champion,
        WorldActorAllegiance::ControlledChampionTeam,
        WorldActorPosition { x: 0.0, y: 0.0 },
    )?;

    for enemy in enemy_configs {
        let Some((x, y)) = enemy.spawn_position_xy else {
            continue;
        };
        world_state.register_actor_position_with_allegiance(
            &enemy.id,
            WorldActorClass::Champion,
            WorldActorAllegiance::OpponentTeam,
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

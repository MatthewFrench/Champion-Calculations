use std::collections::HashMap;

use anyhow::{Result, anyhow};

use super::world_map_model::WorldMapState;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorldActorClass {
    Champion,
    Minion,
    Monster,
    Structure,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WorldActorPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct WorldActorSnapshot {
    pub actor_class: WorldActorClass,
    pub position: WorldActorPosition,
}

#[derive(Debug, Clone)]
pub(crate) struct WorldState {
    pub map: WorldMapState,
    actor_snapshots: HashMap<String, WorldActorSnapshot>,
}

impl WorldState {
    pub(crate) fn new(map: WorldMapState) -> Self {
        Self {
            map,
            actor_snapshots: HashMap::new(),
        }
    }

    pub(crate) fn register_actor_position(
        &mut self,
        actor_id: &str,
        actor_class: WorldActorClass,
        position: WorldActorPosition,
    ) -> Result<()> {
        if actor_id.trim().is_empty() {
            return Err(anyhow!("world actor id cannot be empty"));
        }
        if !self.map.bounds.contains(position.x, position.y) {
            return Err(anyhow!(
                "world actor '{}' position ({:.2}, {:.2}) is outside map bounds [{:.2}, {:.2}] x [{:.2}, {:.2}] for {}",
                actor_id,
                position.x,
                position.y,
                self.map.bounds.min_x,
                self.map.bounds.max_x,
                self.map.bounds.min_y,
                self.map.bounds.max_y,
                self.map.map_name
            ));
        }
        if self.actor_snapshots.contains_key(actor_id) {
            return Err(anyhow!(
                "world actor id '{}' is already registered in world state",
                actor_id
            ));
        }
        self.actor_snapshots.insert(
            actor_id.to_string(),
            WorldActorSnapshot {
                actor_class,
                position,
            },
        );
        Ok(())
    }

    pub(crate) fn actor_position(&self, actor_id: &str) -> Option<WorldActorPosition> {
        self.actor_snapshots
            .get(actor_id)
            .map(|snapshot| snapshot.position)
    }

    pub(crate) fn actor_snapshot(&self, actor_id: &str) -> Option<&WorldActorSnapshot> {
        self.actor_snapshots.get(actor_id)
    }

    #[cfg(test)]
    pub(crate) fn distance_between(
        &self,
        first_actor_id: &str,
        second_actor_id: &str,
    ) -> Option<f64> {
        let first = self.actor_position(first_actor_id)?;
        let second = self.actor_position(second_actor_id)?;
        Some(((second.x - first.x).powi(2) + (second.y - first.y).powi(2)).sqrt())
    }
}

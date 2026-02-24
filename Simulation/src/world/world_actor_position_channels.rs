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

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorldActorAllegiance {
    ControlledChampionTeam,
    OpponentTeam,
    NeutralWorld,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WorldActorPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct WorldActorSnapshot {
    pub actor_class: WorldActorClass,
    pub actor_allegiance: WorldActorAllegiance,
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

    pub(crate) fn register_actor_position_with_allegiance(
        &mut self,
        actor_id: &str,
        actor_class: WorldActorClass,
        actor_allegiance: WorldActorAllegiance,
        position: WorldActorPosition,
    ) -> Result<()> {
        self.validate_actor_registration_input(actor_id, position)?;
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
                actor_allegiance,
                position,
            },
        );
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn register_actor_position(
        &mut self,
        actor_id: &str,
        actor_class: WorldActorClass,
        position: WorldActorPosition,
    ) -> Result<()> {
        self.register_actor_position_with_allegiance(
            actor_id,
            actor_class,
            WorldActorAllegiance::NeutralWorld,
            position,
        )
    }

    // Runtime channels can update actor positions every tick; this channel is explicit
    // upsert-plus-clamp ownership so movement stays map-bounded without panicking.
    pub(crate) fn upsert_actor_position_clamped(
        &mut self,
        actor_id: &str,
        actor_class: WorldActorClass,
        actor_allegiance: WorldActorAllegiance,
        position: WorldActorPosition,
    ) -> WorldActorPosition {
        let (x, y) = self.map.bounds.clamp(position.x, position.y);
        let clamped = WorldActorPosition { x, y };
        if actor_id.trim().is_empty() {
            return clamped;
        }
        self.actor_snapshots
            .entry(actor_id.to_string())
            .and_modify(|snapshot| {
                snapshot.actor_class = actor_class;
                snapshot.actor_allegiance = actor_allegiance;
                snapshot.position = clamped;
            })
            .or_insert(WorldActorSnapshot {
                actor_class,
                actor_allegiance,
                position: clamped,
            });
        clamped
    }

    fn validate_actor_registration_input(
        &self,
        actor_id: &str,
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

    // Controller/view systems need stable world snapshot enumeration without direct mutable access.
    #[allow(dead_code)]
    pub(crate) fn actor_snapshot_entries(&self) -> Vec<(String, WorldActorSnapshot)> {
        self.actor_snapshots
            .iter()
            .map(|(actor_id, snapshot)| (actor_id.clone(), snapshot.clone()))
            .collect()
    }

    pub(crate) fn remove_actor(&mut self, actor_id: &str) -> Option<WorldActorSnapshot> {
        self.actor_snapshots.remove(actor_id)
    }

    #[allow(dead_code)]
    pub(crate) fn actor_count_by_class_and_allegiance(
        &self,
        actor_class: WorldActorClass,
        actor_allegiance: WorldActorAllegiance,
    ) -> usize {
        self.actor_snapshots
            .values()
            .filter(|snapshot| {
                snapshot.actor_class == actor_class && snapshot.actor_allegiance == actor_allegiance
            })
            .count()
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

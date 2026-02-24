use std::collections::HashMap;

use crate::defaults::WorldLifecycleDefaults;

use super::world_actor_position_channels::{
    WorldActorAllegiance, WorldActorClass, WorldActorPosition, WorldState,
};

const BLUE_MINION_SPAWN_ANCHOR_ID: &str = "minion:blue_lane_spawn_anchor";
const RED_MINION_SPAWN_ANCHOR_ID: &str = "minion:red_lane_spawn_anchor";
const DRAGON_OBJECTIVE_SPAWN_ANCHOR_ID: &str = "monster:dragon_pit_anchor";
const BARON_OBJECTIVE_SPAWN_ANCHOR_ID: &str = "monster:baron_pit_anchor";
const DRAGON_OBJECTIVE_ACTOR_ID: &str = "monster:dragon_objective";
const BARON_OBJECTIVE_ACTOR_ID: &str = "monster:baron_nashor";

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct WorldLifecycleTickSummary {
    pub minion_waves_spawned: usize,
    pub minion_actors_spawned: usize,
    pub minion_actors_despawned: usize,
    pub neutral_objectives_spawned: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NeutralObjective {
    DragonObjective,
    BaronNashor,
}

impl NeutralObjective {
    fn actor_id(self) -> &'static str {
        match self {
            Self::DragonObjective => DRAGON_OBJECTIVE_ACTOR_ID,
            Self::BaronNashor => BARON_OBJECTIVE_ACTOR_ID,
        }
    }

    fn spawn_anchor_actor_id(self) -> &'static str {
        match self {
            Self::DragonObjective => DRAGON_OBJECTIVE_SPAWN_ANCHOR_ID,
            Self::BaronNashor => BARON_OBJECTIVE_SPAWN_ANCHOR_ID,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct NeutralObjectiveLifecycleState {
    respawn_seconds: f64,
    next_spawn_at_seconds: f64,
    is_active: bool,
}

impl NeutralObjectiveLifecycleState {
    fn with_initial_spawn(initial_spawn_seconds: f64, respawn_seconds: f64) -> Self {
        Self {
            respawn_seconds: respawn_seconds.max(0.0),
            next_spawn_at_seconds: initial_spawn_seconds.max(0.0),
            is_active: false,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WorldLifecycleState {
    minion_wave_start_seconds: f64,
    minion_wave_interval_seconds: f64,
    minion_units_per_team_per_wave: usize,
    minion_lifetime_seconds: f64,
    next_minion_wave_spawn_at_seconds: f64,
    next_minion_wave_sequence: u64,
    minion_actor_despawn_at_seconds: HashMap<String, f64>,
    dragon_lifecycle: NeutralObjectiveLifecycleState,
    baron_lifecycle: NeutralObjectiveLifecycleState,
    last_processed_time_seconds: f64,
}

impl WorldLifecycleState {
    // World lifecycle channels are deterministic and side-effect constrained:
    // all non-champion spawn/despawn transitions flow through this owner.
    pub(crate) fn new(defaults: WorldLifecycleDefaults) -> Self {
        Self {
            minion_wave_start_seconds: defaults.minion_wave_start_seconds.max(0.0),
            minion_wave_interval_seconds: defaults.minion_wave_interval_seconds.max(0.0),
            minion_units_per_team_per_wave: defaults.minion_units_per_team_per_wave,
            minion_lifetime_seconds: defaults.minion_lifetime_seconds.max(0.0),
            next_minion_wave_spawn_at_seconds: defaults.minion_wave_start_seconds.max(0.0),
            next_minion_wave_sequence: 1,
            minion_actor_despawn_at_seconds: HashMap::new(),
            dragon_lifecycle: NeutralObjectiveLifecycleState::with_initial_spawn(
                defaults.dragon_initial_spawn_seconds,
                defaults.dragon_respawn_seconds,
            ),
            baron_lifecycle: NeutralObjectiveLifecycleState::with_initial_spawn(
                defaults.baron_initial_spawn_seconds,
                defaults.baron_respawn_seconds,
            ),
            last_processed_time_seconds: 0.0,
        }
    }

    pub(crate) fn advance_to_time(
        &mut self,
        world_state: &mut WorldState,
        to_time_seconds: f64,
    ) -> WorldLifecycleTickSummary {
        let target_time_seconds = to_time_seconds.max(0.0);
        if target_time_seconds <= self.last_processed_time_seconds {
            return WorldLifecycleTickSummary::default();
        }

        let mut summary = WorldLifecycleTickSummary::default();
        self.spawn_due_neutral_objectives(world_state, target_time_seconds, &mut summary);
        self.spawn_due_minion_waves(world_state, target_time_seconds, &mut summary);
        self.despawn_expired_minion_actors(world_state, target_time_seconds, &mut summary);
        self.last_processed_time_seconds = target_time_seconds;
        summary
    }

    #[allow(dead_code)]
    pub(crate) fn mark_neutral_objective_defeated(
        &mut self,
        world_state: &mut WorldState,
        objective: NeutralObjective,
        defeated_at_seconds: f64,
    ) -> bool {
        let lifecycle_state = self.neutral_objective_lifecycle_state_mut(objective);
        if !lifecycle_state.is_active {
            return false;
        }
        lifecycle_state.is_active = false;
        lifecycle_state.next_spawn_at_seconds =
            defeated_at_seconds.max(0.0) + lifecycle_state.respawn_seconds;
        world_state.remove_actor(objective.actor_id()).is_some()
    }

    fn spawn_due_neutral_objectives(
        &mut self,
        world_state: &mut WorldState,
        target_time_seconds: f64,
        summary: &mut WorldLifecycleTickSummary,
    ) {
        for objective in [
            NeutralObjective::DragonObjective,
            NeutralObjective::BaronNashor,
        ] {
            let lifecycle_state = self.neutral_objective_lifecycle_state_mut(objective);
            if lifecycle_state.is_active
                || lifecycle_state.next_spawn_at_seconds > target_time_seconds + 1e-9
            {
                continue;
            }
            let Some(anchor_position) =
                world_state.actor_position(objective.spawn_anchor_actor_id())
            else {
                continue;
            };
            world_state.upsert_actor_position_clamped(
                objective.actor_id(),
                WorldActorClass::Monster,
                WorldActorAllegiance::NeutralWorld,
                anchor_position,
            );
            lifecycle_state.is_active = true;
            summary.neutral_objectives_spawned += 1;
        }
    }

    fn spawn_due_minion_waves(
        &mut self,
        world_state: &mut WorldState,
        target_time_seconds: f64,
        summary: &mut WorldLifecycleTickSummary,
    ) {
        if self.minion_wave_interval_seconds <= 0.0
            || self.minion_units_per_team_per_wave == 0
            || self.minion_wave_start_seconds.is_nan()
        {
            return;
        }
        while self.next_minion_wave_spawn_at_seconds <= target_time_seconds + 1e-9 {
            let spawned =
                self.spawn_minion_wave(world_state, self.next_minion_wave_spawn_at_seconds);
            if spawned > 0 {
                summary.minion_waves_spawned += 1;
                summary.minion_actors_spawned += spawned;
            }
            self.next_minion_wave_sequence = self.next_minion_wave_sequence.wrapping_add(1);
            self.next_minion_wave_spawn_at_seconds += self.minion_wave_interval_seconds;
        }
    }

    fn spawn_minion_wave(
        &mut self,
        world_state: &mut WorldState,
        wave_spawn_seconds: f64,
    ) -> usize {
        let mut spawned_actor_count = 0usize;
        spawned_actor_count += self.spawn_minion_wave_for_team(
            world_state,
            "blue",
            WorldActorAllegiance::ControlledChampionTeam,
            BLUE_MINION_SPAWN_ANCHOR_ID,
            wave_spawn_seconds,
        );
        spawned_actor_count += self.spawn_minion_wave_for_team(
            world_state,
            "red",
            WorldActorAllegiance::OpponentTeam,
            RED_MINION_SPAWN_ANCHOR_ID,
            wave_spawn_seconds,
        );
        spawned_actor_count
    }

    fn spawn_minion_wave_for_team(
        &mut self,
        world_state: &mut WorldState,
        team_label: &str,
        allegiance: WorldActorAllegiance,
        anchor_actor_id: &str,
        wave_spawn_seconds: f64,
    ) -> usize {
        let Some(anchor_position) = world_state.actor_position(anchor_actor_id) else {
            return 0;
        };
        let mut spawned_actor_count = 0usize;
        for unit_index in 0..self.minion_units_per_team_per_wave {
            let minion_actor_id = format!(
                "minion:{}:wave_{}:unit_{}",
                team_label,
                self.next_minion_wave_sequence,
                unit_index + 1
            );
            world_state.upsert_actor_position_clamped(
                &minion_actor_id,
                WorldActorClass::Minion,
                allegiance,
                WorldActorPosition {
                    x: anchor_position.x,
                    y: anchor_position.y,
                },
            );
            self.minion_actor_despawn_at_seconds.insert(
                minion_actor_id,
                wave_spawn_seconds + self.minion_lifetime_seconds,
            );
            spawned_actor_count += 1;
        }
        spawned_actor_count
    }

    fn despawn_expired_minion_actors(
        &mut self,
        world_state: &mut WorldState,
        target_time_seconds: f64,
        summary: &mut WorldLifecycleTickSummary,
    ) {
        let mut expired_ids = Vec::new();
        for (actor_id, despawn_at_seconds) in &self.minion_actor_despawn_at_seconds {
            if *despawn_at_seconds <= target_time_seconds + 1e-9 {
                expired_ids.push(actor_id.clone());
            }
        }
        for actor_id in expired_ids {
            self.minion_actor_despawn_at_seconds.remove(&actor_id);
            if world_state.remove_actor(&actor_id).is_some() {
                summary.minion_actors_despawned += 1;
            }
        }
    }

    fn neutral_objective_lifecycle_state_mut(
        &mut self,
        objective: NeutralObjective,
    ) -> &mut NeutralObjectiveLifecycleState {
        match objective {
            NeutralObjective::DragonObjective => &mut self.dragon_lifecycle,
            NeutralObjective::BaronNashor => &mut self.baron_lifecycle,
        }
    }
}

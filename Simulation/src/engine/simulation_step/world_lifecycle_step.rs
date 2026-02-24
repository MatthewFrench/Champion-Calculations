use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn apply_world_lifecycle_step(&mut self, to_time_seconds: f64) {
        let summary = self
            .world_lifecycle_state
            .advance_to_time(&mut self.world_state, to_time_seconds);
        if summary.minion_waves_spawned > 0 {
            self.trace_event(
                "world_minion_wave_spawn",
                format!(
                    "{} minion waves spawned ({} actors)",
                    summary.minion_waves_spawned, summary.minion_actors_spawned
                ),
            );
        }
        if summary.minion_actors_despawned > 0 {
            self.trace_event(
                "world_minion_wave_despawn",
                format!(
                    "{} minion actors despawned",
                    summary.minion_actors_despawned
                ),
            );
        }
        if summary.neutral_objectives_spawned > 0 {
            self.trace_event(
                "world_neutral_objective_spawn",
                format!(
                    "{} neutral objectives became active",
                    summary.neutral_objectives_spawned
                ),
            );
        }
    }

    #[cfg(test)]
    pub(in crate::engine) fn world_actor_count_by_class_and_allegiance(
        &self,
        actor_class: crate::world::WorldActorClass,
        actor_allegiance: crate::world::WorldActorAllegiance,
    ) -> usize {
        self.world_state
            .actor_count_by_class_and_allegiance(actor_class, actor_allegiance)
    }

    #[cfg(test)]
    pub(in crate::engine) fn world_actor_position(
        &self,
        actor_id: &str,
    ) -> Option<crate::world::WorldActorPosition> {
        self.world_state.actor_position(actor_id)
    }
}

use super::super::*;

impl ControlledChampionCombatSimulation {
    // Controlled champion movement is command-owned and deterministic: one move target is stepped
    // at server tick rate, map clamped each tick, and never mutated outside this channel.
    pub(in crate::engine) fn apply_controlled_champion_movement_step(
        &mut self,
        delta_seconds: f64,
    ) {
        if delta_seconds <= 0.0 || self.controlled_champion_is_stunned() {
            return;
        }
        if self.combat_primitives.cast_lock().is_locked() {
            return;
        }

        let Some(target_position) = self.controlled_champion_pending_move_target_position() else {
            return;
        };

        let current_world_position = self
            .world_state
            .actor_position(&self.controlled_champion_world_actor_id)
            .unwrap_or(WorldActorPosition {
                x: self.target_position.x,
                y: self.target_position.y,
            });
        let current_position = Vec2 {
            x: current_world_position.x,
            y: current_world_position.y,
        };

        let distance_to_target = current_position.distance_to(target_position);
        if distance_to_target <= 1e-9 {
            self.clear_controlled_champion_move_command();
            return;
        }

        let move_speed_units_per_second =
            self.controlled_champion_movement_speed_units_per_second();
        if move_speed_units_per_second <= 0.0 {
            return;
        }
        let max_travel_distance = move_speed_units_per_second * delta_seconds;
        let next_position = if distance_to_target <= max_travel_distance + 1e-9 {
            self.clear_controlled_champion_move_command();
            target_position
        } else {
            let ratio = (max_travel_distance / distance_to_target).clamp(0.0, 1.0);
            Vec2 {
                x: current_position.x + (target_position.x - current_position.x) * ratio,
                y: current_position.y + (target_position.y - current_position.y) * ratio,
            }
        };

        let clamped_world_position = self.world_state.upsert_actor_position_clamped(
            &self.controlled_champion_world_actor_id,
            WorldActorClass::Champion,
            WorldActorAllegiance::ControlledChampionTeam,
            WorldActorPosition {
                x: next_position.x,
                y: next_position.y,
            },
        );
        self.target_position = Vec2 {
            x: clamped_world_position.x,
            y: clamped_world_position.y,
        };
    }

    fn controlled_champion_movement_speed_units_per_second(&self) -> f64 {
        let base_move_speed = (self.controlled_champion_base.base_move_speed
            + self.controlled_champion_stats.move_speed_flat)
            .max(0.0)
            * (1.0 + self.controlled_champion_stats.move_speed_percent / 100.0).max(0.0);
        let runtime_movement_speed_multiplier = movement_speed_multiplier(
            &self.controlled_champion_combat_runtime,
            self.time,
            self.sim.champion_level,
        );
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: base_move_speed,
                source: ScalarMetricSource::MovementSpeed,
                clamp_min_zero: true,
            },
            RuntimeBuffState {
                movement_speed_multiplier: runtime_movement_speed_multiplier,
                ..RuntimeBuffState::default()
            },
        )
    }
}

use super::super::*;
use crate::world::{WorldActorAllegiance, WorldActorClass, WorldActorPosition};

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn apply_enemy_movement_step(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }

        if let Some(controlled_champion_world_position) = self
            .world_state
            .actor_position(&self.controlled_champion_world_actor_id)
        {
            self.target_position = Vec2 {
                x: controlled_champion_world_position.x,
                y: controlled_champion_world_position.y,
            };
        }

        for idx in 0..self.enemy_state.len() {
            let movement_step = {
                let state = &mut self.enemy_state[idx];
                if state.respawn_at.is_some()
                    || state.health <= 0.0
                    || state.movement_mode == OpponentMovementMode::HoldPosition
                {
                    None
                } else {
                    let runtime_movement_speed_multiplier =
                        movement_speed_multiplier(&state.runtime, self.time, state.enemy.level);
                    let speed = resolve_stat(
                        StatQuery::ScalarAmount {
                            base_amount: state.move_speed * state.behavior.movement_speed_scale,
                            source: ScalarMetricSource::MovementSpeed,
                            clamp_min_zero: true,
                        },
                        RuntimeBuffState {
                            movement_speed_multiplier: runtime_movement_speed_multiplier,
                            ..RuntimeBuffState::default()
                        },
                    );
                    let tangent_dir = if idx % 2 == 0 { 1.0 } else { -1.0 };
                    let tangential_step_scale = if state.enemy.base.is_melee {
                        0.08
                    } else {
                        0.20
                    };
                    let next_position = update_enemy_orbit_position(
                        state.position,
                        self.target_position,
                        speed * delta,
                        state.behavior.desired_combat_range,
                        tangent_dir,
                        tangential_step_scale,
                    );
                    Some((state.enemy.id.clone(), next_position))
                }
            };

            let Some((enemy_actor_id, next_position)) = movement_step else {
                continue;
            };
            let clamped_world_position = self.world_state.upsert_actor_position_clamped(
                &enemy_actor_id,
                WorldActorClass::Champion,
                WorldActorAllegiance::OpponentTeam,
                WorldActorPosition {
                    x: next_position.x,
                    y: next_position.y,
                },
            );
            self.enemy_state[idx].position = Vec2 {
                x: clamped_world_position.x,
                y: clamped_world_position.y,
            };
        }
    }
}

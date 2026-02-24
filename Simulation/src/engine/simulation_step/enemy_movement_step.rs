use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn apply_enemy_movement_step(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        for (idx, state) in self.enemy_state.iter_mut().enumerate() {
            if state.respawn_at.is_some() || state.health <= 0.0 {
                continue;
            }
            if state.movement_mode == OpponentMovementMode::HoldPosition {
                continue;
            }
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
            state.position = update_enemy_orbit_position(
                state.position,
                self.target_position,
                speed * delta,
                state.behavior.desired_combat_range,
                tangent_dir,
                tangential_step_scale,
            );
        }
    }
}

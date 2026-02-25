use super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn enemy_has_stasis_item(&self, idx: usize) -> bool {
        self.enemy_state
            .get(idx)
            .map(|state| state.stasis_item_available)
            .unwrap_or(false)
    }

    pub(in crate::engine) fn enemy_stasis_item_ready_at_or_zero(&self, idx: usize) -> f64 {
        self.enemy_state
            .get(idx)
            .map(|state| state.stasis_item_ready_at)
            .unwrap_or(0.0)
    }

    pub(in crate::engine) fn enemy_has_emergency_shield_item(&self, idx: usize) -> bool {
        self.enemy_state
            .get(idx)
            .map(|state| state.emergency_shield_item_available)
            .unwrap_or(false)
    }

    pub(in crate::engine) fn enemy_emergency_shield_item_ready_at_or_zero(
        &self,
        idx: usize,
    ) -> f64 {
        self.enemy_state
            .get(idx)
            .map(|state| state.emergency_shield_item_ready_at)
            .unwrap_or(0.0)
    }

    pub(in crate::engine) fn try_activate_enemy_stasis_item_active(&mut self, idx: usize) -> bool {
        if !self.enemy_can_take_actions(idx) {
            return false;
        }
        let Some(state) = self.enemy_state.get_mut(idx) else {
            return false;
        };
        if !state.stasis_item_available {
            return false;
        }
        if self.time + 1e-9 < state.stasis_item_ready_at {
            return false;
        }
        if state.respawn_at.is_some() || state.health <= 0.0 {
            return false;
        }
        state.stasis_item_ready_at = self.time + self.stasis_item_cooldown_seconds;
        state.stasis_until = self.time + self.sim.zhonya_duration_seconds;
        true
    }

    pub(in crate::engine) fn try_activate_enemy_emergency_shield_item_active(
        &mut self,
        idx: usize,
    ) -> bool {
        if !self.enemy_can_take_actions(idx) {
            return false;
        }
        let Some(state) = self.enemy_state.get_mut(idx) else {
            return false;
        };
        if !state.emergency_shield_item_available {
            return false;
        }
        if self.time + 1e-9 < state.emergency_shield_item_ready_at {
            return false;
        }
        if state.respawn_at.is_some() || state.health <= 0.0 {
            return false;
        }
        state.emergency_shield_item_ready_at =
            self.time + self.emergency_shield_item_cooldown_seconds;
        state.emergency_shield_amount += self.sim.protoplasm_bonus_health.max(0.0);
        state.emergency_heal_rate =
            self.sim.protoplasm_heal_total / self.sim.protoplasm_duration_seconds.max(0.001);
        state.emergency_heal_until = self.time + self.sim.protoplasm_duration_seconds;
        true
    }
}

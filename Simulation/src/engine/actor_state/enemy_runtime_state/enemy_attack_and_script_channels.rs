use super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn invalidate_enemy_attack_sequence(
        &mut self,
        idx: usize,
    ) -> Option<u64> {
        let state = self.enemy_state.get_mut(idx)?;
        state.attack_sequence = state.attack_sequence.wrapping_add(1);
        Some(state.attack_sequence)
    }

    pub(in crate::engine) fn begin_enemy_attack_sequence(&mut self, idx: usize) -> Option<u64> {
        let state = self.enemy_state.get_mut(idx)?;
        state.attack_sequence = state.attack_sequence.wrapping_add(1);
        Some(state.attack_sequence)
    }

    pub(in crate::engine) fn enemy_attack_sequence_matches(&self, idx: usize, token: u64) -> bool {
        self.enemy_state
            .get(idx)
            .map(|state| state.attack_sequence == token)
            .unwrap_or(false)
    }

    pub(in crate::engine) fn apply_enemy_next_attack_bonus_physical(
        &mut self,
        idx: usize,
        amount: f64,
    ) -> Option<String> {
        let state = self.enemy_state.get_mut(idx)?;
        state.next_attack_bonus_physical += amount;
        Some(state.enemy.name.clone())
    }

    pub(in crate::engine) fn consume_enemy_attack_damage_with_on_hit(
        &mut self,
        idx: usize,
        target_current_health: f64,
        target_max_health: f64,
    ) -> Option<(f64, f64, f64)> {
        let now = self.time;
        let state = self.enemy_state.get_mut(idx)?;
        let attack_damage = state.physical_hit_damage + state.next_attack_bonus_physical;
        let bonus_attack_damage =
            (state.physical_hit_damage - state.enemy.base.base_attack_damage).max(0.0);
        let (extra_physical, extra_magic, extra_true) = on_hit_bonus_damage(
            state.behavior,
            &mut state.runtime,
            attack_damage,
            state.ability_power,
            bonus_attack_damage,
            target_current_health,
            target_max_health,
            state.max_health,
            now,
            Some(0),
            state.enemy.level,
        );
        let output = (
            attack_damage + extra_physical,
            state.next_attack_bonus_magic + extra_magic,
            state.next_attack_bonus_true + extra_true,
        );
        state.next_attack_bonus_physical = 0.0;
        state.next_attack_bonus_magic = 0.0;
        state.next_attack_bonus_true = 0.0;
        Some(output)
    }

    pub(in crate::engine) fn enemy_aftershock_magic_damage_on_immobilize(
        &mut self,
        idx: usize,
    ) -> f64 {
        let now = self.time;
        let Some(state) = self.enemy_state.get_mut(idx) else {
            return 0.0;
        };
        let enemy_level = state.enemy.level;
        let enemy_bonus_health = (state.max_health - state.enemy.base.base_health).max(0.0);
        on_immobilize_rune_damage(&mut state.runtime, now, enemy_level, enemy_bonus_health)
    }

    pub(in crate::engine) fn execute_enemy_script_event_actions(
        &mut self,
        idx: usize,
        script_event: ChampionScriptEvent,
        distance_to_target: f64,
        target_current_health: f64,
        target_max_health: f64,
        now: f64,
    ) -> Vec<ChampionScriptAction> {
        let Some(state) = self.enemy_state.get_mut(idx) else {
            return Vec::new();
        };
        let input = ChampionScriptExecutionInput {
            event: script_event,
            actor_position: champion_script_point_from_vec2(state.position),
            actor_level: state.enemy.level,
            distance_to_target,
            physical_hit_damage: state.physical_hit_damage,
            actor_ability_power: state.ability_power,
            actor_bonus_attack_damage: (state.physical_hit_damage
                - state.enemy.base.base_attack_damage)
                .max(0.0),
            target_current_health,
            target_max_health,
            now,
        };
        execute_champion_script_event(input, &mut state.runtime)
    }
}

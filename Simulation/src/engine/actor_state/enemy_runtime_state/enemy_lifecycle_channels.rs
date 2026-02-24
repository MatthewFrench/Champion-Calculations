use super::*;
use crate::world::{WorldActorAllegiance, WorldActorClass, WorldActorPosition};

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn apply_enemy_respawn_updates(&mut self) {
        let mut respawned = Vec::new();
        for (idx, state) in self.enemy_state.iter_mut().enumerate() {
            let Some(respawn_at) = state.respawn_at else {
                continue;
            };
            if self.time >= respawn_at {
                state.health = state.max_health;
                state.respawn_at = None;
                state.position = state.spawn_position;
                clear_transient_combat_state(&mut state.runtime);
                state.next_attack_bonus_physical = 0.0;
                state.next_attack_bonus_magic = 0.0;
                state.next_attack_bonus_true = 0.0;
                state.script_epoch = state.script_epoch.wrapping_add(1);
                state.script_event_ready_at.clear();
                state.attack_sequence = state.attack_sequence.wrapping_add(1);
                state.stunned_until = 0.0;
                state.untargetable_until = 0.0;
                state.stasis_until = 0.0;
                state.invulnerable_until = 0.0;
                respawned.push((idx, state.enemy.name.clone(), state.script_epoch));
            }
        }
        for (idx, name, epoch) in respawned {
            let champion_name = self.enemy_state[idx].enemy.name.clone();
            let poll_interval = self.enemy_state[idx].script_poll_interval_seconds.max(0.05);
            for event in scripted_champion_events(&champion_name) {
                self.schedule_event(
                    0.0,
                    12,
                    EventType::ChampionScript(idx, event, epoch),
                    Some(poll_interval),
                );
            }
            self.trace_event("enemy_respawn", format!("{} respawned", name));
            let respawn_position = self.enemy_state[idx].position;
            let enemy_actor_id = self.enemy_state[idx].enemy.id.clone();
            self.world_state.upsert_actor_position_clamped(
                &enemy_actor_id,
                WorldActorClass::Champion,
                WorldActorAllegiance::OpponentTeam,
                WorldActorPosition {
                    x: respawn_position.x,
                    y: respawn_position.y,
                },
            );
        }
    }

    pub(in crate::engine) fn enemy_is_alive(&self, idx: usize) -> bool {
        let state = &self.enemy_state[idx];
        state.respawn_at.is_none() && state.health > 0.0
    }

    pub(in crate::engine) fn enemy_is_active(&self, idx: usize) -> bool {
        self.enemy_is_alive(idx)
    }

    pub(in crate::engine) fn enemy_script_event_should_recur(
        &self,
        idx: usize,
        epoch: u64,
    ) -> bool {
        self.enemy_state
            .get(idx)
            .map(|state| {
                state.script_epoch == epoch && state.respawn_at.is_none() && state.health > 0.0
            })
            .unwrap_or(false)
    }

    pub(in crate::engine) fn enemy_script_epoch_matches(&self, idx: usize, epoch: u64) -> bool {
        self.enemy_state
            .get(idx)
            .map(|state| state.script_epoch == epoch)
            .unwrap_or(false)
    }

    pub(in crate::engine) fn enemy_script_event_ready_at_or_zero(
        &self,
        idx: usize,
        event: ChampionScriptEvent,
    ) -> f64 {
        self.enemy_state
            .get(idx)
            .and_then(|state| state.script_event_ready_at.get(&event).copied())
            .unwrap_or(0.0)
    }

    pub(in crate::engine) fn enemy_ability_haste_or_urf_default(&self, idx: usize) -> f64 {
        self.enemy_state
            .get(idx)
            .map(|state| state.ability_haste)
            .unwrap_or(self.urf.ability_haste)
    }

    pub(in crate::engine) fn set_enemy_script_event_ready_at(
        &mut self,
        idx: usize,
        event: ChampionScriptEvent,
        ready_at: f64,
    ) {
        if let Some(state) = self.enemy_state.get_mut(idx) {
            state.script_event_ready_at.insert(event, ready_at);
        }
    }

    pub(in crate::engine) fn apply_enemy_regeneration_tick(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        for state in &mut self.enemy_state {
            if state.respawn_at.is_some() || state.health <= 0.0 {
                continue;
            }
            let heal = tick_regen_heal(&state.runtime, state.health, state.max_health, delta);
            state.health = (state.health + heal).min(state.max_health);
        }
    }
}

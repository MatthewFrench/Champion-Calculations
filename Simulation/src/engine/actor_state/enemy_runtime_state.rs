use super::super::script_point_coordinate_conversions::champion_script_point_from_vec2;
use super::super::*;

#[derive(Debug, Clone)]
pub(in crate::engine) struct EnemyTraceSnapshot {
    pub name: String,
    pub position: Vec2,
    pub health: f64,
    pub max_health: f64,
    pub armor: f64,
    pub magic_resist: f64,
    pub physical_hit_damage: f64,
    pub ability_power: f64,
    pub attack_speed: f64,
    pub attack_interval_seconds: f64,
    pub ability_haste: f64,
    pub runtime_item_names: Vec<String>,
    pub runtime_rune_names: Vec<String>,
    pub runtime_effect_cooldowns: Vec<String>,
    pub runtime_effect_stacks: Vec<String>,
    pub status_lines: Vec<String>,
    pub scripted_ability_cooldowns: Vec<String>,
}

fn trace_cooldown_status_at(now: f64, ready_at: f64) -> String {
    let remaining = (ready_at - now).max(0.0);
    if remaining <= 1e-9 {
        "ready".to_string()
    } else {
        format!("{remaining:.2}s")
    }
}

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn enemy_name(&self, idx: usize) -> Option<String> {
        self.enemy_state
            .get(idx)
            .map(|state| state.enemy.name.clone())
    }

    pub(in crate::engine) fn enemy_position(&self, idx: usize) -> Option<Vec2> {
        self.enemy_state.get(idx).map(|state| state.position)
    }

    pub(in crate::engine) fn enemy_hitbox_radius(&self, idx: usize) -> Option<f64> {
        self.enemy_state.get(idx).map(|state| state.hitbox_radius)
    }

    pub(in crate::engine) fn enemy_attack_range(&self, idx: usize) -> Option<f64> {
        self.enemy_state
            .get(idx)
            .map(|state| state.behavior.attack_range)
    }

    pub(in crate::engine) fn enemy_attack_windup_seconds(&self, idx: usize) -> Option<f64> {
        self.enemy_state
            .get(idx)
            .map(|state| state.behavior.attack_windup_seconds)
    }

    pub(in crate::engine) fn enemy_attack_projectile_speed(&self, idx: usize) -> Option<f64> {
        self.enemy_state
            .get(idx)
            .map(|state| state.behavior.attack_projectile_speed)
    }

    pub(in crate::engine) fn enemy_attack_effect_hitbox_radius(&self, idx: usize) -> Option<f64> {
        self.enemy_state
            .get(idx)
            .map(|state| state.behavior.attack_effect_hitbox_radius)
    }

    pub(in crate::engine) fn enemy_is_stunned_at(&self, idx: usize, now: f64) -> bool {
        self.enemy_state
            .get(idx)
            .map(|state| now < state.stunned_until)
            .unwrap_or(false)
    }

    pub(in crate::engine) fn enemy_is_invulnerable_or_untargetable_at(
        &self,
        idx: usize,
        now: f64,
    ) -> bool {
        self.enemy_state
            .get(idx)
            .map(|state| {
                now < state.untargetable_until
                    || now < state.stasis_until
                    || now < state.invulnerable_until
            })
            .unwrap_or(false)
    }

    pub(in crate::engine) fn enemy_attack_interval_seconds(
        &self,
        idx: usize,
        now: f64,
        minimum_attack_speed: f64,
    ) -> Option<f64> {
        let state = self.enemy_state.get(idx)?;
        let attack_speed = state.base_attack_speed * attack_speed_multiplier(&state.runtime, now);
        Some(1.0 / attack_speed.max(minimum_attack_speed))
    }

    pub(in crate::engine) fn enemy_target_health_snapshot_or_defaults(
        &self,
        idx: usize,
    ) -> (f64, f64) {
        self.enemy_state
            .get(idx)
            .map(|state| (state.health.max(0.0), state.max_health.max(1.0)))
            .unwrap_or((0.0, 1.0))
    }

    pub(in crate::engine) fn enemy_status_lines_at(&self, idx: usize, now: f64) -> Vec<String> {
        let Some(state) = self.enemy_state.get(idx) else {
            return vec!["none".to_string()];
        };
        let mut lines = Vec::new();
        if let Some(respawn_at) = state.respawn_at {
            lines.push(format!("Respawning in {:.2}s", (respawn_at - now).max(0.0)));
        }
        if now < state.stunned_until {
            lines.push(format!(
                "Stunned {:.2}s",
                (state.stunned_until - now).max(0.0)
            ));
        }
        if now < state.untargetable_until {
            lines.push(format!(
                "Untargetable {:.2}s",
                (state.untargetable_until - now).max(0.0)
            ));
        }
        if now < state.stasis_until {
            lines.push(format!(
                "Stasis {:.2}s",
                (state.stasis_until - now).max(0.0)
            ));
        }
        if now < state.invulnerable_until {
            lines.push(format!(
                "Invulnerable {:.2}s",
                (state.invulnerable_until - now).max(0.0)
            ));
        }
        if lines.is_empty() {
            lines.push("none".to_string());
        }
        lines
    }

    pub(in crate::engine) fn enemy_count(&self) -> usize {
        self.enemy_state.len()
    }

    pub(in crate::engine) fn enemy_trace_snapshot_at(
        &self,
        idx: usize,
        now: f64,
    ) -> Option<EnemyTraceSnapshot> {
        let state = self.enemy_state.get(idx)?;
        let attack_speed = state.base_attack_speed * attack_speed_multiplier(&state.runtime, now);
        let attack_interval_seconds = 1.0 / attack_speed.max(0.001);
        let runtime_effect_cooldowns = describe_runtime_effect_cooldowns(&state.runtime, now);
        let runtime_effect_stacks = describe_runtime_effect_stacks(&state.runtime);
        let status_lines = self.enemy_status_lines_at(idx, now);
        let scripted_ability_cooldowns = scripted_champion_events(&state.enemy.name)
            .into_iter()
            .map(|event| {
                let ready_at = state
                    .script_event_ready_at
                    .get(&event)
                    .copied()
                    .unwrap_or(0.0);
                format!(
                    "{} {}",
                    champion_script_event_label(event),
                    trace_cooldown_status_at(now, ready_at)
                )
            })
            .collect();
        Some(EnemyTraceSnapshot {
            name: state.enemy.name.clone(),
            position: state.position,
            health: state.health.max(0.0),
            max_health: state.max_health,
            armor: state.armor,
            magic_resist: state.magic_resist,
            physical_hit_damage: state.physical_hit_damage,
            ability_power: state.ability_power,
            attack_speed,
            attack_interval_seconds,
            ability_haste: state.ability_haste,
            runtime_item_names: state.runtime_item_names.clone(),
            runtime_rune_names: state.runtime_rune_names.clone(),
            runtime_effect_cooldowns,
            runtime_effect_stacks,
            status_lines,
            scripted_ability_cooldowns,
        })
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
        let state = self
            .enemy_state
            .get_mut(idx)
            .expect("enemy script action index should be valid");
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
        let state = self
            .enemy_state
            .get_mut(idx)
            .expect("champion script event index should be valid");
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

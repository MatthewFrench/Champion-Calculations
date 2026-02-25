use super::*;

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
        if now < state.emergency_heal_until {
            lines.push(format!(
                "Emergency heal-over-time {:.2}s",
                (state.emergency_heal_until - now).max(0.0)
            ));
        }
        if state.emergency_shield_amount > 0.0 {
            lines.push(format!(
                "Emergency shield {:.1}",
                state.emergency_shield_amount
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
}

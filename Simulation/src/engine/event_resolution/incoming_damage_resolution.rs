use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn apply_healing_to_controlled_champion(
        &mut self,
        resolved_heal: f64,
    ) -> f64 {
        if resolved_heal <= 0.0 {
            return 0.0;
        }
        let before = self.health;
        self.health = self.max_health.min(self.health + resolved_heal);
        let applied_heal = (self.health - before).max(0.0);
        self.healing_done_total += applied_heal;
        applied_heal
    }

    pub(in crate::engine) fn apply_revive_or_mark_controlled_champion_death(&mut self) {
        if should_trigger_revive_effect(ReviveEffectDecisionInput {
            available: self.revive_item_available,
            now_seconds: self.time,
            ready_at: self.revive_item_ready_at,
        }) {
            self.revive_item_ready_at = self.time + self.revive_item_cooldown_seconds;
            self.revive_lockout_until = self.time + self.sim.ga_revive_duration_seconds;
            self.health = 1.0_f64.max(
                self.controlled_champion_base.base_health * self.sim.ga_revive_base_health_ratio,
            );
            self.trace_event(
                "revive_effect",
                format!("Revive item restored {}", self.controlled_champion_name),
            );
            return;
        }
        self.finished = true;
        self.death_time = Some(self.time);
        self.trace_event(
            "controlled_champion_death",
            format!("{} died", self.controlled_champion_name),
        );
    }

    pub(in crate::engine) fn apply_incoming_damage_to_enemy(
        &mut self,
        idx: usize,
        raw_physical_damage: f64,
        raw_magic_damage: f64,
        raw_true_damage: f64,
    ) -> f64 {
        if !self.enemy_is_active(idx) {
            return 0.0;
        }
        let (mitigated_physical, mitigated_magic, mitigated_true, enemy_level) = {
            let state = &self.enemy_state[idx];
            let bonus_armor = (state.armor - state.enemy.base.base_armor).max(0.0);
            let bonus_magic_resist =
                (state.magic_resist - state.enemy.base.base_magic_resist).max(0.0);
            let (aftershock_physical_multiplier, aftershock_magic_multiplier) =
                incoming_damage_multipliers(
                    &state.runtime,
                    self.time,
                    state.enemy.level,
                    state.armor,
                    state.magic_resist,
                    bonus_armor,
                    bonus_magic_resist,
                );
            (
                raw_physical_damage.max(0.0)
                    * state.physical_multiplier
                    * aftershock_physical_multiplier,
                raw_magic_damage.max(0.0) * state.magic_multiplier * aftershock_magic_multiplier,
                raw_true_damage.max(0.0),
                state.enemy.level,
            )
        };
        let mitigated = mitigated_physical + mitigated_magic + mitigated_true;
        if mitigated <= 0.0 {
            return 0.0;
        }
        let respawn_delay = self.enemy_respawn_delay_seconds(enemy_level);
        let mut killed_name = None;
        let dealt = {
            let state = &mut self.enemy_state[idx];
            let d = mitigated.min(state.health.max(0.0));
            state.health -= d;
            if state.health <= 0.0 {
                state.health = 0.0;
                state.respawn_at = Some(self.time + respawn_delay);
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
                killed_name = Some(state.enemy.name.clone());
            }
            d
        };
        if let Some(name) = killed_name {
            self.enemy_kills_total += 1;
            let runtime_kill_heal = enemy_kill_heal(
                &mut self.controlled_champion_combat_runtime,
                self.max_health,
            );
            if runtime_kill_heal > 0.0 {
                let script_heal_multiplier = controlled_champion_heal_multiplier();
                let resolved_heal = resolve_stat(
                    StatQuery::ScalarAmount {
                        base_amount: runtime_kill_heal * script_heal_multiplier,
                        source: ScalarMetricSource::Healing,
                        clamp_min_zero: true,
                    },
                    self.controlled_champion_buffs,
                );
                self.apply_healing_to_controlled_champion(resolved_heal);
            }
            self.trace_event(
                "enemy_death",
                format!("{} died; respawn in {:.1}s", name, respawn_delay),
            );
        }
        dealt
    }

    pub(in crate::engine) fn apply_incoming_magic_damage_to_enemy(
        &mut self,
        idx: usize,
        raw_magic_damage: f64,
    ) -> f64 {
        self.apply_incoming_damage_to_enemy(idx, 0.0, raw_magic_damage, 0.0)
    }

    pub(in crate::engine) fn apply_incoming_ability_bonus_damage_to_enemy(
        &mut self,
        idx: usize,
        ability_raw_damage: f64,
        ability_ap_ratio: f64,
        attacker_level: usize,
    ) -> f64 {
        if !self.enemy_is_active(idx) {
            return 0.0;
        }
        let target_current_health = self.enemy_state[idx].health.max(0.0);
        let target_max_health = self.enemy_state[idx].max_health.max(1.0);
        let (bonus_magic, bonus_true) = on_ability_bonus_damage(
            &mut self.controlled_champion_combat_runtime,
            ability_raw_damage,
            ability_ap_ratio,
            self.controlled_champion_stats.ability_power,
            self.controlled_champion_stats.attack_damage.max(0.0),
            target_current_health,
            target_max_health,
            self.time,
            Some(idx),
            attacker_level,
        );
        self.apply_incoming_damage_to_enemy(idx, 0.0, bonus_magic, bonus_true)
    }

    pub(in crate::engine) fn apply_incoming_ability_bonus_damage_to_enemies_in_controlled_champion_range(
        &mut self,
        ability_raw_damage: f64,
        ability_ap_ratio: f64,
        attacker_level: usize,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> (f64, usize) {
        let mut total = 0.0;
        let mut hit_count = 0usize;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius) {
                continue;
            }
            hit_count += 1;
            total += self.apply_incoming_ability_bonus_damage_to_enemy(
                idx,
                ability_raw_damage,
                ability_ap_ratio,
                attacker_level,
            );
        }
        (total, hit_count)
    }

    pub(in crate::engine) fn apply_healing_to_controlled_champion_from_outgoing_damage_runtime(
        &mut self,
        damage_dealt: f64,
    ) {
        if damage_dealt <= 0.0 {
            return;
        }
        let runtime_heal = outgoing_damage_heal(
            &mut self.controlled_champion_combat_runtime,
            damage_dealt,
            self.time,
        );
        if runtime_heal <= 0.0 {
            return;
        }
        let script_heal_multiplier = controlled_champion_heal_multiplier();
        let resolved_heal = resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: runtime_heal * script_heal_multiplier,
                source: ScalarMetricSource::Healing,
                clamp_min_zero: true,
            },
            self.controlled_champion_buffs,
        );
        self.apply_healing_to_controlled_champion(resolved_heal);
    }

    pub(in crate::engine) fn apply_incoming_magic_damage_to_enemies_in_controlled_champion_range(
        &mut self,
        raw_magic_damage: f64,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> (f64, usize) {
        if raw_magic_damage <= 0.0 {
            return (0.0, 0);
        }
        let mut total = 0.0;
        let mut hit_count = 0usize;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius) {
                continue;
            }
            hit_count += 1;
            total += self.apply_incoming_magic_damage_to_enemy(idx, raw_magic_damage);
        }
        (total, hit_count)
    }

    pub(in crate::engine) fn apply_incoming_damage_to_controlled_champion(
        &mut self,
        source: DamageSourceContext,
        physical: f64,
        magic: f64,
        true_damage: f64,
    ) -> DamageApplicationOutcome {
        if self.finished || self.health <= 0.0 {
            return DamageApplicationOutcome::Ignored;
        }
        if !self.is_targetable() {
            return DamageApplicationOutcome::NullifiedUntargetable;
        }
        let bonus_armor = (self.controlled_champion_stats.armor
            - self.controlled_champion_base.base_armor)
            .max(0.0);
        let bonus_magic_resist = (self.controlled_champion_stats.magic_resist
            - self.controlled_champion_base.base_magic_resist)
            .max(0.0);
        let (aftershock_physical_multiplier, aftershock_magic_multiplier) =
            incoming_damage_multipliers(
                &self.controlled_champion_combat_runtime,
                self.time,
                self.sim.champion_level,
                self.controlled_champion_stats.armor,
                self.controlled_champion_stats.magic_resist,
                bonus_armor,
                bonus_magic_resist,
            );
        let mut damage = physical * self.physical_multiplier * aftershock_physical_multiplier
            + magic * self.magic_multiplier * aftershock_magic_multiplier
            + true_damage;
        let active_enemy_count = self
            .enemy_state
            .iter()
            .filter(|state| state.respawn_at.is_none() && state.health > 0.0)
            .count();
        let script_damage_taken_multiplier =
            controlled_champion_damage_taken_multiplier(active_enemy_count);
        damage = resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: damage * script_damage_taken_multiplier,
                source: ScalarMetricSource::IncomingDamageTaken,
                clamp_min_zero: true,
            },
            self.controlled_champion_buffs,
        );
        if self.emergency_shield_amount > 0.0 && damage > 0.0 {
            let absorbed = self.emergency_shield_amount.min(damage);
            self.emergency_shield_amount -= absorbed;
            damage -= absorbed;
        }
        self.trace_event(
            "damage_in",
            format!(
                "{} {} -> {} | physical {:.1}, magic {:.1}, true {:.1}, total {:.1}",
                source.champion_name,
                source.ability_name,
                self.controlled_champion_name,
                physical,
                magic,
                true_damage,
                damage
            ),
        );
        self.health -= damage;
        if self.health <= 0.0 {
            self.apply_revive_or_mark_controlled_champion_death();
        }
        DamageApplicationOutcome::Applied
    }
}

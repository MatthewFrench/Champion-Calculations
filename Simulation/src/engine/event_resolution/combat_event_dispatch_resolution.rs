use super::super::*;

impl ControlledChampionCombatSimulation {
    fn process_event(&mut self, ev: &QueuedEvent) {
        match ev.kind {
            EventType::Attack(idx) => self.resolve_enemy_auto_attack_start_event(idx),
            EventType::AttackWindup { idx, token } => {
                self.resolve_enemy_auto_attack_windup_event(idx, token)
            }
            EventType::AttackHit {
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            } => self.resolve_enemy_auto_attack_hit_event(
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            ),
            EventType::ControlledChampionAttack => {
                self.resolve_controlled_champion_auto_attack_start_event()
            }
            EventType::ControlledChampionAttackWindup { idx, token } => {
                self.resolve_controlled_champion_auto_attack_windup_event(idx, token)
            }
            EventType::ControlledChampionAttackHit {
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            } => self.resolve_controlled_champion_auto_attack_hit_event(
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            ),
            EventType::ControlledChampionOffensivePrimaryHit {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => self.resolve_controlled_champion_offensive_primary_hit_event(
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            ),
            EventType::ControlledChampionOffensiveSecondaryHit => {
                self.resolve_controlled_champion_offensive_secondary_hit_event()
            }
            EventType::ControlledChampionOffensiveUltimateHit => {
                self.resolve_controlled_champion_offensive_ultimate_hit_event()
            }
            EventType::ChampionScript(idx, script_event, epoch) => {
                self.resolve_enemy_champion_script_event(idx, script_event, epoch)
            }
        }
    }

    pub(crate) fn step(&mut self, ticks: usize) -> bool {
        for _ in 0..ticks.max(1) {
            if self.finished || self.time >= self.sim.max_time_seconds {
                self.finished = true;
                return false;
            }

            self.controlled_champion_current_tick_index =
                self.controlled_champion_current_tick_index.wrapping_add(1);
            let target_time = self.sim.max_time_seconds.min(self.time + self.tick_seconds);
            self.enqueue_controller_policy_action_request_for_tick();
            self.process_pending_controlled_champion_action_requests();
            self.maybe_cast_controlled_champion_abilities_and_defensives();

            while let Some(top) = self.event_queue.peek_next().cloned() {
                if top.time > target_time || self.finished {
                    break;
                }
                self.event_queue.pop_next();
                self.apply_hot_effects(top.time);
                self.apply_enemy_respawn_updates();
                self.process_event(&top);
                self.record_processed_event_for_determinism();
                let should_recur = match &top.kind {
                    EventType::ChampionScript(idx, _, epoch) => {
                        self.enemy_script_event_should_recur(*idx, *epoch)
                    }
                    _ => true,
                };
                if let Some(recurring) = top.recurring
                    && recurring > 0.0
                    && !self.finished
                    && should_recur
                {
                    self.event_queue.reschedule_recurring_event(
                        top.time,
                        top.priority,
                        top.recurring,
                        top.kind.clone(),
                    );
                }
                self.maybe_cast_controlled_champion_abilities_and_defensives();
            }

            self.apply_hot_effects(target_time);
            self.apply_enemy_respawn_updates();
            self.maybe_cast_controlled_champion_abilities_and_defensives();

            if self.health <= 0.0 && !self.finished {
                self.apply_revive_or_mark_controlled_champion_death();
            }
            self.record_tick_state_for_determinism();
            if self.finished {
                return false;
            }
        }
        true
    }
}

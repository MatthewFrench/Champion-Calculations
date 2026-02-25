use super::super::*;

fn mix_checksum(checksum: &mut u64, value: u64) {
    *checksum ^= value;
    *checksum = checksum.wrapping_mul(0x1000_0000_01B3);
    *checksum ^= *checksum >> 33;
}

fn mix_f64(checksum: &mut u64, value: f64) {
    mix_checksum(checksum, value.to_bits());
}

fn mix_usize(checksum: &mut u64, value: usize) {
    mix_checksum(checksum, value as u64);
}

fn mix_bool(checksum: &mut u64, value: bool) {
    mix_checksum(checksum, u64::from(value));
}

fn mix_str(checksum: &mut u64, value: &str) {
    mix_usize(checksum, value.len());
    for byte in value.bytes() {
        mix_checksum(checksum, byte as u64);
    }
}

fn status_kind_tag(kind: &StatusEffectKind) -> u64 {
    match kind {
        StatusEffectKind::Stun => 1,
        StatusEffectKind::Silence => 2,
        StatusEffectKind::Root => 3,
        StatusEffectKind::Slow => 4,
        StatusEffectKind::Untargetable => 5,
        StatusEffectKind::Stasis => 6,
        StatusEffectKind::Custom(_) => 7,
    }
}

fn status_persistence_tag(persistence: StatusPersistence) -> u64 {
    match persistence {
        StatusPersistence::Replace => 1,
        StatusPersistence::RefreshDuration => 2,
        StatusPersistence::StackRefreshDuration => 3,
        StatusPersistence::Independent => 4,
    }
}

fn world_actor_class_tag(class: WorldActorClass) -> u64 {
    match class {
        WorldActorClass::Champion => 1,
        WorldActorClass::Minion => 2,
        WorldActorClass::Monster => 3,
        WorldActorClass::Structure => 4,
    }
}

fn world_actor_allegiance_tag(allegiance: WorldActorAllegiance) -> u64 {
    match allegiance {
        WorldActorAllegiance::ControlledChampionTeam => 1,
        WorldActorAllegiance::OpponentTeam => 2,
        WorldActorAllegiance::NeutralWorld => 3,
    }
}

impl ControlledChampionCombatSimulation {
    pub(crate) fn enable_trace(&mut self) {
        self.trace_enabled = true;
        self.determinism_signature_enabled = true;
        self.trace_events.clear();
        self.trace_next_snapshot_at = 0.0;
        self.emit_trace_snapshots_due();
    }

    pub(crate) fn trace_events(&self) -> &[String] {
        &self.trace_events
    }

    pub(crate) fn controlled_champion_rune_proc_telemetry(
        &self,
    ) -> Vec<ChampionRuneProcTelemetryEntry> {
        describe_rune_proc_telemetry(&self.controlled_champion_combat_runtime)
    }

    pub(crate) fn deterministic_replay_signature(&self) -> SimulationDeterminismSignature {
        let final_state_checksum = self.deterministic_state_checksum();
        let queue_checksum = self.event_queue.deterministic_replay_checksum();
        let tick_state_checksum =
            if !self.determinism_signature_enabled || self.deterministic_tick_count == 0 {
                final_state_checksum
            } else {
                self.deterministic_tick_state_checksum
            };
        SimulationDeterminismSignature {
            final_state_checksum,
            tick_state_checksum,
            queue_checksum,
            ticks_executed: self.deterministic_tick_count,
            events_processed: self.deterministic_event_count,
        }
    }

    pub(in crate::engine) fn record_processed_event_for_determinism(&mut self) {
        self.deterministic_event_count = self.deterministic_event_count.wrapping_add(1);
    }

    pub(in crate::engine) fn record_tick_state_for_determinism(&mut self) {
        self.deterministic_tick_count = self.deterministic_tick_count.wrapping_add(1);
        if !self.determinism_signature_enabled {
            return;
        }
        let tick_state_checksum = self.deterministic_state_checksum();
        mix_checksum(
            &mut self.deterministic_tick_state_checksum,
            tick_state_checksum,
        );
    }

    pub(in crate::engine) fn deterministic_state_checksum(&self) -> u64 {
        let mut checksum = DETERMINISTIC_CHECKSUM_OFFSET_BASIS;

        mix_f64(&mut checksum, self.time);
        mix_f64(&mut checksum, self.tick_seconds);
        mix_bool(&mut checksum, self.finished);
        match self.death_time {
            Some(seconds) => {
                mix_checksum(&mut checksum, 1);
                mix_f64(&mut checksum, seconds);
            }
            None => mix_checksum(&mut checksum, 0),
        }
        mix_f64(&mut checksum, self.damage_dealt_total);
        mix_f64(&mut checksum, self.healing_done_total);
        mix_usize(&mut checksum, self.enemy_kills_total);
        mix_f64(&mut checksum, self.invulnerable_seconds_total);
        mix_f64(&mut checksum, self.health);
        mix_f64(&mut checksum, self.max_health);
        mix_f64(&mut checksum, self.target_position.x);
        mix_f64(&mut checksum, self.target_position.y);
        mix_checksum(&mut checksum, self.controlled_champion_attack_sequence);
        mix_checksum(&mut checksum, self.controlled_champion_current_tick_index);
        mix_checksum(
            &mut checksum,
            self.controlled_champion_request_fixed_tick_delay,
        );
        mix_str(&mut checksum, &self.controlled_champion_world_actor_id);
        mix_str(
            &mut checksum,
            &self.controlled_champion_controller_identity.controller_id,
        );
        mix_checksum(
            &mut checksum,
            match self.controlled_champion_controller_identity.controller_kind {
                ChampionControllerKind::HumanPlayer => 1,
                ChampionControllerKind::ArtificialIntelligence => 2,
            },
        );
        mix_bool(&mut checksum, self.controlled_champion_manual_control_mode);
        mix_bool(&mut checksum, self.trace_enabled);
        mix_usize(&mut checksum, self.trace_events.len());
        mix_f64(&mut checksum, self.trace_next_snapshot_at);
        mix_f64(&mut checksum, self.trace_snapshot_interval_seconds);
        mix_checksum(&mut checksum, self.event_queue.queued_event_count() as u64);
        mix_checksum(
            &mut checksum,
            self.event_queue.deterministic_replay_checksum(),
        );

        mix_usize(
            &mut checksum,
            self.controlled_champion_pending_action_requests.len(),
        );
        mix_usize(
            &mut checksum,
            self.controlled_champion_recent_action_status_reports.len(),
        );
        mix_checksum(
            &mut checksum,
            self.controlled_champion_next_action_request_sequence,
        );

        match self.controlled_champion_pending_move_target_position {
            Some(position) => {
                mix_checksum(&mut checksum, 1);
                mix_f64(&mut checksum, position.x);
                mix_f64(&mut checksum, position.y);
            }
            None => mix_checksum(&mut checksum, 0),
        }
        match &self.controlled_champion_basic_attack_target_actor_id {
            Some(actor_id) => {
                mix_checksum(&mut checksum, 1);
                mix_str(&mut checksum, actor_id);
            }
            None => mix_checksum(&mut checksum, 0),
        }

        mix_bool(&mut checksum, self.stasis_item_available);
        mix_bool(&mut checksum, self.revive_item_available);
        mix_bool(&mut checksum, self.emergency_shield_item_available);
        mix_f64(&mut checksum, self.stasis_item_ready_at);
        mix_f64(&mut checksum, self.revive_item_ready_at);
        mix_f64(&mut checksum, self.emergency_shield_item_ready_at);
        mix_f64(&mut checksum, self.pool_until);
        mix_f64(&mut checksum, self.pool_damage_until);
        mix_f64(&mut checksum, self.pool_next_damage_tick_at);
        mix_f64(&mut checksum, self.stasis_until);
        mix_f64(&mut checksum, self.revive_lockout_until);
        mix_f64(&mut checksum, self.stunned_until);
        mix_f64(&mut checksum, self.emergency_shield_amount);
        mix_f64(&mut checksum, self.emergency_heal_rate);
        mix_f64(&mut checksum, self.emergency_heal_until);

        let cast_lock_remaining = self.combat_primitives.cast_lock().remaining();
        mix_f64(&mut checksum, cast_lock_remaining.windup_seconds);
        mix_f64(&mut checksum, cast_lock_remaining.channel_seconds);
        mix_f64(&mut checksum, cast_lock_remaining.lockout_seconds);

        let status_effects = self.combat_primitives.status_effects().effects();
        mix_usize(&mut checksum, status_effects.len());
        for effect in status_effects {
            mix_checksum(&mut checksum, status_kind_tag(&effect.kind));
            if let StatusEffectKind::Custom(label) = &effect.kind {
                mix_str(&mut checksum, label);
            }
            match effect.duration {
                StatusDuration::Timed { remaining_seconds } => {
                    mix_checksum(&mut checksum, 1);
                    mix_f64(&mut checksum, remaining_seconds);
                }
                StatusDuration::Persistent => mix_checksum(&mut checksum, 2),
            }
            mix_checksum(&mut checksum, effect.stacks as u64);
            mix_checksum(&mut checksum, effect.max_stacks as u64);
            mix_checksum(&mut checksum, status_persistence_tag(effect.persistence));
        }

        let mut ability_ready_entries = self
            .controlled_champion_ability_ready_at
            .iter()
            .collect::<Vec<_>>();
        ability_ready_entries.sort_by(|left, right| left.0.cmp(right.0));
        mix_usize(&mut checksum, ability_ready_entries.len());
        for (ability_id, ready_at) in ability_ready_entries {
            mix_str(&mut checksum, ability_id);
            mix_f64(&mut checksum, *ready_at);
        }

        let mut manually_controlled_enemy_actor_ids = self
            .manually_controlled_enemy_actor_ids
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        manually_controlled_enemy_actor_ids.sort();
        mix_usize(&mut checksum, manually_controlled_enemy_actor_ids.len());
        for actor_id in manually_controlled_enemy_actor_ids {
            mix_str(&mut checksum, &actor_id);
        }

        let mut enemy_move_targets = self
            .enemy_pending_move_target_position_by_actor_id
            .iter()
            .collect::<Vec<_>>();
        enemy_move_targets.sort_by(|left, right| left.0.cmp(right.0));
        mix_usize(&mut checksum, enemy_move_targets.len());
        for (actor_id, position) in enemy_move_targets {
            mix_str(&mut checksum, actor_id);
            mix_f64(&mut checksum, position.x);
            mix_f64(&mut checksum, position.y);
        }

        let mut enemy_basic_attack_targets = self
            .enemy_basic_attack_target_actor_id_by_actor_id
            .iter()
            .collect::<Vec<_>>();
        enemy_basic_attack_targets.sort_by(|left, right| left.0.cmp(right.0));
        mix_usize(&mut checksum, enemy_basic_attack_targets.len());
        for (actor_id, target_actor_id) in enemy_basic_attack_targets {
            mix_str(&mut checksum, actor_id);
            mix_str(&mut checksum, target_actor_id);
        }

        mix_usize(&mut checksum, self.enemy_state.len());
        for enemy in &self.enemy_state {
            mix_str(&mut checksum, &enemy.enemy.id);
            mix_str(&mut checksum, &enemy.enemy.name);
            mix_f64(&mut checksum, enemy.position.x);
            mix_f64(&mut checksum, enemy.position.y);
            mix_f64(&mut checksum, enemy.spawn_position.x);
            mix_f64(&mut checksum, enemy.spawn_position.y);
            mix_f64(&mut checksum, enemy.health);
            mix_f64(&mut checksum, enemy.max_health);
            mix_f64(&mut checksum, enemy.move_speed);
            mix_f64(&mut checksum, enemy.base_attack_speed);
            mix_f64(&mut checksum, enemy.ability_haste);
            mix_f64(&mut checksum, enemy.physical_hit_damage);
            mix_f64(&mut checksum, enemy.ability_power);
            mix_f64(&mut checksum, enemy.armor);
            mix_f64(&mut checksum, enemy.magic_resist);
            mix_f64(&mut checksum, enemy.next_attack_bonus_physical);
            mix_f64(&mut checksum, enemy.next_attack_bonus_magic);
            mix_f64(&mut checksum, enemy.next_attack_bonus_true);
            mix_f64(&mut checksum, enemy.physical_multiplier);
            mix_f64(&mut checksum, enemy.magic_multiplier);
            mix_checksum(&mut checksum, enemy.script_epoch);
            mix_f64(&mut checksum, enemy.script_poll_interval_seconds);
            mix_checksum(&mut checksum, enemy.attack_sequence);
            mix_f64(&mut checksum, enemy.stunned_until);
            mix_f64(&mut checksum, enemy.untargetable_until);
            mix_f64(&mut checksum, enemy.stasis_until);
            mix_bool(&mut checksum, enemy.stasis_item_available);
            mix_f64(&mut checksum, enemy.stasis_item_ready_at);
            mix_bool(&mut checksum, enemy.emergency_shield_item_available);
            mix_f64(&mut checksum, enemy.emergency_shield_item_ready_at);
            mix_f64(&mut checksum, enemy.emergency_shield_amount);
            mix_f64(&mut checksum, enemy.emergency_heal_rate);
            mix_f64(&mut checksum, enemy.emergency_heal_until);
            mix_f64(&mut checksum, enemy.invulnerable_until);
            mix_f64(&mut checksum, enemy.hitbox_radius);
            mix_checksum(
                &mut checksum,
                match enemy.movement_mode {
                    OpponentMovementMode::HoldPosition => 1,
                    OpponentMovementMode::MaintainCombatRange => 2,
                },
            );
            match enemy.respawn_at {
                Some(respawn_seconds) => {
                    mix_checksum(&mut checksum, 1);
                    mix_f64(&mut checksum, respawn_seconds);
                }
                None => mix_checksum(&mut checksum, 0),
            }

            let mut script_ready_entries = enemy.script_event_ready_at.iter().collect::<Vec<_>>();
            script_ready_entries.sort_by(|left, right| {
                (*left.0 as usize).cmp(&(*right.0 as usize)).then_with(|| {
                    left.1
                        .partial_cmp(right.1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
            });
            mix_usize(&mut checksum, script_ready_entries.len());
            for (event, ready_at) in script_ready_entries {
                mix_checksum(&mut checksum, *event as u64);
                mix_f64(&mut checksum, *ready_at);
            }
        }

        let mut actor_snapshots = self.world_state.actor_snapshot_entries();
        actor_snapshots.sort_by(|left, right| left.0.cmp(&right.0));
        mix_usize(&mut checksum, actor_snapshots.len());
        for (actor_id, snapshot) in actor_snapshots {
            mix_str(&mut checksum, &actor_id);
            mix_checksum(&mut checksum, world_actor_class_tag(snapshot.actor_class));
            mix_checksum(
                &mut checksum,
                world_actor_allegiance_tag(snapshot.actor_allegiance),
            );
            mix_f64(&mut checksum, snapshot.position.x);
            mix_f64(&mut checksum, snapshot.position.y);
        }

        mix_usize(&mut checksum, self.projectile_block_zones.len());
        for zone in &self.projectile_block_zones {
            mix_f64(&mut checksum, zone.start.x);
            mix_f64(&mut checksum, zone.start.y);
            mix_f64(&mut checksum, zone.end.x);
            mix_f64(&mut checksum, zone.end.y);
            mix_f64(&mut checksum, zone.half_width);
            mix_f64(&mut checksum, zone.expires_at);
        }

        checksum
    }

    pub(in crate::engine) fn trace_event(&mut self, kind: &str, details: String) {
        if !self.trace_enabled {
            return;
        }
        self.trace_events
            .push(format!("{:.3}s [{}] {}", self.time, kind, details));
    }

    fn emit_trace_snapshot(&mut self, checkpoint_seconds: f64) {
        if !self.trace_enabled {
            return;
        }
        let snapshot = self.collect_state_snapshot_summary(checkpoint_seconds);
        self.trace_event("state_snapshot", snapshot);
    }

    pub(in crate::engine) fn emit_trace_snapshots_due(&mut self) {
        if !self.trace_enabled {
            return;
        }
        let interval = self.trace_snapshot_interval_seconds.max(0.1);
        while self.time + 1e-9 >= self.trace_next_snapshot_at {
            let checkpoint = self.trace_next_snapshot_at;
            self.emit_trace_snapshot(checkpoint);
            self.trace_next_snapshot_at += interval;
        }
    }
}

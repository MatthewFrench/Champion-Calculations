use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn maybe_cast_controlled_champion_abilities_and_defensives(&mut self) {
        if self.finished {
            return;
        }
        self.apply_enemy_respawn_updates();

        if self.controlled_champion_script_enabled() {
            let can_cast_now = self.can_cast();
            let offensive_ultimate_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_ultimate_ability_id)
                .is_some();
            let offensive_ultimate_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_ultimate_ability_id,
            );
            let offensive_ultimate_has_viable_targets = can_cast_now
                && offensive_ultimate_equipped
                && self.time >= offensive_ultimate_ready_at
                && self
                    .max_enemy_distance_in_controlled_champion_range(
                        self.cast_profile.offensive_ultimate_range,
                        self.cast_profile.offensive_ultimate_effect_hitbox_radius,
                    )
                    .is_some();

            let defensive_ability = decide_controlled_champion_defensive_ability_activations(
                self.controlled_champion_script.as_ref(),
                ControlledChampionDefensiveAbilityDecisionInput {
                    now_seconds: self.time,
                    can_cast: can_cast_now,
                    defensive_ability_two_ready_at: self.controlled_champion_ability_ready_at(
                        &self.cast_profile.defensive_ability_two_id,
                    ),
                    offensive_ultimate_ready_at,
                    offensive_ultimate_has_viable_targets,
                },
            );

            if defensive_ability.cast_defensive_ability_two {
                let defensive_ability_two_id = self.cast_profile.defensive_ability_two_id.clone();
                self.set_controlled_champion_ability_ready_at(
                    &defensive_ability_two_id,
                    self.time + self.pool_cooldown,
                );
                self.pool_until = self.time + self.pool_duration;
                self.apply_status_effect(StatusEffect::timed(
                    StatusEffectKind::Untargetable,
                    self.pool_duration,
                    1,
                    StatusPersistence::RefreshDuration,
                ));
                let cost = self.health
                    * self.controlled_champion_defensive_ability_two_cost_percent_current_health
                    * self.urf.health_cost_multiplier;
                self.health -= cost;

                let defensive_ability_two_config = ControlledChampionDefensiveAbilityTwoConfig {
                    cooldown_seconds: self.pool_cooldown,
                    duration_seconds: self.pool_duration,
                    effect_range: self.pool_effect_range,
                    damage_tick_interval_seconds: self.pool_damage_tick_interval_seconds,
                    cost_percent_current_health: self
                        .controlled_champion_defensive_ability_two_cost_percent_current_health,
                    damage_per_tick: self.controlled_champion_defensive_ability_two_damage_per_tick,
                    damage_per_tick_bonus_health_ratio: self
                        .controlled_champion_defensive_ability_two_damage_per_tick_bonus_health_ratio,
                    heal_ratio_of_damage: self
                        .controlled_champion_defensive_ability_two_heal_ratio_of_damage,
                };
                let pool_damage_per_tick = controlled_champion_defensive_ability_two_raw_damage(
                    self.controlled_champion_script.as_ref(),
                    defensive_ability_two_config,
                    &self.controlled_champion_stats,
                    &self.controlled_champion_base,
                );
                if self.pool_damage_tick_interval_seconds > 0.0 && self.pool_duration > 0.0 {
                    self.pool_damage_until = self.time + self.pool_duration;
                    self.pool_next_damage_tick_at =
                        self.time + self.pool_damage_tick_interval_seconds;
                } else {
                    self.pool_damage_until = self.time;
                    self.pool_next_damage_tick_at = f64::INFINITY;
                }
                self.controlled_champion_defensive_ability_two_damage_per_tick =
                    pool_damage_per_tick.max(0.0);
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} (untargetable {:.2}s, damage tick {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.defensive_ability_two_id,
                        self.pool_duration,
                        self.pool_damage_tick_interval_seconds
                    ),
                );

                if self.health <= 0.0 {
                    self.apply_revive_or_mark_controlled_champion_death();
                    return;
                }
            }

            // Script-owned cadence for controlled champion offensive spell scheduling.
            let can_cast = self.can_cast();
            let offensive_primary_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_primary_ability_id,
            );
            let offensive_primary_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_primary_ability_id)
                .is_some();
            let offensive_primary_target = if can_cast
                && offensive_primary_equipped
                && self.time >= offensive_primary_ready_at
            {
                self.first_active_enemy_in_controlled_champion_range(
                    self.cast_profile.offensive_primary_range,
                    self.cast_profile.offensive_primary_effect_hitbox_radius,
                )
                .map(|target_index| ControlledChampionTargetSnapshot {
                    target_index,
                    distance: self.distance_to_target(target_index),
                })
            } else {
                None
            };
            let offensive_secondary_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_secondary_ability_id,
            );
            let offensive_secondary_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_secondary_ability_id)
                .is_some();
            let offensive_secondary_max_distance = if can_cast
                && offensive_secondary_equipped
                && self.time >= offensive_secondary_ready_at
            {
                self.max_enemy_distance_in_controlled_champion_range(
                    self.cast_profile.offensive_secondary_range,
                    self.cast_profile.offensive_secondary_effect_hitbox_radius,
                )
            } else {
                None
            };
            let offensive_ultimate_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_ultimate_ability_id,
            );
            let offensive_ultimate_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_ultimate_ability_id)
                .is_some();
            let offensive_ultimate_max_distance = if can_cast
                && offensive_ultimate_equipped
                && self.time >= offensive_ultimate_ready_at
            {
                self.max_enemy_distance_in_controlled_champion_range(
                    self.cast_profile.offensive_ultimate_range,
                    self.cast_profile.offensive_ultimate_effect_hitbox_radius,
                )
            } else {
                None
            };
            let offensive = decide_controlled_champion_offensive_casts(
                self.controlled_champion_script.as_ref(),
                ControlledChampionOffensiveDecisionInput {
                    now_seconds: self.time,
                    can_cast,
                    offensive_primary_ready_at,
                    offensive_secondary_ready_at,
                    offensive_ultimate_ready_at,
                    cooldowns: self.offensive_cooldowns,
                    cast_profile: self.cast_profile.clone(),
                    offensive_primary_target,
                    offensive_secondary_max_distance,
                    offensive_ultimate_max_distance,
                },
            );

            if let Some(offensive_primary) = offensive.offensive_primary
                && let (Some(target_at_cast), Some(target_name)) = (
                    self.enemy_position(offensive_primary.target_index),
                    self.enemy_name(offensive_primary.target_index),
                )
            {
                self.set_controlled_champion_ability_ready_at(
                    &offensive_primary.ability_id,
                    offensive_primary.next_ready_at,
                );
                self.begin_cast_lock_window(
                    self.cast_profile.offensive_primary_windup_seconds,
                    0.0,
                    0.0,
                );
                self.schedule_event(
                    offensive_primary.impact_delay_seconds,
                    50,
                    EventType::ControlledChampionOffensivePrimaryHit {
                        idx: offensive_primary.target_index,
                        source: self.target_position,
                        target_at_cast,
                        projectile_speed: self.cast_profile.offensive_primary_projectile_speed,
                        effect_hitbox_radius: self
                            .cast_profile
                            .offensive_primary_effect_hitbox_radius,
                    },
                    None,
                );
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} on {} (impact in {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_primary_ability_id,
                        target_name,
                        offensive_primary.impact_delay_seconds
                    ),
                );
            }
            if let Some(offensive_secondary) = offensive.offensive_secondary {
                self.set_controlled_champion_ability_ready_at(
                    &offensive_secondary.ability_id,
                    offensive_secondary.next_ready_at,
                );
                self.begin_cast_lock_window(
                    self.cast_profile.offensive_secondary_windup_seconds,
                    0.0,
                    0.0,
                );
                self.schedule_event(
                    offensive_secondary.impact_delay_seconds,
                    49,
                    EventType::ControlledChampionOffensiveSecondaryHit,
                    None,
                );
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} (impact in {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_secondary_ability_id,
                        offensive_secondary.impact_delay_seconds
                    ),
                );
            }
            if let Some(offensive_ultimate) = offensive.offensive_ultimate {
                self.set_controlled_champion_ability_ready_at(
                    &offensive_ultimate.ability_id,
                    offensive_ultimate.next_ready_at,
                );
                self.begin_cast_lock_window(
                    self.cast_profile.offensive_ultimate_windup_seconds,
                    0.0,
                    0.0,
                );
                self.schedule_event(
                    offensive_ultimate.impact_delay_seconds,
                    48,
                    EventType::ControlledChampionOffensiveUltimateHit,
                    None,
                );
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} (impact in {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_ultimate_ability_id,
                        offensive_ultimate.impact_delay_seconds
                    ),
                );
            }
        }

        let defensive_items = decide_defensive_item_activations(DefensiveItemActivationInput {
            now_seconds: self.time,
            can_cast: self.can_cast(),
            health: self.health,
            max_health: self.max_health,
            stasis_available: self.stasis_item_available,
            stasis_ready_at: self.stasis_item_ready_at,
            stasis_trigger_health_percent: self.sim.zhonya_trigger_health_percent,
            untargetable_active_until: self.pool_until,
            revive_lock_active_until: self.revive_lockout_until,
            emergency_shield_available: self.emergency_shield_item_available,
            emergency_shield_ready_at: self.emergency_shield_item_ready_at,
            emergency_shield_trigger_health_percent: self.sim.protoplasm_trigger_health_percent,
        });

        if defensive_items.activate_stasis {
            self.stasis_item_ready_at = self.time + self.stasis_item_cooldown_seconds;
            self.stasis_until = self.time + self.sim.zhonya_duration_seconds;
            self.apply_status_effect(StatusEffect::timed(
                StatusEffectKind::Stasis,
                self.sim.zhonya_duration_seconds,
                1,
                StatusPersistence::RefreshDuration,
            ));
            self.trace_event(
                "controlled_champion_item_active",
                format!(
                    "{} activated stasis item for {:.2}s",
                    self.controlled_champion_name, self.sim.zhonya_duration_seconds
                ),
            );
        }

        if defensive_items.activate_emergency_shield {
            self.emergency_shield_item_ready_at =
                self.time + self.emergency_shield_item_cooldown_seconds;
            self.emergency_shield_amount += self.sim.protoplasm_bonus_health;
            self.emergency_heal_rate =
                self.sim.protoplasm_heal_total / self.sim.protoplasm_duration_seconds.max(0.001);
            self.emergency_heal_until = self.time + self.sim.protoplasm_duration_seconds;
            self.trace_event(
                "controlled_champion_item_active",
                format!(
                    "{} activated emergency shield ({:.1} shield, {:.1}s heal window)",
                    self.controlled_champion_name,
                    self.sim.protoplasm_bonus_health,
                    self.sim.protoplasm_duration_seconds
                ),
            );
        }
    }
}

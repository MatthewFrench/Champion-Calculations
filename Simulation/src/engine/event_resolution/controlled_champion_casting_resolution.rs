use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn maybe_cast_controlled_champion_abilities_and_defensives(&mut self) {
        if self.finished {
            return;
        }
        self.apply_enemy_respawn_updates();
        if self.controlled_champion_manual_control_mode_enabled() {
            return;
        }

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
                if self
                    .activate_controlled_champion_defensive_ability_two(&defensive_ability_two_id)
                    && self.finished
                {
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

            if let Some(offensive_primary) = offensive.offensive_primary {
                let _ = self.schedule_controlled_champion_offensive_primary_cast(
                    &offensive_primary.ability_id,
                    offensive_primary.target_index,
                    offensive_primary.impact_delay_seconds,
                    offensive_primary.next_ready_at,
                );
            }
            if let Some(offensive_secondary) = offensive.offensive_secondary {
                let _ = self.schedule_controlled_champion_offensive_secondary_cast(
                    &offensive_secondary.ability_id,
                    offensive_secondary.impact_delay_seconds,
                    offensive_secondary.next_ready_at,
                );
            }
            if let Some(offensive_ultimate) = offensive.offensive_ultimate {
                let _ = self.schedule_controlled_champion_offensive_ultimate_cast(
                    &offensive_ultimate.ability_id,
                    offensive_ultimate.impact_delay_seconds,
                    offensive_ultimate.next_ready_at,
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
            let _ = self.activate_controlled_champion_stasis_item_active(
                super::super::controlled_champion_controller_channels::STASIS_ITEM_ACTIVE_ID,
            );
        }

        if defensive_items.activate_emergency_shield {
            let _ = self.activate_controlled_champion_emergency_shield_item_active(
                super::super::controlled_champion_controller_channels::EMERGENCY_SHIELD_ITEM_ACTIVE_ID,
            );
        }
    }
}

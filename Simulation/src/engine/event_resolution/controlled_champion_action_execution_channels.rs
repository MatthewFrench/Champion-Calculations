use super::super::*;

impl ControlledChampionCombatSimulation {
    // Shared controlled-champion ability execution channel. Script cadence and controller harness
    // requests both route through these methods to keep cast semantics consistent.
    pub(in crate::engine) fn activate_controlled_champion_defensive_ability_two(
        &mut self,
        ability_id: &str,
    ) -> bool {
        if !self.can_cast() {
            return false;
        }
        self.set_controlled_champion_ability_ready_at(ability_id, self.time + self.pool_cooldown);
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
            self.pool_next_damage_tick_at = self.time + self.pool_damage_tick_interval_seconds;
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
                ability_id,
                self.pool_duration,
                self.pool_damage_tick_interval_seconds
            ),
        );

        if self.health <= 0.0 {
            self.apply_revive_or_mark_controlled_champion_death();
        }
        true
    }

    pub(in crate::engine) fn schedule_controlled_champion_offensive_primary_cast(
        &mut self,
        ability_id: &str,
        target_index: usize,
        impact_delay_seconds: f64,
        next_ready_at: f64,
    ) -> bool {
        if !self.can_cast() {
            return false;
        }
        let (Some(target_at_cast), Some(target_name)) = (
            self.enemy_position(target_index),
            self.enemy_name(target_index),
        ) else {
            return false;
        };
        self.set_controlled_champion_ability_ready_at(ability_id, next_ready_at);
        self.begin_cast_lock_window(self.cast_profile.offensive_primary_windup_seconds, 0.0, 0.0);
        self.schedule_event(
            impact_delay_seconds,
            50,
            EventType::ControlledChampionOffensivePrimaryHit {
                idx: target_index,
                source: self.target_position,
                target_at_cast,
                projectile_speed: self.cast_profile.offensive_primary_projectile_speed,
                effect_hitbox_radius: self.cast_profile.offensive_primary_effect_hitbox_radius,
            },
            None,
        );
        self.trace_event(
            "controlled_champion_cast",
            format!(
                "{} cast {} on {} (impact in {:.2}s)",
                self.controlled_champion_name, ability_id, target_name, impact_delay_seconds
            ),
        );
        true
    }

    pub(in crate::engine) fn schedule_controlled_champion_offensive_secondary_cast(
        &mut self,
        ability_id: &str,
        impact_delay_seconds: f64,
        next_ready_at: f64,
    ) -> bool {
        if !self.can_cast() {
            return false;
        }
        self.set_controlled_champion_ability_ready_at(ability_id, next_ready_at);
        self.begin_cast_lock_window(
            self.cast_profile.offensive_secondary_windup_seconds,
            0.0,
            0.0,
        );
        self.schedule_event(
            impact_delay_seconds,
            49,
            EventType::ControlledChampionOffensiveSecondaryHit,
            None,
        );
        self.trace_event(
            "controlled_champion_cast",
            format!(
                "{} cast {} (impact in {:.2}s)",
                self.controlled_champion_name, ability_id, impact_delay_seconds
            ),
        );
        true
    }

    pub(in crate::engine) fn schedule_controlled_champion_offensive_ultimate_cast(
        &mut self,
        ability_id: &str,
        impact_delay_seconds: f64,
        next_ready_at: f64,
    ) -> bool {
        if !self.can_cast() {
            return false;
        }
        self.set_controlled_champion_ability_ready_at(ability_id, next_ready_at);
        self.begin_cast_lock_window(
            self.cast_profile.offensive_ultimate_windup_seconds,
            0.0,
            0.0,
        );
        self.schedule_event(
            impact_delay_seconds,
            48,
            EventType::ControlledChampionOffensiveUltimateHit,
            None,
        );
        self.trace_event(
            "controlled_champion_cast",
            format!(
                "{} cast {} (impact in {:.2}s)",
                self.controlled_champion_name, ability_id, impact_delay_seconds
            ),
        );
        true
    }

    pub(in crate::engine) fn activate_controlled_champion_stasis_item_active(
        &mut self,
        item_active_id: &str,
    ) -> bool {
        if !self.stasis_item_available || !self.can_cast() || self.time < self.stasis_item_ready_at
        {
            return false;
        }
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
                "{} activated {} for {:.2}s",
                self.controlled_champion_name, item_active_id, self.sim.zhonya_duration_seconds
            ),
        );
        true
    }

    pub(in crate::engine) fn activate_controlled_champion_emergency_shield_item_active(
        &mut self,
        item_active_id: &str,
    ) -> bool {
        if !self.emergency_shield_item_available
            || !self.can_cast()
            || self.time < self.emergency_shield_item_ready_at
        {
            return false;
        }
        self.emergency_shield_item_ready_at =
            self.time + self.emergency_shield_item_cooldown_seconds;
        self.emergency_shield_amount += self.sim.protoplasm_bonus_health;
        self.emergency_heal_rate =
            self.sim.protoplasm_heal_total / self.sim.protoplasm_duration_seconds.max(0.001);
        self.emergency_heal_until = self.time + self.sim.protoplasm_duration_seconds;
        self.trace_event(
            "controlled_champion_item_active",
            format!(
                "{} activated {} ({:.1} shield, {:.1}s heal window)",
                self.controlled_champion_name,
                item_active_id,
                self.sim.protoplasm_bonus_health,
                self.sim.protoplasm_duration_seconds
            ),
        );
        true
    }
}

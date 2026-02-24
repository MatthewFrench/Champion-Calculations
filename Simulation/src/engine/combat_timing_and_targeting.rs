use super::*;

impl ControlledChampionCombatSimulation {
    pub(super) fn schedule_event(
        &mut self,
        delay: f64,
        priority: i32,
        kind: EventType,
        recurring: Option<f64>,
    ) {
        self.event_queue
            .enqueue_event(self.time, delay, priority, kind, recurring);
    }

    pub(crate) fn is_targetable(&self) -> bool {
        self.time >= self.pool_until
            && self.time >= self.stasis_until
            && self.time >= self.revive_lockout_until
    }

    pub(super) fn controlled_champion_is_stunned(&self) -> bool {
        self.time < self.stunned_until
            || self
                .combat_primitives
                .status_effects()
                .is_active(&StatusEffectKind::Stun)
    }

    pub(super) fn controlled_champion_is_invulnerable_or_untargetable(&self) -> bool {
        !self.is_targetable()
            || self
                .combat_primitives
                .status_effects()
                .is_active(&StatusEffectKind::Stasis)
            || self
                .combat_primitives
                .status_effects()
                .is_active(&StatusEffectKind::Untargetable)
    }

    pub(crate) fn can_cast(&self) -> bool {
        !self.controlled_champion_is_stunned()
            && !self.controlled_champion_is_invulnerable_or_untargetable()
            && !self.combat_primitives.cast_lock().is_locked()
    }

    pub(super) fn can_basic_attack(&self) -> bool {
        !self.controlled_champion_is_stunned()
            && !self.controlled_champion_is_invulnerable_or_untargetable()
            && !self.combat_primitives.cast_lock().is_locked()
    }

    pub(super) fn controlled_champion_script_enabled(&self) -> bool {
        controlled_champion_script_enabled(self.controlled_champion_script.as_ref())
    }

    pub(super) fn controlled_champion_ability_ready_at(&self, ability_id: &str) -> f64 {
        self.controlled_champion_ability_ready_at
            .get(ability_id)
            .copied()
            .unwrap_or(0.0)
    }

    pub(super) fn set_controlled_champion_ability_ready_at(
        &mut self,
        ability_id: &str,
        ready_at: f64,
    ) {
        self.controlled_champion_ability_ready_at
            .insert(ability_id.to_string(), ready_at);
    }

    pub(super) fn apply_status_effect(&mut self, effect: StatusEffect) {
        self.combat_primitives.apply_status(effect);
    }

    pub(super) fn apply_stun_window(&mut self, duration_seconds: f64) {
        if duration_seconds <= 0.0 {
            return;
        }
        self.apply_status_effect(StatusEffect::timed(
            StatusEffectKind::Stun,
            duration_seconds,
            1,
            StatusPersistence::RefreshDuration,
        ));
    }

    pub(super) fn begin_cast_lock_window(
        &mut self,
        windup_seconds: f64,
        channel_seconds: f64,
        lockout: f64,
    ) {
        self.combat_primitives.begin_cast_lock(CastLockWindow::new(
            windup_seconds,
            channel_seconds,
            lockout,
        ));
    }

    pub(super) fn enemy_respawn_delay_seconds(&self, enemy_level: usize) -> f64 {
        respawn::urf_respawn_delay_seconds(
            enemy_level,
            self.time,
            respawn::UrfRespawnTuning {
                urf_flat_reduction_seconds: self.sim.urf_respawn_flat_reduction_seconds,
                extrapolation_per_level: self.sim.urf_respawn_extrapolation_per_level,
                time_scaling_enabled: self.sim.urf_respawn_time_scaling_enabled,
                time_scaling_start_seconds: self.sim.urf_respawn_time_scaling_start_seconds,
                time_scaling_per_minute_seconds: self
                    .sim
                    .urf_respawn_time_scaling_per_minute_seconds,
                time_scaling_cap_seconds: self.sim.urf_respawn_time_scaling_cap_seconds,
            },
        )
    }

    pub(super) fn distance_to_target(&self, idx: usize) -> f64 {
        self.enemy_position(idx)
            .map(|enemy_position| enemy_position.distance_to(self.target_position))
            .unwrap_or(f64::INFINITY)
    }

    pub(super) fn enemy_in_attack_range(&self, idx: usize) -> bool {
        let Some(attack_range) = self.enemy_attack_range(idx) else {
            return false;
        };
        let Some(enemy_hitbox_radius) = self.enemy_hitbox_radius(idx) else {
            return false;
        };
        let Some(effect_hitbox_radius) = self.enemy_attack_effect_hitbox_radius(idx) else {
            return false;
        };
        within_reach_with_hitboxes(
            self.distance_to_target(idx),
            attack_range,
            enemy_hitbox_radius,
            self.controlled_champion_hitbox_radius,
            effect_hitbox_radius,
        )
    }

    pub(super) fn enemy_in_controlled_champion_range(
        &self,
        idx: usize,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> bool {
        let Some(enemy_hitbox_radius) = self.enemy_hitbox_radius(idx) else {
            return false;
        };
        within_reach_with_hitboxes(
            self.distance_to_target(idx),
            range,
            self.controlled_champion_hitbox_radius,
            enemy_hitbox_radius,
            effect_hitbox_radius,
        )
    }

    pub(super) fn enemy_projectile_delay_from_points(
        &self,
        source: Vec2,
        target: Vec2,
        speed: f64,
    ) -> f64 {
        projectile_travel_seconds(source.distance_to(target), speed)
    }

    pub(super) fn cleanup_expired_projectile_blocks(&mut self) {
        self.projectile_block_zones
            .retain(|zone| zone.expires_at > self.time);
    }

    pub(super) fn is_projectile_blocked(
        &self,
        source: Vec2,
        target: Vec2,
        projectile_radius: f64,
    ) -> bool {
        self.projectile_block_zones
            .iter()
            .filter(|zone| zone.expires_at > self.time)
            .any(|zone| {
                distance_segment_to_segment(source, target, zone.start, zone.end)
                    <= projectile_radius.max(0.0) + zone.half_width.max(0.0)
            })
    }

    pub(super) fn enemy_is_stunned(&self, idx: usize) -> bool {
        self.enemy_is_stunned_at(idx, self.time)
    }

    pub(super) fn enemy_is_invulnerable_or_untargetable(&self, idx: usize) -> bool {
        self.enemy_is_invulnerable_or_untargetable_at(idx, self.time)
    }

    pub(super) fn enemy_can_take_actions(&self, idx: usize) -> bool {
        self.enemy_is_active(idx)
            && !self.enemy_is_stunned(idx)
            && !self.enemy_is_invulnerable_or_untargetable(idx)
    }

    pub(super) fn first_active_enemy_in_controlled_champion_range(
        &self,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for idx in 0..self.enemy_count() {
            if !self.enemy_is_active(idx)
                || !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius)
            {
                continue;
            }
            let dist = self.distance_to_target(idx);
            match best {
                Some((_, best_dist)) if dist >= best_dist => {}
                _ => best = Some((idx, dist)),
            }
        }
        best.map(|(idx, _)| idx)
    }

    pub(super) fn max_enemy_distance_in_controlled_champion_range(
        &self,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> Option<f64> {
        let mut max_distance = None;
        for idx in 0..self.enemy_count() {
            if !self.enemy_is_active(idx)
                || !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius)
            {
                continue;
            }
            let distance = self.distance_to_target(idx);
            max_distance = Some(match max_distance {
                Some(current) => distance.max(current),
                None => distance,
            });
        }
        max_distance
    }

    pub(super) fn controlled_champion_attack_speed(&self) -> f64 {
        self.controlled_champion_base_attack_speed
            * attack_speed_multiplier(&self.controlled_champion_combat_runtime, self.time)
    }

    pub(super) fn controlled_champion_attack_interval_seconds(&self) -> f64 {
        1.0 / self
            .controlled_champion_attack_speed()
            .max(simulator_defaults().engine_defaults.minimum_attack_speed)
    }

    pub(super) fn controlled_champion_in_attack_range(&self, idx: usize) -> bool {
        let Some(enemy_hitbox_radius) = self.enemy_hitbox_radius(idx) else {
            return false;
        };
        within_reach_with_hitboxes(
            self.distance_to_target(idx),
            self.controlled_champion_behavior.attack_range,
            self.controlled_champion_hitbox_radius,
            enemy_hitbox_radius,
            self.controlled_champion_behavior
                .attack_effect_hitbox_radius,
        )
    }

    pub(super) fn first_active_enemy_in_controlled_champion_attack_range(&self) -> Option<usize> {
        if let Some(preferred_target_index) = self.controlled_champion_basic_attack_target_index()
            && self.controlled_champion_in_attack_range(preferred_target_index)
        {
            return Some(preferred_target_index);
        }

        let mut best: Option<(usize, f64)> = None;
        for idx in 0..self.enemy_count() {
            if !self.enemy_is_active(idx) || !self.controlled_champion_in_attack_range(idx) {
                continue;
            }
            let dist = self.distance_to_target(idx);
            match best {
                Some((_, best_dist)) if dist >= best_dist => {}
                _ => best = Some((idx, dist)),
            }
        }
        best.map(|(idx, _)| idx)
    }

    pub(super) fn schedule_next_controlled_champion_attack(&mut self) {
        self.schedule_event(
            self.controlled_champion_attack_interval_seconds(),
            31,
            EventType::ControlledChampionAttack,
            None,
        );
    }

    pub(super) fn schedule_next_attack(&mut self, idx: usize) {
        let Some(interval) = self.enemy_attack_interval_seconds(
            idx,
            self.time,
            simulator_defaults().engine_defaults.minimum_attack_speed,
        ) else {
            return;
        };
        self.schedule_event(interval, 30, EventType::Attack(idx), None);
    }

    pub(super) fn run_until_end(&mut self) -> CombatOutcome {
        while self.step(1) {}
        CombatOutcome {
            time_alive_seconds: self
                .death_time
                .unwrap_or(self.time.min(self.sim.max_time_seconds)),
            damage_dealt: self.damage_dealt_total,
            healing_done: self.healing_done_total,
            enemy_kills: self.enemy_kills_total,
            invulnerable_seconds: self.invulnerable_seconds_total,
        }
    }

    pub(crate) fn tick_seconds(&self) -> f64 {
        self.tick_seconds
    }

    pub(crate) fn current_time(&self) -> f64 {
        self.time
    }

    pub(crate) fn current_health(&self) -> f64 {
        self.health
    }
}

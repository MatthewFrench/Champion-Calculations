use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn resolve_enemy_auto_attack_start_event(&mut self, idx: usize) {
        let Some(enemy_actor_id) = self
            .enemy_state
            .get(idx)
            .map(|state| state.enemy.id.clone())
        else {
            return;
        };
        if self.enemy_actor_manual_control_mode_enabled(&enemy_actor_id)
            && self.enemy_basic_attack_target_actor_id(&enemy_actor_id)
                != Some(self.controlled_champion_world_actor_id.as_str())
        {
            self.schedule_next_attack(idx);
            return;
        }
        if !self.enemy_can_take_actions(idx) || !self.enemy_in_attack_range(idx) {
            self.schedule_next_attack(idx);
            return;
        }
        let Some(token) = self.begin_enemy_attack_sequence(idx) else {
            return;
        };
        let Some(enemy_name) = self.enemy_name(idx) else {
            self.schedule_next_attack(idx);
            return;
        };
        self.trace_event("attack_start", format!("{} begins auto attack", enemy_name));
        let Some(windup_seconds) = self.enemy_attack_windup_seconds(idx) else {
            self.schedule_next_attack(idx);
            return;
        };
        let windup = windup_seconds.max(0.0);
        self.schedule_event(windup, 35, EventType::AttackWindup { idx, token }, None);
    }

    pub(in crate::engine) fn resolve_enemy_auto_attack_windup_event(
        &mut self,
        idx: usize,
        token: u64,
    ) {
        if !self.enemy_is_active(idx)
            || !self.enemy_attack_sequence_matches(idx, token)
            || !self.enemy_in_attack_range(idx)
        {
            self.schedule_next_attack(idx);
            return;
        }
        if !self.enemy_can_take_actions(idx) {
            let enemy_name = self
                .enemy_name(idx)
                .unwrap_or_else(|| format!("enemy_{idx}"));
            self.trace_event(
                "attack_cancelled",
                format!(
                    "{} auto attack cancelled during windup by crowd control or invulnerability",
                    enemy_name
                ),
            );
            self.schedule_next_attack(idx);
            return;
        }
        let Some(source) = self.enemy_position(idx) else {
            self.schedule_next_attack(idx);
            return;
        };
        let target_at_release = self.target_position;
        let Some(projectile_speed) = self.enemy_attack_projectile_speed(idx) else {
            self.schedule_next_attack(idx);
            return;
        };
        let Some(effect_hitbox_radius) = self.enemy_attack_effect_hitbox_radius(idx) else {
            self.schedule_next_attack(idx);
            return;
        };
        let travel =
            self.enemy_projectile_delay_from_points(source, target_at_release, projectile_speed);
        self.schedule_event(
            travel,
            34,
            EventType::AttackHit {
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            },
            None,
        );
    }

    pub(in crate::engine) fn resolve_enemy_auto_attack_hit_event(
        &mut self,
        idx: usize,
        token: u64,
        source: Vec2,
        target_at_release: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    ) {
        if !self.enemy_is_active(idx) || !self.enemy_attack_sequence_matches(idx, token) {
            self.schedule_next_attack(idx);
            return;
        }
        if projectile_speed <= 0.0 && !self.enemy_can_take_actions(idx) {
            let enemy_name = self
                .enemy_name(idx)
                .unwrap_or_else(|| format!("enemy_{idx}"));
            self.trace_event(
                "attack_cancelled",
                format!(
                    "{} melee attack cancelled before impact by crowd control or invulnerability",
                    enemy_name
                ),
            );
            self.schedule_next_attack(idx);
            return;
        }
        let target_current = self.health.max(0.0);
        let target_max = self.max_health.max(1.0);
        let Some((physical, magic, true_damage)) =
            self.consume_enemy_attack_damage_with_on_hit(idx, target_current, target_max)
        else {
            self.schedule_next_attack(idx);
            return;
        };
        let enemy_name = self
            .enemy_name(idx)
            .unwrap_or_else(|| format!("enemy_{idx}"));
        let outcome = if projectile_speed > 0.0
            && self.is_projectile_blocked(source, target_at_release, effect_hitbox_radius)
        {
            IncomingImpactOutcome::ProjectileBlocked
        } else {
            let hit = if projectile_speed > 0.0 {
                path_hits_circle(
                    source,
                    target_at_release,
                    self.target_position,
                    self.controlled_champion_hitbox_radius,
                    effect_hitbox_radius,
                )
            } else {
                path_hits_circle(
                    source,
                    source,
                    self.target_position,
                    self.controlled_champion_hitbox_radius,
                    effect_hitbox_radius,
                )
            };
            if !hit {
                IncomingImpactOutcome::MissedHitbox
            } else {
                match self.apply_incoming_damage_to_controlled_champion(
                    DamageSourceContext {
                        champion_name: enemy_name.clone(),
                        ability_name: "Auto Attack".to_string(),
                    },
                    physical,
                    magic,
                    true_damage,
                ) {
                    DamageApplicationOutcome::Applied => IncomingImpactOutcome::Applied,
                    DamageApplicationOutcome::NullifiedUntargetable => {
                        IncomingImpactOutcome::NullifiedUntargetable
                    }
                    DamageApplicationOutcome::Ignored => {
                        IncomingImpactOutcome::IgnoredTargetUnavailable
                    }
                }
            }
        };
        match outcome {
            IncomingImpactOutcome::Applied => self.trace_event(
                "attack_hit",
                format!(
                    "{} hit {} (phys {:.1}, magic {:.1}, true {:.1})",
                    enemy_name, self.controlled_champion_name, physical, magic, true_damage
                ),
            ),
            IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                "projectile_blocked",
                format!(
                    "{} auto attack blocked by active projectile block zone",
                    enemy_name
                ),
            ),
            IncomingImpactOutcome::MissedHitbox => self.trace_event(
                "attack_missed",
                format!(
                    "{} auto attack missed {} ({})",
                    enemy_name,
                    self.controlled_champion_name,
                    hitbox_miss_reason(
                        source,
                        if projectile_speed > 0.0 {
                            target_at_release
                        } else {
                            source
                        },
                        self.target_position,
                        self.controlled_champion_hitbox_radius,
                        effect_hitbox_radius
                    )
                ),
            ),
            IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                "impact_nullified",
                format!(
                    "{} auto attack on {} was nullified by untargetable or stasis state",
                    enemy_name, self.controlled_champion_name
                ),
            ),
            IncomingImpactOutcome::IgnoredTargetUnavailable => self.trace_event(
                "impact_ignored",
                format!(
                    "{} auto attack skipped because {} is unavailable",
                    enemy_name, self.controlled_champion_name
                ),
            ),
        }
        self.schedule_next_attack(idx);
    }
}

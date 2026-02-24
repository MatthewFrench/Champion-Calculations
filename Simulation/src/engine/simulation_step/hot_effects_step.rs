use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn apply_hot_effects(&mut self, to_time: f64) {
        if to_time <= self.time {
            return;
        }
        let delta = to_time - self.time;
        if delta > 0.0 {
            let invulnerable_until = self
                .pool_until
                .max(self.stasis_until)
                .max(self.revive_lockout_until);
            let invulnerable_overlap = (to_time.min(invulnerable_until) - self.time).max(0.0);
            self.invulnerable_seconds_total += invulnerable_overlap;
        }
        if self.pool_damage_until > self.time
            && self.pool_damage_tick_interval_seconds > 0.0
            && self.pool_next_damage_tick_at.is_finite()
        {
            while self.pool_next_damage_tick_at <= to_time + 1e-9
                && self.pool_next_damage_tick_at <= self.pool_damage_until + 1e-9
            {
                let (dealt, hit_count) = self
                    .apply_incoming_magic_damage_to_enemies_in_controlled_champion_range(
                        self.controlled_champion_defensive_ability_two_damage_per_tick,
                        self.pool_effect_range,
                        0.0,
                    );
                self.damage_dealt_total += dealt.max(0.0);
                if dealt > 0.0 {
                    let resolved_heal = resolve_stat(
                        StatQuery::ScalarAmount {
                            base_amount: dealt
                                * self
                                    .controlled_champion_defensive_ability_two_heal_ratio_of_damage,
                            source: ScalarMetricSource::Healing,
                            clamp_min_zero: true,
                        },
                        self.controlled_champion_buffs,
                    );
                    self.apply_healing_to_controlled_champion(resolved_heal);
                }
                self.trace_event(
                    "controlled_champion_pool_tick",
                    format!(
                        "{} {} tick dealt {:.1} to {} enemies in range",
                        self.controlled_champion_name,
                        self.cast_profile.defensive_ability_two_id,
                        dealt,
                        hit_count
                    ),
                );
                self.pool_next_damage_tick_at += self.pool_damage_tick_interval_seconds;
            }
            if self.pool_next_damage_tick_at > self.pool_damage_until + 1e-9 {
                self.pool_next_damage_tick_at = f64::INFINITY;
            }
        }
        if self.emergency_heal_until > self.time {
            let active = delta.min(self.emergency_heal_until - self.time);
            let resolved_heal = resolve_stat(
                StatQuery::ScalarAmount {
                    base_amount: self.emergency_heal_rate * active,
                    source: ScalarMetricSource::Healing,
                    clamp_min_zero: true,
                },
                self.controlled_champion_buffs,
            );
            self.apply_healing_to_controlled_champion(resolved_heal);
        }
        let runtime_regen = tick_regen_heal(
            &self.controlled_champion_combat_runtime,
            self.health,
            self.max_health,
            delta,
        );
        if runtime_regen > 0.0 {
            let resolved_regen = resolve_stat(
                StatQuery::ScalarAmount {
                    base_amount: runtime_regen,
                    source: ScalarMetricSource::Healing,
                    clamp_min_zero: true,
                },
                self.controlled_champion_buffs,
            );
            self.apply_healing_to_controlled_champion(resolved_regen);
        }
        self.combat_primitives.tick(delta);
        self.apply_enemy_movement_step(delta);
        self.apply_enemy_regeneration_tick(delta);
        self.time = to_time;
        self.cleanup_expired_projectile_blocks();
        self.emit_trace_snapshots_due();
    }
}

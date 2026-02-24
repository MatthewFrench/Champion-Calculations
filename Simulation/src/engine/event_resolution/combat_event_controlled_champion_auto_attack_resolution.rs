use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn resolve_controlled_champion_auto_attack_start_event(&mut self) {
        if !self.can_basic_attack() {
            self.schedule_next_controlled_champion_attack();
            return;
        }
        let Some(idx) = self.first_active_enemy_in_controlled_champion_attack_range() else {
            self.schedule_next_controlled_champion_attack();
            return;
        };
        self.controlled_champion_attack_sequence =
            self.controlled_champion_attack_sequence.wrapping_add(1);
        let token = self.controlled_champion_attack_sequence;
        let Some(enemy_name) = self.enemy_name(idx) else {
            self.schedule_next_controlled_champion_attack();
            return;
        };
        self.trace_event(
            "controlled_champion_attack_start",
            format!(
                "{} begins auto attack on {}",
                self.controlled_champion_name, enemy_name
            ),
        );
        let windup = self
            .controlled_champion_behavior
            .attack_windup_seconds
            .max(0.0);
        self.schedule_event(
            windup,
            36,
            EventType::ControlledChampionAttackWindup { idx, token },
            None,
        );
    }

    pub(in crate::engine) fn resolve_controlled_champion_auto_attack_windup_event(
        &mut self,
        idx: usize,
        token: u64,
    ) {
        if token != self.controlled_champion_attack_sequence
            || !self.enemy_is_active(idx)
            || !self.controlled_champion_in_attack_range(idx)
        {
            self.schedule_next_controlled_champion_attack();
            return;
        }
        if !self.can_basic_attack() {
            self.trace_event(
                "controlled_champion_attack_cancelled",
                format!(
                    "{} auto attack cancelled during windup by crowd control, cast lock, or invulnerability",
                    self.controlled_champion_name
                ),
            );
            self.schedule_next_controlled_champion_attack();
            return;
        }
        let source = self.target_position;
        let Some(target_at_release) = self.enemy_position(idx) else {
            self.schedule_next_controlled_champion_attack();
            return;
        };
        let projectile_speed = self.controlled_champion_behavior.attack_projectile_speed;
        let effect_hitbox_radius = self
            .controlled_champion_behavior
            .attack_effect_hitbox_radius;
        let travel =
            self.enemy_projectile_delay_from_points(source, target_at_release, projectile_speed);
        self.schedule_event(
            travel,
            35,
            EventType::ControlledChampionAttackHit {
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

    pub(in crate::engine) fn resolve_controlled_champion_auto_attack_hit_event(
        &mut self,
        idx: usize,
        token: u64,
        source: Vec2,
        target_at_release: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    ) {
        if token != self.controlled_champion_attack_sequence || !self.enemy_is_active(idx) {
            self.schedule_next_controlled_champion_attack();
            return;
        }
        if projectile_speed <= 0.0 && !self.can_basic_attack() {
            self.trace_event(
                "controlled_champion_attack_cancelled",
                format!(
                    "{} melee auto attack cancelled before impact by crowd control, cast lock, or invulnerability",
                    self.controlled_champion_name
                ),
            );
            self.schedule_next_controlled_champion_attack();
            return;
        }
        let Some(enemy_name) = self.enemy_name(idx) else {
            self.schedule_next_controlled_champion_attack();
            return;
        };
        if projectile_speed > 0.0
            && self.is_projectile_blocked(source, target_at_release, effect_hitbox_radius)
        {
            self.trace_event(
                "projectile_blocked",
                format!(
                    "{} auto attack blocked by active projectile block zone",
                    self.controlled_champion_name
                ),
            );
            self.schedule_next_controlled_champion_attack();
            return;
        }

        let Some(enemy_position) = self.enemy_position(idx) else {
            self.schedule_next_controlled_champion_attack();
            return;
        };
        let Some(enemy_hitbox_radius) = self.enemy_hitbox_radius(idx) else {
            self.schedule_next_controlled_champion_attack();
            return;
        };
        let hit = if projectile_speed > 0.0 {
            path_hits_circle(
                source,
                target_at_release,
                enemy_position,
                enemy_hitbox_radius,
                effect_hitbox_radius,
            )
        } else {
            path_hits_circle(
                source,
                source,
                enemy_position,
                enemy_hitbox_radius,
                effect_hitbox_radius,
            )
        };
        if !hit {
            self.trace_event(
                "controlled_champion_attack_missed",
                format!(
                    "{} auto attack missed {} ({})",
                    self.controlled_champion_name,
                    enemy_name,
                    hitbox_miss_reason(
                        source,
                        if projectile_speed > 0.0 {
                            target_at_release
                        } else {
                            source
                        },
                        enemy_position,
                        enemy_hitbox_radius,
                        effect_hitbox_radius
                    )
                ),
            );
            self.schedule_next_controlled_champion_attack();
            return;
        }

        let (target_current_health, target_max_health) =
            self.enemy_target_health_snapshot_or_defaults(idx);
        let attack_damage = self.controlled_champion_base.base_attack_damage
            + self.controlled_champion_stats.attack_damage;
        let (extra_physical, extra_magic, extra_true) = on_hit_bonus_damage(
            self.controlled_champion_behavior,
            &mut self.controlled_champion_combat_runtime,
            attack_damage,
            self.controlled_champion_stats.ability_power,
            self.controlled_champion_stats.attack_damage.max(0.0),
            target_current_health,
            target_max_health,
            self.max_health,
            self.time,
            Some(idx),
            self.sim.champion_level,
        );
        let physical = attack_damage + extra_physical;
        let magic = extra_magic;
        let true_damage = extra_true;
        let dealt = self.apply_incoming_damage_to_enemy(idx, physical, magic, true_damage);
        self.damage_dealt_total += dealt.max(0.0);
        self.apply_healing_to_controlled_champion_from_outgoing_damage_runtime(dealt);
        self.trace_event(
            "controlled_champion_attack_hit",
            format!(
                "{} auto attacked {} (phys {:.1}, magic {:.1}, true {:.1}, dealt {:.1})",
                self.controlled_champion_name, enemy_name, physical, magic, true_damage, dealt
            ),
        );
        self.schedule_next_controlled_champion_attack();
    }
}

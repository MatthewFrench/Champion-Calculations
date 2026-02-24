use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn resolve_controlled_champion_offensive_primary_hit_event(
        &mut self,
        idx: usize,
        source: Vec2,
        target_at_cast: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    ) {
        if !self.controlled_champion_script_enabled() {
            return;
        }
        if idx >= self.enemy_count() || !self.enemy_is_active(idx) {
            return;
        }
        let enemy_name = self
            .enemy_name(idx)
            .expect("controlled champion offensive primary target index should be valid");
        if projectile_speed > 0.0
            && self.is_projectile_blocked(source, target_at_cast, effect_hitbox_radius)
        {
            self.trace_event(
                "projectile_blocked",
                format!(
                    "{} {} blocked by active projectile block zone",
                    self.controlled_champion_name, self.cast_profile.offensive_primary_ability_id
                ),
            );
            return;
        }
        let enemy_position = self
            .enemy_position(idx)
            .expect("controlled champion offensive primary target index should be valid");
        let enemy_hitbox_radius = self
            .enemy_hitbox_radius(idx)
            .expect("controlled champion offensive primary target index should be valid");
        let hit = if projectile_speed > 0.0 {
            path_hits_circle(
                source,
                target_at_cast,
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
                "controlled_champion_primary_miss",
                format!(
                    "{} {} missed {} ({})",
                    self.controlled_champion_name,
                    self.cast_profile.offensive_primary_ability_id,
                    enemy_name,
                    hitbox_miss_reason(
                        source,
                        if projectile_speed > 0.0 {
                            target_at_cast
                        } else {
                            source
                        },
                        enemy_position,
                        enemy_hitbox_radius,
                        effect_hitbox_radius
                    )
                ),
            );
            return;
        }
        let q_raw_damage = controlled_champion_offensive_raw_damage(
            self.controlled_champion_script.as_ref(),
            ControlledChampionOffensiveAbility::Primary,
            self.controlled_champion_stats.ability_power,
        );
        let q_ap_ratio = controlled_champion_offensive_ap_ratio(
            self.controlled_champion_script.as_ref(),
            ControlledChampionOffensiveAbility::Primary,
        );
        let generic_runtime_bonus = self.apply_incoming_ability_bonus_damage_to_enemy(
            idx,
            q_raw_damage,
            q_ap_ratio,
            self.sim.champion_level,
        );
        let dealt =
            self.apply_incoming_magic_damage_to_enemy(idx, q_raw_damage) + generic_runtime_bonus;
        self.damage_dealt_total += dealt.max(0.0);
        self.apply_healing_to_controlled_champion_from_outgoing_damage_runtime(dealt);
        if dealt > 0.0 {
            let script_heal_multiplier = controlled_champion_heal_multiplier();
            let resolved_heal = resolve_stat(
                StatQuery::ScalarAmount {
                    base_amount: dealt
                        * controlled_champion_offensive_primary_heal_ratio(
                            self.controlled_champion_script.as_ref(),
                        )
                        * script_heal_multiplier,
                    source: ScalarMetricSource::Healing,
                    clamp_min_zero: true,
                },
                self.controlled_champion_buffs,
            );
            self.apply_healing_to_controlled_champion(resolved_heal);
        }
        self.trace_event(
            "controlled_champion_primary_hit",
            format!(
                "{} {} hit {} for {:.1}",
                self.controlled_champion_name,
                self.cast_profile.offensive_primary_ability_id,
                enemy_name,
                dealt
            ),
        );
    }

    pub(in crate::engine) fn resolve_controlled_champion_offensive_secondary_hit_event(&mut self) {
        if !self.controlled_champion_script_enabled() {
            return;
        }
        let e_raw_damage = controlled_champion_offensive_raw_damage(
            self.controlled_champion_script.as_ref(),
            ControlledChampionOffensiveAbility::Secondary,
            self.controlled_champion_stats.ability_power,
        );
        let e_ap_ratio = controlled_champion_offensive_ap_ratio(
            self.controlled_champion_script.as_ref(),
            ControlledChampionOffensiveAbility::Secondary,
        );
        let (base_dealt, hit_count) = self
            .apply_incoming_magic_damage_to_enemies_in_controlled_champion_range(
                e_raw_damage,
                self.cast_profile.offensive_secondary_range,
                self.cast_profile.offensive_secondary_effect_hitbox_radius,
            );
        let (generic_runtime_bonus, _) = self
            .apply_incoming_ability_bonus_damage_to_enemies_in_controlled_champion_range(
                e_raw_damage,
                e_ap_ratio,
                self.sim.champion_level,
                self.cast_profile.offensive_secondary_range,
                self.cast_profile.offensive_secondary_effect_hitbox_radius,
            );
        let dealt = base_dealt + generic_runtime_bonus;
        self.damage_dealt_total += dealt.max(0.0);
        self.apply_healing_to_controlled_champion_from_outgoing_damage_runtime(dealt);
        self.trace_event(
            "controlled_champion_secondary_hit",
            format!(
                "{} {} dealt {:.1} to {} enemies in range",
                self.controlled_champion_name,
                self.cast_profile.offensive_secondary_ability_id,
                dealt,
                hit_count
            ),
        );
    }

    pub(in crate::engine) fn resolve_controlled_champion_offensive_ultimate_hit_event(&mut self) {
        if !self.controlled_champion_script_enabled() {
            return;
        }
        let r_raw_damage = controlled_champion_offensive_raw_damage(
            self.controlled_champion_script.as_ref(),
            ControlledChampionOffensiveAbility::Ultimate,
            self.controlled_champion_stats.ability_power,
        );
        let r_ap_ratio = controlled_champion_offensive_ap_ratio(
            self.controlled_champion_script.as_ref(),
            ControlledChampionOffensiveAbility::Ultimate,
        );
        let (base_dealt, hit_count) = self
            .apply_incoming_magic_damage_to_enemies_in_controlled_champion_range(
                r_raw_damage,
                self.cast_profile.offensive_ultimate_range,
                self.cast_profile.offensive_ultimate_effect_hitbox_radius,
            );
        let (generic_runtime_bonus, _) = self
            .apply_incoming_ability_bonus_damage_to_enemies_in_controlled_champion_range(
                r_raw_damage,
                r_ap_ratio,
                self.sim.champion_level,
                self.cast_profile.offensive_ultimate_range,
                self.cast_profile.offensive_ultimate_effect_hitbox_radius,
            );
        let dealt = base_dealt + generic_runtime_bonus;
        self.damage_dealt_total += dealt.max(0.0);
        self.apply_healing_to_controlled_champion_from_outgoing_damage_runtime(dealt);
        self.trace_event(
            "controlled_champion_ultimate_hit",
            format!(
                "{} {} dealt {:.1} to {} enemies in range",
                self.controlled_champion_name,
                self.cast_profile.offensive_ultimate_ability_id,
                dealt,
                hit_count
            ),
        );
    }
}

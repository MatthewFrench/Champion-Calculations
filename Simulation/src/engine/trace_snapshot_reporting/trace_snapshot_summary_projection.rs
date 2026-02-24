use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn enemy_next_attack_ready_at(&self, idx: usize) -> Option<f64> {
        self.event_queue.next_enemy_attack_ready_at(idx)
    }

    pub(in crate::engine) fn enemy_next_attack_impact_at(&self, idx: usize) -> Option<f64> {
        self.event_queue.next_enemy_attack_impact_at(idx)
    }

    pub(in crate::engine) fn controlled_champion_next_attack_ready_at(&self) -> Option<f64> {
        self.event_queue.next_controlled_champion_attack_ready_at()
    }

    pub(in crate::engine) fn controlled_champion_next_attack_impact_at(
        &self,
    ) -> Option<(usize, f64)> {
        self.event_queue.next_controlled_champion_attack_impact_at()
    }

    pub(in crate::engine) fn collect_state_snapshot_summary(
        &self,
        checkpoint_seconds: f64,
    ) -> String {
        fn list_or_none(values: &[String]) -> String {
            if values.is_empty() {
                "none".to_string()
            } else {
                values.join(", ")
            }
        }

        fn join_or_none(values: &[String], separator: &str) -> String {
            if values.is_empty() {
                "none".to_string()
            } else {
                values.join(separator)
            }
        }

        let health_ratio = if self.max_health > 0.0 {
            (self.health / self.max_health).clamp(0.0, 1.0) * 100.0
        } else {
            0.0
        };

        let mut controlled_champion_cooldowns = Vec::new();
        if self.stasis_item_available {
            controlled_champion_cooldowns.push(format!(
                "Stasis item {}",
                Self::trace_cooldown_status(self.time, self.stasis_item_ready_at)
            ));
        }
        if self.revive_item_available {
            controlled_champion_cooldowns.push(format!(
                "Revive item {}",
                Self::trace_cooldown_status(self.time, self.revive_item_ready_at)
            ));
        }
        if self.emergency_shield_item_available {
            controlled_champion_cooldowns.push(format!(
                "Emergency shield item {}",
                Self::trace_cooldown_status(self.time, self.emergency_shield_item_ready_at)
            ));
        }
        let runtime_controlled_champion_cooldowns =
            describe_controlled_champion_runtime_cooldowns(self.time);
        let runtime_cooldowns_are_none = runtime_controlled_champion_cooldowns.len() == 1
            && runtime_controlled_champion_cooldowns[0] == "none";
        if !runtime_cooldowns_are_none {
            controlled_champion_cooldowns.extend(runtime_controlled_champion_cooldowns);
        }
        if controlled_champion_cooldowns.is_empty() {
            controlled_champion_cooldowns.push("none".to_string());
        }

        let mut controlled_champion_abilities = self
            .controlled_champion_ability_loadout
            .slot_bindings()
            .into_iter()
            .map(|(slot, ability_id)| {
                format!(
                    "{}:{} {}",
                    slot.label(),
                    ability_id,
                    Self::trace_cooldown_status(
                        self.time,
                        self.controlled_champion_ability_ready_at(ability_id)
                    )
                )
            })
            .collect::<Vec<_>>();
        if let Some((idx, impact_at)) = self.controlled_champion_next_attack_impact_at() {
            let target_name = self.enemy_name(idx).unwrap_or_else(|| "target".to_string());
            controlled_champion_abilities.push(format!(
                "Auto Attack in-flight -> {} ({:.2}s to impact)",
                target_name,
                (impact_at - self.time).max(0.0)
            ));
        } else if let Some(next_attack_ready_at) = self.controlled_champion_next_attack_ready_at() {
            controlled_champion_abilities.push(format!(
                "Auto Attack {}",
                Self::trace_cooldown_status(self.time, next_attack_ready_at)
            ));
        } else {
            controlled_champion_abilities.push("Auto Attack unavailable".to_string());
        }
        let controlled_runtime_effect_cooldowns =
            describe_runtime_effect_cooldowns(&self.controlled_champion_combat_runtime, self.time);
        let controlled_runtime_effect_stacks =
            describe_runtime_effect_stacks(&self.controlled_champion_combat_runtime);
        let controlled_champion_buffs = self.controlled_champion_status_lines();

        let mut lines = Vec::new();
        lines.push(format!(
            "checkpoint {:.1}s (captured_at {:.3}s)",
            checkpoint_seconds, self.time
        ));
        lines.push("controlled_champion:".to_string());
        lines.push(format!("  identity: {}", self.controlled_champion_name));
        lines.push(format!(
            "  core: pos=({:.1}, {:.1}) hp={:.1}/{:.1} ({:.1}%) armor={:.1} mr={:.1}",
            self.target_position.x,
            self.target_position.y,
            self.health.max(0.0),
            self.max_health,
            health_ratio,
            self.controlled_champion_stats.armor,
            self.controlled_champion_stats.magic_resist
        ));
        lines.push(format!(
            "  offense: ap={:.1} ah={:.1}",
            self.controlled_champion_stats.ability_power,
            self.controlled_champion_buffs.ability_haste
        ));
        lines.push(format!(
            "  loadout: items [{}] | runes [{}] | shards [{}]",
            list_or_none(&self.controlled_champion_item_names),
            list_or_none(&self.controlled_champion_rune_names),
            list_or_none(&self.controlled_champion_shard_names)
        ));
        lines.push(format!(
            "  cooldowns: {}",
            join_or_none(&controlled_champion_cooldowns, "; ")
        ));
        lines.push(format!(
            "  abilities: {}",
            join_or_none(&controlled_champion_abilities, "; ")
        ));
        lines.push(format!(
            "  runtime: cooldowns [{}] | stacks [{}]",
            join_or_none(&controlled_runtime_effect_cooldowns, "; "),
            join_or_none(&controlled_runtime_effect_stacks, "; ")
        ));
        lines.push(format!(
            "  buffs: {}",
            join_or_none(&controlled_champion_buffs, "; ")
        ));

        let enemy_count = self.enemy_count();
        if enemy_count == 0 {
            lines.push("enemies: none".to_string());
        } else {
            lines.push("enemies:".to_string());
            for idx in 0..enemy_count {
                let Some(snapshot) = self.enemy_trace_snapshot_at(idx, self.time) else {
                    continue;
                };

                let mut enemy_abilities = Vec::new();
                if let Some(impact_at) = self.enemy_next_attack_impact_at(idx) {
                    enemy_abilities.push(format!(
                        "Auto Attack in-flight ({:.2}s to impact)",
                        (impact_at - self.time).max(0.0)
                    ));
                } else if let Some(next_attack_ready_at) = self.enemy_next_attack_ready_at(idx) {
                    enemy_abilities.push(format!(
                        "Auto Attack {}",
                        Self::trace_cooldown_status(self.time, next_attack_ready_at)
                    ));
                } else {
                    enemy_abilities.push("Auto Attack unavailable".to_string());
                }
                enemy_abilities.extend(snapshot.scripted_ability_cooldowns);

                lines.push(format!("  {}:", snapshot.name));
                lines.push(format!(
                    "    core: pos=({:.1}, {:.1}) hp={:.1}/{:.1} armor={:.1} mr={:.1}",
                    snapshot.position.x,
                    snapshot.position.y,
                    snapshot.health,
                    snapshot.max_health,
                    snapshot.armor,
                    snapshot.magic_resist
                ));
                lines.push(format!(
                    "    combat: ad={:.1} ap={:.1} as={:.3} (interval {:.3}s) ah={:.1}",
                    snapshot.physical_hit_damage,
                    snapshot.ability_power,
                    snapshot.attack_speed,
                    snapshot.attack_interval_seconds,
                    snapshot.ability_haste
                ));
                lines.push(format!(
                    "    loadout: items [{}] | runes [{}]",
                    list_or_none(&snapshot.runtime_item_names),
                    list_or_none(&snapshot.runtime_rune_names)
                ));
                lines.push(format!(
                    "    abilities: {}",
                    join_or_none(&enemy_abilities, "; ")
                ));
                lines.push(format!(
                    "    runtime: cooldowns [{}] | stacks [{}]",
                    join_or_none(&snapshot.runtime_effect_cooldowns, "; "),
                    join_or_none(&snapshot.runtime_effect_stacks, "; ")
                ));
                lines.push(format!(
                    "    buffs: {}",
                    join_or_none(&snapshot.status_lines, "; ")
                ));
            }
        }

        lines.push("field:".to_string());
        let projectile_lines = self.queued_projectile_lines();
        if projectile_lines.len() == 1 && projectile_lines[0] == "none" {
            lines.push("  projectiles: none".to_string());
        } else {
            lines.push("  projectiles:".to_string());
            for projectile in projectile_lines {
                lines.push(format!("    - {projectile}"));
            }
        }
        if self.projectile_block_zones.is_empty() {
            lines.push("  projectile_block_zones: none".to_string());
        } else {
            lines.push("  projectile_block_zones:".to_string());
            for (idx, zone) in self.projectile_block_zones.iter().enumerate() {
                lines.push(format!(
                    "    - zone {}: start=({:.1}, {:.1}) end=({:.1}, {:.1}) width={:.1} expires_in={:.2}s",
                    idx + 1,
                    zone.start.x,
                    zone.start.y,
                    zone.end.x,
                    zone.end.y,
                    zone.half_width * 2.0,
                    (zone.expires_at - self.time).max(0.0)
                ));
            }
        }

        lines.join("\n")
    }
}

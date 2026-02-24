use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn trace_cooldown_status(now: f64, ready_at: f64) -> String {
        let remaining = (ready_at - now).max(0.0);
        if remaining <= 1e-9 {
            "ready".to_string()
        } else {
            format!("{remaining:.2}s")
        }
    }

    fn status_effect_kind_label(kind: &StatusEffectKind) -> String {
        match kind {
            StatusEffectKind::Stun => "Stun".to_string(),
            StatusEffectKind::Silence => "Silence".to_string(),
            StatusEffectKind::Root => "Root".to_string(),
            StatusEffectKind::Slow => "Slow".to_string(),
            StatusEffectKind::Untargetable => "Untargetable".to_string(),
            StatusEffectKind::Stasis => "Stasis".to_string(),
            StatusEffectKind::Custom(name) => (*name).to_string(),
        }
    }

    fn status_effect_summary(effect: &StatusEffect) -> String {
        let duration = match effect.duration {
            StatusDuration::Timed { remaining_seconds } => {
                let remaining = remaining_seconds.max(0.0);
                if remaining <= 1e-9 {
                    "expired".to_string()
                } else {
                    format!("{remaining:.2}s")
                }
            }
            StatusDuration::Persistent => "persistent".to_string(),
        };
        format!(
            "{} x{} ({})",
            Self::status_effect_kind_label(&effect.kind),
            effect.stacks,
            duration
        )
    }

    pub(in crate::engine) fn controlled_champion_status_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if self.time < self.stunned_until {
            lines.push(format!(
                "Stunned {:.2}s",
                (self.stunned_until - self.time).max(0.0)
            ));
        }
        if self.time < self.pool_until {
            lines.push(format!(
                "Pool untargetable {:.2}s",
                (self.pool_until - self.time).max(0.0)
            ));
        }
        if self.time < self.stasis_until {
            lines.push(format!(
                "Stasis {:.2}s",
                (self.stasis_until - self.time).max(0.0)
            ));
        }
        if self.time < self.revive_lockout_until {
            lines.push(format!(
                "Revive lockout {:.2}s",
                (self.revive_lockout_until - self.time).max(0.0)
            ));
        }
        if self.pool_damage_until > self.time {
            lines.push(format!(
                "Pool damage-over-time {:.2}s",
                (self.pool_damage_until - self.time).max(0.0)
            ));
        }
        if self.emergency_heal_until > self.time {
            lines.push(format!(
                "Emergency heal-over-time {:.2}s",
                (self.emergency_heal_until - self.time).max(0.0)
            ));
        }
        if self.emergency_shield_amount > 0.0 {
            lines.push(format!(
                "Emergency shield {:.1}",
                self.emergency_shield_amount
            ));
        }
        lines.extend(
            self.combat_primitives
                .status_effects()
                .effects()
                .iter()
                .map(Self::status_effect_summary),
        );
        if lines.is_empty() {
            lines.push("none".to_string());
        }
        lines
    }

    pub(in crate::engine) fn queued_projectile_lines(&self) -> Vec<String> {
        let mut entries = Vec::new();
        for projection in self
            .event_queue
            .queued_projectile_impact_projections(self.time)
        {
            match projection.kind {
                QueuedProjectileImpactKind::EnemyAttack { enemy_index } => {
                    if let Some(enemy_name) = self.enemy_name(enemy_index) {
                        entries.push(format!(
                            "{} Auto Attack -> {} (impact in {:.2}s)",
                            enemy_name,
                            self.controlled_champion_name,
                            (projection.time_seconds - self.time).max(0.0)
                        ));
                    }
                }
                QueuedProjectileImpactKind::ControlledChampionOffensivePrimary { enemy_index } => {
                    if let Some(enemy_name) = self.enemy_name(enemy_index) {
                        entries.push(format!(
                            "{} {} -> {} (impact in {:.2}s)",
                            self.controlled_champion_name,
                            self.cast_profile.offensive_primary_ability_id,
                            enemy_name,
                            (projection.time_seconds - self.time).max(0.0)
                        ));
                    }
                }
                QueuedProjectileImpactKind::ControlledChampionAttack { enemy_index } => {
                    if let Some(enemy_name) = self.enemy_name(enemy_index) {
                        entries.push(format!(
                            "{} Auto Attack -> {} (impact in {:.2}s)",
                            self.controlled_champion_name,
                            enemy_name,
                            (projection.time_seconds - self.time).max(0.0)
                        ));
                    }
                }
            }
        }
        if entries.is_empty() {
            return vec!["none".to_string()];
        }
        entries
    }
}

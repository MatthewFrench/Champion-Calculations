use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn apply_enemy_script_actions(
        &mut self,
        idx: usize,
        script_event: ChampionScriptEvent,
        epoch: u64,
        actions: Vec<ChampionScriptAction>,
    ) {
        for action in actions {
            match action {
                ChampionScriptAction::AddNextAttackBonusPhysical {
                    amount,
                    trace_message,
                } => {
                    if let Some(enemy_name) =
                        self.apply_enemy_next_attack_bonus_physical(idx, amount)
                    {
                        self.trace_event("enemy_buff", format!("{} {}", enemy_name, trace_message));
                    }
                }
                ChampionScriptAction::ApplyDamage {
                    source,
                    projectile_speed,
                    hitbox,
                    physical,
                    magic,
                    true_damage,
                    stun_duration,
                } => {
                    let source = vec2_from_champion_script_point(source);
                    let effect_hitbox_radius = hitbox.radius();
                    let enemy_name = self
                        .enemy_name(idx)
                        .expect("enemy script action index should be valid");
                    let outcome = if projectile_speed > 0.0
                        && self.is_projectile_blocked(
                            source,
                            self.target_position,
                            effect_hitbox_radius,
                        ) {
                        IncomingImpactOutcome::ProjectileBlocked
                    } else {
                        let hit = path_hits_circle(
                            source,
                            self.target_position,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        );
                        if !hit {
                            IncomingImpactOutcome::MissedHitbox
                        } else {
                            match self.apply_incoming_damage_to_controlled_champion(
                                DamageSourceContext {
                                    champion_name: enemy_name.clone(),
                                    ability_name: champion_script_event_label(script_event)
                                        .to_string(),
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
                    let mut aftershock_magic_damage = 0.0;
                    if stun_duration > 0.0 && outcome == IncomingImpactOutcome::Applied {
                        self.stunned_until = self.stunned_until.max(self.time + stun_duration);
                        self.apply_stun_window(stun_duration);
                        aftershock_magic_damage =
                            self.enemy_aftershock_magic_damage_on_immobilize(idx);
                    }
                    if aftershock_magic_damage > 0.0 {
                        match self.apply_incoming_damage_to_controlled_champion(
                            DamageSourceContext {
                                champion_name: enemy_name.clone(),
                                ability_name: "Aftershock Shockwave".to_string(),
                            },
                            0.0,
                            aftershock_magic_damage,
                            0.0,
                        ) {
                            DamageApplicationOutcome::Applied => {
                                self.trace_event(
                                    "aftershock_hit",
                                    format!(
                                        "{} Aftershock shockwave dealt {:.1} magic damage",
                                        enemy_name, aftershock_magic_damage
                                    ),
                                );
                            }
                            DamageApplicationOutcome::NullifiedUntargetable => {
                                self.trace_event(
                                    "impact_nullified",
                                    format!(
                                        "{} Aftershock shockwave on {} was nullified by untargetable or stasis state",
                                        enemy_name, self.controlled_champion_name
                                    ),
                                );
                            }
                            DamageApplicationOutcome::Ignored => {
                                self.trace_event(
                                    "impact_ignored",
                                    format!(
                                        "{} Aftershock shockwave skipped because {} is unavailable",
                                        enemy_name, self.controlled_champion_name
                                    ),
                                );
                            }
                        }
                    }
                    match outcome {
                        IncomingImpactOutcome::Applied => {}
                        IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                            "projectile_blocked",
                            format!(
                                "{} {} projectile blocked by active projectile block zone",
                                enemy_name,
                                champion_script_event_label(script_event)
                            ),
                        ),
                        IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                            "impact_nullified",
                            format!(
                                "{} {} on {} was nullified by untargetable or stasis state",
                                enemy_name,
                                champion_script_event_label(script_event),
                                self.controlled_champion_name
                            ),
                        ),
                        IncomingImpactOutcome::MissedHitbox => self.trace_event(
                            "impact_missed",
                            format!(
                                "{} {} missed {} ({})",
                                enemy_name,
                                champion_script_event_label(script_event),
                                self.controlled_champion_name,
                                hitbox_miss_reason(
                                    source,
                                    self.target_position,
                                    self.target_position,
                                    self.controlled_champion_hitbox_radius,
                                    effect_hitbox_radius
                                )
                            ),
                        ),
                        IncomingImpactOutcome::IgnoredTargetUnavailable => self.trace_event(
                            "impact_ignored",
                            format!(
                                "{} {} skipped because {} is unavailable",
                                enemy_name,
                                champion_script_event_label(script_event),
                                self.controlled_champion_name
                            ),
                        ),
                    }
                }
                ChampionScriptAction::ScheduleFollowup {
                    delay_seconds,
                    priority,
                    event,
                } => {
                    self.schedule_event(
                        delay_seconds,
                        priority,
                        EventType::ChampionScript(idx, event, epoch),
                        None,
                    );
                }
            }
        }
    }
}

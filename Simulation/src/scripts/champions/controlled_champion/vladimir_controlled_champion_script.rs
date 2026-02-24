use std::sync::Arc;

use crate::defaults::{
    vladimir_cast_profile_defaults, vladimir_defensive_ability_two_policy_defaults,
    vladimir_offensive_ability_defaults, vladimir_sanguine_pool_defaults,
};
use crate::scripts::champions::vladimir;
use crate::scripts::runtime::stat_resolution::{
    CooldownMetricSource, RuntimeBuffState, StatQuery, resolve_stat,
};
use crate::{ChampionBase, Stats};

use super::{
    ControlledChampionAbilityCooldowns, ControlledChampionAbilityTuning,
    ControlledChampionAreaCastDecision, ControlledChampionCastProfile,
    ControlledChampionDefensiveAbilityDecisions, ControlledChampionDefensiveAbilityTwoConfig,
    ControlledChampionOffensiveAbility, ControlledChampionOffensiveCastDecisions,
    ControlledChampionScriptCapability, ControlledChampionScriptHandle,
    ControlledChampionSingleTargetCastDecision, ControlledChampionTargetSnapshot,
};

#[derive(Debug, Clone)]
struct VladimirControlledChampionScript {
    cast_profile: ControlledChampionCastProfile,
    offensive_tuning: ControlledChampionAbilityTuning,
    defensive_ability_two_rank: usize,
    defensive_ability_two_duration_seconds: f64,
    defensive_ability_two_effect_range: f64,
    defensive_ability_two_damage_tick_interval_seconds: f64,
    defensive_ability_two_cost_percent_current_health: f64,
    defensive_ability_two_heal_ratio_of_damage: f64,
    defensive_ability_two_damage_per_tick_by_rank: Vec<f64>,
    defensive_ability_two_base_cooldown_seconds_by_rank: Vec<f64>,
    defensive_ability_two_damage_per_tick_bonus_health_ratio: f64,
    prioritize_offensive_ultimate_before_defensive_ability_two: bool,
}

fn to_vladimir_tuning(tuning: ControlledChampionAbilityTuning) -> vladimir::VladimirAbilityTuning {
    vladimir::VladimirAbilityTuning {
        q_base_damage: tuning.offensive_primary_base_damage,
        q_ap_ratio: tuning.offensive_primary_ap_ratio,
        q_base_cooldown_seconds: tuning.offensive_primary_base_cooldown_seconds,
        e_base_damage: tuning.offensive_secondary_base_damage,
        e_ap_ratio: tuning.offensive_secondary_ap_ratio,
        e_base_cooldown_seconds: tuning.offensive_secondary_base_cooldown_seconds,
        r_base_damage: tuning.offensive_ultimate_base_damage,
        r_ap_ratio: tuning.offensive_ultimate_ap_ratio,
        r_base_cooldown_seconds: tuning.offensive_ultimate_base_cooldown_seconds,
    }
}

fn from_vladimir_cast_profile(
    profile: vladimir::VladimirCastProfile,
) -> ControlledChampionCastProfile {
    ControlledChampionCastProfile {
        offensive_primary_ability_id: profile.q_ability_id,
        defensive_ability_two_id: profile.pool_ability_id,
        offensive_secondary_ability_id: profile.e_ability_id,
        offensive_ultimate_ability_id: profile.r_ability_id,
        offensive_primary_range: profile.q_range,
        offensive_primary_windup_seconds: profile.q_windup_seconds,
        offensive_primary_projectile_speed: profile.q_projectile_speed,
        offensive_primary_effect_hitbox_radius: profile.q_effect_hitbox_radius,
        offensive_secondary_range: profile.e_range,
        offensive_secondary_windup_seconds: profile.e_windup_seconds,
        offensive_secondary_projectile_speed: profile.e_projectile_speed,
        offensive_secondary_effect_hitbox_radius: profile.e_effect_hitbox_radius,
        offensive_ultimate_range: profile.r_range,
        offensive_ultimate_windup_seconds: profile.r_windup_seconds,
        offensive_ultimate_projectile_speed: profile.r_projectile_speed,
        offensive_ultimate_effect_hitbox_radius: profile.r_effect_hitbox_radius,
    }
}

fn to_vladimir_cast_profile(
    profile: ControlledChampionCastProfile,
) -> vladimir::VladimirCastProfile {
    vladimir::VladimirCastProfile {
        q_ability_id: profile.offensive_primary_ability_id,
        pool_ability_id: profile.defensive_ability_two_id,
        e_ability_id: profile.offensive_secondary_ability_id,
        r_ability_id: profile.offensive_ultimate_ability_id,
        q_range: profile.offensive_primary_range,
        q_windup_seconds: profile.offensive_primary_windup_seconds,
        q_projectile_speed: profile.offensive_primary_projectile_speed,
        q_effect_hitbox_radius: profile.offensive_primary_effect_hitbox_radius,
        e_range: profile.offensive_secondary_range,
        e_windup_seconds: profile.offensive_secondary_windup_seconds,
        e_projectile_speed: profile.offensive_secondary_projectile_speed,
        e_effect_hitbox_radius: profile.offensive_secondary_effect_hitbox_radius,
        r_range: profile.offensive_ultimate_range,
        r_windup_seconds: profile.offensive_ultimate_windup_seconds,
        r_projectile_speed: profile.offensive_ultimate_projectile_speed,
        r_effect_hitbox_radius: profile.offensive_ultimate_effect_hitbox_radius,
    }
}

fn resolve_ranked_value(values: &[f64], rank: usize, fallback: f64) -> f64 {
    values
        .get(rank.saturating_sub(1))
        .copied()
        .or_else(|| values.last().copied())
        .unwrap_or(fallback)
}

impl ControlledChampionScriptCapability for VladimirControlledChampionScript {
    fn default_cast_profile(&self) -> ControlledChampionCastProfile {
        self.cast_profile.clone()
    }

    fn offensive_cooldowns_after_haste(
        &self,
        ability_haste: f64,
    ) -> ControlledChampionAbilityCooldowns {
        let cooldowns = vladimir::offensive_cooldowns_after_haste(
            to_vladimir_tuning(self.offensive_tuning),
            ability_haste,
        );
        ControlledChampionAbilityCooldowns {
            offensive_primary_seconds: cooldowns.q_seconds,
            offensive_secondary_seconds: cooldowns.e_seconds,
            offensive_ultimate_seconds: cooldowns.r_seconds,
        }
    }

    fn defensive_ability_two_config(
        &self,
        ability_haste: f64,
    ) -> ControlledChampionDefensiveAbilityTwoConfig {
        let base_cooldown = resolve_ranked_value(
            &self.defensive_ability_two_base_cooldown_seconds_by_rank,
            self.defensive_ability_two_rank,
            16.0,
        );
        let cooldown_seconds = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: base_cooldown,
                source: CooldownMetricSource::Ability,
            },
            RuntimeBuffState {
                ability_haste,
                ..RuntimeBuffState::default()
            },
        );
        let base_damage = resolve_ranked_value(
            &self.defensive_ability_two_damage_per_tick_by_rank,
            self.defensive_ability_two_rank,
            0.0,
        );

        ControlledChampionDefensiveAbilityTwoConfig {
            cooldown_seconds,
            duration_seconds: self.defensive_ability_two_duration_seconds,
            effect_range: self.defensive_ability_two_effect_range,
            damage_tick_interval_seconds: self.defensive_ability_two_damage_tick_interval_seconds,
            cost_percent_current_health: self.defensive_ability_two_cost_percent_current_health,
            damage_per_tick: base_damage,
            damage_per_tick_bonus_health_ratio: self
                .defensive_ability_two_damage_per_tick_bonus_health_ratio,
            heal_ratio_of_damage: self.defensive_ability_two_heal_ratio_of_damage,
        }
    }

    fn decide_defensive_ability_activations(
        &self,
        input: super::ControlledChampionDefensiveAbilityDecisionInput,
    ) -> ControlledChampionDefensiveAbilityDecisions {
        let decisions = vladimir::decide_defensive_ability_activations(
            vladimir::VladimirDefensiveAbilityDecisionInput {
                now_seconds: input.now_seconds,
                can_cast: input.can_cast,
                pool_ready_at: input.defensive_ability_two_ready_at,
                prioritize_offensive_ultimate_before_pool: self
                    .prioritize_offensive_ultimate_before_defensive_ability_two,
                offensive_ultimate_ready_at: input.offensive_ultimate_ready_at,
                offensive_ultimate_has_viable_targets: input.offensive_ultimate_has_viable_targets,
            },
        );
        ControlledChampionDefensiveAbilityDecisions {
            cast_defensive_ability_two: decisions.cast_pool,
        }
    }

    fn decide_offensive_casts(
        &self,
        input: super::ControlledChampionOffensiveDecisionInput,
    ) -> ControlledChampionOffensiveCastDecisions {
        let cast_profile = to_vladimir_cast_profile(input.cast_profile);
        let decisions =
            vladimir::decide_offensive_casts(vladimir::VladimirOffensiveDecisionInput {
                now_seconds: input.now_seconds,
                can_cast: input.can_cast,
                q_ready_at: input.offensive_primary_ready_at,
                e_ready_at: input.offensive_secondary_ready_at,
                r_ready_at: input.offensive_ultimate_ready_at,
                cooldowns: vladimir::VladimirAbilityCooldowns {
                    q_seconds: input.cooldowns.offensive_primary_seconds,
                    e_seconds: input.cooldowns.offensive_secondary_seconds,
                    r_seconds: input.cooldowns.offensive_ultimate_seconds,
                },
                cast_profile,
                q_target: input.offensive_primary_target.map(
                    |target: ControlledChampionTargetSnapshot| vladimir::VladimirTargetSnapshot {
                        target_index: target.target_index,
                        distance: target.distance,
                    },
                ),
                e_max_distance: input.offensive_secondary_max_distance,
                r_max_distance: input.offensive_ultimate_max_distance,
            });

        ControlledChampionOffensiveCastDecisions {
            offensive_primary: decisions
                .q
                .map(|cast| ControlledChampionSingleTargetCastDecision {
                    ability_id: cast.ability_id,
                    target_index: cast.target_index,
                    impact_delay_seconds: cast.impact_delay_seconds,
                    next_ready_at: cast.next_ready_at,
                }),
            offensive_secondary: decisions.e.map(|cast| ControlledChampionAreaCastDecision {
                ability_id: cast.ability_id,
                impact_delay_seconds: cast.impact_delay_seconds,
                next_ready_at: cast.next_ready_at,
            }),
            offensive_ultimate: decisions.r.map(|cast| ControlledChampionAreaCastDecision {
                ability_id: cast.ability_id,
                impact_delay_seconds: cast.impact_delay_seconds,
                next_ready_at: cast.next_ready_at,
            }),
        }
    }

    fn offensive_raw_damage(
        &self,
        ability: ControlledChampionOffensiveAbility,
        ability_power: f64,
    ) -> f64 {
        let tuning = to_vladimir_tuning(self.offensive_tuning);
        match ability {
            ControlledChampionOffensiveAbility::Primary => {
                vladimir::q_damage_raw(tuning, ability_power)
            }
            ControlledChampionOffensiveAbility::Secondary => {
                vladimir::e_damage_raw(tuning, ability_power)
            }
            ControlledChampionOffensiveAbility::Ultimate => {
                vladimir::r_damage_raw(tuning, ability_power)
            }
        }
    }

    fn offensive_ap_ratio(&self, ability: ControlledChampionOffensiveAbility) -> f64 {
        match ability {
            ControlledChampionOffensiveAbility::Primary => {
                self.offensive_tuning.offensive_primary_ap_ratio
            }
            ControlledChampionOffensiveAbility::Secondary => {
                self.offensive_tuning.offensive_secondary_ap_ratio
            }
            ControlledChampionOffensiveAbility::Ultimate => {
                self.offensive_tuning.offensive_ultimate_ap_ratio
            }
        }
    }

    fn offensive_primary_heal_ratio(&self) -> f64 {
        self.offensive_tuning.offensive_primary_heal_ratio_of_damage
    }

    fn defensive_ability_two_raw_damage(
        &self,
        config: ControlledChampionDefensiveAbilityTwoConfig,
        controlled_champion_stats: &Stats,
        controlled_champion_base: &ChampionBase,
    ) -> f64 {
        config.damage_per_tick
            + config.damage_per_tick_bonus_health_ratio
                * (controlled_champion_stats.health - controlled_champion_base.base_health)
    }
}

pub(super) fn build_vladimir_script() -> ControlledChampionScriptHandle {
    let cast_profile = vladimir_cast_profile_defaults("vladimir")
        .map(|defaults| {
            from_vladimir_cast_profile(vladimir::VladimirCastProfile {
                q_ability_id: defaults.q_ability_id,
                pool_ability_id: defaults.pool_ability_id,
                e_ability_id: defaults.e_ability_id,
                r_ability_id: defaults.r_ability_id,
                q_range: defaults.q_range,
                q_windup_seconds: defaults.q_windup_seconds,
                q_projectile_speed: defaults.q_projectile_speed,
                q_effect_hitbox_radius: defaults.q_effect_hitbox_radius,
                e_range: defaults.e_range,
                e_windup_seconds: defaults.e_windup_seconds,
                e_projectile_speed: defaults.e_projectile_speed,
                e_effect_hitbox_radius: defaults.e_effect_hitbox_radius,
                r_range: defaults.r_range,
                r_windup_seconds: defaults.r_windup_seconds,
                r_projectile_speed: defaults.r_projectile_speed,
                r_effect_hitbox_radius: defaults.r_effect_hitbox_radius,
            })
        })
        .unwrap_or_default();
    let offensive_tuning = vladimir_offensive_ability_defaults("vladimir")
        .map(|defaults| ControlledChampionAbilityTuning {
            offensive_primary_base_damage: defaults.q_base_damage,
            offensive_primary_ap_ratio: defaults.q_ap_ratio,
            offensive_primary_heal_ratio_of_damage: defaults.q_heal_ratio_of_damage,
            offensive_primary_base_cooldown_seconds: defaults.q_base_cooldown_seconds,
            offensive_secondary_base_damage: defaults.e_base_damage,
            offensive_secondary_ap_ratio: defaults.e_ap_ratio,
            offensive_secondary_base_cooldown_seconds: defaults.e_base_cooldown_seconds,
            offensive_ultimate_base_damage: defaults.r_base_damage,
            offensive_ultimate_ap_ratio: defaults.r_ap_ratio,
            offensive_ultimate_base_cooldown_seconds: defaults.r_base_cooldown_seconds,
        })
        .unwrap_or_default();
    let pool_defaults = vladimir_sanguine_pool_defaults("vladimir").unwrap_or_else(|| {
        panic!("missing Vladimir Sanguine Pool defaults in canonical champion data")
    });
    let defensive_ability_two_policy_defaults =
        vladimir_defensive_ability_two_policy_defaults("vladimir").unwrap_or_else(|| {
            panic!(
                "missing Vladimir defensive ability two policy defaults in champion simulation data"
            )
        });

    Arc::new(VladimirControlledChampionScript {
        cast_profile,
        offensive_tuning,
        defensive_ability_two_rank: pool_defaults.default_rank,
        defensive_ability_two_duration_seconds: pool_defaults.untargetable_seconds,
        defensive_ability_two_effect_range: pool_defaults.effect_range,
        defensive_ability_two_damage_tick_interval_seconds: pool_defaults
            .damage_tick_interval_seconds,
        defensive_ability_two_cost_percent_current_health: pool_defaults
            .cost_percent_current_health,
        defensive_ability_two_heal_ratio_of_damage: pool_defaults.heal_ratio_of_damage,
        defensive_ability_two_damage_per_tick_by_rank: pool_defaults.damage_per_tick_by_rank,
        defensive_ability_two_base_cooldown_seconds_by_rank: pool_defaults
            .base_cooldown_seconds_by_rank,
        defensive_ability_two_damage_per_tick_bonus_health_ratio: pool_defaults
            .damage_per_tick_bonus_health_ratio,
        prioritize_offensive_ultimate_before_defensive_ability_two:
            defensive_ability_two_policy_defaults
                .prioritize_offensive_ultimate_before_defensive_ability_two,
    })
}

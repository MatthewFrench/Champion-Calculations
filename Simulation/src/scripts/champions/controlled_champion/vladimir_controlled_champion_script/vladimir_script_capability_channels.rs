use crate::scripts::champions::vladimir;
use crate::scripts::runtime::stat_resolution::{
    CooldownMetricSource, RuntimeBuffState, StatQuery, resolve_stat,
};
use crate::{ChampionBase, Stats};

use super::super::{
    ControlledChampionAbilityCooldowns, ControlledChampionAreaCastDecision,
    ControlledChampionDefensiveAbilityDecisions, ControlledChampionDefensiveAbilityTwoConfig,
    ControlledChampionOffensiveAbility, ControlledChampionOffensiveCastDecisions,
    ControlledChampionScriptCapability, ControlledChampionSingleTargetCastDecision,
    ControlledChampionTargetSnapshot,
};
use super::vladimir_script_model::{
    VladimirControlledChampionScript, resolve_ranked_value, to_vladimir_cast_profile,
    to_vladimir_tuning,
};

impl ControlledChampionScriptCapability for VladimirControlledChampionScript {
    fn default_cast_profile(&self) -> super::super::ControlledChampionCastProfile {
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
        input: super::super::ControlledChampionDefensiveAbilityDecisionInput,
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
        input: super::super::ControlledChampionOffensiveDecisionInput,
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

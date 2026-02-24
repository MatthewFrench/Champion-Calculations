use std::sync::Arc;

use anyhow::{Result, anyhow};

use crate::defaults::sona_crescendo_ability_defaults;
use crate::{ChampionBase, Stats};

use super::{
    ControlledChampionAbilityCooldowns, ControlledChampionAreaCastDecision,
    ControlledChampionCastProfile, ControlledChampionDefensiveAbilityDecisions,
    ControlledChampionDefensiveAbilityTwoConfig, ControlledChampionOffensiveAbility,
    ControlledChampionOffensiveCastDecisions, ControlledChampionScriptCapability,
    ControlledChampionScriptHandle,
};

#[derive(Debug, Clone)]
struct SonaControlledChampionScript {
    cast_profile: ControlledChampionCastProfile,
    ultimate_cooldown_seconds: f64,
    ultimate_magic_base_damage: f64,
    ultimate_magic_ability_power_ratio: f64,
}

fn projectile_travel_seconds(distance: f64, speed: f64) -> f64 {
    if speed <= 0.0 {
        0.0
    } else {
        (distance / speed).max(0.0)
    }
}

impl ControlledChampionScriptCapability for SonaControlledChampionScript {
    fn default_cast_profile(&self) -> ControlledChampionCastProfile {
        self.cast_profile.clone()
    }

    fn offensive_cooldowns_after_haste(
        &self,
        ability_haste: f64,
    ) -> ControlledChampionAbilityCooldowns {
        let haste_scalar = (1.0 + ability_haste.max(0.0) / 100.0).max(1.0);
        ControlledChampionAbilityCooldowns {
            offensive_primary_seconds: 0.0,
            offensive_secondary_seconds: 0.0,
            offensive_ultimate_seconds: self.ultimate_cooldown_seconds / haste_scalar,
        }
    }

    fn defensive_ability_two_config(
        &self,
        _ability_haste: f64,
    ) -> ControlledChampionDefensiveAbilityTwoConfig {
        ControlledChampionDefensiveAbilityTwoConfig::default()
    }

    fn decide_defensive_ability_activations(
        &self,
        _input: super::ControlledChampionDefensiveAbilityDecisionInput,
    ) -> ControlledChampionDefensiveAbilityDecisions {
        ControlledChampionDefensiveAbilityDecisions::default()
    }

    fn decide_offensive_casts(
        &self,
        input: super::ControlledChampionOffensiveDecisionInput,
    ) -> ControlledChampionOffensiveCastDecisions {
        if !input.can_cast || input.now_seconds < input.offensive_ultimate_ready_at {
            return ControlledChampionOffensiveCastDecisions::default();
        }
        let Some(max_distance) = input.offensive_ultimate_max_distance else {
            return ControlledChampionOffensiveCastDecisions::default();
        };
        let travel = projectile_travel_seconds(
            max_distance,
            input.cast_profile.offensive_ultimate_projectile_speed,
        );
        ControlledChampionOffensiveCastDecisions {
            offensive_primary: None,
            offensive_secondary: None,
            offensive_ultimate: Some(ControlledChampionAreaCastDecision {
                ability_id: input.cast_profile.offensive_ultimate_ability_id,
                impact_delay_seconds: input.cast_profile.offensive_ultimate_windup_seconds + travel,
                next_ready_at: input.now_seconds + input.cooldowns.offensive_ultimate_seconds,
            }),
        }
    }

    fn offensive_raw_damage(
        &self,
        ability: ControlledChampionOffensiveAbility,
        ability_power: f64,
    ) -> f64 {
        match ability {
            ControlledChampionOffensiveAbility::Ultimate => {
                self.ultimate_magic_base_damage
                    + self.ultimate_magic_ability_power_ratio * ability_power.max(0.0)
            }
            ControlledChampionOffensiveAbility::Primary
            | ControlledChampionOffensiveAbility::Secondary => 0.0,
        }
    }

    fn offensive_ap_ratio(&self, ability: ControlledChampionOffensiveAbility) -> f64 {
        match ability {
            ControlledChampionOffensiveAbility::Ultimate => self.ultimate_magic_ability_power_ratio,
            ControlledChampionOffensiveAbility::Primary
            | ControlledChampionOffensiveAbility::Secondary => 0.0,
        }
    }

    fn offensive_primary_heal_ratio(&self) -> f64 {
        0.0
    }

    fn defensive_ability_two_raw_damage(
        &self,
        _config: ControlledChampionDefensiveAbilityTwoConfig,
        _controlled_champion_stats: &Stats,
        _controlled_champion_base: &ChampionBase,
    ) -> f64 {
        0.0
    }
}

pub(crate) fn build_sona_script() -> Result<ControlledChampionScriptHandle> {
    let defaults = sona_crescendo_ability_defaults("sona")
        .ok_or_else(|| anyhow!("missing Sona Crescendo defaults in canonical champion data"))?;
    Ok(Arc::new(SonaControlledChampionScript {
        cast_profile: ControlledChampionCastProfile {
            offensive_primary_ability_id: "hymn_of_valor".to_string(),
            defensive_ability_two_id: "aria_of_perseverance".to_string(),
            offensive_secondary_ability_id: "song_of_celerity".to_string(),
            offensive_ultimate_ability_id: "crescendo".to_string(),
            offensive_primary_range: 0.0,
            offensive_primary_windup_seconds: 0.0,
            offensive_primary_projectile_speed: 0.0,
            offensive_primary_effect_hitbox_radius: 0.0,
            offensive_secondary_range: 0.0,
            offensive_secondary_windup_seconds: 0.0,
            offensive_secondary_projectile_speed: 0.0,
            offensive_secondary_effect_hitbox_radius: 0.0,
            offensive_ultimate_range: defaults.crescendo_cast_range,
            offensive_ultimate_windup_seconds: defaults.crescendo_execution.cast_windup_seconds,
            offensive_ultimate_projectile_speed: defaults.crescendo_execution.projectile_speed,
            offensive_ultimate_effect_hitbox_radius: defaults
                .crescendo_execution
                .effect_hitbox_radius,
        },
        ultimate_cooldown_seconds: defaults.crescendo_cooldown_seconds,
        ultimate_magic_base_damage: defaults.crescendo_magic_base_damage,
        ultimate_magic_ability_power_ratio: defaults.crescendo_magic_ability_power_ratio,
    }))
}

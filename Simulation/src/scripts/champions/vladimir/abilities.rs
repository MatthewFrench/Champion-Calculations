use crate::scripts::runtime::stat_resolution::{
    CooldownMetricSource, RuntimeBuffState, StatQuery, resolve_stat,
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirAbilityTuning {
    pub q_base_damage: f64,
    pub q_ap_ratio: f64,
    pub q_heal_ratio_of_damage: f64,
    pub q_base_cooldown_seconds: f64,
    pub e_base_damage: f64,
    pub e_ap_ratio: f64,
    pub e_base_cooldown_seconds: f64,
    pub r_base_damage: f64,
    pub r_ap_ratio: f64,
    pub r_base_cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirAbilityCooldowns {
    pub q_seconds: f64,
    pub e_seconds: f64,
    pub r_seconds: f64,
}

pub(crate) fn offensive_cooldowns_after_haste(
    tuning: VladimirAbilityTuning,
    ability_haste: f64,
) -> VladimirAbilityCooldowns {
    let buffs = RuntimeBuffState {
        ability_haste,
        ..RuntimeBuffState::default()
    };
    VladimirAbilityCooldowns {
        q_seconds: resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: tuning.q_base_cooldown_seconds,
                source: CooldownMetricSource::Ability,
            },
            buffs,
        ),
        e_seconds: resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: tuning.e_base_cooldown_seconds,
                source: CooldownMetricSource::Ability,
            },
            buffs,
        ),
        r_seconds: resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: tuning.r_base_cooldown_seconds,
                source: CooldownMetricSource::Ability,
            },
            buffs,
        ),
    }
}

pub(crate) fn q_damage_raw(tuning: VladimirAbilityTuning, ability_power: f64) -> f64 {
    tuning.q_base_damage + tuning.q_ap_ratio * ability_power
}

pub(crate) fn e_damage_raw(tuning: VladimirAbilityTuning, ability_power: f64) -> f64 {
    tuning.e_base_damage + tuning.e_ap_ratio * ability_power
}

pub(crate) fn r_damage_raw(tuning: VladimirAbilityTuning, ability_power: f64) -> f64 {
    tuning.r_base_damage + tuning.r_ap_ratio * ability_power
}

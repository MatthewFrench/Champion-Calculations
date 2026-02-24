use super::LoadoutRuntimeState;
use crate::defaults::rune_runtime_defaults;

pub(super) fn press_the_attack_damage_multiplier(
    runtime: &LoadoutRuntimeState,
    target_id: Option<usize>,
    now: f64,
) -> f64 {
    if !runtime.has_press_the_attack {
        return 0.0;
    }
    let Some(target_idx) = target_id else {
        return 0.0;
    };
    let Some(state) = runtime.press_the_attack_targets.get(&target_idx) else {
        return 0.0;
    };
    if now > state.vulnerable_until {
        return 0.0;
    }
    rune_runtime_defaults()
        .press_the_attack
        .vulnerability_true_damage_ratio
        .max(0.0)
}

pub(super) fn gathering_storm_bonus_ability_power(runtime: &LoadoutRuntimeState, now: f64) -> f64 {
    if !runtime.has_gathering_storm {
        return 0.0;
    }
    let defaults = &rune_runtime_defaults().gathering_storm;
    if defaults.interval_seconds <= 0.0 {
        return defaults
            .ability_power_by_interval
            .last()
            .copied()
            .unwrap_or(0.0)
            .max(0.0);
    }
    if now < defaults.interval_seconds {
        return 0.0;
    }
    let interval_index = (now / defaults.interval_seconds).floor() as usize - 1;
    defaults
        .ability_power_by_interval
        .get(interval_index)
        .copied()
        .or_else(|| defaults.ability_power_by_interval.last().copied())
        .unwrap_or(0.0)
        .max(0.0)
}

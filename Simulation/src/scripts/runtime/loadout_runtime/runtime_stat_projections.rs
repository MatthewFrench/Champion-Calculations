use super::{LoadoutRuntimeState, level_scaled_range_value};
use crate::defaults::rune_runtime_defaults;
use crate::scripts::runtime::stat_resolution::{
    RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
};

pub(super) fn loadout_attack_speed_multiplier_impl(runtime: &LoadoutRuntimeState, now: f64) -> f64 {
    let rune_defaults = rune_runtime_defaults();
    let lethal_tempo_bonus = if runtime.has_lethal_tempo {
        rune_defaults.lethal_tempo.attack_speed_per_stack.max(0.0)
            * runtime.lethal_tempo_stacks as f64
    } else {
        0.0
    };
    let guinsoo_bonus = if runtime.has_guinsoo {
        0.02 * runtime.guinsoo_stacks as f64
    } else {
        0.0
    };
    let hail_of_blades_bonus = if runtime.has_hail_of_blades
        && runtime.hail_of_blades_remaining_attacks > 0
        && now <= runtime.hail_of_blades_expires_at
    {
        if runtime.owner_is_melee {
            rune_defaults
                .hail_of_blades
                .bonus_attack_speed_ratio_melee
                .max(0.0)
        } else {
            rune_defaults
                .hail_of_blades
                .bonus_attack_speed_ratio_ranged
                .max(0.0)
        }
    } else {
        0.0
    };
    1.0 + lethal_tempo_bonus + guinsoo_bonus + hail_of_blades_bonus
}

#[allow(clippy::too_many_arguments)]
pub(super) fn loadout_incoming_damage_multipliers_impl(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    current_armor: f64,
    current_magic_resist: f64,
    bonus_armor: f64,
    bonus_magic_resist: f64,
) -> (f64, f64) {
    let defaults = &rune_runtime_defaults().aftershock;
    if !runtime.has_aftershock || now > runtime.aftershock_active_until {
        return (1.0, 1.0);
    }
    let cap = level_scaled_range_value(actor_level, defaults.resist_cap_by_level);
    let bonus_armor_gain =
        (defaults.resist_base + defaults.resist_bonus_ratio * bonus_armor.max(0.0)).min(cap);
    let bonus_magic_resist_gain =
        (defaults.resist_base + defaults.resist_bonus_ratio * bonus_magic_resist.max(0.0)).min(cap);

    let armor = current_armor.max(0.0);
    let magic_resist = current_magic_resist.max(0.0);
    let physical_multiplier_before = 100.0 / (100.0 + armor);
    let magic_multiplier_before = 100.0 / (100.0 + magic_resist);
    let physical_multiplier_after = 100.0 / (100.0 + armor + bonus_armor_gain.max(0.0));
    let magic_multiplier_after = 100.0 / (100.0 + magic_resist + bonus_magic_resist_gain.max(0.0));

    (
        (physical_multiplier_after / physical_multiplier_before).clamp(0.0, 1.0),
        (magic_multiplier_after / magic_multiplier_before).clamp(0.0, 1.0),
    )
}

pub(super) fn loadout_movement_speed_multiplier_impl(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
) -> f64 {
    if !runtime.has_phase_rush || now > runtime.phase_rush_active_until {
        return 1.0;
    }
    1.0 + level_scaled_range_value(
        actor_level,
        rune_runtime_defaults()
            .phase_rush
            .movement_speed_bonus_ratio_by_level,
    )
}

pub(super) fn tick_loadout_regeneration_impl(
    runtime: &LoadoutRuntimeState,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().second_wind;
    if !runtime.has_second_wind || max_health <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let health_ratio = (current_health / max_health).clamp(0.0, 1.0);
    let base_regen = defaults.base_regen_max_health_ratio_per_second.max(0.0) * max_health * dt;
    let bonus = if health_ratio <= defaults.low_health_threshold_ratio {
        defaults
            .low_health_bonus_regen_max_health_ratio_per_second
            .max(0.0)
            * max_health
            * dt
    } else {
        0.0
    };
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: base_regen + bonus,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

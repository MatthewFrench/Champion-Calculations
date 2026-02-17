use crate::cooldown_after_haste;

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RuntimeBuffState {
    pub ability_haste: f64,
    pub item_haste: f64,
    pub cooldown_rate_multiplier: f64,
    pub incoming_damage_taken_multiplier: f64,
    pub healing_multiplier: f64,
    pub movement_speed_multiplier: f64,
    pub outgoing_ability_damage_multiplier: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum CooldownMetricSource {
    Ability,
    Item,
    Neutral,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ScalarMetricSource {
    Neutral,
    IncomingDamageTaken,
    Healing,
    MovementSpeed,
    OutgoingAbilityDamage,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum StatQuery {
    CooldownSeconds {
        base_seconds: f64,
        source: CooldownMetricSource,
    },
    ScalarAmount {
        base_amount: f64,
        source: ScalarMetricSource,
        clamp_min_zero: bool,
    },
    MovementSpeedUnits {
        base_units: f64,
        flat_bonus_units: f64,
        percent_bonus: f64,
        minimum_units: f64,
    },
}

fn positive_or_one(value: f64) -> f64 {
    if value > 0.0 { value } else { 1.0 }
}

fn resolve_scalar_amount(
    base_amount: f64,
    source: ScalarMetricSource,
    clamp_min_zero: bool,
    buffs: RuntimeBuffState,
) -> f64 {
    let multiplier = match source {
        ScalarMetricSource::Neutral => 1.0,
        ScalarMetricSource::IncomingDamageTaken => {
            positive_or_one(buffs.incoming_damage_taken_multiplier)
        }
        ScalarMetricSource::Healing => positive_or_one(buffs.healing_multiplier),
        ScalarMetricSource::MovementSpeed => positive_or_one(buffs.movement_speed_multiplier),
        ScalarMetricSource::OutgoingAbilityDamage => {
            positive_or_one(buffs.outgoing_ability_damage_multiplier)
        }
    };
    let resolved = base_amount * multiplier;
    if clamp_min_zero {
        resolved.max(0.0)
    } else {
        resolved
    }
}

pub(crate) fn resolve_stat(query: StatQuery, buffs: RuntimeBuffState) -> f64 {
    match query {
        StatQuery::CooldownSeconds {
            base_seconds,
            source,
        } => {
            let base = base_seconds.max(0.0);
            let haste = match source {
                CooldownMetricSource::Ability => buffs.ability_haste,
                CooldownMetricSource::Item => buffs.item_haste,
                CooldownMetricSource::Neutral => 0.0,
            };
            let after_haste = cooldown_after_haste(base, haste.max(-99.0));
            let multiplier = positive_or_one(buffs.cooldown_rate_multiplier);
            (after_haste * multiplier).max(0.0)
        }
        StatQuery::ScalarAmount {
            base_amount,
            source,
            clamp_min_zero,
        } => resolve_scalar_amount(base_amount, source, clamp_min_zero, buffs),
        StatQuery::MovementSpeedUnits {
            base_units,
            flat_bonus_units,
            percent_bonus,
            minimum_units,
        } => {
            let pre_buff_speed = (base_units + flat_bonus_units).max(minimum_units.max(0.0))
                * (1.0 + percent_bonus / 100.0).max(0.0);
            resolve_scalar_amount(
                pre_buff_speed,
                ScalarMetricSource::MovementSpeed,
                true,
                buffs,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_item_cooldown_applies_item_haste() {
        let value = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: 120.0,
                source: CooldownMetricSource::Item,
            },
            RuntimeBuffState {
                item_haste: 300.0,
                ..RuntimeBuffState::default()
            },
        );
        assert!((value - 30.0).abs() < 1e-9);
    }

    #[test]
    fn resolve_ability_cooldown_applies_ability_haste() {
        let value = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: 20.0,
                source: CooldownMetricSource::Ability,
            },
            RuntimeBuffState {
                ability_haste: 300.0,
                ..RuntimeBuffState::default()
            },
        );
        assert!((value - 5.0).abs() < 1e-9);
    }

    #[test]
    fn resolve_healing_scalar_applies_healing_multiplier() {
        let value = resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: 100.0,
                source: ScalarMetricSource::Healing,
                clamp_min_zero: true,
            },
            RuntimeBuffState {
                healing_multiplier: 1.25,
                ..RuntimeBuffState::default()
            },
        );
        assert!((value - 125.0).abs() < 1e-9);
    }

    #[test]
    fn resolve_movement_speed_uses_base_flat_percent_and_multiplier() {
        let value = resolve_stat(
            StatQuery::MovementSpeedUnits {
                base_units: 330.0,
                flat_bonus_units: 20.0,
                percent_bonus: 10.0,
                minimum_units: 150.0,
            },
            RuntimeBuffState {
                movement_speed_multiplier: 1.10,
                ..RuntimeBuffState::default()
            },
        );
        assert!((value - 423.5).abs() < 1e-9);
    }

    #[test]
    fn resolve_scalar_clamps_negative_when_requested() {
        let value = resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: -10.0,
                source: ScalarMetricSource::Neutral,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        );
        assert_eq!(value, 0.0);
    }
}

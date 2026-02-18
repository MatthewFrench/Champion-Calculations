
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

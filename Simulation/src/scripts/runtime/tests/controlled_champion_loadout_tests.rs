use super::*;
use crate::scripts::runes::effects::DYNAMIC_RUNE_KEYS;

#[test]
fn controlled_runtime_is_stateless_and_reports_no_cooldowns() {
    let lines = describe_controlled_champion_runtime_cooldowns(5.0);
    assert_eq!(lines, vec!["none".to_string()]);
}

#[test]
fn defensive_item_decisions_require_health_and_readiness() {
    let decisions = decide_defensive_item_activations(DefensiveItemActivationInput {
        now_seconds: 10.0,
        can_cast: true,
        health: 320.0,
        max_health: 1000.0,
        stasis_available: true,
        stasis_ready_at: 7.0,
        stasis_trigger_health_percent: 0.35,
        untargetable_active_until: 10.0,
        revive_lock_active_until: 9.0,
        emergency_shield_available: true,
        emergency_shield_ready_at: 6.0,
        emergency_shield_trigger_health_percent: 0.40,
    });
    assert!(decisions.activate_stasis);
    assert!(decisions.activate_emergency_shield);
}

#[test]
fn defensive_item_decisions_block_stasis_during_lock_windows() {
    let decisions = decide_defensive_item_activations(DefensiveItemActivationInput {
        now_seconds: 10.0,
        can_cast: true,
        health: 200.0,
        max_health: 1000.0,
        stasis_available: true,
        stasis_ready_at: 0.0,
        stasis_trigger_health_percent: 0.50,
        untargetable_active_until: 11.0,
        revive_lock_active_until: 0.0,
        emergency_shield_available: false,
        emergency_shield_ready_at: 0.0,
        emergency_shield_trigger_health_percent: 0.0,
    });
    assert!(!decisions.activate_stasis);

    let blocked_by_revive = decide_defensive_item_activations(DefensiveItemActivationInput {
        now_seconds: 10.0,
        can_cast: true,
        health: 200.0,
        max_health: 1000.0,
        stasis_available: true,
        stasis_ready_at: 0.0,
        stasis_trigger_health_percent: 0.50,
        untargetable_active_until: 0.0,
        revive_lock_active_until: 11.0,
        emergency_shield_available: false,
        emergency_shield_ready_at: 0.0,
        emergency_shield_trigger_health_percent: 0.0,
    });
    assert!(!blocked_by_revive.activate_stasis);
}

#[test]
fn revive_effect_trigger_checks_cooldown_and_availability() {
    assert!(should_trigger_revive_effect(ReviveEffectDecisionInput {
        available: true,
        now_seconds: 120.0,
        ready_at: 120.0,
    }));
    assert!(!should_trigger_revive_effect(ReviveEffectDecisionInput {
        available: true,
        now_seconds: 119.9,
        ready_at: 120.0,
    }));
    assert!(!should_trigger_revive_effect(ReviveEffectDecisionInput {
        available: false,
        now_seconds: 120.0,
        ready_at: 0.0,
    }));
}

#[test]
fn dynamic_rune_key_list_matches_modeled_coverage_expectations() {
    let mut expected = vec![
        "aftershock",
        "arcanecomet",
        "conqueror",
        "darkharvest",
        "electrocute",
        "firststrike",
        "fleetfootwork",
        "gatheringstorm",
        "graspoftheundying",
        "hailofblades",
        "lethaltempo",
        "phaserush",
        "presstheattack",
        "secondwind",
        "summonaery",
        "triumph",
    ];
    expected.sort_unstable();

    let mut actual = DYNAMIC_RUNE_KEYS.to_vec();
    actual.sort_unstable();

    assert_eq!(
        actual, expected,
        "When adding a dynamic rune key, add an observable-effect coverage assertion too."
    );
}

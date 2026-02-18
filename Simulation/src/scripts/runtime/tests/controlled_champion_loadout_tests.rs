use super::*;

fn selection_with(rune_names: &[&str]) -> LoadoutSelection {
    LoadoutSelection {
        rune_names: rune_names.iter().map(|s| (*s).to_string()).collect(),
        shard_stats: Vec::new(),
    }
}

#[test]
fn controlled_champion_runtime_parses_dynamic_runes() {
    let runtime = build_controlled_champion_loadout_runtime(&selection_with(&[
        "Arcane Comet",
        "Summon Aery",
        "Triumph",
        "Second Wind",
    ]));
    assert!(runtime.has_arcane_comet);
    assert!(runtime.has_summon_aery);
    assert!(runtime.has_triumph);
    assert!(runtime.has_second_wind);
}

#[test]
fn arcane_comet_and_aery_respect_runtime_cooldowns() {
    let mut runtime = build_controlled_champion_loadout_runtime(&selection_with(&[
        "Arcane Comet",
        "Summon Aery",
    ]));

    let first = on_controlled_champion_ability_bonus(
        &mut runtime,
        ControlledChampionAbilityRuntimeInput {
            ability_power: 300.0,
            ability_ap_ratio: 0.6,
            now_seconds: 1.0,
        },
    );
    let second = on_controlled_champion_ability_bonus(
        &mut runtime,
        ControlledChampionAbilityRuntimeInput {
            ability_power: 300.0,
            ability_ap_ratio: 0.6,
            now_seconds: 1.5,
        },
    );
    assert!(first.extra_magic_damage > second.extra_magic_damage);
}

#[test]
fn second_wind_regen_gives_more_heal_at_low_health() {
    let runtime = build_controlled_champion_loadout_runtime(&selection_with(&["Second Wind"]));
    let high = tick_controlled_champion_regen_heal(&runtime, 1800.0, 2000.0, 1.0);
    let low = tick_controlled_champion_regen_heal(&runtime, 500.0, 2000.0, 1.0);
    assert!(low > high);
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

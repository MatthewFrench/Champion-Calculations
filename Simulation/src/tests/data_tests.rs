use super::*;

fn assert_close(actual: f64, expected: f64, field: &str) {
    let epsilon = 1e-9;
    assert!(
        (actual - expected).abs() <= epsilon,
        "{field} mismatch: actual={actual}, expected={expected}"
    );
}

#[test]
fn scenario_controlled_champion_script_uses_canonical_ability_defaults() {
    let scenario_path = scenarios_dir().join("vladimir_urf_teamfight.json");
    let scenario =
        load_json(&scenario_path).expect("scenarios/vladimir_urf_teamfight.json should parse");
    let simulation = scenario
        .get("simulation")
        .expect("scenarios/vladimir_urf_teamfight.json should include simulation");

    for key in [
        "vlad_pool_rank",
        "vlad_pool_untargetable_seconds",
        "vlad_pool_cost_percent_current_health",
        "vlad_pool_heal_ratio_of_damage",
        "vlad_pool_base_damage_by_rank",
        "vlad_pool_bonus_health_ratio",
        "zhonya_duration_seconds",
        "zhonya_cooldown_seconds",
        "zhonya_trigger_health_percent",
        "ga_cooldown_seconds",
        "ga_revive_duration_seconds",
        "ga_revive_base_health_ratio",
        "protoplasm_trigger_health_percent",
        "protoplasm_bonus_health",
        "protoplasm_heal_total",
        "protoplasm_duration_seconds",
        "urf_respawn_flat_reduction_seconds",
        "urf_respawn_extrapolation_per_level",
        "urf_respawn_time_scaling_enabled",
        "urf_respawn_time_scaling_start_seconds",
        "urf_respawn_time_scaling_per_minute_seconds",
        "urf_respawn_time_scaling_cap_seconds",
        "vlad_q_base_damage",
        "vlad_q_ap_ratio",
        "vlad_q_heal_ratio_of_damage",
        "vlad_q_base_cooldown_seconds",
        "vlad_e_base_damage",
        "vlad_e_ap_ratio",
        "vlad_e_base_cooldown_seconds",
        "vlad_r_base_damage",
        "vlad_r_ap_ratio",
        "vlad_r_base_cooldown_seconds",
    ] {
        assert!(
            simulation.get(key).is_none(),
            "Scenario should not duplicate canonical Vladimir offensive constant '{key}'"
        );
    }

    let parsed = parse_simulation_config(simulation).expect("simulation config should parse");
    assert!(
        parsed.controlled_champion_script.is_none(),
        "simulation parser should not inject champion-specific script config"
    );

    let script = crate::scripts::champions::resolve_controlled_champion_script("Vladimir");
    assert!(
        script.is_some(),
        "Vladimir script capability should resolve"
    );
    let canonical = crate::defaults::vladimir_offensive_ability_defaults("vladimir")
        .expect("canonical Vladimir offensive defaults should load");
    let cooldowns = crate::scripts::champions::controlled_champion_offensive_cooldowns_after_haste(
        script.as_ref(),
        0.0,
    );

    assert_close(
        crate::scripts::champions::controlled_champion_offensive_raw_damage(
            script.as_ref(),
            crate::scripts::champions::ControlledChampionOffensiveAbility::Primary,
            0.0,
        ),
        canonical.q_base_damage,
        "offensive_primary_base_damage",
    );
    assert_close(
        crate::scripts::champions::controlled_champion_offensive_ap_ratio(
            script.as_ref(),
            crate::scripts::champions::ControlledChampionOffensiveAbility::Primary,
        ),
        canonical.q_ap_ratio,
        "offensive_primary_ap_ratio",
    );
    assert_close(
        crate::scripts::champions::controlled_champion_offensive_primary_heal_ratio(
            script.as_ref(),
        ),
        canonical.q_heal_ratio_of_damage,
        "offensive_primary_heal_ratio_of_damage",
    );
    assert_close(
        cooldowns.offensive_primary_seconds,
        canonical.q_base_cooldown_seconds,
        "offensive_primary_base_cooldown_seconds",
    );
    assert_close(
        crate::scripts::champions::controlled_champion_offensive_raw_damage(
            script.as_ref(),
            crate::scripts::champions::ControlledChampionOffensiveAbility::Secondary,
            0.0,
        ),
        canonical.e_base_damage,
        "offensive_secondary_base_damage",
    );
    assert_close(
        crate::scripts::champions::controlled_champion_offensive_ap_ratio(
            script.as_ref(),
            crate::scripts::champions::ControlledChampionOffensiveAbility::Secondary,
        ),
        canonical.e_ap_ratio,
        "offensive_secondary_ap_ratio",
    );
    assert_close(
        cooldowns.offensive_secondary_seconds,
        canonical.e_base_cooldown_seconds,
        "offensive_secondary_base_cooldown_seconds",
    );
    assert_close(
        crate::scripts::champions::controlled_champion_offensive_raw_damage(
            script.as_ref(),
            crate::scripts::champions::ControlledChampionOffensiveAbility::Ultimate,
            0.0,
        ),
        canonical.r_base_damage,
        "offensive_ultimate_base_damage",
    );
    assert_close(
        crate::scripts::champions::controlled_champion_offensive_ap_ratio(
            script.as_ref(),
            crate::scripts::champions::ControlledChampionOffensiveAbility::Ultimate,
        ),
        canonical.r_ap_ratio,
        "offensive_ultimate_ap_ratio",
    );
    assert_close(
        cooldowns.offensive_ultimate_seconds,
        canonical.r_base_cooldown_seconds,
        "offensive_ultimate_base_cooldown_seconds",
    );
}

#[test]
fn load_champion_bases_skips_support_defaults_file() {
    let bases = load_champion_bases().expect("champion bases should load");
    assert!(
        !bases.contains_key(&normalize_name("ChampionDefaults")),
        "support defaults file should not be treated as a champion base"
    );
    assert!(
        bases.contains_key(&normalize_name("Vladimir")),
        "known champion base should still be present"
    );
}

#[test]
fn validate_rune_page_selection_rejects_secondary_slot_order_violation() {
    let domain = build_loadout_domain();
    let valid = LoadoutSelection {
        rune_names: vec![
            "Arcane Comet".to_string(),
            "Manaflow Band".to_string(),
            "Transcendence".to_string(),
            "Gathering Storm".to_string(),
            "Cheap Shot".to_string(),
            "Ultimate Hunter".to_string(),
        ],
        shard_stats: vec![
            "ability_haste".to_string(),
            "movement_speed".to_string(),
            "health".to_string(),
        ],
    };
    validate_rune_page_selection(&valid, &domain)
        .expect("known rune page should pass legality validation");

    let invalid_secondary_order = LoadoutSelection {
        rune_names: vec![
            "Arcane Comet".to_string(),
            "Manaflow Band".to_string(),
            "Transcendence".to_string(),
            "Gathering Storm".to_string(),
            "Ultimate Hunter".to_string(),
            "Cheap Shot".to_string(),
        ],
        shard_stats: vec![
            "ability_haste".to_string(),
            "movement_speed".to_string(),
            "health".to_string(),
        ],
    };
    assert!(
        validate_rune_page_selection(&invalid_secondary_order, &domain).is_err(),
        "secondary runes out of slot order should fail validation"
    );
}

#[test]
fn validate_rune_page_selection_rejects_invalid_shard_slot() {
    let domain = build_loadout_domain();
    let invalid_shard_slot = LoadoutSelection {
        rune_names: vec![
            "Lethal Tempo".to_string(),
            "Triumph".to_string(),
            "Legend: Alacrity".to_string(),
            "Last Stand".to_string(),
            "Conditioning".to_string(),
            "Overgrowth".to_string(),
        ],
        shard_stats: vec![
            "health".to_string(),
            "movement_speed".to_string(),
            "tenacity".to_string(),
        ],
    };
    assert!(
        validate_rune_page_selection(&invalid_shard_slot, &domain).is_err(),
        "slot 1 shard should reject unsupported stat keys"
    );
}

#[test]
fn parse_simulation_config_rejects_legacy_max_time_field() {
    let simulation = serde_json::json!({
        "time_limit_seconds": 60.0,
        "max_time_seconds": 60.0
    });
    let error = parse_simulation_config(&simulation)
        .expect_err("legacy simulation.max_time_seconds should be rejected");
    assert!(
        error
            .to_string()
            .contains("simulation.max_time_seconds is no longer supported"),
        "unexpected error: {error}"
    );
}

#[test]
fn parse_simulation_config_rejects_legacy_heartsteel_stack_field() {
    let simulation = serde_json::json!({
        "time_limit_seconds": 60.0,
        "heartsteel_assumed_stacks_at_8m": 20.0
    });
    let error = parse_simulation_config(&simulation)
        .expect_err("legacy simulation.heartsteel_assumed_stacks_at_8m should be rejected");
    assert!(
        error
            .to_string()
            .contains("simulation.heartsteel_assumed_stacks_at_8m is no longer supported"),
        "unexpected error: {error}"
    );
}

#[test]
fn parse_simulation_config_rejects_legacy_item_stacks_map() {
    let simulation = serde_json::json!({
        "time_limit_seconds": 60.0,
        "item_stacks_at_level_20": {
            "Heartsteel": 20.0
        }
    });
    let error = parse_simulation_config(&simulation)
        .expect_err("legacy simulation.item_stacks_at_level_20 should be rejected");
    assert!(
        error
            .to_string()
            .contains("simulation.item_stacks_at_level_20 is no longer supported"),
        "unexpected error: {error}"
    );
}

#[test]
fn parse_simulation_config_rejects_legacy_vladimir_tuning_keys() {
    let simulation = serde_json::json!({
        "vlad_q_base_damage": 200.0
    });
    let error = parse_simulation_config(&simulation)
        .expect_err("legacy simulation.vlad_q_base_damage should be rejected");
    assert!(
        error
            .to_string()
            .contains("simulation.vlad_q_base_damage is no longer supported"),
        "unexpected error: {error}"
    );
}

#[test]
fn parse_simulation_config_accepts_stack_overrides_by_identifier() {
    let simulation = serde_json::json!({
        "stack_overrides": {
            "heartsteel": 20.0
        }
    });
    let parsed =
        parse_simulation_config(&simulation).expect("simulation.stack_overrides should parse");
    let stacks = parsed
        .stack_overrides
        .get("heartsteel")
        .copied()
        .unwrap_or_default();
    assert!(
        (stacks - 20.0).abs() < 1e-9,
        "unexpected stack value: {stacks}"
    );
}

#[test]
fn parse_simulation_config_reads_protoplasm_trigger_override() {
    let simulation = serde_json::json!({
        "protoplasm_trigger_health_percent": 0.42
    });
    let parsed = parse_simulation_config(&simulation)
        .expect("simulation config with protoplasm trigger override should parse");
    assert!(
        (parsed.protoplasm_trigger_health_percent - 0.42).abs() < 1e-9,
        "unexpected protoplasm trigger health percent: {}",
        parsed.protoplasm_trigger_health_percent
    );
}

#[test]
fn parse_loadout_selection_rejects_legacy_rune_ids() {
    let loadout = serde_json::json!({
        "runes_reforged": {
            "rune_ids": [8229, 8226, 8210, 8237, 8345, 8347]
        }
    });
    let error = parse_loadout_selection(Some(&loadout))
        .expect_err("legacy loadout.runes_reforged.rune_ids should be rejected");
    assert!(
        error
            .to_string()
            .contains("loadout.runes_reforged.rune_ids is no longer supported"),
        "unexpected error: {error}"
    );
}

#[test]
fn parse_loadout_selection_rejects_legacy_season2016_masteries() {
    let loadout = serde_json::json!({
        "season2016_masteries": {
            "Ferocity": []
        }
    });
    let error = parse_loadout_selection(Some(&loadout))
        .expect_err("legacy loadout.season2016_masteries should be rejected");
    assert!(
        error
            .to_string()
            .contains("loadout.season2016_masteries is no longer supported"),
        "unexpected error: {error}"
    );
}

#[test]
fn parse_simulation_config_uses_default_time_limit_when_missing() {
    let simulation = serde_json::json!({});
    let parsed = parse_simulation_config(&simulation)
        .expect("simulation config should parse with default time limit");
    assert!(
        (parsed.max_time_seconds - 1200.0).abs() < 1e-9,
        "expected default time_limit_seconds of 1200.0, got {}",
        parsed.max_time_seconds
    );
}

#[test]
fn parse_enemy_config_rejects_deprecated_combat_proxy() {
    let champion_bases = load_champion_bases().expect("champion bases should load");
    let enemy = serde_json::json!({
        "id": "test_enemy",
        "champion": "Warwick",
        "combat": {
            "ability_dps_flat": 30.0
        }
    });
    let error = parse_enemy_config(&enemy, &champion_bases, 20, &HashMap::new())
        .expect_err("opponent combat proxy should be rejected");
    assert!(
        error
            .to_string()
            .contains("deprecated combat proxy settings"),
        "unexpected error: {error}"
    );
}

use super::*;
use serde_json::Value;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn fixed_sweep_repeat_seed_values_are_unique_and_reproducible() {
    let seed_base = fixed_sweep_keystone_seed_base(1337, "Lethal Tempo");
    let mut seen = HashSet::new();
    for repeat_idx in 0..1024 {
        let seed = fixed_sweep_repeat_seed(seed_base, repeat_idx);
        assert_eq!(
            seed,
            fixed_sweep_repeat_seed(seed_base, repeat_idx),
            "repeat seed generation must be deterministic"
        );
        assert!(
            seen.insert(seed),
            "repeat index {} produced duplicate seed {}",
            repeat_idx,
            seed
        );
    }
}

fn sample_rune_proc_telemetry_entry() -> ChampionRuneProcTelemetryEntry {
    ChampionRuneProcTelemetryEntry {
        rune_name: "Conqueror".to_string(),
        proc_count: 4,
        attempt_count: 8,
        eligible_count: 6,
        proc_attempt_rate: 0.5,
        proc_eligible_rate: 0.6666666667,
        bonus_damage: 180.0,
        bonus_healing: 45.0,
        source_breakdown: vec![
            crate::scripts::runtime::loadout_runtime::RuneProcTelemetrySourceEntry {
                source: "ability".to_string(),
                proc_count: 3,
                attempt_count: 6,
                eligible_count: 4,
                proc_attempt_rate: 0.5,
                proc_eligible_rate: 0.75,
                bonus_damage: 150.0,
                bonus_healing: 20.0,
            },
        ],
    }
}

fn assert_rune_telemetry_json_shape(entry: &Value) {
    assert!(entry.get("rune_name").is_some());
    assert!(entry.get("proc_count").is_some());
    assert!(entry.get("attempt_count").is_some());
    assert!(entry.get("eligible_count").is_some());
    assert!(entry.get("proc_attempt_rate").is_some());
    assert!(entry.get("proc_eligible_rate").is_some());
    assert!(entry.get("opportunity_count").is_some());
    assert!(entry.get("proc_opportunity_rate").is_some());
    assert!(entry.get("bonus_damage").is_some());
    assert!(entry.get("bonus_damage_share").is_some());
    assert!(entry.get("bonus_healing").is_some());
    assert!(entry.get("bonus_healing_share").is_some());
    let first_source = entry
        .get("source_breakdown")
        .and_then(Value::as_array)
        .and_then(|entries| entries.first())
        .expect("source breakdown should include at least one source");
    assert!(first_source.get("attempt_count").is_some());
    assert!(first_source.get("eligible_count").is_some());
    assert!(first_source.get("proc_attempt_rate").is_some());
    assert!(first_source.get("proc_eligible_rate").is_some());
}

#[test]
fn fixed_loadout_report_json_contract_schema_and_telemetry_shape() {
    assert_eq!(FIXED_LOADOUT_REPORT_JSON_SCHEMA_VERSION, 2);
    let entry = sample_rune_proc_telemetry_entry();
    let telemetry_json = rune_proc_telemetry_json(&[entry], 900.0, 100.0);
    assert_eq!(telemetry_json.len(), 1);
    assert_rune_telemetry_json_shape(&telemetry_json[0]);
}

#[test]
fn fixed_loadout_rune_sweep_json_contract_schema_and_telemetry_shape() {
    assert_eq!(FIXED_LOADOUT_RUNE_SWEEP_JSON_SCHEMA_VERSION, 2);
    let entry = sample_rune_proc_telemetry_entry();
    let telemetry_json = rune_proc_telemetry_json(&[entry], 600.0, 200.0);
    assert_eq!(telemetry_json.len(), 1);
    assert_rune_telemetry_json_shape(&telemetry_json[0]);
}

#[test]
fn trace_json_contract_schema_and_telemetry_shape() {
    assert_eq!(FIXED_LOADOUT_TRACE_JSON_SCHEMA_VERSION, 3);
    assert_eq!(CONTROLLED_CHAMPION_TRACE_JSON_SCHEMA_VERSION, 2);
    let entry = sample_rune_proc_telemetry_entry();
    let telemetry_json = rune_proc_telemetry_json(&[entry], 1000.0, 250.0);
    assert_eq!(telemetry_json.len(), 1);
    assert_rune_telemetry_json_shape(&telemetry_json[0]);
}

#[test]
fn parse_opponent_encounters_rejects_all_zero_weights() {
    let champion_bases = load_champion_bases().expect("champion data should load");
    let champion_name = champion_bases
        .values()
        .next()
        .map(|c| c.name.clone())
        .expect("at least one champion should exist");
    let scenario = json!({
        "opponents": {
            "encounters": [
                {
                    "name": "all_zero",
                    "weight": 0.0,
                    "actors": [{ "champion": champion_name }]
                }
            ]
        }
    });
    let err = parse_opponent_encounters(&scenario, &champion_bases, 18, &HashMap::new())
        .expect_err("all-zero encounter weights should be rejected");
    assert!(
        err.to_string().contains("weight > 0.0"),
        "unexpected error: {}",
        err
    );
}

#[test]
fn parse_opponent_encounters_accepts_positive_weight_mix() {
    let champion_bases = load_champion_bases().expect("champion data should load");
    let champion_name = champion_bases
        .values()
        .next()
        .map(|c| c.name.clone())
        .expect("at least one champion should exist");
    let scenario = json!({
        "opponents": {
            "encounters": [
                {
                    "name": "zero_ok",
                    "weight": 0.0,
                    "actors": [{ "champion": champion_name }]
                },
                {
                    "name": "positive",
                    "weight": 1.0,
                    "actors": [{ "champion": champion_name }]
                }
            ]
        }
    });
    let encounters = parse_opponent_encounters(&scenario, &champion_bases, 18, &HashMap::new())
        .expect("encounters should parse when at least one encounter weight is positive");
    assert_eq!(encounters.len(), 2);
}

#[test]
fn parse_scenario_search_or_default_uses_portfolio_when_missing() {
    let scenario = json!({});
    let parsed = parse_scenario_search_or_default(&scenario)
        .expect("missing scenario.search should fall back to default search config");
    assert_eq!(parsed.strategy, "portfolio");
}

#[test]
fn parse_scenario_search_or_default_preserves_explicit_search() {
    let scenario = json!({
        "search": {
            "strategy": "beam",
            "beam_width": 7
        }
    });
    let parsed =
        parse_scenario_search_or_default(&scenario).expect("explicit scenario.search should parse");
    assert_eq!(parsed.strategy, "beam");
    assert_eq!(parsed.beam_width, 7);
}

#[test]
fn unique_loadout_selection_count_helpers_track_distinct_loadouts() {
    let base = BuildKey {
        item_indices: vec![1, 2, 3, 4, 5, 6],
        loadout_selection: LoadoutSelection {
            rune_names: vec![
                "Conqueror".to_string(),
                "Triumph".to_string(),
                "Legend: Alacrity".to_string(),
                "Last Stand".to_string(),
                "Second Wind".to_string(),
                "Unflinching".to_string(),
            ],
            shard_stats: vec![
                "ability_haste".to_string(),
                "movement_speed".to_string(),
                "health".to_string(),
            ],
        },
    };
    let mut other = base.clone();
    other.loadout_selection.rune_names[0] = "Press the Attack".to_string();
    let mut duplicate = base.clone();
    duplicate.item_indices = vec![6, 5, 4, 3, 2, 1];

    let candidates = vec![base.clone(), other.clone(), duplicate.clone()];
    assert_eq!(
        unique_loadout_selection_count(&candidates),
        2,
        "duplicate item permutations should not inflate loadout candidate count"
    );

    let ranked = vec![(base, 1.0), (other, 0.9), (duplicate, 0.8)];
    assert_eq!(
        unique_loadout_selection_count_from_ranked(&ranked),
        2,
        "ranked entries should count unique loadout pages"
    );
}

fn test_item(name: &str, boots: bool) -> Item {
    Item {
        name: name.to_string(),
        stats: Stats::default(),
        rank: if boots {
            vec!["BOOTS".to_string()]
        } else {
            Vec::new()
        },
        shop_purchasable: true,
        total_cost: 0.0,
        passive_effects_text: Vec::new(),
        has_active_effect: false,
        structured_effect_count: 0,
    }
}

#[test]
fn max_legal_build_size_enforces_single_boot_slot() {
    let items = vec![
        test_item("Boots A", true),
        test_item("Boots B", true),
        test_item("Item 1", false),
        test_item("Item 2", false),
        test_item("Item 3", false),
    ];
    assert_eq!(
        max_legal_build_size(&items),
        4,
        "pool with two boots and three non-boots can only fill four legal slots"
    );
}

#[test]
fn filter_item_pool_to_modeled_runtime_effects_drops_unmodeled_effect_items() {
    let mut modeled_runtime_item = test_item("Zhonya's Hourglass", false);
    modeled_runtime_item.has_active_effect = true;

    let mut unmodeled_runtime_item = test_item("Unmodeled Runtime Item", false);
    unmodeled_runtime_item.passive_effects_text = vec!["Deals bonus magic damage.".to_string()];

    let stat_only_item = test_item("Stat Stick", false);
    let filtered = filter_item_pool_to_modeled_runtime_effects(&[
        modeled_runtime_item,
        unmodeled_runtime_item,
        stat_only_item,
    ]);
    let names = filtered
        .iter()
        .map(|item| item.name.as_str())
        .collect::<HashSet<_>>();

    assert!(
        names.contains("Zhonya's Hourglass"),
        "known modeled runtime effect item should remain"
    );
    assert!(
        names.contains("Stat Stick"),
        "stat-only items should remain legal because they have no runtime-effect surface"
    );
    assert!(
        !names.contains("Unmodeled Runtime Item"),
        "hard-gated unmodeled runtime effect items should be removed from generation pool"
    );
}

#[test]
fn level_scaled_defaults_recompute_after_controlled_level_override() {
    let simulation_config = json!({ "champion_level": 1 });
    let mut sim = parse_simulation_config(&simulation_config).expect("simulation should parse");
    let before_bonus = sim.protoplasm_bonus_health;
    let before_heal = sim.protoplasm_heal_total;
    let previous_level = sim.champion_level;
    sim.champion_level = 30;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        &simulation_config,
        previous_level,
    );
    assert!(sim.protoplasm_bonus_health > before_bonus);
    assert!(sim.protoplasm_heal_total > before_heal);
}

#[test]
fn level_scaled_defaults_do_not_override_explicit_simulation_values() {
    let simulation_config = json!({
        "champion_level": 1,
        "protoplasm_bonus_health": 777.0,
        "protoplasm_heal_total": 444.0
    });
    let mut sim = parse_simulation_config(&simulation_config).expect("simulation should parse");
    let previous_level = sim.champion_level;
    sim.champion_level = 30;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        &simulation_config,
        previous_level,
    );
    assert!((sim.protoplasm_bonus_health - 777.0).abs() < f64::EPSILON);
    assert!((sim.protoplasm_heal_total - 444.0).abs() < f64::EPSILON);
}

#[test]
fn time_budget_deadline_arms_only_after_non_coverage_simulation_in_maximum_quality() {
    let hard_deadline_state = Arc::new(Mutex::new(None));
    arm_time_budget_deadline_if_unset(
        &hard_deadline_state,
        Some(Duration::from_secs(1)),
        true,
        "coverage_stage",
    );
    assert!(
        hard_deadline_state
            .lock()
            .ok()
            .and_then(|state| *state)
            .is_none()
    );

    arm_time_budget_deadline_if_unset(
        &hard_deadline_state,
        Some(Duration::from_secs(1)),
        true,
        "seed_search:portfolio",
    );
    assert!(
        hard_deadline_state
            .lock()
            .ok()
            .and_then(|state| *state)
            .is_some()
    );
}

#[test]
fn time_budget_deadline_arms_on_coverage_simulation_when_not_deferred() {
    let hard_deadline_state = Arc::new(Mutex::new(None));
    arm_time_budget_deadline_if_unset(
        &hard_deadline_state,
        Some(Duration::from_secs(1)),
        false,
        "coverage_stage",
    );
    assert!(
        hard_deadline_state
            .lock()
            .ok()
            .and_then(|state| *state)
            .is_some()
    );
}

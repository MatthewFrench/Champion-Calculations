use super::*;
use serde_json::Value;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[test]
fn cache_seed_partition_uses_shared_bucket_for_runtime_random_seed() {
    let partition = persistent_cache_seed_partition(0, None, 987_654_321);
    assert_eq!(partition, 0);
}

#[test]
fn cache_seed_partition_uses_effective_seed_for_configured_seed() {
    let partition = persistent_cache_seed_partition(42, None, 42);
    assert_eq!(partition, 42);
}

#[test]
fn cache_seed_partition_uses_effective_seed_for_cli_override() {
    let partition = persistent_cache_seed_partition(0, Some(123), 123);
    assert_eq!(partition, 123);
}

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

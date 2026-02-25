use super::*;
use crate::data::{LoadoutDomain, RunePathDomain};
use serde_json::Value;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
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

#[test]
fn resolve_controlled_champion_script_or_error_accepts_registered_champion() {
    let script = resolve_controlled_champion_script_or_error("Vladimir")
        .expect("registered controlled champion should resolve script");
    let cast_profile = script.default_cast_profile();
    assert!(
        !cast_profile.offensive_primary_ability_id.is_empty(),
        "resolved script should provide a non-empty offensive ability identifier"
    );
}

#[test]
fn resolve_controlled_champion_script_or_error_accepts_second_registered_champion() {
    let script = resolve_controlled_champion_script_or_error("Sona")
        .expect("second registered controlled champion should resolve script");
    let cast_profile = script.default_cast_profile();
    assert_eq!(
        cast_profile.offensive_ultimate_ability_id, "crescendo",
        "Sona controlled script should expose Crescendo as ultimate ability identity"
    );
}

#[test]
fn resolve_controlled_champion_script_or_error_rejects_unregistered_champion() {
    let champion_bases = load_champion_bases().expect("champion data should load");
    let supported_champions =
        crate::scripts::champions::supported_controlled_champion_script_keys()
            .iter()
            .map(|key| key.to_string())
            .collect::<HashSet<_>>();
    let unsupported_champion = champion_bases
        .values()
        .find(|champion| !supported_champions.contains(&crate::to_norm_key(&champion.name)))
        .map(|champion| champion.name.clone())
        .expect("test requires at least one champion without controlled script coverage");
    let error = resolve_controlled_champion_script_or_error(&unsupported_champion)
        .expect_err("unregistered controlled champion should fail fast");
    assert!(
        error
            .to_string()
            .contains("has no registered controlled-champion script"),
        "unexpected error: {}",
        error
    );
    assert!(
        error.to_string().contains("Supported controlled champions"),
        "unexpected error: {}",
        error
    );
}

#[test]
fn validate_world_positions_for_enemy_scenarios_rejects_out_of_bounds_positions() {
    let champion_bases = load_champion_bases().expect("champion data should load");
    let champion_name = champion_bases
        .values()
        .next()
        .map(|champion| champion.name.clone())
        .expect("at least one champion should exist");
    let scenario = json!({
        "opponents": {
            "encounters": [
                {
                    "name": "invalid_world_position",
                    "weight": 1.0,
                    "actors": [{
                        "id": "enemy_out_of_bounds",
                        "champion": champion_name,
                        "placement": {
                            "position": { "x": 9000.0, "y": 0.0 },
                            "movement": "hold_position"
                        }
                    }]
                }
            ]
        }
    });
    let encounters = parse_opponent_encounters(&scenario, &champion_bases, 18, &HashMap::new())
        .expect("encounters should parse before world validation");
    let enemy_scenarios = encounters
        .iter()
        .map(|encounter| {
            (
                encounter.name.clone(),
                encounter.weight,
                encounter.actors.clone(),
            )
        })
        .collect::<Vec<_>>();
    let err = validate_world_positions_for_enemy_scenarios("Vladimir", &enemy_scenarios)
        .expect_err("out-of-bounds encounter positions should fail world validation");
    assert!(
        err.to_string().contains("outside map bounds"),
        "unexpected error: {}",
        err
    );
}

#[test]
fn controlled_champion_stepper_runs_non_vladimir_registered_scenario() {
    let scenario_path = resolve_scenario_path("sona_urf_teamfight");
    run_controlled_champion_stepper(&scenario_path, 1)
        .expect("Sona controlled champion stepper scenario should run");
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

fn assert_determinism_json_shape(entry: &Value) {
    assert!(
        entry
            .get("final_state_checksum_hex")
            .and_then(Value::as_str)
            .is_some()
    );
    assert!(
        entry
            .get("tick_state_checksum_hex")
            .and_then(Value::as_str)
            .is_some()
    );
    assert!(
        entry
            .get("queue_checksum_hex")
            .and_then(Value::as_str)
            .is_some()
    );
    assert!(
        entry
            .get("ticks_executed")
            .and_then(Value::as_u64)
            .is_some()
    );
    assert!(
        entry
            .get("events_processed")
            .and_then(Value::as_u64)
            .is_some()
    );
}

#[test]
fn fixed_loadout_report_json_contract_schema_and_telemetry_shape() {
    assert_eq!(FIXED_LOADOUT_REPORT_JSON_SCHEMA_VERSION, 3);
    let entry = sample_rune_proc_telemetry_entry();
    let telemetry_json = rune_proc_telemetry_json(&[entry], 900.0, 100.0);
    assert_eq!(telemetry_json.len(), 1);
    assert_rune_telemetry_json_shape(&telemetry_json[0]);
    let sample_determinism = json!({
        "final_state_checksum_hex": "00",
        "tick_state_checksum_hex": "00",
        "queue_checksum_hex": "00",
        "ticks_executed": 1,
        "events_processed": 2
    });
    assert_determinism_json_shape(&sample_determinism);
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
    assert_eq!(FIXED_LOADOUT_TRACE_JSON_SCHEMA_VERSION, 4);
    assert_eq!(CONTROLLED_CHAMPION_TRACE_JSON_SCHEMA_VERSION, 3);
    let entry = sample_rune_proc_telemetry_entry();
    let telemetry_json = rune_proc_telemetry_json(&[entry], 1000.0, 250.0);
    assert_eq!(telemetry_json.len(), 1);
    assert_rune_telemetry_json_shape(&telemetry_json[0]);
    let sample_determinism = json!({
        "final_state_checksum_hex": "00",
        "tick_state_checksum_hex": "00",
        "queue_checksum_hex": "00",
        "ticks_executed": 1,
        "events_processed": 2
    });
    assert_determinism_json_shape(&sample_determinism);
}

#[test]
fn parse_controlled_champion_config_rejects_legacy_baseline_items_key() {
    let champion_bases = load_champion_bases().expect("champion data should load");
    let champion_name = champion_bases
        .values()
        .next()
        .map(|champion| champion.name.clone())
        .expect("at least one champion should exist");
    let scenario = json!({
        "controlled_champion": {
            "champion": champion_name,
            "baseline_items": ["Amplifying Tome"]
        }
    });
    let result = parse_controlled_champion_config(&scenario, &champion_bases, 18, &HashMap::new());
    assert!(
        result.is_err(),
        "legacy baseline_items key should be rejected"
    );
    let err = result
        .err()
        .expect("result should contain parse error for baseline_items");
    assert!(
        err.to_string()
            .contains("baseline_items is no longer supported"),
        "unexpected error: {}",
        err
    );
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
fn parse_opponent_encounters_preserves_typed_encounter_fields() {
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
                    "name": "lane_trade",
                    "weight": 0.75,
                    "actors": [{ "champion": champion_name }]
                }
            ]
        }
    });
    let encounters = parse_opponent_encounters(&scenario, &champion_bases, 18, &HashMap::new())
        .expect("encounters should parse");
    assert_eq!(encounters.len(), 1);
    assert_eq!(encounters[0].name, "lane_trade");
    assert!(
        (encounters[0].weight - 0.75).abs() < 1e-9,
        "unexpected weight: {}",
        encounters[0].weight
    );
    assert_eq!(encounters[0].actors.len(), 1);
}

#[test]
fn parse_opponent_encounters_rejects_duplicate_actor_ids_across_encounters() {
    let champion_bases = load_champion_bases().expect("champion data should load");
    let champion_names = champion_bases
        .values()
        .map(|champion| champion.name.clone())
        .collect::<Vec<_>>();
    assert!(
        champion_names.len() >= 2,
        "need at least two champions to validate duplicate-id collision handling"
    );
    let scenario = json!({
        "opponents": {
            "encounters": [
                {
                    "name": "encounter_a",
                    "weight": 1.0,
                    "actors": [{ "id": "frontline", "champion": champion_names[0] }]
                },
                {
                    "name": "encounter_b",
                    "weight": 1.0,
                    "actors": [{ "id": "frontline", "champion": champion_names[1] }]
                }
            ]
        }
    });
    let err = parse_opponent_encounters(&scenario, &champion_bases, 18, &HashMap::new())
        .expect_err("duplicate actor IDs should be rejected to keep actor-ID keyed caches safe");
    assert!(
        err.to_string()
            .contains("actor IDs must map to a single champion identity"),
        "unexpected error: {}",
        err
    );
}

#[test]
fn default_run_output_directory_compacts_popcorn_window_when_equal_to_budget() {
    let output_dir =
        default_run_output_directory(SearchQualityProfile::Fast, Some(300.0), Some(300.0), 5.0);
    let expected_suffix = PathBuf::from("output")
        .join("runs")
        .join("controlled_champion")
        .join("fast")
        .join("300s__popcorn__min_improvement_5pct");
    assert!(
        output_dir.ends_with(&expected_suffix),
        "unexpected output dir: {}",
        output_dir.display()
    );
}

#[test]
fn default_fixed_loadout_output_directory_normalizes_label_key() {
    let output_dir =
        default_fixed_loadout_output_directory(SearchQualityProfile::Balanced, "My Label");
    let expected_suffix = PathBuf::from("output")
        .join("runs")
        .join("controlled_champion")
        .join("fixed_loadout")
        .join("balanced")
        .join("mylabel");
    assert!(
        output_dir.ends_with(&expected_suffix),
        "unexpected output dir: {}",
        output_dir.display()
    );
}

#[test]
fn format_repo_relative_path_uses_repository_relative_simulation_paths() {
    let absolute_path = simulation_dir().join("output").join("test.md");
    assert_eq!(
        format_repo_relative_path(&absolute_path),
        "Simulation/output/test.md"
    );
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

#[test]
fn search_type_counter_helpers_dedupe_keys_and_report_touched_entries_only() {
    let active_strategies = vec!["beam".to_string(), "random".to_string(), "beam".to_string()];
    let counters = initialize_search_type_counters(&active_strategies, "beam");
    assert!(
        counters.contains_key("seed_search:beam"),
        "configured strategy counter should exist"
    );
    assert!(
        counters.contains_key("seed_search:random"),
        "active strategy counter should exist"
    );

    let initial_snapshot = snapshot_search_type_counters(counters.as_ref());
    assert!(
        initial_snapshot.is_empty(),
        "untouched counters should be omitted from snapshot output"
    );

    increment_search_type_counter(counters.as_ref(), "seed_search:beam", 3, 2);
    increment_search_type_counter(counters.as_ref(), "unknown_counter_key", 7, 7);
    let touched_snapshot = snapshot_search_type_counters(counters.as_ref());
    assert_eq!(
        touched_snapshot.len(),
        1,
        "only touched counters should be included"
    );
    assert_eq!(touched_snapshot[0].name, "seed_search:beam");
    assert_eq!(touched_snapshot[0].score_requests, 3);
    assert_eq!(touched_snapshot[0].new_simulations, 2);
}

#[test]
fn estimated_legal_item_build_count_applies_single_boot_constraint() {
    let items = vec![
        test_item("Boots A", true),
        test_item("Boots B", true),
        test_item("Item 1", false),
        test_item("Item 2", false),
        test_item("Item 3", false),
    ];

    let estimated = estimated_legal_item_build_count(&items, 2);
    assert_eq!(
        estimated, 9.0,
        "legal two-item combinations should enforce at most one boots item"
    );
}

#[test]
fn estimated_legal_loadout_count_matches_small_domain_combinatorics() {
    let domain = LoadoutDomain {
        rune_paths: vec![
            RunePathDomain {
                slot_runes: vec![
                    vec!["A keystone 1".to_string(), "A keystone 2".to_string()],
                    vec!["A slot 1".to_string()],
                    vec!["A slot 2".to_string()],
                    vec!["A slot 3".to_string()],
                ],
            },
            RunePathDomain {
                slot_runes: vec![
                    vec!["B keystone".to_string()],
                    vec!["B slot 1a".to_string(), "B slot 1b".to_string()],
                    vec!["B slot 2".to_string()],
                    vec!["B slot 3".to_string()],
                ],
            },
        ],
        shard_slots: [
            vec!["adaptive".to_string(), "ability_haste".to_string()],
            vec!["adaptive".to_string()],
            vec!["health".to_string()],
        ],
    };

    assert_eq!(
        estimated_legal_loadout_count(&domain),
        32.0,
        "small loadout-domain combinatorics should remain stable"
    );
}

#[test]
fn estimate_close_to_optimal_probability_reports_unavailable_when_space_missing() {
    let (probability, note) = estimate_close_to_optimal_probability(12, None);
    assert!(
        probability.is_none(),
        "missing candidate-space estimate should produce unavailable probability"
    );
    assert!(
        note.contains("not finite"),
        "note should explain unavailable finite-space estimate"
    );
}

#[test]
fn format_percent_display_uses_scientific_notation_for_tiny_percent_values() {
    assert_eq!(format_percent_display(0.0000005), "5.000e-7%");
    assert_eq!(format_percent_display(1.23456789), "1.234568%");
}

fn test_candidate(item_idx: usize, rune_key: &str, shard_key: &str) -> BuildKey {
    BuildKey {
        item_indices: vec![item_idx],
        loadout_selection: LoadoutSelection {
            rune_names: vec![rune_key.to_string()],
            shard_stats: vec![shard_key.to_string()],
        },
    }
}

#[test]
fn strict_ranking_heuristic_ordering_sorts_by_signal_when_enabled_without_promotions() {
    let low = test_candidate(0, "Low Rune", "Low Shard");
    let mid = test_candidate(1, "Mid Rune", "Mid Shard");
    let high = test_candidate(2, "High Rune", "High Shard");

    let mut strict_scores = HashMap::new();
    strict_scores.insert(low.clone(), 1.0);
    strict_scores.insert(mid.clone(), 5.0);
    strict_scores.insert(high.clone(), 9.0);

    let remaining = vec![mid.clone(), low.clone(), high.clone()];
    let (sorted, promotions_done) = heuristic_sort_remaining_candidates_for_strict_ranking(
        remaining,
        &strict_scores,
        3,
        0.0,
        0.0,
        1337,
        0,
    );

    assert_eq!(promotions_done, 0);
    assert_eq!(sorted.first(), Some(&high));
    assert_eq!(sorted.last(), Some(&low));
}

#[test]
fn strict_ranking_heuristic_ordering_keeps_input_order_when_scores_are_flat() {
    let first = test_candidate(0, "Rune A", "Shard A");
    let second = test_candidate(1, "Rune B", "Shard B");

    let mut strict_scores = HashMap::new();
    strict_scores.insert(first.clone(), 4.0);
    strict_scores.insert(second.clone(), 4.0);

    let input_order = vec![second.clone(), first.clone()];
    let (sorted, promotions_done) = heuristic_sort_remaining_candidates_for_strict_ranking(
        input_order.clone(),
        &strict_scores,
        2,
        1.0,
        1.0,
        4242,
        0,
    );

    assert_eq!(promotions_done, 0);
    assert_eq!(
        sorted, input_order,
        "flat strict scores should not introduce heuristic reordering"
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

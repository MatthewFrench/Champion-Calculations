use super::*;

#[test]
fn report_path_uses_normalized_champion_key() {
    let path = default_report_path_for_champion("Dr. Mundo");
    let path_text = path.to_string_lossy();
    assert!(path_text.ends_with("output/drmundo_run_report.md"));
}

#[test]
fn controlled_champion_loadout_labels_require_full_rune_page_and_shards() {
    let missing = vec!["Rune: Arcane Comet".to_string()];
    assert!(validate_controlled_champion_selection_labels("Vladimir", &missing).is_err());

    let mut complete = vec![
        "Rune: Arcane Comet".to_string(),
        "Rune: Manaflow Band".to_string(),
        "Rune: Transcendence".to_string(),
        "Rune: Gathering Storm".to_string(),
        "Rune: Cheap Shot".to_string(),
        "Rune: Ultimate Hunter".to_string(),
        "Shard 1: adaptive".to_string(),
        "Shard 2: movement_speed".to_string(),
        "Shard 3: health".to_string(),
    ];
    assert!(validate_controlled_champion_selection_labels("Vladimir", &complete).is_ok());

    complete.pop();
    assert!(validate_controlled_champion_selection_labels("Vladimir", &complete).is_err());
}

#[test]
fn run_report_json_contract_has_schema_version_and_rune_telemetry_shape() {
    let telemetry = vec![
        crate::scripts::runtime::loadout_runtime::RuneProcTelemetryEntry {
            rune_name: "Arcane Comet".to_string(),
            proc_count: 3,
            attempt_count: 5,
            eligible_count: 4,
            proc_attempt_rate: 0.6,
            proc_eligible_rate: 0.75,
            bonus_damage: 120.0,
            bonus_healing: 0.0,
            source_breakdown: vec![
                crate::scripts::runtime::loadout_runtime::RuneProcTelemetrySourceEntry {
                    source: "ability".to_string(),
                    proc_count: 3,
                    attempt_count: 5,
                    eligible_count: 4,
                    proc_attempt_rate: 0.6,
                    proc_eligible_rate: 0.75,
                    bonus_damage: 120.0,
                    bonus_healing: 0.0,
                },
            ],
        },
    ];
    let json_entries = report_rune_proc_telemetry_json(&telemetry, 500.0, 100.0);
    assert_eq!(CONTROLLED_CHAMPION_RUN_REPORT_JSON_SCHEMA_VERSION, 2);
    assert_eq!(json_entries.len(), 1);
    let entry = &json_entries[0];
    assert_eq!(entry.get("attempt_count").and_then(|v| v.as_u64()), Some(5));
    assert_eq!(
        entry.get("eligible_count").and_then(|v| v.as_u64()),
        Some(4)
    );
    assert!(
        entry
            .get("proc_attempt_rate")
            .and_then(|v| v.as_f64())
            .is_some()
    );
    assert!(
        entry
            .get("proc_eligible_rate")
            .and_then(|v| v.as_f64())
            .is_some()
    );
    assert_eq!(
        entry.get("opportunity_count").and_then(|v| v.as_u64()),
        Some(4)
    );
    assert!(
        entry
            .get("source_breakdown")
            .and_then(|v| v.as_array())
            .and_then(|entries| entries.first())
            .and_then(|source| source.get("attempt_count"))
            .and_then(|v| v.as_u64())
            .is_some()
    );
}

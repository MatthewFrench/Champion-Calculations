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

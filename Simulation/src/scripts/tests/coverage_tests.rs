use super::*;
use crate::Stats;

fn test_item(
    name: &str,
    passive_effects_text: Vec<String>,
    has_active_effect: bool,
    structured_effect_count: usize,
) -> Item {
    Item {
        name: name.to_string(),
        stats: Stats::default(),
        rank: vec!["LEGENDARY".to_string()],
        shop_purchasable: true,
        total_cost: 3000.0,
        passive_effects_text,
        has_active_effect,
        structured_effect_count,
    }
}

#[test]
fn modeled_item_effect_registry_contains_expected_entries() {
    assert!(has_modeled_runtime_item_effect("Luden's Echo"));
    assert!(has_modeled_runtime_item_effect("Ludens Companion"));
    assert!(has_modeled_runtime_item_effect("Kraken Slayer"));
    assert!(!has_modeled_runtime_item_effect("Rabadon's Deathcap"));
}

#[test]
fn unmodeled_item_effect_detection_requires_effect_data() {
    let plain_stat_item = test_item("Abyssal Mask", Vec::new(), false, 0);
    assert!(
        !is_item_effect_unmodeled(&plain_stat_item),
        "items with no effect payload should not be flagged by runtime-effect gate"
    );

    let unmodeled_passive_item = test_item(
        "Abyssal Mask",
        vec!["Nearby enemies take increased magic damage.".to_string()],
        false,
        0,
    );
    assert!(is_item_effect_unmodeled(&unmodeled_passive_item));
}

#[test]
fn unmodeled_item_effect_names_are_deduplicated_and_sorted() {
    let items = vec![
        test_item("Abyssal Mask", vec!["Aura".to_string()], false, 1),
        test_item("Kraken Slayer", vec!["Every 3rd hit".to_string()], false, 1),
        test_item("Abyssal Mask", vec!["Aura".to_string()], false, 1),
        test_item("Rabadon's Deathcap", Vec::new(), false, 1),
    ];
    let names = unmodeled_runtime_item_effect_names(&items);
    assert_eq!(
        names,
        vec!["Abyssal Mask".to_string(), "Rabadon's Deathcap".to_string()]
    );
}

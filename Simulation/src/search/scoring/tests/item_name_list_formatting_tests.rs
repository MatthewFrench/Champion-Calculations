use super::*;
use crate::Stats;

fn test_item(name: &str) -> Item {
    Item {
        name: name.to_string(),
        stats: Stats::default(),
        rank: Vec::new(),
        shop_purchasable: true,
        total_cost: 0.0,
        passive_effects_text: Vec::new(),
        has_active_effect: false,
        structured_effect_count: 0,
    }
}

#[test]
fn format_item_name_list_comma_separated_returns_empty_for_empty_list() {
    let formatted = format_item_name_list_comma_separated(&[]);

    assert!(formatted.is_empty());
}

#[test]
fn format_item_name_list_comma_separated_returns_single_name_without_separator() {
    let formatted = format_item_name_list_comma_separated(&[test_item("Rabadon's Deathcap")]);

    assert_eq!(formatted, "Rabadon's Deathcap");
}

#[test]
fn format_item_name_list_comma_separated_joins_multiple_names_in_order() {
    let formatted = format_item_name_list_comma_separated(&[
        test_item("Boots of Swiftness"),
        test_item("Cosmic Drive"),
        test_item("Riftmaker"),
    ]);

    assert_eq!(formatted, "Boots of Swiftness, Cosmic Drive, Riftmaker");
}

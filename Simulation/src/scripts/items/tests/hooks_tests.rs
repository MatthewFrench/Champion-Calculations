
use super::*;
use crate::Stats;

fn test_item(name: &str) -> Item {
    Item {
        name: name.to_string(),
        stats: Stats::default(),
        rank: vec!["LEGENDARY".to_string()],
        shop_purchasable: true,
        total_cost: 3000.0,
        passive_effects_text: Vec::new(),
    }
}

#[test]
fn defensive_item_capabilities_detect_supported_items() {
    let build_items = vec![
        test_item("Zhonya's Hourglass"),
        test_item("Guardian Angel"),
        test_item("Protoplasm Harness"),
    ];
    let capabilities = controlled_champion_defensive_item_capabilities(&build_items);
    assert!(capabilities.has_stasis_item);
    assert!(capabilities.has_revive_item);
    assert!(capabilities.has_emergency_shield_item);
}

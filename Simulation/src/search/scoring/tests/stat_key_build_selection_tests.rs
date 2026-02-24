use std::collections::HashSet;

use super::*;
use crate::Stats;

fn test_item(name: &str, ability_power: f64, boots: bool) -> Item {
    Item {
        name: name.to_string(),
        stats: Stats {
            ability_power,
            ..Stats::default()
        },
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
fn choose_best_build_by_stat_returns_empty_when_no_slots_requested() {
    let item_pool = vec![test_item("Item A", 30.0, false)];

    let best = choose_best_build_by_stat(&item_pool, "ability_power", 0, 8);

    assert!(best.is_empty());
}

#[test]
fn choose_best_build_by_stat_respects_single_boot_constraint() {
    let item_pool = vec![
        test_item("Boots A", 10.0, true),
        test_item("Boots B", 20.0, true),
        test_item("Item A", 40.0, false),
        test_item("Item B", 35.0, false),
    ];

    let best = choose_best_build_by_stat(&item_pool, "ability_power", 3, 8);

    assert_eq!(best.len(), 3);
    let unique = best.iter().copied().collect::<HashSet<_>>();
    assert_eq!(unique.len(), best.len());
    let boots_count = best
        .iter()
        .filter(|idx| item_pool[**idx].rank.iter().any(|rank| rank == "BOOTS"))
        .count();
    assert!(boots_count <= 1);
}

#[test]
fn choose_best_build_by_stat_picks_highest_stat_bundle() {
    let item_pool = vec![
        test_item("Boots A", 10.0, true),
        test_item("Item A", 50.0, false),
        test_item("Item B", 40.0, false),
        test_item("Item C", 5.0, false),
    ];

    let best = choose_best_build_by_stat(&item_pool, "ability_power", 3, 8);
    let total = best
        .iter()
        .map(|idx| item_pool[*idx].stats.ability_power)
        .sum::<f64>();

    assert_eq!(total, 100.0);
}

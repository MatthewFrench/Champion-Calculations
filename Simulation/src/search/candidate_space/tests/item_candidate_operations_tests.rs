use std::collections::HashSet;

use super::*;
use crate::{Item, Stats};

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

fn test_pool() -> Vec<Item> {
    vec![
        test_item("Boots A", true),
        test_item("Boots B", true),
        test_item("Item 1", false),
        test_item("Item 2", false),
        test_item("Item 3", false),
        test_item("Item 4", false),
    ]
}

fn assert_legal_build(item_pool: &[Item], build: &[usize], max_items: usize) {
    assert!(build.len() <= max_items);
    assert!(build.iter().all(|idx| *idx < item_pool.len()));
    let unique = build.iter().copied().collect::<HashSet<_>>();
    assert_eq!(unique.len(), build.len());
    let boots_count = build
        .iter()
        .filter(|idx| item_pool[**idx].rank.iter().any(|rank| rank == "BOOTS"))
        .count();
    assert!(boots_count <= 1);
}

#[test]
fn crossover_builds_produces_legal_child() {
    let item_pool = test_pool();
    let parent_a = vec![0, 2, 3];
    let parent_b = vec![1, 4, 2];
    let mut seed = 1337u64;

    let child = crossover_builds(&parent_a, &parent_b, &item_pool, 4, &mut seed);

    assert_legal_build(&item_pool, &child, 4);
}

#[test]
fn mutate_build_with_zero_rate_keeps_original_build() {
    let item_pool = test_pool();
    let mut build = vec![0, 2, 3];
    let original = build.clone();
    let mut seed = 7u64;

    mutate_build(&mut build, &item_pool, 4, 0.0, &mut seed);

    assert_eq!(build, original);
}

#[test]
fn mutate_build_preserves_build_legality() {
    let item_pool = test_pool();
    let mut build = vec![0, 2, 3];
    let mut seed = 17u64;

    mutate_build(&mut build, &item_pool, 4, 1.0, &mut seed);

    assert_legal_build(&item_pool, &build, 4);
}

#[test]
fn tournament_parent_returns_existing_candidate() {
    let scored_population = vec![
        (vec![0, 2, 3], 1.0),
        (vec![1, 4, 5], 2.0),
        (vec![0, 4, 5], 3.0),
    ];
    let mut seed = 19u64;

    let parent = tournament_parent(&scored_population, &mut seed, 3);

    assert!(scored_population.iter().any(|(build, _)| *build == parent));
}

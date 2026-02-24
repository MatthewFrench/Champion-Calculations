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
    ]
}

fn assert_legal_key(item_pool: &[Item], key: &[usize], max_items: usize) {
    assert!(key.len() <= max_items);
    assert!(key.iter().all(|idx| *idx < item_pool.len()));
    assert!(key.windows(2).all(|pair| pair[0] <= pair[1]));
    let unique = key.iter().copied().collect::<HashSet<_>>();
    assert_eq!(unique.len(), key.len());
    let boots_count = key
        .iter()
        .filter(|idx| item_pool[**idx].rank.iter().any(|rank| rank == "BOOTS"))
        .count();
    assert!(boots_count <= 1);
}

#[test]
fn available_actions_respects_boot_and_duplicate_constraints() {
    let item_pool = test_pool();
    let build = vec![0, 2];

    let actions = available_actions(&item_pool, &build);
    let action_set = actions.into_iter().collect::<HashSet<_>>();

    assert!(action_set.contains(&3));
    assert!(action_set.contains(&4));
    assert!(!action_set.contains(&0));
    assert!(!action_set.contains(&1));
    assert!(!action_set.contains(&2));
}

#[test]
fn rollout_completion_returns_legal_canonical_key_and_expected_score() {
    let item_pool = test_pool();
    let mut seed = 1337u64;
    let (key, score) = rollout_completion(
        &item_pool,
        3,
        &[3],
        &mut seed,
        &|candidate: &[usize]| candidate.iter().sum::<usize>() as f64,
        None,
    );

    assert_legal_key(&item_pool, &key, 3);
    assert_eq!(score, key.iter().sum::<usize>() as f64);
}

#[test]
fn mcts_search_ranked_returns_unique_legal_candidates_with_limit() {
    let item_pool = test_pool();
    let ranked = mcts_search_ranked(
        &item_pool,
        3,
        &MctsSearchConfig {
            iterations: 24,
            rollouts_per_expansion: 2,
            exploration: 1.4,
            seed: 7,
            limit: 3,
        },
        &|candidate: &[usize]| candidate.iter().sum::<usize>() as f64,
        None,
    );

    assert!(!ranked.is_empty());
    assert!(ranked.len() <= 3);
    let unique_keys = ranked
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<HashSet<_>>();
    assert_eq!(unique_keys.len(), ranked.len());
    for (key, _) in ranked {
        assert_legal_key(&item_pool, &key, 3);
    }
}

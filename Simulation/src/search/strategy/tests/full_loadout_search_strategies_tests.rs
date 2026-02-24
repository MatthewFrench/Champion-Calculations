use std::collections::HashSet;

use super::super::super::{FullLoadoutSearchParams, MctsSearchConfig};
use super::*;
use crate::data::{LoadoutDomain, RunePathDomain};
use crate::{BuildKey, Item, LoadoutSelection, Stats};

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

fn test_loadout_domain() -> LoadoutDomain {
    LoadoutDomain {
        rune_paths: vec![
            RunePathDomain {
                slot_runes: vec![
                    vec!["Keystone A".to_string()],
                    vec!["A1".to_string()],
                    vec!["A2".to_string()],
                    vec!["A3".to_string()],
                ],
            },
            RunePathDomain {
                slot_runes: vec![
                    vec!["Keystone B".to_string()],
                    vec!["B1".to_string()],
                    vec!["B2".to_string()],
                    vec!["B3".to_string()],
                ],
            },
        ],
        shard_slots: [
            vec!["adaptive".to_string()],
            vec!["adaptive".to_string()],
            vec!["health".to_string()],
        ],
    }
}

fn test_base_loadout() -> LoadoutSelection {
    LoadoutSelection {
        rune_names: vec![
            "Keystone A".to_string(),
            "A1".to_string(),
            "A2".to_string(),
            "A3".to_string(),
            "B1".to_string(),
            "B2".to_string(),
        ],
        shard_stats: vec![
            "adaptive".to_string(),
            "adaptive".to_string(),
            "health".to_string(),
        ],
    }
}

fn assert_legal_candidate(item_pool: &[Item], candidate: &BuildKey, max_items: usize) {
    assert!(candidate.item_indices.len() <= max_items);
    assert!(
        candidate
            .item_indices
            .iter()
            .all(|idx| *idx < item_pool.len())
    );
    assert!(
        candidate
            .item_indices
            .windows(2)
            .all(|pair| pair[0] <= pair[1])
    );
    let unique = candidate
        .item_indices
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    assert_eq!(unique.len(), candidate.item_indices.len());
    let boots_count = candidate
        .item_indices
        .iter()
        .filter(|idx| item_pool[**idx].rank.iter().any(|rank| rank == "BOOTS"))
        .count();
    assert!(boots_count <= 1);
    assert_eq!(candidate.loadout_selection.rune_names.len(), 6);
    assert_eq!(candidate.loadout_selection.shard_stats.len(), 3);
}

#[test]
fn random_search_ranked_full_respects_limit_and_candidate_legality() {
    let item_pool = test_pool();
    let domain = test_loadout_domain();
    let base_loadout = test_base_loadout();
    let params = FullLoadoutSearchParams {
        item_pool: &item_pool,
        max_items: 3,
        loadout_domain: &domain,
        base_loadout: &base_loadout,
    };

    let ranked = random_search_ranked_full(
        &params,
        24,
        19,
        5,
        &|candidate: &BuildKey| candidate.item_indices.iter().sum::<usize>() as f64,
        None,
    );

    assert!(!ranked.is_empty());
    assert!(ranked.len() <= 5);
    for (candidate, _) in ranked {
        assert_legal_candidate(&item_pool, &candidate, 3);
    }
}

#[test]
fn beam_search_ranked_full_returns_legal_candidates() {
    let item_pool = test_pool();
    let domain = test_loadout_domain();
    let base_loadout = test_base_loadout();
    let params = FullLoadoutSearchParams {
        item_pool: &item_pool,
        max_items: 3,
        loadout_domain: &domain,
        base_loadout: &base_loadout,
    };

    let ranked = beam_search_ranked_full(
        &params,
        3,
        7,
        &|candidate: &BuildKey| candidate.item_indices.iter().sum::<usize>() as f64,
        None,
    );

    assert!(!ranked.is_empty());
    let unique = ranked
        .iter()
        .map(|(candidate, _)| candidate.clone())
        .collect::<HashSet<_>>();
    assert_eq!(unique.len(), ranked.len());
    for (candidate, _) in ranked {
        assert_legal_candidate(&item_pool, &candidate, 3);
    }
}

#[test]
fn mcts_search_ranked_full_returns_unique_legal_candidates_with_limit() {
    let item_pool = test_pool();
    let domain = test_loadout_domain();
    let base_loadout = test_base_loadout();
    let params = FullLoadoutSearchParams {
        item_pool: &item_pool,
        max_items: 3,
        loadout_domain: &domain,
        base_loadout: &base_loadout,
    };
    let ranked = mcts_search_ranked_full(
        &params,
        &MctsSearchConfig {
            iterations: 24,
            rollouts_per_expansion: 2,
            exploration: 1.4,
            seed: 13,
            limit: 4,
        },
        &|candidate: &BuildKey| candidate.item_indices.iter().sum::<usize>() as f64,
        None,
    );

    assert!(!ranked.is_empty());
    assert!(ranked.len() <= 4);
    let unique = ranked
        .iter()
        .map(|(candidate, _)| candidate.clone())
        .collect::<HashSet<_>>();
    assert_eq!(unique.len(), ranked.len());
    for (candidate, _) in ranked {
        assert_legal_candidate(&item_pool, &candidate, 3);
    }
}

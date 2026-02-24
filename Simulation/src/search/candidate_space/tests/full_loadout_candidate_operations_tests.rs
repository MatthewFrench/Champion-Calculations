use super::super::super::FullLoadoutSearchParams;
use super::*;
use crate::data::LoadoutDomain;
use crate::{BuildKey, Item, LoadoutSelection};

fn loadout(runes: [&str; 6], shards: [&str; 3]) -> LoadoutSelection {
    LoadoutSelection {
        rune_names: runes.into_iter().map(|entry| entry.to_string()).collect(),
        shard_stats: shards.into_iter().map(|entry| entry.to_string()).collect(),
    }
}

fn empty_loadout_domain() -> LoadoutDomain {
    LoadoutDomain {
        rune_paths: Vec::new(),
        shard_slots: [Vec::new(), Vec::new(), Vec::new()],
    }
}

#[test]
fn candidate_order_key_tracks_item_and_loadout_slots() {
    let candidate_a = BuildKey {
        item_indices: vec![2, 1],
        loadout_selection: loadout(
            ["A", "B", "C", "D", "E", "F"],
            ["adaptive", "adaptive", "health"],
        ),
    };
    let candidate_b = BuildKey {
        item_indices: vec![1, 2],
        loadout_selection: loadout(
            ["B", "A", "C", "D", "E", "F"],
            ["adaptive", "adaptive", "health"],
        ),
    };

    assert_ne!(
        candidate_order_key(&candidate_a),
        candidate_order_key(&candidate_b)
    );
}

#[test]
fn candidate_loadout_variants_deduplicates_anchor_and_base() {
    let base = loadout(
        ["A", "B", "C", "D", "E", "F"],
        ["adaptive", "adaptive", "health"],
    );
    let item_pool: &[Item] = &[];
    let domain = empty_loadout_domain();
    let params = FullLoadoutSearchParams {
        item_pool,
        max_items: 6,
        loadout_domain: &domain,
        base_loadout: &base,
    };
    let mut seed = 1337u64;

    let variants = candidate_loadout_variants(&base, &params, &mut seed, 0);

    assert_eq!(variants.len(), 1);
    assert_eq!(variants[0], base);
}

#[test]
fn candidate_loadout_variants_includes_anchor_and_base_when_different() {
    let base = loadout(
        ["A", "B", "C", "D", "E", "F"],
        ["adaptive", "adaptive", "health"],
    );
    let anchor = loadout(
        ["G", "H", "I", "J", "K", "L"],
        ["health", "adaptive", "health"],
    );
    let item_pool: &[Item] = &[];
    let domain = empty_loadout_domain();
    let params = FullLoadoutSearchParams {
        item_pool,
        max_items: 6,
        loadout_domain: &domain,
        base_loadout: &base,
    };
    let mut seed = 7u64;

    let variants = candidate_loadout_variants(&anchor, &params, &mut seed, 0);

    assert_eq!(variants.len(), 2);
    assert!(variants.contains(&anchor));
    assert!(variants.contains(&base));
}

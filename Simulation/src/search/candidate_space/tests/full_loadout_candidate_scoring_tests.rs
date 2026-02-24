use super::*;
use crate::{BuildKey, LoadoutSelection};

fn loadout(runes: [&str; 6], shards: [&str; 3]) -> LoadoutSelection {
    LoadoutSelection {
        rune_names: runes.into_iter().map(|entry| entry.to_string()).collect(),
        shard_stats: shards.into_iter().map(|entry| entry.to_string()).collect(),
    }
}

#[test]
fn score_full_candidates_deduplicates_canonical_candidates() {
    let primary = BuildKey {
        item_indices: vec![4, 2, 1],
        loadout_selection: loadout(
            ["A", "B", "C", "D", "E", "F"],
            ["adaptive", "adaptive", "health"],
        ),
    };
    let duplicate = BuildKey {
        item_indices: vec![1, 2, 4],
        loadout_selection: primary.loadout_selection.clone(),
    };
    let alternate = BuildKey {
        item_indices: vec![5, 3, 1],
        loadout_selection: loadout(
            ["G", "H", "I", "J", "K", "L"],
            ["health", "adaptive", "health"],
        ),
    };
    let candidates = vec![primary, duplicate, alternate];

    let scored = score_full_candidates(
        candidates,
        &|candidate: &BuildKey| candidate.item_indices.iter().sum::<usize>() as f64,
        None,
    );

    assert_eq!(scored.len(), 2);
}

#[test]
fn unique_ranked_full_candidates_tie_breaks_by_candidate_order_key() {
    let lower_key = BuildKey {
        item_indices: vec![1, 3, 5],
        loadout_selection: loadout(
            ["A", "B", "C", "D", "E", "F"],
            ["adaptive", "adaptive", "health"],
        ),
    };
    let higher_key = BuildKey {
        item_indices: vec![1, 3, 5],
        loadout_selection: loadout(
            ["Z", "B", "C", "D", "E", "F"],
            ["adaptive", "adaptive", "health"],
        ),
    };
    let ranked = unique_ranked_full_candidates(
        vec![higher_key.clone(), lower_key.clone()],
        &|_: &BuildKey| 1.0,
        2,
        None,
    );

    assert_eq!(ranked.len(), 2);
    assert_eq!(ranked[0].0, lower_key);
    assert_eq!(ranked[1].0, higher_key);
}

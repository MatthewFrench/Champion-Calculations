use std::collections::HashMap;

use super::*;
use crate::LoadoutSelection;

fn candidate(items: &[usize]) -> BuildKey {
    BuildKey {
        item_indices: items.to_vec(),
        loadout_selection: LoadoutSelection {
            rune_names: Vec::new(),
            shard_stats: Vec::new(),
        },
    }
}

#[test]
fn select_diverse_top_candidates_applies_gap_and_diversity_filters() {
    let ranked = vec![
        (candidate(&[0, 1, 2]), 100.0),
        (candidate(&[0, 1, 3]), 99.0),
        (candidate(&[4, 5, 6]), 95.0),
        (candidate(&[7, 8, 9]), 80.0),
    ];

    let selected = select_diverse_top_candidates(&ranked, 3, 3, 10.0);

    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].0.item_indices, vec![0, 1, 2]);
    assert_eq!(selected[1].0.item_indices, vec![4, 5, 6]);
}

#[test]
fn candidate_pareto_front_keys_excludes_dominated_entries() {
    let mut metrics = HashMap::new();
    metrics.insert(
        candidate(&[1]),
        BuildMetrics {
            objective: 10.0,
            ehp_mixed: 10.0,
            ap: 10.0,
            cost_timing: 10.0,
            total_cost: 1000.0,
        },
    );
    metrics.insert(
        candidate(&[2]),
        BuildMetrics {
            objective: 9.0,
            ehp_mixed: 9.0,
            ap: 9.0,
            cost_timing: 9.0,
            total_cost: 900.0,
        },
    );
    metrics.insert(
        candidate(&[3]),
        BuildMetrics {
            objective: 11.0,
            ehp_mixed: 8.0,
            ap: 9.0,
            cost_timing: 11.0,
            total_cost: 1100.0,
        },
    );

    let front = candidate_pareto_front_keys(&metrics);

    assert_eq!(front.len(), 2);
    assert!(front.contains(&candidate(&[1])));
    assert!(front.contains(&candidate(&[3])));
    assert!(!front.contains(&candidate(&[2])));
}

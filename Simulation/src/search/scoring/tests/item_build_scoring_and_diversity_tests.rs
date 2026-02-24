use std::collections::HashMap;

use super::*;

#[test]
fn select_diverse_top_builds_applies_gap_and_diversity_filters() {
    let ranked = vec![
        (vec![0, 1, 2], 100.0),
        (vec![0, 1, 3], 99.0),
        (vec![4, 5, 6], 95.0),
        (vec![7, 8, 9], 80.0),
    ];

    let selected = select_diverse_top_builds(&ranked, 3, 3, 10.0);

    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].0, vec![0, 1, 2]);
    assert_eq!(selected[1].0, vec![4, 5, 6]);
}

#[test]
fn pareto_front_keys_excludes_dominated_entries() {
    let mut metrics = HashMap::new();
    metrics.insert(
        vec![1],
        BuildMetrics {
            objective: 10.0,
            ehp_mixed: 10.0,
            ap: 10.0,
            cost_timing: 10.0,
            total_cost: 1000.0,
        },
    );
    metrics.insert(
        vec![2],
        BuildMetrics {
            objective: 9.0,
            ehp_mixed: 9.0,
            ap: 9.0,
            cost_timing: 9.0,
            total_cost: 900.0,
        },
    );
    metrics.insert(
        vec![3],
        BuildMetrics {
            objective: 11.0,
            ehp_mixed: 8.0,
            ap: 9.0,
            cost_timing: 11.0,
            total_cost: 1100.0,
        },
    );

    let front = pareto_front_keys(&metrics);

    assert_eq!(front.len(), 2);
    assert!(front.contains(&vec![1]));
    assert!(front.contains(&vec![3]));
    assert!(!front.contains(&vec![2]));
}

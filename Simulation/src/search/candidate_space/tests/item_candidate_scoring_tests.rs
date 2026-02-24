use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::time::{Duration, Instant};

use super::*;

#[test]
fn score_candidates_scores_each_unique_canonical_key_once() {
    let score_calls = AtomicUsize::new(0);
    let scored = score_candidates(
        vec![vec![2, 1], vec![1, 2], vec![3, 0]],
        &|key: &[usize]| {
            score_calls.fetch_add(1, AtomicOrdering::SeqCst);
            key.iter().sum::<usize>() as f64
        },
        None,
    );

    assert_eq!(score_calls.load(AtomicOrdering::SeqCst), 2);
    assert_eq!(scored.len(), 3);
}

#[test]
fn unique_ranked_from_candidates_dedupes_keys_and_filters_non_finite_scores() {
    let ranked = unique_ranked_from_candidates(
        vec![vec![2, 1], vec![1, 2], vec![4, 5], vec![3, 0]],
        &|key: &[usize]| {
            if key == [0, 3] {
                f64::NEG_INFINITY
            } else {
                key[0] as f64
            }
        },
        4,
        None,
    );

    assert_eq!(ranked.len(), 2);
    assert_eq!(ranked[0].0, vec![4, 5]);
    assert_eq!(ranked[1].0, vec![1, 2]);
}

#[test]
fn score_candidates_returns_empty_when_deadline_reached() {
    let deadline = Some(Instant::now() - Duration::from_millis(1));
    let scored = score_candidates(vec![vec![1, 2]], &|_: &[usize]| 1.0, deadline);

    assert!(scored.is_empty());
}

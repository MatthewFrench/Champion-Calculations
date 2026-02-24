use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::canonical_key;

pub(in crate::search) fn unique_ranked_from_candidates<F>(
    candidates: Vec<Vec<usize>>,
    score_fn: &F,
    limit: usize,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let scored = score_candidates(candidates, score_fn, deadline);
    let mut ranked = Vec::new();
    let mut seen = HashSet::new();
    for (_, key, score) in scored {
        if !score.is_finite() {
            continue;
        }
        if seen.insert(key.clone()) {
            ranked.push((key, score));
            if ranked.len() >= limit.max(1) {
                break;
            }
        }
    }
    ranked
}

pub(in crate::search) fn score_candidates<F>(
    candidates: Vec<Vec<usize>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    if candidates.is_empty() || deadline_reached(deadline) {
        return Vec::new();
    }
    let unique_keys: HashSet<Vec<usize>> = candidates.iter().map(|c| canonical_key(c)).collect();
    let mut key_list = unique_keys.into_iter().collect::<Vec<_>>();
    key_list.sort_unstable();

    let score_pairs = key_list
        .par_iter()
        .map(|key| {
            if deadline_reached(deadline) {
                (key.clone(), f64::NEG_INFINITY)
            } else {
                (key.clone(), score_fn(key))
            }
        })
        .collect::<Vec<_>>();
    let score_map = score_pairs
        .into_iter()
        .collect::<HashMap<Vec<usize>, f64>>();

    let mut scored = candidates
        .into_iter()
        .map(|candidate| {
            let key = canonical_key(&candidate);
            let score = score_map.get(&key).copied().unwrap_or(f64::NEG_INFINITY);
            (candidate, key, score)
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| {
        b.2.partial_cmp(&a.2)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.0.cmp(&b.0))
    });
    scored
}

#[cfg(test)]
#[path = "tests/item_candidate_scoring_tests.rs"]
mod item_candidate_scoring_tests;

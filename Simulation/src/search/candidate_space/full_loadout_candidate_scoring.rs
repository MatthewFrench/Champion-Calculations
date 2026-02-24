use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::{BuildKey, canonical_build_candidate};
use super::full_loadout_candidate_operations::candidate_order_key;

pub(in crate::search) fn score_full_candidates<F>(
    candidates: Vec<BuildKey>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    if candidates.is_empty() {
        return Vec::new();
    }
    let unique = candidates
        .into_iter()
        .map(canonical_build_candidate)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut scored = unique
        .par_iter()
        .map(|candidate| {
            let score = if deadline_reached(deadline) {
                f64::NEG_INFINITY
            } else {
                score_fn(candidate)
            };
            (candidate.clone(), score)
        })
        .collect::<Vec<_>>();
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| candidate_order_key(&a.0).cmp(&candidate_order_key(&b.0)))
    });
    scored
}

pub(in crate::search) fn unique_ranked_full_candidates<F>(
    candidates: Vec<BuildKey>,
    score_fn: &F,
    limit: usize,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    score_full_candidates(candidates, score_fn, deadline)
        .into_iter()
        .take(limit.max(1))
        .collect::<Vec<_>>()
}

#[cfg(test)]
#[path = "tests/full_loadout_candidate_scoring_tests.rs"]
mod full_loadout_candidate_scoring_tests;

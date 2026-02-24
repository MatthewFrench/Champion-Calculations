use std::collections::HashSet;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::super::candidate_space::item_candidate_scoring::score_candidates;
use super::super::super::{Item, is_boots};

pub(super) fn beam_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    beam_width: usize,
    score_fn: F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    let mut final_scored: Vec<(Vec<usize>, Vec<usize>, f64)> = vec![];

    for _ in 0..max_items {
        if deadline_reached(deadline) {
            break;
        }
        let mut next_candidates = Vec::new();
        for build in &candidates {
            let has_boots = build.iter().any(|&i| is_boots(&item_pool[i]));
            let used = build.iter().copied().collect::<HashSet<_>>();
            for (item_idx, item) in item_pool.iter().enumerate() {
                if used.contains(&item_idx) {
                    continue;
                }
                if is_boots(item) && has_boots {
                    continue;
                }
                let mut next = build.clone();
                next.push(item_idx);
                next_candidates.push(next);
            }
        }

        let scored = score_candidates(next_candidates, &score_fn, deadline);
        candidates = scored
            .iter()
            .take(beam_width)
            .map(|(candidate, _, _)| candidate.clone())
            .collect();
        final_scored = scored;
    }

    let mut ranked = Vec::new();
    let mut seen = HashSet::new();
    for (_, key, score) in final_scored {
        if seen.insert(key.clone()) {
            ranked.push((key, score));
        }
    }
    ranked
}

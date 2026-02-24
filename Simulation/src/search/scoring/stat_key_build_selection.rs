use std::cmp::Ordering;
use std::collections::HashSet;

use crate::{Item, build_from_indices, build_item_stats, is_boots};

pub(in crate::search) fn choose_best_build_by_stat(
    item_pool: &[Item],
    stat_key: &str,
    max_items: usize,
    beam_width: usize,
) -> Vec<usize> {
    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    for _ in 0..max_items {
        let mut next_candidates = Vec::new();
        for build in &candidates {
            let has_boots = build.iter().any(|&idx| is_boots(&item_pool[idx]));
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
        next_candidates.sort_by(|a, b| {
            let score_a = build_item_stats(&build_from_indices(item_pool, a)).get_stat(stat_key);
            let score_b = build_item_stats(&build_from_indices(item_pool, b)).get_stat(stat_key);
            score_b.partial_cmp(&score_a).unwrap_or(Ordering::Equal)
        });
        next_candidates.truncate(beam_width.max(1));
        candidates = next_candidates;
    }
    candidates.into_iter().next().unwrap_or_default()
}

#[cfg(test)]
#[path = "tests/stat_key_build_selection_tests.rs"]
mod stat_key_build_selection_tests;

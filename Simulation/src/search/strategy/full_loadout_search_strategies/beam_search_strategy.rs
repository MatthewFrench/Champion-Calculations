use std::collections::HashSet;
use std::time::Instant;

use crate::status::deadline_reached;

use super::super::super::candidate_space::full_loadout_candidate_operations::{
    candidate_loadout_variants, canonical_build_candidate, canonical_key,
};
use super::super::super::candidate_space::full_loadout_candidate_scoring::score_full_candidates;
use super::super::super::{BuildKey, FullLoadoutSearchParams, is_boots};

pub(super) fn beam_search_ranked_full<F>(
    params: &FullLoadoutSearchParams<'_>,
    beam_width: usize,
    seed: u64,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let mut local_seed = seed;
    let mut candidates: Vec<BuildKey> = vec![BuildKey {
        item_indices: Vec::new(),
        loadout_selection: params.base_loadout.clone(),
    }];
    let mut final_scored = Vec::new();

    for _ in 0..params.max_items {
        if deadline_reached(deadline) {
            break;
        }
        let mut next_candidates = Vec::new();
        for candidate in &candidates {
            let has_boots = candidate
                .item_indices
                .iter()
                .any(|&idx| is_boots(&params.item_pool[idx]));
            let used = candidate
                .item_indices
                .iter()
                .copied()
                .collect::<HashSet<_>>();
            for (item_idx, item) in params.item_pool.iter().enumerate() {
                if used.contains(&item_idx) {
                    continue;
                }
                if is_boots(item) && has_boots {
                    continue;
                }
                let mut next = candidate.clone();
                next.item_indices.push(item_idx);
                next.item_indices = canonical_key(&next.item_indices);
                let loadout_variants =
                    candidate_loadout_variants(&next.loadout_selection, params, &mut local_seed, 1);
                for loadout_selection in loadout_variants {
                    let mut variant = next.clone();
                    variant.loadout_selection = loadout_selection;
                    next_candidates.push(canonical_build_candidate(variant));
                }
            }
        }
        let scored = score_full_candidates(next_candidates, score_fn, deadline);
        candidates = scored
            .iter()
            .take(beam_width.max(1))
            .map(|(candidate, _)| candidate.clone())
            .collect::<Vec<_>>();
        final_scored = scored;
    }
    final_scored
}

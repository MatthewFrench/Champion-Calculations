use super::*;

pub(super) struct SeedCandidateCollectionContext<'a> {
    pub(super) search_cfg: &'a BuildSearchConfig,
    pub(super) seed_ranked: &'a [Vec<(BuildKey, f64)>],
    pub(super) coverage_seed_candidates: &'a [BuildKey],
    pub(super) bleed_candidates: Vec<BuildKey>,
    pub(super) adaptive_candidates: Vec<BuildKey>,
    pub(super) item_pool: &'a [Item],
    pub(super) max_items: usize,
    pub(super) search_loadout_domain: &'a crate::data::LoadoutDomain,
    pub(super) controlled_champion_search_base_loadout_selection: &'a LoadoutSelection,
}

pub(super) struct SeedCandidateCollectionOutput {
    pub(super) seed_top_sets: Vec<HashSet<BuildKey>>,
    pub(super) best_seeded_candidate: Option<(BuildKey, f64)>,
    pub(super) unique_candidate_keys: Vec<BuildKey>,
    pub(super) strict_scores: HashMap<BuildKey, f64>,
    pub(super) candidate_keys_generated: usize,
    pub(super) candidate_duplicates_pruned: usize,
    pub(super) strict_seed_scored_candidates: usize,
    pub(super) bleed_candidate_count: usize,
    pub(super) adaptive_candidate_count: usize,
}

pub(super) fn collect_seed_candidate_state(
    context: SeedCandidateCollectionContext<'_>,
) -> SeedCandidateCollectionOutput {
    let SeedCandidateCollectionContext {
        search_cfg,
        seed_ranked,
        coverage_seed_candidates,
        bleed_candidates,
        adaptive_candidates,
        item_pool,
        max_items,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
    } = context;

    let bleed_candidate_count = bleed_candidates.len();
    let adaptive_candidate_count = adaptive_candidates.len();

    let mut candidate_keys = Vec::new();
    let mut best_seeded_candidate: Option<(BuildKey, f64)> = None;
    let mut seed_top_sets = Vec::new();
    for (seed_idx, ranked) in seed_ranked.iter().enumerate() {
        let mut seed_top = HashSet::new();
        for (ranked_idx, (candidate, score)) in ranked.iter().enumerate() {
            if score.is_finite() {
                let candidate_key = canonical_build_candidate(candidate.clone());
                let replace = best_seeded_candidate
                    .as_ref()
                    .map(|(best_key, best_score)| {
                        *score > *best_score
                            || ((*score - *best_score).abs() <= f64::EPSILON
                                && build_key_cache_string(&candidate_key)
                                    < build_key_cache_string(best_key))
                    })
                    .unwrap_or(true);
                if replace {
                    best_seeded_candidate = Some((candidate_key, *score));
                }
            }
            if candidate.item_indices.len() == max_items {
                candidate_keys.push(candidate.clone());
                if ranked_idx < search_cfg.ensemble_seed_top_k.max(1) {
                    seed_top.insert(candidate.clone());
                }
                continue;
            }
            if !score.is_finite() {
                continue;
            }
            let mut completion_seed =
                partial_candidate_completion_seed(search_cfg.seed, seed_idx, ranked_idx, candidate);
            let completed = complete_partial_candidate_to_full(
                candidate,
                item_pool,
                max_items,
                &mut completion_seed,
            );
            if completed.item_indices.len() != max_items {
                continue;
            }
            candidate_keys.push(completed.clone());
            if ranked_idx < search_cfg.ensemble_seed_top_k.max(1) {
                seed_top.insert(completed);
            }
        }
        seed_top_sets.push(seed_top);
    }

    for candidate in coverage_seed_candidates {
        candidate_keys.push(candidate.clone());
    }
    for candidate in bleed_candidates {
        candidate_keys.push(candidate);
    }
    for candidate in adaptive_candidates {
        candidate_keys.push(candidate);
    }

    let candidate_keys_generated = candidate_keys.len();
    let mut unique_candidate_keys = candidate_keys
        .into_iter()
        .map(canonical_build_candidate)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_candidate_keys.sort_by_key(build_key_cache_string);
    if unique_candidate_keys.is_empty() {
        let mut fallback_seed = search_cfg.seed ^ 0x9e37_79b9_7f4a_7c15;
        unique_candidate_keys.push(canonical_build_candidate(BuildKey {
            item_indices: random_valid_build(item_pool, max_items, &mut fallback_seed),
            loadout_selection: random_loadout_selection(
                controlled_champion_search_base_loadout_selection,
                search_loadout_domain,
                &mut fallback_seed,
            ),
        }));
    }

    let candidate_duplicates_pruned =
        candidate_keys_generated.saturating_sub(unique_candidate_keys.len());

    let mut strict_scores = HashMap::<BuildKey, f64>::new();
    for ranked in seed_ranked {
        for (candidate, score) in ranked {
            if candidate.item_indices.len() != max_items {
                continue;
            }
            if !score.is_finite() {
                continue;
            }
            let entry = strict_scores.entry(candidate.clone()).or_insert(*score);
            if *score > *entry {
                *entry = *score;
            }
        }
    }

    let strict_seed_scored_candidates = strict_scores.len().min(unique_candidate_keys.len());

    SeedCandidateCollectionOutput {
        seed_top_sets,
        best_seeded_candidate,
        unique_candidate_keys,
        strict_scores,
        candidate_keys_generated,
        candidate_duplicates_pruned,
        strict_seed_scored_candidates,
        bleed_candidate_count,
        adaptive_candidate_count,
    }
}

use std::cmp::Ordering;

use super::*;

pub(super) struct StrictCandidateScoringContext<'a> {
    pub(super) search_cfg: &'a BuildSearchConfig,
    pub(super) item_pool: &'a [Item],
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) status: &'a mut StatusReporter,
    pub(super) current_deadline: &'a (dyn Fn() -> Option<Instant> + Sync),
    pub(super) full_score_for_search_type: &'a (dyn Fn(&str, &BuildKey) -> f64 + Sync),
    pub(super) unique_candidate_keys: Vec<BuildKey>,
    pub(super) strict_scores: HashMap<BuildKey, f64>,
}

pub(super) struct StrictCandidateScoringOutput {
    pub(super) strict_scores: HashMap<BuildKey, f64>,
    pub(super) strict_remaining_candidates: usize,
    pub(super) strict_non_finite_candidates: usize,
    pub(super) strict_candidates_skipped_timeout: usize,
    pub(super) strict_completion_percent: f64,
    pub(super) strict_random_promotions_done: usize,
    pub(super) processed_candidates: usize,
    pub(super) total_candidates: usize,
    pub(super) timed_out: bool,
}

pub(super) fn score_remaining_strict_candidates(
    context: StrictCandidateScoringContext<'_>,
) -> StrictCandidateScoringOutput {
    let StrictCandidateScoringContext {
        search_cfg,
        item_pool,
        timeout_flag,
        status,
        current_deadline,
        full_score_for_search_type,
        unique_candidate_keys,
        mut strict_scores,
    } = context;

    let total_candidates = unique_candidate_keys.len();
    let mut processed_keys = strict_scores.keys().cloned().collect::<HashSet<_>>();
    let mut processed_candidates = processed_keys.len().min(total_candidates);
    let mut timed_out = timeout_flag.load(AtomicOrdering::Relaxed) > 0;

    status.emit(
        "strict_full_ranking",
        Some((processed_candidates, total_candidates)),
        strict_scores
            .values()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
        Some("evaluating all generated candidates"),
        true,
    );

    let remaining_keys = unique_candidate_keys
        .iter()
        .filter(|key| !processed_keys.contains(*key))
        .cloned()
        .collect::<Vec<_>>();
    let strict_remaining_candidates = remaining_keys.len();
    let (remaining_keys, strict_random_promotions_done) =
        if search_cfg.strict_ranking_enable_heuristic_ordering {
            heuristic_sort_remaining_candidates_for_strict_ranking(
                remaining_keys,
                &strict_scores,
                item_pool.len(),
                search_cfg.strict_ranking_rune_signal_weight,
                search_cfg.strict_ranking_shard_signal_weight,
                search_cfg.seed,
                search_cfg.strict_ranking_exploration_promotions,
            )
        } else {
            (remaining_keys, 0)
        };

    let mut strict_non_finite_candidates = 0usize;
    let batch_size = 128usize;
    for batch in remaining_keys.chunks(batch_size) {
        if deadline_reached(current_deadline()) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            timed_out = true;
            break;
        }
        let scored_batch = batch
            .par_iter()
            .map(|key| {
                (
                    key.clone(),
                    full_score_for_search_type("strict_full_ranking", key),
                )
            })
            .collect::<Vec<_>>();
        for (key, score) in scored_batch {
            if score.is_finite() {
                strict_scores.insert(key.clone(), score);
            } else {
                strict_non_finite_candidates += 1;
            }
            processed_keys.insert(key);
            processed_candidates += 1;
            status.emit(
                "strict_full_ranking",
                Some((processed_candidates, total_candidates)),
                strict_scores
                    .values()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
                None,
                false,
            );
        }
    }

    let strict_candidates_skipped_timeout =
        total_candidates.saturating_sub(processed_candidates.min(total_candidates));
    let strict_completion_percent = if total_candidates > 0 {
        100.0 * (processed_candidates.min(total_candidates) as f64) / (total_candidates as f64)
    } else {
        100.0
    };

    StrictCandidateScoringOutput {
        strict_scores,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_random_promotions_done,
        processed_candidates,
        total_candidates,
        timed_out,
    }
}

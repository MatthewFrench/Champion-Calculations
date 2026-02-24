use super::*;

pub(super) struct ControlledChampionCandidateScoringContext<'a, 'ctx> {
    pub(super) search_cfg: &'a BuildSearchConfig,
    pub(super) item_pool: &'a [Item],
    pub(super) max_items: usize,
    pub(super) time_budget: Option<Duration>,
    pub(super) defer_hard_budget_until_coverage: bool,
    pub(super) hard_deadline_state: &'a Arc<Mutex<Option<Instant>>>,
    pub(super) deadline_for_search_type: &'a (dyn Fn(&str) -> Option<Instant> + Sync),
    pub(super) record_score_progress: &'a (dyn Fn(f64) + Sync),
    pub(super) resolve_loadout_for_selection:
        &'a (dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout> + Sync),
    pub(super) evaluate_build_with_bonus: &'a EvaluateBuildWithBonusFn<'ctx>,
    pub(super) item_has_unmodeled_effect_by_index: &'a [bool],
    pub(super) best_loadout_by_candidate: &'a Arc<Mutex<ResolvedByCandidateMap>>,
    pub(super) best_outcome_by_candidate: &'a Arc<Mutex<OutcomeByCandidateMap>>,
    pub(super) full_eval_count: &'a AtomicUsize,
    pub(super) unmodeled_rune_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_rune_candidates_penalized: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_rejected: &'a AtomicUsize,
    pub(super) unmodeled_item_effect_candidates_penalized: &'a AtomicUsize,
    pub(super) full_cache: &'a Arc<BlockingScoreCache>,
    pub(super) unique_scored_candidate_keys: &'a Arc<ShardedStringSet>,
    pub(super) search_type_counters: &'a Arc<HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>>,
    pub(super) timeout_flag: &'a AtomicUsize,
}

pub(super) fn score_candidate_for_search_type(
    search_type: &str,
    candidate: &BuildKey,
    context: &ControlledChampionCandidateScoringContext<'_, '_>,
) -> f64 {
    increment_search_type_counter(context.search_type_counters.as_ref(), search_type, 1, 0);
    if deadline_reached((context.deadline_for_search_type)(search_type)) {
        context.timeout_flag.store(1, AtomicOrdering::Relaxed);
        return f64::NEG_INFINITY;
    }

    let key = canonical_build_candidate(candidate.clone());
    let is_full_candidate = key.item_indices.len() == context.max_items;
    let cache_key = build_key_cache_string(&key);
    let cache = Arc::clone(context.full_cache);
    let search_type_owned = search_type.to_string();
    cache.get_or_compute(cache_key.clone(), || {
        if deadline_reached((context.deadline_for_search_type)(&search_type_owned)) {
            context.timeout_flag.store(1, AtomicOrdering::Relaxed);
            return f64::NEG_INFINITY;
        }
        let Some(resolved_loadout) =
            (context.resolve_loadout_for_selection)(&key.loadout_selection)
        else {
            return f64::NEG_INFINITY;
        };
        let unmodeled_rune_count = resolved_loadout.unmodeled_rune_names.len();
        let unmodeled_item_effect_count =
            count_unmodeled_item_effects(&key, context.item_has_unmodeled_effect_by_index);
        if unmodeled_rune_count > 0 && context.search_cfg.unmodeled_rune_hard_gate {
            context
                .unmodeled_rune_candidates_rejected
                .fetch_add(1, AtomicOrdering::Relaxed);
            return f64::NEG_INFINITY;
        }
        if unmodeled_item_effect_count > 0 && context.search_cfg.unmodeled_item_effect_hard_gate {
            context
                .unmodeled_item_effect_candidates_rejected
                .fetch_add(1, AtomicOrdering::Relaxed);
            return f64::NEG_INFINITY;
        }
        arm_time_budget_deadline_if_unset(
            context.hard_deadline_state,
            context.time_budget,
            context.defer_hard_budget_until_coverage,
            &search_type_owned,
        );
        if is_full_candidate {
            context
                .full_eval_count
                .fetch_add(1, AtomicOrdering::Relaxed);
        }
        increment_search_type_counter(
            context.search_type_counters.as_ref(),
            &search_type_owned,
            0,
            1,
        );
        let build_items = build_from_indices(context.item_pool, &key.item_indices);
        let (score, outcome) = (context.evaluate_build_with_bonus)(
            &build_items,
            &resolved_loadout.bonus_stats,
            Some(&key.loadout_selection),
        );
        let mut score = score;
        if unmodeled_rune_count > 0 {
            context
                .unmodeled_rune_candidates_penalized
                .fetch_add(1, AtomicOrdering::Relaxed);
            score -= context.search_cfg.unmodeled_rune_penalty_per_rune.max(0.0)
                * unmodeled_rune_count as f64;
        }
        if unmodeled_item_effect_count > 0 {
            context
                .unmodeled_item_effect_candidates_penalized
                .fetch_add(1, AtomicOrdering::Relaxed);
            score -= context
                .search_cfg
                .unmodeled_item_effect_penalty_per_item
                .max(0.0)
                * unmodeled_item_effect_count as f64;
        }
        if is_full_candidate && score.is_finite() {
            context
                .unique_scored_candidate_keys
                .insert(cache_key.clone());
        }
        if is_full_candidate {
            if let Ok(mut map) = context.best_loadout_by_candidate.lock() {
                map.insert(key.clone(), resolved_loadout);
            }
            if let Ok(mut map) = context.best_outcome_by_candidate.lock() {
                map.insert(key.clone(), outcome);
            }
            (context.record_score_progress)(score);
        }
        score
    })
}

pub(super) fn evaluate_candidate_direct_for_strict_fallback(
    candidate: &BuildKey,
    context: &ControlledChampionCandidateScoringContext<'_, '_>,
) -> Option<(BuildKey, f64, CombatOutcome, ResolvedLoadout)> {
    let key = canonical_build_candidate(candidate.clone());
    let resolved_loadout = (context.resolve_loadout_for_selection)(&key.loadout_selection)?;
    let unmodeled_rune_count = resolved_loadout.unmodeled_rune_names.len();
    let unmodeled_item_effect_count =
        count_unmodeled_item_effects(&key, context.item_has_unmodeled_effect_by_index);
    if unmodeled_rune_count > 0 && context.search_cfg.unmodeled_rune_hard_gate {
        return None;
    }
    if unmodeled_item_effect_count > 0 && context.search_cfg.unmodeled_item_effect_hard_gate {
        return None;
    }
    arm_time_budget_deadline_if_unset(
        context.hard_deadline_state,
        context.time_budget,
        context.defer_hard_budget_until_coverage,
        "strict_fallback",
    );
    let build_items = build_from_indices(context.item_pool, &key.item_indices);
    let (score, outcome) = (context.evaluate_build_with_bonus)(
        &build_items,
        &resolved_loadout.bonus_stats,
        Some(&key.loadout_selection),
    );
    let score = score
        - context.search_cfg.unmodeled_rune_penalty_per_rune.max(0.0) * unmodeled_rune_count as f64
        - context
            .search_cfg
            .unmodeled_item_effect_penalty_per_item
            .max(0.0)
            * unmodeled_item_effect_count as f64;
    if key.item_indices.len() == context.max_items && score.is_finite() {
        context
            .unique_scored_candidate_keys
            .insert(build_key_cache_string(&key));
    }
    Some((key, score, outcome, resolved_loadout))
}

fn count_unmodeled_item_effects(
    candidate: &BuildKey,
    item_has_unmodeled_effect_by_index: &[bool],
) -> usize {
    candidate
        .item_indices
        .iter()
        .filter(|item_index| {
            item_has_unmodeled_effect_by_index
                .get(**item_index)
                .copied()
                .unwrap_or(false)
        })
        .count()
}

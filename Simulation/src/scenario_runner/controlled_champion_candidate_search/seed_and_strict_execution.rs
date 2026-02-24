use std::cmp::Ordering;

use super::*;

#[derive(Debug)]
pub(in crate::scenario_runner) struct SeedAndStrictRankingExecution {
    pub(in crate::scenario_runner) seed_ranked: Vec<Vec<(BuildKey, f64)>>,
    pub(in crate::scenario_runner) seed_top_sets: Vec<HashSet<BuildKey>>,
    pub(in crate::scenario_runner) best_seeded_candidate: Option<(BuildKey, f64)>,
    pub(in crate::scenario_runner) unique_candidate_keys: Vec<BuildKey>,
    pub(in crate::scenario_runner) strict_scores: HashMap<BuildKey, f64>,
    pub(in crate::scenario_runner) candidate_keys_generated: usize,
    pub(in crate::scenario_runner) candidate_duplicates_pruned: usize,
    pub(in crate::scenario_runner) strict_seed_scored_candidates: usize,
    pub(in crate::scenario_runner) strict_remaining_candidates: usize,
    pub(in crate::scenario_runner) strict_non_finite_candidates: usize,
    pub(in crate::scenario_runner) strict_candidates_skipped_timeout: usize,
    pub(in crate::scenario_runner) strict_completion_percent: f64,
    pub(in crate::scenario_runner) strict_random_promotions_done: usize,
    pub(in crate::scenario_runner) processed_candidates: usize,
    pub(in crate::scenario_runner) total_candidates: usize,
    pub(in crate::scenario_runner) timed_out: bool,
    pub(in crate::scenario_runner) bleed_candidate_count: usize,
    pub(in crate::scenario_runner) adaptive_candidate_count: usize,
}

pub(in crate::scenario_runner) struct SeedAndStrictRankingRunContext<'a> {
    pub(in crate::scenario_runner) search_cfg: &'a BuildSearchConfig,
    pub(in crate::scenario_runner) active_strategies: &'a [String],
    pub(in crate::scenario_runner) item_pool: &'a [Item],
    pub(in crate::scenario_runner) max_items: usize,
    pub(in crate::scenario_runner) search_loadout_domain: &'a crate::data::LoadoutDomain,
    pub(in crate::scenario_runner) controlled_champion_search_base_loadout_selection:
        &'a LoadoutSelection,
    pub(in crate::scenario_runner) full_search_params: FullLoadoutSearchParams<'a>,
    pub(in crate::scenario_runner) coverage_seed_candidates: &'a [BuildKey],
    pub(in crate::scenario_runner) timeout_flag: &'a AtomicUsize,
    pub(in crate::scenario_runner) status: &'a mut StatusReporter,
    pub(in crate::scenario_runner) current_deadline: &'a (dyn Fn() -> Option<Instant> + Sync),
    pub(in crate::scenario_runner) full_score_for_search_type:
        &'a (dyn Fn(&str, &BuildKey) -> f64 + Sync),
}

pub(in crate::scenario_runner) fn run_seed_and_strict_ranking(
    context: SeedAndStrictRankingRunContext<'_>,
) -> SeedAndStrictRankingExecution {
    let SeedAndStrictRankingRunContext {
        search_cfg,
        active_strategies,
        item_pool,
        max_items,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
        full_search_params,
        coverage_seed_candidates,
        timeout_flag,
        status,
        current_deadline,
        full_score_for_search_type,
    } = context;

    let ensemble_seeds = search_cfg.ensemble_seeds.max(1);
    status.emit(
        "seed_search",
        Some((0, ensemble_seeds)),
        None,
        Some("running ensemble seeds"),
        true,
    );
    let mut seed_ranked = (0..ensemble_seeds)
        .into_par_iter()
        .map(|seed_idx| {
            if deadline_reached(current_deadline()) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                return (seed_idx, Vec::new());
            }
            let mut cfg = search_cfg.clone();
            cfg.seed = search_cfg.seed.wrapping_add(
                search_cfg
                    .ensemble_seed_stride
                    .wrapping_mul(seed_idx as u64),
            );
            cfg.ranked_limit = cfg.ranked_limit.max(64);
            let search_type = format!("seed_search:{}", cfg.strategy);
            let score_fn =
                |candidate: &BuildKey| full_score_for_search_type(search_type.as_str(), candidate);
            let ranked = build_search_ranked_full_loadout(
                &full_search_params,
                &cfg,
                &score_fn,
                current_deadline(),
            );
            (seed_idx, ranked)
        })
        .collect::<Vec<_>>();
    seed_ranked.sort_by_key(|(seed_idx, _)| *seed_idx);
    let seed_ranked = seed_ranked
        .into_iter()
        .map(|(_, ranked)| ranked)
        .collect::<Vec<_>>();
    status.emit(
        "seed_search",
        Some((seed_ranked.len().min(ensemble_seeds), ensemble_seeds)),
        None,
        None,
        false,
    );
    let strategy_elite_score_fn =
        |candidate: &BuildKey| full_score_for_search_type("strategy_elites", candidate);
    let mut strategy_elites = strategy_seed_elites_full_loadout(
        &full_search_params,
        search_cfg,
        active_strategies,
        &strategy_elite_score_fn,
        current_deadline(),
    );
    if !coverage_seed_candidates.is_empty() {
        let mut target_strategies = active_strategies.to_vec();
        if target_strategies.is_empty() {
            target_strategies.push(search_cfg.strategy.clone());
        }
        for (idx, candidate) in coverage_seed_candidates.iter().enumerate() {
            let strategy = target_strategies[idx % target_strategies.len()].clone();
            strategy_elites
                .entry(strategy)
                .or_default()
                .push(candidate.clone());
        }
        for candidates in strategy_elites.values_mut() {
            let mut unique = candidates
                .iter()
                .cloned()
                .map(canonical_build_candidate)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            unique.sort_by_key(build_key_cache_string);
            *candidates = unique;
        }
    }
    let adaptive_score_fn =
        |candidate: &BuildKey| full_score_for_search_type("adaptive_search", candidate);
    let adaptive_candidates = adaptive_strategy_candidates_full_loadout(
        &full_search_params,
        search_cfg,
        &strategy_elites,
        &adaptive_score_fn,
        current_deadline(),
    );
    let bleed_candidates =
        generate_bleed_candidates_full_loadout(&full_search_params, search_cfg, &strategy_elites);
    status.emit(
        "candidate_merge",
        None,
        None,
        Some("merging strict candidates"),
        true,
    );
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
    for ranked in &seed_ranked {
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

    let total_candidates = unique_candidate_keys.len();
    let strict_seed_scored_candidates = strict_scores.len().min(total_candidates);
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

    SeedAndStrictRankingExecution {
        seed_ranked,
        seed_top_sets,
        best_seeded_candidate,
        unique_candidate_keys,
        strict_scores,
        candidate_keys_generated,
        candidate_duplicates_pruned,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_random_promotions_done,
        processed_candidates,
        total_candidates,
        timed_out,
        bleed_candidate_count,
        adaptive_candidate_count,
    }
}

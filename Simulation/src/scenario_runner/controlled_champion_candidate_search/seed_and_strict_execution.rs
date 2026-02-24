use super::*;

mod seed_candidate_collection;
mod strict_candidate_scoring;

use self::seed_candidate_collection::{
    SeedCandidateCollectionContext, collect_seed_candidate_state,
};
use self::strict_candidate_scoring::{
    StrictCandidateScoringContext, score_remaining_strict_candidates,
};

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

    let collected = collect_seed_candidate_state(SeedCandidateCollectionContext {
        search_cfg,
        seed_ranked: &seed_ranked,
        coverage_seed_candidates,
        bleed_candidates,
        adaptive_candidates,
        item_pool,
        max_items,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
    });

    let strict_scoring = score_remaining_strict_candidates(StrictCandidateScoringContext {
        search_cfg,
        item_pool,
        timeout_flag,
        status,
        current_deadline,
        full_score_for_search_type,
        unique_candidate_keys: collected.unique_candidate_keys.clone(),
        strict_scores: collected.strict_scores,
    });

    SeedAndStrictRankingExecution {
        seed_ranked,
        seed_top_sets: collected.seed_top_sets,
        best_seeded_candidate: collected.best_seeded_candidate,
        unique_candidate_keys: collected.unique_candidate_keys,
        strict_scores: strict_scoring.strict_scores,
        candidate_keys_generated: collected.candidate_keys_generated,
        candidate_duplicates_pruned: collected.candidate_duplicates_pruned,
        strict_seed_scored_candidates: collected.strict_seed_scored_candidates,
        strict_remaining_candidates: strict_scoring.strict_remaining_candidates,
        strict_non_finite_candidates: strict_scoring.strict_non_finite_candidates,
        strict_candidates_skipped_timeout: strict_scoring.strict_candidates_skipped_timeout,
        strict_completion_percent: strict_scoring.strict_completion_percent,
        strict_random_promotions_done: strict_scoring.strict_random_promotions_done,
        processed_candidates: strict_scoring.processed_candidates,
        total_candidates: strict_scoring.total_candidates,
        timed_out: strict_scoring.timed_out,
        bleed_candidate_count: collected.bleed_candidate_count,
        adaptive_candidate_count: collected.adaptive_candidate_count,
    }
}

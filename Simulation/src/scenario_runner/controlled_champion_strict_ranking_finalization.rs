use super::*;

pub(super) type ControlledChampionFallbackEvaluation =
    (BuildKey, f64, CombatOutcome, ResolvedLoadout);

pub(super) struct ControlledChampionStrictRankingFinalizationContext<'a> {
    pub(super) strict_scores: HashMap<BuildKey, f64>,
    pub(super) best_seeded_candidate: &'a Option<(BuildKey, f64)>,
    pub(super) unique_candidate_keys: &'a [BuildKey],
    pub(super) search_seed: u64,
    pub(super) item_pool: &'a [Item],
    pub(super) max_items: usize,
    pub(super) best_outcome_by_candidate: &'a Arc<Mutex<OutcomeByCandidateMap>>,
    pub(super) best_loadout_by_candidate: &'a Arc<Mutex<ResolvedByCandidateMap>>,
    pub(super) evaluate_candidate_direct:
        &'a dyn Fn(&BuildKey) -> Option<ControlledChampionFallbackEvaluation>,
    pub(super) sim_max_time_seconds: f64,
    pub(super) objective_component_weights: ObjectiveComponentWeights,
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) current_deadline: &'a dyn Fn() -> Option<Instant>,
    pub(super) seed_ranked: &'a [Vec<(BuildKey, f64)>],
    pub(super) seed_top_sets: &'a [HashSet<BuildKey>],
    pub(super) ensemble_seed_top_k: usize,
}

pub(super) struct ControlledChampionStrictRankingFinalization {
    pub(super) controlled_champion_ranked: Vec<(BuildKey, f64)>,
    pub(super) seed_best_scores: Vec<f64>,
    pub(super) seed_hits_by_key: HashMap<BuildKey, usize>,
}

pub(super) fn finalize_controlled_champion_strict_ranking(
    context: ControlledChampionStrictRankingFinalizationContext<'_>,
) -> ControlledChampionStrictRankingFinalization {
    let ControlledChampionStrictRankingFinalizationContext {
        strict_scores,
        best_seeded_candidate,
        unique_candidate_keys,
        search_seed,
        item_pool,
        max_items,
        best_outcome_by_candidate,
        best_loadout_by_candidate,
        evaluate_candidate_direct,
        sim_max_time_seconds,
        objective_component_weights,
        timeout_flag,
        current_deadline,
        seed_ranked,
        seed_top_sets,
        ensemble_seed_top_k,
    } = context;

    let mut strict_scores = strict_scores;
    maybe_insert_strict_ranking_fallback(StrictRankingFallbackInsertionContext {
        strict_scores: &mut strict_scores,
        best_seeded_candidate,
        unique_candidate_keys,
        search_seed,
        item_pool,
        max_items,
        best_outcome_by_candidate,
        best_loadout_by_candidate,
        evaluate_candidate_direct,
    });

    let controlled_champion_ranked = sorted_controlled_champion_ranked(
        strict_scores,
        best_outcome_by_candidate,
        sim_max_time_seconds,
        objective_component_weights,
    );
    let seed_best_scores = collect_seed_best_scores(
        seed_ranked,
        ensemble_seed_top_k,
        timeout_flag,
        current_deadline,
    );
    let seed_hits_by_key = collect_seed_hits_by_key(seed_top_sets);

    ControlledChampionStrictRankingFinalization {
        controlled_champion_ranked,
        seed_best_scores,
        seed_hits_by_key,
    }
}

struct StrictRankingFallbackInsertionContext<'a> {
    strict_scores: &'a mut HashMap<BuildKey, f64>,
    best_seeded_candidate: &'a Option<(BuildKey, f64)>,
    unique_candidate_keys: &'a [BuildKey],
    search_seed: u64,
    item_pool: &'a [Item],
    max_items: usize,
    best_outcome_by_candidate: &'a Arc<Mutex<OutcomeByCandidateMap>>,
    best_loadout_by_candidate: &'a Arc<Mutex<ResolvedByCandidateMap>>,
    evaluate_candidate_direct:
        &'a dyn Fn(&BuildKey) -> Option<ControlledChampionFallbackEvaluation>,
}

fn maybe_insert_strict_ranking_fallback(context: StrictRankingFallbackInsertionContext<'_>) {
    let StrictRankingFallbackInsertionContext {
        strict_scores,
        best_seeded_candidate,
        unique_candidate_keys,
        search_seed,
        item_pool,
        max_items,
        best_outcome_by_candidate,
        best_loadout_by_candidate,
        evaluate_candidate_direct,
    } = context;

    if !strict_scores.is_empty() {
        return;
    }
    let Some(fallback_key) = strict_ranking_fallback_key(
        best_seeded_candidate,
        unique_candidate_keys,
        search_seed,
        item_pool,
        max_items,
    ) else {
        return;
    };
    let Some((key, fallback_score, fallback_outcome, fallback_loadout)) =
        evaluate_candidate_direct(&fallback_key)
    else {
        return;
    };
    strict_scores.insert(key.clone(), fallback_score);
    if let Ok(mut map) = best_outcome_by_candidate.lock() {
        map.insert(key.clone(), fallback_outcome);
    }
    if let Ok(mut map) = best_loadout_by_candidate.lock() {
        map.insert(key, fallback_loadout);
    }
}

fn strict_ranking_fallback_key(
    best_seeded_candidate: &Option<(BuildKey, f64)>,
    unique_candidate_keys: &[BuildKey],
    search_seed: u64,
    item_pool: &[Item],
    max_items: usize,
) -> Option<BuildKey> {
    best_seeded_candidate
        .as_ref()
        .map(|(candidate, _)| {
            if candidate.item_indices.len() == max_items {
                return candidate.clone();
            }
            let mut completion_seed =
                partial_candidate_completion_seed(search_seed, 0, 0, candidate);
            complete_partial_candidate_to_full(
                candidate,
                item_pool,
                max_items,
                &mut completion_seed,
            )
        })
        .or_else(|| unique_candidate_keys.first().cloned())
}

fn sorted_controlled_champion_ranked(
    strict_scores: HashMap<BuildKey, f64>,
    best_outcome_by_candidate: &Arc<Mutex<OutcomeByCandidateMap>>,
    sim_max_time_seconds: f64,
    objective_component_weights: ObjectiveComponentWeights,
) -> Vec<(BuildKey, f64)> {
    let mut controlled_champion_ranked = strict_scores.into_iter().collect::<Vec<_>>();
    let outcome_map_for_tiebreak = best_outcome_by_candidate
        .lock()
        .map(|map| map.clone())
        .unwrap_or_default();
    controlled_champion_ranked.sort_by(|a, b| {
        let by_score = b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal);
        if by_score != Ordering::Equal {
            return by_score;
        }
        let out_a = outcome_map_for_tiebreak.get(&a.0);
        let out_b = outcome_map_for_tiebreak.get(&b.0);
        let cap_a = out_a
            .map(|outcome| outcome.time_alive_seconds >= sim_max_time_seconds - 1e-6)
            .unwrap_or(false);
        let cap_b = out_b
            .map(|outcome| outcome.time_alive_seconds >= sim_max_time_seconds - 1e-6)
            .unwrap_or(false);
        if cap_a != cap_b {
            return cap_b.cmp(&cap_a);
        }
        let combo_a = out_a
            .map(|outcome| {
                objective_component_weights.damage * outcome.damage_dealt
                    + objective_component_weights.healing * outcome.healing_done
                    + objective_component_weights.enemy_kills * outcome.enemy_kills as f64
                    + objective_component_weights.invulnerable_seconds
                        * outcome.invulnerable_seconds
            })
            .unwrap_or(0.0);
        let combo_b = out_b
            .map(|outcome| {
                objective_component_weights.damage * outcome.damage_dealt
                    + objective_component_weights.healing * outcome.healing_done
                    + objective_component_weights.enemy_kills * outcome.enemy_kills as f64
                    + objective_component_weights.invulnerable_seconds
                        * outcome.invulnerable_seconds
            })
            .unwrap_or(0.0);
        let by_combo = combo_b.partial_cmp(&combo_a).unwrap_or(Ordering::Equal);
        if by_combo != Ordering::Equal {
            return by_combo;
        }
        build_key_cache_string(&a.0).cmp(&build_key_cache_string(&b.0))
    });
    controlled_champion_ranked
}

fn collect_seed_best_scores(
    seed_ranked: &[Vec<(BuildKey, f64)>],
    ensemble_seed_top_k: usize,
    timeout_flag: &AtomicUsize,
    current_deadline: &dyn Fn() -> Option<Instant>,
) -> Vec<f64> {
    let mut seed_best_scores = Vec::new();
    for ranked in seed_ranked {
        if deadline_reached(current_deadline()) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let best = ranked
            .iter()
            .take(ensemble_seed_top_k.max(1))
            .map(|(_, score)| *score)
            .fold(f64::NEG_INFINITY, |acc, value| acc.max(value));
        if best.is_finite() {
            seed_best_scores.push(best);
        }
    }
    seed_best_scores
}

fn collect_seed_hits_by_key(seed_top_sets: &[HashSet<BuildKey>]) -> HashMap<BuildKey, usize> {
    let mut seed_hits_by_key = HashMap::new();
    for top in seed_top_sets {
        for key in top {
            *seed_hits_by_key.entry(key.clone()).or_insert(0) += 1;
        }
    }
    seed_hits_by_key
}

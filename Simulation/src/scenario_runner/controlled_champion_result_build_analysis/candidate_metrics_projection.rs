use super::*;

pub(super) struct DiverseTopProjection {
    pub(super) diverse_top_raw: Vec<(BuildKey, f64)>,
    pub(super) diverse_top_keys: Vec<BuildKey>,
    pub(super) diverse_top_builds: Vec<(Vec<Item>, f64)>,
}

pub(super) fn project_diverse_top_candidates(
    controlled_champion_ranked: &[(BuildKey, f64)],
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
    item_pool: &[Item],
) -> DiverseTopProjection {
    let diverse_top_raw = select_diverse_top_candidates(
        controlled_champion_ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
    );
    let diverse_top_keys = diverse_top_raw
        .iter()
        .map(|(candidate, _)| candidate.clone())
        .collect::<Vec<_>>();
    let diverse_top_builds = diverse_top_raw
        .iter()
        .map(|(candidate, score)| {
            (
                build_from_indices(item_pool, &candidate.item_indices),
                *score,
            )
        })
        .collect::<Vec<_>>();

    DiverseTopProjection {
        diverse_top_raw,
        diverse_top_keys,
        diverse_top_builds,
    }
}

pub(super) fn build_resolved_by_candidate_snapshot(
    best_loadout_by_candidate: &Arc<Mutex<ResolvedByCandidateMap>>,
) -> ResolvedByCandidateMap {
    best_loadout_by_candidate
        .lock()
        .map(|map| map.clone())
        .unwrap_or_default()
}

pub(super) struct ProjectMetricsAndParetoContext<'a> {
    pub(super) controlled_champion_ranked: &'a [(BuildKey, f64)],
    pub(super) item_pool: &'a [Item],
    pub(super) controlled_champion_base: &'a ChampionBase,
    pub(super) resolved_by_candidate_snapshot: &'a ResolvedByCandidateMap,
    pub(super) resolve_loadout_for_selection:
        &'a dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout>,
    pub(super) controlled_champion_base_loadout: &'a ResolvedLoadout,
    pub(super) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(super) sim: &'a SimulationConfig,
}

pub(super) fn project_metrics_and_pareto(
    context: ProjectMetricsAndParetoContext<'_>,
) -> (HashMap<BuildKey, BuildMetrics>, HashSet<BuildKey>) {
    let ProjectMetricsAndParetoContext {
        controlled_champion_ranked,
        item_pool,
        controlled_champion_base,
        resolved_by_candidate_snapshot,
        resolve_loadout_for_selection,
        controlled_champion_base_loadout,
        controlled_champion_stack_overrides,
        sim,
    } = context;

    let mut metrics_by_key = HashMap::new();
    for (candidate, score) in controlled_champion_ranked {
        let candidate_bonus_stats = candidate_bonus_stats_for_key(
            candidate,
            resolved_by_candidate_snapshot,
            resolve_loadout_for_selection,
            controlled_champion_base_loadout,
        );
        metrics_by_key.insert(
            candidate.clone(),
            compute_build_metrics_for_candidate(
                candidate,
                item_pool,
                controlled_champion_base,
                &candidate_bonus_stats,
                controlled_champion_stack_overrides,
                sim,
                *score,
            ),
        );
    }
    let pareto_front = candidate_pareto_front_keys(&metrics_by_key);

    (metrics_by_key, pareto_front)
}

pub(super) fn project_build_confidence(
    controlled_champion_ranked: &[(BuildKey, f64)],
    seed_hits_by_key: &HashMap<BuildKey, usize>,
    ensemble_seeds: usize,
    robust_min_seed_hit_rate: f64,
) -> Vec<BuildConfidence> {
    let seed_denominator = ensemble_seeds.max(1) as f64;
    controlled_champion_ranked
        .iter()
        .map(|(key, _)| {
            let hits = seed_hits_by_key.get(key).copied().unwrap_or(0);
            let hit_rate = hits as f64 / seed_denominator;
            let robustness = if hit_rate >= robust_min_seed_hit_rate {
                "robust".to_string()
            } else {
                "fragile".to_string()
            };
            BuildConfidence {
                key: key.clone(),
                seed_hits: hits,
                seed_hit_rate: hit_rate,
                robustness,
            }
        })
        .collect::<Vec<_>>()
}

pub(super) fn project_build_order_input(
    diverse_top_raw: &[(BuildKey, f64)],
    build_confidence: &[BuildConfidence],
    pareto_front: &HashSet<BuildKey>,
    item_pool: &[Item],
) -> Vec<(BuildKey, Vec<Item>)> {
    let confidence_by_key = build_confidence
        .iter()
        .map(|confidence| (confidence.key.clone(), confidence.clone()))
        .collect::<HashMap<_, _>>();
    let mut order_input = diverse_top_raw
        .iter()
        .filter_map(|(candidate, _)| {
            let robust = confidence_by_key
                .get(candidate)
                .map(|confidence| confidence.robustness == "robust")
                .unwrap_or(false);
            let pareto = pareto_front.contains(candidate);
            if robust || pareto {
                Some((
                    candidate.clone(),
                    build_from_indices(item_pool, &candidate.item_indices),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if order_input.is_empty() {
        order_input = diverse_top_raw
            .iter()
            .take(2)
            .map(|(candidate, _)| {
                (
                    candidate.clone(),
                    build_from_indices(item_pool, &candidate.item_indices),
                )
            })
            .collect::<Vec<_>>();
    }
    order_input
}

use super::*;

pub(super) struct ControlledChampionBestCandidateProjection {
    pub(super) controlled_champion_best_build: Vec<Item>,
    pub(super) controlled_champion_runtime_loadout_selection: LoadoutSelection,
    pub(super) controlled_champion_loadout: ResolvedLoadout,
    pub(super) controlled_champion_best_score: f64,
    pub(super) controlled_champion_best_outcome: CombatOutcome,
    pub(super) best_cap_survivor: bool,
    pub(super) best_score_breakdown: ObjectiveScoreBreakdown,
}

pub(super) struct ControlledChampionBestCandidateProjectionContext<'a, 'ctx> {
    pub(super) controlled_champion_ranked: &'a [(BuildKey, f64)],
    pub(super) item_pool: &'a [Item],
    pub(super) best_loadout_by_candidate: &'a Arc<Mutex<ResolvedByCandidateMap>>,
    pub(super) best_outcome_by_candidate: &'a Arc<Mutex<OutcomeByCandidateMap>>,
    pub(super) resolve_loadout_for_selection:
        &'a dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout>,
    pub(super) controlled_champion_base_loadout: &'a ResolvedLoadout,
    pub(super) objective_eval_ctx: &'a ObjectiveEvalContext<'ctx>,
    pub(super) sim: &'a SimulationConfig,
}

pub(super) fn project_controlled_champion_best_candidate(
    context: ControlledChampionBestCandidateProjectionContext<'_, '_>,
) -> ControlledChampionBestCandidateProjection {
    let ControlledChampionBestCandidateProjectionContext {
        controlled_champion_ranked,
        item_pool,
        best_loadout_by_candidate,
        best_outcome_by_candidate,
        resolve_loadout_for_selection,
        controlled_champion_base_loadout,
        objective_eval_ctx,
        sim,
    } = context;

    let controlled_champion_best_candidate = controlled_champion_ranked[0].0.clone();
    let controlled_champion_best_build =
        build_from_indices(item_pool, &controlled_champion_best_candidate.item_indices);
    let controlled_champion_runtime_loadout_selection =
        controlled_champion_best_candidate.loadout_selection.clone();
    let controlled_champion_loadout = best_loadout_by_candidate
        .lock()
        .ok()
        .and_then(|m| m.get(&controlled_champion_best_candidate).cloned())
        .or_else(|| resolve_loadout_for_selection(&controlled_champion_runtime_loadout_selection))
        .unwrap_or_else(|| controlled_champion_base_loadout.clone());

    let controlled_champion_best_score = controlled_champion_ranked
        .first()
        .map(|(_, s)| *s)
        .unwrap_or(0.0);
    let controlled_champion_best_outcome = best_outcome_by_candidate
        .lock()
        .ok()
        .and_then(|m| m.get(&controlled_champion_best_candidate).copied())
        .unwrap_or_else(|| {
            aggregate_objective_score_and_outcome_with_loadout_selection(
                objective_eval_ctx,
                &controlled_champion_best_build,
                &controlled_champion_loadout.bonus_stats,
                Some(&controlled_champion_runtime_loadout_selection),
            )
            .1
        });
    let (_, _, best_score_breakdown) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            objective_eval_ctx,
            &controlled_champion_best_build,
            &controlled_champion_loadout.bonus_stats,
            Some(&controlled_champion_runtime_loadout_selection),
        );
    let best_cap_survivor =
        controlled_champion_best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;

    ControlledChampionBestCandidateProjection {
        controlled_champion_best_build,
        controlled_champion_runtime_loadout_selection,
        controlled_champion_loadout,
        controlled_champion_best_score,
        controlled_champion_best_outcome,
        best_cap_survivor,
        best_score_breakdown,
    }
}

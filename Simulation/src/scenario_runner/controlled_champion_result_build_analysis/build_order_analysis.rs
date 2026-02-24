use super::*;

pub(super) struct BuildOrderAndEndStateContext<'a> {
    pub(super) order_input: &'a [(BuildKey, Vec<Item>)],
    pub(super) resolved_by_candidate_snapshot: &'a ResolvedByCandidateMap,
    pub(super) resolve_loadout_for_selection:
        &'a dyn Fn(&LoadoutSelection) -> Option<ResolvedLoadout>,
    pub(super) controlled_champion_base_loadout: &'a ResolvedLoadout,
    pub(super) controlled_champion_base_raw: &'a ChampionBase,
    pub(super) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(super) enemy_build_scenarios: &'a [EnemyBuildScenario],
    pub(super) raw_enemy_bases: &'a HashMap<String, ChampionBase>,
    pub(super) sim: &'a SimulationConfig,
    pub(super) urf: &'a UrfBuffs,
    pub(super) objective_component_weights: ObjectiveComponentWeights,
    pub(super) objective_worst_case_weight: f64,
    pub(super) controlled_champion_base: &'a ChampionBase,
    pub(super) controlled_champion_best_build: &'a [Item],
    pub(super) controlled_champion_loadout: &'a ResolvedLoadout,
}

pub(super) struct BuildOrderAndEndStateProjection {
    pub(super) build_order_results: Vec<BuildOrderResult>,
    pub(super) controlled_champion_end_stats: Stats,
    pub(super) stack_notes: Vec<String>,
}

pub(super) fn project_build_order_and_end_state(
    context: BuildOrderAndEndStateContext<'_>,
) -> BuildOrderAndEndStateProjection {
    let BuildOrderAndEndStateContext {
        order_input,
        resolved_by_candidate_snapshot,
        resolve_loadout_for_selection,
        controlled_champion_base_loadout,
        controlled_champion_base_raw,
        controlled_champion_stack_overrides,
        enemy_build_scenarios,
        raw_enemy_bases,
        sim,
        urf,
        objective_component_weights,
        objective_worst_case_weight,
        controlled_champion_base,
        controlled_champion_best_build,
        controlled_champion_loadout,
    } = context;

    let build_order_results = order_input
        .iter()
        .map(|(candidate, build)| {
            let candidate_bonus_stats = candidate_bonus_stats_for_key(
                candidate,
                resolved_by_candidate_snapshot,
                resolve_loadout_for_selection,
                controlled_champion_base_loadout,
            );
            let build_order_context = BuildOrderEvalContext {
                controlled_champion_base_raw,
                controlled_champion_bonus_stats: &candidate_bonus_stats,
                controlled_champion_stack_overrides,
                enemy_build_scenarios,
                raw_enemy_bases,
                sim,
                urf,
                objective_weights: objective_component_weights,
                multi_scenario_worst_weight: objective_worst_case_weight,
            };
            optimize_build_order(build, &build_order_context)
        })
        .collect::<Vec<_>>();

    let best_order_acquired_map = build_order_results.first().map(|build_result| {
        acquisition_level_map(&build_result.ordered_items, &build_result.acquired_levels)
    });
    let best_effective_item_stats = compute_effective_item_stats_for_build(
        controlled_champion_base,
        controlled_champion_best_build,
        &controlled_champion_loadout.bonus_stats,
        sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
        Some(controlled_champion_stack_overrides),
    );
    let controlled_champion_end_stats =
        compute_champion_final_stats(controlled_champion_base, &best_effective_item_stats);
    let stack_notes = build_stack_notes(
        controlled_champion_best_build,
        controlled_champion_base,
        sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
        Some(controlled_champion_stack_overrides),
    );

    BuildOrderAndEndStateProjection {
        build_order_results,
        controlled_champion_end_stats,
        stack_notes,
    }
}

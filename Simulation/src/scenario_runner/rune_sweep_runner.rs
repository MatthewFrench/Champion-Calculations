use anyhow::{Result, anyhow};
use rayon::prelude::*;
use std::cmp::Ordering;
use std::path::Path;

use super::controlled_champion_enemy_scenario_projection::{
    ControlledChampionEnemyBuildProjection, build_enemy_build_projection,
    build_scenario_reference_outcomes, parse_scaled_enemy_scenarios,
};
use super::*;
mod report_writing;
mod result_aggregation;

use self::report_writing::{RuneSweepReportWriteInput, write_rune_sweep_reports};
use self::result_aggregation::{
    RuneSweepEntry, average_combat_outcomes, average_objective_breakdowns,
};

pub(super) fn run_controlled_champion_fixed_loadout_rune_sweep_impl(
    scenario_path: &Path,
    options: &ControlledChampionFixedLoadoutOptions<'_>,
) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim = parse_simulation_config(simulation_config)?;
    sim.collect_rune_proc_telemetry = false;
    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim.champion_level;
    sim.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        simulation_config,
        simulation_level_before_controlled_override,
    );

    let mut controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim.champion_level);
    let controlled_champion_name = controlled_champion_base.name.clone();
    sim.controlled_champion_script = resolve_controlled_champion_script(&controlled_champion_name);

    let loadout_domain = build_loadout_domain();
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    if let Some(runes) = &options.fixed_rune_names {
        controlled_champion_loadout_selection.rune_names = runes.clone();
    }
    if let Some(shards) = &options.fixed_shard_stats {
        controlled_champion_loadout_selection.shard_stats = shards.clone();
    }
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    let fixed_build_items = item_pool_from_names(&items, &options.fixed_item_names)?;

    let baseline_keystone = controlled_champion_loadout_selection
        .rune_names
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("Controlled champion rune page must include a primary keystone"))?;
    let baseline_keystone_key = to_norm_key(&baseline_keystone);
    let primary_keystone_slot = loadout_domain
        .rune_paths
        .iter()
        .find_map(|path| {
            path.slot_runes.first().and_then(|slot| {
                slot.iter()
                    .any(|rune| to_norm_key(rune) == baseline_keystone_key)
                    .then_some(slot.clone())
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "Unable to resolve primary rune path for baseline keystone '{}'",
                baseline_keystone
            )
        })?;
    let mut keystone_candidates = primary_keystone_slot
        .into_iter()
        .map(|name| (to_norm_key(&name), name))
        .collect::<Vec<_>>();
    keystone_candidates.sort_by(|a, b| a.0.cmp(&b.0));
    keystone_candidates.dedup_by(|a, b| a.0 == b.0);
    let keystone_candidates = keystone_candidates
        .into_iter()
        .map(|(_, name)| name)
        .collect::<Vec<_>>();
    if keystone_candidates.is_empty() {
        return Err(anyhow!(
            "No keystone candidates found for baseline rune path '{}'",
            baseline_keystone
        ));
    }

    let enemy_scenarios = parse_scaled_enemy_scenarios(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;

    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;
    let ControlledChampionEnemyBuildProjection {
        enemy_build_scenarios,
        enemy_builds,
    } = build_enemy_build_projection(&enemy_scenarios, &enemy_presets, &items)?;

    let mut search_cfg = parse_scenario_search_or_default(&scenario)?;
    apply_search_quality_profile(&mut search_cfg, options.search_quality_profile);
    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
        search_cfg.objective_enemy_kills_weight,
        search_cfg.objective_invulnerable_seconds_weight,
    );
    let scenario_reference_outcomes = build_scenario_reference_outcomes(
        &enemy_build_scenarios,
        &sim,
        &urf,
        &controlled_champion_base,
    );
    let objective_eval_ctx = ObjectiveEvalContext {
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        enemy_build_scenarios: &enemy_build_scenarios,
        sim: &sim,
        urf: &urf,
        scenario_reference_outcomes: &scenario_reference_outcomes,
        weights: objective_component_weights,
        worst_case_weight: objective_worst_case_weight,
    };
    let sweep_seed_repeats = options.fixed_sweep_seed_repeats.max(1);
    let sweep_results_parallel = keystone_candidates
        .par_iter()
        .map(|keystone| -> Result<RuneSweepEntry> {
            let mut loadout_selection = controlled_champion_loadout_selection.clone();
            if let Some(primary_slot) = loadout_selection.rune_names.first_mut() {
                *primary_slot = keystone.clone();
            }
            loadout_selection =
                ensure_complete_loadout_selection(&loadout_selection, &loadout_domain)?;
            let resolved_loadout = resolve_loadout(&loadout_selection, sim.champion_level, true)?;
            let keystone_seed_base = fixed_sweep_keystone_seed_base(search_cfg.seed, keystone);
            let mut repeat_results = (0..sweep_seed_repeats)
                .into_par_iter()
                .map(|repeat_idx| {
                    let repeat_seed = fixed_sweep_repeat_seed(keystone_seed_base, repeat_idx);
                    let mut repeat_sim = sim.clone();
                    repeat_sim.combat_seed = Some(repeat_seed);
                    let repeat_objective_eval_ctx = ObjectiveEvalContext {
                        controlled_champion_base: objective_eval_ctx.controlled_champion_base,
                        controlled_champion_stack_overrides: objective_eval_ctx
                            .controlled_champion_stack_overrides,
                        enemy_build_scenarios: objective_eval_ctx.enemy_build_scenarios,
                        sim: &repeat_sim,
                        urf: objective_eval_ctx.urf,
                        scenario_reference_outcomes: objective_eval_ctx.scenario_reference_outcomes,
                        weights: objective_eval_ctx.weights,
                        worst_case_weight: objective_eval_ctx.worst_case_weight,
                    };
                    let (score, outcome, breakdown) =
                        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
                            &repeat_objective_eval_ctx,
                            &fixed_build_items,
                            &resolved_loadout.bonus_stats,
                            Some(&loadout_selection),
                        );
                    (repeat_idx, repeat_seed, score, outcome, breakdown)
                })
                .collect::<Vec<_>>();
            repeat_results.sort_by_key(|entry| entry.0);
            let seed_repeat_values = repeat_results
                .iter()
                .map(|entry| entry.1)
                .collect::<Vec<_>>();
            let seed_repeat_scores = repeat_results
                .iter()
                .map(|entry| entry.2)
                .collect::<Vec<_>>();
            let repeated_outcomes = repeat_results
                .iter()
                .map(|entry| entry.3)
                .collect::<Vec<_>>();
            let repeated_breakdowns = repeat_results
                .iter()
                .map(|entry| entry.4)
                .collect::<Vec<_>>();

            let objective_score =
                seed_repeat_scores.iter().sum::<f64>() / seed_repeat_scores.len().max(1) as f64;
            let outcome = average_combat_outcomes(&repeated_outcomes);
            let objective_breakdown = average_objective_breakdowns(&repeated_breakdowns);
            let mut trace_sim_cfg = sim.clone();
            trace_sim_cfg.collect_rune_proc_telemetry = true;
            if let Some(seed) = seed_repeat_values.first().copied() {
                trace_sim_cfg.combat_seed = Some(seed);
            }

            let mut trace_sim =
                ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
                    controlled_champion_base.clone(),
                    &fixed_build_items,
                    &resolved_loadout.bonus_stats,
                    Some(&loadout_selection),
                    None,
                    Some(&controlled_champion_stack_overrides),
                    &enemy_builds,
                    trace_sim_cfg,
                    urf.clone(),
                );
            while trace_sim.step(1) {}

            Ok(RuneSweepEntry {
                keystone_name: keystone.clone(),
                loadout_selection,
                objective_score,
                outcome,
                objective_breakdown,
                rune_proc_telemetry: trace_sim.controlled_champion_rune_proc_telemetry(),
                seed_repeat_scores,
                seed_repeat_values,
            })
        })
        .collect::<Vec<_>>();
    let mut sweep_results = sweep_results_parallel
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    sweep_results.sort_by(|a, b| {
        b.objective_score
            .partial_cmp(&a.objective_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| to_norm_key(&a.keystone_name).cmp(&to_norm_key(&b.keystone_name)))
    });
    write_rune_sweep_reports(&RuneSweepReportWriteInput {
        scenario_path,
        report_path_override: options.report_path_override,
        search_quality_profile: options.search_quality_profile,
        fixed_eval_label: options.fixed_eval_label.as_deref(),
        controlled_champion_name: &controlled_champion_name,
        fixed_build_items: &fixed_build_items,
        controlled_champion_loadout_selection: &controlled_champion_loadout_selection,
        sweep_seed_repeats,
        seed_base: search_cfg.seed,
        sweep_results: &sweep_results,
    })
}

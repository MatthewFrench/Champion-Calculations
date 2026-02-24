use super::super::super::controlled_champion_scenario_setup::{
    ControlledChampionEnemyBuildSetup, ControlledChampionEnemyBuildSetupContext,
    ControlledChampionScenarioSearchSetup, ControlledChampionScenarioSearchSetupContext,
    prepare_controlled_champion_enemy_build_setup,
    prepare_controlled_champion_scenario_search_setup,
};
use super::*;

pub(super) struct ControlledChampionRuntimeSetupContext<'a> {
    pub(super) scenario_path: &'a Path,
    pub(super) search_quality_profile: SearchQualityProfile,
    pub(super) seed_override: Option<u64>,
    pub(super) current_deadline: &'a (dyn Fn() -> Option<Instant> + Sync),
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) status: &'a mut StatusReporter,
}

pub(super) struct ControlledChampionRuntimeSetup {
    pub(super) urf: UrfBuffs,
    pub(super) sim: SimulationConfig,
    pub(super) controlled_champion_base: ChampionBase,
    pub(super) controlled_champion_base_raw: ChampionBase,
    pub(super) controlled_champion_name: String,
    pub(super) controlled_champion_stack_overrides: HashMap<String, f64>,
    pub(super) raw_enemy_bases: HashMap<String, ChampionBase>,
    pub(super) enemy_presets_used: HashMap<String, EnemyUrfPreset>,
    pub(super) enemy_build_scenarios: Vec<EnemyBuildScenario>,
    pub(super) enemy_builds: Vec<EnemyBuildEntry>,
    pub(super) enemy_derived_combat_stats: Vec<EnemyDerivedCombatStats>,
    pub(super) enemy_similarity_notes: Vec<String>,
    pub(super) search_cfg: BuildSearchConfig,
    pub(super) active_strategies: Vec<String>,
    pub(super) search_loadout_domain: Arc<crate::data::LoadoutDomain>,
    pub(super) controlled_champion_search_base_loadout_selection: LoadoutSelection,
    pub(super) item_pool: Vec<Item>,
    pub(super) max_items: usize,
    pub(super) controlled_champion_base_loadout: ResolvedLoadout,
    pub(super) resolve_cache: Arc<Mutex<HashMap<String, ResolvedLoadout>>>,
    pub(super) best_loadout_by_candidate: Arc<Mutex<ResolvedByCandidateMap>>,
    pub(super) best_outcome_by_candidate: Arc<Mutex<OutcomeByCandidateMap>>,
    pub(super) objective_worst_case_weight: f64,
    pub(super) objective_component_weights: ObjectiveComponentWeights,
    pub(super) scenario_reference_outcomes: Vec<CombatOutcome>,
    pub(super) item_has_unmodeled_effect_by_index: Vec<bool>,
    pub(super) enemy_loadout: ResolvedLoadout,
}

pub(super) fn prepare_controlled_champion_runtime_setup(
    context: ControlledChampionRuntimeSetupContext<'_>,
) -> Result<ControlledChampionRuntimeSetup> {
    let ControlledChampionRuntimeSetupContext {
        scenario_path,
        search_quality_profile,
        seed_override,
        current_deadline,
        timeout_flag,
        status,
    } = context;

    let scenario_search_setup = prepare_controlled_champion_scenario_search_setup(
        ControlledChampionScenarioSearchSetupContext {
            scenario_path,
            search_quality_profile,
            seed_override,
            current_deadline,
            timeout_flag,
            status,
        },
    )?;
    let ControlledChampionScenarioSearchSetup {
        items,
        urf,
        sim,
        controlled_champion_base,
        controlled_champion_base_raw,
        controlled_champion_name,
        controlled_champion_loadout_selection,
        controlled_champion_stack_overrides,
        raw_enemy_bases,
        enemy_scenarios,
        enemy_presets,
        search_cfg,
        active_strategies,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
        item_pool,
        max_items,
    } = scenario_search_setup;
    let enemy_loadout = ResolvedLoadout::default();

    let enemy_build_setup =
        prepare_controlled_champion_enemy_build_setup(ControlledChampionEnemyBuildSetupContext {
            enemy_scenarios: &enemy_scenarios,
            enemy_presets: &enemy_presets,
            items: &items,
            sim: &sim,
            urf: &urf,
            current_deadline,
            timeout_flag,
            status,
        })?;
    let ControlledChampionEnemyBuildSetup {
        enemy_presets_used,
        enemy_build_scenarios,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
    } = enemy_build_setup;

    let controlled_champion_base_loadout = resolve_loadout(
        &controlled_champion_loadout_selection,
        sim.champion_level,
        true,
    )?;
    let resolve_cache: Arc<Mutex<HashMap<String, ResolvedLoadout>>> =
        Arc::new(Mutex::new(HashMap::new()));
    if let Ok(mut map) = resolve_cache.lock() {
        map.insert(
            loadout_selection_key(&controlled_champion_loadout_selection),
            controlled_champion_base_loadout.clone(),
        );
    }
    let best_loadout_by_candidate: Arc<Mutex<ResolvedByCandidateMap>> =
        Arc::new(Mutex::new(HashMap::new()));
    let best_outcome_by_candidate: Arc<Mutex<OutcomeByCandidateMap>> =
        Arc::new(Mutex::new(HashMap::new()));

    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
        search_cfg.objective_enemy_kills_weight,
        search_cfg.objective_invulnerable_seconds_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            let damage_reference = enemy_builds_s
                .iter()
                .map(|(enemy, build, bonus_stats)| {
                    derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf).max_health
                })
                .sum::<f64>()
                .max(1.0);
            CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: damage_reference,
                healing_done: controlled_champion_base.base_health.max(1.0),
                enemy_kills: enemy_builds_s.len().max(1),
                invulnerable_seconds: sim.max_time_seconds.max(1.0),
            }
        })
        .collect::<Vec<_>>();
    let item_has_unmodeled_effect_by_index = item_pool
        .iter()
        .map(is_item_effect_unmodeled)
        .collect::<Vec<_>>();

    Ok(ControlledChampionRuntimeSetup {
        urf,
        sim,
        controlled_champion_base,
        controlled_champion_base_raw,
        controlled_champion_name,
        controlled_champion_stack_overrides,
        raw_enemy_bases,
        enemy_presets_used,
        enemy_build_scenarios,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
        search_cfg,
        active_strategies,
        search_loadout_domain,
        controlled_champion_search_base_loadout_selection,
        item_pool,
        max_items,
        controlled_champion_base_loadout,
        resolve_cache,
        best_loadout_by_candidate,
        best_outcome_by_candidate,
        objective_worst_case_weight,
        objective_component_weights,
        scenario_reference_outcomes,
        item_has_unmodeled_effect_by_index,
        enemy_loadout,
    })
}

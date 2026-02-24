use super::*;

pub(super) type ParsedEnemyScenarioSet = Vec<(String, f64, Vec<EnemyConfig>)>;

pub(super) struct ControlledChampionScenarioSearchSetup {
    pub(super) items: HashMap<String, Item>,
    pub(super) urf: UrfBuffs,
    pub(super) sim: SimulationConfig,
    pub(super) controlled_champion_base: ChampionBase,
    pub(super) controlled_champion_base_raw: ChampionBase,
    pub(super) controlled_champion_name: String,
    pub(super) controlled_champion_loadout_selection: LoadoutSelection,
    pub(super) controlled_champion_stack_overrides: HashMap<String, f64>,
    pub(super) raw_enemy_bases: HashMap<String, ChampionBase>,
    pub(super) enemy_scenarios: ParsedEnemyScenarioSet,
    pub(super) enemy_presets: HashMap<String, EnemyUrfPreset>,
    pub(super) search_cfg: BuildSearchConfig,
    pub(super) active_strategies: Vec<String>,
    pub(super) search_loadout_domain: Arc<crate::data::LoadoutDomain>,
    pub(super) controlled_champion_search_base_loadout_selection: LoadoutSelection,
    pub(super) item_pool: Vec<Item>,
    pub(super) max_items: usize,
}

pub(super) struct ControlledChampionScenarioSearchSetupContext<'a> {
    pub(super) scenario_path: &'a Path,
    pub(super) search_quality_profile: SearchQualityProfile,
    pub(super) seed_override: Option<u64>,
    pub(super) current_deadline: &'a dyn Fn() -> Option<Instant>,
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) status: &'a mut StatusReporter,
}

pub(super) fn prepare_controlled_champion_scenario_search_setup(
    context: ControlledChampionScenarioSearchSetupContext<'_>,
) -> Result<ControlledChampionScenarioSearchSetup> {
    let ControlledChampionScenarioSearchSetupContext {
        scenario_path,
        search_quality_profile,
        seed_override,
        current_deadline,
        timeout_flag,
        status,
    } = context;

    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;
    status.emit("initialization", None, None, Some("core data loaded"), true);

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim = parse_simulation_config(simulation_config)?;
    sim.collect_rune_proc_telemetry = false;
    if deadline_reached(current_deadline()) {
        timeout_flag.store(1, AtomicOrdering::Relaxed);
    }

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
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim.champion_level);
    let controlled_champion_base_raw = controlled_champion_config.base;
    let controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;
    let controlled_champion_name = controlled_champion_base_raw.name.clone();
    sim.controlled_champion_script = resolve_controlled_champion_script(&controlled_champion_name);

    let enemy_scenarios_raw: Vec<ParsedOpponentEncounter> = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let raw_enemy_bases = enemy_scenarios_raw
        .iter()
        .flat_map(|encounter| encounter.actors.iter())
        .fold(HashMap::new(), |mut map, enemy| {
            map.entry(enemy.id.clone())
                .or_insert_with(|| enemy.base.clone());
            map
        });
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|encounter| {
            let scaled = encounter
                .actors
                .iter()
                .cloned()
                .map(|mut enemy| {
                    enemy.base = champion_at_level(&enemy.base, enemy.level);
                    enemy
                })
                .collect::<Vec<_>>();
            (encounter.name.clone(), encounter.weight, scaled)
        })
        .collect::<Vec<_>>();

    let mut search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    apply_search_quality_profile(&mut search_cfg, search_quality_profile);
    if let Some(seed) = seed_override {
        search_cfg.seed = seed.max(1);
    }
    if search_cfg.seed == 0 {
        search_cfg.seed = runtime_random_seed();
    }

    let active_strategies = portfolio_strategy_list(&search_cfg);
    let full_loadout_domain = Arc::new(build_loadout_domain());
    let controlled_champion_loadout_selection = ensure_complete_loadout_selection(
        &controlled_champion_loadout_selection,
        full_loadout_domain.as_ref(),
    )?;
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &full_loadout_domain, &urf)?;

    let max_items = search_cfg.max_items;
    let full_item_pool = default_item_pool(&items, &urf);
    let search_loadout_domain = if search_cfg.unmodeled_rune_hard_gate {
        Arc::new(filter_loadout_domain_to_modeled_runes(
            full_loadout_domain.as_ref(),
            sim.champion_level,
            true,
        )?)
    } else {
        Arc::clone(&full_loadout_domain)
    };
    if search_cfg.unmodeled_rune_hard_gate && search_loadout_domain.rune_paths.len() < 2 {
        return Err(anyhow!(
            "Unmodeled rune hard gate left fewer than two legal rune paths for controlled champion search."
        ));
    }
    let controlled_champion_search_base_loadout_selection = if search_cfg.unmodeled_rune_hard_gate {
        select_search_base_loadout_selection(
            &controlled_champion_loadout_selection,
            search_loadout_domain.as_ref(),
        )?
    } else {
        controlled_champion_loadout_selection.clone()
    };
    let item_pool = if search_cfg.unmodeled_item_effect_hard_gate {
        filter_item_pool_to_modeled_runtime_effects(&full_item_pool)
    } else {
        full_item_pool
    };
    if item_pool.is_empty() {
        return Err(anyhow!(
            "Unmodeled item-effect hard gate left zero legal items in the controlled champion search pool."
        ));
    }
    let max_legal_items = max_legal_build_size(&item_pool);
    if max_legal_items < max_items {
        return Err(anyhow!(
            "Controlled champion search cannot form a full build under active item constraints: max_items={} but at most {} legal item slots are available.",
            max_items,
            max_legal_items
        ));
    }
    status.emit(
        "configuration",
        None,
        None,
        Some("search profile and enemy presets ready"),
        true,
    );

    Ok(ControlledChampionScenarioSearchSetup {
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
    })
}

pub(super) struct ControlledChampionEnemyBuildSetup {
    pub(super) enemy_presets_used: HashMap<String, EnemyUrfPreset>,
    pub(super) enemy_build_scenarios: Vec<EnemyBuildScenario>,
    pub(super) enemy_builds: Vec<EnemyBuildEntry>,
    pub(super) enemy_derived_combat_stats: Vec<EnemyDerivedCombatStats>,
    pub(super) enemy_similarity_notes: Vec<String>,
}

pub(super) struct ControlledChampionEnemyBuildSetupContext<'a> {
    pub(super) enemy_scenarios: &'a [(String, f64, Vec<EnemyConfig>)],
    pub(super) enemy_presets: &'a HashMap<String, EnemyUrfPreset>,
    pub(super) items: &'a HashMap<String, Item>,
    pub(super) sim: &'a SimulationConfig,
    pub(super) urf: &'a UrfBuffs,
    pub(super) current_deadline: &'a dyn Fn() -> Option<Instant>,
    pub(super) timeout_flag: &'a AtomicUsize,
    pub(super) status: &'a mut StatusReporter,
}

pub(super) fn prepare_controlled_champion_enemy_build_setup(
    context: ControlledChampionEnemyBuildSetupContext<'_>,
) -> Result<ControlledChampionEnemyBuildSetup> {
    let ControlledChampionEnemyBuildSetupContext {
        enemy_scenarios,
        enemy_presets,
        items,
        sim,
        urf,
        current_deadline,
        timeout_flag,
        status,
    } = context;

    let mut enemy_presets_used: HashMap<String, EnemyUrfPreset> = HashMap::new();
    let mut enemy_build_scenarios: Vec<EnemyBuildScenario> = Vec::new();
    for (name, weight, enemies) in enemy_scenarios {
        if deadline_reached(current_deadline()) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(items, &preset.item_names)?;
            let preset_enemy_loadout =
                resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?;
            let enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            enemy_presets_used.insert(preset_key, preset.clone());
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }

    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, builds)| builds.clone())
        .unwrap_or_default();
    let enemy_derived_combat_stats = enemy_builds
        .iter()
        .map(|(enemy, build, bonus_stats)| {
            derive_enemy_combat_stats(enemy, build, bonus_stats, sim, urf)
        })
        .collect::<Vec<_>>();
    let enemy_similarity_notes = build_enemy_similarity_notes(&enemy_derived_combat_stats);
    status.emit(
        "enemy_setup",
        None,
        None,
        Some("enemy preset builds loaded"),
        true,
    );

    Ok(ControlledChampionEnemyBuildSetup {
        enemy_presets_used,
        enemy_build_scenarios,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
    })
}

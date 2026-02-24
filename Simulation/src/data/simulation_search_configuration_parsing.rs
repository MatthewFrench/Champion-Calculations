use super::*;

pub(crate) fn as_str<'a>(obj: &'a Value, key: &str) -> Result<&'a str> {
    obj.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Missing string key: {}", key))
}

pub(crate) fn parse_stack_overrides_map(data: Option<&Value>) -> Result<HashMap<String, f64>> {
    let Some(raw) = data else {
        return Ok(HashMap::new());
    };
    let object = raw
        .as_object()
        .ok_or_else(|| anyhow!("stack_overrides must be an object keyed by stack identifier"))?;
    let mut out = HashMap::new();
    for (stack_identifier, value) in object {
        let stack_value = value
            .as_f64()
            .ok_or_else(|| anyhow!("stack_overrides.{} must be numeric", stack_identifier))?;
        if stack_value < 0.0 {
            bail!(
                "stack_overrides.{} must be >= 0.0, got {}",
                stack_identifier,
                stack_value
            );
        }
        out.insert(stack_identifier.clone(), stack_value);
    }
    Ok(out)
}

pub(crate) fn parse_simulation_config(data: &Value) -> Result<SimulationConfig> {
    let defaults = simulator_defaults();
    let sim_defaults = &defaults.simulation_defaults;
    let urf_respawn = urf_respawn_defaults();
    let zhonya_defaults = zhonya_time_stop_defaults();
    let guardian_angel_defaults = guardian_angel_rebirth_defaults();
    let protoplasm_defaults = protoplasm_lifeline_defaults();
    let controlled_champion_stasis_trigger_health_percent =
        controlled_champion_stasis_trigger_health_percent_default();
    if data.get("max_time_seconds").is_some() {
        bail!(
            "simulation.max_time_seconds is no longer supported. Use simulation.time_limit_seconds."
        );
    }
    if data.get("heartsteel_assumed_stacks_at_8m").is_some() {
        bail!(
            "simulation.heartsteel_assumed_stacks_at_8m is no longer supported. Use simulation.stack_overrides.heartsteel."
        );
    }
    if data.get("item_stacks_at_level_20").is_some() {
        bail!(
            "simulation.item_stacks_at_level_20 is no longer supported. Use simulation.stack_overrides."
        );
    }
    for legacy_key in [
        "vlad_pool_rank",
        "vlad_pool_untargetable_seconds",
        "vlad_pool_cost_percent_current_health",
        "vlad_pool_heal_ratio_of_damage",
        "vlad_pool_base_damage_by_rank",
        "vlad_pool_base_cooldown_seconds_by_rank",
        "vlad_pool_bonus_health_ratio",
        "vlad_q_base_damage",
        "vlad_q_ap_ratio",
        "vlad_q_heal_ratio_of_damage",
        "vlad_q_base_cooldown_seconds",
        "vlad_e_base_damage",
        "vlad_e_ap_ratio",
        "vlad_e_base_cooldown_seconds",
        "vlad_r_base_damage",
        "vlad_r_ap_ratio",
        "vlad_r_base_cooldown_seconds",
    ] {
        if data.get(legacy_key).is_some() {
            bail!(
                "simulation.{} is no longer supported. Controlled champion ability tuning must come from canonical champion data and controlled champion script capabilities.",
                legacy_key
            );
        }
    }
    let server_tick_rate_hz = data
        .get("server_tick_rate_hz")
        .and_then(Value::as_f64)
        .unwrap_or(sim_defaults.server_tick_rate_hz);
    let champion_level = data
        .get("champion_level")
        .and_then(Value::as_u64)
        .unwrap_or(sim_defaults.champion_level as u64) as usize;
    let dt = data.get("dt").and_then(Value::as_f64).unwrap_or_else(|| {
        if server_tick_rate_hz > 0.0 {
            1.0 / server_tick_rate_hz
        } else {
            sim_defaults.dt_fallback_seconds
        }
    });
    let protoplasm_level_t = ((champion_level.max(1) as f64 - 1.0) / 29.0).clamp(0.0, 1.0);
    let protoplasm_bonus_health_default = protoplasm_defaults.bonus_health_min
        + (protoplasm_defaults.bonus_health_max - protoplasm_defaults.bonus_health_min)
            * protoplasm_level_t;
    let protoplasm_heal_total_default = protoplasm_defaults.heal_total_min
        + (protoplasm_defaults.heal_total_max - protoplasm_defaults.heal_total_min)
            * protoplasm_level_t;

    let max_time_seconds = data
        .get("time_limit_seconds")
        .and_then(Value::as_f64)
        .unwrap_or(sim_defaults.time_limit_seconds);
    if !(max_time_seconds.is_finite() && max_time_seconds > 0.0) {
        bail!(
            "simulation.time_limit_seconds must be a positive finite number, got {}",
            max_time_seconds
        );
    }
    const MAX_TIME_LIMIT_SECONDS: f64 = 20.0 * 60.0;
    if max_time_seconds > MAX_TIME_LIMIT_SECONDS {
        bail!(
            "simulation.time_limit_seconds must be <= {:.0} seconds (20 minutes), got {}",
            MAX_TIME_LIMIT_SECONDS,
            max_time_seconds
        );
    }

    let mut stack_overrides = sim_defaults.stack_overrides.clone();
    stack_overrides.extend(parse_stack_overrides_map(data.get("stack_overrides"))?);
    let combat_seed = data.get("combat_seed").and_then(Value::as_u64);
    let collect_rune_proc_telemetry = data
        .get("collect_rune_proc_telemetry")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    Ok(SimulationConfig {
        dt,
        server_tick_rate_hz,
        champion_level,
        max_time_seconds,
        combat_seed,
        collect_rune_proc_telemetry,
        controlled_champion_script: None,
        zhonya_duration_seconds: data
            .get("zhonya_duration_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(zhonya_defaults.duration_seconds),
        zhonya_cooldown_seconds: data
            .get("zhonya_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(zhonya_defaults.cooldown_seconds),
        zhonya_trigger_health_percent: data
            .get("zhonya_trigger_health_percent")
            .and_then(Value::as_f64)
            .unwrap_or(controlled_champion_stasis_trigger_health_percent),
        ga_cooldown_seconds: data
            .get("ga_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(guardian_angel_defaults.cooldown_seconds),
        ga_revive_duration_seconds: data
            .get("ga_revive_duration_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(guardian_angel_defaults.revive_duration_seconds),
        ga_revive_base_health_ratio: data
            .get("ga_revive_base_health_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(guardian_angel_defaults.revive_base_health_ratio),
        protoplasm_trigger_health_percent: data
            .get("protoplasm_trigger_health_percent")
            .and_then(Value::as_f64)
            .unwrap_or(protoplasm_defaults.trigger_health_percent),
        protoplasm_bonus_health: data
            .get("protoplasm_bonus_health")
            .and_then(Value::as_f64)
            .unwrap_or(protoplasm_bonus_health_default),
        protoplasm_heal_total: data
            .get("protoplasm_heal_total")
            .and_then(Value::as_f64)
            .unwrap_or(protoplasm_heal_total_default),
        protoplasm_duration_seconds: data
            .get("protoplasm_duration_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(protoplasm_defaults.duration_seconds),
        stack_overrides,
        urf_respawn_flat_reduction_seconds: data
            .get("urf_respawn_flat_reduction_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(urf_respawn.flat_reduction_seconds),
        urf_respawn_extrapolation_per_level: data
            .get("urf_respawn_extrapolation_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(urf_respawn.extrapolation_per_level),
        urf_respawn_time_scaling_enabled: data
            .get("urf_respawn_time_scaling_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(urf_respawn.time_scaling_enabled),
        urf_respawn_time_scaling_start_seconds: data
            .get("urf_respawn_time_scaling_start_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(urf_respawn.time_scaling_start_seconds),
        urf_respawn_time_scaling_per_minute_seconds: data
            .get("urf_respawn_time_scaling_per_minute_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(urf_respawn.time_scaling_per_minute_seconds),
        urf_respawn_time_scaling_cap_seconds: data
            .get("urf_respawn_time_scaling_cap_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(urf_respawn.time_scaling_cap_seconds),
    })
}

pub(crate) fn parse_enemy_config(
    data: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
    default_level: usize,
    default_stack_overrides: &HashMap<String, f64>,
) -> Result<EnemyConfig> {
    let champion = data
        .get("champion")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Opponent actor requires champion"))?;
    let base = lookup_champion_base(champion_bases, champion)?;
    let actor_id = data
        .get("id")
        .and_then(Value::as_str)
        .unwrap_or(champion)
        .to_string();
    if data.get("combat").is_some() {
        bail!(
            "Opponent actor '{}' uses deprecated combat proxy settings. Remove actor.combat and model champion behavior through champion scripts/data.",
            actor_id
        );
    }
    let placement = data.get("placement").unwrap_or(&Value::Null);
    let spawn_position_xy = if let Some(position) = placement.get("position") {
        let position_object = position.as_object().ok_or_else(|| {
            anyhow!("Opponent actor placement.position must be an object with x/y fields")
        })?;
        let x = position_object
            .get("x")
            .and_then(Value::as_f64)
            .ok_or_else(|| anyhow!("Opponent actor placement.position.x is required"))?;
        let y = position_object
            .get("y")
            .and_then(Value::as_f64)
            .ok_or_else(|| anyhow!("Opponent actor placement.position.y is required"))?;
        Some((x, y))
    } else {
        None
    };
    let movement_mode = match placement.get("movement").and_then(Value::as_str) {
        Some(movement) => match to_norm_key(movement).as_str() {
            "holdposition" | "hold" | "static" => OpponentMovementMode::HoldPosition,
            "maintaincombatrange" | "maintainrange" | "orbit" | "kite" => {
                OpponentMovementMode::MaintainCombatRange
            }
            _ => bail!(
                "Opponent actor '{}' has unsupported placement.movement '{}'. Allowed values: hold_position, maintain_combat_range.",
                actor_id,
                movement
            ),
        },
        None => OpponentMovementMode::MaintainCombatRange,
    };
    let level = data
        .get("level")
        .and_then(Value::as_u64)
        .unwrap_or(default_level as u64)
        .max(1) as usize;
    if data.get("assumptions").is_some() {
        bail!(
            "Opponent actor '{}' uses deprecated assumptions. Use actor.stack_overrides instead.",
            actor_id
        );
    }
    if data.get("item_stacks_at_level_20").is_some() {
        bail!(
            "Opponent actor '{}' uses deprecated item_stacks_at_level_20. Use actor.stack_overrides instead.",
            actor_id
        );
    }
    let mut stack_overrides = default_stack_overrides.clone();
    stack_overrides.extend(parse_stack_overrides_map(data.get("stack_overrides"))?);

    Ok(EnemyConfig {
        id: actor_id,
        name: base.name.clone(),
        level,
        base,
        spawn_position_xy,
        movement_mode,
        loadout_item_names: Vec::new(),
        loadout_rune_names: Vec::new(),
        loadout_shards: Vec::new(),
        stack_overrides,
    })
}

pub(crate) fn parse_build_search(data: &Value) -> Result<BuildSearchConfig> {
    let defaults = simulator_defaults();
    let search_defaults = &defaults.search_defaults;
    let portfolio_strategies = data
        .get("portfolio_strategies")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(BuildSearchConfig {
        strategy: as_str(data, "strategy")?.to_string(),
        beam_width: data
            .get("beam_width")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.beam_width as u64) as usize,
        max_items: data
            .get("max_items")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.max_items as u64) as usize,
        random_samples: data
            .get("random_samples")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.random_samples as u64) as usize,
        hill_climb_restarts: data
            .get("hill_climb_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.hill_climb_restarts as u64)
            as usize,
        hill_climb_steps: data
            .get("hill_climb_steps")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.hill_climb_steps as u64) as usize,
        hill_climb_neighbors: data
            .get("hill_climb_neighbors")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.hill_climb_neighbors as u64)
            as usize,
        genetic_population: data
            .get("genetic_population")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.genetic_population as u64)
            as usize,
        genetic_generations: data
            .get("genetic_generations")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.genetic_generations as u64)
            as usize,
        genetic_mutation_rate: data
            .get("genetic_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.genetic_mutation_rate),
        genetic_crossover_rate: data
            .get("genetic_crossover_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.genetic_crossover_rate),
        portfolio_strategies,
        ranked_limit: data
            .get("ranked_limit")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ranked_limit as u64) as usize,
        simulated_annealing_restarts: data
            .get("simulated_annealing_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.simulated_annealing_restarts as u64)
            as usize,
        simulated_annealing_iterations: data
            .get("simulated_annealing_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.simulated_annealing_iterations as u64)
            as usize,
        simulated_annealing_initial_temp: data
            .get("simulated_annealing_initial_temp")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.simulated_annealing_initial_temp),
        simulated_annealing_cooling_rate: data
            .get("simulated_annealing_cooling_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.simulated_annealing_cooling_rate),
        mcts_iterations: data
            .get("mcts_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.mcts_iterations as u64) as usize,
        mcts_rollouts_per_expansion: data
            .get("mcts_rollouts_per_expansion")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.mcts_rollouts_per_expansion as u64)
            as usize,
        mcts_exploration: data
            .get("mcts_exploration")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.mcts_exploration),
        ensemble_seeds: data
            .get("ensemble_seeds")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ensemble_seeds as u64) as usize,
        ensemble_seed_stride: data
            .get("ensemble_seed_stride")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ensemble_seed_stride),
        ensemble_seed_top_k: data
            .get("ensemble_seed_top_k")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ensemble_seed_top_k as u64)
            as usize,
        objective_survival_weight: data
            .get("objective_survival_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_survival_weight),
        objective_damage_weight: data
            .get("objective_damage_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_damage_weight),
        objective_healing_weight: data
            .get("objective_healing_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_healing_weight),
        objective_enemy_kills_weight: data
            .get("objective_enemy_kills_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_enemy_kills_weight),
        objective_invulnerable_seconds_weight: data
            .get("objective_invulnerable_seconds_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_invulnerable_seconds_weight),
        robust_min_seed_hit_rate: data
            .get("robust_min_seed_hit_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.robust_min_seed_hit_rate),
        bleed_enabled: data
            .get("bleed_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.bleed_enabled),
        bleed_budget: data
            .get("bleed_budget")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.bleed_budget as u64) as usize,
        bleed_mutation_rate: data
            .get("bleed_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.bleed_mutation_rate),
        multi_scenario_worst_weight: data
            .get("multi_scenario_worst_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.multi_scenario_worst_weight),
        strict_ranking_enable_heuristic_ordering: data
            .get("strict_ranking_enable_heuristic_ordering")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.strict_ranking_enable_heuristic_ordering),
        strict_ranking_rune_signal_weight: data
            .get("strict_ranking_rune_signal_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.strict_ranking_rune_signal_weight),
        strict_ranking_shard_signal_weight: data
            .get("strict_ranking_shard_signal_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.strict_ranking_shard_signal_weight),
        strict_ranking_exploration_promotions: data
            .get("strict_ranking_exploration_promotions")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.strict_ranking_exploration_promotions as u64)
            as usize,
        unmodeled_rune_hard_gate: data
            .get("unmodeled_rune_hard_gate")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.unmodeled_rune_hard_gate),
        unmodeled_rune_penalty_per_rune: data
            .get("unmodeled_rune_penalty_per_rune")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.unmodeled_rune_penalty_per_rune),
        unmodeled_item_effect_hard_gate: data
            .get("unmodeled_item_effect_hard_gate")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.unmodeled_item_effect_hard_gate),
        unmodeled_item_effect_penalty_per_item: data
            .get("unmodeled_item_effect_penalty_per_item")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.unmodeled_item_effect_penalty_per_item),
        seed: data
            .get("seed")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.seed),
    })
}

pub(crate) fn apply_search_quality_profile(
    search: &mut BuildSearchConfig,
    profile: SearchQualityProfile,
) {
    fn apply_profile_overrides(search: &mut BuildSearchConfig, preset: SearchQualityProfilePreset) {
        search.beam_width = preset.beam_width;
        search.random_samples = preset.random_samples;
        search.hill_climb_restarts = preset.hill_climb_restarts;
        search.hill_climb_steps = preset.hill_climb_steps;
        search.hill_climb_neighbors = preset.hill_climb_neighbors;
        search.genetic_population = preset.genetic_population;
        search.genetic_generations = preset.genetic_generations;
        search.simulated_annealing_restarts = preset.simulated_annealing_restarts;
        search.simulated_annealing_iterations = preset.simulated_annealing_iterations;
        search.mcts_iterations = preset.mcts_iterations;
        search.mcts_rollouts_per_expansion = preset.mcts_rollouts_per_expansion;
        search.ensemble_seeds = preset.ensemble_seeds;
        search.ensemble_seed_top_k = preset.ensemble_seed_top_k;
        search.ranked_limit = preset.ranked_limit;
        search.bleed_budget = preset.bleed_budget;
    }

    fn apply_profile_minimums(search: &mut BuildSearchConfig, preset: SearchQualityProfilePreset) {
        search.beam_width = search.beam_width.max(preset.beam_width);
        search.random_samples = search.random_samples.max(preset.random_samples);
        search.hill_climb_restarts = search.hill_climb_restarts.max(preset.hill_climb_restarts);
        search.hill_climb_steps = search.hill_climb_steps.max(preset.hill_climb_steps);
        search.hill_climb_neighbors = search.hill_climb_neighbors.max(preset.hill_climb_neighbors);
        search.genetic_population = search.genetic_population.max(preset.genetic_population);
        search.genetic_generations = search.genetic_generations.max(preset.genetic_generations);
        search.simulated_annealing_restarts = search
            .simulated_annealing_restarts
            .max(preset.simulated_annealing_restarts);
        search.simulated_annealing_iterations = search
            .simulated_annealing_iterations
            .max(preset.simulated_annealing_iterations);
        search.mcts_iterations = search.mcts_iterations.max(preset.mcts_iterations);
        search.mcts_rollouts_per_expansion = search
            .mcts_rollouts_per_expansion
            .max(preset.mcts_rollouts_per_expansion);
        search.ensemble_seeds = search.ensemble_seeds.max(preset.ensemble_seeds);
        search.ensemble_seed_top_k = search.ensemble_seed_top_k.max(preset.ensemble_seed_top_k);
        search.ranked_limit = search.ranked_limit.max(preset.ranked_limit);
        search.bleed_budget = search.bleed_budget.max(preset.bleed_budget);
    }

    let profile_defaults = &simulator_defaults().search_quality_profile_defaults;
    match profile {
        SearchQualityProfile::Fast => {
            apply_profile_overrides(search, profile_defaults.fast);
            search.unmodeled_rune_hard_gate = false;
            search.unmodeled_rune_penalty_per_rune =
                search.unmodeled_rune_penalty_per_rune.max(0.0);
            search.unmodeled_item_effect_hard_gate = false;
            search.unmodeled_item_effect_penalty_per_item =
                search.unmodeled_item_effect_penalty_per_item.max(0.0);
        }
        SearchQualityProfile::Balanced => {
            apply_profile_overrides(search, profile_defaults.balanced);
            search.unmodeled_rune_hard_gate = false;
            search.unmodeled_rune_penalty_per_rune =
                search.unmodeled_rune_penalty_per_rune.max(0.0);
            search.unmodeled_item_effect_hard_gate = false;
            search.unmodeled_item_effect_penalty_per_item =
                search.unmodeled_item_effect_penalty_per_item.max(0.0);
        }
        SearchQualityProfile::MaximumQuality => {
            apply_profile_minimums(search, profile_defaults.maximum_quality_minimums);
            search.unmodeled_rune_hard_gate = true;
            search.unmodeled_rune_penalty_per_rune = 0.0;
            search.unmodeled_item_effect_hard_gate = true;
            search.unmodeled_item_effect_penalty_per_item = 0.0;
        }
    }
}

pub(crate) fn parse_loadout_selection(data: Option<&Value>) -> Result<LoadoutSelection> {
    let mut out = LoadoutSelection::default();
    let Some(obj) = data.and_then(Value::as_object) else {
        return Ok(out);
    };
    if obj.get("season2016_masteries").is_some() {
        bail!("loadout.season2016_masteries is no longer supported. Use loadout.runes_reforged.");
    }

    if let Some(runes_obj) = obj.get("runes_reforged").and_then(Value::as_object) {
        if runes_obj.get("rune_ids").is_some() {
            bail!(
                "loadout.runes_reforged.rune_ids is no longer supported. Use loadout.runes_reforged.rune_names."
            );
        }
        if let Some(names) = runes_obj.get("rune_names").and_then(Value::as_array) {
            out.rune_names = names
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect();
        }
        if let Some(shards) = runes_obj.get("shard_stats").and_then(Value::as_array) {
            out.shard_stats = shards
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect();
        }
    }
    Ok(out)
}

pub(crate) fn loadout_selection_key(sel: &LoadoutSelection) -> String {
    format!(
        "r={}|s={}",
        sel.rune_names.join(","),
        sel.shard_stats.join(",")
    )
}

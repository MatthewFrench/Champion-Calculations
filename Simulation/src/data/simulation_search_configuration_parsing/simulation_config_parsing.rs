use super::shared_parsing_primitives::parse_stack_overrides_map;
use super::*;

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

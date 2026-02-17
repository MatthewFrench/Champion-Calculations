use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::defaults::{
    SearchQualityProfilePreset, controlled_champion_stasis_trigger_health_percent_default,
    guardian_angel_rebirth_defaults, protoplasm_lifeline_defaults, simulator_defaults,
    urf_respawn_defaults, vladimir_offensive_ability_defaults, vladimir_sanguine_pool_defaults,
    zhonya_time_stop_defaults,
};
use crate::scripts::registry::hooks::{LoadoutHookContext, resolve_loadout_with_hooks};

use super::{
    BuildSearchConfig, ChampionBase, EXCLUDED_RANKS, EnemyConfig, ITEM_EVOLUTION_REPLACEMENTS,
    Item, LEGENDARY_RANK, LoadoutSelection, OpponentMovementMode, ResolvedLoadout,
    SearchQualityProfile, SimulationConfig, Stats, UrfBuffs, rand_index, shuffle_usize,
};

pub(crate) fn simulation_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

pub(crate) fn scenarios_dir() -> PathBuf {
    simulation_dir().join("scenarios")
}

pub(crate) fn resolve_scenario_path(raw: &str) -> PathBuf {
    let direct_candidate = PathBuf::from(raw);
    let looks_like_path = direct_candidate.is_absolute()
        || direct_candidate.exists()
        || raw.contains(std::path::MAIN_SEPARATOR)
        || raw.contains('/')
        || raw.contains('\\');
    if looks_like_path {
        return direct_candidate;
    }
    let by_name = scenarios_dir().join(&direct_candidate);
    if by_name.extension().is_some() {
        by_name
    } else {
        by_name.with_extension("json")
    }
}

pub(crate) fn items_dir() -> PathBuf {
    simulation_dir().join("..").join("Items")
}

pub(crate) fn game_mode_dir() -> PathBuf {
    simulation_dir().join("..").join("Game Mode")
}

pub(crate) fn characters_dir() -> PathBuf {
    simulation_dir().join("..").join("Characters")
}

pub(crate) fn rune_data_dir() -> PathBuf {
    simulation_dir().join("..").join("Masteries")
}

pub(crate) fn simulation_data_dir() -> PathBuf {
    simulation_dir().join("data")
}

pub(crate) fn load_json(path: &Path) -> Result<Value> {
    let text =
        fs::read_to_string(path).with_context(|| format!("Failed reading {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("Failed parsing {}", path.display()))
}

pub(crate) fn to_norm_key(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

pub(crate) fn value_from_effect(effect: &Value, rank: usize, level: usize) -> Option<f64> {
    if let Some(v) = effect
        .get("formula")
        .and_then(|f| f.get("value"))
        .and_then(Value::as_f64)
    {
        return Some(v);
    }
    if let Some(v) = effect.get("value").and_then(Value::as_f64) {
        return Some(v);
    }
    if let Some(values) = effect.get("values").and_then(Value::as_array) {
        let idx = rank.saturating_sub(1).min(values.len().saturating_sub(1));
        if let Some(v) = values.get(idx).and_then(Value::as_f64) {
            return Some(v);
        }
    }
    if let Some(vr) = effect.get("value_range").and_then(Value::as_object)
        && let (Some(min), Some(max)) = (
            vr.get("min").and_then(Value::as_f64),
            vr.get("max").and_then(Value::as_f64),
        )
    {
        let t = ((level.max(1) as f64 - 1.0) / 29.0).clamp(0.0, 1.0);
        return Some(min + (max - min) * t);
    }
    None
}

pub(crate) fn apply_stat_bonus(
    stats: &mut Stats,
    stat: &str,
    value: f64,
    is_percent_unit: bool,
    for_controlled_champion: bool,
) -> bool {
    match stat {
        "health" => {
            stats.health += value;
            true
        }
        "armor" => {
            stats.armor += value;
            true
        }
        "magic_resist" | "mr" => {
            stats.magic_resist += value;
            true
        }
        "attack_damage" => {
            stats.attack_damage += value;
            true
        }
        "ability_power" => {
            stats.ability_power += value;
            true
        }
        "ability_haste" => {
            stats.ability_haste += value;
            true
        }
        "attack_speed" => {
            stats.attack_speed_percent += value;
            true
        }
        "movement_speed" => {
            if is_percent_unit {
                stats.move_speed_percent += value;
            } else {
                stats.move_speed_flat += value;
            }
            true
        }
        "adaptive" => {
            if for_controlled_champion {
                stats.ability_power += value;
                true
            } else {
                false
            }
        }
        "cooldown" => {
            // Approximate CDR% to AH for deterministic use.
            let pct = if is_percent_unit {
                value / 100.0
            } else {
                value
            };
            let pct = pct.clamp(0.0, 0.95);
            let ah = 100.0 * pct / (1.0 - pct);
            stats.ability_haste += ah;
            true
        }
        _ => false,
    }
}

pub(crate) fn apply_structured_effect(
    effect: &Value,
    rank: usize,
    level: usize,
    for_controlled_champion: bool,
    stats: &mut Stats,
) -> Result<bool> {
    let effect_type = effect
        .get("effect_type")
        .and_then(Value::as_str)
        .unwrap_or("");
    if effect_type != "stat_modifier" && effect_type != "cooldown" {
        return Ok(false);
    }
    let trigger = effect
        .get("trigger")
        .and_then(Value::as_str)
        .unwrap_or("passive");
    let unconditional_trigger = matches!(trigger, "" | "passive" | "on_equip" | "always");
    if !unconditional_trigger {
        return Ok(false);
    }
    let stat = effect.get("stat").and_then(Value::as_str).unwrap_or("");
    if stat.is_empty() {
        return Ok(false);
    }
    let Some(value) = value_from_effect(effect, rank, level) else {
        return Ok(false);
    };
    let unit = effect.get("unit").and_then(Value::as_str).unwrap_or("");
    let is_percent_unit = unit.contains("percent") || unit == "ratio";
    Ok(apply_stat_bonus(
        stats,
        stat,
        value,
        is_percent_unit,
        for_controlled_champion,
    ))
}

pub(crate) fn resolve_loadout(
    selection: &LoadoutSelection,
    level: usize,
    for_controlled_champion: bool,
) -> Result<ResolvedLoadout> {
    let runes_data = load_json(&rune_data_dir().join("RunesReforged.json"))?;
    let loadout_domain = build_loadout_domain();

    let mut runes_by_name: HashMap<String, Value> = HashMap::new();
    if let Some(paths) = runes_data.get("paths").and_then(Value::as_array) {
        for path in paths {
            if let Some(slots) = path.get("slots").and_then(Value::as_array) {
                for slot in slots {
                    if let Some(runes) = slot.get("runes").and_then(Value::as_array) {
                        for rune in runes {
                            if let Some(name) = rune.get("name").and_then(Value::as_str) {
                                runes_by_name.insert(to_norm_key(name), rune.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    let mut out = ResolvedLoadout::default();
    validate_rune_page_selection(selection, &loadout_domain)?;

    for name in &selection.rune_names {
        let key = to_norm_key(name);
        if let Some(rune) = runes_by_name.get(&key) {
            let real_name = rune.get("name").and_then(Value::as_str).unwrap_or(name);
            out.selection_labels.push(format!("Rune: {}", real_name));
            for effect in rune
                .get("effects_structured")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
            {
                if apply_structured_effect(
                    &effect,
                    1,
                    level,
                    for_controlled_champion,
                    &mut out.bonus_stats,
                )? {
                    out.applied_notes
                        .push(format!("Applied rune stat effect from {}.", real_name));
                }
            }
        } else {
            out.skipped_notes
                .push(format!("Rune '{}' not found in RunesReforged.", name));
        }
    }

    if let Some(shards) = runes_data.get("stat_shards").and_then(Value::as_array) {
        for (idx, shard_key) in selection.shard_stats.iter().enumerate() {
            let Some(slot) = shards.get(idx) else {
                out.skipped_notes.push(format!(
                    "Shard '{}' ignored: slot {} does not exist.",
                    shard_key,
                    idx + 1
                ));
                continue;
            };
            let key = to_norm_key(shard_key);
            let mut applied = false;
            for option in slot
                .get("options")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
            {
                let stat = option.get("stat").and_then(Value::as_str).unwrap_or("");
                if to_norm_key(stat) != key {
                    continue;
                }
                let mut val = option
                    .get("numbers_extracted")
                    .and_then(Value::as_array)
                    .and_then(|a| a.first())
                    .and_then(Value::as_f64)
                    .unwrap_or(0.0);
                if stat == "health" {
                    // health shard scales with level using extracted [min, max]
                    if let Some(nums) = option.get("numbers_extracted").and_then(Value::as_array)
                        && nums.len() >= 2
                        && let (Some(min), Some(max)) = (nums[0].as_f64(), nums[1].as_f64())
                    {
                        let t = ((level.max(1) as f64 - 1.0) / 29.0).clamp(0.0, 1.0);
                        val = min + (max - min) * t;
                    }
                }
                let is_percent = option
                    .get("unit_hint")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .contains("percent");
                if apply_stat_bonus(
                    &mut out.bonus_stats,
                    stat,
                    val,
                    is_percent,
                    for_controlled_champion,
                ) {
                    out.selection_labels
                        .push(format!("Shard {}: {}", idx + 1, shard_key));
                    out.applied_notes.push(format!(
                        "Applied shard '{}' in slot {}.",
                        shard_key,
                        idx + 1
                    ));
                    applied = true;
                    break;
                }
            }
            if !applied {
                out.skipped_notes.push(format!(
                    "Shard '{}' in slot {} not applicable in current stat model.",
                    shard_key,
                    idx + 1
                ));
            }
        }
    }

    let hook_ctx = LoadoutHookContext {
        selection,
        level,
        for_controlled_champion,
    };
    resolve_loadout_with_hooks(&hook_ctx, &mut out)?;

    Ok(out)
}

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
    let pool_defaults = vladimir_sanguine_pool_defaults("vladimir").ok_or_else(|| {
        anyhow!(
            "Missing Characters/Vladimir.json abilities.basic_ability_2 defaults for Sanguine Pool"
        )
    })?;
    let zhonya_defaults = zhonya_time_stop_defaults();
    let guardian_angel_defaults = guardian_angel_rebirth_defaults();
    let protoplasm_defaults = protoplasm_lifeline_defaults();
    let controlled_champion_stasis_trigger_health_percent =
        controlled_champion_stasis_trigger_health_percent_default();
    let vladimir_ability_defaults =
        vladimir_offensive_ability_defaults("vladimir").ok_or_else(|| {
            anyhow!(
                "Missing Characters/Vladimir.json abilities.basic_ability_1/abilities.basic_ability_3/abilities.ultimate offensive defaults"
            )
        })?;
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
    if data.get("enemy_uptime_model_enabled").is_some() {
        bail!(
            "simulation.enemy_uptime_model_enabled is no longer supported. Use opponents.uptime_windows_enabled."
        );
    }
    if data.get("item_stacks_at_level_20").is_some() {
        bail!(
            "simulation.item_stacks_at_level_20 is no longer supported. Use simulation.stack_overrides."
        );
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

    let base_damage = data
        .get("vlad_pool_base_damage_by_rank")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .map(|v| v.as_f64().ok_or_else(|| anyhow!("Invalid base damage")))
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?
        .unwrap_or_else(|| pool_defaults.base_damage_by_rank.clone());
    let pool_cooldown_by_rank = data
        .get("vlad_pool_base_cooldown_seconds_by_rank")
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .map(|v| {
                    v.as_f64()
                        .ok_or_else(|| anyhow!("Invalid Sanguine Pool cooldown value"))
                })
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?
        .unwrap_or_else(|| pool_defaults.base_cooldown_seconds_by_rank.clone());
    if base_damage.is_empty() {
        bail!("vlad_pool_base_damage_by_rank must include at least one value");
    }
    if pool_cooldown_by_rank.is_empty() {
        bail!("vlad_pool_base_cooldown_seconds_by_rank must include at least one value");
    }

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

    let stack_overrides = parse_stack_overrides_map(data.get("stack_overrides"))?;

    Ok(SimulationConfig {
        dt,
        server_tick_rate_hz,
        champion_level,
        max_time_seconds,
        vlad_pool_rank: data
            .get("vlad_pool_rank")
            .and_then(Value::as_u64)
            .unwrap_or(pool_defaults.default_rank as u64)
            .max(1) as usize,
        vlad_pool_untargetable_seconds: data
            .get("vlad_pool_untargetable_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(pool_defaults.untargetable_seconds),
        vlad_pool_cost_percent_current_health: data
            .get("vlad_pool_cost_percent_current_health")
            .and_then(Value::as_f64)
            .unwrap_or(pool_defaults.cost_percent_current_health),
        vlad_pool_heal_ratio_of_damage: data
            .get("vlad_pool_heal_ratio_of_damage")
            .and_then(Value::as_f64)
            .unwrap_or(pool_defaults.heal_ratio_of_damage),
        vlad_pool_base_damage_by_rank: base_damage,
        vlad_pool_base_cooldown_seconds_by_rank: pool_cooldown_by_rank,
        vlad_pool_bonus_health_ratio: data
            .get("vlad_pool_bonus_health_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(pool_defaults.bonus_health_ratio),
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
        protoplasm_trigger_health_percent: protoplasm_defaults.trigger_health_percent,
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
        enemy_uptime_model_enabled: sim_defaults.enemy_uptime_model_enabled,
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
        vlad_q_base_damage: data
            .get("vlad_q_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.q_base_damage),
        vlad_q_ap_ratio: data
            .get("vlad_q_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.q_ap_ratio),
        vlad_q_heal_ratio_of_damage: data
            .get("vlad_q_heal_ratio_of_damage")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.q_heal_ratio_of_damage),
        vlad_q_base_cooldown_seconds: data
            .get("vlad_q_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.q_base_cooldown_seconds),
        vlad_e_base_damage: data
            .get("vlad_e_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.e_base_damage),
        vlad_e_ap_ratio: data
            .get("vlad_e_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.e_ap_ratio),
        vlad_e_base_cooldown_seconds: data
            .get("vlad_e_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.e_base_cooldown_seconds),
        vlad_r_base_damage: data
            .get("vlad_r_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.r_base_damage),
        vlad_r_ap_ratio: data
            .get("vlad_r_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.r_ap_ratio),
        vlad_r_base_cooldown_seconds: data
            .get("vlad_r_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(vladimir_ability_defaults.r_base_cooldown_seconds),
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
    let combat = data.get("combat").unwrap_or(&Value::Null);
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
        ability_dps_flat: combat
            .get("ability_dps_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_dps_ad_ratio: combat
            .get("ability_dps_ad_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_dps_ap_ratio: combat
            .get("ability_dps_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_tick_interval_seconds: combat
            .get("ability_tick_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        stun_interval_seconds: combat
            .get("stun_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        stun_duration_seconds: combat
            .get("stun_duration_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_interval_seconds: combat
            .get("burst_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_start_offset_seconds: combat
            .get("burst_start_offset_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_magic_flat: combat
            .get("burst_magic_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_physical_flat: combat
            .get("burst_physical_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_true_flat: combat
            .get("burst_true_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_ad_ratio: combat
            .get("burst_ad_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_ap_ratio: combat
            .get("burst_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_cycle_seconds: combat
            .get("uptime_cycle_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_active_seconds: combat
            .get("uptime_active_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_phase_seconds: combat
            .get("uptime_phase_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
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
        }
        SearchQualityProfile::Balanced => {
            apply_profile_overrides(search, profile_defaults.balanced);
        }
        SearchQualityProfile::MaximumQuality => {
            apply_profile_minimums(search, profile_defaults.maximum_quality_minimums);
        }
    }
}

pub(crate) fn parse_loadout_selection(data: Option<&Value>) -> LoadoutSelection {
    let mut out = LoadoutSelection::default();
    let Some(obj) = data.and_then(Value::as_object) else {
        return out;
    };

    if let Some(runes_obj) = obj.get("runes_reforged").and_then(Value::as_object) {
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
    out
}

pub(crate) fn loadout_selection_key(sel: &LoadoutSelection) -> String {
    format!(
        "r={}|s={}",
        sel.rune_names.join(","),
        sel.shard_stats.join(",")
    )
}

#[derive(Debug, Clone)]
pub(crate) struct RunePathDomain {
    pub(crate) slot_runes: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutDomain {
    pub(crate) rune_paths: Vec<RunePathDomain>,
    pub(crate) shard_slots: [Vec<String>; 3],
}

pub(crate) fn build_loadout_domain() -> LoadoutDomain {
    let runes_data = load_json(&rune_data_dir().join("RunesReforged.json")).unwrap_or(Value::Null);

    let rune_paths = runes_data
        .get("paths")
        .and_then(Value::as_array)
        .map(|paths| {
            paths
                .iter()
                .filter_map(|path| {
                    let slots = path.get("slots").and_then(Value::as_array)?;
                    let slot_runes = slots
                        .iter()
                        .map(|slot| {
                            slot.get("runes")
                                .and_then(Value::as_array)
                                .map(|runes| {
                                    runes
                                        .iter()
                                        .filter_map(|r| r.get("name").and_then(Value::as_str))
                                        .map(ToOwned::to_owned)
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default()
                        })
                        .collect::<Vec<_>>();
                    if slot_runes.len() >= 4 {
                        Some(RunePathDomain { slot_runes })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let shard_slots = {
        let slots = runes_data
            .get("stat_shards")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let read_slot = |idx: usize| -> Vec<String> {
            slots
                .get(idx)
                .and_then(|s| s.get("options"))
                .and_then(Value::as_array)
                .map(|options| {
                    options
                        .iter()
                        .filter_map(|o| o.get("stat").and_then(Value::as_str))
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        [read_slot(0), read_slot(1), read_slot(2)]
    };

    LoadoutDomain {
        rune_paths,
        shard_slots,
    }
}

fn is_rune_name_in_slot(slot_runes: &[String], rune_name: &str) -> bool {
    let key = to_norm_key(rune_name);
    slot_runes.iter().any(|r| to_norm_key(r) == key)
}

fn is_shard_name_in_slot(slot_shards: &[String], shard_name: &str) -> bool {
    let key = to_norm_key(shard_name);
    slot_shards.iter().any(|s| to_norm_key(s) == key)
}

pub(crate) fn is_legal_rune_page_selection(
    selection: &LoadoutSelection,
    loadout_domain: &LoadoutDomain,
) -> bool {
    let has_any_selection = !selection.rune_names.is_empty() || !selection.shard_stats.is_empty();
    if !has_any_selection {
        return true;
    }
    if selection.rune_names.len() != 6 || selection.shard_stats.len() != 3 {
        return false;
    }
    if loadout_domain.rune_paths.len() < 2 {
        return false;
    }
    for (idx, shard_name) in selection.shard_stats.iter().enumerate() {
        let valid = loadout_domain
            .shard_slots
            .get(idx)
            .map(|slot| is_shard_name_in_slot(slot, shard_name))
            .unwrap_or(false);
        if !valid {
            return false;
        }
    }

    let primary_keystone = &selection.rune_names[0];
    let primary_minor_1 = &selection.rune_names[1];
    let primary_minor_2 = &selection.rune_names[2];
    let primary_minor_3 = &selection.rune_names[3];
    let secondary_minor_1 = &selection.rune_names[4];
    let secondary_minor_2 = &selection.rune_names[5];

    for (primary_idx, primary_path) in loadout_domain.rune_paths.iter().enumerate() {
        if primary_path.slot_runes.len() < 4 {
            continue;
        }
        if !is_rune_name_in_slot(&primary_path.slot_runes[0], primary_keystone)
            || !is_rune_name_in_slot(&primary_path.slot_runes[1], primary_minor_1)
            || !is_rune_name_in_slot(&primary_path.slot_runes[2], primary_minor_2)
            || !is_rune_name_in_slot(&primary_path.slot_runes[3], primary_minor_3)
        {
            continue;
        }

        for (secondary_idx, secondary_path) in loadout_domain.rune_paths.iter().enumerate() {
            if secondary_idx == primary_idx || secondary_path.slot_runes.len() < 4 {
                continue;
            }
            let secondary_minor_1_slot = (1..=3).find(|slot| {
                is_rune_name_in_slot(&secondary_path.slot_runes[*slot], secondary_minor_1)
            });
            let secondary_minor_2_slot = (1..=3).find(|slot| {
                is_rune_name_in_slot(&secondary_path.slot_runes[*slot], secondary_minor_2)
            });
            if let (Some(slot_a), Some(slot_b)) = (secondary_minor_1_slot, secondary_minor_2_slot)
                && slot_a < slot_b
            {
                return true;
            }
        }
    }

    false
}

pub(crate) fn validate_rune_page_selection(
    selection: &LoadoutSelection,
    loadout_domain: &LoadoutDomain,
) -> Result<()> {
    if is_legal_rune_page_selection(selection, loadout_domain) {
        return Ok(());
    }
    bail!(
        "Invalid rune page selection. Provide ordered runes_reforged.rune_names with six runes [primary keystone, primary slot2, primary slot3, primary slot4, secondary slot2/3/4 rune A, secondary higher-slot rune B], plus ordered shard_stats for the three shard slots."
    );
}

pub(crate) fn random_loadout_selection(
    base: &LoadoutSelection,
    domain: &LoadoutDomain,
    seed: &mut u64,
) -> LoadoutSelection {
    let mut out = base.clone();

    if domain.rune_paths.len() >= 2
        && domain.shard_slots.iter().all(|s| !s.is_empty())
        && domain.rune_paths.iter().all(|p| p.slot_runes.len() >= 4)
    {
        let primary_idx = rand_index(seed, domain.rune_paths.len());
        let secondary_choices = (0..domain.rune_paths.len())
            .filter(|idx| *idx != primary_idx)
            .collect::<Vec<_>>();
        if !secondary_choices.is_empty() {
            let secondary_idx = secondary_choices[rand_index(seed, secondary_choices.len())];
            let primary = &domain.rune_paths[primary_idx];
            let secondary = &domain.rune_paths[secondary_idx];
            if primary.slot_runes[..4].iter().all(|slot| !slot.is_empty()) {
                let secondary_slots = (1..=3)
                    .filter(|slot| {
                        secondary
                            .slot_runes
                            .get(*slot)
                            .map(|r| !r.is_empty())
                            .unwrap_or(false)
                    })
                    .collect::<Vec<_>>();
                if secondary_slots.len() >= 2 {
                    let mut picks = secondary_slots.clone();
                    shuffle_usize(&mut picks, seed);
                    let (sa, sb) = if picks[0] <= picks[1] {
                        (picks[0], picks[1])
                    } else {
                        (picks[1], picks[0])
                    };
                    out.rune_names = vec![
                        primary.slot_runes[0][rand_index(seed, primary.slot_runes[0].len())]
                            .clone(),
                        primary.slot_runes[1][rand_index(seed, primary.slot_runes[1].len())]
                            .clone(),
                        primary.slot_runes[2][rand_index(seed, primary.slot_runes[2].len())]
                            .clone(),
                        primary.slot_runes[3][rand_index(seed, primary.slot_runes[3].len())]
                            .clone(),
                        secondary.slot_runes[sa][rand_index(seed, secondary.slot_runes[sa].len())]
                            .clone(),
                        secondary.slot_runes[sb][rand_index(seed, secondary.slot_runes[sb].len())]
                            .clone(),
                    ];
                    out.shard_stats = domain
                        .shard_slots
                        .iter()
                        .map(|slot| slot[rand_index(seed, slot.len())].clone())
                        .collect::<Vec<_>>();
                }
            }
        }
    }

    out
}

pub(crate) fn normalize_name(input: &str) -> String {
    input
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn is_character_support_file(stem: &str) -> bool {
    normalize_name(stem) == "championdefaults"
}

pub(crate) fn champion_base_from_character_data(
    character: &Value,
    fallback_name: &str,
) -> Result<ChampionBase> {
    let base_stats = character
        .get("base_stats")
        .ok_or_else(|| anyhow!("Missing base_stats for {}", fallback_name))?;

    let attack_speed = base_stats
        .get("attack_speed")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_speed.base for {}", fallback_name))?;
    let armor = base_stats
        .get("armor")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing armor.base for {}", fallback_name))?;
    let magic_resist = base_stats
        .get("magic_resist")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing magic_resist.base for {}", fallback_name))?;
    let attack_damage = base_stats
        .get("attack_damage")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_damage.base for {}", fallback_name))?;
    let health = base_stats
        .get("health")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing health.base for {}", fallback_name))?;
    let move_speed = base_stats
        .get("move_speed")
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing move_speed for {}", fallback_name))?;
    let attack_speed_per_level_percent = base_stats
        .get("attack_speed")
        .and_then(|v| v.get("per_level_percent"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let armor_per_level = base_stats
        .get("armor")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let magic_resist_per_level = base_stats
        .get("magic_resist")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let attack_damage_per_level = base_stats
        .get("attack_damage")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let health_per_level = base_stats
        .get("health")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);

    let attack_type = character
        .get("basic_attack")
        .and_then(|v| v.get("attack_type"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_lowercase();
    let is_melee = attack_type == "melee";
    let base_attack_projectile_speed = character
        .get("basic_attack")
        .and_then(|v| v.get("missile_speed"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let attack_range = base_stats
        .get("attack_range")
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_range for {}", fallback_name))?;

    let champion_name = character
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(fallback_name)
        .to_string();

    Ok(ChampionBase {
        name: champion_name,
        base_health: health,
        health_per_level,
        base_armor: armor,
        armor_per_level,
        base_magic_resist: magic_resist,
        magic_resist_per_level,
        base_attack_damage: attack_damage,
        attack_damage_per_level,
        base_attack_speed: attack_speed,
        attack_speed_per_level_percent,
        base_attack_range: attack_range,
        base_attack_projectile_speed,
        base_move_speed: move_speed,
        is_melee,
    })
}

pub(crate) fn load_champion_bases() -> Result<HashMap<String, ChampionBase>> {
    let mut out = HashMap::new();
    let mut entries = fs::read_dir(characters_dir())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    entries.sort();

    for path in entries {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid character filename"))?;
        if is_character_support_file(stem) {
            continue;
        }
        let data = load_json(&path)?;
        let base = champion_base_from_character_data(&data, stem)?;
        out.insert(normalize_name(stem), base.clone());
        out.insert(normalize_name(&base.name), base);
    }
    Ok(out)
}

pub(crate) fn lookup_champion_base(
    champion_bases: &HashMap<String, ChampionBase>,
    champion_name: &str,
) -> Result<ChampionBase> {
    champion_bases
        .get(&normalize_name(champion_name))
        .cloned()
        .ok_or_else(|| anyhow!("Champion not found: {}", champion_name))
}

pub(crate) fn load_urf_buffs() -> Result<UrfBuffs> {
    let path = game_mode_dir().join("URF.json");
    let data = load_json(&path)?;
    let buffs = data.get("global_buffs").cloned().unwrap_or(Value::Null);
    let allowed_item_keys = data
        .get("allowed_items")
        .and_then(|v| v.get("items"))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing Game Mode/URF.json allowed_items.items; URF item legality must be explicit."
            )
        })?
        .iter()
        .filter_map(Value::as_str)
        .map(to_norm_key)
        .collect::<HashSet<_>>();
    if allowed_item_keys.is_empty() {
        bail!("Game Mode/URF.json allowed_items.items must not be empty.");
    }

    Ok(UrfBuffs {
        ability_haste: buffs
            .get("ability_haste")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        item_haste: buffs
            .get("item_haste")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        health_cost_multiplier: buffs
            .get("resource_costs")
            .and_then(|v| v.get("health_cost_multiplier"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        bonus_attack_speed_multiplier_melee: buffs
            .get("bonus_attack_speed_multiplier")
            .and_then(|v| v.get("melee"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        bonus_attack_speed_multiplier_ranged: buffs
            .get("bonus_attack_speed_multiplier")
            .and_then(|v| v.get("ranged"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        allowed_item_keys,
    })
}

pub(crate) fn stat_key_map(key: &str) -> Option<&'static str> {
    match key {
        "abilityPower" => Some("ability_power"),
        "health" => Some("health"),
        "armor" => Some("armor"),
        "magicResist" => Some("magic_resist"),
        "attackDamage" => Some("attack_damage"),
        "attackSpeed" => Some("attack_speed_percent"),
        "movespeed" | "moveSpeed" | "movementSpeed" => Some("move_speed_flat"),
        "abilityHaste" => Some("ability_haste"),
        "critChance" => Some("crit_chance_percent"),
        _ => None,
    }
}

pub(crate) fn load_items() -> Result<HashMap<String, Item>> {
    let mut items = HashMap::new();
    let mut entries = fs::read_dir(items_dir())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    entries.sort();

    for path in entries {
        let data = load_json(&path)?;
        let rank = data
            .get("rank")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let purchasable = data
            .get("shop")
            .and_then(|v| v.get("purchasable"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let total_cost = data
            .get("shop")
            .and_then(|v| v.get("prices"))
            .and_then(|v| v.get("total"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0);
        if rank.iter().any(|r| EXCLUDED_RANKS.contains(&r.as_str())) {
            continue;
        }

        let mut stats = Stats::default();
        if let Some(stats_obj) = data.get("stats").and_then(Value::as_object) {
            for (raw_key, raw_values) in stats_obj {
                let Some(stat_key) = stat_key_map(raw_key) else {
                    continue;
                };

                if let Some(flat) = raw_values.get("flat").and_then(Value::as_f64) {
                    add_stat_value(&mut stats, stat_key, flat);
                }
                if let Some(percent) = raw_values.get("percent").and_then(Value::as_f64) {
                    if stat_key == "move_speed_flat" {
                        stats.move_speed_flat += percent;
                    } else {
                        add_stat_value(&mut stats, stat_key, percent);
                    }
                }
            }
        }

        let passive_effects_text = data
            .get("passives")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| p.get("effects").and_then(Value::as_str))
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let Some(name) = data
            .get("name")
            .and_then(Value::as_str)
            .map(|s| s.to_string())
        else {
            continue;
        };
        items.insert(
            name.clone(),
            Item {
                name,
                stats,
                rank,
                shop_purchasable: purchasable,
                total_cost,
                passive_effects_text,
            },
        );
    }

    Ok(items)
}

pub(crate) fn add_stat_value(stats: &mut Stats, stat_key: &str, value: f64) {
    match stat_key {
        "ability_power" => stats.ability_power += value,
        "health" => stats.health += value,
        "armor" => stats.armor += value,
        "magic_resist" => stats.magic_resist += value,
        "attack_damage" => stats.attack_damage += value,
        "attack_speed_percent" => stats.attack_speed_percent += value,
        "ability_haste" => stats.ability_haste += value,
        "move_speed_flat" => stats.move_speed_flat += value,
        "crit_chance_percent" => stats.crit_chance_percent += value,
        _ => {}
    }
}

pub(crate) fn item_pool_from_names(
    items: &HashMap<String, Item>,
    names: &[String],
) -> Result<Vec<Item>> {
    let mut out = Vec::new();
    for name in names {
        let resolved = resolve_evolved_item_name(items, name);
        out.push(
            items
                .get(&resolved)
                .cloned()
                .ok_or_else(|| anyhow!("Item not found: {}", resolved))?,
        );
    }
    Ok(out)
}

pub(crate) fn resolve_evolved_item_name(items: &HashMap<String, Item>, name: &str) -> String {
    for (source, evolved) in ITEM_EVOLUTION_REPLACEMENTS {
        if *source == name && items.contains_key(*evolved) {
            return (*evolved).to_string();
        }
    }
    name.to_string()
}

pub(crate) fn is_legendary(item: &Item) -> bool {
    item.rank.iter().any(|r| r == LEGENDARY_RANK)
}

pub(crate) fn is_pre_evolution_item(items: &HashMap<String, Item>, item_name: &str) -> bool {
    ITEM_EVOLUTION_REPLACEMENTS
        .iter()
        .any(|(source, evolved)| *source == item_name && items.contains_key(*evolved))
}

pub(crate) fn is_evolution_target(item_name: &str) -> bool {
    ITEM_EVOLUTION_REPLACEMENTS
        .iter()
        .any(|(_, evolved)| *evolved == item_name)
}

pub(crate) fn looks_arena_or_non_summoners_rift(item: &Item) -> bool {
    // Conservative guard: these naming patterns are commonly Arena/distributed-only.
    // We already constrain to LEGENDARY for search; this helps future-proof odd imports.
    let lower = item.name.to_ascii_lowercase();
    let arena_like_tokens = [
        "dragonheart",
        "hemomancer",
        "runecarver",
        "gambler",
        "golden spatula",
        "black hole gauntlet",
        "reaper",
        "demon king",
        "pyromancer",
        "molten stone",
        "wooglet",
        "entropy",
        "decapitator",
        "regicide",
        "lucky dice",
    ];
    arena_like_tokens.iter().any(|t| lower.contains(t))
}

pub(crate) fn item_is_allowed_in_urf(item_name: &str, urf: &UrfBuffs) -> bool {
    let key = to_norm_key(item_name);
    let mapped = match key.as_str() {
        // Item dataset may lag rename migrations; treat renamed canonical entries as equivalent.
        "ludensecho" => "ludenscompanion",
        _ => key.as_str(),
    };
    urf.allowed_item_keys.contains(mapped)
}

pub(crate) fn ensure_item_names_allowed_in_urf(
    item_names: &[String],
    urf: &UrfBuffs,
    context: &str,
) -> Result<()> {
    let disallowed = item_names
        .iter()
        .filter(|name| !item_is_allowed_in_urf(name, urf))
        .cloned()
        .collect::<Vec<_>>();
    if disallowed.is_empty() {
        return Ok(());
    }
    bail!(
        "{} includes item(s) not present in URF allowed_items: {}",
        context,
        disallowed.join(", ")
    );
}

pub(crate) fn default_item_pool(items: &HashMap<String, Item>, urf: &UrfBuffs) -> Vec<Item> {
    let mut pool = items
        .values()
        .filter(|item| item.shop_purchasable || is_evolution_target(&item.name))
        .filter(|item| is_legendary(item))
        .filter(|item| !is_pre_evolution_item(items, &item.name))
        .filter(|item| !looks_arena_or_non_summoners_rift(item))
        .filter(|item| item_is_allowed_in_urf(&item.name, urf))
        .cloned()
        .collect::<Vec<_>>();
    pool.sort_by(|a, b| a.name.cmp(&b.name));
    pool
}

#[derive(Debug, Clone)]
pub(crate) struct EnemyUrfPreset {
    pub(crate) champion: String,
    pub(crate) source_url: String,
    pub(crate) last_checked: String,
    pub(crate) item_names: Vec<String>,
    pub(crate) runes: Vec<String>,
    pub(crate) shards: Vec<String>,
}

pub(crate) fn enemy_preset_data_path() -> PathBuf {
    simulation_data_dir().join("enemy_urf_presets.json")
}

pub(crate) fn load_enemy_urf_presets() -> Result<HashMap<String, EnemyUrfPreset>> {
    let data = load_json(&enemy_preset_data_path())?;
    let defaults = data.get("defaults").and_then(Value::as_object);
    let default_source_url = defaults
        .and_then(|o| o.get("source_url"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let default_last_checked = defaults
        .and_then(|o| o.get("last_checked"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let presets = data
        .get("presets")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing presets array in {}",
                enemy_preset_data_path().display()
            )
        })?;

    let mut by_champion = HashMap::new();
    for preset in presets {
        let champion = preset
            .get("champion")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Missing preset champion field"))?
            .to_string();
        let item_names = preset
            .get("item_names")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing item_names for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let runes = preset
            .get("runes")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing runes for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let shards = preset
            .get("shards")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing shards for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let source_url = preset
            .get("source_url")
            .and_then(Value::as_str)
            .unwrap_or(&default_source_url)
            .to_string();
        let last_checked = preset
            .get("last_checked")
            .and_then(Value::as_str)
            .unwrap_or(&default_last_checked)
            .to_string();

        let loaded = EnemyUrfPreset {
            champion: champion.clone(),
            source_url,
            last_checked,
            item_names,
            runes,
            shards,
        };
        by_champion.insert(to_norm_key(&champion), loaded);
    }
    Ok(by_champion)
}

pub(crate) fn validate_enemy_urf_presets(
    presets: &HashMap<String, EnemyUrfPreset>,
    items: &HashMap<String, Item>,
    loadout_domain: &LoadoutDomain,
    urf: &UrfBuffs,
) -> Result<()> {
    let all_runes = loadout_domain
        .rune_paths
        .iter()
        .flat_map(|p| p.slot_runes.iter())
        .flat_map(|slot| slot.iter())
        .map(|s| to_norm_key(s))
        .collect::<HashSet<_>>();
    for preset in presets.values() {
        if preset.item_names.len() != 6 {
            bail!(
                "Enemy preset for {} must contain exactly six full items",
                preset.champion
            );
        }
        if preset.runes.len() != 6 {
            bail!(
                "Enemy preset for {} must contain exactly six runes",
                preset.champion
            );
        }
        if preset.shards.len() != 3 {
            bail!(
                "Enemy preset for {} must contain exactly three shards",
                preset.champion
            );
        }
        for item_name in &preset.item_names {
            if !items.contains_key(item_name) {
                bail!(
                    "Enemy preset item '{}' for {} is not present in Items/",
                    item_name,
                    preset.champion
                );
            }
            if !item_is_allowed_in_urf(item_name, urf) {
                bail!(
                    "Enemy preset item '{}' for {} is not present in Game Mode/URF.json allowed_items.",
                    item_name,
                    preset.champion
                );
            }
        }
        for rune_name in &preset.runes {
            if !all_runes.contains(&to_norm_key(rune_name)) {
                bail!(
                    "Enemy preset rune '{}' for {} is not present in RunesReforged",
                    rune_name,
                    preset.champion
                );
            }
        }
        validate_rune_page_selection(
            &LoadoutSelection {
                rune_names: preset.runes.clone(),
                shard_stats: preset.shards.clone(),
            },
            loadout_domain,
        )?;
        for (idx, shard) in preset.shards.iter().enumerate() {
            let valid = loadout_domain
                .shard_slots
                .get(idx)
                .map(|slot| slot.iter().any(|s| to_norm_key(s) == to_norm_key(shard)))
                .unwrap_or(false);
            if !valid {
                bail!(
                    "Enemy preset shard '{}' in slot {} for {} is invalid",
                    shard,
                    idx + 1,
                    preset.champion
                );
            }
        }
    }
    Ok(())
}

pub(crate) fn enemy_loadout_from_preset(preset: &EnemyUrfPreset) -> LoadoutSelection {
    LoadoutSelection {
        rune_names: preset.runes.clone(),
        shard_stats: preset.shards.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f64, expected: f64, field: &str) {
        let epsilon = 1e-9;
        assert!(
            (actual - expected).abs() <= epsilon,
            "{field} mismatch: actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn scenario_vladimir_offense_uses_canonical_ability_defaults() {
        let scenario_path = scenarios_dir().join("vladimir_urf_teamfight.json");
        let scenario =
            load_json(&scenario_path).expect("scenarios/vladimir_urf_teamfight.json should parse");
        let simulation = scenario
            .get("simulation")
            .expect("scenarios/vladimir_urf_teamfight.json should include simulation");

        for key in [
            "vlad_pool_rank",
            "vlad_pool_untargetable_seconds",
            "vlad_pool_cost_percent_current_health",
            "vlad_pool_heal_ratio_of_damage",
            "vlad_pool_base_damage_by_rank",
            "vlad_pool_bonus_health_ratio",
            "zhonya_duration_seconds",
            "zhonya_cooldown_seconds",
            "zhonya_trigger_health_percent",
            "ga_cooldown_seconds",
            "ga_revive_duration_seconds",
            "ga_revive_base_health_ratio",
            "protoplasm_trigger_health_percent",
            "protoplasm_bonus_health",
            "protoplasm_heal_total",
            "protoplasm_duration_seconds",
            "urf_respawn_flat_reduction_seconds",
            "urf_respawn_extrapolation_per_level",
            "urf_respawn_time_scaling_enabled",
            "urf_respawn_time_scaling_start_seconds",
            "urf_respawn_time_scaling_per_minute_seconds",
            "urf_respawn_time_scaling_cap_seconds",
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
            assert!(
                simulation.get(key).is_none(),
                "Scenario should not duplicate canonical Vladimir offensive constant '{key}'"
            );
        }

        let parsed = parse_simulation_config(simulation).expect("simulation config should parse");
        let canonical = vladimir_offensive_ability_defaults("vladimir")
            .expect("canonical Vladimir offensive defaults should load");

        assert_close(
            parsed.vlad_q_base_damage,
            canonical.q_base_damage,
            "vlad_q_base_damage",
        );
        assert_close(
            parsed.vlad_q_ap_ratio,
            canonical.q_ap_ratio,
            "vlad_q_ap_ratio",
        );
        assert_close(
            parsed.vlad_q_heal_ratio_of_damage,
            canonical.q_heal_ratio_of_damage,
            "vlad_q_heal_ratio_of_damage",
        );
        assert_close(
            parsed.vlad_q_base_cooldown_seconds,
            canonical.q_base_cooldown_seconds,
            "vlad_q_base_cooldown_seconds",
        );
        assert_close(
            parsed.vlad_e_base_damage,
            canonical.e_base_damage,
            "vlad_e_base_damage",
        );
        assert_close(
            parsed.vlad_e_ap_ratio,
            canonical.e_ap_ratio,
            "vlad_e_ap_ratio",
        );
        assert_close(
            parsed.vlad_e_base_cooldown_seconds,
            canonical.e_base_cooldown_seconds,
            "vlad_e_base_cooldown_seconds",
        );
        assert_close(
            parsed.vlad_r_base_damage,
            canonical.r_base_damage,
            "vlad_r_base_damage",
        );
        assert_close(
            parsed.vlad_r_ap_ratio,
            canonical.r_ap_ratio,
            "vlad_r_ap_ratio",
        );
        assert_close(
            parsed.vlad_r_base_cooldown_seconds,
            canonical.r_base_cooldown_seconds,
            "vlad_r_base_cooldown_seconds",
        );
    }

    #[test]
    fn load_champion_bases_skips_support_defaults_file() {
        let bases = load_champion_bases().expect("champion bases should load");
        assert!(
            !bases.contains_key(&normalize_name("ChampionDefaults")),
            "support defaults file should not be treated as a champion base"
        );
        assert!(
            bases.contains_key(&normalize_name("Vladimir")),
            "known champion base should still be present"
        );
    }

    #[test]
    fn validate_rune_page_selection_rejects_secondary_slot_order_violation() {
        let domain = build_loadout_domain();
        let valid = LoadoutSelection {
            rune_names: vec![
                "Arcane Comet".to_string(),
                "Manaflow Band".to_string(),
                "Transcendence".to_string(),
                "Gathering Storm".to_string(),
                "Cheap Shot".to_string(),
                "Ultimate Hunter".to_string(),
            ],
            shard_stats: vec![
                "ability_haste".to_string(),
                "movement_speed".to_string(),
                "health".to_string(),
            ],
        };
        validate_rune_page_selection(&valid, &domain)
            .expect("known rune page should pass legality validation");

        let invalid_secondary_order = LoadoutSelection {
            rune_names: vec![
                "Arcane Comet".to_string(),
                "Manaflow Band".to_string(),
                "Transcendence".to_string(),
                "Gathering Storm".to_string(),
                "Ultimate Hunter".to_string(),
                "Cheap Shot".to_string(),
            ],
            shard_stats: vec![
                "ability_haste".to_string(),
                "movement_speed".to_string(),
                "health".to_string(),
            ],
        };
        assert!(
            validate_rune_page_selection(&invalid_secondary_order, &domain).is_err(),
            "secondary runes out of slot order should fail validation"
        );
    }

    #[test]
    fn validate_rune_page_selection_rejects_invalid_shard_slot() {
        let domain = build_loadout_domain();
        let invalid_shard_slot = LoadoutSelection {
            rune_names: vec![
                "Lethal Tempo".to_string(),
                "Triumph".to_string(),
                "Legend: Alacrity".to_string(),
                "Last Stand".to_string(),
                "Conditioning".to_string(),
                "Overgrowth".to_string(),
            ],
            shard_stats: vec![
                "health".to_string(),
                "movement_speed".to_string(),
                "tenacity".to_string(),
            ],
        };
        assert!(
            validate_rune_page_selection(&invalid_shard_slot, &domain).is_err(),
            "slot 1 shard should reject unsupported stat keys"
        );
    }

    #[test]
    fn parse_simulation_config_rejects_legacy_max_time_field() {
        let simulation = serde_json::json!({
            "time_limit_seconds": 60.0,
            "max_time_seconds": 60.0
        });
        let error = parse_simulation_config(&simulation)
            .expect_err("legacy simulation.max_time_seconds should be rejected");
        assert!(
            error
                .to_string()
                .contains("simulation.max_time_seconds is no longer supported"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn parse_simulation_config_rejects_legacy_heartsteel_stack_field() {
        let simulation = serde_json::json!({
            "time_limit_seconds": 60.0,
            "heartsteel_assumed_stacks_at_8m": 20.0
        });
        let error = parse_simulation_config(&simulation)
            .expect_err("legacy simulation.heartsteel_assumed_stacks_at_8m should be rejected");
        assert!(
            error
                .to_string()
                .contains("simulation.heartsteel_assumed_stacks_at_8m is no longer supported"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn parse_simulation_config_rejects_legacy_enemy_uptime_field() {
        let simulation = serde_json::json!({
            "time_limit_seconds": 60.0,
            "enemy_uptime_model_enabled": true
        });
        let error = parse_simulation_config(&simulation)
            .expect_err("legacy simulation.enemy_uptime_model_enabled should be rejected");
        assert!(
            error
                .to_string()
                .contains("simulation.enemy_uptime_model_enabled is no longer supported"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn parse_simulation_config_rejects_legacy_item_stacks_map() {
        let simulation = serde_json::json!({
            "time_limit_seconds": 60.0,
            "item_stacks_at_level_20": {
                "Heartsteel": 20.0
            }
        });
        let error = parse_simulation_config(&simulation)
            .expect_err("legacy simulation.item_stacks_at_level_20 should be rejected");
        assert!(
            error
                .to_string()
                .contains("simulation.item_stacks_at_level_20 is no longer supported"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn parse_simulation_config_accepts_stack_overrides_by_identifier() {
        let simulation = serde_json::json!({
            "stack_overrides": {
                "heartsteel": 20.0
            }
        });
        let parsed =
            parse_simulation_config(&simulation).expect("simulation.stack_overrides should parse");
        let stacks = parsed
            .stack_overrides
            .get("heartsteel")
            .copied()
            .unwrap_or_default();
        assert!(
            (stacks - 20.0).abs() < 1e-9,
            "unexpected stack value: {stacks}"
        );
    }

    #[test]
    fn parse_simulation_config_uses_default_time_limit_when_missing() {
        let simulation = serde_json::json!({});
        let parsed = parse_simulation_config(&simulation)
            .expect("simulation config should parse with default time limit");
        assert!(
            (parsed.max_time_seconds - 1200.0).abs() < 1e-9,
            "expected default time_limit_seconds of 1200.0, got {}",
            parsed.max_time_seconds
        );
    }
}

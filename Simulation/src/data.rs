use anyhow::{Context, Result, anyhow, bail};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use super::{
    BuildSearchConfig, ChampionBase, EXCLUDED_RANKS, EnemyConfig, ITEM_EVOLUTION_REPLACEMENTS,
    Item, LEGENDARY_RANK, LoadoutSelection, MasterySelection, ResolvedLoadout,
    SearchQualityProfile, SimulationConfig, Stats, UrfBuffs, rand_index, shuffle_usize,
};

pub(crate) fn simulation_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
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

pub(crate) fn masteries_dir() -> PathBuf {
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
    for_vlad: bool,
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
            if for_vlad {
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
    for_vlad: bool,
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
        for_vlad,
    ))
}

pub(crate) fn resolve_loadout(
    selection: &LoadoutSelection,
    level: usize,
    for_vlad: bool,
) -> Result<ResolvedLoadout> {
    let runes_data = load_json(&masteries_dir().join("RunesReforged.json"))?;
    let masteries_data = load_json(&masteries_dir().join("Season2016.json"))?;

    let mut runes_by_id: HashMap<i64, Value> = HashMap::new();
    let mut runes_by_name: HashMap<String, Value> = HashMap::new();
    if let Some(paths) = runes_data.get("paths").and_then(Value::as_array) {
        for path in paths {
            if let Some(slots) = path.get("slots").and_then(Value::as_array) {
                for slot in slots {
                    if let Some(runes) = slot.get("runes").and_then(Value::as_array) {
                        for rune in runes {
                            if let Some(id) = rune.get("id").and_then(Value::as_i64) {
                                runes_by_id.insert(id, rune.clone());
                            }
                            if let Some(name) = rune.get("name").and_then(Value::as_str) {
                                runes_by_name.insert(to_norm_key(name), rune.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    let mastery_by_name = masteries_data
        .get("masteries")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    m.get("display_name")
                        .and_then(Value::as_str)
                        .map(|name| (to_norm_key(name), m.clone()))
                })
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default();

    let mut out = ResolvedLoadout::default();

    for id in &selection.rune_ids {
        if let Some(rune) = runes_by_id.get(id) {
            let name = rune
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("unknown rune");
            out.selection_labels.push(format!("Rune: {}", name));
            for effect in rune
                .get("effects_structured")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
            {
                if apply_structured_effect(&effect, 1, level, for_vlad, &mut out.bonus_stats)? {
                    out.applied_notes
                        .push(format!("Applied rune stat effect from {}.", name));
                }
            }
        } else {
            out.skipped_notes
                .push(format!("Rune id {} not found in RunesReforged.", id));
        }
    }

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
                if apply_structured_effect(&effect, 1, level, for_vlad, &mut out.bonus_stats)? {
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
                if apply_stat_bonus(&mut out.bonus_stats, stat, val, is_percent, for_vlad) {
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

    for mastery in &selection.masteries {
        let key = to_norm_key(&mastery.name);
        let Some(m) = mastery_by_name.get(&key) else {
            out.skipped_notes.push(format!(
                "Mastery '{}' not found in Season2016.",
                mastery.name
            ));
            continue;
        };
        let max_ranks = m.get("ranks").and_then(Value::as_u64).unwrap_or(1) as usize;
        let rank = mastery.rank.clamp(1, max_ranks);
        let name = m
            .get("display_name")
            .and_then(Value::as_str)
            .unwrap_or(&mastery.name);
        out.selection_labels
            .push(format!("Mastery: {} ({}/{})", name, rank, max_ranks));
        for effect in m
            .get("effects_structured")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
        {
            if apply_structured_effect(&effect, rank, level, for_vlad, &mut out.bonus_stats)? {
                out.applied_notes
                    .push(format!("Applied mastery stat effect from {}.", name));
            }
        }
    }

    Ok(out)
}

pub(crate) fn as_f64(obj: &Value, key: &str) -> Result<f64> {
    obj.get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing f64 key: {}", key))
}

pub(crate) fn as_str<'a>(obj: &'a Value, key: &str) -> Result<&'a str> {
    obj.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Missing string key: {}", key))
}

pub(crate) fn parse_simulation_config(data: &Value) -> Result<SimulationConfig> {
    let server_tick_rate_hz = data
        .get("server_tick_rate_hz")
        .and_then(Value::as_f64)
        .unwrap_or(30.0);
    let dt = data.get("dt").and_then(Value::as_f64).unwrap_or_else(|| {
        if server_tick_rate_hz > 0.0 {
            1.0 / server_tick_rate_hz
        } else {
            0.05
        }
    });

    let base_damage = data
        .get("vlad_pool_base_damage_by_rank")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing vlad_pool_base_damage_by_rank"))?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| anyhow!("Invalid base damage")))
        .collect::<Result<Vec<_>>>()?;

    Ok(SimulationConfig {
        dt,
        server_tick_rate_hz,
        champion_level: data
            .get("champion_level")
            .and_then(Value::as_u64)
            .unwrap_or(20) as usize,
        max_time_seconds: as_f64(data, "max_time_seconds")?,
        vlad_pool_rank: data
            .get("vlad_pool_rank")
            .and_then(Value::as_u64)
            .ok_or_else(|| anyhow!("Missing vlad_pool_rank"))? as usize,
        vlad_pool_untargetable_seconds: as_f64(data, "vlad_pool_untargetable_seconds")?,
        vlad_pool_cost_percent_current_health: as_f64(
            data,
            "vlad_pool_cost_percent_current_health",
        )?,
        vlad_pool_heal_ratio_of_damage: as_f64(data, "vlad_pool_heal_ratio_of_damage")?,
        vlad_pool_base_damage_by_rank: base_damage,
        vlad_pool_bonus_health_ratio: as_f64(data, "vlad_pool_bonus_health_ratio")?,
        zhonya_duration_seconds: as_f64(data, "zhonya_duration_seconds")?,
        zhonya_cooldown_seconds: as_f64(data, "zhonya_cooldown_seconds")?,
        zhonya_trigger_health_percent: as_f64(data, "zhonya_trigger_health_percent")?,
        ga_cooldown_seconds: as_f64(data, "ga_cooldown_seconds")?,
        ga_revive_duration_seconds: as_f64(data, "ga_revive_duration_seconds")?,
        ga_revive_base_health_ratio: as_f64(data, "ga_revive_base_health_ratio")?,
        protoplasm_trigger_health_percent: as_f64(data, "protoplasm_trigger_health_percent")?,
        protoplasm_bonus_health: as_f64(data, "protoplasm_bonus_health")?,
        protoplasm_heal_total: as_f64(data, "protoplasm_heal_total")?,
        protoplasm_duration_seconds: as_f64(data, "protoplasm_duration_seconds")?,
        heartsteel_assumed_stacks_at_8m: data
            .get("heartsteel_assumed_stacks_at_8m")
            .and_then(Value::as_f64)
            .unwrap_or(20.0),
        enemy_uptime_model_enabled: data
            .get("enemy_uptime_model_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        urf_respawn_flat_reduction_seconds: data
            .get("urf_respawn_flat_reduction_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(3.0),
        urf_respawn_extrapolation_per_level: data
            .get("urf_respawn_extrapolation_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(2.5),
        vlad_q_base_damage: data
            .get("vlad_q_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(220.0),
        vlad_q_ap_ratio: data
            .get("vlad_q_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.60),
        vlad_q_heal_ratio_of_damage: data
            .get("vlad_q_heal_ratio_of_damage")
            .and_then(Value::as_f64)
            .unwrap_or(0.30),
        vlad_q_base_cooldown_seconds: data
            .get("vlad_q_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(4.0),
        vlad_e_base_damage: data
            .get("vlad_e_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(180.0),
        vlad_e_ap_ratio: data
            .get("vlad_e_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.50),
        vlad_e_base_cooldown_seconds: data
            .get("vlad_e_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(8.0),
        vlad_r_base_damage: data
            .get("vlad_r_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(350.0),
        vlad_r_ap_ratio: data
            .get("vlad_r_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.70),
        vlad_r_base_cooldown_seconds: data
            .get("vlad_r_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(90.0),
    })
}

pub(crate) fn parse_champion_base(data: &Value) -> Result<ChampionBase> {
    Ok(ChampionBase {
        name: as_str(data, "name")?.to_string(),
        base_health: as_f64(data, "base_health")?,
        health_per_level: data
            .get("health_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_armor: as_f64(data, "base_armor")?,
        armor_per_level: data
            .get("armor_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_magic_resist: as_f64(data, "base_magic_resist")?,
        magic_resist_per_level: data
            .get("magic_resist_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_attack_damage: as_f64(data, "base_attack_damage")?,
        attack_damage_per_level: data
            .get("attack_damage_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_attack_speed: as_f64(data, "base_attack_speed")?,
        attack_speed_per_level_percent: data
            .get("attack_speed_per_level_percent")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_move_speed: as_f64(data, "base_move_speed")?,
        is_melee: data
            .get("is_melee")
            .and_then(Value::as_bool)
            .ok_or_else(|| anyhow!("Missing is_melee"))?,
    })
}

pub(crate) fn parse_enemy_config(
    data: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
) -> Result<EnemyConfig> {
    let base = if let Some(champion) = data.get("champion").and_then(Value::as_str) {
        lookup_champion_base(champion_bases, champion)?
    } else if let Some(legacy) = data.get("base") {
        parse_champion_base(legacy)?
    } else {
        bail!("Enemy requires champion or base field");
    };
    Ok(EnemyConfig {
        name: data
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or(&base.name)
            .to_string(),
        base,
        ability_dps_flat: data
            .get("ability_dps_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_dps_ad_ratio: data
            .get("ability_dps_ad_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_dps_ap_ratio: data
            .get("ability_dps_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_tick_interval_seconds: data
            .get("ability_tick_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        stun_interval_seconds: data
            .get("stun_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        stun_duration_seconds: data
            .get("stun_duration_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_interval_seconds: data
            .get("burst_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_start_offset_seconds: data
            .get("burst_start_offset_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_magic_flat: data
            .get("burst_magic_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_physical_flat: data
            .get("burst_physical_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_true_flat: data
            .get("burst_true_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_ad_ratio: data
            .get("burst_ad_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_ap_ratio: data
            .get("burst_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_cycle_seconds: data
            .get("uptime_cycle_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_active_seconds: data
            .get("uptime_active_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_phase_seconds: data
            .get("uptime_phase_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
    })
}

pub(crate) fn parse_build_search(data: &Value) -> Result<BuildSearchConfig> {
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
        beam_width: data.get("beam_width").and_then(Value::as_u64).unwrap_or(20) as usize,
        max_items: data.get("max_items").and_then(Value::as_u64).unwrap_or(6) as usize,
        random_samples: data
            .get("random_samples")
            .and_then(Value::as_u64)
            .unwrap_or(200) as usize,
        hill_climb_restarts: data
            .get("hill_climb_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(64) as usize,
        hill_climb_steps: data
            .get("hill_climb_steps")
            .and_then(Value::as_u64)
            .unwrap_or(20) as usize,
        hill_climb_neighbors: data
            .get("hill_climb_neighbors")
            .and_then(Value::as_u64)
            .unwrap_or(24) as usize,
        genetic_population: data
            .get("genetic_population")
            .and_then(Value::as_u64)
            .unwrap_or(80) as usize,
        genetic_generations: data
            .get("genetic_generations")
            .and_then(Value::as_u64)
            .unwrap_or(30) as usize,
        genetic_mutation_rate: data
            .get("genetic_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.18),
        genetic_crossover_rate: data
            .get("genetic_crossover_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.90),
        portfolio_strategies,
        ranked_limit: data
            .get("ranked_limit")
            .and_then(Value::as_u64)
            .unwrap_or(400) as usize,
        simulated_annealing_restarts: data
            .get("simulated_annealing_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(32) as usize,
        simulated_annealing_iterations: data
            .get("simulated_annealing_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(220) as usize,
        simulated_annealing_initial_temp: data
            .get("simulated_annealing_initial_temp")
            .and_then(Value::as_f64)
            .unwrap_or(1.2),
        simulated_annealing_cooling_rate: data
            .get("simulated_annealing_cooling_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.985),
        mcts_iterations: data
            .get("mcts_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(5000) as usize,
        mcts_rollouts_per_expansion: data
            .get("mcts_rollouts_per_expansion")
            .and_then(Value::as_u64)
            .unwrap_or(2) as usize,
        mcts_exploration: data
            .get("mcts_exploration")
            .and_then(Value::as_f64)
            .unwrap_or(1.2),
        ensemble_seeds: data
            .get("ensemble_seeds")
            .and_then(Value::as_u64)
            .unwrap_or(3) as usize,
        ensemble_seed_stride: data
            .get("ensemble_seed_stride")
            .and_then(Value::as_u64)
            .unwrap_or(1_000_003),
        ensemble_seed_top_k: data
            .get("ensemble_seed_top_k")
            .and_then(Value::as_u64)
            .unwrap_or(25) as usize,
        objective_survival_weight: data
            .get("objective_survival_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.55),
        objective_damage_weight: data
            .get("objective_damage_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.30),
        objective_healing_weight: data
            .get("objective_healing_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.15),
        robust_min_seed_hit_rate: data
            .get("robust_min_seed_hit_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.5),
        bleed_enabled: data
            .get("bleed_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        bleed_budget: data
            .get("bleed_budget")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize,
        bleed_mutation_rate: data
            .get("bleed_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.35),
        multi_scenario_worst_weight: data
            .get("multi_scenario_worst_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.35),
        seed: data.get("seed").and_then(Value::as_u64).unwrap_or(1337),
    })
}

pub(crate) fn apply_search_quality_profile(
    search: &mut BuildSearchConfig,
    profile: SearchQualityProfile,
) {
    match profile {
        SearchQualityProfile::Fast => {
            search.beam_width = 24;
            search.random_samples = 192;
            search.hill_climb_restarts = 24;
            search.hill_climb_steps = 12;
            search.hill_climb_neighbors = 12;
            search.genetic_population = 48;
            search.genetic_generations = 14;
            search.simulated_annealing_restarts = 12;
            search.simulated_annealing_iterations = 96;
            search.mcts_iterations = 1200;
            search.mcts_rollouts_per_expansion = 1;
            search.ensemble_seeds = 1;
            search.ensemble_seed_top_k = 10;
            search.ranked_limit = 200;
            search.bleed_budget = 120;
        }
        SearchQualityProfile::Balanced => {
            search.beam_width = 36;
            search.random_samples = 360;
            search.hill_climb_restarts = 48;
            search.hill_climb_steps = 18;
            search.hill_climb_neighbors = 20;
            search.genetic_population = 72;
            search.genetic_generations = 24;
            search.simulated_annealing_restarts = 24;
            search.simulated_annealing_iterations = 180;
            search.mcts_iterations = 2600;
            search.mcts_rollouts_per_expansion = 2;
            search.ensemble_seeds = 2;
            search.ensemble_seed_top_k = 18;
            search.ranked_limit = 320;
            search.bleed_budget = 300;
        }
        SearchQualityProfile::MaximumQuality => {
            search.beam_width = search.beam_width.max(64);
            search.random_samples = search.random_samples.max(900);
            search.hill_climb_restarts = search.hill_climb_restarts.max(128);
            search.hill_climb_steps = search.hill_climb_steps.max(28);
            search.hill_climb_neighbors = search.hill_climb_neighbors.max(30);
            search.genetic_population = search.genetic_population.max(140);
            search.genetic_generations = search.genetic_generations.max(52);
            search.simulated_annealing_restarts = search.simulated_annealing_restarts.max(56);
            search.simulated_annealing_iterations = search.simulated_annealing_iterations.max(340);
            search.mcts_iterations = search.mcts_iterations.max(9000);
            search.mcts_rollouts_per_expansion = search.mcts_rollouts_per_expansion.max(3);
            search.ensemble_seeds = search.ensemble_seeds.max(4);
            search.ensemble_seed_top_k = search.ensemble_seed_top_k.max(36);
            search.ranked_limit = search.ranked_limit.max(640);
            search.bleed_budget = search.bleed_budget.max(1200);
        }
    }
}

pub(crate) fn parse_loadout_selection(data: Option<&Value>) -> LoadoutSelection {
    let mut out = LoadoutSelection::default();
    let Some(obj) = data.and_then(Value::as_object) else {
        return out;
    };

    if let Some(runes_obj) = obj.get("runes_reforged").and_then(Value::as_object) {
        if let Some(ids) = runes_obj.get("rune_ids").and_then(Value::as_array) {
            out.rune_ids = ids.iter().filter_map(Value::as_i64).collect();
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

    if let Some(masteries) = obj.get("season2016_masteries").and_then(Value::as_array) {
        for entry in masteries {
            if let Some(name) = entry.as_str() {
                out.masteries.push(MasterySelection {
                    name: name.to_string(),
                    rank: 1,
                });
                continue;
            }
            let Some(mo) = entry.as_object() else {
                continue;
            };
            let Some(name) = mo.get("name").and_then(Value::as_str) else {
                continue;
            };
            let rank = mo.get("rank").and_then(Value::as_u64).unwrap_or(1) as usize;
            out.masteries.push(MasterySelection {
                name: name.to_string(),
                rank,
            });
        }
    }
    out
}

pub(crate) fn loadout_selection_key(sel: &LoadoutSelection) -> String {
    let mut runes = sel.rune_names.clone();
    runes.sort();
    let mut shards = sel.shard_stats.clone();
    shards.sort();
    let mut masteries = sel
        .masteries
        .iter()
        .map(|m| format!("{}:{}", m.name, m.rank))
        .collect::<Vec<_>>();
    masteries.sort();
    format!(
        "r={}|s={}|m={}",
        runes.join(","),
        shards.join(","),
        masteries.join(",")
    )
}

#[derive(Debug, Clone)]
pub(crate) struct RunePathDomain {
    pub(crate) slot_runes: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub(crate) struct MasteryOptionDomain {
    pub(crate) name: String,
    pub(crate) max_rank: usize,
    pub(crate) points_required_in_tree: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct MasteryTierDomain {
    pub(crate) points_required: usize,
    pub(crate) points_available: usize,
    pub(crate) is_keystone_tier: bool,
    pub(crate) options: Vec<MasteryOptionDomain>,
}

#[derive(Debug, Clone)]
pub(crate) struct MasteryTreeDomain {
    pub(crate) tiers: Vec<MasteryTierDomain>,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutDomain {
    pub(crate) rune_paths: Vec<RunePathDomain>,
    pub(crate) shard_slots: [Vec<String>; 3],
    pub(crate) mastery_trees: Vec<MasteryTreeDomain>,
    pub(crate) mastery_primary_points: usize,
    pub(crate) mastery_secondary_points: usize,
    pub(crate) mastery_keystone_requirement: usize,
}

pub(crate) fn build_loadout_domain() -> LoadoutDomain {
    let runes_data = load_json(&masteries_dir().join("RunesReforged.json")).unwrap_or(Value::Null);
    let masteries_data = load_json(&masteries_dir().join("Season2016.json")).unwrap_or(Value::Null);

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

    let mastery_lookup = masteries_data
        .get("masteries")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    let name = m.get("display_name").and_then(Value::as_str)?;
                    let ranks = m.get("ranks").and_then(Value::as_u64).unwrap_or(1) as usize;
                    let points_required_in_tree = m
                        .get("points_required_in_tree")
                        .and_then(Value::as_u64)
                        .unwrap_or(0) as usize;
                    Some((
                        to_norm_key(name),
                        MasteryOptionDomain {
                            name: name.to_string(),
                            max_rank: ranks.max(1),
                            points_required_in_tree,
                        },
                    ))
                })
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default();

    let mastery_trees = masteries_data
        .get("trees")
        .and_then(Value::as_array)
        .map(|trees| {
            trees
                .iter()
                .map(|tree| {
                    let tiers = tree
                        .get("tiers")
                        .and_then(Value::as_array)
                        .map(|tier_arr| {
                            tier_arr
                                .iter()
                                .map(|tier| {
                                    let options = tier
                                        .get("masteries")
                                        .and_then(Value::as_array)
                                        .map(|names| {
                                            names
                                                .iter()
                                                .filter_map(Value::as_str)
                                                .filter_map(|name| {
                                                    mastery_lookup.get(&to_norm_key(name)).cloned()
                                                })
                                                .collect::<Vec<_>>()
                                        })
                                        .unwrap_or_default();
                                    MasteryTierDomain {
                                        points_required: tier
                                            .get("points_in_tree_required")
                                            .and_then(Value::as_u64)
                                            .unwrap_or(0)
                                            as usize,
                                        points_available: tier
                                            .get("points_available")
                                            .and_then(Value::as_u64)
                                            .unwrap_or(5)
                                            as usize,
                                        is_keystone_tier: tier
                                            .get("is_keystone_tier")
                                            .and_then(Value::as_bool)
                                            .unwrap_or(false),
                                        options,
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    MasteryTreeDomain { tiers }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let rules = masteries_data
        .get("selection_rules")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    LoadoutDomain {
        rune_paths,
        shard_slots,
        mastery_trees,
        mastery_primary_points: rules
            .get("primary_tree_points")
            .and_then(Value::as_u64)
            .unwrap_or(18) as usize,
        mastery_secondary_points: rules
            .get("secondary_tree_points")
            .and_then(Value::as_u64)
            .unwrap_or(12) as usize,
        mastery_keystone_requirement: rules
            .get("keystone_requirement_points_in_tree")
            .and_then(Value::as_u64)
            .unwrap_or(17) as usize,
    }
}

pub(crate) fn random_tree_masteries(
    tree: &MasteryTreeDomain,
    target_points: usize,
    keystone_requirement: usize,
    seed: &mut u64,
) -> Option<Vec<MasterySelection>> {
    if tree.tiers.is_empty() || target_points == 0 {
        return Some(Vec::new());
    }
    for _ in 0..128 {
        let mut points = 0usize;
        let mut tier_spent = vec![0usize; tree.tiers.len()];
        let mut ranks = tree
            .tiers
            .iter()
            .map(|t| vec![0usize; t.options.len()])
            .collect::<Vec<_>>();

        while points < target_points {
            let mut choices = Vec::new();
            for (tier_idx, tier) in tree.tiers.iter().enumerate() {
                if tier.options.is_empty()
                    || points < tier.points_required
                    || tier_spent[tier_idx] >= tier.points_available
                {
                    continue;
                }
                if tier.is_keystone_tier && points < keystone_requirement {
                    continue;
                }
                for (opt_idx, opt) in tier.options.iter().enumerate() {
                    if ranks[tier_idx][opt_idx] >= opt.max_rank
                        || points < opt.points_required_in_tree
                    {
                        continue;
                    }
                    choices.push((tier_idx, opt_idx));
                }
            }
            if choices.is_empty() {
                break;
            }
            let (tier_idx, opt_idx) = choices[rand_index(seed, choices.len())];
            ranks[tier_idx][opt_idx] += 1;
            tier_spent[tier_idx] += 1;
            points += 1;
        }

        if points != target_points {
            continue;
        }
        let mut out = Vec::new();
        for (tier_idx, tier) in tree.tiers.iter().enumerate() {
            for (opt_idx, opt) in tier.options.iter().enumerate() {
                let rank = ranks[tier_idx][opt_idx];
                if rank > 0 {
                    out.push(MasterySelection {
                        name: opt.name.clone(),
                        rank,
                    });
                }
            }
        }
        return Some(out);
    }
    None
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
                    let sa = picks[0];
                    let sb = picks[1];
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
                    out.rune_ids.clear();
                    out.shard_stats = domain
                        .shard_slots
                        .iter()
                        .map(|slot| slot[rand_index(seed, slot.len())].clone())
                        .collect::<Vec<_>>();
                }
            }
        }
    }

    if domain.mastery_trees.len() >= 2 {
        let primary_idx = rand_index(seed, domain.mastery_trees.len());
        let secondary_choices = (0..domain.mastery_trees.len())
            .filter(|idx| *idx != primary_idx)
            .collect::<Vec<_>>();
        if !secondary_choices.is_empty() {
            let secondary_idx = secondary_choices[rand_index(seed, secondary_choices.len())];
            let primary = random_tree_masteries(
                &domain.mastery_trees[primary_idx],
                domain.mastery_primary_points,
                domain.mastery_keystone_requirement,
                seed,
            );
            let secondary = random_tree_masteries(
                &domain.mastery_trees[secondary_idx],
                domain.mastery_secondary_points,
                domain.mastery_keystone_requirement,
                seed,
            );
            if let (Some(mut p), Some(s)) = (primary, secondary) {
                p.extend(s);
                out.masteries = p;
            }
        }
    }

    out
}

pub(crate) fn loadout_eval_budget(
    search: &BuildSearchConfig,
    profile: SearchQualityProfile,
) -> usize {
    let base = search
        .random_samples
        .max(search.beam_width * 8)
        .max(search.hill_climb_restarts * 4)
        .max(search.genetic_population * 2)
        .max(128);
    match profile {
        SearchQualityProfile::Fast => base.min(256),
        SearchQualityProfile::Balanced => base.min(1024),
        SearchQualityProfile::MaximumQuality => base.min(4096),
    }
}

pub(crate) fn normalize_name(input: &str) -> String {
    input
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
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
        let data = load_json(&path)?;
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid character filename"))?;
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

pub(crate) fn default_item_pool(items: &HashMap<String, Item>) -> Vec<Item> {
    let mut pool = items
        .values()
        .filter(|item| item.shop_purchasable || is_evolution_target(&item.name))
        .filter(|item| is_legendary(item))
        .filter(|item| !is_pre_evolution_item(items, &item.name))
        .filter(|item| !looks_arena_or_non_summoners_rift(item))
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
    pub(crate) masteries: Vec<MasterySelection>,
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
        let masteries = preset
            .get("masteries")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing masteries for preset {}", champion))?
            .iter()
            .filter_map(|m| {
                let name = m.get("name").and_then(Value::as_str)?;
                let rank = m.get("rank").and_then(Value::as_u64).unwrap_or(1) as usize;
                Some(MasterySelection {
                    name: name.to_string(),
                    rank,
                })
            })
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
            masteries,
        };
        by_champion.insert(to_norm_key(&champion), loaded);
    }
    Ok(by_champion)
}

pub(crate) fn validate_enemy_urf_presets(
    presets: &HashMap<String, EnemyUrfPreset>,
    items: &HashMap<String, Item>,
    loadout_domain: &LoadoutDomain,
) -> Result<()> {
    let all_runes = loadout_domain
        .rune_paths
        .iter()
        .flat_map(|p| p.slot_runes.iter())
        .flat_map(|slot| slot.iter())
        .map(|s| to_norm_key(s))
        .collect::<HashSet<_>>();
    let all_masteries = loadout_domain
        .mastery_trees
        .iter()
        .flat_map(|t| t.tiers.iter())
        .flat_map(|tier| tier.options.iter())
        .map(|m| to_norm_key(&m.name))
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
        for mastery in &preset.masteries {
            if !all_masteries.contains(&to_norm_key(&mastery.name)) {
                bail!(
                    "Enemy preset mastery '{}' for {} is not present in Season2016",
                    mastery.name,
                    preset.champion
                );
            }
        }
    }
    Ok(())
}

pub(crate) fn enemy_loadout_from_preset(preset: &EnemyUrfPreset) -> LoadoutSelection {
    LoadoutSelection {
        rune_ids: Vec::new(),
        rune_names: preset.runes.clone(),
        shard_stats: preset.shards.clone(),
        masteries: preset.masteries.clone(),
    }
}

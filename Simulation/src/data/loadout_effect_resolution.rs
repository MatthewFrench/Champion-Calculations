use super::*;

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
        "tenacity" => {
            stats.tenacity_percent += value;
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
            // Only percentage cooldown reduction maps to deterministic ability haste.
            // Cooldown values expressed in seconds describe ability/summoner timings and should
            // not inflate champion runtime stats.
            if !is_percent_unit {
                return false;
            }
            let pct = value / 100.0;
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
            let mut static_effect_applied = false;
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
                    static_effect_applied = true;
                    out.applied_notes
                        .push(format!("Applied rune stat effect from {}.", real_name));
                }
            }
            if for_controlled_champion
                && !static_effect_applied
                && !has_dynamic_rune_effect(real_name)
            {
                out.unmodeled_rune_names.push(real_name.to_string());
                out.skipped_notes.push(format!(
                    "Rune '{}' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.",
                    real_name
                ));
            }
        } else {
            out.skipped_notes
                .push(format!("Rune '{}' not found in RunesReforged.", name));
        }
    }

    if let Some(shards) = runes_data.get("stat_shards").and_then(Value::as_array) {
        for (idx, shard_key) in selection.shard_stats.iter().enumerate() {
            out.selection_labels
                .push(format!("Shard {}: {}", idx + 1, shard_key));
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

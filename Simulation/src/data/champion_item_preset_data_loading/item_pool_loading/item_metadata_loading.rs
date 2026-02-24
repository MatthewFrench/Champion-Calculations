use super::*;

fn stat_key_map(key: &str) -> Option<&'static str> {
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

fn add_stat_value(stats: &mut Stats, stat_key: &str, value: f64) {
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

fn has_active_effect(data: &Value) -> bool {
    data.get("active")
        .and_then(Value::as_object)
        .map(|active| {
            active
                .get("name")
                .and_then(Value::as_str)
                .is_some_and(|name| !name.trim().is_empty())
                || active
                    .get("effects")
                    .and_then(Value::as_str)
                    .is_some_and(|text| !text.trim().is_empty())
                || active
                    .get("effects_structured")
                    .and_then(Value::as_array)
                    .is_some_and(|effects| !effects.is_empty())
        })
        .unwrap_or(false)
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
        let structured_effect_count = data
            .get("effects_structured")
            .and_then(Value::as_array)
            .map(Vec::len)
            .unwrap_or(0);

        let Some(name) = data
            .get("name")
            .and_then(Value::as_str)
            .map(ToString::to_string)
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
                has_active_effect: has_active_effect(&data),
                structured_effect_count,
            },
        );
    }

    Ok(items)
}

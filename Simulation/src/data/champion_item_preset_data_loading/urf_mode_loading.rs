use super::*;

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

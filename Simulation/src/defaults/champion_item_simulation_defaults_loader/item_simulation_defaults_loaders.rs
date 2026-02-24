use super::super::*;
use super::simulation_defaults_extraction_helpers::*;

pub(in super::super) fn load_zhonya_time_stop_defaults() -> Result<ZhonyaTimeStopDefaults> {
    let (item_path, item_data) = read_item_file("Zhonyas Hourglass.json")?;
    let effects = item_effects(&item_data, &item_path)?;
    let time_stop = ability_effect_by_id(effects, "zhonyas_time_stop").ok_or_else(|| {
        anyhow!(
            "Missing effects_structured[id=zhonyas_time_stop] in {}",
            item_path.display()
        )
    })?;
    let duration_seconds = time_stop
        .get("status_effects")
        .and_then(Value::as_array)
        .and_then(|effects| {
            effects.iter().find_map(|status| {
                let status_type = status.get("type").and_then(Value::as_str)?;
                if status_type.eq_ignore_ascii_case("stasis") {
                    status.get("duration_seconds").and_then(Value::as_f64)
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "Missing stasis duration in effects_structured[id=zhonyas_time_stop] in {}",
                item_path.display()
            )
        })?;
    let cooldown_seconds = time_stop
        .get("cooldown_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing cooldown_seconds in effects_structured[id=zhonyas_time_stop] in {}",
                item_path.display()
            )
        })?;
    Ok(ZhonyaTimeStopDefaults {
        duration_seconds,
        cooldown_seconds,
    })
}

pub(in super::super) fn load_guardian_angel_rebirth_defaults()
-> Result<GuardianAngelRebirthDefaults> {
    let (item_path, item_data) = read_item_file("Guardian Angel.json")?;
    let effects = item_effects(&item_data, &item_path)?;
    let rebirth = ability_effect_by_id(
        effects,
        "rebirth_resurrection_with_post_revive_health_and_mana_restore",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing Guardian Angel rebirth effect id in {}",
            item_path.display()
        )
    })?;
    let cooldown_seconds = rebirth
        .get("cooldown_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing cooldown_seconds in Guardian Angel rebirth effect in {}",
                item_path.display()
            )
        })?;
    let revive_duration_seconds = rebirth
        .get("duration_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing duration_seconds in Guardian Angel rebirth effect in {}",
                item_path.display()
            )
        })?;
    let revive_base_health_ratio = rebirth
        .get("scaling_terms")
        .and_then(Value::as_array)
        .and_then(|terms| {
            terms.iter().find_map(|term| {
                let applies_to = term.get("applies_to").and_then(Value::as_str)?;
                if applies_to != "heal_on_revive" {
                    return None;
                }
                let stat = term.get("stat").and_then(Value::as_str)?;
                if stat != "base_health" {
                    return None;
                }
                term.get("ratio").and_then(Value::as_f64)
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "Missing base_health revive ratio in Guardian Angel rebirth effect in {}",
                item_path.display()
            )
        })?;
    Ok(GuardianAngelRebirthDefaults {
        cooldown_seconds,
        revive_duration_seconds,
        revive_base_health_ratio,
    })
}

pub(in super::super) fn load_protoplasm_lifeline_defaults() -> Result<ProtoplasmLifelineDefaults> {
    let (item_path, item_data) = read_item_file("Protoplasm Harness.json")?;
    let effects = item_effects(&item_data, &item_path)?;
    let lifeline_bonus =
        ability_effect_by_id(effects, "lifeline_gain_bonus_health_below_health_threshold")
            .ok_or_else(|| {
                anyhow!(
                    "Missing Protoplasm lifeline bonus health effect in {}",
                    item_path.display()
                )
            })?;
    let lifeline_heal =
        ability_effect_by_id(effects, "lifeline_heal_over_time_scaling_with_resists").ok_or_else(
            || {
                anyhow!(
                    "Missing Protoplasm lifeline heal effect in {}",
                    item_path.display()
                )
            },
        )?;
    let trigger_health_percent =
        ratio_from_health_threshold_condition(lifeline_bonus).ok_or_else(|| {
            anyhow!(
                "Missing parseable health threshold condition in Protoplasm lifeline effect in {}",
                item_path.display()
            )
        })?;
    let (bonus_health_min, bonus_health_max) =
        effect_value_range(lifeline_bonus).ok_or_else(|| {
            anyhow!(
                "Missing value_range in Protoplasm lifeline bonus health effect in {}",
                item_path.display()
            )
        })?;
    let (heal_total_min, heal_total_max) = effect_value_range(lifeline_heal).ok_or_else(|| {
        anyhow!(
            "Missing value_range in Protoplasm lifeline heal effect in {}",
            item_path.display()
        )
    })?;
    let duration_seconds = lifeline_bonus
        .get("duration_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing duration_seconds in Protoplasm lifeline bonus effect in {}",
                item_path.display()
            )
        })?;
    Ok(ProtoplasmLifelineDefaults {
        trigger_health_percent,
        bonus_health_min,
        bonus_health_max,
        heal_total_min,
        heal_total_max,
        duration_seconds,
    })
}

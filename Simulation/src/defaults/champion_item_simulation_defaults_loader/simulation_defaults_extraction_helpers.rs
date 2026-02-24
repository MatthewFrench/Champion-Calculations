use super::super::*;

pub(super) fn effect_value_range(effect: &Value) -> Option<(f64, f64)> {
    let min = effect.pointer("/value_range/min").and_then(Value::as_f64)?;
    let max = effect.pointer("/value_range/max").and_then(Value::as_f64)?;
    Some((min, max))
}

pub(super) fn ratio_from_health_threshold_condition(effect: &Value) -> Option<f64> {
    let conditions = effect.get("conditions").and_then(Value::as_array)?;
    for condition in conditions {
        let raw = condition.as_str()?.trim().to_ascii_lowercase();
        if !raw.starts_with("health_below_") || !raw.ends_with("_percent") {
            continue;
        }
        let middle = raw
            .strip_prefix("health_below_")
            .and_then(|value| value.strip_suffix("_percent"))?;
        let percent = middle.parse::<f64>().ok()?;
        return Some(percent / 100.0);
    }
    None
}

pub(super) fn champion_ability<'a>(
    champion_data: &'a Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<&'a Value> {
    champion_data
        .get("abilities")
        .and_then(|abilities| abilities.get(ability_key))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{} in {}",
                ability_key,
                champion_path.display()
            )
        })
}

pub(super) fn champion_ability_effects<'a>(
    ability: &'a Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<&'a [Value]> {
    ability
        .get("effects")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{}.effects in {}",
                ability_key,
                champion_path.display()
            )
        })
}

pub(super) fn champion_ability_range(
    ability: &Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<f64> {
    ability.get("range").and_then(Value::as_f64).ok_or_else(|| {
        anyhow!(
            "Missing abilities.{}.range in {}",
            ability_key,
            champion_path.display()
        )
    })
}

pub(super) fn champion_ability_cooldown_seconds(
    ability: &Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<f64> {
    ability
        .get("cooldown_seconds_by_rank")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{}.cooldown_seconds_by_rank in {}",
                ability_key,
                champion_path.display()
            )
        })
}

pub(super) fn champion_ability_cooldown_seconds_by_rank(
    ability: &Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<Vec<f64>> {
    ability
        .get("cooldown_seconds_by_rank")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{}.cooldown_seconds_by_rank in {}",
                ability_key,
                champion_path.display()
            )
        })?
        .iter()
        .map(|value| {
            value.as_f64().ok_or_else(|| {
                anyhow!(
                    "Invalid abilities.{}.cooldown_seconds_by_rank value in {}",
                    ability_key,
                    champion_path.display()
                )
            })
        })
        .collect()
}

pub(super) fn champion_is_melee_from_data(champion_data: &Value) -> bool {
    if let Some(attack_type) = champion_data
        .get("basic_attack")
        .and_then(|basic| basic.get("attack_type"))
        .and_then(Value::as_str)
    {
        return attack_type.eq_ignore_ascii_case("melee");
    }
    champion_data
        .pointer("/base_stats/attack_range")
        .and_then(Value::as_f64)
        .map(|range| range <= 200.0)
        .unwrap_or(false)
}

pub(super) fn champion_ability_execution_defaults_for_role_internal(
    is_melee: bool,
) -> AbilityExecutionDefaultsEntry {
    let defaults = super::super::load_once_ref_or_hard_fail(
        &CHAMPION_ABILITY_EXECUTION_DEFAULTS,
        "champion_ability_execution_defaults",
        load_champion_ability_execution_defaults,
    );
    if is_melee {
        defaults.melee
    } else {
        defaults.ranged
    }
}

pub(super) fn champion_ability_execution_profile_from_ability(
    ability: &Value,
    is_melee: bool,
) -> AbilityExecutionProfile {
    let role_defaults = champion_ability_execution_defaults_for_role_internal(is_melee);
    let execution = ability
        .get("execution")
        .cloned()
        .and_then(|value| serde_json::from_value::<AbilityExecutionOverrideEntry>(value).ok())
        .unwrap_or_default();
    AbilityExecutionProfile {
        cast_windup_seconds: execution
            .cast_windup_seconds
            .unwrap_or(role_defaults.cast_windup_seconds),
        projectile_speed: execution
            .projectile_speed
            .unwrap_or(role_defaults.projectile_speed),
        effect_hitbox_radius: execution
            .effect_hitbox_radius
            .unwrap_or(role_defaults.effect_hitbox_radius),
    }
}

pub(super) fn effect_base_by_rank(effects: &[Value], effect_id: &str) -> Option<f64> {
    ability_effect_by_id(effects, effect_id)
        .and_then(|effect| effect.get("base_by_rank"))
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
}

pub(super) fn effect_base_by_rank_values(effects: &[Value], effect_id: &str) -> Option<Vec<f64>> {
    ability_effect_by_id(effects, effect_id)
        .and_then(|effect| effect.get("base_by_rank"))
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(Value::as_f64).collect::<Vec<_>>())
        .filter(|values| !values.is_empty())
}

pub(super) fn effect_formula_coefficient(effect: &Value, input_stat: &str) -> Option<f64> {
    effect
        .get("formula")
        .and_then(|formula| formula.get("terms"))
        .and_then(Value::as_array)
        .and_then(|terms| {
            terms
                .iter()
                .find(|term| term.get("input_stat").and_then(Value::as_str) == Some(input_stat))
        })
        .and_then(|term| term.get("coefficient"))
        .and_then(Value::as_f64)
}

pub(super) fn effect_formula_coefficient_by_id(
    effects: &[Value],
    effect_id: &str,
    input_stat: &str,
) -> Option<f64> {
    ability_effect_by_id(effects, effect_id)
        .and_then(|effect| effect_formula_coefficient(effect, input_stat))
}

pub(super) fn effect_contextual_multiplier_by_rank(effect: &Value) -> Option<f64> {
    effect
        .pointer("/formula/contextual_multipliers/by_rank")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
}

pub(super) fn effect_duration_seconds(effect: &Value) -> Option<f64> {
    effect
        .get("value_seconds_by_rank")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .or_else(|| effect.get("value_seconds").and_then(Value::as_f64))
}

pub(super) fn effect_duration_seconds_by_id(effects: &[Value], effect_id: &str) -> Option<f64> {
    ability_effect_by_id(effects, effect_id).and_then(effect_duration_seconds)
}

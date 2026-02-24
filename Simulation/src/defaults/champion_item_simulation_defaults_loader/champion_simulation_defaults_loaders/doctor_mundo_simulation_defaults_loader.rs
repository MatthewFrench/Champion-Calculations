use super::super::super::*;
use super::super::simulation_defaults_extraction_helpers::*;

pub(in super::super::super) fn load_doctor_mundo_infected_bonesaw_ability_defaults()
-> Result<DoctorMundoInfectedBonesawAbilityDefaults> {
    let path = repository_root_dir()
        .join("Characters")
        .join("DrMundo.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading DrMundo champion file: {}", path.display()))?;
    let data: Value = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing DrMundo champion file: {}", path.display()))?;

    let ability = data
        .get("abilities")
        .and_then(|abilities| abilities.get("basic_ability_1"))
        .ok_or_else(|| anyhow!("Missing abilities.basic_ability_1 in {}", path.display()))?;
    let champion_is_melee =
        super::super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(&data);
    let effects = ability
        .get("effects")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects in {}",
                path.display()
            )
        })?;

    let cast_range = ability
        .get("range")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.range in {}",
                path.display()
            )
        })?;
    let cooldown_seconds = champion_ability_cooldown_seconds(ability, "basic_ability_1", &path)?;

    let current_health_ratio = ability_effect_by_id(effects, "magic_damage_percent_current_health")
        .and_then(|effect| effect.pointer("/formula/contextual_multipliers/by_rank"))
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=magic_damage_percent_current_health] ratio in {}",
                path.display()
            )
        })?;
    let minimum_magic_damage = ability_effect_by_id(effects, "minimum_damage")
        .and_then(|effect| effect.get("base_by_rank"))
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=minimum_damage] base_by_rank in {}",
                path.display()
            )
        })?;

    Ok(DoctorMundoInfectedBonesawAbilityDefaults {
        cast_range,
        cooldown_seconds,
        infected_bonesaw_execution: champion_ability_execution_profile_from_ability(
            ability,
            champion_is_melee,
        ),
        current_health_ratio,
        minimum_magic_damage,
    })
}

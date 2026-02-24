use super::super::super::*;
use super::super::simulation_defaults_extraction_helpers::*;

pub(in super::super::super) fn load_vayne_tumble_ability_defaults()
-> Result<VayneTumbleAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vayne.json")?;
    let tumble = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let effects = champion_ability_effects(tumble, "basic_ability_1", &champion_path)?;
    let bonus_damage_effect =
        ability_effect_by_id(effects, "bonus_physical_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=bonus_physical_damage] in {}",
                champion_path.display()
            )
        })?;

    let tumble_bonus_physical_attack_damage_ratio =
        effect_contextual_multiplier_by_rank(bonus_damage_effect).ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=bonus_physical_damage].formula.contextual_multipliers.by_rank in {}",
                champion_path.display()
            )
        })?;
    let tumble_bonus_physical_ability_power_ratio =
        effect_formula_coefficient(bonus_damage_effect, "ability_power").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=bonus_physical_damage] ability_power coefficient in {}",
                champion_path.display()
            )
        })?;

    Ok(VayneTumbleAbilityDefaults {
        tumble_cooldown_seconds: champion_ability_cooldown_seconds(
            tumble,
            "basic_ability_1",
            &champion_path,
        )?,
        tumble_bonus_physical_attack_damage_ratio,
        tumble_bonus_physical_ability_power_ratio,
    })
}

pub(in super::super::super) fn load_vayne_silver_bolts_ability_defaults()
-> Result<VayneSilverBoltsAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vayne.json")?;
    let silver_bolts = champion_ability(&champion_data, "basic_ability_2", &champion_path)?;
    let effects = champion_ability_effects(silver_bolts, "basic_ability_2", &champion_path)?;
    let periodic_true_hit_every = ability_effect_by_id(effects, "max_stacks")
        .and_then(|effect| effect.get("value"))
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=max_stacks].value in {}",
                champion_path.display()
            )
        })?;
    let periodic_true_hit_base =
        effect_base_by_rank(effects, "minimum_bonus_true_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=minimum_bonus_true_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let periodic_true_hit_target_max_health_ratio = ability_effect_by_id(
        effects,
        "bonus_true_damage_percent_target_max_health",
    )
    .and_then(effect_contextual_multiplier_by_rank)
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_2.effects[id=bonus_true_damage_percent_target_max_health].formula.contextual_multipliers.by_rank in {}",
            champion_path.display()
        )
    })?;
    Ok(VayneSilverBoltsAbilityDefaults {
        periodic_true_hit_every,
        periodic_true_hit_base,
        periodic_true_hit_target_max_health_ratio,
    })
}

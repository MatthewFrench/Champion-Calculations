use super::super::super::*;
use super::super::simulation_defaults_extraction_helpers::*;

pub(in super::super::super) fn load_warwick_infinite_duress_ability_defaults()
-> Result<WarwickInfiniteDuressAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Warwick.json")?;
    let ultimate = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let effects = champion_ability_effects(ultimate, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(
            &champion_data,
        );

    let infinite_duress_magic_base_damage = effect_base_by_rank(effects, "total_magic_damage")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=total_magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let infinite_duress_magic_attack_damage_ratio =
        effect_formula_coefficient_by_id(effects, "total_magic_damage", "bonus_attack_damage")
            .ok_or_else(|| {
                anyhow!(
                    "Missing abilities.ultimate.effects[id=total_magic_damage] bonus_attack_damage coefficient in {}",
                    champion_path.display()
                )
            })?;
    let infinite_duress_stun_duration_seconds =
        effect_duration_seconds_by_id(effects, "suppression_duration").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=suppression_duration] duration in {}",
                champion_path.display()
            )
        })?;
    let on_hit_applications = ability_effect_by_id(effects, "on_hit_applications")
        .and_then(|effect| effect.get("value"))
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=on_hit_applications].value in {}",
                champion_path.display()
            )
        })?;
    let physical_attack_damage_ratio_per_hit = ability_effect_by_id(
        effects,
        "physical_attack_damage_ratio_per_hit",
    )
    .and_then(|effect| effect.get("value_ratio"))
    .and_then(Value::as_f64)
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=physical_attack_damage_ratio_per_hit].value_ratio in {}",
            champion_path.display()
        )
    })?;

    Ok(WarwickInfiniteDuressAbilityDefaults {
        infinite_duress_cast_range: champion_ability_range(ultimate, "ultimate", &champion_path)?,
        infinite_duress_cooldown_seconds: champion_ability_cooldown_seconds(
            ultimate,
            "ultimate",
            &champion_path,
        )?,
        infinite_duress_execution: champion_ability_execution_profile_from_ability(
            ultimate,
            champion_is_melee,
        ),
        infinite_duress_physical_attack_damage_ratio: on_hit_applications
            * physical_attack_damage_ratio_per_hit,
        infinite_duress_magic_base_damage,
        infinite_duress_magic_attack_damage_ratio,
        infinite_duress_stun_duration_seconds,
    })
}

pub(in super::super::super) fn load_warwick_eternal_hunger_passive_defaults()
-> Result<WarwickEternalHungerPassiveDefaults> {
    let (champion_path, champion_data) = read_champion_file("Warwick.json")?;
    let passive = champion_ability(&champion_data, "passive", &champion_path)?;
    let effects = champion_ability_effects(passive, "passive", &champion_path)?;
    let on_hit_effect =
        ability_effect_by_id(effects, "bonus_magic_damage_on_hit").ok_or_else(|| {
            anyhow!(
                "Missing abilities.passive.effects[id=bonus_magic_damage_on_hit] in {}",
                champion_path.display()
            )
        })?;
    let on_hit_magic_flat = on_hit_effect
        .get("base_by_champion_level")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.passive.effects[id=bonus_magic_damage_on_hit].base_by_champion_level in {}",
                champion_path.display()
            )
        })?;
    let on_hit_magic_ad_ratio = effect_formula_coefficient(on_hit_effect, "bonus_attack_damage")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.passive.effects[id=bonus_magic_damage_on_hit] bonus_attack_damage coefficient in {}",
                champion_path.display()
            )
        })?;
    Ok(WarwickEternalHungerPassiveDefaults {
        on_hit_magic_flat,
        on_hit_magic_ad_ratio,
    })
}

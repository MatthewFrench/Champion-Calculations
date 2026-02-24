use super::super::super::*;
use super::super::simulation_defaults_extraction_helpers::*;

pub(in super::super::super) fn load_morgana_binding_and_soul_shackles_ability_defaults()
-> Result<MorganaBindingAndSoulShacklesAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Morgana.json")?;
    let dark_binding = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let soul_shackles = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(
            &champion_data,
        );
    let dark_binding_effects =
        champion_ability_effects(dark_binding, "basic_ability_1", &champion_path)?;
    let soul_shackles_effects =
        champion_ability_effects(soul_shackles, "ultimate", &champion_path)?;

    let dark_binding_magic_base_damage = effect_base_by_rank(dark_binding_effects, "magic_damage")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let dark_binding_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        dark_binding_effects,
        "magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_1.effects[id=magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;
    let dark_binding_stun_duration_seconds =
        effect_duration_seconds_by_id(dark_binding_effects, "root_duration").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=root_duration] duration in {}",
                champion_path.display()
            )
        })?;

    let soul_shackles_initial_magic_damage =
        effect_base_by_rank(soul_shackles_effects, "initial_magic_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=initial_magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let soul_shackles_initial_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        soul_shackles_effects,
        "initial_magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=initial_magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;

    let soul_shackles_total_magic_damage =
        effect_base_by_rank(soul_shackles_effects, "total_magic_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=total_magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let soul_shackles_total_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        soul_shackles_effects,
        "total_magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=total_magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;
    let soul_shackles_detonate_stun_duration_seconds =
        effect_duration_seconds_by_id(soul_shackles_effects, "stun_duration").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=stun_duration] duration in {}",
                champion_path.display()
            )
        })?;
    let soul_shackles_detonate_delay_seconds =
        effect_duration_seconds_by_id(soul_shackles_effects, "tether_duration").ok_or_else(
            || {
                anyhow!(
                    "Missing abilities.ultimate.effects[id=tether_duration] duration in {}",
                    champion_path.display()
                )
            },
        )?;

    Ok(MorganaBindingAndSoulShacklesAbilityDefaults {
        dark_binding_cast_range: champion_ability_range(
            dark_binding,
            "basic_ability_1",
            &champion_path,
        )?,
        dark_binding_cooldown_seconds: champion_ability_cooldown_seconds(
            dark_binding,
            "basic_ability_1",
            &champion_path,
        )?,
        dark_binding_execution: champion_ability_execution_profile_from_ability(
            dark_binding,
            champion_is_melee,
        ),
        dark_binding_magic_base_damage,
        dark_binding_magic_ability_power_ratio,
        dark_binding_stun_duration_seconds,
        soul_shackles_cast_range: champion_ability_range(
            soul_shackles,
            "ultimate",
            &champion_path,
        )?,
        soul_shackles_cooldown_seconds: champion_ability_cooldown_seconds(
            soul_shackles,
            "ultimate",
            &champion_path,
        )?,
        soul_shackles_execution: champion_ability_execution_profile_from_ability(
            soul_shackles,
            champion_is_melee,
        ),
        soul_shackles_detonate_delay_seconds,
        soul_shackles_initial_magic_damage,
        soul_shackles_initial_magic_ability_power_ratio,
        soul_shackles_detonate_magic_damage: (soul_shackles_total_magic_damage
            - soul_shackles_initial_magic_damage)
            .max(0.0),
        soul_shackles_detonate_magic_ability_power_ratio:
            (soul_shackles_total_magic_ability_power_ratio
                - soul_shackles_initial_magic_ability_power_ratio)
                .max(0.0),
        soul_shackles_detonate_stun_duration_seconds,
    })
}

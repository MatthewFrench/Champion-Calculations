use super::super::super::*;
use super::super::simulation_defaults_extraction_helpers::*;

pub(in super::super::super) fn load_sona_crescendo_ability_defaults()
-> Result<SonaCrescendoAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Sona.json")?;
    let crescendo = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let effects = champion_ability_effects(crescendo, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(
            &champion_data,
        );

    let crescendo_magic_base_damage =
        effect_base_by_rank(effects, "magic_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let crescendo_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        effects,
        "magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;
    let crescendo_stun_duration_seconds = effect_duration_seconds_by_id(effects, "stun")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=stun] duration in {}",
                champion_path.display()
            )
        })?;

    Ok(SonaCrescendoAbilityDefaults {
        crescendo_cast_range: champion_ability_range(crescendo, "ultimate", &champion_path)?,
        crescendo_cooldown_seconds: champion_ability_cooldown_seconds(
            crescendo,
            "ultimate",
            &champion_path,
        )?,
        crescendo_execution: champion_ability_execution_profile_from_ability(
            crescendo,
            champion_is_melee,
        ),
        crescendo_magic_base_damage,
        crescendo_magic_ability_power_ratio,
        crescendo_stun_duration_seconds,
    })
}

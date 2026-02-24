use super::super::super::*;
use super::super::simulation_defaults_extraction_helpers::*;

pub(in super::super::super) fn load_vladimir_cast_profile_defaults()
-> Result<VladimirCastProfileDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let q_ability = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let e_ability = champion_ability(&champion_data, "basic_ability_3", &champion_path)?;
    let r_ability = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(
            &champion_data,
        );
    let q_execution = champion_ability_execution_profile_from_ability(q_ability, champion_is_melee);
    let e_execution = champion_ability_execution_profile_from_ability(e_ability, champion_is_melee);
    let r_execution = champion_ability_execution_profile_from_ability(r_ability, champion_is_melee);
    let slot_bindings = champion_slot_bindings("vladimir");

    let q_ability_id = slot_bindings.get("Q").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for Q from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;
    let pool_ability_id = slot_bindings.get("W").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for W from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;
    let e_ability_id = slot_bindings.get("E").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for E from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;
    let r_ability_id = slot_bindings.get("R").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for R from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;

    Ok(VladimirCastProfileDefaults {
        q_ability_id,
        e_ability_id,
        r_ability_id,
        pool_ability_id,
        q_range: champion_ability_range(q_ability, "basic_ability_1", &champion_path)?,
        q_windup_seconds: q_execution.cast_windup_seconds,
        q_projectile_speed: q_execution.projectile_speed,
        q_effect_hitbox_radius: q_execution.effect_hitbox_radius,
        e_range: champion_ability_range(e_ability, "basic_ability_3", &champion_path)?,
        e_windup_seconds: e_execution.cast_windup_seconds,
        e_projectile_speed: e_execution.projectile_speed,
        e_effect_hitbox_radius: e_execution.effect_hitbox_radius,
        r_range: champion_ability_range(r_ability, "ultimate", &champion_path)?,
        r_windup_seconds: r_execution.cast_windup_seconds,
        r_projectile_speed: r_execution.projectile_speed,
        r_effect_hitbox_radius: r_execution.effect_hitbox_radius,
    })
}

pub(in super::super::super) fn load_vladimir_offensive_ability_defaults()
-> Result<VladimirOffensiveAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let q_ability = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let e_ability = champion_ability(&champion_data, "basic_ability_3", &champion_path)?;
    let r_ability = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let q_effects = champion_ability_effects(q_ability, "basic_ability_1", &champion_path)?;
    let e_effects = champion_ability_effects(e_ability, "basic_ability_3", &champion_path)?;
    let r_effects = champion_ability_effects(r_ability, "ultimate", &champion_path)?;

    let q_base_damage = effect_base_by_rank(q_effects, "magic_damage").ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_1.effects[id=magic_damage].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let q_ap_ratio = effect_formula_coefficient_by_id(q_effects, "magic_damage", "ability_power")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=magic_damage] ability_power coefficient in {}",
                champion_path.display()
            )
        })?;
    let q_heal_base = effect_base_by_rank(q_effects, "heal").ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_1.effects[id=heal].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let q_heal_ratio_of_damage = if q_base_damage > 0.0 {
        q_heal_base / q_base_damage
    } else {
        0.0
    };

    let e_base_damage = effect_base_by_rank(e_effects, "maximum_damage").ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_3.effects[id=maximum_damage].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let e_ap_ratio = effect_formula_coefficient_by_id(e_effects, "maximum_damage", "ability_power")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_3.effects[id=maximum_damage] ability_power coefficient in {}",
                champion_path.display()
            )
        })?;

    let r_base_damage = effect_base_by_rank(r_effects, "detonation_damage").ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=detonation_damage].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let r_ap_ratio = effect_formula_coefficient_by_id(r_effects, "detonation_damage", "ability_power")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=detonation_damage] ability_power coefficient in {}",
                champion_path.display()
            )
        })?;

    Ok(VladimirOffensiveAbilityDefaults {
        q_base_damage,
        q_ap_ratio,
        q_heal_ratio_of_damage,
        q_base_cooldown_seconds: champion_ability_cooldown_seconds(
            q_ability,
            "basic_ability_1",
            &champion_path,
        )?,
        e_base_damage,
        e_ap_ratio,
        e_base_cooldown_seconds: champion_ability_cooldown_seconds(
            e_ability,
            "basic_ability_3",
            &champion_path,
        )?,
        r_base_damage,
        r_ap_ratio,
        r_base_cooldown_seconds: champion_ability_cooldown_seconds(
            r_ability,
            "ultimate",
            &champion_path,
        )?,
    })
}

pub(in super::super::super) fn load_vladimir_sanguine_pool_defaults()
-> Result<VladimirSanguinePoolDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let pool_ability = champion_ability(&champion_data, "basic_ability_2", &champion_path)?;
    let pool_effects = champion_ability_effects(pool_ability, "basic_ability_2", &champion_path)?;

    let base_cooldown_seconds_by_rank =
        champion_ability_cooldown_seconds_by_rank(pool_ability, "basic_ability_2", &champion_path)?;
    let default_rank = base_cooldown_seconds_by_rank.len().max(1);
    let effect_range = champion_ability_range(pool_ability, "basic_ability_2", &champion_path)?;
    let untargetable_seconds = effect_duration_seconds_by_id(pool_effects, "untargetable")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=untargetable] duration in {}",
                champion_path.display()
            )
        })?;
    let damage_tick_interval_seconds = ability_effect_by_id(pool_effects, "damage_per_tick")
        .and_then(|effect| effect.get("tick_interval_seconds"))
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=damage_per_tick].tick_interval_seconds in {}",
                champion_path.display()
            )
        })?;
    let cost_percent_current_health = pool_ability
        .pointer("/cost/ratio")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.cost.ratio in {}",
                champion_path.display()
            )
        })?;
    let heal_ratio_of_damage = ability_effect_by_id(pool_effects, "heal_from_damage")
        .and_then(|effect| {
            effect
                .pointer("/formula/contextual_multipliers/by_target_type/champions")
                .and_then(Value::as_f64)
        })
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=heal_from_damage] champion multiplier in {}",
                champion_path.display()
            )
        })?;
    let damage_per_tick_by_rank = effect_base_by_rank_values(pool_effects, "damage_per_tick")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=damage_per_tick].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let damage_per_tick_bonus_health_ratio =
        effect_formula_coefficient_by_id(pool_effects, "damage_per_tick", "bonus_health")
            .ok_or_else(|| {
                anyhow!(
                    "Missing abilities.basic_ability_2.effects[id=damage_per_tick] bonus_health coefficient in {}",
                    champion_path.display()
                )
            })?;

    Ok(VladimirSanguinePoolDefaults {
        base_cooldown_seconds_by_rank,
        default_rank,
        effect_range,
        untargetable_seconds,
        damage_tick_interval_seconds,
        cost_percent_current_health,
        heal_ratio_of_damage,
        damage_per_tick_by_rank,
        damage_per_tick_bonus_health_ratio,
    })
}

pub(in super::super::super) fn load_vladimir_defensive_ability_two_policy_defaults()
-> Result<VladimirDefensiveAbilityTwoPolicyDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let prioritize_offensive_ultimate_before_defensive_ability_two = champion_data
        .pointer("/simulation/controlled_champion/defensive_ability_two/prioritize_offensive_ultimate_before_defensive_ability_two_when_ready_and_targets_in_range")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            anyhow!(
                "Missing simulation.controlled_champion.defensive_ability_two.prioritize_offensive_ultimate_before_defensive_ability_two_when_ready_and_targets_in_range in {}",
                champion_path.display()
            )
        })?;
    Ok(VladimirDefensiveAbilityTwoPolicyDefaults {
        prioritize_offensive_ultimate_before_defensive_ability_two,
    })
}

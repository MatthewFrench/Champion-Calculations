use super::super::*;
use super::simulation_defaults_extraction_helpers::*;

pub(in super::super) fn load_vladimir_cast_profile_defaults() -> Result<VladimirCastProfileDefaults>
{
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let q_ability = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let e_ability = champion_ability(&champion_data, "basic_ability_3", &champion_path)?;
    let r_ability = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(&champion_data);
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

pub(in super::super) fn load_vladimir_offensive_ability_defaults()
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
    let e_ap_ratio =
        effect_formula_coefficient_by_id(e_effects, "maximum_damage", "ability_power")
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
    let r_ap_ratio =
        effect_formula_coefficient_by_id(r_effects, "detonation_damage", "ability_power")
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

pub(in super::super) fn load_vladimir_sanguine_pool_defaults()
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

pub(in super::super) fn load_vladimir_defensive_ability_two_policy_defaults()
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

pub(in super::super) fn load_warwick_infinite_duress_ability_defaults()
-> Result<WarwickInfiniteDuressAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Warwick.json")?;
    let ultimate = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let effects = champion_ability_effects(ultimate, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(&champion_data);

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

pub(in super::super) fn load_warwick_eternal_hunger_passive_defaults()
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

pub(in super::super) fn load_vayne_tumble_ability_defaults() -> Result<VayneTumbleAbilityDefaults> {
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

pub(in super::super) fn load_vayne_silver_bolts_ability_defaults()
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

pub(in super::super) fn load_morgana_binding_and_soul_shackles_ability_defaults()
-> Result<MorganaBindingAndSoulShacklesAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Morgana.json")?;
    let dark_binding = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let soul_shackles = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(&champion_data);
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

pub(in super::super) fn load_sona_crescendo_ability_defaults()
-> Result<SonaCrescendoAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Sona.json")?;
    let crescendo = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let effects = champion_ability_effects(crescendo, "ultimate", &champion_path)?;
    let champion_is_melee =
        super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(&champion_data);

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

pub(in super::super) fn load_doctor_mundo_infected_bonesaw_ability_defaults()
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
        super::simulation_defaults_extraction_helpers::champion_is_melee_from_data(&data);
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

use super::*;

pub(super) fn derive_enemy_model(
    enemy: &EnemyConfig,
    build: &[Item],
    enemy_bonus: &Stats,
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> EnemyDerivedModel {
    let mut enemy_stats = Stats::default();
    for item in build {
        enemy_stats.add(&item.stats);
    }
    enemy_stats.add(enemy_bonus);
    apply_item_assumptions(
        &mut enemy_stats,
        &enemy.base,
        build,
        sim,
        enemy.level,
        None,
        Some(&enemy.stack_overrides),
    );

    let attack_damage = enemy.base.base_attack_damage + enemy_stats.attack_damage;
    let ability_power = enemy_stats.ability_power.max(0.0);
    let ability_haste = enemy_stats.ability_haste + urf.ability_haste;
    let runtime_buffs = RuntimeBuffState {
        ability_haste,
        item_haste: urf.item_haste,
        cooldown_rate_multiplier: 1.0,
        ..RuntimeBuffState::default()
    };
    let armor = (enemy.base.base_armor + enemy_stats.armor).max(0.0);
    let magic_resist = (enemy.base.base_magic_resist + enemy_stats.magic_resist).max(0.0);
    let physical_multiplier = 100.0 / (100.0 + armor);
    let max_health = (enemy.base.base_health + enemy_stats.health).max(1.0);
    let move_speed = resolve_stat(
        StatQuery::MovementSpeedUnits {
            base_units: enemy.base.base_move_speed,
            flat_bonus_units: enemy_stats.move_speed_flat,
            percent_bonus: enemy_stats.move_speed_percent,
            minimum_units: 150.0,
        },
        runtime_buffs,
    );

    let attack_speed_bonus = enemy_stats.attack_speed_percent / 100.0;
    let mut attack_speed = enemy.base.base_attack_speed * (1.0 + attack_speed_bonus);
    attack_speed *= if enemy.base.is_melee {
        urf.bonus_attack_speed_multiplier_melee
    } else {
        urf.bonus_attack_speed_multiplier_ranged
    };
    let base_attack_speed = attack_speed.max(0.001);

    let runtime_item_names = if enemy.loadout_item_names.is_empty() {
        build
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
    } else {
        enemy.loadout_item_names.clone()
    };
    let runtime_rune_names = enemy.loadout_rune_names.clone();
    let runtime = build_champion_loadout_runtime(
        &runtime_item_names,
        &runtime_rune_names,
        urf.item_haste,
        enemy.base.is_melee,
        sim.collect_rune_proc_telemetry,
    );
    attack_speed = base_attack_speed * attack_speed_multiplier(&runtime, 0.0);

    let attack_interval = 1.0 / attack_speed.max(0.001);
    let behavior = behavior_profile(
        &enemy.name,
        enemy.base.is_melee,
        enemy.base.base_attack_range,
        enemy.base.base_attack_projectile_speed,
    );

    EnemyDerivedModel {
        behavior,
        runtime,
        runtime_item_names,
        runtime_rune_names,
        max_health,
        armor,
        magic_resist,
        physical_multiplier,
        magic_multiplier: 100.0 / (100.0 + magic_resist),
        attack_damage,
        ability_power,
        ability_haste,
        attack_speed,
        attack_interval,
        move_speed,
    }
}

pub(crate) fn derive_enemy_combat_stats(
    enemy: &EnemyConfig,
    build: &[Item],
    enemy_bonus: &Stats,
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> EnemyDerivedCombatStats {
    let model = derive_enemy_model(enemy, build, enemy_bonus, sim, urf);
    EnemyDerivedCombatStats {
        champion: enemy.name.clone(),
        max_health: model.max_health,
        armor: model.armor,
        magic_resist: model.magic_resist,
        attack_damage: model.attack_damage,
        attack_speed: model.attack_speed,
        attack_interval_seconds: model.attack_interval,
        attack_range: model.behavior.attack_range,
        attack_projectile_speed: model.behavior.attack_projectile_speed,
        move_speed: model.move_speed,
        desired_combat_range: model.behavior.desired_combat_range,
        physical_hit_damage: model.attack_damage,
        ability_hit_damage: 0.0,
        burst_physical_damage: 0.0,
        burst_magic_damage: 0.0,
        burst_true_damage: 0.0,
    }
}

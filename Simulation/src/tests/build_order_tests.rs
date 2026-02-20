use super::*;
use crate::OpponentMovementMode;
use std::collections::HashMap;

fn test_champion_base(name: &str) -> ChampionBase {
    ChampionBase {
        name: name.to_string(),
        base_health: 1000.0,
        health_per_level: 100.0,
        base_armor: 30.0,
        armor_per_level: 5.0,
        base_magic_resist: 30.0,
        magic_resist_per_level: 2.0,
        base_attack_damage: 60.0,
        attack_damage_per_level: 3.0,
        base_attack_speed: 0.7,
        attack_speed_per_level_percent: 2.0,
        base_attack_range: 125.0,
        base_attack_projectile_speed: 0.0,
        base_move_speed: 340.0,
        is_melee: true,
    }
}

#[test]
fn level_scaled_enemy_builds_use_enemy_configured_level() {
    let raw_base = test_champion_base("Enemy");
    let configured_level = 18usize;
    let enemy_config = EnemyConfig {
        id: "enemy_a".to_string(),
        name: "Enemy".to_string(),
        level: configured_level,
        base: champion_at_level(&raw_base, 1),
        spawn_position_xy: None,
        movement_mode: OpponentMovementMode::HoldPosition,
        loadout_item_names: Vec::new(),
        loadout_rune_names: Vec::new(),
        loadout_shards: Vec::new(),
        stack_overrides: HashMap::new(),
    };
    let enemy_builds = vec![(enemy_config.clone(), Vec::new(), Stats::default())];
    let raw_enemy_bases = HashMap::from([(enemy_config.id.clone(), raw_base.clone())]);

    let scaled = level_scaled_enemy_builds(&enemy_builds, &raw_enemy_bases);
    let scaled_base = &scaled[0].0.base;
    let expected = champion_at_level(&raw_base, configured_level);
    let stage_scaled = champion_at_level(&raw_base, 5);

    assert!(
        (scaled_base.base_health - expected.base_health).abs() < 1e-9,
        "enemy stage scaling should use configured level"
    );
    assert!(
        (scaled_base.base_health - stage_scaled.base_health).abs() > 1e-9,
        "enemy stage scaling should not use controlled champion stage level"
    );
}

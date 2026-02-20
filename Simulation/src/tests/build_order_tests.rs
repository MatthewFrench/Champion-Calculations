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

#[test]
fn level_scaled_enemy_builds_do_not_double_scale_when_raw_base_is_missing() {
    let raw_base = test_champion_base("Enemy");
    let configured_level = 18usize;
    let leveled_base = champion_at_level(&raw_base, configured_level);
    let enemy_config = EnemyConfig {
        id: "enemy_a".to_string(),
        name: "Enemy".to_string(),
        level: configured_level,
        base: leveled_base.clone(),
        spawn_position_xy: None,
        movement_mode: OpponentMovementMode::HoldPosition,
        loadout_item_names: Vec::new(),
        loadout_rune_names: Vec::new(),
        loadout_shards: Vec::new(),
        stack_overrides: HashMap::new(),
    };
    let enemy_builds = vec![(enemy_config, Vec::new(), Stats::default())];
    let raw_enemy_bases = HashMap::new();

    let scaled = level_scaled_enemy_builds(&enemy_builds, &raw_enemy_bases);
    let scaled_base = &scaled[0].0.base;
    let double_scaled = champion_at_level(&leveled_base, configured_level);

    assert!(
        (scaled_base.base_health - leveled_base.base_health).abs() < 1e-9,
        "missing raw-base lookup should preserve already-leveled actor base"
    );
    assert!(
        (scaled_base.base_health - double_scaled.base_health).abs() > 1e-9,
        "fallback path must not apply champion_at_level twice"
    );
}

#[test]
fn blended_stage_score_ignores_worst_case_when_only_one_positive_weight() {
    let weighted_mean_score = 10.0;
    let worst_case_score = Some(1.0);
    let worst_case_weight = 0.8;
    let positive_weight_scenarios = 1usize;

    let blended = blended_stage_score(
        weighted_mean_score,
        worst_case_score,
        positive_weight_scenarios,
        worst_case_weight,
    );

    assert!(
        (blended - weighted_mean_score).abs() < 1e-9,
        "zero-weight encounters should not influence worst-case blend"
    );
}

#[test]
fn blended_stage_score_applies_worst_case_with_multiple_positive_weights() {
    let blended = blended_stage_score(10.0, Some(1.0), 2, 0.8);
    let expected = 10.0 * 0.2 + 1.0 * 0.8;
    assert!((blended - expected).abs() < 1e-9);
}

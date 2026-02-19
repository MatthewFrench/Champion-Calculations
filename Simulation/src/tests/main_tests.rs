use super::*;
use clap::Parser;

#[test]
fn loadout_selection_key_is_order_sensitive() {
    let a = LoadoutSelection {
        rune_names: vec!["Triumph".to_string(), "Lethal Tempo".to_string()],
        shard_stats: vec!["adaptive".to_string(), "health".to_string()],
    };
    let b = LoadoutSelection {
        rune_names: vec!["Lethal Tempo".to_string(), "Triumph".to_string()],
        shard_stats: vec!["health".to_string(), "adaptive".to_string()],
    };
    assert_ne!(
        loadout_selection_key(&a),
        loadout_selection_key(&b),
        "rune and shard order must remain slot-aware"
    );
}

#[test]
fn compute_vladimir_stats_does_not_recursively_reapply_conversions() {
    let base = ChampionBase {
        name: "Vladimir".to_string(),
        base_health: 1000.0,
        health_per_level: 0.0,
        base_armor: 30.0,
        armor_per_level: 0.0,
        base_magic_resist: 30.0,
        magic_resist_per_level: 0.0,
        base_attack_damage: 60.0,
        attack_damage_per_level: 0.0,
        base_attack_speed: 0.658,
        attack_speed_per_level_percent: 0.0,
        base_attack_range: 450.0,
        base_attack_projectile_speed: 1600.0,
        base_move_speed: 340.0,
        is_melee: false,
    };
    let item_stats = Stats {
        ability_power: 100.0,
        health: 200.0,
        ..Stats::default()
    };
    let out = compute_champion_final_stats(&base, &item_stats);
    let expected_ap = 100.0 + 0.033 * 200.0;
    let expected_health = 1000.0 + 200.0 + 1.6 * 100.0;
    assert!((out.ability_power - expected_ap).abs() < 1e-9);
    assert!((out.health - expected_health).abs() < 1e-9);
}

#[test]
fn enemy_preset_data_validates_against_local_data() {
    let presets = load_enemy_urf_presets().expect("enemy presets should load");
    let items = load_items().expect("items should load");
    let domain = build_loadout_domain();
    let urf = load_urf_buffs().expect("urf config should load");
    validate_enemy_urf_presets(&presets, &items, &domain, &urf)
        .expect("enemy preset validation should pass");
}

#[test]
fn random_loadout_generation_produces_legal_shapes() {
    let domain = build_loadout_domain();
    assert!(domain.rune_paths.len() >= 2);
    assert!(domain.shard_slots.iter().all(|s| !s.is_empty()));

    let base = LoadoutSelection::default();
    let mut seed = 1337u64;
    for _ in 0..64 {
        let sample = random_loadout_selection(&base, &domain, &mut seed);
        assert_eq!(sample.rune_names.len(), 6);
        assert_eq!(sample.shard_stats.len(), 3);
    }
}

#[test]
fn cli_mode_parsing_accepts_generic_names_and_legacy_aliases() {
    let controlled = Cli::try_parse_from([
        "urf_sim",
        "--scenario",
        "vladimir_urf_teamfight",
        "--mode",
        "controlled_champion",
    ])
    .expect("controlled_champion mode should parse");
    assert!(matches!(controlled.mode, Mode::ControlledChampion));

    let controlled_alias = Cli::try_parse_from([
        "urf_sim",
        "--scenario",
        "vladimir_urf_teamfight",
        "--mode",
        "vladimir",
    ])
    .expect("legacy vladimir alias should parse");
    assert!(matches!(controlled_alias.mode, Mode::ControlledChampion));

    let step = Cli::try_parse_from([
        "urf_sim",
        "--scenario",
        "vladimir_urf_teamfight",
        "--mode",
        "controlled_champion_step",
    ])
    .expect("controlled_champion_step mode should parse");
    assert!(matches!(step.mode, Mode::ControlledChampionStep));

    let step_alias = Cli::try_parse_from([
        "urf_sim",
        "--scenario",
        "vladimir_urf_teamfight",
        "--mode",
        "vladimir_step",
    ])
    .expect("legacy vladimir_step alias should parse");
    assert!(matches!(step_alias.mode, Mode::ControlledChampionStep));

    let fixed_sweep = Cli::try_parse_from([
        "urf_sim",
        "--scenario",
        "vladimir_urf_teamfight",
        "--mode",
        "controlled_champion_fixed_loadout_rune_sweep",
        "--fixed-item-names",
        "Rabadon's Deathcap",
    ])
    .expect("controlled_champion_fixed_loadout_rune_sweep mode should parse");
    assert!(matches!(
        fixed_sweep.mode,
        Mode::ControlledChampionFixedLoadoutRuneSweep
    ));
}

#[test]
fn cli_fixed_sweep_seed_repeats_defaults_and_overrides() {
    let defaults = Cli::try_parse_from([
        "urf_sim",
        "--scenario",
        "vladimir_urf_teamfight",
        "--mode",
        "controlled_champion_fixed_loadout_rune_sweep",
        "--fixed-item-names",
        "Rabadon's Deathcap",
    ])
    .expect("fixed sweep mode should parse with default repeat count");
    assert_eq!(defaults.fixed_sweep_seed_repeats, 1);

    let override_value = Cli::try_parse_from([
        "urf_sim",
        "--scenario",
        "vladimir_urf_teamfight",
        "--mode",
        "controlled_champion_fixed_loadout_rune_sweep",
        "--fixed-item-names",
        "Rabadon's Deathcap",
        "--fixed-sweep-seed-repeats",
        "4",
    ])
    .expect("fixed sweep mode should parse custom repeat count");
    assert_eq!(override_value.fixed_sweep_seed_repeats, 4);
}

#[test]
fn objective_weights_and_scoring_are_normalized() {
    let w = normalized_objective_weights(0.50, 0.25, 0.15, 0.10, 0.05);
    assert!(
        (w.survival + w.damage + w.healing + w.enemy_kills + w.invulnerable_seconds - 1.0).abs()
            < 1e-9
    );

    let reference = CombatOutcome {
        time_alive_seconds: 20.0,
        damage_dealt: 8000.0,
        healing_done: 2000.0,
        enemy_kills: 2,
        invulnerable_seconds: 1.0,
    };
    let reference_score = objective_score_from_outcome(reference, reference, w);
    assert!((reference_score - 1.0).abs() < 1e-9);

    let better = CombatOutcome {
        time_alive_seconds: 22.0,
        damage_dealt: 8800.0,
        healing_done: 2400.0,
        enemy_kills: 3,
        invulnerable_seconds: 2.0,
    };
    assert!(objective_score_from_outcome(better, reference, w) > reference_score);

    let kills_only_upgrade = CombatOutcome {
        enemy_kills: reference.enemy_kills + 1,
        ..reference
    };
    assert!(objective_score_from_outcome(kills_only_upgrade, reference, w) > reference_score);

    let invulnerable_only_upgrade = CombatOutcome {
        invulnerable_seconds: reference.invulnerable_seconds + 1.0,
        ..reference
    };
    assert!(
        objective_score_from_outcome(invulnerable_only_upgrade, reference, w) > reference_score
    );
}

#[test]
fn urf_respawn_timer_scales_with_level() {
    let tuning = respawn::UrfRespawnTuning {
        urf_flat_reduction_seconds: 3.0,
        extrapolation_per_level: 2.5,
        time_scaling_enabled: true,
        time_scaling_start_seconds: 300.0,
        time_scaling_per_minute_seconds: 0.4,
        time_scaling_cap_seconds: 20.0,
    };
    let mut prev = 0.0;
    for lvl in 1..=30 {
        let t = respawn::urf_respawn_delay_seconds(lvl, 600.0, tuning);
        assert!(t >= 1.0);
        assert!(t >= prev);
        prev = t;
    }
    let no_scale_tuning = respawn::UrfRespawnTuning {
        time_scaling_enabled: false,
        ..tuning
    };
    assert!((respawn::urf_respawn_delay_seconds(1, 0.0, no_scale_tuning) - 7.0).abs() < 1e-9);
}

#[test]
fn urf_respawn_timer_increases_with_game_time_after_scaling_start() {
    let tuning = respawn::UrfRespawnTuning {
        urf_flat_reduction_seconds: 3.0,
        extrapolation_per_level: 2.5,
        time_scaling_enabled: true,
        time_scaling_start_seconds: 300.0,
        time_scaling_per_minute_seconds: 0.4,
        time_scaling_cap_seconds: 20.0,
    };
    let level = 16;
    let before = respawn::urf_respawn_delay_seconds(level, 240.0, tuning);
    let after = respawn::urf_respawn_delay_seconds(level, 1200.0, tuning);
    assert!(after > before);
}

#[test]
fn shared_core_modules_do_not_include_vladimir_shortcuts() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for module_path in ["engine.rs", "core.rs", "search.rs", "reporting.rs"] {
        let source = std::fs::read_to_string(src_dir.join(module_path))
            .unwrap_or_else(|err| panic!("failed reading {}: {}", module_path, err));
        let lowered = source.to_ascii_lowercase();
        for forbidden in [
            "vlad_",
            "simulate_vlad",
            "transfusion",
            "tides of blood",
            "hemoplague",
            "sanguine pool",
        ] {
            assert!(
                !lowered.contains(forbidden),
                "{} contains forbidden Vladimir-specific shortcut '{}'",
                module_path,
                forbidden
            );
        }
    }
}

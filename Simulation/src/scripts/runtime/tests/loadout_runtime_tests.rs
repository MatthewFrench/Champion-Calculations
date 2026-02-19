use super::*;

fn default_on_hit_profile() -> OnHitEffectProfile {
    OnHitEffectProfile {
        on_hit_magic_flat: 0.0,
        on_hit_magic_ad_ratio: 0.0,
        periodic_true_hit_every: 0,
        periodic_true_hit_base: 0.0,
        periodic_true_hit_ad_ratio: 0.0,
        periodic_true_hit_target_max_health_ratio: 0.0,
    }
}

#[test]
fn builder_marks_item_and_rune_runtime_flags() {
    let runtime = build_loadout_runtime_state(
        &[
            "Kraken Slayer".to_string(),
            "Guinsoo's Rageblade".to_string(),
            "Heartsteel".to_string(),
        ],
        &[
            "Lethal Tempo".to_string(),
            "Grasp of the Undying".to_string(),
            "Second Wind".to_string(),
            "Press the Attack".to_string(),
            "Fleet Footwork".to_string(),
            "Conqueror".to_string(),
            "Aftershock".to_string(),
        ],
        0.0,
        false,
    );

    assert!(runtime.has_kraken);
    assert!(runtime.has_guinsoo);
    assert!(runtime.has_heartsteel);
    assert!(runtime.has_lethal_tempo);
    assert!(runtime.has_grasp);
    assert!(runtime.has_second_wind);
    assert!(runtime.has_press_the_attack);
    assert!(runtime.has_fleet_footwork);
    assert!(runtime.has_conqueror);
    assert!(runtime.has_aftershock);
}

#[test]
fn lethal_tempo_and_kraken_stack_runtime() {
    let mut runtime = build_loadout_runtime_state(
        &["Kraken Slayer".to_string()],
        &["Lethal Tempo".to_string()],
        0.0,
        false,
    );
    assert!(loadout_attack_speed_multiplier(&runtime) >= 1.0);

    let profile = default_on_hit_profile();
    let _ = calculate_on_hit_bonus_damage(
        profile,
        &mut runtime,
        180.0,
        2500.0,
        3000.0,
        1600.0,
        1.0,
        Some(0),
        18,
    );
    let _ = calculate_on_hit_bonus_damage(
        profile,
        &mut runtime,
        180.0,
        2500.0,
        3000.0,
        1600.0,
        2.0,
        Some(0),
        18,
    );
    let (_, _, true_damage) = calculate_on_hit_bonus_damage(
        profile,
        &mut runtime,
        180.0,
        2500.0,
        3000.0,
        1600.0,
        3.0,
        Some(0),
        18,
    );
    assert!(runtime.lethal_tempo_stacks > 0);
    assert!(true_damage > 0.0);
}

#[test]
fn clear_transient_state_resets_stack_counters() {
    let mut runtime = build_loadout_runtime_state(
        &[
            "Kraken Slayer".to_string(),
            "Guinsoo's Rageblade".to_string(),
        ],
        &["Lethal Tempo".to_string(), "Conqueror".to_string()],
        0.0,
        false,
    );
    runtime.attacks_landed = 7;
    runtime.lethal_tempo_stacks = 6;
    runtime.guinsoo_stacks = 8;
    runtime.conqueror_stacks = 12;
    reset_transient_loadout_state(&mut runtime);
    assert_eq!(runtime.attacks_landed, 0);
    assert_eq!(runtime.lethal_tempo_stacks, 0);
    assert_eq!(runtime.guinsoo_stacks, 0);
    assert_eq!(runtime.conqueror_stacks, 0);
}

#[test]
fn ability_bonus_damage_respects_luden_cooldown() {
    let mut runtime = build_loadout_runtime_state(&["Luden's Echo".to_string()], &[], 0.0, false);
    let (magic_a, true_a) =
        calculate_ability_bonus_damage(&mut runtime, 300.0, 0.0, 2500.0, 0.0, Some(0), 18);
    let (magic_b, true_b) =
        calculate_ability_bonus_damage(&mut runtime, 300.0, 0.0, 2500.0, 1.0, Some(0), 18);
    assert!(magic_a > magic_b);
    assert_eq!(true_a, true_b);
}

#[test]
fn luden_and_heartsteel_cooldowns_scale_with_item_haste() {
    let runtime = build_loadout_runtime_state(
        &["Luden's Echo".to_string(), "Heartsteel".to_string()],
        &[],
        300.0,
        false,
    );
    assert!((runtime.luden_cooldown_seconds - 3.0).abs() < 1e-9);
    assert!((runtime.heartsteel_cooldown_seconds - 7.5).abs() < 1e-9);
}

#[test]
fn second_wind_regen_scales_when_low_health() {
    let runtime = build_loadout_runtime_state(&[], &["Second Wind".to_string()], 0.0, false);
    let high = tick_loadout_regeneration(&runtime, 2800.0, 3000.0, 1.0);
    let low = tick_loadout_regeneration(&runtime, 900.0, 3000.0, 1.0);
    assert!(high > 0.0);
    assert!(low > high);
}

#[test]
fn press_the_attack_third_hit_and_vulnerability_bonus_are_applied() {
    let mut runtime =
        build_loadout_runtime_state(&[], &["Press the Attack".to_string()], 0.0, false);
    let profile = default_on_hit_profile();

    let _ = calculate_on_hit_bonus_damage(
        profile,
        &mut runtime,
        120.0,
        2000.0,
        2500.0,
        2000.0,
        1.0,
        Some(0),
        18,
    );
    let _ = calculate_on_hit_bonus_damage(
        profile,
        &mut runtime,
        120.0,
        2000.0,
        2500.0,
        2000.0,
        2.0,
        Some(0),
        18,
    );
    let (_, third_hit_magic, _) = calculate_on_hit_bonus_damage(
        profile,
        &mut runtime,
        120.0,
        2000.0,
        2500.0,
        2000.0,
        3.0,
        Some(0),
        18,
    );
    let (_, vulnerability_true) =
        calculate_ability_bonus_damage(&mut runtime, 300.0, 0.6, 2500.0, 3.2, Some(0), 18);

    assert!(third_hit_magic > 0.0);
    assert!(vulnerability_true > 0.0);
}

#[test]
fn fleet_footwork_grants_pending_heal_after_attack() {
    let mut runtime = build_loadout_runtime_state(&[], &["Fleet Footwork".to_string()], 0.0, false);
    let profile = default_on_hit_profile();
    let _ = calculate_on_hit_bonus_damage(
        profile,
        &mut runtime,
        150.0,
        2200.0,
        2600.0,
        2200.0,
        1.0,
        Some(0),
        18,
    );
    let heal = on_outgoing_damage_heal(&mut runtime, 200.0, 1.0);
    assert!(heal > 0.0);
}

#[test]
fn conqueror_stacks_amplify_ability_and_enable_heal() {
    let mut runtime = build_loadout_runtime_state(&[], &["Conqueror".to_string()], 0.0, false);
    let mut last_bonus_magic = 0.0;
    for step in 0..6 {
        let now = 1.0 + step as f64;
        let (bonus_magic, _) =
            calculate_ability_bonus_damage(&mut runtime, 260.0, 0.6, 2500.0, now, Some(0), 18);
        last_bonus_magic = bonus_magic;
    }
    assert_eq!(runtime.conqueror_stacks, 12);
    assert!(last_bonus_magic > 0.0);
    let heal = on_outgoing_damage_heal(&mut runtime, 400.0, 10.0);
    assert!(heal > 0.0);
}

#[test]
fn aftershock_trigger_applies_damage_and_respects_cooldown() {
    let mut runtime = build_loadout_runtime_state(&[], &["Aftershock".to_string()], 0.0, true);
    let first = trigger_immobilize_rune_damage(&mut runtime, 1.0, 18, 500.0);
    let blocked = trigger_immobilize_rune_damage(&mut runtime, 2.0, 18, 500.0);
    let second = trigger_immobilize_rune_damage(&mut runtime, 21.1, 18, 500.0);
    assert!(first > 0.0);
    assert_eq!(blocked, 0.0);
    assert!(second > 0.0);
}

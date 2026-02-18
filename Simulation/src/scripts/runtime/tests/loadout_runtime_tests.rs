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
        ],
        0.0,
    );

    assert!(runtime.has_kraken);
    assert!(runtime.has_guinsoo);
    assert!(runtime.has_heartsteel);
    assert!(runtime.has_lethal_tempo);
    assert!(runtime.has_grasp);
    assert!(runtime.has_second_wind);
}

#[test]
fn lethal_tempo_and_kraken_stack_runtime() {
    let mut runtime = build_loadout_runtime_state(
        &["Kraken Slayer".to_string()],
        &["Lethal Tempo".to_string()],
        0.0,
    );
    assert!(loadout_attack_speed_multiplier(&runtime) >= 1.0);

    let profile = default_on_hit_profile();
    let _ =
        calculate_on_hit_bonus_damage(profile, &mut runtime, 180.0, 2500.0, 3000.0, 1600.0, 1.0);
    let _ =
        calculate_on_hit_bonus_damage(profile, &mut runtime, 180.0, 2500.0, 3000.0, 1600.0, 2.0);
    let (_, _, true_damage) =
        calculate_on_hit_bonus_damage(profile, &mut runtime, 180.0, 2500.0, 3000.0, 1600.0, 3.0);
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
        &["Lethal Tempo".to_string()],
        0.0,
    );
    runtime.attacks_landed = 7;
    runtime.lethal_tempo_stacks = 6;
    runtime.guinsoo_stacks = 8;
    reset_transient_loadout_state(&mut runtime);
    assert_eq!(runtime.attacks_landed, 0);
    assert_eq!(runtime.lethal_tempo_stacks, 0);
    assert_eq!(runtime.guinsoo_stacks, 0);
}

#[test]
fn ability_bonus_damage_respects_luden_cooldown() {
    let mut runtime = build_loadout_runtime_state(&["Luden's Echo".to_string()], &[], 0.0);
    let (magic_a, true_a) = calculate_ability_bonus_damage(&mut runtime, 300.0, 2500.0, 0.0);
    let (magic_b, true_b) = calculate_ability_bonus_damage(&mut runtime, 300.0, 2500.0, 1.0);
    assert!(magic_a > magic_b);
    assert_eq!(true_a, true_b);
}

#[test]
fn luden_and_heartsteel_cooldowns_scale_with_item_haste() {
    let runtime = build_loadout_runtime_state(
        &["Luden's Echo".to_string(), "Heartsteel".to_string()],
        &[],
        300.0,
    );
    assert!((runtime.luden_cooldown_seconds - 3.0).abs() < 1e-9);
    assert!((runtime.heartsteel_cooldown_seconds - 7.5).abs() < 1e-9);
}

#[test]
fn second_wind_regen_scales_when_low_health() {
    let runtime = build_loadout_runtime_state(&[], &["Second Wind".to_string()], 0.0);
    let high = tick_loadout_regeneration(&runtime, 2800.0, 3000.0, 1.0);
    let low = tick_loadout_regeneration(&runtime, 900.0, 3000.0, 1.0);
    assert!(high > 0.0);
    assert!(low > high);
}

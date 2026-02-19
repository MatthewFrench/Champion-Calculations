use super::*;
use crate::defaults::rune_runtime_defaults;
use std::time::Instant;

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

fn on_hit(
    runtime: &mut LoadoutRuntimeState,
    attack_damage: f64,
    now: f64,
    target_current_health: f64,
    target_max_health: f64,
) -> (f64, f64, f64) {
    on_hit_at_level(
        runtime,
        attack_damage,
        now,
        target_current_health,
        target_max_health,
        18,
    )
}

fn on_hit_at_level(
    runtime: &mut LoadoutRuntimeState,
    attack_damage: f64,
    now: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_level: usize,
) -> (f64, f64, f64) {
    calculate_on_hit_bonus_damage(
        default_on_hit_profile(),
        runtime,
        attack_damage,
        0.0,
        0.0,
        target_current_health,
        target_max_health,
        1800.0,
        now,
        Some(0),
        attacker_level,
    )
}

#[allow(clippy::too_many_arguments)]
fn ability_hit(
    runtime: &mut LoadoutRuntimeState,
    ability_raw_damage: f64,
    ability_ap_ratio: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    now: f64,
) -> (f64, f64) {
    ability_hit_at_level(
        runtime,
        ability_raw_damage,
        ability_ap_ratio,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        target_current_health,
        target_max_health,
        now,
        18,
    )
}

#[allow(clippy::too_many_arguments)]
fn ability_hit_at_level(
    runtime: &mut LoadoutRuntimeState,
    ability_raw_damage: f64,
    ability_ap_ratio: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    now: f64,
    attacker_level: usize,
) -> (f64, f64) {
    calculate_ability_bonus_damage(
        runtime,
        ability_raw_damage,
        ability_ap_ratio,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        target_current_health,
        target_max_health,
        now,
        Some(0),
        attacker_level,
    )
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
            "Electrocute".to_string(),
            "First Strike".to_string(),
            "Phase Rush".to_string(),
            "Arcane Comet".to_string(),
            "Summon Aery".to_string(),
            "Hail of Blades".to_string(),
            "Dark Harvest".to_string(),
            "Triumph".to_string(),
            "Gathering Storm".to_string(),
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
    assert!(runtime.has_electrocute);
    assert!(runtime.has_first_strike);
    assert!(runtime.has_phase_rush);
    assert!(runtime.has_arcane_comet);
    assert!(runtime.has_summon_aery);
    assert!(runtime.has_hail_of_blades);
    assert!(runtime.has_dark_harvest);
    assert!(runtime.has_triumph);
    assert!(runtime.has_gathering_storm);
}

#[test]
fn lethal_tempo_and_kraken_stack_runtime() {
    let mut runtime = build_loadout_runtime_state(
        &["Kraken Slayer".to_string()],
        &["Lethal Tempo".to_string()],
        0.0,
        false,
    );
    assert!(loadout_attack_speed_multiplier(&runtime, 0.0) >= 1.0);

    let _ = on_hit(&mut runtime, 180.0, 1.0, 2500.0, 3000.0);
    let _ = on_hit(&mut runtime, 180.0, 2.0, 2500.0, 3000.0);
    let (_, _, true_damage) = on_hit(&mut runtime, 180.0, 3.0, 2500.0, 3000.0);
    assert!(runtime.lethal_tempo_stacks > 0);
    assert!(true_damage > 0.0);
    assert!(loadout_attack_speed_multiplier(&runtime, 3.0) > 1.0);
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
    runtime.dark_harvest_souls = 5;
    reset_transient_loadout_state(&mut runtime);
    assert_eq!(runtime.attacks_landed, 0);
    assert_eq!(runtime.lethal_tempo_stacks, 0);
    assert_eq!(runtime.guinsoo_stacks, 0);
    assert_eq!(runtime.conqueror_stacks, 0);
    assert_eq!(runtime.dark_harvest_souls, 5);
}

#[test]
fn ability_bonus_damage_respects_luden_cooldown() {
    let mut runtime = build_loadout_runtime_state(&["Luden's Echo".to_string()], &[], 0.0, false);
    let (magic_a, true_a) = ability_hit(&mut runtime, 300.0, 0.0, 0.0, 0.0, 2400.0, 2500.0, 0.0);
    let (magic_b, true_b) = ability_hit(&mut runtime, 300.0, 0.0, 0.0, 0.0, 2400.0, 2500.0, 1.0);
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

    let _ = on_hit(&mut runtime, 120.0, 1.0, 2000.0, 2500.0);
    let _ = on_hit(&mut runtime, 120.0, 2.0, 2000.0, 2500.0);
    let (_, third_hit_magic, _) = on_hit(&mut runtime, 120.0, 3.0, 2000.0, 2500.0);
    let (_, vulnerability_true) =
        ability_hit(&mut runtime, 300.0, 0.6, 0.0, 0.0, 2000.0, 2500.0, 3.2);

    assert!(third_hit_magic > 0.0);
    assert!(vulnerability_true > 0.0);
}

#[test]
fn fleet_footwork_grants_pending_heal_after_attack() {
    let mut runtime = build_loadout_runtime_state(&[], &["Fleet Footwork".to_string()], 0.0, false);
    let _ = on_hit(&mut runtime, 150.0, 1.0, 2200.0, 2600.0);
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
            ability_hit(&mut runtime, 260.0, 0.6, 200.0, 0.0, 2200.0, 2500.0, now);
        last_bonus_magic = bonus_magic;
    }
    assert_eq!(
        runtime.conqueror_stacks,
        rune_runtime_defaults().conqueror.max_stacks
    );
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

    let (physical_taken, magic_taken) =
        loadout_incoming_damage_multipliers(&runtime, 21.2, 18, 120.0, 120.0, 80.0, 80.0);
    assert!(physical_taken < 1.0);
    assert!(magic_taken < 1.0);
}

#[test]
fn electrocute_damage_matches_level_scaling_breakpoints() {
    let defaults = rune_runtime_defaults();
    for level in [1usize, 10, 18] {
        let mut runtime =
            build_loadout_runtime_state(&[], &["Electrocute".to_string()], 0.0, false);
        let _ = ability_hit_at_level(
            &mut runtime,
            200.0,
            0.0,
            0.0,
            0.0,
            2200.0,
            2500.0,
            1.0,
            level,
        );
        let _ = ability_hit_at_level(
            &mut runtime,
            200.0,
            0.0,
            0.0,
            0.0,
            2200.0,
            2500.0,
            1.6,
            level,
        );
        let (electrocute_magic, _) = ability_hit_at_level(
            &mut runtime,
            200.0,
            0.0,
            0.0,
            0.0,
            2200.0,
            2500.0,
            2.2,
            level,
        );
        let expected =
            level_scaled_range_value(level, defaults.electrocute.proc_magic_damage_by_level);
        assert!((electrocute_magic - expected).abs() < 1e-6);
    }
}

#[test]
fn arcane_comet_damage_matches_formula_breakpoints() {
    let defaults = rune_runtime_defaults();
    let attacker_ability_power = 320.0;
    let attacker_bonus_attack_damage = 110.0;
    for level in [1usize, 18] {
        let mut runtime =
            build_loadout_runtime_state(&[], &["Arcane Comet".to_string()], 0.0, false);
        let (comet_magic, _) = ability_hit_at_level(
            &mut runtime,
            240.0,
            0.0,
            attacker_ability_power,
            attacker_bonus_attack_damage,
            2200.0,
            2500.0,
            1.0,
            level,
        );
        let expected =
            level_scaled_range_value(level, defaults.arcane_comet.proc_magic_damage_by_level)
                + defaults.arcane_comet.ability_power_ratio * attacker_ability_power
                + defaults.arcane_comet.bonus_attack_damage_ratio * attacker_bonus_attack_damage;
        assert!((comet_magic - expected).abs() < 1e-6);
    }
}

#[test]
fn first_strike_window_and_cooldown_follow_level_scaling() {
    let defaults = rune_runtime_defaults();
    let raw = 300.0;
    let expected_true = defaults.first_strike.bonus_true_damage_ratio * raw;

    let mut level_one = build_loadout_runtime_state(&[], &["First Strike".to_string()], 0.0, false);
    let (_, first_true) =
        ability_hit_at_level(&mut level_one, raw, 0.0, 0.0, 0.0, 2200.0, 2500.0, 1.0, 1);
    let (_, window_true) =
        ability_hit_at_level(&mut level_one, raw, 0.0, 0.0, 0.0, 2200.0, 2500.0, 2.0, 1);
    let (_, blocked_true) =
        ability_hit_at_level(&mut level_one, raw, 0.0, 0.0, 0.0, 2200.0, 2500.0, 20.0, 1);
    let (_, post_cd_true) =
        ability_hit_at_level(&mut level_one, raw, 0.0, 0.0, 0.0, 2200.0, 2500.0, 26.0, 1);
    assert!((first_true - expected_true).abs() < 1e-6);
    assert!((window_true - expected_true).abs() < 1e-6);
    assert_eq!(blocked_true, 0.0);
    assert!((post_cd_true - expected_true).abs() < 1e-6);

    let mut level_eighteen =
        build_loadout_runtime_state(&[], &["First Strike".to_string()], 0.0, false);
    let (_, _) = ability_hit_at_level(
        &mut level_eighteen,
        raw,
        0.0,
        0.0,
        0.0,
        2200.0,
        2500.0,
        1.0,
        18,
    );
    let (_, post_cd_true_level_eighteen) = ability_hit_at_level(
        &mut level_eighteen,
        raw,
        0.0,
        0.0,
        0.0,
        2200.0,
        2500.0,
        20.0,
        18,
    );
    assert!(
        post_cd_true_level_eighteen > 0.0,
        "level 18 cooldown should be short enough to re-open before 20s"
    );
}

#[test]
fn aftershock_resist_cap_matches_level_caps() {
    let defaults = rune_runtime_defaults();
    for level in [1usize, 18] {
        let mut runtime = build_loadout_runtime_state(&[], &["Aftershock".to_string()], 0.0, true);
        let _ = trigger_immobilize_rune_damage(&mut runtime, 1.0, level, 0.0);
        let (physical_taken, magic_taken) =
            loadout_incoming_damage_multipliers(&runtime, 1.1, level, 100.0, 100.0, 1000.0, 1000.0);
        let cap = level_scaled_range_value(level, defaults.aftershock.resist_cap_by_level);
        let expected_gain = (defaults.aftershock.resist_base
            + defaults.aftershock.resist_bonus_ratio * 1000.0)
            .min(cap);
        let expected = (100.0 / (100.0 + 100.0 + expected_gain)) / (100.0 / (100.0 + 100.0));
        assert!((physical_taken - expected).abs() < 1e-6);
        assert!((magic_taken - expected).abs() < 1e-6);
    }
}

#[test]
fn rune_proc_telemetry_includes_source_breakdown() {
    let mut runtime = build_loadout_runtime_state(&[], &["Summon Aery".to_string()], 0.0, false);
    let _ = on_hit(&mut runtime, 120.0, 1.0, 1500.0, 2500.0);
    let _ = ability_hit(&mut runtime, 220.0, 0.6, 300.0, 0.0, 1500.0, 2500.0, 3.5);

    let telemetry = rune_proc_telemetry(&runtime);
    let aery = telemetry
        .iter()
        .find(|entry| entry.rune_name == "Summon Aery")
        .expect("summon aery telemetry should exist");
    let sources = aery
        .source_breakdown
        .iter()
        .map(|entry| entry.source.as_str())
        .collect::<Vec<_>>();
    assert!(sources.contains(&"on_hit"));
    assert!(sources.contains(&"ability"));
    assert!(aery.attempt_count >= aery.eligible_count);
    assert!(aery.eligible_count >= aery.proc_count);
    assert!(aery.proc_attempt_rate > 0.0);
    assert!(aery.proc_attempt_rate <= 1.0);
    assert!(aery.proc_eligible_rate > 0.0);
    assert!(aery.proc_eligible_rate <= 1.0);
}

#[test]
fn modeled_loadout_runes_have_observable_effects() {
    let mut lethal_tempo_runtime =
        build_loadout_runtime_state(&[], &["Lethal Tempo".to_string()], 0.0, false);
    let _ = on_hit(&mut lethal_tempo_runtime, 120.0, 1.0, 1800.0, 2200.0);
    assert!(loadout_attack_speed_multiplier(&lethal_tempo_runtime, 1.0) > 1.0);

    let mut grasp_runtime =
        build_loadout_runtime_state(&[], &["Grasp of the Undying".to_string()], 0.0, true);
    let (_, grasp_magic, _) = on_hit(&mut grasp_runtime, 120.0, 1.0, 1800.0, 2200.0);
    assert!(grasp_magic > 0.0);

    let mut pta_runtime =
        build_loadout_runtime_state(&[], &["Press the Attack".to_string()], 0.0, false);
    let _ = on_hit(&mut pta_runtime, 120.0, 1.0, 1800.0, 2200.0);
    let _ = on_hit(&mut pta_runtime, 120.0, 2.0, 1800.0, 2200.0);
    let (_, pta_magic, _) = on_hit(&mut pta_runtime, 120.0, 3.0, 1800.0, 2200.0);
    assert!(pta_magic > 0.0);

    let mut fleet_runtime =
        build_loadout_runtime_state(&[], &["Fleet Footwork".to_string()], 0.0, false);
    let _ = on_hit(&mut fleet_runtime, 120.0, 1.0, 1800.0, 2200.0);
    assert!(on_outgoing_damage_heal(&mut fleet_runtime, 150.0, 1.0) > 0.0);

    let mut conqueror_runtime =
        build_loadout_runtime_state(&[], &["Conqueror".to_string()], 0.0, false);
    for step in 0..6 {
        let _ = ability_hit(
            &mut conqueror_runtime,
            260.0,
            0.7,
            200.0,
            0.0,
            2200.0,
            2500.0,
            1.0 + step as f64,
        );
    }
    assert!(on_outgoing_damage_heal(&mut conqueror_runtime, 400.0, 10.0) > 0.0);

    let mut aftershock_runtime =
        build_loadout_runtime_state(&[], &["Aftershock".to_string()], 0.0, true);
    assert!(trigger_immobilize_rune_damage(&mut aftershock_runtime, 1.0, 18, 400.0) > 0.0);

    let mut electrocute_runtime =
        build_loadout_runtime_state(&[], &["Electrocute".to_string()], 0.0, false);
    let _ = ability_hit(
        &mut electrocute_runtime,
        200.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        1.0,
    );
    let _ = ability_hit(
        &mut electrocute_runtime,
        200.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        1.6,
    );
    let (electrocute_magic, _) = ability_hit(
        &mut electrocute_runtime,
        200.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        2.2,
    );
    assert!(electrocute_magic > 0.0);

    let mut first_strike_runtime =
        build_loadout_runtime_state(&[], &["First Strike".to_string()], 0.0, false);
    let (_, first_strike_true) = ability_hit(
        &mut first_strike_runtime,
        240.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        1.0,
    );
    assert!(first_strike_true > 0.0);

    let mut phase_rush_runtime =
        build_loadout_runtime_state(&[], &["Phase Rush".to_string()], 0.0, false);
    let _ = ability_hit(
        &mut phase_rush_runtime,
        200.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        1.0,
    );
    let _ = ability_hit(
        &mut phase_rush_runtime,
        200.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        1.6,
    );
    let _ = ability_hit(
        &mut phase_rush_runtime,
        200.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        2.2,
    );
    assert!(loadout_movement_speed_multiplier(&phase_rush_runtime, 2.3, 18) > 1.0);

    let mut comet_runtime =
        build_loadout_runtime_state(&[], &["Arcane Comet".to_string()], 0.0, false);
    let (comet_magic, _) = ability_hit(
        &mut comet_runtime,
        240.0,
        0.6,
        300.0,
        0.0,
        2200.0,
        2500.0,
        1.0,
    );
    assert!(comet_magic > 0.0);

    let mut aery_runtime =
        build_loadout_runtime_state(&[], &["Summon Aery".to_string()], 0.0, false);
    let (_, aery_magic, _) = on_hit(&mut aery_runtime, 120.0, 1.0, 1800.0, 2200.0);
    assert!(aery_magic > 0.0);

    let mut hail_runtime =
        build_loadout_runtime_state(&[], &["Hail of Blades".to_string()], 0.0, false);
    let _ = on_hit(&mut hail_runtime, 120.0, 1.0, 1800.0, 2200.0);
    assert!(loadout_attack_speed_multiplier(&hail_runtime, 1.1) > 1.0);

    let mut harvest_runtime =
        build_loadout_runtime_state(&[], &["Dark Harvest".to_string()], 0.0, false);
    let (harvest_magic, _) = ability_hit(
        &mut harvest_runtime,
        220.0,
        0.6,
        200.0,
        0.0,
        400.0,
        1000.0,
        1.0,
    );
    assert!(harvest_magic > 0.0);

    let second_wind_runtime =
        build_loadout_runtime_state(&[], &["Second Wind".to_string()], 0.0, false);
    assert!(tick_loadout_regeneration(&second_wind_runtime, 1400.0, 2000.0, 1.0) > 0.0);

    let mut triumph_runtime =
        build_loadout_runtime_state(&[], &["Triumph".to_string()], 0.0, false);
    assert!(on_enemy_kill_heal(&mut triumph_runtime, 2000.0) > 0.0);

    let mut gathering_storm_runtime =
        build_loadout_runtime_state(&[], &["Gathering Storm".to_string()], 0.0, false);
    let (gathering_storm_magic, _) = ability_hit(
        &mut gathering_storm_runtime,
        200.0,
        0.6,
        0.0,
        0.0,
        2200.0,
        2500.0,
        600.0,
    );
    assert!(gathering_storm_magic > 0.0);
}

#[test]
#[ignore = "manual performance check"]
fn rune_proc_telemetry_overhead_smoke_benchmark() {
    fn run_workload(telemetry_enabled: bool) -> f64 {
        let mut runtime = build_loadout_runtime_state(
            &["Luden's Echo".to_string()],
            &[
                "Press the Attack".to_string(),
                "Fleet Footwork".to_string(),
                "Conqueror".to_string(),
                "Electrocute".to_string(),
                "First Strike".to_string(),
                "Arcane Comet".to_string(),
                "Summon Aery".to_string(),
                "Dark Harvest".to_string(),
                "Triumph".to_string(),
                "Aftershock".to_string(),
            ],
            0.0,
            false,
        );
        runtime.rune_proc_telemetry_enabled = telemetry_enabled;
        let start = Instant::now();
        for step in 0..30_000 {
            let now = 1.0 + step as f64 * 0.01;
            let _ = on_hit(&mut runtime, 145.0, now, 1800.0, 2500.0);
            let _ = ability_hit(
                &mut runtime,
                260.0,
                0.7,
                300.0,
                0.0,
                1200.0,
                2500.0,
                now + 0.005,
            );
            let _ = on_outgoing_damage_heal(&mut runtime, 220.0, now + 0.006);
            if step % 2000 == 0 {
                let _ = on_enemy_kill_heal(&mut runtime, 2500.0);
            }
        }
        start.elapsed().as_secs_f64()
    }

    let disabled_seconds = run_workload(false);
    let enabled_seconds = run_workload(true);
    let ratio = enabled_seconds / disabled_seconds.max(1e-9);
    eprintln!(
        "rune telemetry overhead benchmark: disabled={:.4}s enabled={:.4}s ratio={:.3}",
        disabled_seconds, enabled_seconds, ratio
    );
    assert!(enabled_seconds > 0.0);
    assert!(disabled_seconds > 0.0);
}

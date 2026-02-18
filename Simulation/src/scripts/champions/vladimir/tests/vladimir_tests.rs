use super::*;

fn test_profile() -> VladimirCastProfile {
    VladimirCastProfile {
        q_ability_id: "vladimir_transfusion".to_string(),
        e_ability_id: "vladimir_tides_of_blood".to_string(),
        r_ability_id: "vladimir_hemoplague".to_string(),
        pool_ability_id: "vladimir_sanguine_pool".to_string(),
        q_range: 600.0,
        q_windup_seconds: 0.20,
        q_projectile_speed: 1200.0,
        q_effect_hitbox_radius: 80.0,
        e_range: 600.0,
        e_windup_seconds: 0.30,
        e_projectile_speed: 1000.0,
        e_effect_hitbox_radius: 275.0,
        r_range: 700.0,
        r_windup_seconds: 0.25,
        r_projectile_speed: 2000.0,
        r_effect_hitbox_radius: 375.0,
    }
}

#[test]
fn offensive_decisions_schedule_ready_spells() {
    let profile = test_profile();
    let decisions = decide_offensive_casts(VladimirOffensiveDecisionInput {
        now_seconds: 10.0,
        can_cast: true,
        q_ready_at: 10.0,
        e_ready_at: 9.0,
        r_ready_at: 12.0,
        cooldowns: VladimirAbilityCooldowns {
            q_seconds: 3.5,
            e_seconds: 5.0,
            r_seconds: 60.0,
        },
        cast_profile: profile,
        q_target: Some(VladimirTargetSnapshot {
            target_index: 2,
            distance: 600.0,
        }),
        e_max_distance: Some(500.0),
        r_max_distance: Some(650.0),
    });

    let q = decisions.q.expect("q cast should be scheduled");
    assert_eq!(q.ability_id, "vladimir_transfusion");
    assert_eq!(q.target_index, 2);
    assert!((q.impact_delay_seconds - 0.70).abs() < 1e-9);
    assert!((q.next_ready_at - 13.50).abs() < 1e-9);

    let e = decisions.e.expect("e cast should be scheduled");
    assert_eq!(e.ability_id, "vladimir_tides_of_blood");
    assert!((e.impact_delay_seconds - 0.80).abs() < 1e-9);
    assert!((e.next_ready_at - 15.00).abs() < 1e-9);

    assert!(decisions.r.is_none());
}

#[test]
fn offensive_decisions_require_cast_permission() {
    let decisions = decide_offensive_casts(VladimirOffensiveDecisionInput {
        now_seconds: 20.0,
        can_cast: false,
        q_ready_at: 0.0,
        e_ready_at: 0.0,
        r_ready_at: 0.0,
        cooldowns: VladimirAbilityCooldowns {
            q_seconds: 1.0,
            e_seconds: 1.0,
            r_seconds: 1.0,
        },
        cast_profile: test_profile(),
        q_target: Some(VladimirTargetSnapshot {
            target_index: 0,
            distance: 200.0,
        }),
        e_max_distance: Some(200.0),
        r_max_distance: Some(200.0),
    });
    assert!(decisions.q.is_none());
    assert!(decisions.e.is_none());
    assert!(decisions.r.is_none());
}

#[test]
fn defensive_ability_decisions_match_trigger_conditions() {
    let decisions = decide_defensive_ability_activations(VladimirDefensiveAbilityDecisionInput {
        now_seconds: 10.0,
        can_cast: true,
        pool_ready_at: 8.0,
    });
    assert!(decisions.cast_pool);
}

#[test]
fn defensive_ability_decisions_require_readiness_and_cast_permission() {
    let not_ready = decide_defensive_ability_activations(VladimirDefensiveAbilityDecisionInput {
        now_seconds: 10.0,
        can_cast: true,
        pool_ready_at: 11.0,
    });
    assert!(!not_ready.cast_pool);

    let cannot_cast = decide_defensive_ability_activations(VladimirDefensiveAbilityDecisionInput {
        now_seconds: 10.0,
        can_cast: false,
        pool_ready_at: 0.0,
    });
    assert!(!cannot_cast.cast_pool);
}

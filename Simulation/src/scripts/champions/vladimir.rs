pub(crate) mod abilities;
pub(crate) mod decisions;
pub(crate) mod hook;

pub(crate) use abilities::{
    VladimirAbilityCooldowns, VladimirAbilityTuning, e_damage_raw, offensive_cooldowns_after_haste,
    q_damage_raw, r_damage_raw,
};
pub(crate) use decisions::{
    VladimirCastProfile, VladimirDefensiveDecisionInput, VladimirGuardianAngelDecisionInput,
    VladimirOffensiveDecisionInput, VladimirTargetSnapshot, decide_defensive_activations,
    decide_offensive_casts, default_cast_profile, should_trigger_guardian_angel,
};
pub(crate) use hook::VLADIMIR_HOOK;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_profile() -> VladimirCastProfile {
        VladimirCastProfile {
            q_range: 600.0,
            q_windup_seconds: 0.20,
            q_projectile_speed: 1200.0,
            e_range: 600.0,
            e_windup_seconds: 0.30,
            e_projectile_speed: 1000.0,
            r_range: 700.0,
            r_windup_seconds: 0.25,
            r_projectile_speed: 2000.0,
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
                enemy_index: 2,
                distance: 600.0,
            }),
            e_max_distance: Some(500.0),
            r_max_distance: Some(650.0),
        });

        let q = decisions.q.expect("q cast should be scheduled");
        assert_eq!(q.target_index, 2);
        assert!((q.impact_delay_seconds - 0.70).abs() < 1e-9);
        assert!((q.next_ready_at - 13.50).abs() < 1e-9);

        let e = decisions.e.expect("e cast should be scheduled");
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
                enemy_index: 0,
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
    fn defensive_decisions_match_trigger_conditions() {
        let decisions = decide_defensive_activations(VladimirDefensiveDecisionInput {
            now_seconds: 10.0,
            can_cast: true,
            health: 320.0,
            max_health: 1000.0,
            pool_ready_at: 8.0,
            zhonya_available: true,
            zhonya_ready_at: 7.0,
            zhonya_trigger_health_percent: 0.35,
            pool_active_until: 10.0,
            ga_revive_active_until: 9.0,
            protoplasm_available: true,
            protoplasm_ready_at: 6.0,
            protoplasm_trigger_health_percent: 0.40,
        });
        assert!(decisions.cast_pool);
        assert!(decisions.activate_zhonya);
        assert!(decisions.activate_protoplasm);
    }

    #[test]
    fn defensive_decisions_block_zhonya_while_pool_or_revive_active() {
        let blocked_by_pool = decide_defensive_activations(VladimirDefensiveDecisionInput {
            now_seconds: 10.0,
            can_cast: true,
            health: 200.0,
            max_health: 1000.0,
            pool_ready_at: 0.0,
            zhonya_available: true,
            zhonya_ready_at: 0.0,
            zhonya_trigger_health_percent: 0.50,
            pool_active_until: 11.0,
            ga_revive_active_until: 0.0,
            protoplasm_available: false,
            protoplasm_ready_at: 0.0,
            protoplasm_trigger_health_percent: 0.0,
        });
        assert!(!blocked_by_pool.activate_zhonya);

        let blocked_by_revive = decide_defensive_activations(VladimirDefensiveDecisionInput {
            ga_revive_active_until: 11.0,
            pool_active_until: 0.0,
            ..VladimirDefensiveDecisionInput {
                now_seconds: 10.0,
                can_cast: true,
                health: 200.0,
                max_health: 1000.0,
                pool_ready_at: 0.0,
                zhonya_available: true,
                zhonya_ready_at: 0.0,
                zhonya_trigger_health_percent: 0.50,
                pool_active_until: 0.0,
                ga_revive_active_until: 0.0,
                protoplasm_available: false,
                protoplasm_ready_at: 0.0,
                protoplasm_trigger_health_percent: 0.0,
            }
        });
        assert!(!blocked_by_revive.activate_zhonya);
    }

    #[test]
    fn guardian_angel_trigger_checks_cooldown_and_availability() {
        assert!(should_trigger_guardian_angel(
            VladimirGuardianAngelDecisionInput {
                available: true,
                now_seconds: 120.0,
                ready_at: 120.0,
            }
        ));
        assert!(!should_trigger_guardian_angel(
            VladimirGuardianAngelDecisionInput {
                available: true,
                now_seconds: 119.9,
                ready_at: 120.0,
            }
        ));
        assert!(!should_trigger_guardian_angel(
            VladimirGuardianAngelDecisionInput {
                available: false,
                now_seconds: 120.0,
                ready_at: 0.0,
            }
        ));
    }
}

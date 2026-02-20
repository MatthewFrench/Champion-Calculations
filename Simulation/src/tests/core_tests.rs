use super::*;

const EPS: f64 = 1e-9;

#[test]
fn status_effect_refresh_duration_keeps_effect_active() {
    let mut effects = StatusEffectSet::default();
    effects.apply(StatusEffect::timed(
        StatusEffectKind::Stun,
        1.0,
        1,
        StatusPersistence::RefreshDuration,
    ));
    effects.tick(0.4);
    effects.apply(StatusEffect::timed(
        StatusEffectKind::Stun,
        2.0,
        1,
        StatusPersistence::RefreshDuration,
    ));

    let remaining = effects
        .effects()
        .first()
        .and_then(|effect| effect.duration.remaining_seconds())
        .unwrap_or_default();
    assert!((remaining - 2.0).abs() < EPS);
    assert!(effects.is_active(&StatusEffectKind::Stun));

    effects.tick(2.1);
    assert!(!effects.is_active(&StatusEffectKind::Stun));
}

#[test]
fn status_effect_stack_refresh_respects_max_stacks() {
    let mut effects = StatusEffectSet::default();
    effects.apply(
        StatusEffect::timed(
            StatusEffectKind::Custom("hemoplague"),
            5.0,
            2,
            StatusPersistence::StackRefreshDuration,
        )
        .with_max_stacks(5),
    );
    effects.apply(
        StatusEffect::timed(
            StatusEffectKind::Custom("hemoplague"),
            2.0,
            4,
            StatusPersistence::StackRefreshDuration,
        )
        .with_max_stacks(5),
    );

    assert_eq!(
        effects.total_stacks(&StatusEffectKind::Custom("hemoplague")),
        5
    );
    let remaining = effects
        .effects()
        .first()
        .and_then(|effect| effect.duration.remaining_seconds())
        .unwrap_or_default();
    assert!((remaining - 5.0).abs() < EPS);
}

#[test]
fn status_effect_set_supports_independent_and_persistent_effects() {
    let mut effects = StatusEffectSet::default();
    effects.apply(
        StatusEffect::persistent(StatusEffectKind::Silence, 1, StatusPersistence::Replace)
            .with_max_stacks(2),
    );
    effects.apply(StatusEffect::timed(
        StatusEffectKind::Root,
        0.3,
        1,
        StatusPersistence::Independent,
    ));
    effects.apply(StatusEffect::timed(
        StatusEffectKind::Slow,
        0.1,
        1,
        StatusPersistence::Independent,
    ));

    effects.tick(0.2);
    assert!(effects.is_active(&StatusEffectKind::Silence));
    assert!(effects.is_active(&StatusEffectKind::Root));
    assert!(!effects.is_active(&StatusEffectKind::Slow));
}

#[test]
fn cast_lock_state_advances_through_all_phases() {
    let mut cast_lock = CastLockState::default();
    cast_lock.begin(CastLockWindow::new(0.2, 0.5, 0.3));
    assert_eq!(cast_lock.phase(), CastLockPhase::Windup);
    assert!(cast_lock.is_locked());

    cast_lock.tick(0.2);
    assert_eq!(cast_lock.phase(), CastLockPhase::Channel);

    cast_lock.tick(0.5);
    assert_eq!(cast_lock.phase(), CastLockPhase::Lockout);

    cast_lock.tick(0.3);
    assert_eq!(cast_lock.phase(), CastLockPhase::Idle);
    assert!(!cast_lock.is_locked());
    let remaining = cast_lock.remaining();
    assert!(remaining.windup_seconds.abs() < EPS);
    assert!(remaining.channel_seconds.abs() < EPS);
    assert!(remaining.lockout_seconds.abs() < EPS);
}

#[test]
fn combat_primitives_tick_updates_status_and_cast_lock() {
    let mut state = CombatPrimitivesState::default();
    state.apply_status(StatusEffect::timed(
        StatusEffectKind::Stasis,
        0.5,
        1,
        StatusPersistence::Replace,
    ));
    state.begin_cast_lock(CastLockWindow::new(0.1, 0.0, 0.2));

    state.tick(0.15);
    assert!(state.status_effects().is_active(&StatusEffectKind::Stasis));
    assert_eq!(state.cast_lock().phase(), CastLockPhase::Lockout);

    state.tick(0.5);
    assert!(!state.status_effects().is_active(&StatusEffectKind::Stasis));
    assert_eq!(state.cast_lock().phase(), CastLockPhase::Idle);
}

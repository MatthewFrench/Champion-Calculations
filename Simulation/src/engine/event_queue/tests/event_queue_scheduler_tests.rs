use super::*;

#[test]
fn event_queue_scheduler_orders_by_time_then_priority_then_sequence() {
    let mut queue = EventQueueScheduler::new();
    queue.enqueue_event(0.0, 1.0, 10, EventType::Attack(0), None);
    queue.enqueue_event(0.0, 1.0, 12, EventType::Attack(1), None);
    queue.enqueue_event(0.0, 0.5, 1, EventType::Attack(2), None);
    queue.enqueue_event(0.0, 1.0, 12, EventType::Attack(3), None);

    let first = queue.pop_next().expect("expected first event");
    let second = queue.pop_next().expect("expected second event");
    let third = queue.pop_next().expect("expected third event");
    let fourth = queue.pop_next().expect("expected fourth event");

    assert!(matches!(first.kind, EventType::Attack(2)));
    assert!(matches!(second.kind, EventType::Attack(0)));
    assert!(matches!(third.kind, EventType::Attack(1)));
    assert!(matches!(fourth.kind, EventType::Attack(3)));
}

#[test]
fn event_queue_scheduler_reschedules_positive_recurring_events() {
    let mut queue = EventQueueScheduler::new();
    queue.enqueue_event(5.0, 0.25, 7, EventType::ControlledChampionAttack, Some(2.0));
    let next = queue.pop_next().expect("expected initial recurring event");
    queue.reschedule_recurring_event(next.time, next.priority, next.recurring, next.kind.clone());
    let recurring = queue.pop_next().expect("expected recurring requeue");
    assert!((recurring.time - 7.25).abs() < 1e-9);
    assert_eq!(recurring.priority, 7);
    assert!(matches!(
        recurring.kind,
        EventType::ControlledChampionAttack
    ));
    assert_eq!(recurring.recurring, Some(2.0));
}

#[test]
fn event_queue_scheduler_reports_next_attack_and_impact_queries() {
    let mut queue = EventQueueScheduler::new();
    queue.enqueue_event(0.0, 4.0, 10, EventType::Attack(2), None);
    queue.enqueue_event(
        0.0,
        2.5,
        11,
        EventType::AttackHit {
            idx: 2,
            token: 1,
            source: crate::engine::geometry::Vec2 { x: 0.0, y: 0.0 },
            target_at_release: crate::engine::geometry::Vec2 { x: 10.0, y: 0.0 },
            projectile_speed: 1000.0,
            effect_hitbox_radius: 0.0,
        },
        None,
    );
    queue.enqueue_event(0.0, 3.0, 12, EventType::ControlledChampionAttack, None);
    queue.enqueue_event(
        0.0,
        1.5,
        12,
        EventType::ControlledChampionAttackHit {
            idx: 1,
            token: 1,
            source: crate::engine::geometry::Vec2 { x: 0.0, y: 0.0 },
            target_at_release: crate::engine::geometry::Vec2 { x: 10.0, y: 0.0 },
            projectile_speed: 1000.0,
            effect_hitbox_radius: 0.0,
        },
        None,
    );

    assert_eq!(queue.next_enemy_attack_ready_at(2), Some(4.0));
    assert_eq!(queue.next_enemy_attack_impact_at(2), Some(2.5));
    assert_eq!(queue.next_controlled_champion_attack_ready_at(), Some(3.0));
    assert_eq!(
        queue.next_controlled_champion_attack_impact_at(),
        Some((1, 1.5))
    );
}

#[test]
fn event_queue_scheduler_collects_projectile_projections_in_time_order() {
    let mut queue = EventQueueScheduler::new();
    queue.enqueue_event(
        0.0,
        3.0,
        12,
        EventType::ControlledChampionAttackHit {
            idx: 2,
            token: 1,
            source: crate::engine::geometry::Vec2 { x: 0.0, y: 0.0 },
            target_at_release: crate::engine::geometry::Vec2 { x: 10.0, y: 0.0 },
            projectile_speed: 1000.0,
            effect_hitbox_radius: 0.0,
        },
        None,
    );
    queue.enqueue_event(
        0.0,
        1.0,
        12,
        EventType::AttackHit {
            idx: 4,
            token: 1,
            source: crate::engine::geometry::Vec2 { x: 0.0, y: 0.0 },
            target_at_release: crate::engine::geometry::Vec2 { x: 10.0, y: 0.0 },
            projectile_speed: 1000.0,
            effect_hitbox_radius: 0.0,
        },
        None,
    );
    queue.enqueue_event(
        0.0,
        2.0,
        12,
        EventType::ControlledChampionOffensivePrimaryHit {
            idx: 3,
            source: crate::engine::geometry::Vec2 { x: 0.0, y: 0.0 },
            target_at_cast: crate::engine::geometry::Vec2 { x: 10.0, y: 0.0 },
            projectile_speed: 1000.0,
            effect_hitbox_radius: 0.0,
        },
        None,
    );

    let projections = queue.queued_projectile_impact_projections(0.0);
    assert_eq!(projections.len(), 3);
    assert!((projections[0].time_seconds - 1.0).abs() < 1e-9);
    assert!(matches!(
        projections[0].kind,
        QueuedProjectileImpactKind::EnemyAttack { enemy_index: 4 }
    ));
    assert!((projections[1].time_seconds - 2.0).abs() < 1e-9);
    assert!(matches!(
        projections[1].kind,
        QueuedProjectileImpactKind::ControlledChampionOffensivePrimary { enemy_index: 3 }
    ));
    assert!((projections[2].time_seconds - 3.0).abs() < 1e-9);
    assert!(matches!(
        projections[2].kind,
        QueuedProjectileImpactKind::ControlledChampionAttack { enemy_index: 2 }
    ));
}

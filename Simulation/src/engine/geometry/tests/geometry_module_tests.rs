use super::*;

use crate::scripts::champions::ChampionBehaviorProfile;

#[test]
fn projectile_travel_seconds_handles_instant_and_ranged() {
    assert_eq!(projectile_travel_seconds(400.0, 0.0), 0.0);
    assert!((projectile_travel_seconds(500.0, 2000.0) - 0.25).abs() < 1e-9);
}

#[test]
fn enemy_spawn_position_keeps_melee_closer_than_ranged() {
    let melee = ChampionBehaviorProfile::default_for(true, 125.0, 0.0);
    let ranged = ChampionBehaviorProfile::default_for(false, 550.0, 2000.0);
    let melee_pos = enemy_spawn_position(0, 5, melee);
    let ranged_pos = enemy_spawn_position(0, 5, ranged);
    let origin = Vec2 { x: 0.0, y: 0.0 };
    assert!(melee_pos.distance_to(origin) < ranged_pos.distance_to(origin));
}

#[test]
fn segment_intersection_covers_cross_and_miss_cases() {
    let source = Vec2 { x: 0.0, y: 0.0 };
    let target = Vec2 { x: 1000.0, y: 0.0 };
    let wall_start = Vec2 { x: 500.0, y: 200.0 };
    let wall_end = Vec2 {
        x: 500.0,
        y: -200.0,
    };
    assert!(segment_intersection_checks::line_segments_intersect(
        source, target, wall_start, wall_end
    ));

    let miss_start = Vec2 { x: 500.0, y: 300.0 };
    let miss_end = Vec2 { x: 500.0, y: 600.0 };
    assert!(!segment_intersection_checks::line_segments_intersect(
        source, target, miss_start, miss_end
    ));
}

#[test]
fn enemy_orbit_position_update_moves_toward_desired_range() {
    let current = Vec2 { x: 600.0, y: 0.0 };
    let target = Vec2 { x: 0.0, y: 0.0 };
    let moved = update_enemy_orbit_position(current, target, 50.0, 500.0, 1.0, 0.0);
    assert!((moved.x - 550.0).abs() < 1e-9);
    assert!((moved.y - 0.0).abs() < 1e-9);
}

#[test]
fn enemy_orbit_position_update_applies_tangential_offset() {
    let current = Vec2 { x: 500.0, y: 0.0 };
    let target = Vec2 { x: 0.0, y: 0.0 };
    let moved = update_enemy_orbit_position(current, target, 100.0, 500.0, -1.0, 0.2);
    assert!((moved.x - 500.0).abs() < 1e-9);
    assert!((moved.y + 20.0).abs() < 1e-9);
}

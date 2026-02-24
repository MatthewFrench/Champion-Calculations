pub(super) mod enemy_orbit_position_updates;
pub(super) mod hitbox_distance_checks;
pub(super) mod projectile_kinematics;
pub(super) mod range_reach_checks;
pub(super) mod segment_intersection_checks;
pub(super) mod spawn_positioning;
pub(super) mod vector_2d_math;

pub(super) use self::enemy_orbit_position_updates::update_enemy_orbit_position;
pub(super) use self::hitbox_distance_checks::{distance_segment_to_segment, path_hits_circle};
pub(super) use self::projectile_kinematics::projectile_travel_seconds;
pub(super) use self::range_reach_checks::{hitbox_miss_reason, within_reach_with_hitboxes};
pub(super) use self::spawn_positioning::enemy_spawn_position;
pub(super) use self::vector_2d_math::Vec2;

#[cfg(test)]
#[path = "geometry/tests/geometry_module_tests.rs"]
mod tests;

use super::vector_2d_math::Vec2;

const MINIMUM_DESIRED_COMBAT_RANGE: f64 = 75.0;

pub(in crate::engine) fn update_enemy_orbit_position(
    current_position: Vec2,
    target_position: Vec2,
    movement_step: f64,
    desired_combat_range: f64,
    tangential_direction: f64,
    tangential_step_scale: f64,
) -> Vec2 {
    if movement_step <= 0.0 {
        return current_position;
    }

    let mut radial = Vec2 {
        x: current_position.x - target_position.x,
        y: current_position.y - target_position.y,
    };
    let distance = radial.distance_to(Vec2 { x: 0.0, y: 0.0 }).max(1e-6);
    radial.x /= distance;
    radial.y /= distance;

    let desired = desired_combat_range.max(MINIMUM_DESIRED_COMBAT_RANGE);
    let radial_error = distance - desired;
    let radial_step = radial_error.clamp(-movement_step, movement_step);
    let mut next_position = Vec2 {
        x: current_position.x - radial.x * radial_step,
        y: current_position.y - radial.y * radial_step,
    };

    let tangent = Vec2 {
        x: -radial.y * tangential_direction,
        y: radial.x * tangential_direction,
    };
    let tangential_step = movement_step * tangential_step_scale;
    next_position.x += tangent.x * tangential_step;
    next_position.y += tangent.y * tangential_step;
    next_position
}

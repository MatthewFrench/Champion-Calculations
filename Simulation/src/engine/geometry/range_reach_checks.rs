use super::hitbox_distance_checks::distance_point_to_segment;
use super::vector_2d_math::Vec2;

pub(in crate::engine) fn within_reach_with_hitboxes(
    center_distance: f64,
    range: f64,
    source_hitbox_radius: f64,
    target_hitbox_radius: f64,
    effect_hitbox_radius: f64,
) -> bool {
    center_distance
        <= range.max(0.0)
            + source_hitbox_radius.max(0.0)
            + target_hitbox_radius.max(0.0)
            + effect_hitbox_radius.max(0.0)
}

pub(in crate::engine) fn hitbox_miss_reason(
    source: Vec2,
    aim_point: Vec2,
    target_center: Vec2,
    target_hitbox_radius: f64,
    effect_hitbox_radius: f64,
) -> String {
    let reach = target_hitbox_radius.max(0.0) + effect_hitbox_radius.max(0.0);
    let path_distance = if source.distance_to(aim_point) <= 1e-9 {
        source.distance_to(target_center)
    } else {
        distance_point_to_segment(target_center, source, aim_point)
    };
    format!(
        "target outside hitbox path (distance {:.1} > reach {:.1})",
        path_distance, reach
    )
}

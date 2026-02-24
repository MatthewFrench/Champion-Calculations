use super::segment_intersection_checks::line_segments_intersect;
use super::vector_2d_math::{Vec2, dot};

pub(super) fn distance_point_to_segment(point: Vec2, seg_start: Vec2, seg_end: Vec2) -> f64 {
    let segment = Vec2 {
        x: seg_end.x - seg_start.x,
        y: seg_end.y - seg_start.y,
    };
    let len_sq = dot(segment, segment);
    if len_sq <= 1e-9 {
        return point.distance_to(seg_start);
    }
    let from_start = Vec2 {
        x: point.x - seg_start.x,
        y: point.y - seg_start.y,
    };
    let t = (dot(from_start, segment) / len_sq).clamp(0.0, 1.0);
    let projection = Vec2 {
        x: seg_start.x + segment.x * t,
        y: seg_start.y + segment.y * t,
    };
    point.distance_to(projection)
}

pub(in crate::engine) fn distance_segment_to_segment(
    a1: Vec2,
    a2: Vec2,
    b1: Vec2,
    b2: Vec2,
) -> f64 {
    if line_segments_intersect(a1, a2, b1, b2) {
        return 0.0;
    }
    distance_point_to_segment(a1, b1, b2)
        .min(distance_point_to_segment(a2, b1, b2))
        .min(distance_point_to_segment(b1, a1, a2))
        .min(distance_point_to_segment(b2, a1, a2))
}

pub(in crate::engine) fn path_hits_circle(
    source: Vec2,
    aim_point: Vec2,
    target_center: Vec2,
    target_hitbox_radius: f64,
    effect_hitbox_radius: f64,
) -> bool {
    let reach = target_hitbox_radius.max(0.0) + effect_hitbox_radius.max(0.0);
    if source.distance_to(aim_point) <= 1e-9 {
        return source.distance_to(target_center) <= reach;
    }
    distance_point_to_segment(target_center, source, aim_point) <= reach
}

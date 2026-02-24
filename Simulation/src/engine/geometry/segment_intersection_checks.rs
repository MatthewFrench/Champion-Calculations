use super::vector_2d_math::{Vec2, almost_zero, cross};

fn on_segment(a: Vec2, b: Vec2, p: Vec2) -> bool {
    if !almost_zero(cross(a, b, p)) {
        return false;
    }
    let min_x = a.x.min(b.x) - 1e-9;
    let max_x = a.x.max(b.x) + 1e-9;
    let min_y = a.y.min(b.y) - 1e-9;
    let max_y = a.y.max(b.y) + 1e-9;
    p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
}

pub(in crate::engine) fn line_segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = cross(a1, a2, b1);
    let d2 = cross(a1, a2, b2);
    let d3 = cross(b1, b2, a1);
    let d4 = cross(b1, b2, a2);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }

    if almost_zero(d1) && on_segment(a1, a2, b1) {
        return true;
    }
    if almost_zero(d2) && on_segment(a1, a2, b2) {
        return true;
    }
    if almost_zero(d3) && on_segment(b1, b2, a1) {
        return true;
    }
    if almost_zero(d4) && on_segment(b1, b2, a2) {
        return true;
    }

    false
}

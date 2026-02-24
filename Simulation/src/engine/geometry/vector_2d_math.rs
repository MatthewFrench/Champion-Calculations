#[derive(Debug, Clone, Copy)]
pub(in crate::engine) struct Vec2 {
    pub(in crate::engine) x: f64,
    pub(in crate::engine) y: f64,
}

impl Vec2 {
    pub(in crate::engine) fn distance_to(self, other: Vec2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub(super) fn cross(a: Vec2, b: Vec2, c: Vec2) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

pub(super) fn dot(a: Vec2, b: Vec2) -> f64 {
    a.x * b.x + a.y * b.y
}

pub(super) fn almost_zero(value: f64) -> bool {
    value.abs() <= 1e-9
}

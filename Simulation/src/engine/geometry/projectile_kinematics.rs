pub(in crate::engine) fn projectile_travel_seconds(distance: f64, speed: f64) -> f64 {
    if speed <= 0.0 {
        0.0
    } else {
        (distance / speed).max(0.0)
    }
}

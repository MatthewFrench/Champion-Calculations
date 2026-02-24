use super::{champion_script_point_from_vec2, vec2_from_champion_script_point};
use crate::engine::geometry::Vec2;

#[test]
fn champion_script_point_from_vec2_preserves_coordinates() {
    let source = Vec2 {
        x: 123.25,
        y: -456.75,
    };
    let point = champion_script_point_from_vec2(source);
    assert_eq!(point.x, 123.25);
    assert_eq!(point.y, -456.75);
}

#[test]
fn vec2_from_champion_script_point_round_trips_coordinates() {
    let source = Vec2 { x: -32.0, y: 48.5 };
    let point = champion_script_point_from_vec2(source);
    let round_trip = vec2_from_champion_script_point(point);
    assert_eq!(round_trip.x, source.x);
    assert_eq!(round_trip.y, source.y);
}

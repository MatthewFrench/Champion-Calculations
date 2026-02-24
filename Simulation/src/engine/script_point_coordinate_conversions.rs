use crate::scripts::champions::ChampionScriptPoint;

use super::geometry::Vec2;

pub(in crate::engine) fn champion_script_point_from_vec2(point: Vec2) -> ChampionScriptPoint {
    ChampionScriptPoint {
        x: point.x,
        y: point.y,
    }
}

pub(in crate::engine) fn vec2_from_champion_script_point(point: ChampionScriptPoint) -> Vec2 {
    Vec2 {
        x: point.x,
        y: point.y,
    }
}

#[cfg(test)]
#[path = "tests/script_point_coordinate_conversions_tests.rs"]
mod tests;

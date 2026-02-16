use super::{
    ChampionScriptAction, ChampionScriptEvent, ChampionScriptExecutionInput, ChampionScriptPoint,
    ChampionScriptSchedule,
};

pub(crate) const CHAMPION_KEY: &str = "yasuo";

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::YasuoWindWall,
        start_offset_seconds: 5.0,
        interval_seconds: 18.0,
    }]
}

pub(crate) fn execute_wind_wall(input: ChampionScriptExecutionInput) -> Vec<ChampionScriptAction> {
    let to_target_x = input.target_position.x - input.actor_position.x;
    let to_target_y = input.target_position.y - input.actor_position.y;
    let length = (to_target_x * to_target_x + to_target_y * to_target_y)
        .sqrt()
        .max(1e-6);
    let nx = to_target_x / length;
    let ny = to_target_y / length;
    let center = ChampionScriptPoint {
        x: input.actor_position.x + nx * 180.0,
        y: input.actor_position.y + ny * 180.0,
    };
    let tangent_x = -ny;
    let tangent_y = nx;
    let half_width = 180.0;
    let start = ChampionScriptPoint {
        x: center.x + tangent_x * half_width,
        y: center.y + tangent_y * half_width,
    };
    let end = ChampionScriptPoint {
        x: center.x - tangent_x * half_width,
        y: center.y - tangent_y * half_width,
    };
    vec![ChampionScriptAction::CreateProjectileBlockZone {
        start,
        end,
        half_width: 40.0,
        duration_seconds: 4.0,
    }]
}

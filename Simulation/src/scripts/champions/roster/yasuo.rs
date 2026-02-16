use super::{
    EnemyScriptAction, EnemyScriptEvent, EnemyScriptExecutionInput, EnemyScriptPoint,
    EnemyScriptSchedule,
};

pub(crate) const CHAMPION_KEY: &str = "yasuo";

pub(crate) fn schedules() -> Vec<EnemyScriptSchedule> {
    vec![EnemyScriptSchedule {
        event: EnemyScriptEvent::YasuoWindWall,
        start_offset_seconds: 5.0,
        interval_seconds: 18.0,
    }]
}

pub(crate) fn execute_wind_wall(input: EnemyScriptExecutionInput) -> Vec<EnemyScriptAction> {
    let to_controlled_champion_x = input.controlled_champion_position.x - input.enemy_position.x;
    let to_controlled_champion_y = input.controlled_champion_position.y - input.enemy_position.y;
    let length = (to_controlled_champion_x * to_controlled_champion_x
        + to_controlled_champion_y * to_controlled_champion_y)
        .sqrt()
        .max(1e-6);
    let nx = to_controlled_champion_x / length;
    let ny = to_controlled_champion_y / length;
    let center = EnemyScriptPoint {
        x: input.enemy_position.x + nx * 180.0,
        y: input.enemy_position.y + ny * 180.0,
    };
    let tangent_x = -ny;
    let tangent_y = nx;
    let half_width = 180.0;
    let start = EnemyScriptPoint {
        x: center.x + tangent_x * half_width,
        y: center.y + tangent_y * half_width,
    };
    let end = EnemyScriptPoint {
        x: center.x - tangent_x * half_width,
        y: center.y - tangent_y * half_width,
    };
    vec![EnemyScriptAction::CreateProjectileBlockZone {
        start,
        end,
        duration_seconds: 4.0,
    }]
}

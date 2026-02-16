use super::{
    ChampionScriptAction, ChampionScriptEvent, ChampionScriptExecutionInput, ChampionScriptPoint,
    ChampionScriptSchedule,
};
use crate::defaults::simulator_defaults;

pub(crate) const CHAMPION_KEY: &str = "yasuo";

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    let defaults = &simulator_defaults().champion_script_defaults.yasuo;
    let schedule = defaults.wind_wall_schedule;
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::YasuoWindWall,
        start_offset_seconds: schedule.start_offset_seconds,
        interval_seconds: schedule.interval_seconds,
    }]
}

pub(crate) fn execute_wind_wall(input: ChampionScriptExecutionInput) -> Vec<ChampionScriptAction> {
    let defaults = &simulator_defaults().champion_script_defaults.yasuo;
    let to_target_x = input.target_position.x - input.actor_position.x;
    let to_target_y = input.target_position.y - input.actor_position.y;
    let length = (to_target_x * to_target_x + to_target_y * to_target_y)
        .sqrt()
        .max(1e-6);
    let nx = to_target_x / length;
    let ny = to_target_y / length;
    let center = ChampionScriptPoint {
        x: input.actor_position.x + nx * defaults.wind_wall_forward_offset,
        y: input.actor_position.y + ny * defaults.wind_wall_forward_offset,
    };
    let tangent_x = -ny;
    let tangent_y = nx;
    let half_width = defaults.wind_wall_half_length;
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
        half_width: defaults.wind_wall_block_half_width,
        duration_seconds: defaults.wind_wall_duration_seconds,
    }]
}

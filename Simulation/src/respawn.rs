#[derive(Debug, Clone, Copy)]
pub(crate) struct UrfRespawnTuning {
    pub urf_flat_reduction_seconds: f64,
    pub extrapolation_per_level: f64,
    pub time_scaling_enabled: bool,
    pub time_scaling_start_seconds: f64,
    pub time_scaling_per_minute_seconds: f64,
    pub time_scaling_cap_seconds: f64,
}

pub(crate) fn urf_respawn_delay_seconds(
    level: usize,
    game_time_seconds: f64,
    tuning: UrfRespawnTuning,
) -> f64 {
    // Baseline Summoner's Rift respawn waits by level (1-18), then extrapolated for URF levels.
    const BASE_BY_LEVEL: [f64; 18] = [
        10.0, 10.0, 12.0, 12.0, 14.0, 16.0, 20.0, 25.0, 28.0, 32.5, 35.0, 37.5, 40.0, 42.5, 45.0,
        47.5, 50.0, 52.5,
    ];
    let lvl = level.max(1);
    let base = if lvl <= BASE_BY_LEVEL.len() {
        BASE_BY_LEVEL[lvl - 1]
    } else {
        // Conservative smooth extrapolation for URF-only levels 19-30.
        let extra_levels = (lvl - BASE_BY_LEVEL.len()) as f64;
        BASE_BY_LEVEL[BASE_BY_LEVEL.len() - 1]
            + tuning.extrapolation_per_level.max(0.0) * extra_levels
    };
    // URF modifier from Riot patch history: reduce death timers by 3s at all levels.
    let mut delay = base - tuning.urf_flat_reduction_seconds.max(0.0);
    if tuning.time_scaling_enabled && game_time_seconds > tuning.time_scaling_start_seconds {
        let elapsed_after_start_minutes =
            (game_time_seconds - tuning.time_scaling_start_seconds).max(0.0) / 60.0;
        let time_bonus = (elapsed_after_start_minutes
            * tuning.time_scaling_per_minute_seconds.max(0.0))
        .min(tuning.time_scaling_cap_seconds.max(0.0));
        delay += time_bonus;
    }
    delay.max(1.0)
}

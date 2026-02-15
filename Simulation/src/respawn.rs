pub(crate) fn urf_respawn_delay_seconds(
    level: usize,
    urf_flat_reduction_seconds: f64,
    extrapolation_per_level: f64,
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
        BASE_BY_LEVEL[BASE_BY_LEVEL.len() - 1] + extrapolation_per_level.max(0.0) * extra_levels
    };
    // URF modifier from Riot patch history: reduce death timers by 3s at all levels.
    (base - urf_flat_reduction_seconds.max(0.0)).max(1.0)
}

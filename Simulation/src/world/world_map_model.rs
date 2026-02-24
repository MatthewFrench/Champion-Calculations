#[derive(Debug, Clone, Copy)]
pub(crate) struct WorldBounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl WorldBounds {
    pub(crate) fn contains(self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    pub(crate) fn clamp(self, x: f64, y: f64) -> (f64, f64) {
        (
            x.clamp(self.min_x, self.max_x),
            y.clamp(self.min_y, self.max_y),
        )
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WorldMapState {
    pub map_name: String,
    pub bounds: WorldBounds,
}

pub(crate) fn default_urf_world_map_state() -> WorldMapState {
    WorldMapState {
        map_name: "Summoner's Rift".to_string(),
        bounds: WorldBounds {
            min_x: -8000.0,
            max_x: 8000.0,
            min_y: -8000.0,
            max_y: 8000.0,
        },
    }
}

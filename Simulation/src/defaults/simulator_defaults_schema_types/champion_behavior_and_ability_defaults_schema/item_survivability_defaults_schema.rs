#[derive(Debug, Clone, Copy)]
pub(crate) struct ZhonyaTimeStopDefaults {
    pub duration_seconds: f64,
    pub cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct GuardianAngelRebirthDefaults {
    pub cooldown_seconds: f64,
    pub revive_duration_seconds: f64,
    pub revive_base_health_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProtoplasmLifelineDefaults {
    pub trigger_health_percent: f64,
    pub bonus_health_min: f64,
    pub bonus_health_max: f64,
    pub heal_total_min: f64,
    pub heal_total_max: f64,
    pub duration_seconds: f64,
}

use serde::Deserialize;

mod champion_ai_and_execution_schema;
mod champion_behavior_and_ability_defaults_schema;
mod champion_file_defaults_schema;
mod rune_runtime_defaults_schema;
mod simulation_search_and_engine_defaults_schema;

pub(crate) use self::champion_ai_and_execution_schema::*;
pub(crate) use self::champion_behavior_and_ability_defaults_schema::*;
pub(crate) use self::champion_file_defaults_schema::*;
pub(crate) use self::rune_runtime_defaults_schema::*;
pub(crate) use self::simulation_search_and_engine_defaults_schema::*;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SimulatorDefaults {
    pub simulation_defaults: SimulationDefaults,
    pub search_defaults: SearchDefaults,
    pub search_quality_profile_defaults: SearchQualityProfileDefaults,
    pub rune_runtime_defaults: RuneRuntimeDefaults,
    pub engine_defaults: EngineDefaults,
}

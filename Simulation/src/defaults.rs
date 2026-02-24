use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

mod champion_item_simulation_defaults_loader;
mod champion_simulation_data_loading;
mod defaults_path_key_and_effect_helpers;
mod simulator_defaults_schema_types;

pub(crate) use self::simulator_defaults_schema_types::*;

use self::champion_item_simulation_defaults_loader::{
    load_doctor_mundo_infected_bonesaw_ability_defaults, load_guardian_angel_rebirth_defaults,
    load_morgana_binding_and_soul_shackles_ability_defaults, load_protoplasm_lifeline_defaults,
    load_sona_crescendo_ability_defaults, load_vayne_silver_bolts_ability_defaults,
    load_vayne_tumble_ability_defaults, load_vladimir_cast_profile_defaults,
    load_vladimir_defensive_ability_two_policy_defaults, load_vladimir_offensive_ability_defaults,
    load_vladimir_sanguine_pool_defaults, load_warwick_eternal_hunger_passive_defaults,
    load_warwick_infinite_duress_ability_defaults, load_zhonya_time_stop_defaults,
};
use self::champion_simulation_data_loading::{
    champion_ability_execution_defaults_for_role_internal, load_champion_ability_execution_data,
    load_champion_ability_execution_defaults, load_champion_ai_profiles,
    load_champion_behavior_defaults, load_champion_simulation_data, load_champion_slot_bindings,
    load_urf_respawn_defaults,
};
use self::defaults_path_key_and_effect_helpers::{
    ability_effect_by_id, highest_rank_value, item_effects, load_defaults_from_disk,
    load_heartsteel_colossal_consumption_cooldown_seconds_default,
    load_luden_echo_cooldown_seconds_default, load_protoplasm_lifeline_cooldown_seconds_default,
    normalize_key, read_champion_file, read_item_file, repository_root_dir,
};

static SIMULATOR_DEFAULTS: OnceLock<SimulatorDefaults> = OnceLock::new();
static CHAMPION_SIMULATION_DATA: OnceLock<HashMap<String, ChampionSimulationData>> =
    OnceLock::new();
static CHAMPION_SLOT_BINDINGS: OnceLock<HashMap<String, HashMap<String, String>>> = OnceLock::new();
static CHAMPION_BEHAVIOR_DEFAULTS: OnceLock<ChampionBehaviorDefaults> = OnceLock::new();
static CHAMPION_ABILITY_EXECUTION_DEFAULTS: OnceLock<AbilityExecutionDefaultsByRole> =
    OnceLock::new();
#[allow(dead_code)]
static CHAMPION_ABILITY_EXECUTION_DATA: OnceLock<HashMap<String, ChampionAbilityExecutionData>> =
    OnceLock::new();
static CHAMPION_AI_PROFILES: OnceLock<ChampionAiProfilesFile> = OnceLock::new();
static URF_RESPAWN_DEFAULTS: OnceLock<UrfRespawnDefaults> = OnceLock::new();
static HEARTSTEEL_COLOSSAL_CONSUMPTION_COOLDOWN_SECONDS_DEFAULT: OnceLock<f64> = OnceLock::new();
static LUDEN_ECHO_COOLDOWN_SECONDS_DEFAULT: OnceLock<f64> = OnceLock::new();
static PROTOPLASM_LIFELINE_COOLDOWN_SECONDS_DEFAULT: OnceLock<f64> = OnceLock::new();
type CachedDefaultsLoad<T> = std::result::Result<T, String>;

static VLADIMIR_SANGUINE_POOL_DEFAULTS: OnceLock<CachedDefaultsLoad<VladimirSanguinePoolDefaults>> =
    OnceLock::new();
static VLADIMIR_DEFENSIVE_ABILITY_TWO_POLICY_DEFAULTS: OnceLock<
    CachedDefaultsLoad<VladimirDefensiveAbilityTwoPolicyDefaults>,
> = OnceLock::new();
static ZHONYA_TIME_STOP_DEFAULTS: OnceLock<ZhonyaTimeStopDefaults> = OnceLock::new();
static GUARDIAN_ANGEL_REBIRTH_DEFAULTS: OnceLock<GuardianAngelRebirthDefaults> = OnceLock::new();
static PROTOPLASM_LIFELINE_DEFAULTS: OnceLock<ProtoplasmLifelineDefaults> = OnceLock::new();
static DOCTOR_MUNDO_INFECTED_BONESAW_ABILITY_DEFAULTS: OnceLock<
    CachedDefaultsLoad<DoctorMundoInfectedBonesawAbilityDefaults>,
> = OnceLock::new();
static VLADIMIR_CAST_PROFILE_DEFAULTS: OnceLock<CachedDefaultsLoad<VladimirCastProfileDefaults>> =
    OnceLock::new();
static VLADIMIR_OFFENSIVE_ABILITY_DEFAULTS: OnceLock<
    CachedDefaultsLoad<VladimirOffensiveAbilityDefaults>,
> = OnceLock::new();
static WARWICK_INFINITE_DURESS_ABILITY_DEFAULTS: OnceLock<
    CachedDefaultsLoad<WarwickInfiniteDuressAbilityDefaults>,
> = OnceLock::new();
static WARWICK_ETERNAL_HUNGER_PASSIVE_DEFAULTS: OnceLock<
    CachedDefaultsLoad<WarwickEternalHungerPassiveDefaults>,
> = OnceLock::new();
static VAYNE_TUMBLE_ABILITY_DEFAULTS: OnceLock<CachedDefaultsLoad<VayneTumbleAbilityDefaults>> =
    OnceLock::new();
static VAYNE_SILVER_BOLTS_ABILITY_DEFAULTS: OnceLock<
    CachedDefaultsLoad<VayneSilverBoltsAbilityDefaults>,
> = OnceLock::new();
static MORGANA_BINDING_AND_SOUL_SHACKLES_ABILITY_DEFAULTS: OnceLock<
    CachedDefaultsLoad<MorganaBindingAndSoulShacklesAbilityDefaults>,
> = OnceLock::new();
static SONA_CRESCENDO_ABILITY_DEFAULTS: OnceLock<CachedDefaultsLoad<SonaCrescendoAbilityDefaults>> =
    OnceLock::new();

fn load_once_clone_or_none<T: Clone>(
    cache: &OnceLock<CachedDefaultsLoad<T>>,
    load: impl FnOnce() -> Result<T>,
) -> Option<T> {
    cache
        .get_or_init(|| load().map_err(|err| err.to_string()))
        .as_ref()
        .ok()
        .cloned()
}

pub(crate) fn simulator_defaults() -> &'static SimulatorDefaults {
    SIMULATOR_DEFAULTS
        .get_or_init(|| load_defaults_from_disk().unwrap_or_else(|err| panic!("{}", err)))
}

pub(crate) fn rune_runtime_defaults() -> &'static RuneRuntimeDefaults {
    &simulator_defaults().rune_runtime_defaults
}

fn champion_simulation_data_map() -> &'static HashMap<String, ChampionSimulationData> {
    CHAMPION_SIMULATION_DATA.get_or_init(|| load_champion_simulation_data().unwrap_or_default())
}

fn champion_simulation_data(champion_name: &str) -> Option<&'static ChampionSimulationData> {
    champion_simulation_data_map().get(&normalize_key(champion_name))
}

pub(crate) fn champion_behavior_defaults_for_role(is_melee: bool) -> ChampionBehaviorDefaultsEntry {
    let defaults = CHAMPION_BEHAVIOR_DEFAULTS
        .get_or_init(|| load_champion_behavior_defaults().unwrap_or_else(|err| panic!("{}", err)));
    if is_melee {
        defaults.melee
    } else {
        defaults.ranged
    }
}

#[allow(dead_code)]
pub(crate) fn default_ability_execution_profile(is_melee: bool) -> AbilityExecutionProfile {
    let defaults = champion_ability_execution_defaults_for_role_internal(is_melee);
    AbilityExecutionProfile {
        cast_windup_seconds: defaults.cast_windup_seconds,
        projectile_speed: defaults.projectile_speed,
        effect_hitbox_radius: defaults.effect_hitbox_radius,
    }
}

#[allow(dead_code)]
pub(crate) fn champion_ability_execution_profile(
    champion_name: &str,
    ability_key: &str,
    is_melee: bool,
) -> Option<AbilityExecutionProfile> {
    let execution_data = CHAMPION_ABILITY_EXECUTION_DATA
        .get_or_init(|| load_champion_ability_execution_data().unwrap_or_default());
    let champion_execution_data = execution_data.get(&normalize_key(champion_name))?;
    let role_defaults = champion_ability_execution_defaults_for_role_internal(
        champion_execution_data.is_melee || is_melee,
    );
    let execution = champion_execution_data
        .abilities
        .get(ability_key)
        .copied()
        .unwrap_or_default();
    Some(AbilityExecutionProfile {
        cast_windup_seconds: execution
            .cast_windup_seconds
            .unwrap_or(role_defaults.cast_windup_seconds),
        projectile_speed: execution
            .projectile_speed
            .unwrap_or(role_defaults.projectile_speed),
        effect_hitbox_radius: execution
            .effect_hitbox_radius
            .unwrap_or(role_defaults.effect_hitbox_radius),
    })
}

pub(crate) fn urf_respawn_defaults() -> &'static UrfRespawnDefaults {
    URF_RESPAWN_DEFAULTS
        .get_or_init(|| load_urf_respawn_defaults().unwrap_or_else(|err| panic!("{}", err)))
}

pub(crate) fn heartsteel_colossal_consumption_cooldown_seconds_default() -> f64 {
    *HEARTSTEEL_COLOSSAL_CONSUMPTION_COOLDOWN_SECONDS_DEFAULT.get_or_init(|| {
        load_heartsteel_colossal_consumption_cooldown_seconds_default()
            .unwrap_or_else(|err| panic!("{}", err))
    })
}

pub(crate) fn luden_echo_cooldown_seconds_default() -> f64 {
    *LUDEN_ECHO_COOLDOWN_SECONDS_DEFAULT.get_or_init(|| {
        load_luden_echo_cooldown_seconds_default().unwrap_or_else(|err| panic!("{}", err))
    })
}

pub(crate) fn protoplasm_lifeline_cooldown_seconds_default() -> f64 {
    *PROTOPLASM_LIFELINE_COOLDOWN_SECONDS_DEFAULT.get_or_init(|| {
        load_protoplasm_lifeline_cooldown_seconds_default().unwrap_or_else(|err| panic!("{}", err))
    })
}

pub(crate) fn vladimir_sanguine_pool_defaults(
    champion_name: &str,
) -> Option<VladimirSanguinePoolDefaults> {
    if normalize_key(champion_name) != "vladimir" {
        return None;
    }
    load_once_clone_or_none(
        &VLADIMIR_SANGUINE_POOL_DEFAULTS,
        load_vladimir_sanguine_pool_defaults,
    )
}

pub(crate) fn vladimir_defensive_ability_two_policy_defaults(
    champion_name: &str,
) -> Option<VladimirDefensiveAbilityTwoPolicyDefaults> {
    if normalize_key(champion_name) != "vladimir" {
        return None;
    }
    load_once_clone_or_none(
        &VLADIMIR_DEFENSIVE_ABILITY_TWO_POLICY_DEFAULTS,
        load_vladimir_defensive_ability_two_policy_defaults,
    )
}

pub(crate) fn zhonya_time_stop_defaults() -> &'static ZhonyaTimeStopDefaults {
    ZHONYA_TIME_STOP_DEFAULTS
        .get_or_init(|| load_zhonya_time_stop_defaults().unwrap_or_else(|err| panic!("{}", err)))
}

pub(crate) fn guardian_angel_rebirth_defaults() -> &'static GuardianAngelRebirthDefaults {
    GUARDIAN_ANGEL_REBIRTH_DEFAULTS.get_or_init(|| {
        load_guardian_angel_rebirth_defaults().unwrap_or_else(|err| panic!("{}", err))
    })
}

pub(crate) fn protoplasm_lifeline_defaults() -> &'static ProtoplasmLifelineDefaults {
    PROTOPLASM_LIFELINE_DEFAULTS
        .get_or_init(|| load_protoplasm_lifeline_defaults().unwrap_or_else(|err| panic!("{}", err)))
}

pub(crate) fn controlled_champion_stasis_trigger_health_percent_default() -> f64 {
    let defaults = champion_ai_profiles().controlled_champion_defaults;
    defaults.stasis_trigger_health_percent
}

pub(crate) fn doctor_mundo_infected_bonesaw_ability_defaults(
    champion_name: &str,
) -> Option<DoctorMundoInfectedBonesawAbilityDefaults> {
    if normalize_key(champion_name) != "drmundo" {
        return None;
    }
    load_once_clone_or_none(
        &DOCTOR_MUNDO_INFECTED_BONESAW_ABILITY_DEFAULTS,
        load_doctor_mundo_infected_bonesaw_ability_defaults,
    )
}

pub(crate) fn champion_slot_bindings(champion_name: &str) -> HashMap<String, String> {
    CHAMPION_SLOT_BINDINGS
        .get_or_init(|| load_champion_slot_bindings().unwrap_or_default())
        .get(&normalize_key(champion_name))
        .cloned()
        .unwrap_or_default()
}

pub(crate) fn champion_behavior_override(
    champion_name: &str,
) -> Option<ChampionBehaviorOverrideEntry> {
    champion_simulation_data(champion_name).and_then(|profile| profile.behavior)
}

fn champion_ai_profiles() -> &'static ChampionAiProfilesFile {
    CHAMPION_AI_PROFILES
        .get_or_init(|| load_champion_ai_profiles().unwrap_or_else(|err| panic!("{}", err)))
}

pub(crate) fn champion_ai_profile(
    champion_name: &str,
    base_attack_range: f64,
) -> ChampionAiProfile {
    let defaults = champion_ai_profiles().defaults;
    let override_entry = champion_ai_profiles()
        .champions
        .get(&normalize_key(champion_name));
    let desired_combat_range = override_entry
        .and_then(|entry| entry.desired_combat_range)
        .unwrap_or_else(|| base_attack_range.max(75.0));
    let movement_speed_scale = override_entry
        .and_then(|entry| entry.movement_speed_scale)
        .unwrap_or(defaults.movement_speed_scale);
    let script_poll_interval_seconds = override_entry
        .and_then(|entry| entry.script_poll_interval_seconds)
        .unwrap_or(defaults.script_poll_interval_seconds);
    ChampionAiProfile {
        desired_combat_range,
        movement_speed_scale,
        script_poll_interval_seconds,
    }
}

pub(crate) fn champion_ai_script_priority_override(
    champion_name: &str,
    script_event_key: &str,
) -> Option<i32> {
    let key = normalize_key(champion_name);
    let event_key = normalize_key(script_event_key);
    champion_ai_profiles()
        .champions
        .get(&key)
        .and_then(|entry| entry.script_priority_overrides.get(&event_key))
        .copied()
}

pub(crate) fn vladimir_cast_profile_defaults(
    champion_name: &str,
) -> Option<VladimirCastProfileDefaults> {
    if normalize_key(champion_name) != "vladimir" {
        return None;
    }
    load_once_clone_or_none(
        &VLADIMIR_CAST_PROFILE_DEFAULTS,
        load_vladimir_cast_profile_defaults,
    )
}

pub(crate) fn vladimir_offensive_ability_defaults(
    champion_name: &str,
) -> Option<VladimirOffensiveAbilityDefaults> {
    if normalize_key(champion_name) != "vladimir" {
        return None;
    }
    load_once_clone_or_none(
        &VLADIMIR_OFFENSIVE_ABILITY_DEFAULTS,
        load_vladimir_offensive_ability_defaults,
    )
}

pub(crate) fn warwick_infinite_duress_ability_defaults(
    champion_name: &str,
) -> Option<WarwickInfiniteDuressAbilityDefaults> {
    if normalize_key(champion_name) != "warwick" {
        return None;
    }
    load_once_clone_or_none(
        &WARWICK_INFINITE_DURESS_ABILITY_DEFAULTS,
        load_warwick_infinite_duress_ability_defaults,
    )
}

pub(crate) fn warwick_eternal_hunger_passive_defaults(
    champion_name: &str,
) -> Option<WarwickEternalHungerPassiveDefaults> {
    if normalize_key(champion_name) != "warwick" {
        return None;
    }
    load_once_clone_or_none(
        &WARWICK_ETERNAL_HUNGER_PASSIVE_DEFAULTS,
        load_warwick_eternal_hunger_passive_defaults,
    )
}

pub(crate) fn vayne_tumble_ability_defaults(
    champion_name: &str,
) -> Option<VayneTumbleAbilityDefaults> {
    if normalize_key(champion_name) != "vayne" {
        return None;
    }
    load_once_clone_or_none(
        &VAYNE_TUMBLE_ABILITY_DEFAULTS,
        load_vayne_tumble_ability_defaults,
    )
}

pub(crate) fn vayne_silver_bolts_ability_defaults(
    champion_name: &str,
) -> Option<VayneSilverBoltsAbilityDefaults> {
    if normalize_key(champion_name) != "vayne" {
        return None;
    }
    load_once_clone_or_none(
        &VAYNE_SILVER_BOLTS_ABILITY_DEFAULTS,
        load_vayne_silver_bolts_ability_defaults,
    )
}

pub(crate) fn morgana_binding_and_soul_shackles_ability_defaults(
    champion_name: &str,
) -> Option<MorganaBindingAndSoulShacklesAbilityDefaults> {
    if normalize_key(champion_name) != "morgana" {
        return None;
    }
    load_once_clone_or_none(
        &MORGANA_BINDING_AND_SOUL_SHACKLES_ABILITY_DEFAULTS,
        load_morgana_binding_and_soul_shackles_ability_defaults,
    )
}

pub(crate) fn sona_crescendo_ability_defaults(
    champion_name: &str,
) -> Option<SonaCrescendoAbilityDefaults> {
    if normalize_key(champion_name) != "sona" {
        return None;
    }
    load_once_clone_or_none(
        &SONA_CRESCENDO_ABILITY_DEFAULTS,
        load_sona_crescendo_ability_defaults,
    )
}

pub(crate) fn champion_hitbox_radius(champion_name: &str) -> f64 {
    let defaults = simulator_defaults();
    let key = normalize_key(champion_name);
    defaults
        .engine_defaults
        .champion_hitbox_radius_overrides
        .get(&key)
        .copied()
        .unwrap_or(defaults.engine_defaults.default_champion_hitbox_radius)
}

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

type CachedDefaultsLoad<T> = std::result::Result<T, String>;

static SIMULATOR_DEFAULTS: OnceLock<CachedDefaultsLoad<SimulatorDefaults>> = OnceLock::new();
static CHAMPION_SIMULATION_DATA: OnceLock<
    CachedDefaultsLoad<HashMap<String, ChampionSimulationData>>,
> = OnceLock::new();
static CHAMPION_SLOT_BINDINGS: OnceLock<
    CachedDefaultsLoad<HashMap<String, HashMap<String, String>>>,
> = OnceLock::new();
static CHAMPION_BEHAVIOR_DEFAULTS: OnceLock<CachedDefaultsLoad<ChampionBehaviorDefaults>> =
    OnceLock::new();
static CHAMPION_ABILITY_EXECUTION_DEFAULTS: OnceLock<
    CachedDefaultsLoad<AbilityExecutionDefaultsByRole>,
> = OnceLock::new();
#[allow(dead_code)]
static CHAMPION_ABILITY_EXECUTION_DATA: OnceLock<
    CachedDefaultsLoad<HashMap<String, ChampionAbilityExecutionData>>,
> = OnceLock::new();
static CHAMPION_AI_PROFILES: OnceLock<CachedDefaultsLoad<ChampionAiProfilesFile>> = OnceLock::new();
static URF_RESPAWN_DEFAULTS: OnceLock<CachedDefaultsLoad<UrfRespawnDefaults>> = OnceLock::new();
static HEARTSTEEL_COLOSSAL_CONSUMPTION_COOLDOWN_SECONDS_DEFAULT: OnceLock<CachedDefaultsLoad<f64>> =
    OnceLock::new();
static LUDEN_ECHO_COOLDOWN_SECONDS_DEFAULT: OnceLock<CachedDefaultsLoad<f64>> = OnceLock::new();
static PROTOPLASM_LIFELINE_COOLDOWN_SECONDS_DEFAULT: OnceLock<CachedDefaultsLoad<f64>> =
    OnceLock::new();

static VLADIMIR_SANGUINE_POOL_DEFAULTS: OnceLock<CachedDefaultsLoad<VladimirSanguinePoolDefaults>> =
    OnceLock::new();
static VLADIMIR_DEFENSIVE_ABILITY_TWO_POLICY_DEFAULTS: OnceLock<
    CachedDefaultsLoad<VladimirDefensiveAbilityTwoPolicyDefaults>,
> = OnceLock::new();
static ZHONYA_TIME_STOP_DEFAULTS: OnceLock<CachedDefaultsLoad<ZhonyaTimeStopDefaults>> =
    OnceLock::new();
static GUARDIAN_ANGEL_REBIRTH_DEFAULTS: OnceLock<CachedDefaultsLoad<GuardianAngelRebirthDefaults>> =
    OnceLock::new();
static PROTOPLASM_LIFELINE_DEFAULTS: OnceLock<CachedDefaultsLoad<ProtoplasmLifelineDefaults>> =
    OnceLock::new();
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

// Required defaults channels are process-fatal if unavailable. This keeps runtime behavior
// strict and deterministic while avoiding scattered panic callsites.
fn hard_fail_required_defaults_channel(channel: &str, err: &str) -> ! {
    eprintln!(
        "fatal defaults load failure in required channel `{}`: {}",
        channel, err
    );
    std::process::exit(2);
}

fn load_once_ref_or_hard_fail<T>(
    cache: &'static OnceLock<CachedDefaultsLoad<T>>,
    channel: &str,
    load: impl FnOnce() -> Result<T>,
) -> &'static T {
    match cache.get_or_init(|| load().map_err(|err| err.to_string())) {
        Ok(value) => value,
        Err(err) => hard_fail_required_defaults_channel(channel, err),
    }
}

fn load_once_copy_or_hard_fail<T: Copy>(
    cache: &'static OnceLock<CachedDefaultsLoad<T>>,
    channel: &str,
    load: impl FnOnce() -> Result<T>,
) -> T {
    match cache.get_or_init(|| load().map_err(|err| err.to_string())) {
        Ok(value) => *value,
        Err(err) => hard_fail_required_defaults_channel(channel, err),
    }
}

fn load_once_clone_or_none<T: Clone>(
    cache: &'static OnceLock<CachedDefaultsLoad<T>>,
    load: impl FnOnce() -> Result<T>,
) -> Option<T> {
    cache
        .get_or_init(|| load().map_err(|err| err.to_string()))
        .as_ref()
        .ok()
        .cloned()
}

fn preload_required_defaults_channel<T>(
    cache: &'static OnceLock<CachedDefaultsLoad<T>>,
    channel: &str,
    load: impl FnOnce() -> Result<T>,
) -> Result<()> {
    match cache.get_or_init(|| load().map_err(|err| err.to_string())) {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow!(
            "required defaults channel `{}` failed to load: {}",
            channel,
            err
        )),
    }
}

// Preload required defaults at startup to surface typed failures before run dispatch.
pub(crate) fn preflight_required_defaults_channels() -> Result<()> {
    preload_required_defaults_channel(
        &SIMULATOR_DEFAULTS,
        "simulator_defaults",
        load_defaults_from_disk,
    )?;
    preload_required_defaults_channel(
        &CHAMPION_SIMULATION_DATA,
        "champion_simulation_data",
        load_champion_simulation_data,
    )?;
    preload_required_defaults_channel(
        &CHAMPION_SLOT_BINDINGS,
        "champion_slot_bindings",
        load_champion_slot_bindings,
    )?;
    preload_required_defaults_channel(
        &CHAMPION_BEHAVIOR_DEFAULTS,
        "champion_behavior_defaults",
        load_champion_behavior_defaults,
    )?;
    preload_required_defaults_channel(
        &CHAMPION_ABILITY_EXECUTION_DEFAULTS,
        "champion_ability_execution_defaults",
        load_champion_ability_execution_defaults,
    )?;
    preload_required_defaults_channel(
        &CHAMPION_ABILITY_EXECUTION_DATA,
        "champion_ability_execution_data",
        load_champion_ability_execution_data,
    )?;
    preload_required_defaults_channel(
        &CHAMPION_AI_PROFILES,
        "champion_ai_profiles",
        load_champion_ai_profiles,
    )?;
    preload_required_defaults_channel(
        &URF_RESPAWN_DEFAULTS,
        "urf_respawn_defaults",
        load_urf_respawn_defaults,
    )?;
    preload_required_defaults_channel(
        &HEARTSTEEL_COLOSSAL_CONSUMPTION_COOLDOWN_SECONDS_DEFAULT,
        "heartsteel_colossal_consumption_cooldown_seconds_default",
        load_heartsteel_colossal_consumption_cooldown_seconds_default,
    )?;
    preload_required_defaults_channel(
        &LUDEN_ECHO_COOLDOWN_SECONDS_DEFAULT,
        "luden_echo_cooldown_seconds_default",
        load_luden_echo_cooldown_seconds_default,
    )?;
    preload_required_defaults_channel(
        &PROTOPLASM_LIFELINE_COOLDOWN_SECONDS_DEFAULT,
        "protoplasm_lifeline_cooldown_seconds_default",
        load_protoplasm_lifeline_cooldown_seconds_default,
    )?;
    preload_required_defaults_channel(
        &ZHONYA_TIME_STOP_DEFAULTS,
        "zhonya_time_stop_defaults",
        load_zhonya_time_stop_defaults,
    )?;
    preload_required_defaults_channel(
        &GUARDIAN_ANGEL_REBIRTH_DEFAULTS,
        "guardian_angel_rebirth_defaults",
        load_guardian_angel_rebirth_defaults,
    )?;
    preload_required_defaults_channel(
        &PROTOPLASM_LIFELINE_DEFAULTS,
        "protoplasm_lifeline_defaults",
        load_protoplasm_lifeline_defaults,
    )?;
    Ok(())
}

pub(crate) fn simulator_defaults() -> &'static SimulatorDefaults {
    load_once_ref_or_hard_fail(
        &SIMULATOR_DEFAULTS,
        "simulator_defaults",
        load_defaults_from_disk,
    )
}

pub(crate) fn rune_runtime_defaults() -> &'static RuneRuntimeDefaults {
    &simulator_defaults().rune_runtime_defaults
}

fn champion_simulation_data_map() -> &'static HashMap<String, ChampionSimulationData> {
    load_once_ref_or_hard_fail(
        &CHAMPION_SIMULATION_DATA,
        "champion_simulation_data",
        load_champion_simulation_data,
    )
}

fn champion_simulation_data(champion_name: &str) -> Option<&'static ChampionSimulationData> {
    champion_simulation_data_map().get(&normalize_key(champion_name))
}

pub(crate) fn champion_behavior_defaults_for_role(is_melee: bool) -> ChampionBehaviorDefaultsEntry {
    let defaults = load_once_ref_or_hard_fail(
        &CHAMPION_BEHAVIOR_DEFAULTS,
        "champion_behavior_defaults",
        load_champion_behavior_defaults,
    );
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
    let execution_data = load_once_ref_or_hard_fail(
        &CHAMPION_ABILITY_EXECUTION_DATA,
        "champion_ability_execution_data",
        load_champion_ability_execution_data,
    );
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
    load_once_ref_or_hard_fail(
        &URF_RESPAWN_DEFAULTS,
        "urf_respawn_defaults",
        load_urf_respawn_defaults,
    )
}

pub(crate) fn heartsteel_colossal_consumption_cooldown_seconds_default() -> f64 {
    load_once_copy_or_hard_fail(
        &HEARTSTEEL_COLOSSAL_CONSUMPTION_COOLDOWN_SECONDS_DEFAULT,
        "heartsteel_colossal_consumption_cooldown_seconds_default",
        load_heartsteel_colossal_consumption_cooldown_seconds_default,
    )
}

pub(crate) fn luden_echo_cooldown_seconds_default() -> f64 {
    load_once_copy_or_hard_fail(
        &LUDEN_ECHO_COOLDOWN_SECONDS_DEFAULT,
        "luden_echo_cooldown_seconds_default",
        load_luden_echo_cooldown_seconds_default,
    )
}

pub(crate) fn protoplasm_lifeline_cooldown_seconds_default() -> f64 {
    load_once_copy_or_hard_fail(
        &PROTOPLASM_LIFELINE_COOLDOWN_SECONDS_DEFAULT,
        "protoplasm_lifeline_cooldown_seconds_default",
        load_protoplasm_lifeline_cooldown_seconds_default,
    )
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
    load_once_ref_or_hard_fail(
        &ZHONYA_TIME_STOP_DEFAULTS,
        "zhonya_time_stop_defaults",
        load_zhonya_time_stop_defaults,
    )
}

pub(crate) fn guardian_angel_rebirth_defaults() -> &'static GuardianAngelRebirthDefaults {
    load_once_ref_or_hard_fail(
        &GUARDIAN_ANGEL_REBIRTH_DEFAULTS,
        "guardian_angel_rebirth_defaults",
        load_guardian_angel_rebirth_defaults,
    )
}

pub(crate) fn protoplasm_lifeline_defaults() -> &'static ProtoplasmLifelineDefaults {
    load_once_ref_or_hard_fail(
        &PROTOPLASM_LIFELINE_DEFAULTS,
        "protoplasm_lifeline_defaults",
        load_protoplasm_lifeline_defaults,
    )
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
    load_once_ref_or_hard_fail(
        &CHAMPION_SLOT_BINDINGS,
        "champion_slot_bindings",
        load_champion_slot_bindings,
    )
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
    load_once_ref_or_hard_fail(
        &CHAMPION_AI_PROFILES,
        "champion_ai_profiles",
        load_champion_ai_profiles,
    )
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

pub(crate) fn world_lifecycle_defaults() -> WorldLifecycleDefaults {
    simulator_defaults().engine_defaults.world_lifecycle
}

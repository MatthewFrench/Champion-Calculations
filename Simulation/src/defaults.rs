use anyhow::{Context, Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

mod champion_item_simulation_defaults_loader;
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
use self::defaults_path_key_and_effect_helpers::{
    ability_effect_by_id, highest_rank_value, is_character_support_file, item_effects,
    load_defaults_from_disk, load_heartsteel_colossal_consumption_cooldown_seconds_default,
    load_luden_echo_cooldown_seconds_default, load_protoplasm_lifeline_cooldown_seconds_default,
    normalize_key, normalize_snake_key, read_champion_file, read_item_file, repository_root_dir,
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
static VLADIMIR_SANGUINE_POOL_DEFAULTS: OnceLock<VladimirSanguinePoolDefaults> = OnceLock::new();
static VLADIMIR_DEFENSIVE_ABILITY_TWO_POLICY_DEFAULTS: OnceLock<
    VladimirDefensiveAbilityTwoPolicyDefaults,
> = OnceLock::new();
static ZHONYA_TIME_STOP_DEFAULTS: OnceLock<ZhonyaTimeStopDefaults> = OnceLock::new();
static GUARDIAN_ANGEL_REBIRTH_DEFAULTS: OnceLock<GuardianAngelRebirthDefaults> = OnceLock::new();
static PROTOPLASM_LIFELINE_DEFAULTS: OnceLock<ProtoplasmLifelineDefaults> = OnceLock::new();
static DOCTOR_MUNDO_INFECTED_BONESAW_ABILITY_DEFAULTS: OnceLock<
    DoctorMundoInfectedBonesawAbilityDefaults,
> = OnceLock::new();
static VLADIMIR_CAST_PROFILE_DEFAULTS: OnceLock<VladimirCastProfileDefaults> = OnceLock::new();
static VLADIMIR_OFFENSIVE_ABILITY_DEFAULTS: OnceLock<VladimirOffensiveAbilityDefaults> =
    OnceLock::new();
static WARWICK_INFINITE_DURESS_ABILITY_DEFAULTS: OnceLock<WarwickInfiniteDuressAbilityDefaults> =
    OnceLock::new();
static WARWICK_ETERNAL_HUNGER_PASSIVE_DEFAULTS: OnceLock<WarwickEternalHungerPassiveDefaults> =
    OnceLock::new();
static VAYNE_TUMBLE_ABILITY_DEFAULTS: OnceLock<VayneTumbleAbilityDefaults> = OnceLock::new();
static VAYNE_SILVER_BOLTS_ABILITY_DEFAULTS: OnceLock<VayneSilverBoltsAbilityDefaults> =
    OnceLock::new();
static MORGANA_BINDING_AND_SOUL_SHACKLES_ABILITY_DEFAULTS: OnceLock<
    MorganaBindingAndSoulShacklesAbilityDefaults,
> = OnceLock::new();
static SONA_CRESCENDO_ABILITY_DEFAULTS: OnceLock<SonaCrescendoAbilityDefaults> = OnceLock::new();

fn load_champion_simulation_data() -> Result<HashMap<String, ChampionSimulationData>> {
    let mut profiles = HashMap::new();
    let characters_dir = repository_root_dir().join("Characters");
    for entry in std::fs::read_dir(&characters_dir)
        .with_context(|| format!("Failed reading {}", characters_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid champion filename: {}", path.display()))?;
        if is_character_support_file(stem) {
            continue;
        }

        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
        let champion_file: ChampionFileEnvelope = serde_json::from_str(&text)
            .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;

        let ChampionFileEnvelope { name, behavior } = champion_file;

        if behavior.is_none() {
            continue;
        }

        let profile = ChampionSimulationData { behavior };

        profiles.insert(normalize_key(stem), profile.clone());
        if !name.trim().is_empty() {
            profiles.insert(normalize_key(&name), profile);
        }
    }
    Ok(profiles)
}

fn derive_ability_identifier(
    champion_name: &str,
    ability_key: &str,
    ability_data: &Value,
) -> String {
    if let Some(explicit_identifier) = ability_data
        .get("ability_id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|identifier| !identifier.is_empty())
    {
        return explicit_identifier.to_string();
    }
    let champion_prefix = normalize_key(champion_name);
    let ability_name = ability_data
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(ability_key);
    let mut ability_suffix = normalize_snake_key(ability_name);
    if ability_suffix.is_empty() {
        ability_suffix = normalize_snake_key(ability_key);
    }
    if ability_suffix.is_empty() {
        champion_prefix
    } else {
        format!("{}_{}", champion_prefix, ability_suffix)
    }
}

fn load_champion_slot_bindings() -> Result<HashMap<String, HashMap<String, String>>> {
    let mut bindings_by_champion = HashMap::new();
    let characters_dir = repository_root_dir().join("Characters");
    for entry in std::fs::read_dir(&characters_dir)
        .with_context(|| format!("Failed reading {}", characters_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid champion filename: {}", path.display()))?;
        if is_character_support_file(stem) {
            continue;
        }

        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
        let champion_data: Value = serde_json::from_str(&text)
            .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;

        let champion_name = champion_data
            .get("name")
            .and_then(Value::as_str)
            .filter(|name| !name.trim().is_empty())
            .unwrap_or(stem);
        let mut slot_bindings = HashMap::new();
        if let Some(abilities) = champion_data.get("abilities").and_then(Value::as_object) {
            for (ability_key, ability_data) in abilities {
                let slot = ability_data
                    .get("slot")
                    .and_then(Value::as_str)
                    .or_else(|| {
                        ability_data
                            .get("default_keybinding")
                            .and_then(Value::as_str)
                    })
                    .map(str::trim)
                    .filter(|slot| !slot.is_empty())
                    .map(|slot| slot.to_ascii_uppercase());
                let Some(slot) = slot else {
                    continue;
                };
                let ability_identifier =
                    derive_ability_identifier(champion_name, ability_key, ability_data);
                slot_bindings.insert(slot, ability_identifier);
            }
        }
        if slot_bindings.is_empty() {
            continue;
        }

        bindings_by_champion.insert(normalize_key(stem), slot_bindings.clone());
        bindings_by_champion.insert(normalize_key(champion_name), slot_bindings);
    }
    Ok(bindings_by_champion)
}

fn load_champion_behavior_defaults() -> Result<ChampionBehaviorDefaults> {
    let path = repository_root_dir()
        .join("Characters")
        .join("ChampionDefaults.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion defaults file: {}", path.display()))?;
    let defaults_file: ChampionDefaultsFile = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion defaults file: {}", path.display()))?;
    let to_entry =
        |base_stats: ChampionBaseStatsDefaultsEntry,
         basic_attack: ChampionBasicAttackDefaultsEntry,
         behavior: ChampionBehaviorModifiersEntry| ChampionBehaviorDefaultsEntry {
            attack_range: base_stats.attack_range,
            attack_windup_seconds: basic_attack.base_windup_seconds,
            attack_projectile_speed: basic_attack.missile_speed,
            attack_effect_hitbox_radius: basic_attack.raw_timing_stats.gameplay_radius,
            on_hit_magic_flat: behavior.on_hit.magic_flat,
            on_hit_magic_ad_ratio: behavior.on_hit.magic_ad_ratio,
            periodic_true_hit_every: behavior.periodic_true_hit.every,
            periodic_true_hit_base: behavior.periodic_true_hit.base,
            periodic_true_hit_ad_ratio: behavior.periodic_true_hit.ad_ratio,
            periodic_true_hit_target_max_health_ratio: behavior
                .periodic_true_hit
                .target_max_health_ratio,
        };
    Ok(ChampionBehaviorDefaults {
        melee: to_entry(
            defaults_file.base_stats.melee,
            defaults_file.basic_attack.melee,
            defaults_file.behavior.melee,
        ),
        ranged: to_entry(
            defaults_file.base_stats.ranged,
            defaults_file.basic_attack.ranged,
            defaults_file.behavior.ranged,
        ),
    })
}

fn load_champion_ability_execution_defaults() -> Result<AbilityExecutionDefaultsByRole> {
    let path = repository_root_dir()
        .join("Characters")
        .join("ChampionDefaults.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion defaults file: {}", path.display()))?;
    let defaults_file: ChampionDefaultsFile = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion defaults file: {}", path.display()))?;
    Ok(defaults_file.abilities.execution_defaults)
}

fn load_champion_ai_profiles() -> Result<ChampionAiProfilesFile> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("champion_ai_profiles.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion AI profiles: {}", path.display()))?;
    let mut profiles: ChampionAiProfilesFile = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion AI profiles: {}", path.display()))?;
    let mut normalized = HashMap::new();
    for (champion_key, mut entry) in profiles.champions {
        let mut normalized_script_priority_overrides = HashMap::new();
        for (event_key, priority) in entry.script_priority_overrides {
            normalized_script_priority_overrides.insert(normalize_key(&event_key), priority);
        }
        entry.script_priority_overrides = normalized_script_priority_overrides;
        normalized.insert(normalize_key(&champion_key), entry);
    }
    profiles.champions = normalized;
    Ok(profiles)
}

#[allow(dead_code)]
fn load_champion_ability_execution_data() -> Result<HashMap<String, ChampionAbilityExecutionData>> {
    let mut data = HashMap::new();
    let characters_dir = repository_root_dir().join("Characters");
    for entry in std::fs::read_dir(&characters_dir)
        .with_context(|| format!("Failed reading {}", characters_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid champion filename: {}", path.display()))?;
        if is_character_support_file(stem) {
            continue;
        }

        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
        let champion_data: Value = serde_json::from_str(&text)
            .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;

        let mut abilities = HashMap::new();
        if let Some(ability_object) = champion_data.get("abilities").and_then(Value::as_object) {
            for (ability_key, ability) in ability_object {
                let execution = ability
                    .get("execution")
                    .cloned()
                    .and_then(|value| {
                        serde_json::from_value::<AbilityExecutionOverrideEntry>(value).ok()
                    })
                    .unwrap_or_default();
                if execution.cast_windup_seconds.is_some()
                    || execution.projectile_speed.is_some()
                    || execution.effect_hitbox_radius.is_some()
                {
                    abilities.insert(ability_key.clone(), execution);
                }
            }
        }

        let profile = ChampionAbilityExecutionData {
            is_melee: champion_is_melee_from_data(&champion_data),
            abilities,
        };
        data.insert(normalize_key(stem), profile.clone());
        if let Some(name) = champion_data.get("name").and_then(Value::as_str)
            && !name.trim().is_empty()
        {
            data.insert(normalize_key(name), profile);
        }
    }

    Ok(data)
}

fn load_urf_respawn_defaults() -> Result<UrfRespawnDefaults> {
    let path = repository_root_dir().join("Game Mode").join("URF.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading URF mode file: {}", path.display()))?;
    let urf: UrfFileEnvelope = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing URF mode file: {}", path.display()))?;
    let respawn = urf
        .respawn
        .ok_or_else(|| anyhow!("Missing respawn in {}", path.display()))?;
    Ok(respawn)
}

fn champion_is_melee_from_data(champion_data: &Value) -> bool {
    if let Some(attack_type) = champion_data
        .get("basic_attack")
        .and_then(|basic| basic.get("attack_type"))
        .and_then(Value::as_str)
    {
        return attack_type.eq_ignore_ascii_case("melee");
    }
    champion_data
        .pointer("/base_stats/attack_range")
        .and_then(Value::as_f64)
        .map(|range| range <= 200.0)
        .unwrap_or(false)
}

fn champion_ability_execution_defaults_for_role_internal(
    is_melee: bool,
) -> AbilityExecutionDefaultsEntry {
    let defaults = CHAMPION_ABILITY_EXECUTION_DEFAULTS.get_or_init(|| {
        load_champion_ability_execution_defaults().unwrap_or_else(|err| panic!("{}", err))
    });
    if is_melee {
        defaults.melee
    } else {
        defaults.ranged
    }
}

pub(crate) fn simulator_defaults() -> &'static SimulatorDefaults {
    SIMULATOR_DEFAULTS
        .get_or_init(|| load_defaults_from_disk().unwrap_or_else(|err| panic!("{}", err)))
}

pub(crate) fn rune_runtime_defaults() -> &'static RuneRuntimeDefaults {
    &simulator_defaults().rune_runtime_defaults
}

fn champion_simulation_data_map() -> &'static HashMap<String, ChampionSimulationData> {
    CHAMPION_SIMULATION_DATA
        .get_or_init(|| load_champion_simulation_data().unwrap_or_else(|err| panic!("{}", err)))
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
    let execution_data = CHAMPION_ABILITY_EXECUTION_DATA.get_or_init(|| {
        load_champion_ability_execution_data().unwrap_or_else(|err| panic!("{}", err))
    });
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
    Some(
        VLADIMIR_SANGUINE_POOL_DEFAULTS
            .get_or_init(|| {
                load_vladimir_sanguine_pool_defaults().unwrap_or_else(|err| panic!("{}", err))
            })
            .clone(),
    )
}

pub(crate) fn vladimir_defensive_ability_two_policy_defaults(
    champion_name: &str,
) -> Option<VladimirDefensiveAbilityTwoPolicyDefaults> {
    if normalize_key(champion_name) != "vladimir" {
        return None;
    }
    Some(
        *VLADIMIR_DEFENSIVE_ABILITY_TWO_POLICY_DEFAULTS.get_or_init(|| {
            load_vladimir_defensive_ability_two_policy_defaults()
                .unwrap_or_else(|err| panic!("{}", err))
        }),
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

pub(crate) fn doctor_mundo_infected_bonesaw_ability_defaults()
-> &'static DoctorMundoInfectedBonesawAbilityDefaults {
    DOCTOR_MUNDO_INFECTED_BONESAW_ABILITY_DEFAULTS.get_or_init(|| {
        load_doctor_mundo_infected_bonesaw_ability_defaults()
            .unwrap_or_else(|err| panic!("{}", err))
    })
}

pub(crate) fn champion_slot_bindings(champion_name: &str) -> HashMap<String, String> {
    CHAMPION_SLOT_BINDINGS
        .get_or_init(|| load_champion_slot_bindings().unwrap_or_else(|err| panic!("{}", err)))
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
    Some(
        VLADIMIR_CAST_PROFILE_DEFAULTS
            .get_or_init(|| {
                load_vladimir_cast_profile_defaults().unwrap_or_else(|err| panic!("{}", err))
            })
            .clone(),
    )
}

pub(crate) fn vladimir_offensive_ability_defaults(
    champion_name: &str,
) -> Option<VladimirOffensiveAbilityDefaults> {
    if normalize_key(champion_name) != "vladimir" {
        return None;
    }
    Some(*VLADIMIR_OFFENSIVE_ABILITY_DEFAULTS.get_or_init(|| {
        load_vladimir_offensive_ability_defaults().unwrap_or_else(|err| panic!("{}", err))
    }))
}

pub(crate) fn warwick_infinite_duress_ability_defaults(
    champion_name: &str,
) -> Option<WarwickInfiniteDuressAbilityDefaults> {
    if normalize_key(champion_name) != "warwick" {
        return None;
    }
    Some(*WARWICK_INFINITE_DURESS_ABILITY_DEFAULTS.get_or_init(|| {
        load_warwick_infinite_duress_ability_defaults().unwrap_or_else(|err| panic!("{}", err))
    }))
}

pub(crate) fn warwick_eternal_hunger_passive_defaults(
    champion_name: &str,
) -> Option<WarwickEternalHungerPassiveDefaults> {
    if normalize_key(champion_name) != "warwick" {
        return None;
    }
    Some(*WARWICK_ETERNAL_HUNGER_PASSIVE_DEFAULTS.get_or_init(|| {
        load_warwick_eternal_hunger_passive_defaults().unwrap_or_else(|err| panic!("{}", err))
    }))
}

pub(crate) fn vayne_tumble_ability_defaults(
    champion_name: &str,
) -> Option<VayneTumbleAbilityDefaults> {
    if normalize_key(champion_name) != "vayne" {
        return None;
    }
    Some(*VAYNE_TUMBLE_ABILITY_DEFAULTS.get_or_init(|| {
        load_vayne_tumble_ability_defaults().unwrap_or_else(|err| panic!("{}", err))
    }))
}

pub(crate) fn vayne_silver_bolts_ability_defaults(
    champion_name: &str,
) -> Option<VayneSilverBoltsAbilityDefaults> {
    if normalize_key(champion_name) != "vayne" {
        return None;
    }
    Some(*VAYNE_SILVER_BOLTS_ABILITY_DEFAULTS.get_or_init(|| {
        load_vayne_silver_bolts_ability_defaults().unwrap_or_else(|err| panic!("{}", err))
    }))
}

pub(crate) fn morgana_binding_and_soul_shackles_ability_defaults(
    champion_name: &str,
) -> Option<MorganaBindingAndSoulShacklesAbilityDefaults> {
    if normalize_key(champion_name) != "morgana" {
        return None;
    }
    Some(
        *MORGANA_BINDING_AND_SOUL_SHACKLES_ABILITY_DEFAULTS.get_or_init(|| {
            load_morgana_binding_and_soul_shackles_ability_defaults()
                .unwrap_or_else(|err| panic!("{}", err))
        }),
    )
}

pub(crate) fn sona_crescendo_ability_defaults(
    champion_name: &str,
) -> Option<SonaCrescendoAbilityDefaults> {
    if normalize_key(champion_name) != "sona" {
        return None;
    }
    Some(*SONA_CRESCENDO_ABILITY_DEFAULTS.get_or_init(|| {
        load_sona_crescendo_ability_defaults().unwrap_or_else(|err| panic!("{}", err))
    }))
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

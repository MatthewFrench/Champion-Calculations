use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SimulationDefaults {
    pub server_tick_rate_hz: f64,
    pub dt_fallback_seconds: f64,
    pub champion_level: usize,
    pub time_limit_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SearchDefaults {
    pub beam_width: usize,
    pub max_items: usize,
    pub random_samples: usize,
    pub hill_climb_restarts: usize,
    pub hill_climb_steps: usize,
    pub hill_climb_neighbors: usize,
    pub genetic_population: usize,
    pub genetic_generations: usize,
    pub genetic_mutation_rate: f64,
    pub genetic_crossover_rate: f64,
    pub ranked_limit: usize,
    pub simulated_annealing_restarts: usize,
    pub simulated_annealing_iterations: usize,
    pub simulated_annealing_initial_temp: f64,
    pub simulated_annealing_cooling_rate: f64,
    pub mcts_iterations: usize,
    pub mcts_rollouts_per_expansion: usize,
    pub mcts_exploration: f64,
    pub ensemble_seeds: usize,
    pub ensemble_seed_stride: u64,
    pub ensemble_seed_top_k: usize,
    pub objective_survival_weight: f64,
    pub objective_damage_weight: f64,
    pub objective_healing_weight: f64,
    pub objective_enemy_kills_weight: f64,
    pub objective_invulnerable_seconds_weight: f64,
    pub robust_min_seed_hit_rate: f64,
    pub bleed_enabled: bool,
    pub bleed_budget: usize,
    pub bleed_mutation_rate: f64,
    pub multi_scenario_worst_weight: f64,
    pub seed: u64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct SearchQualityProfilePreset {
    pub beam_width: usize,
    pub random_samples: usize,
    pub hill_climb_restarts: usize,
    pub hill_climb_steps: usize,
    pub hill_climb_neighbors: usize,
    pub genetic_population: usize,
    pub genetic_generations: usize,
    pub simulated_annealing_restarts: usize,
    pub simulated_annealing_iterations: usize,
    pub mcts_iterations: usize,
    pub mcts_rollouts_per_expansion: usize,
    pub ensemble_seeds: usize,
    pub ensemble_seed_top_k: usize,
    pub ranked_limit: usize,
    pub bleed_budget: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SearchQualityProfileDefaults {
    pub fast: SearchQualityProfilePreset,
    pub balanced: SearchQualityProfilePreset,
    pub maximum_quality_minimums: SearchQualityProfilePreset,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct EngineDefaults {
    pub default_champion_hitbox_radius: f64,
    pub champion_hitbox_radius_overrides: HashMap<String, f64>,
    pub melee_spawn_attack_range_threshold: f64,
    pub melee_spawn_radius: f64,
    pub ranged_spawn_radius_multiplier: f64,
    pub ranged_spawn_radius_min: f64,
    pub ranged_spawn_radius_max: f64,
    pub minimum_attack_speed: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct ChampionBehaviorDefaultsEntry {
    pub attack_range: f64,
    pub attack_windup_seconds: f64,
    pub attack_projectile_speed: f64,
    pub attack_effect_hitbox_radius: f64,
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub(crate) struct ChampionBehaviorOverrideEntry {
    #[serde(default)]
    pub attack_range: Option<f64>,
    #[serde(default)]
    pub attack_windup_seconds: Option<f64>,
    #[serde(default)]
    pub attack_projectile_speed: Option<f64>,
    #[serde(default)]
    pub attack_effect_hitbox_radius: Option<f64>,
    #[serde(default)]
    pub on_hit_magic_flat: Option<f64>,
    #[serde(default)]
    pub on_hit_magic_ad_ratio: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_every: Option<usize>,
    #[serde(default)]
    pub periodic_true_hit_base: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_ad_ratio: Option<f64>,
    #[serde(default)]
    pub periodic_true_hit_target_max_health_ratio: Option<f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct VladimirCastProfileDefaults {
    pub q_ability_id: String,
    pub e_ability_id: String,
    pub r_ability_id: String,
    pub pool_ability_id: String,
    pub q_range: f64,
    pub q_windup_seconds: f64,
    pub q_projectile_speed: f64,
    pub q_effect_hitbox_radius: f64,
    pub e_range: f64,
    pub e_windup_seconds: f64,
    pub e_projectile_speed: f64,
    pub e_effect_hitbox_radius: f64,
    pub r_range: f64,
    pub r_windup_seconds: f64,
    pub r_projectile_speed: f64,
    pub r_effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VladimirOffensiveAbilityDefaults {
    pub q_base_damage: f64,
    pub q_ap_ratio: f64,
    pub q_heal_ratio_of_damage: f64,
    pub q_base_cooldown_seconds: f64,
    pub e_base_damage: f64,
    pub e_ap_ratio: f64,
    pub e_base_cooldown_seconds: f64,
    pub r_base_damage: f64,
    pub r_ap_ratio: f64,
    pub r_base_cooldown_seconds: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct VladimirSanguinePoolDefaults {
    pub base_cooldown_seconds_by_rank: Vec<f64>,
    pub default_rank: usize,
    pub untargetable_seconds: f64,
    pub cost_percent_current_health: f64,
    pub heal_ratio_of_damage: f64,
    pub base_damage_by_rank: Vec<f64>,
    pub bonus_health_ratio: f64,
}

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

#[derive(Debug, Clone, Copy, Deserialize)]
struct ControlledChampionDefensiveItemPolicyDefaults {
    pub stasis_trigger_health_percent: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct ChampionAiDefaults {
    pub script_poll_interval_seconds: f64,
    pub movement_speed_scale: f64,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct ChampionAiProfileOverrideEntry {
    #[serde(default)]
    pub desired_combat_range: Option<f64>,
    #[serde(default)]
    pub movement_speed_scale: Option<f64>,
    #[serde(default)]
    pub script_poll_interval_seconds: Option<f64>,
    #[serde(default)]
    pub script_priority_overrides: HashMap<String, i32>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChampionAiProfilesFile {
    defaults: ChampionAiDefaults,
    controlled_champion_defaults: ControlledChampionDefensiveItemPolicyDefaults,
    #[serde(default)]
    champions: HashMap<String, ChampionAiProfileOverrideEntry>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct AbilityExecutionDefaultsEntry {
    #[serde(default)]
    pub cast_windup_seconds: f64,
    #[serde(default)]
    pub projectile_speed: f64,
    #[serde(default)]
    pub effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct AbilityExecutionDefaultsByRole {
    melee: AbilityExecutionDefaultsEntry,
    ranged: AbilityExecutionDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct AbilityExecutionOverrideEntry {
    #[serde(default)]
    pub cast_windup_seconds: Option<f64>,
    #[serde(default)]
    pub projectile_speed: Option<f64>,
    #[serde(default)]
    pub effect_hitbox_radius: Option<f64>,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
struct ChampionAbilityExecutionData {
    is_melee: bool,
    abilities: HashMap<String, AbilityExecutionOverrideEntry>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionAiProfile {
    pub desired_combat_range: f64,
    pub movement_speed_scale: f64,
    pub script_poll_interval_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct AbilityExecutionProfile {
    pub cast_windup_seconds: f64,
    pub projectile_speed: f64,
    pub effect_hitbox_radius: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WarwickInfiniteDuressAbilityDefaults {
    pub infinite_duress_cast_range: f64,
    pub infinite_duress_cooldown_seconds: f64,
    pub infinite_duress_execution: AbilityExecutionProfile,
    pub infinite_duress_physical_attack_damage_ratio: f64,
    pub infinite_duress_magic_base_damage: f64,
    pub infinite_duress_magic_attack_damage_ratio: f64,
    pub infinite_duress_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct WarwickEternalHungerPassiveDefaults {
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VayneTumbleAbilityDefaults {
    pub tumble_cooldown_seconds: f64,
    pub tumble_bonus_physical_attack_damage_ratio: f64,
    pub tumble_bonus_physical_ability_power_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VayneSilverBoltsAbilityDefaults {
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MorganaBindingAndSoulShacklesAbilityDefaults {
    pub dark_binding_cast_range: f64,
    pub dark_binding_cooldown_seconds: f64,
    pub dark_binding_execution: AbilityExecutionProfile,
    pub dark_binding_magic_base_damage: f64,
    pub dark_binding_magic_ability_power_ratio: f64,
    pub dark_binding_stun_duration_seconds: f64,
    pub soul_shackles_cast_range: f64,
    pub soul_shackles_cooldown_seconds: f64,
    pub soul_shackles_execution: AbilityExecutionProfile,
    pub soul_shackles_detonate_delay_seconds: f64,
    pub soul_shackles_initial_magic_damage: f64,
    pub soul_shackles_initial_magic_ability_power_ratio: f64,
    pub soul_shackles_detonate_magic_damage: f64,
    pub soul_shackles_detonate_magic_ability_power_ratio: f64,
    pub soul_shackles_detonate_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SonaCrescendoAbilityDefaults {
    pub crescendo_cast_range: f64,
    pub crescendo_cooldown_seconds: f64,
    pub crescendo_execution: AbilityExecutionProfile,
    pub crescendo_magic_base_damage: f64,
    pub crescendo_magic_ability_power_ratio: f64,
    pub crescendo_stun_duration_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DoctorMundoInfectedBonesawAbilityDefaults {
    pub cast_range: f64,
    pub cooldown_seconds: f64,
    pub infected_bonesaw_execution: AbilityExecutionProfile,
    pub current_health_ratio: f64,
    pub minimum_magic_damage: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub(crate) struct ChampionSimulationData {
    #[serde(default)]
    pub behavior: Option<ChampionBehaviorOverrideEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChampionFileEnvelope {
    #[serde(default)]
    name: String,
    #[serde(default)]
    behavior: Option<ChampionBehaviorOverrideEntry>,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ChampionBaseStatsDefaultsEntry {
    #[serde(default)]
    attack_range: f64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct ChampionBaseStatsDefaultsByRole {
    melee: ChampionBaseStatsDefaultsEntry,
    ranged: ChampionBaseStatsDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ChampionBasicAttackRawTimingDefaultsEntry {
    #[serde(default)]
    gameplay_radius: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ChampionBasicAttackDefaultsEntry {
    #[serde(default)]
    base_windup_seconds: f64,
    #[serde(default)]
    missile_speed: f64,
    #[serde(default)]
    raw_timing_stats: ChampionBasicAttackRawTimingDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct ChampionBasicAttackDefaultsByRole {
    melee: ChampionBasicAttackDefaultsEntry,
    ranged: ChampionBasicAttackDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ChampionBehaviorOnHitDefaultsEntry {
    #[serde(default)]
    magic_flat: f64,
    #[serde(default)]
    magic_ad_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ChampionBehaviorPeriodicTrueHitDefaultsEntry {
    #[serde(default)]
    every: usize,
    #[serde(default)]
    base: f64,
    #[serde(default)]
    ad_ratio: f64,
    #[serde(default)]
    target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ChampionBehaviorModifiersEntry {
    #[serde(default)]
    on_hit: ChampionBehaviorOnHitDefaultsEntry,
    #[serde(default)]
    periodic_true_hit: ChampionBehaviorPeriodicTrueHitDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Default, Deserialize)]
struct ChampionBehaviorModifiersByRole {
    #[serde(default)]
    melee: ChampionBehaviorModifiersEntry,
    #[serde(default)]
    ranged: ChampionBehaviorModifiersEntry,
}

#[derive(Debug, Clone, Deserialize)]
struct ChampionAbilitiesDefaults {
    execution_defaults: AbilityExecutionDefaultsByRole,
}

#[derive(Debug, Clone, Deserialize)]
struct ChampionDefaultsFile {
    base_stats: ChampionBaseStatsDefaultsByRole,
    basic_attack: ChampionBasicAttackDefaultsByRole,
    abilities: ChampionAbilitiesDefaults,
    #[serde(default)]
    behavior: ChampionBehaviorModifiersByRole,
}

#[derive(Debug, Clone)]
struct ChampionBehaviorDefaults {
    melee: ChampionBehaviorDefaultsEntry,
    ranged: ChampionBehaviorDefaultsEntry,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub(crate) struct UrfRespawnDefaults {
    pub flat_reduction_seconds: f64,
    pub extrapolation_per_level: f64,
    pub time_scaling_enabled: bool,
    pub time_scaling_start_seconds: f64,
    pub time_scaling_per_minute_seconds: f64,
    pub time_scaling_cap_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct UrfFileEnvelope {
    #[serde(default)]
    respawn: Option<UrfRespawnDefaults>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct SimulatorDefaults {
    pub simulation_defaults: SimulationDefaults,
    pub search_defaults: SearchDefaults,
    pub search_quality_profile_defaults: SearchQualityProfileDefaults,
    pub engine_defaults: EngineDefaults,
}

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
static PROTOPLASM_LIFELINE_COOLDOWN_SECONDS_DEFAULT: OnceLock<f64> = OnceLock::new();
static VLADIMIR_SANGUINE_POOL_DEFAULTS: OnceLock<VladimirSanguinePoolDefaults> = OnceLock::new();
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

fn normalize_key(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn normalize_snake_key(input: &str) -> String {
    let mut normalized = String::new();
    let mut previous_was_separator = false;
    for character in input.chars() {
        if character.is_ascii_alphanumeric() {
            normalized.push(character.to_ascii_lowercase());
            previous_was_separator = false;
        } else if !normalized.is_empty() && !previous_was_separator {
            normalized.push('_');
            previous_was_separator = true;
        }
    }
    while normalized.ends_with('_') {
        normalized.pop();
    }
    normalized
}

fn is_character_support_file(stem: &str) -> bool {
    normalize_key(stem) == "championdefaults"
}

fn repository_root_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn load_defaults_from_disk() -> Result<SimulatorDefaults> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("simulator_defaults.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading simulator defaults: {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing simulator defaults: {}", path.display()))
}

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
    serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion AI profiles: {}", path.display()))
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

fn load_protoplasm_lifeline_cooldown_seconds_default() -> Result<f64> {
    let path = repository_root_dir()
        .join("Items")
        .join("Protoplasm Harness.json");
    let text = std::fs::read_to_string(&path).with_context(|| {
        format!(
            "Failed reading Protoplasm Harness item file: {}",
            path.display()
        )
    })?;
    let item_file: Value = serde_json::from_str(&text).with_context(|| {
        format!(
            "Failed parsing Protoplasm Harness item file: {}",
            path.display()
        )
    })?;
    let effects = item_file
        .get("effects_structured")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing effects_structured in {}", path.display()))?;
    let lifeline_cooldown_seconds = ability_effect_by_id(
        effects,
        "lifeline_gain_bonus_health_below_health_threshold",
    )
    .and_then(|effect| effect.get("cooldown_seconds"))
    .and_then(Value::as_f64)
    .or_else(|| {
        effects
            .iter()
            .find_map(|effect| effect.get("cooldown_seconds").and_then(Value::as_f64))
    })
    .ok_or_else(|| {
        anyhow!(
            "Missing effects_structured[*].cooldown_seconds for Protoplasm Harness lifeline in {}",
            path.display()
        )
    })?;
    Ok(lifeline_cooldown_seconds)
}

fn ability_effect_by_id<'a>(effects: &'a [Value], effect_id: &str) -> Option<&'a Value> {
    effects
        .iter()
        .find(|effect| effect.get("id").and_then(Value::as_str) == Some(effect_id))
}

fn highest_rank_value(values: &[Value]) -> Option<f64> {
    values.last().and_then(Value::as_f64)
}

fn read_champion_file(champion_file_name: &str) -> Result<(std::path::PathBuf, Value)> {
    let path = repository_root_dir()
        .join("Characters")
        .join(champion_file_name);
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading champion file: {}", path.display()))?;
    let data: Value = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing champion file: {}", path.display()))?;
    Ok((path, data))
}

fn read_item_file(item_file_name: &str) -> Result<(std::path::PathBuf, Value)> {
    let path = repository_root_dir().join("Items").join(item_file_name);
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading item file: {}", path.display()))?;
    let data: Value = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing item file: {}", path.display()))?;
    Ok((path, data))
}

fn item_effects<'a>(item_data: &'a Value, item_path: &std::path::Path) -> Result<&'a [Value]> {
    item_data
        .get("effects_structured")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| anyhow!("Missing effects_structured in {}", item_path.display()))
}

fn effect_value_range(effect: &Value) -> Option<(f64, f64)> {
    let min = effect.pointer("/value_range/min").and_then(Value::as_f64)?;
    let max = effect.pointer("/value_range/max").and_then(Value::as_f64)?;
    Some((min, max))
}

fn ratio_from_health_threshold_condition(effect: &Value) -> Option<f64> {
    let conditions = effect.get("conditions").and_then(Value::as_array)?;
    for condition in conditions {
        let raw = condition.as_str()?.trim().to_ascii_lowercase();
        if !raw.starts_with("health_below_") || !raw.ends_with("_percent") {
            continue;
        }
        let middle = raw
            .strip_prefix("health_below_")
            .and_then(|value| value.strip_suffix("_percent"))?;
        let percent = middle.parse::<f64>().ok()?;
        return Some(percent / 100.0);
    }
    None
}

fn champion_ability<'a>(
    champion_data: &'a Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<&'a Value> {
    champion_data
        .get("abilities")
        .and_then(|abilities| abilities.get(ability_key))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{} in {}",
                ability_key,
                champion_path.display()
            )
        })
}

fn champion_ability_effects<'a>(
    ability: &'a Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<&'a [Value]> {
    ability
        .get("effects")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{}.effects in {}",
                ability_key,
                champion_path.display()
            )
        })
}

fn champion_ability_range(
    ability: &Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<f64> {
    ability.get("range").and_then(Value::as_f64).ok_or_else(|| {
        anyhow!(
            "Missing abilities.{}.range in {}",
            ability_key,
            champion_path.display()
        )
    })
}

fn champion_ability_cooldown_seconds(
    ability: &Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<f64> {
    ability
        .get("cooldown_seconds_by_rank")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{}.cooldown_seconds_by_rank in {}",
                ability_key,
                champion_path.display()
            )
        })
}

fn champion_ability_cooldown_seconds_by_rank(
    ability: &Value,
    ability_key: &str,
    champion_path: &std::path::Path,
) -> Result<Vec<f64>> {
    ability
        .get("cooldown_seconds_by_rank")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.{}.cooldown_seconds_by_rank in {}",
                ability_key,
                champion_path.display()
            )
        })?
        .iter()
        .map(|value| {
            value.as_f64().ok_or_else(|| {
                anyhow!(
                    "Invalid abilities.{}.cooldown_seconds_by_rank value in {}",
                    ability_key,
                    champion_path.display()
                )
            })
        })
        .collect()
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

fn champion_ability_execution_profile_from_ability(
    ability: &Value,
    is_melee: bool,
) -> AbilityExecutionProfile {
    let role_defaults = champion_ability_execution_defaults_for_role_internal(is_melee);
    let execution = ability
        .get("execution")
        .cloned()
        .and_then(|value| serde_json::from_value::<AbilityExecutionOverrideEntry>(value).ok())
        .unwrap_or_default();
    AbilityExecutionProfile {
        cast_windup_seconds: execution
            .cast_windup_seconds
            .unwrap_or(role_defaults.cast_windup_seconds),
        projectile_speed: execution
            .projectile_speed
            .unwrap_or(role_defaults.projectile_speed),
        effect_hitbox_radius: execution
            .effect_hitbox_radius
            .unwrap_or(role_defaults.effect_hitbox_radius),
    }
}

fn effect_base_by_rank(effects: &[Value], effect_id: &str) -> Option<f64> {
    ability_effect_by_id(effects, effect_id)
        .and_then(|effect| effect.get("base_by_rank"))
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
}

fn effect_base_by_rank_values(effects: &[Value], effect_id: &str) -> Option<Vec<f64>> {
    ability_effect_by_id(effects, effect_id)
        .and_then(|effect| effect.get("base_by_rank"))
        .and_then(Value::as_array)
        .map(|values| values.iter().filter_map(Value::as_f64).collect::<Vec<_>>())
        .filter(|values| !values.is_empty())
}

fn effect_formula_coefficient(effect: &Value, input_stat: &str) -> Option<f64> {
    effect
        .get("formula")
        .and_then(|formula| formula.get("terms"))
        .and_then(Value::as_array)
        .and_then(|terms| {
            terms
                .iter()
                .find(|term| term.get("input_stat").and_then(Value::as_str) == Some(input_stat))
        })
        .and_then(|term| term.get("coefficient"))
        .and_then(Value::as_f64)
}

fn effect_formula_coefficient_by_id(
    effects: &[Value],
    effect_id: &str,
    input_stat: &str,
) -> Option<f64> {
    ability_effect_by_id(effects, effect_id)
        .and_then(|effect| effect_formula_coefficient(effect, input_stat))
}

fn effect_contextual_multiplier_by_rank(effect: &Value) -> Option<f64> {
    effect
        .pointer("/formula/contextual_multipliers/by_rank")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
}

fn effect_duration_seconds(effect: &Value) -> Option<f64> {
    effect
        .get("value_seconds_by_rank")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .or_else(|| effect.get("value_seconds").and_then(Value::as_f64))
}

fn effect_duration_seconds_by_id(effects: &[Value], effect_id: &str) -> Option<f64> {
    ability_effect_by_id(effects, effect_id).and_then(effect_duration_seconds)
}

fn load_vladimir_cast_profile_defaults() -> Result<VladimirCastProfileDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let q_ability = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let e_ability = champion_ability(&champion_data, "basic_ability_3", &champion_path)?;
    let r_ability = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let champion_is_melee = champion_is_melee_from_data(&champion_data);
    let q_execution = champion_ability_execution_profile_from_ability(q_ability, champion_is_melee);
    let e_execution = champion_ability_execution_profile_from_ability(e_ability, champion_is_melee);
    let r_execution = champion_ability_execution_profile_from_ability(r_ability, champion_is_melee);
    let slot_bindings = champion_slot_bindings("vladimir");

    let q_ability_id = slot_bindings.get("Q").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for Q from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;
    let pool_ability_id = slot_bindings.get("W").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for W from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;
    let e_ability_id = slot_bindings.get("E").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for E from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;
    let r_ability_id = slot_bindings.get("R").cloned().ok_or_else(|| {
        anyhow!(
            "Missing derived slot binding for R from abilities.<ability>.slot/default_keybinding in {}",
            champion_path.display()
        )
    })?;

    Ok(VladimirCastProfileDefaults {
        q_ability_id,
        e_ability_id,
        r_ability_id,
        pool_ability_id,
        q_range: champion_ability_range(q_ability, "basic_ability_1", &champion_path)?,
        q_windup_seconds: q_execution.cast_windup_seconds,
        q_projectile_speed: q_execution.projectile_speed,
        q_effect_hitbox_radius: q_execution.effect_hitbox_radius,
        e_range: champion_ability_range(e_ability, "basic_ability_3", &champion_path)?,
        e_windup_seconds: e_execution.cast_windup_seconds,
        e_projectile_speed: e_execution.projectile_speed,
        e_effect_hitbox_radius: e_execution.effect_hitbox_radius,
        r_range: champion_ability_range(r_ability, "ultimate", &champion_path)?,
        r_windup_seconds: r_execution.cast_windup_seconds,
        r_projectile_speed: r_execution.projectile_speed,
        r_effect_hitbox_radius: r_execution.effect_hitbox_radius,
    })
}

fn load_vladimir_offensive_ability_defaults() -> Result<VladimirOffensiveAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let q_ability = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let e_ability = champion_ability(&champion_data, "basic_ability_3", &champion_path)?;
    let r_ability = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let q_effects = champion_ability_effects(q_ability, "basic_ability_1", &champion_path)?;
    let e_effects = champion_ability_effects(e_ability, "basic_ability_3", &champion_path)?;
    let r_effects = champion_ability_effects(r_ability, "ultimate", &champion_path)?;

    let q_base_damage = effect_base_by_rank(q_effects, "magic_damage").ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_1.effects[id=magic_damage].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let q_ap_ratio = effect_formula_coefficient_by_id(q_effects, "magic_damage", "ability_power")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=magic_damage] ability_power coefficient in {}",
                champion_path.display()
            )
        })?;
    let q_heal_base = effect_base_by_rank(q_effects, "heal").ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_1.effects[id=heal].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let q_heal_ratio_of_damage = if q_base_damage > 0.0 {
        q_heal_base / q_base_damage
    } else {
        0.0
    };

    let e_base_damage = effect_base_by_rank(e_effects, "maximum_damage").ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_3.effects[id=maximum_damage].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let e_ap_ratio =
        effect_formula_coefficient_by_id(e_effects, "maximum_damage", "ability_power")
            .ok_or_else(|| {
                anyhow!(
                    "Missing abilities.basic_ability_3.effects[id=maximum_damage] ability_power coefficient in {}",
                    champion_path.display()
                )
            })?;

    let r_base_damage = effect_base_by_rank(r_effects, "detonation_damage").ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=detonation_damage].base_by_rank in {}",
            champion_path.display()
        )
    })?;
    let r_ap_ratio =
        effect_formula_coefficient_by_id(r_effects, "detonation_damage", "ability_power")
            .ok_or_else(|| {
                anyhow!(
                    "Missing abilities.ultimate.effects[id=detonation_damage] ability_power coefficient in {}",
                    champion_path.display()
                )
            })?;

    Ok(VladimirOffensiveAbilityDefaults {
        q_base_damage,
        q_ap_ratio,
        q_heal_ratio_of_damage,
        q_base_cooldown_seconds: champion_ability_cooldown_seconds(
            q_ability,
            "basic_ability_1",
            &champion_path,
        )?,
        e_base_damage,
        e_ap_ratio,
        e_base_cooldown_seconds: champion_ability_cooldown_seconds(
            e_ability,
            "basic_ability_3",
            &champion_path,
        )?,
        r_base_damage,
        r_ap_ratio,
        r_base_cooldown_seconds: champion_ability_cooldown_seconds(
            r_ability,
            "ultimate",
            &champion_path,
        )?,
    })
}

fn load_vladimir_sanguine_pool_defaults() -> Result<VladimirSanguinePoolDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vladimir.json")?;
    let pool_ability = champion_ability(&champion_data, "basic_ability_2", &champion_path)?;
    let pool_effects = champion_ability_effects(pool_ability, "basic_ability_2", &champion_path)?;

    let base_cooldown_seconds_by_rank =
        champion_ability_cooldown_seconds_by_rank(pool_ability, "basic_ability_2", &champion_path)?;
    let default_rank = base_cooldown_seconds_by_rank.len().max(1);
    let untargetable_seconds = effect_duration_seconds_by_id(pool_effects, "untargetable")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=untargetable] duration in {}",
                champion_path.display()
            )
        })?;
    let cost_percent_current_health = pool_ability
        .pointer("/cost/ratio")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.cost.ratio in {}",
                champion_path.display()
            )
        })?;
    let heal_ratio_of_damage = ability_effect_by_id(pool_effects, "heal_from_damage")
        .and_then(|effect| {
            effect
                .pointer("/formula/contextual_multipliers/by_target_type/champions")
                .and_then(Value::as_f64)
        })
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=heal_from_damage] champion multiplier in {}",
                champion_path.display()
            )
        })?;
    let base_damage_by_rank =
        effect_base_by_rank_values(pool_effects, "total_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=total_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let bonus_health_ratio =
        effect_formula_coefficient_by_id(pool_effects, "total_damage", "bonus_health")
            .ok_or_else(|| {
                anyhow!(
                    "Missing abilities.basic_ability_2.effects[id=total_damage] bonus_health coefficient in {}",
                    champion_path.display()
                )
            })?;

    Ok(VladimirSanguinePoolDefaults {
        base_cooldown_seconds_by_rank,
        default_rank,
        untargetable_seconds,
        cost_percent_current_health,
        heal_ratio_of_damage,
        base_damage_by_rank,
        bonus_health_ratio,
    })
}

fn load_zhonya_time_stop_defaults() -> Result<ZhonyaTimeStopDefaults> {
    let (item_path, item_data) = read_item_file("Zhonyas Hourglass.json")?;
    let effects = item_effects(&item_data, &item_path)?;
    let time_stop = ability_effect_by_id(effects, "zhonyas_time_stop").ok_or_else(|| {
        anyhow!(
            "Missing effects_structured[id=zhonyas_time_stop] in {}",
            item_path.display()
        )
    })?;
    let duration_seconds = time_stop
        .get("status_effects")
        .and_then(Value::as_array)
        .and_then(|effects| {
            effects.iter().find_map(|status| {
                let status_type = status.get("type").and_then(Value::as_str)?;
                if status_type.eq_ignore_ascii_case("stasis") {
                    status.get("duration_seconds").and_then(Value::as_f64)
                } else {
                    None
                }
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "Missing stasis duration in effects_structured[id=zhonyas_time_stop] in {}",
                item_path.display()
            )
        })?;
    let cooldown_seconds = time_stop
        .get("cooldown_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing cooldown_seconds in effects_structured[id=zhonyas_time_stop] in {}",
                item_path.display()
            )
        })?;
    Ok(ZhonyaTimeStopDefaults {
        duration_seconds,
        cooldown_seconds,
    })
}

fn load_guardian_angel_rebirth_defaults() -> Result<GuardianAngelRebirthDefaults> {
    let (item_path, item_data) = read_item_file("Guardian Angel.json")?;
    let effects = item_effects(&item_data, &item_path)?;
    let rebirth = ability_effect_by_id(
        effects,
        "rebirth_resurrection_with_post_revive_health_and_mana_restore",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing Guardian Angel rebirth effect id in {}",
            item_path.display()
        )
    })?;
    let cooldown_seconds = rebirth
        .get("cooldown_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing cooldown_seconds in Guardian Angel rebirth effect in {}",
                item_path.display()
            )
        })?;
    let revive_duration_seconds = rebirth
        .get("duration_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing duration_seconds in Guardian Angel rebirth effect in {}",
                item_path.display()
            )
        })?;
    let revive_base_health_ratio = rebirth
        .get("scaling_terms")
        .and_then(Value::as_array)
        .and_then(|terms| {
            terms.iter().find_map(|term| {
                let applies_to = term.get("applies_to").and_then(Value::as_str)?;
                if applies_to != "heal_on_revive" {
                    return None;
                }
                let stat = term.get("stat").and_then(Value::as_str)?;
                if stat != "base_health" {
                    return None;
                }
                term.get("ratio").and_then(Value::as_f64)
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "Missing base_health revive ratio in Guardian Angel rebirth effect in {}",
                item_path.display()
            )
        })?;
    Ok(GuardianAngelRebirthDefaults {
        cooldown_seconds,
        revive_duration_seconds,
        revive_base_health_ratio,
    })
}

fn load_protoplasm_lifeline_defaults() -> Result<ProtoplasmLifelineDefaults> {
    let (item_path, item_data) = read_item_file("Protoplasm Harness.json")?;
    let effects = item_effects(&item_data, &item_path)?;
    let lifeline_bonus =
        ability_effect_by_id(effects, "lifeline_gain_bonus_health_below_health_threshold")
            .ok_or_else(|| {
                anyhow!(
                    "Missing Protoplasm lifeline bonus health effect in {}",
                    item_path.display()
                )
            })?;
    let lifeline_heal =
        ability_effect_by_id(effects, "lifeline_heal_over_time_scaling_with_resists").ok_or_else(
            || {
                anyhow!(
                    "Missing Protoplasm lifeline heal effect in {}",
                    item_path.display()
                )
            },
        )?;
    let trigger_health_percent =
        ratio_from_health_threshold_condition(lifeline_bonus).ok_or_else(|| {
            anyhow!(
                "Missing parseable health threshold condition in Protoplasm lifeline effect in {}",
                item_path.display()
            )
        })?;
    let (bonus_health_min, bonus_health_max) =
        effect_value_range(lifeline_bonus).ok_or_else(|| {
            anyhow!(
                "Missing value_range in Protoplasm lifeline bonus health effect in {}",
                item_path.display()
            )
        })?;
    let (heal_total_min, heal_total_max) = effect_value_range(lifeline_heal).ok_or_else(|| {
        anyhow!(
            "Missing value_range in Protoplasm lifeline heal effect in {}",
            item_path.display()
        )
    })?;
    let duration_seconds = lifeline_bonus
        .get("duration_seconds")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing duration_seconds in Protoplasm lifeline bonus effect in {}",
                item_path.display()
            )
        })?;
    Ok(ProtoplasmLifelineDefaults {
        trigger_health_percent,
        bonus_health_min,
        bonus_health_max,
        heal_total_min,
        heal_total_max,
        duration_seconds,
    })
}

fn load_warwick_infinite_duress_ability_defaults() -> Result<WarwickInfiniteDuressAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Warwick.json")?;
    let ultimate = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let effects = champion_ability_effects(ultimate, "ultimate", &champion_path)?;
    let champion_is_melee = champion_is_melee_from_data(&champion_data);

    let infinite_duress_magic_base_damage = effect_base_by_rank(effects, "total_magic_damage")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=total_magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let infinite_duress_magic_attack_damage_ratio =
        effect_formula_coefficient_by_id(effects, "total_magic_damage", "bonus_attack_damage")
            .ok_or_else(|| {
                anyhow!(
                    "Missing abilities.ultimate.effects[id=total_magic_damage] bonus_attack_damage coefficient in {}",
                    champion_path.display()
                )
            })?;
    let infinite_duress_stun_duration_seconds =
        effect_duration_seconds_by_id(effects, "suppression_duration").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=suppression_duration] duration in {}",
                champion_path.display()
            )
        })?;
    let on_hit_applications = ability_effect_by_id(effects, "on_hit_applications")
        .and_then(|effect| effect.get("value"))
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=on_hit_applications].value in {}",
                champion_path.display()
            )
        })?;
    let physical_attack_damage_ratio_per_hit = ability_effect_by_id(
        effects,
        "physical_attack_damage_ratio_per_hit",
    )
    .and_then(|effect| effect.get("value_ratio"))
    .and_then(Value::as_f64)
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=physical_attack_damage_ratio_per_hit].value_ratio in {}",
            champion_path.display()
        )
    })?;

    Ok(WarwickInfiniteDuressAbilityDefaults {
        infinite_duress_cast_range: champion_ability_range(ultimate, "ultimate", &champion_path)?,
        infinite_duress_cooldown_seconds: champion_ability_cooldown_seconds(
            ultimate,
            "ultimate",
            &champion_path,
        )?,
        infinite_duress_execution: champion_ability_execution_profile_from_ability(
            ultimate,
            champion_is_melee,
        ),
        infinite_duress_physical_attack_damage_ratio: on_hit_applications
            * physical_attack_damage_ratio_per_hit,
        infinite_duress_magic_base_damage,
        infinite_duress_magic_attack_damage_ratio,
        infinite_duress_stun_duration_seconds,
    })
}

fn load_warwick_eternal_hunger_passive_defaults() -> Result<WarwickEternalHungerPassiveDefaults> {
    let (champion_path, champion_data) = read_champion_file("Warwick.json")?;
    let passive = champion_ability(&champion_data, "passive", &champion_path)?;
    let effects = champion_ability_effects(passive, "passive", &champion_path)?;
    let on_hit_effect =
        ability_effect_by_id(effects, "bonus_magic_damage_on_hit").ok_or_else(|| {
            anyhow!(
                "Missing abilities.passive.effects[id=bonus_magic_damage_on_hit] in {}",
                champion_path.display()
            )
        })?;
    let on_hit_magic_flat = on_hit_effect
        .get("base_by_champion_level")
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.passive.effects[id=bonus_magic_damage_on_hit].base_by_champion_level in {}",
                champion_path.display()
            )
        })?;
    let on_hit_magic_ad_ratio = effect_formula_coefficient(on_hit_effect, "bonus_attack_damage")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.passive.effects[id=bonus_magic_damage_on_hit] bonus_attack_damage coefficient in {}",
                champion_path.display()
            )
        })?;
    Ok(WarwickEternalHungerPassiveDefaults {
        on_hit_magic_flat,
        on_hit_magic_ad_ratio,
    })
}

fn load_vayne_tumble_ability_defaults() -> Result<VayneTumbleAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vayne.json")?;
    let tumble = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let effects = champion_ability_effects(tumble, "basic_ability_1", &champion_path)?;
    let bonus_damage_effect =
        ability_effect_by_id(effects, "bonus_physical_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=bonus_physical_damage] in {}",
                champion_path.display()
            )
        })?;

    let tumble_bonus_physical_attack_damage_ratio =
        effect_contextual_multiplier_by_rank(bonus_damage_effect).ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=bonus_physical_damage].formula.contextual_multipliers.by_rank in {}",
                champion_path.display()
            )
        })?;
    let tumble_bonus_physical_ability_power_ratio =
        effect_formula_coefficient(bonus_damage_effect, "ability_power").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=bonus_physical_damage] ability_power coefficient in {}",
                champion_path.display()
            )
        })?;

    Ok(VayneTumbleAbilityDefaults {
        tumble_cooldown_seconds: champion_ability_cooldown_seconds(
            tumble,
            "basic_ability_1",
            &champion_path,
        )?,
        tumble_bonus_physical_attack_damage_ratio,
        tumble_bonus_physical_ability_power_ratio,
    })
}

fn load_vayne_silver_bolts_ability_defaults() -> Result<VayneSilverBoltsAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Vayne.json")?;
    let silver_bolts = champion_ability(&champion_data, "basic_ability_2", &champion_path)?;
    let effects = champion_ability_effects(silver_bolts, "basic_ability_2", &champion_path)?;
    let periodic_true_hit_every = ability_effect_by_id(effects, "max_stacks")
        .and_then(|effect| effect.get("value"))
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=max_stacks].value in {}",
                champion_path.display()
            )
        })?;
    let periodic_true_hit_base =
        effect_base_by_rank(effects, "minimum_bonus_true_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_2.effects[id=minimum_bonus_true_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let periodic_true_hit_target_max_health_ratio = ability_effect_by_id(
        effects,
        "bonus_true_damage_percent_target_max_health",
    )
    .and_then(effect_contextual_multiplier_by_rank)
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_2.effects[id=bonus_true_damage_percent_target_max_health].formula.contextual_multipliers.by_rank in {}",
            champion_path.display()
        )
    })?;
    Ok(VayneSilverBoltsAbilityDefaults {
        periodic_true_hit_every,
        periodic_true_hit_base,
        periodic_true_hit_target_max_health_ratio,
    })
}

fn load_morgana_binding_and_soul_shackles_ability_defaults()
-> Result<MorganaBindingAndSoulShacklesAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Morgana.json")?;
    let dark_binding = champion_ability(&champion_data, "basic_ability_1", &champion_path)?;
    let soul_shackles = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let champion_is_melee = champion_is_melee_from_data(&champion_data);
    let dark_binding_effects =
        champion_ability_effects(dark_binding, "basic_ability_1", &champion_path)?;
    let soul_shackles_effects =
        champion_ability_effects(soul_shackles, "ultimate", &champion_path)?;

    let dark_binding_magic_base_damage = effect_base_by_rank(dark_binding_effects, "magic_damage")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let dark_binding_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        dark_binding_effects,
        "magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_1.effects[id=magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;
    let dark_binding_stun_duration_seconds =
        effect_duration_seconds_by_id(dark_binding_effects, "root_duration").ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=root_duration] duration in {}",
                champion_path.display()
            )
        })?;

    let soul_shackles_initial_magic_damage =
        effect_base_by_rank(soul_shackles_effects, "initial_magic_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=initial_magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let soul_shackles_initial_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        soul_shackles_effects,
        "initial_magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=initial_magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;

    let soul_shackles_total_magic_damage =
        effect_base_by_rank(soul_shackles_effects, "total_magic_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=total_magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let soul_shackles_total_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        soul_shackles_effects,
        "total_magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=total_magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;
    let soul_shackles_detonate_stun_duration_seconds =
        effect_duration_seconds_by_id(soul_shackles_effects, "stun_duration").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=stun_duration] duration in {}",
                champion_path.display()
            )
        })?;
    let soul_shackles_detonate_delay_seconds =
        effect_duration_seconds_by_id(soul_shackles_effects, "tether_duration").ok_or_else(
            || {
                anyhow!(
                    "Missing abilities.ultimate.effects[id=tether_duration] duration in {}",
                    champion_path.display()
                )
            },
        )?;

    Ok(MorganaBindingAndSoulShacklesAbilityDefaults {
        dark_binding_cast_range: champion_ability_range(
            dark_binding,
            "basic_ability_1",
            &champion_path,
        )?,
        dark_binding_cooldown_seconds: champion_ability_cooldown_seconds(
            dark_binding,
            "basic_ability_1",
            &champion_path,
        )?,
        dark_binding_execution: champion_ability_execution_profile_from_ability(
            dark_binding,
            champion_is_melee,
        ),
        dark_binding_magic_base_damage,
        dark_binding_magic_ability_power_ratio,
        dark_binding_stun_duration_seconds,
        soul_shackles_cast_range: champion_ability_range(
            soul_shackles,
            "ultimate",
            &champion_path,
        )?,
        soul_shackles_cooldown_seconds: champion_ability_cooldown_seconds(
            soul_shackles,
            "ultimate",
            &champion_path,
        )?,
        soul_shackles_execution: champion_ability_execution_profile_from_ability(
            soul_shackles,
            champion_is_melee,
        ),
        soul_shackles_detonate_delay_seconds,
        soul_shackles_initial_magic_damage,
        soul_shackles_initial_magic_ability_power_ratio,
        soul_shackles_detonate_magic_damage: (soul_shackles_total_magic_damage
            - soul_shackles_initial_magic_damage)
            .max(0.0),
        soul_shackles_detonate_magic_ability_power_ratio:
            (soul_shackles_total_magic_ability_power_ratio
                - soul_shackles_initial_magic_ability_power_ratio)
                .max(0.0),
        soul_shackles_detonate_stun_duration_seconds,
    })
}

fn load_sona_crescendo_ability_defaults() -> Result<SonaCrescendoAbilityDefaults> {
    let (champion_path, champion_data) = read_champion_file("Sona.json")?;
    let crescendo = champion_ability(&champion_data, "ultimate", &champion_path)?;
    let effects = champion_ability_effects(crescendo, "ultimate", &champion_path)?;
    let champion_is_melee = champion_is_melee_from_data(&champion_data);

    let crescendo_magic_base_damage =
        effect_base_by_rank(effects, "magic_damage").ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=magic_damage].base_by_rank in {}",
                champion_path.display()
            )
        })?;
    let crescendo_magic_ability_power_ratio = effect_formula_coefficient_by_id(
        effects,
        "magic_damage",
        "ability_power",
    )
    .ok_or_else(|| {
        anyhow!(
            "Missing abilities.ultimate.effects[id=magic_damage] ability_power coefficient in {}",
            champion_path.display()
        )
    })?;
    let crescendo_stun_duration_seconds = effect_duration_seconds_by_id(effects, "stun")
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.ultimate.effects[id=stun] duration in {}",
                champion_path.display()
            )
        })?;

    Ok(SonaCrescendoAbilityDefaults {
        crescendo_cast_range: champion_ability_range(crescendo, "ultimate", &champion_path)?,
        crescendo_cooldown_seconds: champion_ability_cooldown_seconds(
            crescendo,
            "ultimate",
            &champion_path,
        )?,
        crescendo_execution: champion_ability_execution_profile_from_ability(
            crescendo,
            champion_is_melee,
        ),
        crescendo_magic_base_damage,
        crescendo_magic_ability_power_ratio,
        crescendo_stun_duration_seconds,
    })
}

fn load_doctor_mundo_infected_bonesaw_ability_defaults()
-> Result<DoctorMundoInfectedBonesawAbilityDefaults> {
    let path = repository_root_dir()
        .join("Characters")
        .join("DrMundo.json");
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed reading DrMundo champion file: {}", path.display()))?;
    let data: Value = serde_json::from_str(&text)
        .with_context(|| format!("Failed parsing DrMundo champion file: {}", path.display()))?;

    let ability = data
        .get("abilities")
        .and_then(|abilities| abilities.get("basic_ability_1"))
        .ok_or_else(|| anyhow!("Missing abilities.basic_ability_1 in {}", path.display()))?;
    let champion_is_melee = champion_is_melee_from_data(&data);
    let effects = ability
        .get("effects")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects in {}",
                path.display()
            )
        })?;

    let cast_range = ability
        .get("range")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.range in {}",
                path.display()
            )
        })?;
    let cooldown_seconds = champion_ability_cooldown_seconds(ability, "basic_ability_1", &path)?;

    let current_health_ratio = ability_effect_by_id(effects, "magic_damage_percent_current_health")
        .and_then(|effect| effect.pointer("/formula/contextual_multipliers/by_rank"))
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
        anyhow!(
            "Missing abilities.basic_ability_1.effects[id=magic_damage_percent_current_health] ratio in {}",
            path.display()
        )
    })?;
    let minimum_magic_damage = ability_effect_by_id(effects, "minimum_damage")
        .and_then(|effect| effect.get("base_by_rank"))
        .and_then(Value::as_array)
        .and_then(|values| highest_rank_value(values))
        .ok_or_else(|| {
            anyhow!(
                "Missing abilities.basic_ability_1.effects[id=minimum_damage] base_by_rank in {}",
                path.display()
            )
        })?;

    Ok(DoctorMundoInfectedBonesawAbilityDefaults {
        cast_range,
        cooldown_seconds,
        infected_bonesaw_execution: champion_ability_execution_profile_from_ability(
            ability,
            champion_is_melee,
        ),
        current_health_ratio,
        minimum_magic_damage,
    })
}

pub(crate) fn simulator_defaults() -> &'static SimulatorDefaults {
    SIMULATOR_DEFAULTS
        .get_or_init(|| load_defaults_from_disk().unwrap_or_else(|err| panic!("{}", err)))
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

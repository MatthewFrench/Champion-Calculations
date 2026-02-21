use crate::to_norm_key;
use crate::{
    defaults::{
        LevelScalingRange, heartsteel_colossal_consumption_cooldown_seconds_default,
        luden_echo_cooldown_seconds_default, rune_runtime_defaults,
    },
    scripts::runtime::stat_resolution::{
        CooldownMetricSource, RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
    },
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OnHitEffectProfile {
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct PressTheAttackTargetState {
    stacks: usize,
    stack_expires_at: f64,
    vulnerable_until: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct HitWindowTargetState {
    stacks: usize,
    expires_at: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum RuneProcTriggerSource {
    OnHit,
    Ability,
    Immobilize,
    EnemyKill,
    RuntimeActivation,
}

impl RuneProcTriggerSource {
    fn label(self) -> &'static str {
        match self {
            Self::OnHit => "on_hit",
            Self::Ability => "ability",
            Self::Immobilize => "immobilize",
            Self::EnemyKill => "enemy_kill",
            Self::RuntimeActivation => "runtime_activation",
        }
    }
}

const RUNE_PROC_TRIGGER_SOURCES: [RuneProcTriggerSource; 5] = [
    RuneProcTriggerSource::OnHit,
    RuneProcTriggerSource::Ability,
    RuneProcTriggerSource::Immobilize,
    RuneProcTriggerSource::EnemyKill,
    RuneProcTriggerSource::RuntimeActivation,
];

const MODELED_RUNE_TELEMETRY_KEYS: [&str; 17] = [
    "aftershock",
    "arcanecomet",
    "conqueror",
    "darkharvest",
    "electrocute",
    "firststrike",
    "fleetfootwork",
    "graspoftheundying",
    "phaserush",
    "presstheattack",
    "summonaery",
    "triumph",
    "scorch",
    "cheapshot",
    "tasteofblood",
    "absorblife",
    "coupdegrace",
];

const MODELED_RUNE_TELEMETRY_KEY_COUNT: usize = MODELED_RUNE_TELEMETRY_KEYS.len();
const RUNE_PROC_TRIGGER_SOURCE_COUNT: usize = RUNE_PROC_TRIGGER_SOURCES.len();

#[derive(Debug, Clone, Copy, Default)]
struct RuneProcTelemetrySourceTotals {
    proc_count: usize,
    attempt_count: usize,
    eligible_count: usize,
    bonus_damage: f64,
    bonus_healing: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct RuneProcTelemetryTotals {
    proc_count: usize,
    attempt_count: usize,
    eligible_count: usize,
    bonus_damage: f64,
    bonus_healing: f64,
    by_source: [RuneProcTelemetrySourceTotals; RUNE_PROC_TRIGGER_SOURCE_COUNT],
}

impl RuneProcTelemetryTotals {
    fn has_recorded_activity(self) -> bool {
        self.proc_count > 0
            || self.attempt_count > 0
            || self.eligible_count > 0
            || self.bonus_damage > 0.0
            || self.bonus_healing > 0.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuneProcTelemetrySourceEntry {
    pub source: String,
    pub proc_count: usize,
    pub attempt_count: usize,
    pub eligible_count: usize,
    pub proc_attempt_rate: f64,
    pub proc_eligible_rate: f64,
    pub bonus_damage: f64,
    pub bonus_healing: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RuneProcTelemetryEntry {
    pub rune_name: String,
    pub proc_count: usize,
    pub attempt_count: usize,
    pub eligible_count: usize,
    pub proc_attempt_rate: f64,
    pub proc_eligible_rate: f64,
    pub bonus_damage: f64,
    pub bonus_healing: f64,
    pub source_breakdown: Vec<RuneProcTelemetrySourceEntry>,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutRuntimeState {
    has_lethal_tempo: bool,
    has_grasp: bool,
    has_kraken: bool,
    has_blade_of_the_ruined_king: bool,
    has_heartsteel: bool,
    has_liandry: bool,
    has_luden: bool,
    has_guinsoo: bool,
    has_second_wind: bool,
    has_press_the_attack: bool,
    has_fleet_footwork: bool,
    has_conqueror: bool,
    has_aftershock: bool,
    has_electrocute: bool,
    has_first_strike: bool,
    has_phase_rush: bool,
    has_arcane_comet: bool,
    has_summon_aery: bool,
    has_hail_of_blades: bool,
    has_dark_harvest: bool,
    has_triumph: bool,
    has_gathering_storm: bool,
    has_scorch: bool,
    has_cheap_shot: bool,
    has_taste_of_blood: bool,
    has_absorb_life: bool,
    has_coup_de_grace: bool,
    owner_is_melee: bool,
    rune_proc_telemetry_enabled: bool,

    pub attacks_landed: usize,
    pub lethal_tempo_stacks: usize,
    pub hail_of_blades_remaining_attacks: usize,
    pub guinsoo_stacks: usize,
    pub conqueror_stacks: usize,
    pub dark_harvest_souls: usize,
    pub grasp_cooldown_seconds: f64,
    pub heartsteel_cooldown_seconds: f64,
    pub luden_cooldown_seconds: f64,
    pub grasp_ready_at: f64,
    pub heartsteel_ready_at: f64,
    pub luden_ready_at: f64,
    pub conqueror_expires_at: f64,
    pub fleet_ready_at: f64,
    pub aftershock_ready_at: f64,
    pub aftershock_active_until: f64,
    pub electrocute_ready_at: f64,
    pub first_strike_ready_at: f64,
    pub first_strike_window_until: f64,
    pub phase_rush_ready_at: f64,
    pub phase_rush_active_until: f64,
    pub arcane_comet_ready_at: f64,
    pub summon_aery_ready_at: f64,
    pub hail_of_blades_ready_at: f64,
    pub hail_of_blades_expires_at: f64,
    pub dark_harvest_ready_at: f64,
    pub scorch_ready_at: f64,
    pub cheap_shot_ready_at: f64,
    pub taste_of_blood_ready_at: f64,
    pub pending_fleet_heal: f64,
    pub pending_taste_of_blood_heal: f64,
    press_the_attack_targets: HashMap<usize, PressTheAttackTargetState>,
    electrocute_targets: HashMap<usize, HitWindowTargetState>,
    phase_rush_targets: HashMap<usize, HitWindowTargetState>,
    rune_proc_telemetry_totals: [RuneProcTelemetryTotals; MODELED_RUNE_TELEMETRY_KEY_COUNT],
}

impl Default for LoadoutRuntimeState {
    fn default() -> Self {
        Self {
            has_lethal_tempo: false,
            has_grasp: false,
            has_kraken: false,
            has_blade_of_the_ruined_king: false,
            has_heartsteel: false,
            has_liandry: false,
            has_luden: false,
            has_guinsoo: false,
            has_second_wind: false,
            has_press_the_attack: false,
            has_fleet_footwork: false,
            has_conqueror: false,
            has_aftershock: false,
            has_electrocute: false,
            has_first_strike: false,
            has_phase_rush: false,
            has_arcane_comet: false,
            has_summon_aery: false,
            has_hail_of_blades: false,
            has_dark_harvest: false,
            has_triumph: false,
            has_gathering_storm: false,
            has_scorch: false,
            has_cheap_shot: false,
            has_taste_of_blood: false,
            has_absorb_life: false,
            has_coup_de_grace: false,
            owner_is_melee: false,
            rune_proc_telemetry_enabled: true,
            attacks_landed: 0,
            lethal_tempo_stacks: 0,
            hail_of_blades_remaining_attacks: 0,
            guinsoo_stacks: 0,
            conqueror_stacks: 0,
            dark_harvest_souls: 0,
            grasp_cooldown_seconds: 4.0,
            heartsteel_cooldown_seconds: 0.0,
            luden_cooldown_seconds: 0.0,
            grasp_ready_at: 0.0,
            heartsteel_ready_at: 0.0,
            luden_ready_at: 0.0,
            conqueror_expires_at: 0.0,
            fleet_ready_at: 0.0,
            aftershock_ready_at: 0.0,
            aftershock_active_until: 0.0,
            electrocute_ready_at: 0.0,
            first_strike_ready_at: 0.0,
            first_strike_window_until: 0.0,
            phase_rush_ready_at: 0.0,
            phase_rush_active_until: 0.0,
            arcane_comet_ready_at: 0.0,
            summon_aery_ready_at: 0.0,
            hail_of_blades_ready_at: 0.0,
            hail_of_blades_expires_at: 0.0,
            dark_harvest_ready_at: 0.0,
            scorch_ready_at: 0.0,
            cheap_shot_ready_at: 0.0,
            taste_of_blood_ready_at: 0.0,
            pending_fleet_heal: 0.0,
            pending_taste_of_blood_heal: 0.0,
            press_the_attack_targets: HashMap::new(),
            electrocute_targets: HashMap::new(),
            phase_rush_targets: HashMap::new(),
            rune_proc_telemetry_totals: [RuneProcTelemetryTotals::default();
                MODELED_RUNE_TELEMETRY_KEY_COUNT],
        }
    }
}

fn level_scaled_value(level: usize, min: f64, max: f64) -> f64 {
    let clamped_level = level.clamp(1, 18);
    let t = (clamped_level as f64 - 1.0) / 17.0;
    min + (max - min) * t
}

fn level_scaled_range_value(level: usize, range: LevelScalingRange) -> f64 {
    level_scaled_value(level, range.min, range.max)
}

fn title_case_rune_name(normalized_rune_key: &str) -> String {
    match normalized_rune_key {
        "graspoftheundying" => "Grasp of the Undying".to_string(),
        "lethaltempo" => "Lethal Tempo".to_string(),
        "presstheattack" => "Press the Attack".to_string(),
        "fleetfootwork" => "Fleet Footwork".to_string(),
        "conqueror" => "Conqueror".to_string(),
        "aftershock" => "Aftershock".to_string(),
        "electrocute" => "Electrocute".to_string(),
        "firststrike" => "First Strike".to_string(),
        "phaserush" => "Phase Rush".to_string(),
        "arcanecomet" => "Arcane Comet".to_string(),
        "summonaery" => "Summon Aery".to_string(),
        "hailofblades" => "Hail of Blades".to_string(),
        "darkharvest" => "Dark Harvest".to_string(),
        "triumph" => "Triumph".to_string(),
        "gatheringstorm" => "Gathering Storm".to_string(),
        "scorch" => "Scorch".to_string(),
        "cheapshot" => "Cheap Shot".to_string(),
        "tasteofblood" => "Taste of Blood".to_string(),
        "absorblife" => "Absorb Life".to_string(),
        "coupdegrace" => "Coup de Grace".to_string(),
        _ => normalized_rune_key.to_string(),
    }
}

fn record_rune_proc(
    runtime: &mut LoadoutRuntimeState,
    rune_key: &'static str,
    source: RuneProcTriggerSource,
    damage: f64,
    healing: f64,
) {
    if !runtime.rune_proc_telemetry_enabled {
        return;
    }
    let Some(entry) = rune_telemetry_entry_mut(runtime, rune_key) else {
        return;
    };
    let source_entry = &mut entry.by_source[rune_proc_trigger_source_index(source)];
    entry.proc_count += 1;
    entry.bonus_damage += damage.max(0.0);
    entry.bonus_healing += healing.max(0.0);
    source_entry.proc_count += 1;
    source_entry.bonus_damage += damage.max(0.0);
    source_entry.bonus_healing += healing.max(0.0);
}

fn rune_proc_trigger_source_index(source: RuneProcTriggerSource) -> usize {
    match source {
        RuneProcTriggerSource::OnHit => 0,
        RuneProcTriggerSource::Ability => 1,
        RuneProcTriggerSource::Immobilize => 2,
        RuneProcTriggerSource::EnemyKill => 3,
        RuneProcTriggerSource::RuntimeActivation => 4,
    }
}

fn rune_telemetry_index(rune_key: &str) -> Option<usize> {
    match rune_key {
        "aftershock" => Some(0),
        "arcanecomet" => Some(1),
        "conqueror" => Some(2),
        "darkharvest" => Some(3),
        "electrocute" => Some(4),
        "firststrike" => Some(5),
        "fleetfootwork" => Some(6),
        "graspoftheundying" => Some(7),
        "phaserush" => Some(8),
        "presstheattack" => Some(9),
        "summonaery" => Some(10),
        "triumph" => Some(11),
        "scorch" => Some(12),
        "cheapshot" => Some(13),
        "tasteofblood" => Some(14),
        "absorblife" => Some(15),
        "coupdegrace" => Some(16),
        _ => None,
    }
}

fn rune_telemetry_entry_mut<'a>(
    runtime: &'a mut LoadoutRuntimeState,
    rune_key: &'static str,
) -> Option<&'a mut RuneProcTelemetryTotals> {
    rune_telemetry_index(rune_key).map(|idx| &mut runtime.rune_proc_telemetry_totals[idx])
}

fn increment_rune_counter(
    runtime: &mut LoadoutRuntimeState,
    rune_key: &'static str,
    source: RuneProcTriggerSource,
    increment_entry: impl FnOnce(&mut RuneProcTelemetryTotals),
    increment_source: impl FnOnce(&mut RuneProcTelemetrySourceTotals),
) {
    if !runtime.rune_proc_telemetry_enabled {
        return;
    }
    let Some(entry) = rune_telemetry_entry_mut(runtime, rune_key) else {
        return;
    };
    let source_idx = rune_proc_trigger_source_index(source);
    increment_entry(entry);
    increment_source(&mut entry.by_source[source_idx]);
}

fn record_rune_proc_attempt(
    runtime: &mut LoadoutRuntimeState,
    rune_key: &'static str,
    source: RuneProcTriggerSource,
) {
    increment_rune_counter(
        runtime,
        rune_key,
        source,
        |entry| entry.attempt_count += 1,
        |source_entry| source_entry.attempt_count += 1,
    );
}

fn record_rune_proc_eligibility(
    runtime: &mut LoadoutRuntimeState,
    rune_key: &'static str,
    source: RuneProcTriggerSource,
) {
    increment_rune_counter(
        runtime,
        rune_key,
        source,
        |entry| entry.eligible_count += 1,
        |source_entry| source_entry.eligible_count += 1,
    );
}

fn accumulate_window_stacks(
    stacks_by_target: &mut HashMap<usize, HitWindowTargetState>,
    target_idx: usize,
    now: f64,
    window_seconds: f64,
    max_stacks: usize,
) -> usize {
    let state = stacks_by_target.entry(target_idx).or_default();
    if now > state.expires_at {
        state.stacks = 0;
    }
    state.stacks = (state.stacks + 1).min(max_stacks);
    state.expires_at = now + window_seconds.max(0.0);
    state.stacks
}

fn decay_expired_conqueror_stacks(runtime: &mut LoadoutRuntimeState, now: f64) {
    if runtime.has_conqueror && now > runtime.conqueror_expires_at {
        runtime.conqueror_stacks = 0;
    }
}

fn add_conqueror_stacks(runtime: &mut LoadoutRuntimeState, stacks: usize, now: f64) {
    if !runtime.has_conqueror || stacks == 0 {
        return;
    }
    let defaults = &rune_runtime_defaults().conqueror;
    decay_expired_conqueror_stacks(runtime, now);
    runtime.conqueror_stacks = (runtime.conqueror_stacks + stacks).min(defaults.max_stacks.max(1));
    runtime.conqueror_expires_at = now + defaults.stack_duration_seconds.max(0.0);
}

fn maybe_apply_first_strike(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    raw_damage: f64,
    attacker_level: usize,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().first_strike;
    if !runtime.has_first_strike {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "firststrike", source);
    if now >= runtime.first_strike_ready_at && now > runtime.first_strike_window_until {
        runtime.first_strike_window_until = now + defaults.window_duration_seconds.max(0.0);
        runtime.first_strike_ready_at =
            now + level_scaled_range_value(attacker_level, defaults.cooldown_by_level).max(0.0);
    }
    if now > runtime.first_strike_window_until {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "firststrike", source);
    let bonus_true = defaults.bonus_true_damage_ratio.max(0.0) * raw_damage.max(0.0);
    if bonus_true > 0.0 {
        record_rune_proc(runtime, "firststrike", source, bonus_true, 0.0);
    }
    bonus_true
}

fn maybe_apply_electrocute(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().electrocute;
    if !runtime.has_electrocute {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "electrocute", source);
    if now < runtime.electrocute_ready_at {
        return 0.0;
    }
    let Some(target_idx) = target_id else {
        return 0.0;
    };
    let stacks = accumulate_window_stacks(
        &mut runtime.electrocute_targets,
        target_idx,
        now,
        defaults.hit_window_seconds,
        defaults.hits_to_proc.max(1),
    );
    if stacks < defaults.hits_to_proc.max(1) {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "electrocute", source);
    runtime.electrocute_ready_at =
        now + level_scaled_range_value(attacker_level, defaults.cooldown_by_level).max(0.0);
    if let Some(state) = runtime.electrocute_targets.get_mut(&target_idx) {
        state.stacks = 0;
        state.expires_at = now;
    }
    let damage = level_scaled_range_value(attacker_level, defaults.proc_magic_damage_by_level);
    record_rune_proc(runtime, "electrocute", source, damage, 0.0);
    damage
}

fn maybe_apply_phase_rush(runtime: &mut LoadoutRuntimeState, now: f64, target_id: Option<usize>) {
    let defaults = &rune_runtime_defaults().phase_rush;
    if !runtime.has_phase_rush {
        return;
    }
    record_rune_proc_attempt(
        runtime,
        "phaserush",
        RuneProcTriggerSource::RuntimeActivation,
    );
    if now < runtime.phase_rush_ready_at {
        return;
    }
    let Some(target_idx) = target_id else {
        return;
    };
    let stacks = accumulate_window_stacks(
        &mut runtime.phase_rush_targets,
        target_idx,
        now,
        defaults.hit_window_seconds,
        defaults.hits_to_proc.max(1),
    );
    if stacks < defaults.hits_to_proc.max(1) {
        return;
    }
    record_rune_proc_eligibility(
        runtime,
        "phaserush",
        RuneProcTriggerSource::RuntimeActivation,
    );
    runtime.phase_rush_ready_at = now + defaults.cooldown_seconds.max(0.0);
    runtime.phase_rush_active_until = now + defaults.active_duration_seconds.max(0.0);
    if let Some(state) = runtime.phase_rush_targets.get_mut(&target_idx) {
        state.stacks = 0;
        state.expires_at = now;
    }
    record_rune_proc(
        runtime,
        "phaserush",
        RuneProcTriggerSource::RuntimeActivation,
        0.0,
        0.0,
    );
}

fn gathering_storm_bonus_ability_power(runtime: &LoadoutRuntimeState, now: f64) -> f64 {
    if !runtime.has_gathering_storm {
        return 0.0;
    }
    let defaults = &rune_runtime_defaults().gathering_storm;
    let interval = defaults.interval_seconds.max(1.0);
    let intervals_completed = (now / interval).floor().max(0.0) as usize;
    if intervals_completed == 0 {
        return 0.0;
    }
    defaults
        .ability_power_by_interval
        .get(intervals_completed - 1)
        .copied()
        .or_else(|| defaults.ability_power_by_interval.last().copied())
        .unwrap_or(0.0)
}

fn update_hail_of_blades_state(runtime: &mut LoadoutRuntimeState, now: f64) {
    if !runtime.has_hail_of_blades {
        return;
    }
    let defaults = &rune_runtime_defaults().hail_of_blades;
    if runtime.hail_of_blades_remaining_attacks > 0 && now > runtime.hail_of_blades_expires_at {
        runtime.hail_of_blades_remaining_attacks = 0;
        runtime.hail_of_blades_expires_at = 0.0;
        runtime.hail_of_blades_ready_at = now + defaults.cooldown_seconds.max(0.0);
    }
    if runtime.hail_of_blades_remaining_attacks == 0 && now >= runtime.hail_of_blades_ready_at {
        runtime.hail_of_blades_remaining_attacks = defaults.empowered_attack_count.max(1);
        runtime.hail_of_blades_expires_at = now + defaults.active_duration_seconds.max(0.0);
    }
    if runtime.hail_of_blades_remaining_attacks > 0 {
        runtime.hail_of_blades_remaining_attacks -= 1;
        runtime.hail_of_blades_expires_at = now + defaults.active_duration_seconds.max(0.0);
        if runtime.hail_of_blades_remaining_attacks == 0 {
            runtime.hail_of_blades_ready_at = now + defaults.cooldown_seconds.max(0.0);
        }
    }
}

fn maybe_apply_arcane_comet(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    attacker_level: usize,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().arcane_comet;
    if !runtime.has_arcane_comet {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "arcanecomet", RuneProcTriggerSource::Ability);
    if now < runtime.arcane_comet_ready_at {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "arcanecomet", RuneProcTriggerSource::Ability);
    runtime.arcane_comet_ready_at =
        now + level_scaled_range_value(attacker_level, defaults.cooldown_by_level).max(0.0);
    let damage = level_scaled_range_value(attacker_level, defaults.proc_magic_damage_by_level)
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    if damage > 0.0 {
        record_rune_proc(
            runtime,
            "arcanecomet",
            RuneProcTriggerSource::Ability,
            damage,
            0.0,
        );
    }
    damage
}

fn maybe_apply_summon_aery(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    attacker_level: usize,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().summon_aery;
    if !runtime.has_summon_aery {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "summonaery", source);
    if now < runtime.summon_aery_ready_at {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "summonaery", source);
    runtime.summon_aery_ready_at = now + defaults.cooldown_seconds.max(0.0);
    let damage = level_scaled_range_value(attacker_level, defaults.proc_magic_damage_by_level)
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    if damage > 0.0 {
        record_rune_proc(runtime, "summonaery", source, damage, 0.0);
    }
    damage
}

fn maybe_apply_dark_harvest(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    source: RuneProcTriggerSource,
) -> f64 {
    let defaults = &rune_runtime_defaults().dark_harvest;
    if !runtime.has_dark_harvest {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "darkharvest", source);
    if now < runtime.dark_harvest_ready_at || target_max_health <= 0.0 {
        return 0.0;
    }
    let health_ratio = (target_current_health / target_max_health).clamp(0.0, 1.0);
    if health_ratio > defaults.trigger_health_ratio {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "darkharvest", source);
    runtime.dark_harvest_ready_at = now + defaults.cooldown_seconds.max(0.0);
    let damage = defaults.base_magic_damage.max(0.0)
        + defaults.soul_magic_damage.max(0.0) * runtime.dark_harvest_souls as f64
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    runtime.dark_harvest_souls = runtime.dark_harvest_souls.saturating_add(1);
    if damage > 0.0 {
        record_rune_proc(runtime, "darkharvest", source, damage, 0.0);
    }
    damage
}

fn press_the_attack_damage_multiplier(
    runtime: &LoadoutRuntimeState,
    target_id: Option<usize>,
    now: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().press_the_attack;
    if !runtime.has_press_the_attack {
        return 0.0;
    }
    target_id
        .and_then(|idx| runtime.press_the_attack_targets.get(&idx))
        .filter(|state| now <= state.vulnerable_until)
        .map(|_| defaults.vulnerability_true_damage_ratio.max(0.0))
        .unwrap_or(0.0)
}

#[cfg(test)]
pub(crate) fn build_loadout_runtime_state(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
) -> LoadoutRuntimeState {
    build_loadout_runtime_state_with_telemetry(
        item_names,
        rune_names,
        item_haste,
        owner_is_melee,
        true,
    )
}

pub(crate) fn build_loadout_runtime_state_with_telemetry(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
    rune_proc_telemetry_enabled: bool,
) -> LoadoutRuntimeState {
    let rune_defaults = rune_runtime_defaults();
    let mut runtime = LoadoutRuntimeState {
        owner_is_melee,
        rune_proc_telemetry_enabled,
        ..LoadoutRuntimeState::default()
    };
    let clamped_item_haste = item_haste.max(-99.0);
    let item_buff_state = RuntimeBuffState {
        item_haste: clamped_item_haste,
        ..RuntimeBuffState::default()
    };
    runtime.heartsteel_cooldown_seconds = resolve_stat(
        StatQuery::CooldownSeconds {
            base_seconds: heartsteel_colossal_consumption_cooldown_seconds_default(),
            source: CooldownMetricSource::Item,
        },
        item_buff_state,
    );
    runtime.luden_cooldown_seconds = resolve_stat(
        StatQuery::CooldownSeconds {
            base_seconds: luden_echo_cooldown_seconds_default(),
            source: CooldownMetricSource::Item,
        },
        item_buff_state,
    );
    runtime.grasp_cooldown_seconds = resolve_stat(
        StatQuery::CooldownSeconds {
            base_seconds: rune_defaults.grasp_of_the_undying.cooldown_seconds,
            source: CooldownMetricSource::Neutral,
        },
        RuntimeBuffState::default(),
    );

    for item in item_names {
        match to_norm_key(item).as_str() {
            "krakenslayer" => runtime.has_kraken = true,
            "bladeoftheruinedking" => runtime.has_blade_of_the_ruined_king = true,
            "heartsteel" => runtime.has_heartsteel = true,
            "liandrystorment" => runtime.has_liandry = true,
            "ludensecho" => runtime.has_luden = true,
            "guinsoosrageblade" => runtime.has_guinsoo = true,
            _ => {}
        }
    }

    for rune in rune_names {
        match to_norm_key(rune).as_str() {
            "lethaltempo" => runtime.has_lethal_tempo = true,
            "graspoftheundying" => runtime.has_grasp = true,
            "secondwind" => runtime.has_second_wind = true,
            "presstheattack" => runtime.has_press_the_attack = true,
            "fleetfootwork" => runtime.has_fleet_footwork = true,
            "conqueror" => runtime.has_conqueror = true,
            "aftershock" => runtime.has_aftershock = true,
            "electrocute" => runtime.has_electrocute = true,
            "firststrike" => runtime.has_first_strike = true,
            "phaserush" => runtime.has_phase_rush = true,
            "arcanecomet" => runtime.has_arcane_comet = true,
            "summonaery" => runtime.has_summon_aery = true,
            "hailofblades" => runtime.has_hail_of_blades = true,
            "darkharvest" => runtime.has_dark_harvest = true,
            "triumph" => runtime.has_triumph = true,
            "gatheringstorm" => runtime.has_gathering_storm = true,
            "scorch" => runtime.has_scorch = true,
            "cheapshot" => runtime.has_cheap_shot = true,
            "tasteofblood" => runtime.has_taste_of_blood = true,
            "absorblife" => runtime.has_absorb_life = true,
            "coupdegrace" => runtime.has_coup_de_grace = true,
            _ => {}
        }
    }

    runtime
}

pub(crate) fn loadout_attack_speed_multiplier(runtime: &LoadoutRuntimeState, now: f64) -> f64 {
    let rune_defaults = rune_runtime_defaults();
    let lethal_tempo_bonus = if runtime.has_lethal_tempo {
        rune_defaults.lethal_tempo.attack_speed_per_stack.max(0.0)
            * runtime.lethal_tempo_stacks as f64
    } else {
        0.0
    };
    let guinsoo_bonus = if runtime.has_guinsoo {
        0.02 * runtime.guinsoo_stacks as f64
    } else {
        0.0
    };
    let hail_of_blades_bonus = if runtime.has_hail_of_blades
        && runtime.hail_of_blades_remaining_attacks > 0
        && now <= runtime.hail_of_blades_expires_at
    {
        if runtime.owner_is_melee {
            rune_defaults
                .hail_of_blades
                .bonus_attack_speed_ratio_melee
                .max(0.0)
        } else {
            rune_defaults
                .hail_of_blades
                .bonus_attack_speed_ratio_ranged
                .max(0.0)
        }
    } else {
        0.0
    };
    1.0 + lethal_tempo_bonus + guinsoo_bonus + hail_of_blades_bonus
}

pub(crate) fn reset_transient_loadout_state(runtime: &mut LoadoutRuntimeState) {
    runtime.attacks_landed = 0;
    runtime.lethal_tempo_stacks = 0;
    runtime.hail_of_blades_remaining_attacks = 0;
    runtime.hail_of_blades_expires_at = 0.0;
    runtime.guinsoo_stacks = 0;
    runtime.conqueror_stacks = 0;
    runtime.conqueror_expires_at = 0.0;
    runtime.pending_fleet_heal = 0.0;
    runtime.press_the_attack_targets.clear();
    runtime.electrocute_targets.clear();
    runtime.phase_rush_targets.clear();
    runtime.rune_proc_telemetry_totals =
        [RuneProcTelemetryTotals::default(); MODELED_RUNE_TELEMETRY_KEY_COUNT];
    runtime.aftershock_active_until = 0.0;
    runtime.first_strike_window_until = 0.0;
    runtime.scorch_ready_at = 0.0;
    runtime.cheap_shot_ready_at = 0.0;
    runtime.taste_of_blood_ready_at = 0.0;
    runtime.pending_taste_of_blood_heal = 0.0;
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn calculate_on_hit_bonus_damage(
    profile: OnHitEffectProfile,
    runtime: &mut LoadoutRuntimeState,
    attack_damage: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64, f64) {
    let defaults = rune_runtime_defaults();
    decay_expired_conqueror_stacks(runtime, now);
    runtime.attacks_landed += 1;
    if runtime.has_lethal_tempo {
        runtime.lethal_tempo_stacks =
            (runtime.lethal_tempo_stacks + 1).min(defaults.lethal_tempo.max_stacks.max(1));
    }
    if runtime.has_guinsoo {
        runtime.guinsoo_stacks = (runtime.guinsoo_stacks + 1).min(8);
    }
    update_hail_of_blades_state(runtime, now);

    let magic = profile.on_hit_magic_flat + profile.on_hit_magic_ad_ratio * attack_damage;
    let mut extra_physical = 0.0;
    let mut extra_magic = magic.max(0.0);
    let mut extra_true = 0.0;

    maybe_apply_phase_rush(runtime, now, target_id);
    extra_true += maybe_apply_first_strike(
        runtime,
        now,
        attack_damage,
        attacker_level,
        RuneProcTriggerSource::OnHit,
    );
    if runtime.has_press_the_attack {
        record_rune_proc_attempt(runtime, "presstheattack", RuneProcTriggerSource::OnHit);
    }
    let pta_multiplier = press_the_attack_damage_multiplier(runtime, target_id, now);
    if pta_multiplier > 0.0 {
        record_rune_proc_eligibility(runtime, "presstheattack", RuneProcTriggerSource::OnHit);
        let pta_bonus_true = pta_multiplier * attack_damage.max(0.0);
        extra_true += pta_bonus_true;
        if pta_bonus_true > 0.0 {
            record_rune_proc(
                runtime,
                "presstheattack",
                RuneProcTriggerSource::OnHit,
                pta_bonus_true,
                0.0,
            );
        }
    }

    if profile.periodic_true_hit_every > 0
        && runtime
            .attacks_landed
            .is_multiple_of(profile.periodic_true_hit_every)
    {
        extra_true += profile.periodic_true_hit_base
            + profile.periodic_true_hit_ad_ratio * attack_damage
            + profile.periodic_true_hit_target_max_health_ratio * target_max_health;
    }

    if runtime.has_blade_of_the_ruined_king {
        extra_physical += 0.06 * target_current_health.max(0.0);
    }

    if runtime.has_kraken && runtime.attacks_landed.is_multiple_of(3) {
        extra_true += 65.0 + 0.45 * attack_damage;
    }

    if runtime.has_grasp {
        record_rune_proc_attempt(runtime, "graspoftheundying", RuneProcTriggerSource::OnHit);
    }
    if runtime.has_grasp && now >= runtime.grasp_ready_at {
        record_rune_proc_eligibility(runtime, "graspoftheundying", RuneProcTriggerSource::OnHit);
        let grasp_damage = defaults.grasp_of_the_undying.base_magic_damage.max(0.0)
            + defaults
                .grasp_of_the_undying
                .target_max_health_ratio
                .max(0.0)
                * target_max_health.max(0.0);
        extra_magic += grasp_damage;
        runtime.grasp_ready_at = now + runtime.grasp_cooldown_seconds;
        record_rune_proc(
            runtime,
            "graspoftheundying",
            RuneProcTriggerSource::OnHit,
            grasp_damage,
            0.0,
        );
    }

    if runtime.has_heartsteel && now >= runtime.heartsteel_ready_at {
        extra_physical += 70.0 + 0.06 * attacker_max_health.max(0.0);
        runtime.heartsteel_ready_at = now + runtime.heartsteel_cooldown_seconds;
    }
    if runtime.has_fleet_footwork {
        record_rune_proc_attempt(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
    }
    if runtime.has_fleet_footwork && now >= runtime.fleet_ready_at {
        record_rune_proc_eligibility(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
        runtime.pending_fleet_heal +=
            level_scaled_range_value(attacker_level, defaults.fleet_footwork.heal_by_level)
                + defaults.fleet_footwork.attack_damage_ratio.max(0.0) * attack_damage.max(0.0);
        runtime.fleet_ready_at = now + defaults.fleet_footwork.cooldown_seconds.max(0.0);
    }
    if runtime.has_press_the_attack
        && let Some(target_idx) = target_id
    {
        let state = runtime
            .press_the_attack_targets
            .entry(target_idx)
            .or_default();
        if now > state.stack_expires_at {
            state.stacks = 0;
        }
        state.stacks = (state.stacks + 1).min(3);
        state.stack_expires_at = now + defaults.press_the_attack.stack_window_seconds.max(0.0);
        if state.stacks >= 3 {
            let pta_burst_damage = level_scaled_range_value(
                attacker_level,
                defaults.press_the_attack.burst_magic_damage_by_level,
            );
            extra_magic += pta_burst_damage;
            state.stacks = 0;
            state.vulnerable_until = now
                + defaults
                    .press_the_attack
                    .vulnerability_duration_seconds
                    .max(0.0);
            record_rune_proc_eligibility(runtime, "presstheattack", RuneProcTriggerSource::OnHit);
            record_rune_proc(
                runtime,
                "presstheattack",
                RuneProcTriggerSource::OnHit,
                pta_burst_damage,
                0.0,
            );
        }
    }
    if runtime.has_conqueror {
        let basic_attack_stacks = if runtime.owner_is_melee {
            defaults.conqueror.melee_basic_attack_stacks
        } else {
            defaults.conqueror.ranged_basic_attack_stacks
        };
        add_conqueror_stacks(runtime, basic_attack_stacks, now);
    }
    let electrocute_damage = maybe_apply_electrocute(
        runtime,
        now,
        target_id,
        attacker_level,
        RuneProcTriggerSource::OnHit,
    );
    extra_magic += electrocute_damage;
    extra_magic += maybe_apply_summon_aery(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::OnHit,
    );
    extra_magic += maybe_apply_dark_harvest(
        runtime,
        now,
        target_current_health,
        target_max_health,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::OnHit,
    );

    extra_true += maybe_apply_coup_de_grace_bonus(
        runtime,
        target_current_health,
        target_max_health,
        attack_damage,
        RuneProcTriggerSource::OnHit,
    );

    (
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_physical,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_magic,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_true,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
    )
}

fn maybe_apply_coup_de_grace_bonus(
    runtime: &mut LoadoutRuntimeState,
    target_current_health: f64,
    target_max_health: f64,
    source_damage: f64,
    source: RuneProcTriggerSource,
) -> f64 {
    if !runtime.has_coup_de_grace {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "coupdegrace", source);
    if target_max_health <= 0.0 {
        return 0.0;
    }
    let defaults = &rune_runtime_defaults().coup_de_grace;
    if target_current_health > defaults.trigger_health_ratio.max(0.0) * target_max_health {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "coupdegrace", source);
    let bonus = defaults.bonus_damage_ratio.max(0.0) * source_damage.max(0.0);
    if bonus > 0.0 {
        record_rune_proc(runtime, "coupdegrace", source, bonus, 0.0);
    }
    bonus
}

fn maybe_apply_scorch(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    attacker_level: usize,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
) -> f64 {
    if !runtime.has_scorch {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "scorch", RuneProcTriggerSource::Ability);
    if now < runtime.scorch_ready_at {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "scorch", RuneProcTriggerSource::Ability);
    let defaults = &rune_runtime_defaults().scorch;
    runtime.scorch_ready_at = now + defaults.cooldown_seconds.max(0.0);
    let damage = level_scaled_range_value(attacker_level, defaults.proc_magic_damage_by_level)
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    if damage > 0.0 {
        record_rune_proc(
            runtime,
            "scorch",
            RuneProcTriggerSource::Ability,
            damage,
            0.0,
        );
    }
    damage.max(0.0)
}

fn maybe_trigger_taste_of_blood(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    attacker_level: usize,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
) {
    if !runtime.has_taste_of_blood {
        return;
    }
    record_rune_proc_attempt(
        runtime,
        "tasteofblood",
        RuneProcTriggerSource::RuntimeActivation,
    );
    if now < runtime.taste_of_blood_ready_at {
        return;
    }
    record_rune_proc_eligibility(
        runtime,
        "tasteofblood",
        RuneProcTriggerSource::RuntimeActivation,
    );
    let defaults = &rune_runtime_defaults().taste_of_blood;
    runtime.taste_of_blood_ready_at = now + defaults.cooldown_seconds.max(0.0);
    let heal = level_scaled_range_value(attacker_level, defaults.heal_by_level)
        + defaults.ability_power_ratio.max(0.0) * attacker_ability_power.max(0.0)
        + defaults.bonus_attack_damage_ratio.max(0.0) * attacker_bonus_attack_damage.max(0.0);
    runtime.pending_taste_of_blood_heal += heal.max(0.0);
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn calculate_ability_bonus_damage(
    runtime: &mut LoadoutRuntimeState,
    ability_raw_damage: f64,
    ability_ap_ratio: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64) {
    let defaults = rune_runtime_defaults();
    decay_expired_conqueror_stacks(runtime, now);
    let mut extra_magic = 0.0;
    let mut extra_true = 0.0_f64;
    maybe_apply_phase_rush(runtime, now, target_id);
    extra_true += maybe_apply_first_strike(
        runtime,
        now,
        ability_raw_damage,
        attacker_level,
        RuneProcTriggerSource::Ability,
    );
    if runtime.has_press_the_attack {
        record_rune_proc_attempt(runtime, "presstheattack", RuneProcTriggerSource::Ability);
    }
    let pta_multiplier = press_the_attack_damage_multiplier(runtime, target_id, now);
    if pta_multiplier > 0.0 {
        record_rune_proc_eligibility(runtime, "presstheattack", RuneProcTriggerSource::Ability);
        let pta_bonus_true = pta_multiplier * ability_raw_damage.max(0.0);
        extra_true += pta_bonus_true;
        if pta_bonus_true > 0.0 {
            record_rune_proc(
                runtime,
                "presstheattack",
                RuneProcTriggerSource::Ability,
                pta_bonus_true,
                0.0,
            );
        }
    }
    if runtime.has_conqueror {
        record_rune_proc_attempt(runtime, "conqueror", RuneProcTriggerSource::Ability);
        add_conqueror_stacks(runtime, defaults.conqueror.ability_hit_stacks, now);
        record_rune_proc_eligibility(runtime, "conqueror", RuneProcTriggerSource::Ability);
        let adaptive_ability_power = level_scaled_range_value(
            attacker_level,
            defaults.conqueror.adaptive_ability_power_per_stack_by_level,
        ) * runtime.conqueror_stacks as f64;
        let conqueror_damage = adaptive_ability_power * ability_ap_ratio.max(0.0);
        extra_magic += conqueror_damage;
        if conqueror_damage > 0.0 {
            record_rune_proc(
                runtime,
                "conqueror",
                RuneProcTriggerSource::Ability,
                conqueror_damage,
                0.0,
            );
        }
    }
    extra_magic += ability_ap_ratio.max(0.0) * gathering_storm_bonus_ability_power(runtime, now);

    if runtime.has_liandry {
        extra_magic += 0.04 * target_max_health.max(0.0);
    }

    if runtime.has_luden && now >= runtime.luden_ready_at {
        extra_magic += 90.0 + 0.10 * ability_raw_damage.max(0.0);
        runtime.luden_ready_at = now + runtime.luden_cooldown_seconds;
    }
    let electrocute_damage = maybe_apply_electrocute(
        runtime,
        now,
        target_id,
        attacker_level,
        RuneProcTriggerSource::Ability,
    );
    extra_magic += electrocute_damage;
    extra_magic += maybe_apply_arcane_comet(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
    );
    extra_magic += maybe_apply_summon_aery(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::Ability,
    );
    extra_magic += maybe_apply_dark_harvest(
        runtime,
        now,
        target_current_health,
        target_max_health,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        RuneProcTriggerSource::Ability,
    );

    extra_magic += maybe_apply_scorch(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
    );

    maybe_trigger_taste_of_blood(
        runtime,
        now,
        attacker_level,
        attacker_ability_power,
        attacker_bonus_attack_damage,
    );

    extra_true += maybe_apply_coup_de_grace_bonus(
        runtime,
        target_current_health,
        target_max_health,
        ability_raw_damage,
        RuneProcTriggerSource::Ability,
    );

    (
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_magic,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_true,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
    )
}

pub(crate) fn on_outgoing_damage_heal(
    runtime: &mut LoadoutRuntimeState,
    damage_dealt: f64,
    now: f64,
) -> f64 {
    let defaults = rune_runtime_defaults();
    decay_expired_conqueror_stacks(runtime, now);
    if runtime.has_fleet_footwork {
        record_rune_proc_attempt(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
    }
    if runtime.has_conqueror {
        record_rune_proc_attempt(runtime, "conqueror", RuneProcTriggerSource::OnHit);
    }
    let fleet_heal = runtime.pending_fleet_heal.max(0.0);
    runtime.pending_fleet_heal = 0.0;
    let taste_of_blood_heal = runtime.pending_taste_of_blood_heal.max(0.0);
    runtime.pending_taste_of_blood_heal = 0.0;
    let mut heal = fleet_heal + taste_of_blood_heal;
    if fleet_heal > 0.0 {
        record_rune_proc_eligibility(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
        record_rune_proc(
            runtime,
            "fleetfootwork",
            RuneProcTriggerSource::OnHit,
            0.0,
            fleet_heal,
        );
    }
    if taste_of_blood_heal > 0.0 {
        record_rune_proc(
            runtime,
            "tasteofblood",
            RuneProcTriggerSource::RuntimeActivation,
            0.0,
            taste_of_blood_heal,
        );
    }
    if runtime.has_conqueror
        && runtime.conqueror_stacks >= defaults.conqueror.max_stacks.max(1)
        && now <= runtime.conqueror_expires_at
        && damage_dealt > 0.0
    {
        record_rune_proc_eligibility(runtime, "conqueror", RuneProcTriggerSource::OnHit);
        let conqueror_heal_ratio = if runtime.owner_is_melee {
            defaults.conqueror.melee_heal_ratio.max(0.0)
        } else {
            defaults.conqueror.ranged_heal_ratio.max(0.0)
        };
        let conqueror_heal = damage_dealt.max(0.0) * conqueror_heal_ratio;
        heal += conqueror_heal;
        if conqueror_heal > 0.0 {
            record_rune_proc(
                runtime,
                "conqueror",
                RuneProcTriggerSource::OnHit,
                0.0,
                conqueror_heal,
            );
        }
    }
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: heal,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn on_enemy_kill_heal(
    runtime: &mut LoadoutRuntimeState,
    max_health: f64,
    actor_level: usize,
) -> f64 {
    let defaults = rune_runtime_defaults();
    let mut heal = 0.0;

    if runtime.has_triumph && max_health > 0.0 {
        record_rune_proc_attempt(runtime, "triumph", RuneProcTriggerSource::EnemyKill);
        let triumph_heal = defaults.triumph.heal_max_health_ratio.max(0.0) * max_health.max(0.0);
        if triumph_heal > 0.0 {
            heal += triumph_heal;
            record_rune_proc_eligibility(runtime, "triumph", RuneProcTriggerSource::EnemyKill);
            record_rune_proc(
                runtime,
                "triumph",
                RuneProcTriggerSource::EnemyKill,
                0.0,
                triumph_heal,
            );
        }
    }

    if runtime.has_absorb_life {
        record_rune_proc_attempt(runtime, "absorblife", RuneProcTriggerSource::EnemyKill);
        let absorb_life_heal =
            level_scaled_range_value(actor_level, defaults.absorb_life.heal_by_level).max(0.0);
        if absorb_life_heal > 0.0 {
            heal += absorb_life_heal;
            record_rune_proc_eligibility(runtime, "absorblife", RuneProcTriggerSource::EnemyKill);
            record_rune_proc(
                runtime,
                "absorblife",
                RuneProcTriggerSource::EnemyKill,
                0.0,
                absorb_life_heal,
            );
        }
    }

    if heal <= 0.0 {
        return 0.0;
    }

    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: heal,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn trigger_immobilize_rune_damage(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    actor_bonus_health: f64,
) -> f64 {
    let aftershock_defaults = &rune_runtime_defaults().aftershock;
    let cheap_shot_defaults = &rune_runtime_defaults().cheap_shot;
    let mut bonus_damage = 0.0;

    if runtime.has_aftershock {
        record_rune_proc_attempt(runtime, "aftershock", RuneProcTriggerSource::Immobilize);
        if now >= runtime.aftershock_ready_at {
            record_rune_proc_eligibility(runtime, "aftershock", RuneProcTriggerSource::Immobilize);
            runtime.aftershock_ready_at = now + aftershock_defaults.cooldown_seconds.max(0.0);
            runtime.aftershock_active_until =
                now + aftershock_defaults.active_duration_seconds.max(0.0);
            let shockwave_magic = level_scaled_range_value(
                actor_level,
                aftershock_defaults.shockwave_magic_damage_by_level,
            ) + aftershock_defaults.shockwave_bonus_health_ratio.max(0.0)
                * actor_bonus_health.max(0.0);
            if shockwave_magic > 0.0 {
                bonus_damage += shockwave_magic;
                record_rune_proc(
                    runtime,
                    "aftershock",
                    RuneProcTriggerSource::Immobilize,
                    shockwave_magic,
                    0.0,
                );
            }
        }
    }

    if runtime.has_cheap_shot {
        record_rune_proc_attempt(runtime, "cheapshot", RuneProcTriggerSource::Immobilize);
        if now >= runtime.cheap_shot_ready_at {
            record_rune_proc_eligibility(runtime, "cheapshot", RuneProcTriggerSource::Immobilize);
            runtime.cheap_shot_ready_at = now + cheap_shot_defaults.cooldown_seconds.max(0.0);
            let cheap_shot_true = level_scaled_range_value(
                actor_level,
                cheap_shot_defaults.proc_true_damage_by_level,
            );
            if cheap_shot_true > 0.0 {
                bonus_damage += cheap_shot_true;
                record_rune_proc(
                    runtime,
                    "cheapshot",
                    RuneProcTriggerSource::Immobilize,
                    cheap_shot_true,
                    0.0,
                );
            }
        }
    }

    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: bonus_damage,
            source: ScalarMetricSource::OutgoingAbilityDamage,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn loadout_incoming_damage_multipliers(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    current_armor: f64,
    current_magic_resist: f64,
    bonus_armor: f64,
    bonus_magic_resist: f64,
) -> (f64, f64) {
    let defaults = &rune_runtime_defaults().aftershock;
    if !runtime.has_aftershock || now > runtime.aftershock_active_until {
        return (1.0, 1.0);
    }
    let cap = level_scaled_range_value(actor_level, defaults.resist_cap_by_level);
    let bonus_armor_gain =
        (defaults.resist_base + defaults.resist_bonus_ratio * bonus_armor.max(0.0)).min(cap);
    let bonus_magic_resist_gain =
        (defaults.resist_base + defaults.resist_bonus_ratio * bonus_magic_resist.max(0.0)).min(cap);

    let armor = current_armor.max(0.0);
    let magic_resist = current_magic_resist.max(0.0);
    let physical_multiplier_before = 100.0 / (100.0 + armor);
    let magic_multiplier_before = 100.0 / (100.0 + magic_resist);
    let physical_multiplier_after = 100.0 / (100.0 + armor + bonus_armor_gain.max(0.0));
    let magic_multiplier_after = 100.0 / (100.0 + magic_resist + bonus_magic_resist_gain.max(0.0));

    (
        (physical_multiplier_after / physical_multiplier_before).clamp(0.0, 1.0),
        (magic_multiplier_after / magic_multiplier_before).clamp(0.0, 1.0),
    )
}

pub(crate) fn loadout_movement_speed_multiplier(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
) -> f64 {
    if !runtime.has_phase_rush || now > runtime.phase_rush_active_until {
        return 1.0;
    }
    1.0 + level_scaled_range_value(
        actor_level,
        rune_runtime_defaults()
            .phase_rush
            .movement_speed_bonus_ratio_by_level,
    )
}

pub(crate) fn rune_proc_telemetry(runtime: &LoadoutRuntimeState) -> Vec<RuneProcTelemetryEntry> {
    let mut entries = MODELED_RUNE_TELEMETRY_KEYS
        .iter()
        .enumerate()
        .filter_map(|(rune_idx, rune_key)| {
            let totals = runtime.rune_proc_telemetry_totals[rune_idx];
            if !totals.has_recorded_activity() {
                return None;
            }
            let mut source_breakdown = RUNE_PROC_TRIGGER_SOURCES
                .iter()
                .enumerate()
                .filter_map(|(source_idx, source)| {
                    let source_totals = totals.by_source[source_idx];
                    if source_totals.proc_count == 0
                        && source_totals.attempt_count == 0
                        && source_totals.eligible_count == 0
                        && source_totals.bonus_damage <= 0.0
                        && source_totals.bonus_healing <= 0.0
                    {
                        return None;
                    }
                    let mut attempt_count = source_totals
                        .attempt_count
                        .max(source_totals.eligible_count)
                        .max(source_totals.proc_count);
                    let mut eligible_count =
                        source_totals.eligible_count.max(source_totals.proc_count);
                    attempt_count = attempt_count.max(eligible_count);
                    eligible_count = eligible_count.min(attempt_count);
                    let proc_attempt_rate = if attempt_count > 0 {
                        source_totals.proc_count as f64 / attempt_count as f64
                    } else {
                        0.0
                    };
                    let proc_eligible_rate = if eligible_count > 0 {
                        source_totals.proc_count as f64 / eligible_count as f64
                    } else {
                        0.0
                    };
                    Some(RuneProcTelemetrySourceEntry {
                        source: source.label().to_string(),
                        proc_count: source_totals.proc_count,
                        attempt_count,
                        eligible_count,
                        proc_attempt_rate,
                        proc_eligible_rate,
                        bonus_damage: source_totals.bonus_damage,
                        bonus_healing: source_totals.bonus_healing,
                    })
                })
                .collect::<Vec<_>>();
            source_breakdown.sort_by(|a, b| a.source.cmp(&b.source));
            let mut attempt_count = totals
                .attempt_count
                .max(totals.eligible_count)
                .max(totals.proc_count);
            let mut eligible_count = totals.eligible_count.max(totals.proc_count);
            attempt_count = attempt_count.max(eligible_count);
            eligible_count = eligible_count.min(attempt_count);
            let proc_attempt_rate = if attempt_count > 0 {
                totals.proc_count as f64 / attempt_count as f64
            } else {
                0.0
            };
            let proc_eligible_rate = if eligible_count > 0 {
                totals.proc_count as f64 / eligible_count as f64
            } else {
                0.0
            };
            Some(RuneProcTelemetryEntry {
                rune_name: title_case_rune_name(rune_key),
                proc_count: totals.proc_count,
                attempt_count,
                eligible_count,
                proc_attempt_rate,
                proc_eligible_rate,
                bonus_damage: totals.bonus_damage,
                bonus_healing: totals.bonus_healing,
                source_breakdown,
            })
        })
        .collect::<Vec<_>>();
    entries.sort_by(|a, b| a.rune_name.cmp(&b.rune_name));
    entries
}

pub(crate) fn tick_loadout_regeneration(
    runtime: &LoadoutRuntimeState,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().second_wind;
    if !runtime.has_second_wind || max_health <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let health_ratio = (current_health / max_health).clamp(0.0, 1.0);
    let base_regen = defaults.base_regen_max_health_ratio_per_second.max(0.0) * max_health * dt;
    let bonus = if health_ratio <= defaults.low_health_threshold_ratio {
        defaults
            .low_health_bonus_regen_max_health_ratio_per_second
            .max(0.0)
            * max_health
            * dt
    } else {
        0.0
    };
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: base_regen + bonus,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

fn cooldown_status(now: f64, ready_at: f64) -> String {
    let remaining = (ready_at - now).max(0.0);
    if remaining <= 1e-9 {
        "ready".to_string()
    } else {
        format!("{remaining:.2}s")
    }
}

pub(crate) fn describe_runtime_cooldowns(runtime: &LoadoutRuntimeState, now: f64) -> Vec<String> {
    let mut lines = Vec::new();

    if runtime.has_grasp {
        lines.push(format!(
            "Grasp of the Undying: {} (cooldown {:.2}s)",
            cooldown_status(now, runtime.grasp_ready_at),
            runtime.grasp_cooldown_seconds
        ));
    }
    if runtime.has_heartsteel {
        lines.push(format!(
            "Heartsteel Colossal Consumption: {} (cooldown {:.2}s)",
            cooldown_status(now, runtime.heartsteel_ready_at),
            runtime.heartsteel_cooldown_seconds
        ));
    }
    if runtime.has_luden {
        lines.push(format!(
            "Luden's Echo: {} (cooldown {:.2}s)",
            cooldown_status(now, runtime.luden_ready_at),
            runtime.luden_cooldown_seconds
        ));
    }
    if runtime.has_fleet_footwork {
        lines.push(format!(
            "Fleet Footwork: {}",
            cooldown_status(now, runtime.fleet_ready_at)
        ));
    }
    if runtime.has_aftershock {
        lines.push(format!(
            "Aftershock: {} (active {})",
            cooldown_status(now, runtime.aftershock_ready_at),
            if now <= runtime.aftershock_active_until {
                "yes"
            } else {
                "no"
            }
        ));
    }
    if runtime.has_electrocute {
        lines.push(format!(
            "Electrocute: {}",
            cooldown_status(now, runtime.electrocute_ready_at)
        ));
    }
    if runtime.has_first_strike {
        lines.push(format!(
            "First Strike: {} (window active {})",
            cooldown_status(now, runtime.first_strike_ready_at),
            if now <= runtime.first_strike_window_until {
                "yes"
            } else {
                "no"
            }
        ));
    }
    if runtime.has_phase_rush {
        lines.push(format!(
            "Phase Rush: {} (active {})",
            cooldown_status(now, runtime.phase_rush_ready_at),
            if now <= runtime.phase_rush_active_until {
                "yes"
            } else {
                "no"
            }
        ));
    }
    if runtime.has_arcane_comet {
        lines.push(format!(
            "Arcane Comet: {}",
            cooldown_status(now, runtime.arcane_comet_ready_at)
        ));
    }
    if runtime.has_summon_aery {
        lines.push(format!(
            "Summon Aery: {}",
            cooldown_status(now, runtime.summon_aery_ready_at)
        ));
    }
    if runtime.has_hail_of_blades {
        lines.push(format!(
            "Hail of Blades: {} (remaining attacks {})",
            cooldown_status(now, runtime.hail_of_blades_ready_at),
            runtime.hail_of_blades_remaining_attacks
        ));
    }
    if runtime.has_dark_harvest {
        lines.push(format!(
            "Dark Harvest: {}",
            cooldown_status(now, runtime.dark_harvest_ready_at)
        ));
    }

    if lines.is_empty() {
        lines.push("none".to_string());
    }
    lines
}

pub(crate) fn describe_runtime_stacks(runtime: &LoadoutRuntimeState) -> Vec<String> {
    let defaults = rune_runtime_defaults();
    let mut lines = Vec::new();
    if runtime.has_lethal_tempo {
        lines.push(format!(
            "Lethal Tempo stacks: {}/{}",
            runtime.lethal_tempo_stacks,
            defaults.lethal_tempo.max_stacks.max(1)
        ));
    }
    if runtime.has_guinsoo {
        lines.push(format!("Guinsoo stacks: {}/8", runtime.guinsoo_stacks));
    }
    if runtime.has_conqueror {
        lines.push(format!(
            "Conqueror stacks: {}/{}",
            runtime.conqueror_stacks,
            defaults.conqueror.max_stacks.max(1)
        ));
    }
    if runtime.has_hail_of_blades {
        lines.push(format!(
            "Hail of Blades empowered attacks remaining: {}",
            runtime.hail_of_blades_remaining_attacks
        ));
    }
    if runtime.has_dark_harvest {
        lines.push(format!(
            "Dark Harvest souls: {}",
            runtime.dark_harvest_souls
        ));
    }
    if runtime.has_press_the_attack {
        let vulnerable_targets = runtime
            .press_the_attack_targets
            .values()
            .filter(|state| state.vulnerable_until > 0.0)
            .count();
        lines.push(format!(
            "Press the Attack tracked targets: {}",
            vulnerable_targets
        ));
    }
    if runtime.has_electrocute {
        let primed_targets = runtime
            .electrocute_targets
            .values()
            .filter(|state| state.stacks > 0)
            .count();
        lines.push(format!("Electrocute primed targets: {}", primed_targets));
    }
    if runtime.has_phase_rush {
        let tracked_targets = runtime
            .phase_rush_targets
            .values()
            .filter(|state| state.stacks > 0)
            .count();
        lines.push(format!("Phase Rush tracked targets: {}", tracked_targets));
    }
    if runtime.has_kraken || runtime.has_blade_of_the_ruined_king {
        lines.push(format!("Attacks landed: {}", runtime.attacks_landed));
    }
    if lines.is_empty() {
        lines.push("none".to_string());
    }
    lines
}

#[cfg(test)]
#[path = "tests/loadout_runtime_tests.rs"]
mod tests;

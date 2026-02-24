use super::LoadoutRuntimeState;

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

pub(super) const MODELED_RUNE_TELEMETRY_KEYS: [&str; 12] = [
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
];

pub(super) const MODELED_RUNE_TELEMETRY_KEY_COUNT: usize = MODELED_RUNE_TELEMETRY_KEYS.len();
const RUNE_PROC_TRIGGER_SOURCE_COUNT: usize = RUNE_PROC_TRIGGER_SOURCES.len();

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RuneProcTelemetrySourceTotals {
    proc_count: usize,
    attempt_count: usize,
    eligible_count: usize,
    bonus_damage: f64,
    bonus_healing: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct RuneProcTelemetryTotals {
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

pub(super) fn record_rune_proc(
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

pub(super) fn record_rune_proc_attempt(
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

pub(super) fn record_rune_proc_eligibility(
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

pub(super) fn build_rune_proc_telemetry_entries(
    runtime: &LoadoutRuntimeState,
) -> Vec<RuneProcTelemetryEntry> {
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
                    let (attempt_count, eligible_count) = normalized_attempt_and_eligible_counts(
                        source_totals.attempt_count,
                        source_totals.eligible_count,
                        source_totals.proc_count,
                    );
                    let (proc_attempt_rate, proc_eligible_rate) =
                        proc_rate_values(source_totals.proc_count, attempt_count, eligible_count);
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
            let (attempt_count, eligible_count) = normalized_attempt_and_eligible_counts(
                totals.attempt_count,
                totals.eligible_count,
                totals.proc_count,
            );
            let (proc_attempt_rate, proc_eligible_rate) =
                proc_rate_values(totals.proc_count, attempt_count, eligible_count);
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

fn rune_proc_trigger_source_index(source: RuneProcTriggerSource) -> usize {
    match source {
        RuneProcTriggerSource::OnHit => 0,
        RuneProcTriggerSource::Ability => 1,
        RuneProcTriggerSource::Immobilize => 2,
        RuneProcTriggerSource::EnemyKill => 3,
        RuneProcTriggerSource::RuntimeActivation => 4,
    }
}

pub(super) fn rune_telemetry_index(rune_key: &str) -> Option<usize> {
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

fn normalized_attempt_and_eligible_counts(
    attempt_count: usize,
    eligible_count: usize,
    proc_count: usize,
) -> (usize, usize) {
    let mut normalized_attempt_count = attempt_count.max(eligible_count).max(proc_count);
    let mut normalized_eligible_count = eligible_count.max(proc_count);
    normalized_attempt_count = normalized_attempt_count.max(normalized_eligible_count);
    normalized_eligible_count = normalized_eligible_count.min(normalized_attempt_count);
    (normalized_attempt_count, normalized_eligible_count)
}

fn proc_rate_values(proc_count: usize, attempt_count: usize, eligible_count: usize) -> (f64, f64) {
    let proc_attempt_rate = if attempt_count > 0 {
        proc_count as f64 / attempt_count as f64
    } else {
        0.0
    };
    let proc_eligible_rate = if eligible_count > 0 {
        proc_count as f64 / eligible_count as f64
    } else {
        0.0
    };
    (proc_attempt_rate, proc_eligible_rate)
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
        _ => normalized_rune_key.to_string(),
    }
}

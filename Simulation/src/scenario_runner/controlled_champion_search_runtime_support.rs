use anyhow::Result;
use serde_json::{Value, json};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::*;

pub(super) fn fixed_sweep_keystone_seed_base(seed_base: u64, keystone: &str) -> u64 {
    let mut keystone_seed_hasher = DefaultHasher::new();
    to_norm_key(keystone).hash(&mut keystone_seed_hasher);
    seed_base.wrapping_add(keystone_seed_hasher.finish())
}

pub(super) fn fixed_sweep_repeat_seed(keystone_seed_base: u64, repeat_idx: usize) -> u64 {
    keystone_seed_base
        .wrapping_add((repeat_idx as u64).wrapping_mul(FIXED_SWEEP_REPEAT_SEED_STRIDE))
}

pub(super) fn append_rune_proc_telemetry_markdown_entries(
    content: &mut String,
    entry_prefix: &str,
    detail_prefix: &str,
    entries: &[ChampionRuneProcTelemetryEntry],
    total_damage: f64,
    total_healing: f64,
) {
    fn share_percent(part: f64, total: f64) -> f64 {
        if total > 0.0 {
            (part.max(0.0) / total) * 100.0
        } else {
            0.0
        }
    }

    if entries.is_empty() {
        content.push_str(&format!("{entry_prefix}none\n"));
        return;
    }
    for entry in entries {
        let damage_share_percent = share_percent(entry.bonus_damage, total_damage);
        let healing_share_percent = share_percent(entry.bonus_healing, total_healing);
        content.push_str(&format!(
            "{entry_prefix}{}:\n{detail_prefix}- Procs: `{}`\n{detail_prefix}- Attempts: `{}`\n{detail_prefix}- Eligible: `{}`\n{detail_prefix}- Proc rate (vs attempts): `{:.1}%`\n{detail_prefix}- Proc rate (vs eligible): `{:.1}%`\n{detail_prefix}- Bonus damage: `{:.2}` ({:.2}% share)\n{detail_prefix}- Bonus healing: `{:.2}` ({:.2}% share)\n",
            entry.rune_name,
            entry.proc_count,
            entry.attempt_count,
            entry.eligible_count,
            entry.proc_attempt_rate * 100.0,
            entry.proc_eligible_rate * 100.0,
            entry.bonus_damage,
            damage_share_percent,
            entry.bonus_healing,
            healing_share_percent
        ));
        if !entry.source_breakdown.is_empty() {
            content.push_str(&format!("{detail_prefix}- Sources:\n"));
            let source_prefix = format!("{detail_prefix}  ");
            let source_detail_prefix = format!("{source_prefix}  ");
            for source in &entry.source_breakdown {
                content.push_str(&format!(
                    "{}- {}:\n{}- Procs: `{}`\n{}- Attempts: `{}`\n{}- Eligible: `{}`\n{}- Proc rate (vs attempts): `{:.1}%`\n{}- Proc rate (vs eligible): `{:.1}%`\n{}- Bonus damage: `{:.2}`\n{}- Bonus healing: `{:.2}`\n",
                    source_prefix,
                    source.source,
                    source_detail_prefix,
                    source.proc_count,
                    source_detail_prefix,
                    source.attempt_count,
                    source_detail_prefix,
                    source.eligible_count,
                    source_detail_prefix,
                    source.proc_attempt_rate * 100.0,
                    source_detail_prefix,
                    source.proc_eligible_rate * 100.0,
                    source_detail_prefix,
                    source.bonus_damage,
                    source_detail_prefix,
                    source.bonus_healing
                ));
            }
        }
    }
}

pub(super) fn rune_proc_telemetry_json(
    entries: &[ChampionRuneProcTelemetryEntry],
    total_damage: f64,
    total_healing: f64,
) -> Vec<Value> {
    entries
        .iter()
        .map(|entry| {
            let damage_share = if total_damage > 0.0 {
                entry.bonus_damage.max(0.0) / total_damage
            } else {
                0.0
            };
            let healing_share = if total_healing > 0.0 {
                entry.bonus_healing.max(0.0) / total_healing
            } else {
                0.0
            };
            json!({
                "rune_name": entry.rune_name,
                "proc_count": entry.proc_count,
                "attempt_count": entry.attempt_count,
                "eligible_count": entry.eligible_count,
                "proc_attempt_rate": entry.proc_attempt_rate,
                "proc_eligible_rate": entry.proc_eligible_rate,
                "opportunity_count": entry.eligible_count,
                "proc_opportunity_rate": entry.proc_eligible_rate,
                "bonus_damage": entry.bonus_damage,
                "bonus_damage_share": damage_share,
                "bonus_healing": entry.bonus_healing,
                "bonus_healing_share": healing_share,
                "source_breakdown": entry.source_breakdown.iter().map(|source| {
                    json!({
                        "source": source.source,
                        "proc_count": source.proc_count,
                        "attempt_count": source.attempt_count,
                        "eligible_count": source.eligible_count,
                        "proc_attempt_rate": source.proc_attempt_rate,
                        "proc_eligible_rate": source.proc_eligible_rate,
                        "opportunity_count": source.eligible_count,
                        "proc_opportunity_rate": source.proc_eligible_rate,
                        "bonus_damage": source.bonus_damage,
                        "bonus_healing": source.bonus_healing
                    })
                }).collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>()
}

#[derive(Debug, Clone, Copy)]
pub(super) struct SignificantProgressState {
    pub(super) best_overall_score: f64,
    pub(super) best_significant_score: f64,
    pub(super) significant_events: usize,
    pub(super) last_significant_at: Instant,
}

#[derive(Debug, Clone, Default)]
pub(super) struct SearchTypeRuntimeCounter {
    pub(super) score_requests: usize,
    pub(super) new_simulations: usize,
}

#[derive(Debug, Default)]
pub(super) struct AtomicSearchTypeRuntimeCounter {
    score_requests: AtomicUsize,
    new_simulations: AtomicUsize,
}

impl AtomicSearchTypeRuntimeCounter {
    pub(super) fn add(&self, score_requests: usize, new_simulations: usize) {
        self.score_requests
            .fetch_add(score_requests, AtomicOrdering::Relaxed);
        self.new_simulations
            .fetch_add(new_simulations, AtomicOrdering::Relaxed);
    }

    pub(super) fn snapshot(&self) -> SearchTypeRuntimeCounter {
        SearchTypeRuntimeCounter {
            score_requests: self.score_requests.load(AtomicOrdering::Relaxed),
            new_simulations: self.new_simulations.load(AtomicOrdering::Relaxed),
        }
    }
}

#[derive(Debug)]
pub(super) struct ShardedStringSet {
    shards: Vec<Mutex<HashSet<String>>>,
}

impl ShardedStringSet {
    pub(super) fn new() -> Self {
        let shard_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8)
            .next_power_of_two()
            .max(8);
        let shards = (0..shard_count)
            .map(|_| Mutex::new(HashSet::new()))
            .collect::<Vec<_>>();
        Self { shards }
    }

    fn shard_index(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) & (self.shards.len() - 1)
    }

    pub(super) fn insert(&self, key: String) {
        let shard = self.shard_index(&key);
        if let Ok(mut set) = self.shards[shard].lock() {
            set.insert(key);
        }
    }

    pub(super) fn len(&self) -> usize {
        self.shards
            .iter()
            .map(|shard| shard.lock().map(|set| set.len()).unwrap_or(0))
            .sum()
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct CoverageStageDiagnostics {
    pub(super) enabled: bool,
    pub(super) elapsed_seconds: f64,
    pub(super) assets_total: usize,
    pub(super) assets_covered: usize,
    pub(super) seed_candidates: usize,
    pub(super) seed_candidates_unique: usize,
    pub(super) coverage_incomplete: bool,
    pub(super) coverage_warning: String,
}

#[derive(Debug, Clone)]
pub(super) enum CoverageLockedAsset {
    Item(usize),
    Rune(String),
    Shard { slot: usize, stat: String },
}

impl CoverageLockedAsset {
    pub(super) fn display_label(&self, item_pool: &[Item]) -> String {
        match self {
            Self::Item(item_idx) => item_pool
                .get(*item_idx)
                .map(|item| format!("item:{}", item.name))
                .unwrap_or_else(|| format!("item_index:{item_idx}")),
            Self::Rune(name) => format!("rune:{name}"),
            Self::Shard { slot, stat } => format!("shard_slot_{}:{}", slot + 1, stat),
        }
    }
}

pub(super) fn coverage_locked_assets(
    item_pool: &[Item],
    loadout_domain: &crate::data::LoadoutDomain,
) -> Vec<CoverageLockedAsset> {
    let mut out = Vec::new();
    for item_idx in 0..item_pool.len() {
        out.push(CoverageLockedAsset::Item(item_idx));
    }

    let mut rune_by_key = HashMap::<String, String>::new();
    let primary_path_indices = loadout_domain
        .rune_paths
        .iter()
        .enumerate()
        .filter_map(|(idx, path)| {
            (path.slot_runes.len() >= 4
                && path.slot_runes.iter().take(4).all(|slot| !slot.is_empty()))
            .then_some(idx)
        })
        .collect::<Vec<_>>();
    for &primary_idx in &primary_path_indices {
        let primary_path = &loadout_domain.rune_paths[primary_idx];
        let secondary_path_indices = loadout_domain
            .rune_paths
            .iter()
            .enumerate()
            .filter_map(|(idx, path)| {
                if idx == primary_idx || path.slot_runes.len() < 4 {
                    return None;
                }
                let secondary_slot_count = (1..=3)
                    .filter(|slot| {
                        path.slot_runes
                            .get(*slot)
                            .map(|slot_runes| !slot_runes.is_empty())
                            .unwrap_or(false)
                    })
                    .count();
                (secondary_slot_count >= 2).then_some(idx)
            })
            .collect::<Vec<_>>();
        if secondary_path_indices.is_empty() {
            continue;
        }

        for slot in primary_path.slot_runes.iter().take(4) {
            for rune_name in slot {
                let key = to_norm_key(rune_name);
                rune_by_key.entry(key).or_insert_with(|| rune_name.clone());
            }
        }
        for secondary_idx in secondary_path_indices {
            let secondary_path = &loadout_domain.rune_paths[secondary_idx];
            for slot_idx in 1..=3 {
                let Some(slot) = secondary_path.slot_runes.get(slot_idx) else {
                    continue;
                };
                if slot.is_empty() {
                    continue;
                }
                for rune_name in slot {
                    let key = to_norm_key(rune_name);
                    rune_by_key.entry(key).or_insert_with(|| rune_name.clone());
                }
            }
        }
    }
    let mut rune_values = rune_by_key.into_values().collect::<Vec<_>>();
    rune_values.sort_by_key(|name| to_norm_key(name));
    out.extend(rune_values.into_iter().map(CoverageLockedAsset::Rune));

    for (slot_idx, slot_stats) in loadout_domain.shard_slots.iter().enumerate() {
        for stat in slot_stats {
            out.push(CoverageLockedAsset::Shard {
                slot: slot_idx,
                stat: stat.clone(),
            });
        }
    }
    out
}

pub(super) fn filter_item_pool_to_modeled_runtime_effects(item_pool: &[Item]) -> Vec<Item> {
    item_pool
        .iter()
        .filter(|item| !is_item_effect_unmodeled(item))
        .cloned()
        .collect::<Vec<_>>()
}

pub(super) fn max_legal_build_size(item_pool: &[Item]) -> usize {
    let boots_count = item_pool.iter().filter(|item| is_boots(item)).count();
    let non_boots_count = item_pool.len().saturating_sub(boots_count);
    non_boots_count + usize::from(boots_count > 0)
}

pub(super) fn select_search_base_loadout_selection(
    configured: &LoadoutSelection,
    search_domain: &crate::data::LoadoutDomain,
) -> Result<LoadoutSelection> {
    if is_legal_rune_page_selection(configured, search_domain) {
        return Ok(configured.clone());
    }
    ensure_complete_loadout_selection(&LoadoutSelection::default(), search_domain)
}

pub(super) fn apply_level_scaled_sim_defaults_after_controlled_level_override(
    sim: &mut SimulationConfig,
    simulation_config: &Value,
    previous_level: usize,
) {
    if sim.champion_level == previous_level {
        return;
    }
    let protoplasm_defaults = protoplasm_lifeline_defaults();
    let protoplasm_level_t = ((sim.champion_level.max(1) as f64 - 1.0) / 29.0).clamp(0.0, 1.0);
    if simulation_config.get("protoplasm_bonus_health").is_none() {
        sim.protoplasm_bonus_health = protoplasm_defaults.bonus_health_min
            + (protoplasm_defaults.bonus_health_max - protoplasm_defaults.bonus_health_min)
                * protoplasm_level_t;
    }
    if simulation_config.get("protoplasm_heal_total").is_none() {
        sim.protoplasm_heal_total = protoplasm_defaults.heal_total_min
            + (protoplasm_defaults.heal_total_max - protoplasm_defaults.heal_total_min)
                * protoplasm_level_t;
    }
}

pub(super) fn partial_candidate_completion_seed(
    search_seed: u64,
    seed_index: usize,
    ranked_index: usize,
    candidate: &BuildKey,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    candidate.hash(&mut hasher);
    search_seed
        ^ hasher.finish()
        ^ ((seed_index as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15))
        ^ ((ranked_index as u64 + 1).wrapping_mul(0xbf58_476d_1ce4_e5b9))
}

pub(super) fn complete_partial_candidate_to_full(
    partial: &BuildKey,
    item_pool: &[Item],
    max_items: usize,
    seed: &mut u64,
) -> BuildKey {
    let mut candidate = canonical_build_candidate(partial.clone());
    if candidate.item_indices.len() >= max_items {
        candidate.item_indices.truncate(max_items);
        candidate.item_indices = canonical_key(&candidate.item_indices);
        return canonical_build_candidate(candidate);
    }

    while candidate.item_indices.len() < max_items {
        let options = (0..item_pool.len())
            .filter(|idx| can_add_item_to_build(item_pool, &candidate.item_indices, *idx))
            .collect::<Vec<_>>();
        if options.is_empty() {
            break;
        }
        let pick = options[rand_index(seed, options.len())];
        candidate.item_indices.push(pick);
        candidate.item_indices = canonical_key(&candidate.item_indices);
    }

    if candidate.item_indices.len() < max_items {
        candidate.item_indices = random_valid_build(item_pool, max_items, seed);
    }
    canonical_build_candidate(candidate)
}

pub(super) fn arm_time_budget_deadline_if_unset(
    hard_deadline_state: &Arc<Mutex<Option<Instant>>>,
    time_budget: Option<Duration>,
    defer_hard_budget_until_coverage: bool,
    search_type: &str,
) {
    let Some(duration) = time_budget else {
        return;
    };
    if defer_hard_budget_until_coverage && search_type == "coverage_stage" {
        return;
    }
    if let Ok(mut state) = hard_deadline_state.lock()
        && state.is_none()
    {
        *state = Some(Instant::now() + duration);
    }
}

fn candidate_matches_locked_asset(candidate: &BuildKey, asset: &CoverageLockedAsset) -> bool {
    match asset {
        CoverageLockedAsset::Item(item_idx) => candidate.item_indices.contains(item_idx),
        CoverageLockedAsset::Rune(name) => {
            let target = to_norm_key(name);
            candidate
                .loadout_selection
                .rune_names
                .iter()
                .any(|rune| to_norm_key(rune) == target)
        }
        CoverageLockedAsset::Shard { slot, stat } => candidate
            .loadout_selection
            .shard_stats
            .get(*slot)
            .map(|value| to_norm_key(value) == to_norm_key(stat))
            .unwrap_or(false),
    }
}

fn enforce_locked_item(
    item_pool: &[Item],
    max_items: usize,
    item_idx: usize,
    build: &mut Vec<usize>,
    seed: &mut u64,
) -> bool {
    if build.contains(&item_idx) {
        return true;
    }
    if build.len() < max_items && can_add_item_to_build(item_pool, build, item_idx) {
        build.push(item_idx);
        *build = canonical_key(build);
        return true;
    }

    let mut slots = (0..build.len()).collect::<Vec<_>>();
    shuffle_usize(&mut slots, seed);
    for slot in slots {
        let mut trial = build.clone();
        trial[slot] = item_idx;
        repair_build(item_pool, &mut trial, max_items, seed);
        if trial.contains(&item_idx) {
            *build = canonical_key(&trial);
            return true;
        }
    }

    let mut fallback = vec![item_idx];
    repair_build(item_pool, &mut fallback, max_items, seed);
    if fallback.contains(&item_idx) {
        *build = canonical_key(&fallback);
        return true;
    }
    false
}

fn random_loadout_matching_asset(
    base_loadout: &LoadoutSelection,
    loadout_domain: &crate::data::LoadoutDomain,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> Option<LoadoutSelection> {
    let attempts = 4096usize;
    for _ in 0..attempts {
        let selection = random_loadout_selection(base_loadout, loadout_domain, seed);
        let candidate = BuildKey {
            item_indices: Vec::new(),
            loadout_selection: selection.clone(),
        };
        if candidate_matches_locked_asset(&candidate, asset) {
            return Some(selection);
        }
    }
    None
}

fn enforce_locked_asset(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &mut BuildKey,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> bool {
    match asset {
        CoverageLockedAsset::Item(item_idx) => {
            if !enforce_locked_item(
                params.item_pool,
                params.max_items,
                *item_idx,
                &mut candidate.item_indices,
                seed,
            ) {
                return false;
            }
        }
        CoverageLockedAsset::Rune(_) | CoverageLockedAsset::Shard { .. } => {
            if let Some(selection) = random_loadout_matching_asset(
                params.base_loadout,
                params.loadout_domain,
                asset,
                seed,
            ) {
                candidate.loadout_selection = selection;
            } else {
                return false;
            }
        }
    }
    candidate.item_indices = canonical_key(&candidate.item_indices);
    candidate_matches_locked_asset(candidate, asset)
}

pub(super) fn random_locked_candidate(
    params: &FullLoadoutSearchParams<'_>,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> Option<BuildKey> {
    let mut candidate = BuildKey {
        item_indices: random_valid_build(params.item_pool, params.max_items, seed),
        loadout_selection: random_loadout_selection(
            params.base_loadout,
            params.loadout_domain,
            seed,
        ),
    };
    if !enforce_locked_asset(params, &mut candidate, asset, seed) {
        return None;
    }
    Some(canonical_build_candidate(candidate))
}

pub(super) fn mutate_locked_candidate(
    params: &FullLoadoutSearchParams<'_>,
    candidate: &BuildKey,
    asset: &CoverageLockedAsset,
    seed: &mut u64,
) -> Option<BuildKey> {
    let mut out = candidate.clone();

    if !out.item_indices.is_empty() && rand_f64(seed) < 0.85 {
        let slot = rand_index(seed, out.item_indices.len());
        for _ in 0..params.item_pool.len().max(1) {
            let replacement = rand_index(seed, params.item_pool.len());
            if out.item_indices[slot] == replacement {
                continue;
            }
            out.item_indices[slot] = replacement;
            repair_build(
                params.item_pool,
                &mut out.item_indices,
                params.max_items,
                seed,
            );
            if out.item_indices.contains(&replacement) {
                break;
            }
        }
    }

    if rand_f64(seed) < 0.85 {
        out.loadout_selection =
            random_loadout_selection(&out.loadout_selection, params.loadout_domain, seed);
    }

    if !enforce_locked_asset(params, &mut out, asset, seed) {
        return None;
    }
    Some(canonical_build_candidate(out))
}

pub(super) fn structured_trace_event(line: &str) -> Value {
    let (header, multiline_details) = match line.split_once('\n') {
        Some((head, details)) => (head, Some(details)),
        None => (line, None),
    };

    let mut timestamp_seconds = None::<f64>;
    let mut event_type = "unknown".to_string();
    let mut details = header.to_string();

    if let Some((time_part, rest)) = header.split_once("s [") {
        timestamp_seconds = time_part.parse::<f64>().ok();
        if let Some((kind, event_details)) = rest.split_once("] ") {
            event_type = kind.to_string();
            details = event_details.to_string();
        } else if let Some((kind, event_details)) = rest.split_once(']') {
            event_type = kind.to_string();
            details = event_details.trim_start().to_string();
        }
    }

    if let Some(extra) = multiline_details {
        if details.is_empty() {
            details = extra.to_string();
        } else {
            details.push('\n');
            details.push_str(extra);
        }
    }

    json!({
        "timestamp_seconds": timestamp_seconds,
        "event_type": event_type,
        "details": details,
        "raw": line,
    })
}

pub(super) fn build_enemy_similarity_notes(profiles: &[EnemyDerivedCombatStats]) -> Vec<String> {
    let mut pair_notes = Vec::new();
    for i in 0..profiles.len() {
        for j in (i + 1)..profiles.len() {
            let a = &profiles[i];
            let b = &profiles[j];
            let attack_damage_close = (a.attack_damage - b.attack_damage).abs() <= 8.0;
            let interval_close =
                (a.attack_interval_seconds - b.attack_interval_seconds).abs() <= 0.10;
            let range_close = (a.attack_range - b.attack_range).abs() <= 40.0;
            if attack_damage_close && interval_close && range_close {
                pair_notes.push(format!(
                    "{} and {} have very similar auto profiles (AD {:.1}/{:.1}, interval {:.3}/{:.3}, range {:.0}/{:.0}).",
                    a.champion,
                    b.champion,
                    a.attack_damage,
                    b.attack_damage,
                    a.attack_interval_seconds,
                    b.attack_interval_seconds,
                    a.attack_range,
                    b.attack_range
                ));
            }
        }
    }

    if pair_notes.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    out.push(format!(
        "Detected {} pair(s) of enemy auto profiles that are unusually similar; verify presets/loadout ingestion if this looks incorrect.",
        pair_notes.len()
    ));
    out.extend(pair_notes.into_iter().take(8));
    out
}

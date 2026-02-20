use anyhow::{Result, anyhow};
use rayon::prelude::*;
use serde_json::{Value, json};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::build_order::{acquisition_level_map, optimize_build_order};
use crate::cache::BlockingScoreCache;
use crate::data::{
    ensure_complete_loadout_selection, filter_loadout_domain_to_modeled_runes,
    is_legal_rune_page_selection, parse_stack_overrides_map,
};
use crate::defaults::protoplasm_lifeline_defaults;
use crate::engine::{
    ControlledChampionCombatSimulation, EnemyDerivedCombatStats, derive_enemy_combat_stats,
};
use crate::reporting::{
    write_controlled_champion_report_json, write_controlled_champion_report_markdown,
};
use crate::scripts::champions::{
    ChampionRuneProcTelemetryEntry, resolve_controlled_champion_script,
};
use crate::scripts::coverage::is_item_effect_unmodeled;
use crate::search::{
    FullLoadoutSearchParams, adaptive_strategy_candidates_full_loadout,
    build_search_ranked_full_loadout, candidate_pareto_front_keys, choose_best_build_by_stat,
    compute_build_metrics_for_candidate, generate_bleed_candidates_full_loadout, item_names,
    portfolio_strategy_list, search_strategy_summary, select_diverse_top_candidates,
    strategy_seed_elites_full_loadout,
};
use crate::status::{StatusReporter, deadline_reached};

use super::*;

struct ControlledChampionScenarioConfig {
    base: ChampionBase,
    level: usize,
    loadout_selection: LoadoutSelection,
    stack_overrides: HashMap<String, f64>,
}

const FIXED_LOADOUT_TRACE_JSON_SCHEMA_VERSION: u32 = 3;
const FIXED_LOADOUT_REPORT_JSON_SCHEMA_VERSION: u32 = 2;
const FIXED_LOADOUT_RUNE_SWEEP_JSON_SCHEMA_VERSION: u32 = 2;
const CONTROLLED_CHAMPION_TRACE_JSON_SCHEMA_VERSION: u32 = 2;
const FIXED_SWEEP_REPEAT_SEED_STRIDE: u64 = 0x9E37_79B9_7F4A_7C15;

fn fixed_sweep_keystone_seed_base(seed_base: u64, keystone: &str) -> u64 {
    let mut keystone_seed_hasher = DefaultHasher::new();
    to_norm_key(keystone).hash(&mut keystone_seed_hasher);
    seed_base.wrapping_add(keystone_seed_hasher.finish())
}

fn fixed_sweep_repeat_seed(keystone_seed_base: u64, repeat_idx: usize) -> u64 {
    keystone_seed_base
        .wrapping_add((repeat_idx as u64).wrapping_mul(FIXED_SWEEP_REPEAT_SEED_STRIDE))
}

fn append_rune_proc_telemetry_markdown_entries(
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

fn rune_proc_telemetry_json(
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
struct SignificantProgressState {
    best_overall_score: f64,
    best_significant_score: f64,
    significant_events: usize,
    last_significant_at: Instant,
}

#[derive(Debug, Clone, Default)]
struct SearchTypeRuntimeCounter {
    score_requests: usize,
    new_simulations: usize,
}

#[derive(Debug, Default)]
struct AtomicSearchTypeRuntimeCounter {
    score_requests: AtomicUsize,
    new_simulations: AtomicUsize,
}

impl AtomicSearchTypeRuntimeCounter {
    fn add(&self, score_requests: usize, new_simulations: usize) {
        self.score_requests
            .fetch_add(score_requests, AtomicOrdering::Relaxed);
        self.new_simulations
            .fetch_add(new_simulations, AtomicOrdering::Relaxed);
    }

    fn snapshot(&self) -> SearchTypeRuntimeCounter {
        SearchTypeRuntimeCounter {
            score_requests: self.score_requests.load(AtomicOrdering::Relaxed),
            new_simulations: self.new_simulations.load(AtomicOrdering::Relaxed),
        }
    }
}

#[derive(Debug)]
struct ShardedStringSet {
    shards: Vec<Mutex<HashSet<String>>>,
}

impl ShardedStringSet {
    fn new() -> Self {
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

    fn insert(&self, key: String) {
        let shard = self.shard_index(&key);
        if let Ok(mut set) = self.shards[shard].lock() {
            set.insert(key);
        }
    }

    fn len(&self) -> usize {
        self.shards
            .iter()
            .map(|shard| shard.lock().map(|set| set.len()).unwrap_or(0))
            .sum()
    }
}

#[derive(Debug, Clone, Default)]
struct CoverageStageDiagnostics {
    enabled: bool,
    elapsed_seconds: f64,
    assets_total: usize,
    assets_covered: usize,
    seed_candidates: usize,
    seed_candidates_unique: usize,
    coverage_incomplete: bool,
    coverage_warning: String,
}

#[derive(Debug, Clone)]
enum CoverageLockedAsset {
    Item(usize),
    Rune(String),
    Shard { slot: usize, stat: String },
}

impl CoverageLockedAsset {
    fn display_label(&self, item_pool: &[Item]) -> String {
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

fn coverage_locked_assets(
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

fn filter_item_pool_to_modeled_runtime_effects(item_pool: &[Item]) -> Vec<Item> {
    item_pool
        .iter()
        .filter(|item| !is_item_effect_unmodeled(item))
        .cloned()
        .collect::<Vec<_>>()
}

fn max_legal_build_size(item_pool: &[Item]) -> usize {
    let boots_count = item_pool.iter().filter(|item| is_boots(item)).count();
    let non_boots_count = item_pool.len().saturating_sub(boots_count);
    non_boots_count + usize::from(boots_count > 0)
}

fn select_search_base_loadout_selection(
    configured: &LoadoutSelection,
    search_domain: &crate::data::LoadoutDomain,
) -> Result<LoadoutSelection> {
    if is_legal_rune_page_selection(configured, search_domain) {
        return Ok(configured.clone());
    }
    ensure_complete_loadout_selection(&LoadoutSelection::default(), search_domain)
}

fn apply_level_scaled_sim_defaults_after_controlled_level_override(
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

fn partial_candidate_completion_seed(
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

fn complete_partial_candidate_to_full(
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

fn arm_time_budget_deadline_if_unset(
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

fn random_locked_candidate(
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

fn mutate_locked_candidate(
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

fn initialize_search_type_counters(
    active_strategies: &[String],
    configured_strategy: &str,
) -> Arc<HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>> {
    let mut keys = vec![
        "coverage_stage".to_string(),
        "strategy_elites".to_string(),
        "adaptive_search".to_string(),
        "strict_full_ranking".to_string(),
        format!("seed_search:{}", configured_strategy),
    ];
    for strategy in active_strategies {
        keys.push(format!("seed_search:{strategy}"));
    }
    keys.sort();
    keys.dedup();

    Arc::new(
        keys.into_iter()
            .map(|key| (key, Arc::new(AtomicSearchTypeRuntimeCounter::default())))
            .collect::<HashMap<_, _>>(),
    )
}

fn increment_search_type_counter(
    counters: &HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>,
    search_type: &str,
    score_requests: usize,
    new_simulations: usize,
) {
    if let Some(counter) = counters.get(search_type) {
        counter.add(score_requests, new_simulations);
    }
}

fn snapshot_search_type_counters(
    counters: &HashMap<String, Arc<AtomicSearchTypeRuntimeCounter>>,
) -> Vec<SearchTypeBreakdown> {
    counters
        .iter()
        .filter_map(|(name, counter)| {
            let snapshot = counter.snapshot();
            let touched = snapshot.score_requests > 0 || snapshot.new_simulations > 0;
            touched.then(|| SearchTypeBreakdown {
                name: name.clone(),
                score_requests: snapshot.score_requests,
                new_simulations: snapshot.new_simulations,
            })
        })
        .collect::<Vec<_>>()
}

fn n_choose_k(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    if k == 0 {
        return 1;
    }
    let mut result = 1u128;
    for i in 1..=k {
        let numerator = (n - k + i) as u128;
        let denominator = i as u128;
        result = (result * numerator) / denominator;
    }
    result
}

fn estimated_legal_item_build_count(item_pool: &[Item], max_items: usize) -> f64 {
    if max_items == 0 {
        return 1.0;
    }
    let boots_count = item_pool.iter().filter(|item| is_boots(item)).count();
    let non_boots_count = item_pool.len().saturating_sub(boots_count);
    let max_boots = boots_count.min(1).min(max_items);
    let mut total = 0u128;
    for boots_used in 0..=max_boots {
        let non_boots_used = max_items.saturating_sub(boots_used);
        if non_boots_used > non_boots_count {
            continue;
        }
        total = total.saturating_add(
            n_choose_k(boots_count, boots_used)
                .saturating_mul(n_choose_k(non_boots_count, non_boots_used)),
        );
    }
    total as f64
}

fn estimated_legal_loadout_count(loadout_domain: &crate::data::LoadoutDomain) -> f64 {
    if loadout_domain.rune_paths.len() < 2 {
        return 0.0;
    }
    let shard_count = loadout_domain
        .shard_slots
        .iter()
        .map(|slot| slot.len() as u128)
        .product::<u128>();
    if shard_count == 0 {
        return 0.0;
    }
    let mut rune_pages = 0u128;
    for (primary_index, primary_path) in loadout_domain.rune_paths.iter().enumerate() {
        if primary_path.slot_runes.len() < 4 {
            continue;
        }
        let primary_count = primary_path.slot_runes[..4]
            .iter()
            .map(|slot| slot.len() as u128)
            .product::<u128>();
        if primary_count == 0 {
            continue;
        }
        for (secondary_index, secondary_path) in loadout_domain.rune_paths.iter().enumerate() {
            if secondary_index == primary_index || secondary_path.slot_runes.len() < 4 {
                continue;
            }
            let secondary_pair_count = [(1usize, 2usize), (1usize, 3usize), (2usize, 3usize)]
                .iter()
                .map(|(slot_a, slot_b)| {
                    (secondary_path.slot_runes[*slot_a].len() as u128)
                        .saturating_mul(secondary_path.slot_runes[*slot_b].len() as u128)
                })
                .sum::<u128>();
            rune_pages =
                rune_pages.saturating_add(primary_count.saturating_mul(secondary_pair_count));
        }
    }
    rune_pages.saturating_mul(shard_count) as f64
}

fn heuristic_sort_remaining_candidates_for_strict_ranking(
    mut remaining_keys: Vec<BuildKey>,
    strict_scores: &HashMap<BuildKey, f64>,
    item_pool_len: usize,
    rune_signal_weight: f64,
    shard_signal_weight: f64,
    seed: u64,
    exploration_promotions: usize,
) -> (Vec<BuildKey>, usize) {
    if remaining_keys.len() <= 1 {
        return (remaining_keys, 0);
    }

    let finite_scores = strict_scores
        .values()
        .copied()
        .filter(|score| score.is_finite())
        .collect::<Vec<_>>();
    if finite_scores.len() > 1 {
        let min_score = finite_scores.iter().copied().fold(f64::INFINITY, f64::min);
        let max_score = finite_scores
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let span = max_score - min_score;
        if span > 1e-9 {
            let mut item_signals = vec![0.0_f64; item_pool_len];
            let mut rune_signals = HashMap::<String, f64>::new();
            let mut shard_signals = HashMap::<String, f64>::new();
            for (candidate, score) in strict_scores {
                if !score.is_finite() {
                    continue;
                }
                let centered = ((*score - min_score) / span) - 0.5;
                for &item_idx in &candidate.item_indices {
                    if let Some(signal) = item_signals.get_mut(item_idx) {
                        *signal += centered;
                    }
                }
                for rune_name in &candidate.loadout_selection.rune_names {
                    *rune_signals.entry(to_norm_key(rune_name)).or_insert(0.0) += centered;
                }
                for (slot_idx, shard_stat) in
                    candidate.loadout_selection.shard_stats.iter().enumerate()
                {
                    let shard_key = format!("{}:{}", slot_idx, to_norm_key(shard_stat));
                    *shard_signals.entry(shard_key).or_insert(0.0) += centered;
                }
            }

            let mut ranked = remaining_keys
                .into_iter()
                .map(|candidate| {
                    let item_score = candidate
                        .item_indices
                        .iter()
                        .filter_map(|idx| item_signals.get(*idx).copied())
                        .sum::<f64>();
                    let rune_score = candidate
                        .loadout_selection
                        .rune_names
                        .iter()
                        .map(|rune_name| {
                            rune_signals
                                .get(&to_norm_key(rune_name))
                                .copied()
                                .unwrap_or(0.0)
                        })
                        .sum::<f64>();
                    let shard_score = candidate
                        .loadout_selection
                        .shard_stats
                        .iter()
                        .enumerate()
                        .map(|(slot_idx, shard_stat)| {
                            let shard_key = format!("{}:{}", slot_idx, to_norm_key(shard_stat));
                            shard_signals.get(&shard_key).copied().unwrap_or(0.0)
                        })
                        .sum::<f64>();
                    let heuristic_score = item_score
                        + rune_signal_weight * rune_score
                        + shard_signal_weight * shard_score;
                    (candidate, heuristic_score)
                })
                .collect::<Vec<_>>();
            ranked.sort_by(|(a_key, a_score), (b_key, b_score)| {
                b_score
                    .partial_cmp(a_score)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| build_key_cache_string(a_key).cmp(&build_key_cache_string(b_key)))
            });
            remaining_keys = ranked.into_iter().map(|(candidate, _)| candidate).collect();
        }
    }

    let mut promotions_done = 0usize;
    let mut promotion_seed = seed ^ 0x4f6d_13aa_a31b_2f17;
    for _ in 0..exploration_promotions {
        if remaining_keys.len() <= 1 {
            break;
        }
        let idx = rand_index(&mut promotion_seed, remaining_keys.len());
        if idx > 0 {
            remaining_keys.swap(0, idx);
            promotions_done += 1;
        }
    }

    (remaining_keys, promotions_done)
}

fn estimate_close_to_optimal_probability(
    evaluated_candidates: usize,
    total_candidate_space: Option<f64>,
) -> (Option<f64>, String) {
    let Some(total) = total_candidate_space else {
        return (
            None,
            "Unavailable: total legal candidate space estimate was not finite.".to_string(),
        );
    };
    if !total.is_finite() || total <= 0.0 {
        return (
            None,
            "Unavailable: total legal candidate space estimate was not positive.".to_string(),
        );
    }
    let draws = evaluated_candidates as f64;
    if draws <= 0.0 {
        return (
            Some(0.0),
            "0.0%: no unique candidates were scored in this run.".to_string(),
        );
    }
    let conservative_top_quantile = 0.00000001_f64; // top 0.000001%
    let minimum_quantile = (1.0 / total).clamp(0.0, 1.0);
    let hit_rate = conservative_top_quantile
        .max(minimum_quantile)
        .clamp(0.0, 1.0);
    let probability = if hit_rate >= 1.0 {
        1.0
    } else {
        1.0 - (1.0 - hit_rate).powf(draws)
    };
    let implied_top_candidate_count = (hit_rate * total).max(1.0).round();
    let note = format!(
        "Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = {:.9}% (about top {:.0} candidates in the legal space) and n = {} unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.",
        hit_rate * 100.0,
        implied_top_candidate_count,
        evaluated_candidates
    );
    (Some(probability.clamp(0.0, 1.0)), note)
}

fn format_percent_display(percent: f64) -> String {
    if !percent.is_finite() {
        return percent.to_string();
    }
    if percent > 0.0 && percent < 0.000001 {
        format!("{percent:.3e}%")
    } else {
        format!("{percent:.6}%")
    }
}

fn format_repo_relative_path(path: &Path) -> String {
    if !path.is_absolute() {
        return path.display().to_string();
    }
    let repository_root = simulation_dir()
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(simulation_dir);
    path.strip_prefix(&repository_root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

fn parse_controlled_champion_config(
    scenario: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
    default_level: usize,
    default_stack_overrides: &HashMap<String, f64>,
) -> Result<ControlledChampionScenarioConfig> {
    let controlled_champion = scenario
        .get("controlled_champion")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("Missing controlled_champion object"))?;
    let champion_name = controlled_champion
        .get("champion")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Missing controlled_champion.champion"))?;
    if controlled_champion.get("baseline_items").is_some() {
        return Err(anyhow!(
            "controlled_champion.baseline_items is no longer supported."
        ));
    }
    let loadout_selection = parse_loadout_selection(controlled_champion.get("loadout"))?;
    let champion_base = lookup_champion_base(champion_bases, champion_name)?;
    let level = controlled_champion
        .get("level")
        .and_then(Value::as_u64)
        .unwrap_or(default_level as u64)
        .max(1) as usize;
    if controlled_champion.get("assumptions").is_some() {
        return Err(anyhow!(
            "controlled_champion.assumptions is no longer supported. Use controlled_champion.stack_overrides."
        ));
    }
    if controlled_champion.get("item_stacks_at_level_20").is_some() {
        return Err(anyhow!(
            "controlled_champion.item_stacks_at_level_20 is no longer supported. Use controlled_champion.stack_overrides."
        ));
    }
    let mut stack_overrides = default_stack_overrides.clone();
    stack_overrides.extend(parse_stack_overrides_map(
        controlled_champion.get("stack_overrides"),
    )?);
    Ok(ControlledChampionScenarioConfig {
        base: champion_base,
        level,
        loadout_selection,
        stack_overrides,
    })
}

fn parse_opponent_encounters(
    scenario: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
    default_level: usize,
    default_stack_overrides: &HashMap<String, f64>,
) -> Result<Vec<(String, f64, Vec<EnemyConfig>)>> {
    let opponents = scenario
        .get("opponents")
        .and_then(Value::as_object)
        .ok_or_else(|| anyhow!("Missing opponents object"))?;
    let opponent_default_level = opponents
        .get("default_level")
        .and_then(Value::as_u64)
        .unwrap_or(default_level as u64)
        .max(1) as usize;
    if opponents.get("assumptions").is_some() {
        return Err(anyhow!(
            "opponents.assumptions is no longer supported. Use opponents.stack_overrides."
        ));
    }
    if opponents.get("item_stacks_at_level_20").is_some() {
        return Err(anyhow!(
            "opponents.item_stacks_at_level_20 is no longer supported. Use opponents.stack_overrides."
        ));
    }
    if opponents.get("shared_loadout").is_some() {
        return Err(anyhow!(
            "opponents.shared_loadout is no longer supported. Enemy champions always use their own preset rune pages and shard selections."
        ));
    }
    if opponents.get("uptime_windows_enabled").is_some() {
        return Err(anyhow!(
            "opponents.uptime_windows_enabled is no longer supported. Enemy combat windows are modeled by champion scripts and runtime state."
        ));
    }
    let mut opponent_default_stack_overrides = default_stack_overrides.clone();
    opponent_default_stack_overrides
        .extend(parse_stack_overrides_map(opponents.get("stack_overrides"))?);
    let encounters = opponents
        .get("encounters")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing opponents.encounters"))?;
    if encounters.is_empty() {
        return Err(anyhow!(
            "opponents.encounters must include at least one encounter"
        ));
    }
    let mut parsed = Vec::with_capacity(encounters.len());
    let mut total_weight = 0.0;
    let mut positive_weight_count = 0usize;
    for (index, encounter) in encounters.iter().enumerate() {
        let name = encounter
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Missing opponents.encounters[{index}].name"))?;
        let weight = encounter
            .get("weight")
            .and_then(Value::as_f64)
            .unwrap_or(1.0);
        if weight < 0.0 {
            return Err(anyhow!(
                "opponents.encounters[{index}].weight must be >= 0.0"
            ));
        }
        total_weight += weight;
        if weight > 0.0 {
            positive_weight_count += 1;
        }
        let actors = encounter
            .get("actors")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing opponents.encounters[{index}].actors"))?;
        if actors.is_empty() {
            return Err(anyhow!(
                "opponents.encounters[{index}].actors must include at least one actor"
            ));
        }
        let parsed_actors = actors
            .iter()
            .map(|actor| {
                parse_enemy_config(
                    actor,
                    champion_bases,
                    opponent_default_level,
                    &opponent_default_stack_overrides,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        parsed.push((name.to_string(), weight, parsed_actors));
    }
    if positive_weight_count == 0 || total_weight <= 0.0 {
        return Err(anyhow!(
            "opponents.encounters must include at least one encounter with weight > 0.0"
        ));
    }
    Ok(parsed)
}

fn search_quality_profile_key(search_quality_profile: SearchQualityProfile) -> &'static str {
    match search_quality_profile {
        SearchQualityProfile::Fast => "fast",
        SearchQualityProfile::Balanced => "balanced",
        SearchQualityProfile::MaximumQuality => "maximum_quality",
    }
}

fn parse_scenario_search_or_default(scenario: &Value) -> Result<BuildSearchConfig> {
    if let Some(search) = scenario.get("search") {
        return parse_build_search(search);
    }
    parse_build_search(&json!({ "strategy": "portfolio" }))
}

fn unique_loadout_selection_count(candidates: &[BuildKey]) -> usize {
    candidates
        .iter()
        .map(|candidate| loadout_selection_key(&candidate.loadout_selection))
        .collect::<HashSet<_>>()
        .len()
}

fn unique_loadout_selection_count_from_ranked(ranked: &[(BuildKey, f64)]) -> usize {
    ranked
        .iter()
        .map(|(candidate, _)| loadout_selection_key(&candidate.loadout_selection))
        .collect::<HashSet<_>>()
        .len()
}

fn runtime_budget_key(max_runtime_seconds: Option<f64>) -> String {
    match max_runtime_seconds {
        Some(seconds) if seconds > 0.0 => {
            let rounded = seconds.round();
            if (seconds - rounded).abs() < 1e-9 {
                format!("{rounded:.0}s")
            } else {
                format!("{seconds:.1}s")
            }
        }
        _ => "no_hard_cap".to_string(),
    }
}

fn format_seconds_key(seconds: f64) -> String {
    let rounded = seconds.round();
    if (seconds - rounded).abs() < 1e-9 {
        format!("{rounded:.0}s")
    } else {
        format!("{seconds:.1}s")
    }
}

fn format_percent_key(percent: f64) -> String {
    let clamped = percent.max(0.0);
    let rounded = clamped.round();
    let rendered = if (clamped - rounded).abs() < 1e-9 {
        format!("{rounded:.0}")
    } else {
        format!("{clamped:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    };
    rendered.replace('.', "_")
}

fn runtime_stop_key(
    max_runtime_seconds: Option<f64>,
    popcorn_window_seconds: Option<f64>,
    popcorn_min_relative_improvement_percent: f64,
) -> String {
    let budget = runtime_budget_key(max_runtime_seconds);
    match popcorn_window_seconds {
        Some(seconds) if seconds > 0.0 => {
            let min_improvement = format_percent_key(popcorn_min_relative_improvement_percent);
            let popcorn_window = format_seconds_key(seconds);
            if popcorn_window == budget {
                format!("{budget}__popcorn__min_improvement_{min_improvement}pct")
            } else {
                format!("{budget}__popcorn_{popcorn_window}__min_improvement_{min_improvement}pct")
            }
        }
        _ => budget,
    }
}

fn structured_trace_event(line: &str) -> Value {
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

fn default_run_output_directory(
    search_quality_profile: SearchQualityProfile,
    max_runtime_seconds: Option<f64>,
    popcorn_window_seconds: Option<f64>,
    popcorn_min_relative_improvement_percent: f64,
) -> PathBuf {
    simulation_dir()
        .join("output")
        .join("runs")
        .join("controlled_champion")
        .join(search_quality_profile_key(search_quality_profile))
        .join(runtime_stop_key(
            max_runtime_seconds,
            popcorn_window_seconds,
            popcorn_min_relative_improvement_percent,
        ))
}

fn default_fixed_loadout_output_directory(
    search_quality_profile: SearchQualityProfile,
    run_label: &str,
) -> PathBuf {
    simulation_dir()
        .join("output")
        .join("runs")
        .join("controlled_champion")
        .join("fixed_loadout")
        .join(search_quality_profile_key(search_quality_profile))
        .join(to_norm_key(run_label))
}

fn default_fixed_loadout_rune_sweep_output_directory(
    search_quality_profile: SearchQualityProfile,
    run_label: &str,
) -> PathBuf {
    simulation_dir()
        .join("output")
        .join("runs")
        .join("controlled_champion")
        .join("fixed_loadout_rune_sweep")
        .join(search_quality_profile_key(search_quality_profile))
        .join(to_norm_key(run_label))
}

pub(super) fn run_controlled_champion_fixed_loadout_evaluation(
    scenario_path: &Path,
    options: &ControlledChampionFixedLoadoutOptions<'_>,
) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim = parse_simulation_config(simulation_config)?;
    sim.collect_rune_proc_telemetry = false;
    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim.champion_level;
    sim.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        simulation_config,
        simulation_level_before_controlled_override,
    );

    let mut controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim.champion_level);
    let controlled_champion_name = controlled_champion_base.name.clone();
    sim.controlled_champion_script = resolve_controlled_champion_script(&controlled_champion_name);

    let loadout_domain = build_loadout_domain();
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    if let Some(runes) = &options.fixed_rune_names {
        controlled_champion_loadout_selection.rune_names = runes.clone();
    }
    if let Some(shards) = &options.fixed_shard_stats {
        controlled_champion_loadout_selection.shard_stats = shards.clone();
    }
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    let controlled_champion_loadout = resolve_loadout(
        &controlled_champion_loadout_selection,
        sim.champion_level,
        true,
    )?;
    let fixed_build_items = item_pool_from_names(&items, &options.fixed_item_names)?;

    let enemy_scenarios_raw = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|(name, weight, enemies)| {
            let scaled = enemies
                .iter()
                .cloned()
                .map(|mut enemy| {
                    enemy.base = champion_at_level(&enemy.base, enemy.level);
                    enemy
                })
                .collect::<Vec<_>>();
            (name.clone(), *weight, scaled)
        })
        .collect::<Vec<_>>();

    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;
    let mut enemy_build_scenarios = Vec::new();
    for (name, weight, enemies) in &enemy_scenarios {
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(&items, &preset.item_names)?;
            let preset_enemy_loadout =
                resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?;
            let enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }
    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, builds)| builds.clone())
        .unwrap_or_default();

    let mut search_cfg = parse_scenario_search_or_default(&scenario)?;
    apply_search_quality_profile(&mut search_cfg, options.search_quality_profile);
    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
        search_cfg.objective_enemy_kills_weight,
        search_cfg.objective_invulnerable_seconds_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            let damage_reference = enemy_builds_s
                .iter()
                .map(|(enemy, build, bonus_stats)| {
                    derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf).max_health
                })
                .sum::<f64>()
                .max(1.0);
            CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: damage_reference,
                healing_done: controlled_champion_base.base_health.max(1.0),
                enemy_kills: enemy_builds_s.len().max(1),
                invulnerable_seconds: sim.max_time_seconds.max(1.0),
            }
        })
        .collect::<Vec<_>>();
    let objective_eval_ctx = ObjectiveEvalContext {
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        enemy_build_scenarios: &enemy_build_scenarios,
        sim: &sim,
        urf: &urf,
        scenario_reference_outcomes: &scenario_reference_outcomes,
        weights: objective_component_weights,
        worst_case_weight: objective_worst_case_weight,
    };
    let (fixed_score, fixed_outcome, fixed_breakdown) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            &objective_eval_ctx,
            &fixed_build_items,
            &controlled_champion_loadout.bonus_stats,
            Some(&controlled_champion_loadout_selection),
        );

    let run_label = options
        .fixed_eval_label
        .as_deref()
        .unwrap_or("fixed_loadout");
    let default_output_dir =
        default_fixed_loadout_output_directory(options.search_quality_profile, run_label);
    fs::create_dir_all(&default_output_dir)?;
    let controlled_champion_key = to_norm_key(&controlled_champion_name);
    let report_path = options
        .report_path_override
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            default_output_dir.join(format!("{controlled_champion_key}_fixed_loadout_report.md"))
        });
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json_report_path = report_path.with_extension("json");
    let trace_markdown_path =
        default_output_dir.join(format!("{controlled_champion_key}_fixed_loadout_trace.md"));
    let trace_json_path = default_output_dir.join(format!(
        "{controlled_champion_key}_fixed_loadout_trace.json"
    ));

    let mut trace_sim_cfg = sim.clone();
    trace_sim_cfg.collect_rune_proc_telemetry = true;
    let mut trace_sim = ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
        controlled_champion_base.clone(),
        &fixed_build_items,
        &controlled_champion_loadout.bonus_stats,
        Some(&controlled_champion_loadout_selection),
        None,
        Some(&controlled_champion_stack_overrides),
        &enemy_builds,
        trace_sim_cfg,
        urf.clone(),
    );
    trace_sim.enable_trace();
    while trace_sim.step(1) {}
    let trace_events = trace_sim.trace_events();
    let rune_proc_telemetry = trace_sim.controlled_champion_rune_proc_telemetry();
    let mut trace_markdown = String::new();
    trace_markdown.push_str("# Controlled Champion Fixed Loadout Trace\n\n");
    trace_markdown.push_str("## Rune Proc Telemetry\n");
    append_rune_proc_telemetry_markdown_entries(
        &mut trace_markdown,
        "- ",
        "  ",
        &rune_proc_telemetry,
        fixed_outcome.damage_dealt,
        fixed_outcome.healing_done,
    );
    trace_markdown.push_str("\n## Events\n");
    for entry in trace_events {
        trace_markdown.push_str("- ");
        trace_markdown.push_str(entry);
        trace_markdown.push('\n');
    }
    fs::write(&trace_markdown_path, trace_markdown)?;
    let trace_json = json!({
        "schema_version": FIXED_LOADOUT_TRACE_JSON_SCHEMA_VERSION,
        "event_encoding": "structured",
        "rune_proc_telemetry": rune_proc_telemetry_json(
            &rune_proc_telemetry,
            fixed_outcome.damage_dealt,
            fixed_outcome.healing_done,
        ),
        "events": trace_events
            .iter()
            .map(|entry| structured_trace_event(entry))
            .collect::<Vec<_>>(),
    });
    fs::write(&trace_json_path, serde_json::to_string_pretty(&trace_json)?)?;

    let mut markdown_report = format!(
        "# Controlled Champion Fixed Loadout Evaluation\n\n- Scenario: `{}`\n- Search quality profile: `{}`\n- Controlled champion: `{}`\n- Build items: `{}`\n- Runes: `{}`\n- Shards: `{}`\n\n## Headline\n- Objective score: **{:.4}**\n- Outcome:\n  - Time alive: **{:.2}s**\n  - Damage dealt: **{:.1}**\n  - Healing done: **{:.1}**\n  - Enemy kills: **{}**\n  - Invulnerable seconds: **{:.2}s**\n\n## Objective Score Breakdown\n- Weighted-mean score: `{:.4}`\n- Worst-case scenario score: `{:.4}`\n- Worst-case blend weight: `{:.2}`\n- Final blended objective score: `{:.4}`\n- survival: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- damage: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- healing: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- enemy_kills: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n- invulnerable_seconds: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%`\n\n## Notes\n- This mode evaluates one fixed build and loadout directly; no candidate search or mutation is performed.\n- Trace markdown: `{}`\n- Trace json: `{}`\n",
        scenario_path.display(),
        search_quality_profile_key(options.search_quality_profile),
        controlled_champion_name,
        fixed_build_items
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        controlled_champion_loadout_selection.rune_names.join(", "),
        controlled_champion_loadout_selection.shard_stats.join(", "),
        fixed_score,
        fixed_outcome.time_alive_seconds,
        fixed_outcome.damage_dealt,
        fixed_outcome.healing_done,
        fixed_outcome.enemy_kills,
        fixed_outcome.invulnerable_seconds,
        fixed_breakdown.weighted_mean_score,
        fixed_breakdown.worst_case_score,
        fixed_breakdown.worst_case_weight,
        fixed_breakdown.final_score,
        fixed_breakdown.survival.weight,
        fixed_breakdown.survival.normalized_ratio,
        fixed_breakdown.survival.contribution,
        fixed_breakdown.survival.impact_percent,
        fixed_breakdown.damage.weight,
        fixed_breakdown.damage.normalized_ratio,
        fixed_breakdown.damage.contribution,
        fixed_breakdown.damage.impact_percent,
        fixed_breakdown.healing.weight,
        fixed_breakdown.healing.normalized_ratio,
        fixed_breakdown.healing.contribution,
        fixed_breakdown.healing.impact_percent,
        fixed_breakdown.enemy_kills.weight,
        fixed_breakdown.enemy_kills.normalized_ratio,
        fixed_breakdown.enemy_kills.contribution,
        fixed_breakdown.enemy_kills.impact_percent,
        fixed_breakdown.invulnerable_seconds.weight,
        fixed_breakdown.invulnerable_seconds.normalized_ratio,
        fixed_breakdown.invulnerable_seconds.contribution,
        fixed_breakdown.invulnerable_seconds.impact_percent,
        format_repo_relative_path(&trace_markdown_path),
        format_repo_relative_path(&trace_json_path),
    );
    markdown_report.push_str("\n## Rune Proc Telemetry\n");
    if rune_proc_telemetry.is_empty() {
        markdown_report.push_str("- No rune procs were recorded.\n");
    } else {
        append_rune_proc_telemetry_markdown_entries(
            &mut markdown_report,
            "- ",
            "  ",
            &rune_proc_telemetry,
            fixed_outcome.damage_dealt,
            fixed_outcome.healing_done,
        );
    }
    markdown_report.push('\n');
    fs::write(&report_path, markdown_report)?;

    let structured_report = json!({
        "schema_version": FIXED_LOADOUT_REPORT_JSON_SCHEMA_VERSION,
        "scenario_path": scenario_path.display().to_string(),
        "search_quality_profile": search_quality_profile_key(options.search_quality_profile),
        "controlled_champion_name": controlled_champion_name,
        "items": fixed_build_items.iter().map(|item| item.name.clone()).collect::<Vec<_>>(),
        "runes": controlled_champion_loadout_selection.rune_names,
        "shards": controlled_champion_loadout_selection.shard_stats,
        "objective_score": fixed_score,
        "outcome": {
            "time_alive_seconds": fixed_outcome.time_alive_seconds,
            "damage_dealt": fixed_outcome.damage_dealt,
            "healing_done": fixed_outcome.healing_done,
            "enemy_kills": fixed_outcome.enemy_kills,
            "invulnerable_seconds": fixed_outcome.invulnerable_seconds
        },
        "objective_breakdown": {
            "weighted_mean_score": fixed_breakdown.weighted_mean_score,
            "worst_case_score": fixed_breakdown.worst_case_score,
            "worst_case_weight": fixed_breakdown.worst_case_weight,
            "final_score": fixed_breakdown.final_score,
            "components": {
                "survival": {
                    "weight": fixed_breakdown.survival.weight,
                    "normalized_ratio": fixed_breakdown.survival.normalized_ratio,
                    "contribution": fixed_breakdown.survival.contribution,
                    "impact_percent": fixed_breakdown.survival.impact_percent
                },
                "damage": {
                    "weight": fixed_breakdown.damage.weight,
                    "normalized_ratio": fixed_breakdown.damage.normalized_ratio,
                    "contribution": fixed_breakdown.damage.contribution,
                    "impact_percent": fixed_breakdown.damage.impact_percent
                },
                "healing": {
                    "weight": fixed_breakdown.healing.weight,
                    "normalized_ratio": fixed_breakdown.healing.normalized_ratio,
                    "contribution": fixed_breakdown.healing.contribution,
                    "impact_percent": fixed_breakdown.healing.impact_percent
                },
                "enemy_kills": {
                    "weight": fixed_breakdown.enemy_kills.weight,
                    "normalized_ratio": fixed_breakdown.enemy_kills.normalized_ratio,
                    "contribution": fixed_breakdown.enemy_kills.contribution,
                    "impact_percent": fixed_breakdown.enemy_kills.impact_percent
                },
                "invulnerable_seconds": {
                    "weight": fixed_breakdown.invulnerable_seconds.weight,
                    "normalized_ratio": fixed_breakdown.invulnerable_seconds.normalized_ratio,
                    "contribution": fixed_breakdown.invulnerable_seconds.contribution,
                    "impact_percent": fixed_breakdown.invulnerable_seconds.impact_percent
                }
            }
        },
        "reference_outcome": {
            "time_alive_seconds": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.time_alive_seconds)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64),
            "damage_dealt": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.damage_dealt)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64),
            "healing_done": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.healing_done)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64),
            "enemy_kills": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.enemy_kills)
                .sum::<usize>()
                / scenario_reference_outcomes.len().max(1),
            "invulnerable_seconds": scenario_reference_outcomes
                .iter()
                .map(|outcome| outcome.invulnerable_seconds)
                .sum::<f64>()
                / (scenario_reference_outcomes.len().max(1) as f64)
        },
        "rune_proc_telemetry": rune_proc_telemetry_json(
            &rune_proc_telemetry,
            fixed_outcome.damage_dealt,
            fixed_outcome.healing_done,
        ),
        "notes": [
            "No search stage is run in controlled_champion_fixed_loadout mode.",
            "This report is intended for direct loadout-to-loadout comparisons."
        ]
    });
    fs::write(
        &json_report_path,
        serde_json::to_string_pretty(&structured_report)?,
    )?;

    println!(
        "Fixed-loadout report written: {}",
        format_repo_relative_path(&report_path)
    );
    println!(
        "Fixed-loadout JSON written: {}",
        format_repo_relative_path(&json_report_path)
    );
    println!(
        "Fixed-loadout trace written: {}",
        format_repo_relative_path(&trace_markdown_path)
    );

    Ok(())
}

pub(super) fn run_controlled_champion_fixed_loadout_rune_sweep(
    scenario_path: &Path,
    options: &ControlledChampionFixedLoadoutOptions<'_>,
) -> Result<()> {
    #[derive(Debug, Clone)]
    struct RuneSweepEntry {
        keystone_name: String,
        loadout_selection: LoadoutSelection,
        objective_score: f64,
        outcome: CombatOutcome,
        objective_breakdown: ObjectiveScoreBreakdown,
        rune_proc_telemetry: Vec<crate::scripts::champions::ChampionRuneProcTelemetryEntry>,
        seed_repeat_scores: Vec<f64>,
        seed_repeat_values: Vec<u64>,
    }

    fn average_combat_outcomes(outcomes: &[CombatOutcome]) -> CombatOutcome {
        if outcomes.is_empty() {
            return CombatOutcome::default();
        }
        let n = outcomes.len() as f64;
        CombatOutcome {
            time_alive_seconds: outcomes
                .iter()
                .map(|outcome| outcome.time_alive_seconds)
                .sum::<f64>()
                / n,
            damage_dealt: outcomes
                .iter()
                .map(|outcome| outcome.damage_dealt)
                .sum::<f64>()
                / n,
            healing_done: outcomes
                .iter()
                .map(|outcome| outcome.healing_done)
                .sum::<f64>()
                / n,
            enemy_kills: ((outcomes
                .iter()
                .map(|outcome| outcome.enemy_kills as f64)
                .sum::<f64>()
                / n)
                .round()
                .max(0.0)) as usize,
            invulnerable_seconds: outcomes
                .iter()
                .map(|outcome| outcome.invulnerable_seconds)
                .sum::<f64>()
                / n,
        }
    }

    fn average_component_impacts(impacts: &[ObjectiveComponentImpact]) -> ObjectiveComponentImpact {
        if impacts.is_empty() {
            return ObjectiveComponentImpact::default();
        }
        let n = impacts.len() as f64;
        ObjectiveComponentImpact {
            weight: impacts.iter().map(|impact| impact.weight).sum::<f64>() / n,
            normalized_ratio: impacts
                .iter()
                .map(|impact| impact.normalized_ratio)
                .sum::<f64>()
                / n,
            contribution: impacts
                .iter()
                .map(|impact| impact.contribution)
                .sum::<f64>()
                / n,
            impact_percent: impacts
                .iter()
                .map(|impact| impact.impact_percent)
                .sum::<f64>()
                / n,
        }
    }

    fn average_objective_breakdowns(
        breakdowns: &[ObjectiveScoreBreakdown],
    ) -> ObjectiveScoreBreakdown {
        if breakdowns.is_empty() {
            return ObjectiveScoreBreakdown::default();
        }
        let n = breakdowns.len() as f64;
        ObjectiveScoreBreakdown {
            weighted_mean_score: breakdowns
                .iter()
                .map(|breakdown| breakdown.weighted_mean_score)
                .sum::<f64>()
                / n,
            worst_case_score: breakdowns
                .iter()
                .map(|breakdown| breakdown.worst_case_score)
                .sum::<f64>()
                / n,
            worst_case_weight: breakdowns
                .iter()
                .map(|breakdown| breakdown.worst_case_weight)
                .sum::<f64>()
                / n,
            final_score: breakdowns
                .iter()
                .map(|breakdown| breakdown.final_score)
                .sum::<f64>()
                / n,
            survival: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.survival)
                    .collect::<Vec<_>>(),
            ),
            damage: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.damage)
                    .collect::<Vec<_>>(),
            ),
            healing: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.healing)
                    .collect::<Vec<_>>(),
            ),
            enemy_kills: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.enemy_kills)
                    .collect::<Vec<_>>(),
            ),
            invulnerable_seconds: average_component_impacts(
                &breakdowns
                    .iter()
                    .map(|breakdown| breakdown.invulnerable_seconds)
                    .collect::<Vec<_>>(),
            ),
        }
    }

    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim = parse_simulation_config(simulation_config)?;
    sim.collect_rune_proc_telemetry = false;
    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim.champion_level;
    sim.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        simulation_config,
        simulation_level_before_controlled_override,
    );

    let mut controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim.champion_level);
    let controlled_champion_name = controlled_champion_base.name.clone();
    sim.controlled_champion_script = resolve_controlled_champion_script(&controlled_champion_name);

    let loadout_domain = build_loadout_domain();
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    if let Some(runes) = &options.fixed_rune_names {
        controlled_champion_loadout_selection.rune_names = runes.clone();
    }
    if let Some(shards) = &options.fixed_shard_stats {
        controlled_champion_loadout_selection.shard_stats = shards.clone();
    }
    controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    let fixed_build_items = item_pool_from_names(&items, &options.fixed_item_names)?;

    let baseline_keystone = controlled_champion_loadout_selection
        .rune_names
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("Controlled champion rune page must include a primary keystone"))?;
    let baseline_keystone_key = to_norm_key(&baseline_keystone);
    let primary_keystone_slot = loadout_domain
        .rune_paths
        .iter()
        .find_map(|path| {
            path.slot_runes.first().and_then(|slot| {
                slot.iter()
                    .any(|rune| to_norm_key(rune) == baseline_keystone_key)
                    .then_some(slot.clone())
            })
        })
        .ok_or_else(|| {
            anyhow!(
                "Unable to resolve primary rune path for baseline keystone '{}'",
                baseline_keystone
            )
        })?;
    let mut keystone_candidates = primary_keystone_slot
        .into_iter()
        .map(|name| (to_norm_key(&name), name))
        .collect::<Vec<_>>();
    keystone_candidates.sort_by(|a, b| a.0.cmp(&b.0));
    keystone_candidates.dedup_by(|a, b| a.0 == b.0);
    let keystone_candidates = keystone_candidates
        .into_iter()
        .map(|(_, name)| name)
        .collect::<Vec<_>>();
    if keystone_candidates.is_empty() {
        return Err(anyhow!(
            "No keystone candidates found for baseline rune path '{}'",
            baseline_keystone
        ));
    }

    let enemy_scenarios_raw = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|(name, weight, enemies)| {
            let scaled = enemies
                .iter()
                .cloned()
                .map(|mut enemy| {
                    enemy.base = champion_at_level(&enemy.base, enemy.level);
                    enemy
                })
                .collect::<Vec<_>>();
            (name.clone(), *weight, scaled)
        })
        .collect::<Vec<_>>();

    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;
    let mut enemy_build_scenarios = Vec::new();
    for (name, weight, enemies) in &enemy_scenarios {
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(&items, &preset.item_names)?;
            let preset_enemy_loadout =
                resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?;
            let enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }
    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, builds)| builds.clone())
        .unwrap_or_default();

    let mut search_cfg = parse_scenario_search_or_default(&scenario)?;
    apply_search_quality_profile(&mut search_cfg, options.search_quality_profile);
    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
        search_cfg.objective_enemy_kills_weight,
        search_cfg.objective_invulnerable_seconds_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            let damage_reference = enemy_builds_s
                .iter()
                .map(|(enemy, build, bonus_stats)| {
                    derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf).max_health
                })
                .sum::<f64>()
                .max(1.0);
            CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: damage_reference,
                healing_done: controlled_champion_base.base_health.max(1.0),
                enemy_kills: enemy_builds_s.len().max(1),
                invulnerable_seconds: sim.max_time_seconds.max(1.0),
            }
        })
        .collect::<Vec<_>>();
    let objective_eval_ctx = ObjectiveEvalContext {
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        enemy_build_scenarios: &enemy_build_scenarios,
        sim: &sim,
        urf: &urf,
        scenario_reference_outcomes: &scenario_reference_outcomes,
        weights: objective_component_weights,
        worst_case_weight: objective_worst_case_weight,
    };
    let sweep_seed_repeats = options.fixed_sweep_seed_repeats.max(1);
    let sweep_results_parallel = keystone_candidates
        .par_iter()
        .map(|keystone| -> Result<RuneSweepEntry> {
            let mut loadout_selection = controlled_champion_loadout_selection.clone();
            if let Some(primary_slot) = loadout_selection.rune_names.first_mut() {
                *primary_slot = keystone.clone();
            }
            loadout_selection =
                ensure_complete_loadout_selection(&loadout_selection, &loadout_domain)?;
            let resolved_loadout = resolve_loadout(&loadout_selection, sim.champion_level, true)?;
            let keystone_seed_base = fixed_sweep_keystone_seed_base(search_cfg.seed, keystone);
            let mut repeat_results = (0..sweep_seed_repeats)
                .into_par_iter()
                .map(|repeat_idx| {
                    let repeat_seed = fixed_sweep_repeat_seed(keystone_seed_base, repeat_idx);
                    let mut repeat_sim = sim.clone();
                    repeat_sim.combat_seed = Some(repeat_seed);
                    let repeat_objective_eval_ctx = ObjectiveEvalContext {
                        controlled_champion_base: objective_eval_ctx.controlled_champion_base,
                        controlled_champion_stack_overrides: objective_eval_ctx
                            .controlled_champion_stack_overrides,
                        enemy_build_scenarios: objective_eval_ctx.enemy_build_scenarios,
                        sim: &repeat_sim,
                        urf: objective_eval_ctx.urf,
                        scenario_reference_outcomes: objective_eval_ctx.scenario_reference_outcomes,
                        weights: objective_eval_ctx.weights,
                        worst_case_weight: objective_eval_ctx.worst_case_weight,
                    };
                    let (score, outcome, breakdown) =
                        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
                            &repeat_objective_eval_ctx,
                            &fixed_build_items,
                            &resolved_loadout.bonus_stats,
                            Some(&loadout_selection),
                        );
                    (repeat_idx, repeat_seed, score, outcome, breakdown)
                })
                .collect::<Vec<_>>();
            repeat_results.sort_by_key(|entry| entry.0);
            let seed_repeat_values = repeat_results
                .iter()
                .map(|entry| entry.1)
                .collect::<Vec<_>>();
            let seed_repeat_scores = repeat_results
                .iter()
                .map(|entry| entry.2)
                .collect::<Vec<_>>();
            let repeated_outcomes = repeat_results
                .iter()
                .map(|entry| entry.3)
                .collect::<Vec<_>>();
            let repeated_breakdowns = repeat_results
                .iter()
                .map(|entry| entry.4)
                .collect::<Vec<_>>();

            let objective_score =
                seed_repeat_scores.iter().sum::<f64>() / seed_repeat_scores.len().max(1) as f64;
            let outcome = average_combat_outcomes(&repeated_outcomes);
            let objective_breakdown = average_objective_breakdowns(&repeated_breakdowns);
            let mut trace_sim_cfg = sim.clone();
            trace_sim_cfg.collect_rune_proc_telemetry = true;
            if let Some(seed) = seed_repeat_values.first().copied() {
                trace_sim_cfg.combat_seed = Some(seed);
            }

            let mut trace_sim =
                ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
                    controlled_champion_base.clone(),
                    &fixed_build_items,
                    &resolved_loadout.bonus_stats,
                    Some(&loadout_selection),
                    None,
                    Some(&controlled_champion_stack_overrides),
                    &enemy_builds,
                    trace_sim_cfg,
                    urf.clone(),
                );
            while trace_sim.step(1) {}

            Ok(RuneSweepEntry {
                keystone_name: keystone.clone(),
                loadout_selection,
                objective_score,
                outcome,
                objective_breakdown,
                rune_proc_telemetry: trace_sim.controlled_champion_rune_proc_telemetry(),
                seed_repeat_scores,
                seed_repeat_values,
            })
        })
        .collect::<Vec<_>>();
    let mut sweep_results = sweep_results_parallel
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    sweep_results.sort_by(|a, b| {
        b.objective_score
            .partial_cmp(&a.objective_score)
            .unwrap_or(Ordering::Equal)
            .then_with(|| to_norm_key(&a.keystone_name).cmp(&to_norm_key(&b.keystone_name)))
    });

    let run_label = options
        .fixed_eval_label
        .as_deref()
        .unwrap_or("fixed_loadout_rune_sweep");
    let default_output_dir = default_fixed_loadout_rune_sweep_output_directory(
        options.search_quality_profile,
        run_label,
    );
    fs::create_dir_all(&default_output_dir)?;
    let controlled_champion_key = to_norm_key(&controlled_champion_name);
    let report_path = options
        .report_path_override
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            default_output_dir.join(format!(
                "{controlled_champion_key}_fixed_loadout_rune_sweep_report.md"
            ))
        });
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json_report_path = report_path.with_extension("json");

    let best_score = sweep_results
        .first()
        .map(|entry| entry.objective_score)
        .unwrap_or(0.0);
    let mut markdown = String::new();
    markdown.push_str("# Controlled Champion Fixed Loadout Rune Sweep\n\n");
    markdown.push_str(&format!("- Scenario: `{}`\n", scenario_path.display()));
    markdown.push_str(&format!(
        "- Search quality profile: `{}`\n",
        search_quality_profile_key(options.search_quality_profile)
    ));
    markdown.push_str(&format!(
        "- Controlled champion: `{}`\n",
        controlled_champion_name
    ));
    markdown.push_str(&format!(
        "- Build items: `{}`\n",
        fixed_build_items
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    ));
    markdown.push_str(&format!(
        "- Baseline runes: `{}`\n",
        controlled_champion_loadout_selection.rune_names.join(", ")
    ));
    markdown.push_str(&format!(
        "- Baseline shards: `{}`\n\n",
        controlled_champion_loadout_selection.shard_stats.join(", ")
    ));
    markdown.push_str(&format!(
        "- Seed repeats per keystone: `{}`\n\n",
        sweep_seed_repeats
    ));
    markdown.push_str(&format!("- Seed base: `{}`\n\n", search_cfg.seed));
    markdown.push_str("## Rune Sweep Ranking\n");
    for (idx, result) in sweep_results.iter().enumerate() {
        if sweep_seed_repeats > 1 {
            let (_, score_stddev) = mean_std(&result.seed_repeat_scores);
            let repeat_seeds = result
                .seed_repeat_values
                .iter()
                .map(|seed| seed.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            markdown.push_str(&format!(
                "{}. `{}`\n  - Score: `{:.4}`\n  - Delta: `{:+.4}`\n  - Seed stddev: `{:.4}`\n  - Outcome:\n    - Time alive: `{:.2}s`\n    - Damage dealt: `{:.1}`\n    - Healing done: `{:.1}`\n    - Enemy kills: `{}`\n    - Invulnerable seconds: `{:.2}s`\n  - Seeds: `[{}]`\n",
                idx + 1,
                result.keystone_name,
                result.objective_score,
                result.objective_score - best_score,
                score_stddev,
                result.outcome.time_alive_seconds,
                result.outcome.damage_dealt,
                result.outcome.healing_done,
                result.outcome.enemy_kills,
                result.outcome.invulnerable_seconds,
                repeat_seeds
            ));
        } else {
            markdown.push_str(&format!(
                "{}. `{}`\n  - Score: `{:.4}`\n  - Delta: `{:+.4}`\n  - Outcome:\n    - Time alive: `{:.2}s`\n    - Damage dealt: `{:.1}`\n    - Healing done: `{:.1}`\n    - Enemy kills: `{}`\n    - Invulnerable seconds: `{:.2}s`\n",
                idx + 1,
                result.keystone_name,
                result.objective_score,
                result.objective_score - best_score,
                result.outcome.time_alive_seconds,
                result.outcome.damage_dealt,
                result.outcome.healing_done,
                result.outcome.enemy_kills,
                result.outcome.invulnerable_seconds
            ));
        }
    }
    markdown.push('\n');
    markdown.push_str("## Rune Proc Telemetry\n");
    for result in &sweep_results {
        markdown.push_str(&format!("- {}:\n", result.keystone_name));
        append_rune_proc_telemetry_markdown_entries(
            &mut markdown,
            "  - ",
            "    ",
            &result.rune_proc_telemetry,
            result.outcome.damage_dealt,
            result.outcome.healing_done,
        );
    }
    fs::write(&report_path, markdown)?;

    let json_report = json!({
        "schema_version": FIXED_LOADOUT_RUNE_SWEEP_JSON_SCHEMA_VERSION,
        "scenario_path": scenario_path.display().to_string(),
        "search_quality_profile": search_quality_profile_key(options.search_quality_profile),
        "controlled_champion_name": controlled_champion_name,
        "items": fixed_build_items.iter().map(|item| item.name.clone()).collect::<Vec<_>>(),
        "baseline_rune_names": controlled_champion_loadout_selection.rune_names,
        "baseline_shard_stats": controlled_champion_loadout_selection.shard_stats,
        "seed_base": search_cfg.seed,
        "seed_repeats_per_keystone": sweep_seed_repeats,
        "results": sweep_results.iter().map(|result| {
            json!({
                "keystone_name": result.keystone_name,
                "loadout_selection": {
                    "rune_names": result.loadout_selection.rune_names,
                    "shard_stats": result.loadout_selection.shard_stats
                },
                "objective_score": result.objective_score,
                "outcome": {
                    "time_alive_seconds": result.outcome.time_alive_seconds,
                    "damage_dealt": result.outcome.damage_dealt,
                    "healing_done": result.outcome.healing_done,
                    "enemy_kills": result.outcome.enemy_kills,
                    "invulnerable_seconds": result.outcome.invulnerable_seconds
                },
                "objective_breakdown": {
                    "weighted_mean_score": result.objective_breakdown.weighted_mean_score,
                    "worst_case_score": result.objective_breakdown.worst_case_score,
                    "worst_case_weight": result.objective_breakdown.worst_case_weight,
                    "final_score": result.objective_breakdown.final_score,
                    "components": {
                        "survival": {
                            "weight": result.objective_breakdown.survival.weight,
                            "normalized_ratio": result.objective_breakdown.survival.normalized_ratio,
                            "contribution": result.objective_breakdown.survival.contribution,
                            "impact_percent": result.objective_breakdown.survival.impact_percent
                        },
                        "damage": {
                            "weight": result.objective_breakdown.damage.weight,
                            "normalized_ratio": result.objective_breakdown.damage.normalized_ratio,
                            "contribution": result.objective_breakdown.damage.contribution,
                            "impact_percent": result.objective_breakdown.damage.impact_percent
                        },
                        "healing": {
                            "weight": result.objective_breakdown.healing.weight,
                            "normalized_ratio": result.objective_breakdown.healing.normalized_ratio,
                            "contribution": result.objective_breakdown.healing.contribution,
                            "impact_percent": result.objective_breakdown.healing.impact_percent
                        },
                        "enemy_kills": {
                            "weight": result.objective_breakdown.enemy_kills.weight,
                            "normalized_ratio": result.objective_breakdown.enemy_kills.normalized_ratio,
                            "contribution": result.objective_breakdown.enemy_kills.contribution,
                            "impact_percent": result.objective_breakdown.enemy_kills.impact_percent
                        },
                        "invulnerable_seconds": {
                            "weight": result.objective_breakdown.invulnerable_seconds.weight,
                            "normalized_ratio": result.objective_breakdown.invulnerable_seconds.normalized_ratio,
                            "contribution": result.objective_breakdown.invulnerable_seconds.contribution,
                            "impact_percent": result.objective_breakdown.invulnerable_seconds.impact_percent
                        }
                    }
                },
                "seed_repeat_scores": result.seed_repeat_scores,
                "seed_repeat_values": result.seed_repeat_values,
                "rune_proc_telemetry": rune_proc_telemetry_json(
                    &result.rune_proc_telemetry,
                    result.outcome.damage_dealt,
                    result.outcome.healing_done,
                )
            })
        }).collect::<Vec<_>>()
    });
    fs::write(
        &json_report_path,
        serde_json::to_string_pretty(&json_report)?,
    )?;

    println!(
        "Fixed-loadout rune sweep report written: {}",
        format_repo_relative_path(&report_path)
    );
    println!(
        "Fixed-loadout rune sweep JSON written: {}",
        format_repo_relative_path(&json_report_path)
    );

    Ok(())
}

pub(super) fn run_controlled_champion_scenario(
    scenario_path: &Path,
    options: &ControlledChampionRunOptions<'_>,
) -> Result<()> {
    let top_x = options.top_x;
    let min_item_diff = options.min_item_diff;
    let max_relative_gap_percent = options.max_relative_gap_percent;
    let report_path_override = options.report_path_override;
    let max_runtime_seconds = options.max_runtime_seconds;
    let popcorn_window_seconds = options.popcorn_window_seconds.filter(|s| *s > 0.0);
    let popcorn_window = popcorn_window_seconds.map(Duration::from_secs_f64);
    let popcorn_min_relative_improvement_percent =
        options.popcorn_min_relative_improvement_percent.max(0.0);
    let popcorn_min_relative_improvement = popcorn_min_relative_improvement_percent / 100.0;
    let status_every_seconds = options.status_every_seconds;
    let seed_override = options.seed_override;
    let search_quality_profile = if popcorn_window_seconds.is_some() {
        SearchQualityProfile::MaximumQuality
    } else {
        options.search_quality_profile
    };

    let run_start = Instant::now();
    let time_budget = max_runtime_seconds
        .filter(|s| *s > 0.0)
        .map(Duration::from_secs_f64);
    let defer_hard_budget_until_coverage =
        matches!(search_quality_profile, SearchQualityProfile::MaximumQuality);
    let hard_deadline_state = Arc::new(Mutex::new(None::<Instant>));
    let progress_state = Arc::new(Mutex::new(SignificantProgressState {
        best_overall_score: f64::NEG_INFINITY,
        best_significant_score: f64::NEG_INFINITY,
        significant_events: 0,
        last_significant_at: run_start,
    }));
    let hard_deadline_value = || hard_deadline_state.lock().ok().and_then(|state| *state);
    let coverage_stage_deadline = || hard_deadline_value();
    let current_deadline = || {
        let hard_deadline = hard_deadline_value();
        let progress_deadline = popcorn_window.map(|window| {
            let last_significant_at = progress_state
                .lock()
                .ok()
                .map(|state| state.last_significant_at)
                .unwrap_or(run_start);
            last_significant_at + window
        });
        match (hard_deadline, progress_deadline) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    };
    let deadline_for_search_type = |search_type: &str| {
        if search_type == "coverage_stage" {
            coverage_stage_deadline()
        } else {
            current_deadline()
        }
    };
    let record_score_progress = |score: f64| {
        if !score.is_finite() {
            return;
        }
        if let Ok(mut state) = progress_state.lock() {
            let previous_best_overall = state.best_overall_score;
            if score > state.best_overall_score {
                state.best_overall_score = score;
            }
            let significant = if !state.best_significant_score.is_finite() {
                true
            } else {
                let previous_best = previous_best_overall;
                let delta = if previous_best.is_finite() {
                    score - previous_best
                } else {
                    score - state.best_significant_score
                };
                if delta <= 0.0 {
                    false
                } else {
                    let threshold_base = if previous_best.is_finite() {
                        previous_best.abs().max(1e-9)
                    } else {
                        state.best_significant_score.abs().max(1e-9)
                    };
                    let threshold = threshold_base * popcorn_min_relative_improvement;
                    delta >= threshold
                }
            };
            if significant {
                state.best_significant_score = score;
                state.last_significant_at = Instant::now();
                state.significant_events += 1;
            }
        }
    };
    let status_every = Duration::from_secs_f64(status_every_seconds.max(1.0));
    let mut status = StatusReporter::new(run_start, status_every);
    let timeout_flag = Arc::new(AtomicUsize::new(0));
    status.emit("initialization", None, None, Some("starting"), true);
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;
    status.emit("initialization", None, None, Some("core data loaded"), true);

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim = parse_simulation_config(simulation_config)?;
    sim.collect_rune_proc_telemetry = false;
    if deadline_reached(current_deadline()) {
        timeout_flag.store(1, AtomicOrdering::Relaxed);
    }

    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim.champion_level;
    sim.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim,
        simulation_config,
        simulation_level_before_controlled_override,
    );
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim.champion_level);
    let controlled_champion_base_raw = controlled_champion_config.base;
    let controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;
    let controlled_champion_name = controlled_champion_base_raw.name.clone();
    sim.controlled_champion_script = resolve_controlled_champion_script(&controlled_champion_name);

    let enemy_scenarios_raw = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim.champion_level,
        &sim.stack_overrides,
    )?;
    let raw_enemy_bases = enemy_scenarios_raw
        .iter()
        .flat_map(|(_, _, enemies)| enemies.iter())
        .fold(HashMap::new(), |mut map, enemy| {
            map.entry(enemy.id.clone())
                .or_insert_with(|| enemy.base.clone());
            map
        });
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|(name, weight, enemies)| {
            let scaled = enemies
                .iter()
                .cloned()
                .map(|mut e| {
                    e.base = champion_at_level(&e.base, e.level);
                    e
                })
                .collect::<Vec<_>>();
            (name.clone(), *weight, scaled)
        })
        .collect::<Vec<_>>();

    let mut search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    apply_search_quality_profile(&mut search_cfg, search_quality_profile);
    if let Some(seed) = seed_override {
        search_cfg.seed = seed.max(1);
    }
    if search_cfg.seed == 0 {
        search_cfg.seed = runtime_random_seed();
    }
    let active_strategies = portfolio_strategy_list(&search_cfg);
    let full_loadout_domain = Arc::new(build_loadout_domain());
    let controlled_champion_loadout_selection = ensure_complete_loadout_selection(
        &controlled_champion_loadout_selection,
        full_loadout_domain.as_ref(),
    )?;
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &full_loadout_domain, &urf)?;
    let enemy_loadout = ResolvedLoadout::default();
    let max_items = search_cfg.max_items;
    let full_item_pool = default_item_pool(&items, &urf);
    let search_loadout_domain = if search_cfg.unmodeled_rune_hard_gate {
        Arc::new(filter_loadout_domain_to_modeled_runes(
            full_loadout_domain.as_ref(),
            sim.champion_level,
            true,
        )?)
    } else {
        Arc::clone(&full_loadout_domain)
    };
    if search_cfg.unmodeled_rune_hard_gate && search_loadout_domain.rune_paths.len() < 2 {
        return Err(anyhow!(
            "Unmodeled rune hard gate left fewer than two legal rune paths for controlled champion search."
        ));
    }
    let controlled_champion_search_base_loadout_selection = if search_cfg.unmodeled_rune_hard_gate {
        select_search_base_loadout_selection(
            &controlled_champion_loadout_selection,
            search_loadout_domain.as_ref(),
        )?
    } else {
        controlled_champion_loadout_selection.clone()
    };
    let item_pool = if search_cfg.unmodeled_item_effect_hard_gate {
        filter_item_pool_to_modeled_runtime_effects(&full_item_pool)
    } else {
        full_item_pool
    };
    if item_pool.is_empty() {
        return Err(anyhow!(
            "Unmodeled item-effect hard gate left zero legal items in the controlled champion search pool."
        ));
    }
    let max_legal_items = max_legal_build_size(&item_pool);
    if max_legal_items < max_items {
        return Err(anyhow!(
            "Controlled champion search cannot form a full build under active item constraints: max_items={} but at most {} legal item slots are available.",
            max_items,
            max_legal_items
        ));
    }
    status.emit(
        "configuration",
        None,
        None,
        Some("search profile and enemy presets ready"),
        true,
    );

    let mut enemy_presets_used: HashMap<String, EnemyUrfPreset> = HashMap::new();
    let mut enemy_build_scenarios = Vec::new();
    for (name, weight, enemies) in &enemy_scenarios {
        if deadline_reached(current_deadline()) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(&items, &preset.item_names)?;
            let preset_enemy_loadout =
                resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?;
            let enemy_bonus_stats = preset_enemy_loadout.bonus_stats;
            enemy_presets_used.insert(preset_key, preset.clone());
            let mut enemy_with_loadout = enemy.clone();
            enemy_with_loadout.loadout_item_names = preset.item_names.clone();
            enemy_with_loadout.loadout_rune_names = preset.runes.clone();
            enemy_with_loadout.loadout_shards = preset.shards.clone();
            builds.push((enemy_with_loadout, build_items, enemy_bonus_stats));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }
    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, b)| b.clone())
        .unwrap_or_default();
    let enemy_derived_combat_stats = enemy_builds
        .iter()
        .map(|(enemy, build, bonus_stats)| {
            derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf)
        })
        .collect::<Vec<_>>();
    let enemy_similarity_notes = build_enemy_similarity_notes(&enemy_derived_combat_stats);
    status.emit(
        "enemy_setup",
        None,
        None,
        Some("enemy preset builds loaded"),
        true,
    );

    let controlled_champion_base_loadout = resolve_loadout(
        &controlled_champion_loadout_selection,
        sim.champion_level,
        true,
    )?;
    let resolve_cache: Arc<Mutex<HashMap<String, ResolvedLoadout>>> =
        Arc::new(Mutex::new(HashMap::new()));
    if let Ok(mut map) = resolve_cache.lock() {
        map.insert(
            loadout_selection_key(&controlled_champion_loadout_selection),
            controlled_champion_base_loadout.clone(),
        );
    }
    let best_loadout_by_candidate: Arc<Mutex<ResolvedByCandidateMap>> =
        Arc::new(Mutex::new(HashMap::new()));
    let best_outcome_by_candidate: Arc<Mutex<OutcomeByCandidateMap>> =
        Arc::new(Mutex::new(HashMap::new()));

    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
        search_cfg.objective_enemy_kills_weight,
        search_cfg.objective_invulnerable_seconds_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            let damage_reference = enemy_builds_s
                .iter()
                .map(|(enemy, build, bonus_stats)| {
                    derive_enemy_combat_stats(enemy, build, bonus_stats, &sim, &urf).max_health
                })
                .sum::<f64>()
                .max(1.0);
            CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: damage_reference,
                healing_done: controlled_champion_base.base_health.max(1.0),
                enemy_kills: enemy_builds_s.len().max(1),
                invulnerable_seconds: sim.max_time_seconds.max(1.0),
            }
        })
        .collect::<Vec<_>>();
    let objective_eval_ctx = ObjectiveEvalContext {
        controlled_champion_base: &controlled_champion_base,
        controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
        enemy_build_scenarios: &enemy_build_scenarios,
        sim: &sim,
        urf: &urf,
        scenario_reference_outcomes: &scenario_reference_outcomes,
        weights: objective_component_weights,
        worst_case_weight: objective_worst_case_weight,
    };
    let evaluate_build_with_bonus =
        |build_items: &[Item],
         bonus_stats: &Stats,
         loadout_selection: Option<&LoadoutSelection>| {
            aggregate_objective_score_and_outcome_with_loadout_selection(
                &objective_eval_ctx,
                build_items,
                bonus_stats,
                loadout_selection,
            )
        };

    let resolve_loadout_for_selection = |selection: &LoadoutSelection| -> Option<ResolvedLoadout> {
        let key = loadout_selection_key(selection);
        if let Ok(map) = resolve_cache.lock()
            && let Some(existing) = map.get(&key).cloned()
        {
            return Some(existing);
        }
        let resolved = resolve_loadout(selection, sim.champion_level, true).ok()?;
        if let Ok(mut map) = resolve_cache.lock() {
            map.insert(key, resolved.clone());
        }
        Some(resolved)
    };
    let item_has_unmodeled_effect_by_index = item_pool
        .iter()
        .map(is_item_effect_unmodeled)
        .collect::<Vec<_>>();

    let full_eval_count = AtomicUsize::new(0);
    let unmodeled_rune_candidates_rejected = AtomicUsize::new(0);
    let unmodeled_rune_candidates_penalized = AtomicUsize::new(0);
    let unmodeled_item_effect_candidates_rejected = AtomicUsize::new(0);
    let unmodeled_item_effect_candidates_penalized = AtomicUsize::new(0);
    let full_cache = Arc::new(BlockingScoreCache::new());
    let unique_scored_candidate_keys = Arc::new(ShardedStringSet::new());
    let search_type_counters =
        initialize_search_type_counters(&active_strategies, &search_cfg.strategy);
    let full_score_for_search_type = |search_type: &str, candidate: &BuildKey| {
        increment_search_type_counter(search_type_counters.as_ref(), search_type, 1, 0);
        if deadline_reached(deadline_for_search_type(search_type)) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            return f64::NEG_INFINITY;
        }
        let key = canonical_build_candidate(candidate.clone());
        let is_full_candidate = key.item_indices.len() == max_items;
        let cache_key = build_key_cache_string(&key);
        let cache = Arc::clone(&full_cache);
        let search_type_owned = search_type.to_string();
        cache.get_or_compute(cache_key.clone(), || {
            if deadline_reached(deadline_for_search_type(&search_type_owned)) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            let Some(resolved_loadout) = resolve_loadout_for_selection(&key.loadout_selection)
            else {
                return f64::NEG_INFINITY;
            };
            let unmodeled_rune_count = resolved_loadout.unmodeled_rune_names.len();
            let unmodeled_item_effect_count = key
                .item_indices
                .iter()
                .filter(|item_idx| {
                    item_has_unmodeled_effect_by_index
                        .get(**item_idx)
                        .copied()
                        .unwrap_or(false)
                })
                .count();
            if unmodeled_rune_count > 0 && search_cfg.unmodeled_rune_hard_gate {
                unmodeled_rune_candidates_rejected.fetch_add(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            if unmodeled_item_effect_count > 0 && search_cfg.unmodeled_item_effect_hard_gate {
                unmodeled_item_effect_candidates_rejected.fetch_add(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            arm_time_budget_deadline_if_unset(
                &hard_deadline_state,
                time_budget,
                defer_hard_budget_until_coverage,
                &search_type_owned,
            );
            if is_full_candidate {
                full_eval_count.fetch_add(1, AtomicOrdering::Relaxed);
            }
            increment_search_type_counter(search_type_counters.as_ref(), &search_type_owned, 0, 1);
            let build_items = build_from_indices(&item_pool, &key.item_indices);
            let (score, outcome) = evaluate_build_with_bonus(
                &build_items,
                &resolved_loadout.bonus_stats,
                Some(&key.loadout_selection),
            );
            let mut score = score;
            if unmodeled_rune_count > 0 {
                unmodeled_rune_candidates_penalized.fetch_add(1, AtomicOrdering::Relaxed);
                score -= search_cfg.unmodeled_rune_penalty_per_rune.max(0.0)
                    * unmodeled_rune_count as f64;
            }
            if unmodeled_item_effect_count > 0 {
                unmodeled_item_effect_candidates_penalized.fetch_add(1, AtomicOrdering::Relaxed);
                score -= search_cfg.unmodeled_item_effect_penalty_per_item.max(0.0)
                    * unmodeled_item_effect_count as f64;
            }
            if is_full_candidate && score.is_finite() {
                unique_scored_candidate_keys.insert(cache_key.clone());
            }
            if is_full_candidate {
                if let Ok(mut map) = best_loadout_by_candidate.lock() {
                    map.insert(key.clone(), resolved_loadout);
                }
                if let Ok(mut map) = best_outcome_by_candidate.lock() {
                    map.insert(key.clone(), outcome);
                }
            }
            if is_full_candidate {
                record_score_progress(score);
            }
            score
        })
    };
    let evaluate_candidate_direct = |candidate: &BuildKey| {
        let key = canonical_build_candidate(candidate.clone());
        let resolved_loadout = resolve_loadout_for_selection(&key.loadout_selection)?;
        let unmodeled_rune_count = resolved_loadout.unmodeled_rune_names.len();
        let unmodeled_item_effect_count = key
            .item_indices
            .iter()
            .filter(|item_idx| {
                item_has_unmodeled_effect_by_index
                    .get(**item_idx)
                    .copied()
                    .unwrap_or(false)
            })
            .count();
        if unmodeled_rune_count > 0 && search_cfg.unmodeled_rune_hard_gate {
            return None;
        }
        if unmodeled_item_effect_count > 0 && search_cfg.unmodeled_item_effect_hard_gate {
            return None;
        }
        arm_time_budget_deadline_if_unset(
            &hard_deadline_state,
            time_budget,
            defer_hard_budget_until_coverage,
            "strict_fallback",
        );
        let build_items = build_from_indices(&item_pool, &key.item_indices);
        let (score, outcome) = evaluate_build_with_bonus(
            &build_items,
            &resolved_loadout.bonus_stats,
            Some(&key.loadout_selection),
        );
        let score = score
            - search_cfg.unmodeled_rune_penalty_per_rune.max(0.0) * unmodeled_rune_count as f64
            - search_cfg.unmodeled_item_effect_penalty_per_item.max(0.0)
                * unmodeled_item_effect_count as f64;
        if key.item_indices.len() == max_items && score.is_finite() {
            unique_scored_candidate_keys.insert(build_key_cache_string(&key));
        }
        Some((key, score, outcome, resolved_loadout))
    };

    let full_search_params = FullLoadoutSearchParams {
        item_pool: &item_pool,
        max_items,
        loadout_domain: search_loadout_domain.as_ref(),
        base_loadout: &controlled_champion_search_base_loadout_selection,
    };

    let mut coverage_stage_diagnostics = CoverageStageDiagnostics::default();
    let mut coverage_seed_candidates = Vec::<BuildKey>::new();
    if matches!(search_quality_profile, SearchQualityProfile::MaximumQuality) {
        coverage_stage_diagnostics.enabled = true;
        let coverage_start = Instant::now();
        let coverage_assets = coverage_locked_assets(&item_pool, search_loadout_domain.as_ref());
        coverage_stage_diagnostics.assets_total = coverage_assets.len();
        let mut coverage_stage_stopped_early = false;
        let coverage_trials_per_asset = (search_cfg.random_samples / 14).clamp(12, 48);
        let coverage_refinement_steps = (search_cfg.hill_climb_steps / 4).clamp(2, 8);

        status.emit(
            "coverage_stage",
            Some((0, coverage_assets.len())),
            None,
            Some("locking each item/rune/shard at least once"),
            true,
        );
        for (asset_index, asset) in coverage_assets.iter().enumerate() {
            if deadline_reached(coverage_stage_deadline()) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                coverage_stage_stopped_early = true;
                break;
            }

            let mut local_seed = search_cfg
                .seed
                .wrapping_add((asset_index as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15));
            let mut local_candidates = Vec::<BuildKey>::new();
            for _ in 0..coverage_trials_per_asset {
                if deadline_reached(coverage_stage_deadline()) {
                    timeout_flag.store(1, AtomicOrdering::Relaxed);
                    coverage_stage_stopped_early = true;
                    break;
                }
                if let Some(candidate) =
                    random_locked_candidate(&full_search_params, asset, &mut local_seed)
                {
                    local_candidates.push(candidate);
                }
            }

            let seed_snapshot = local_candidates.clone();
            for _ in 0..coverage_refinement_steps {
                if deadline_reached(coverage_stage_deadline()) {
                    timeout_flag.store(1, AtomicOrdering::Relaxed);
                    coverage_stage_stopped_early = true;
                    break;
                }
                if seed_snapshot.is_empty() {
                    break;
                }
                let parent = &seed_snapshot[rand_index(&mut local_seed, seed_snapshot.len())];
                if let Some(mutated) =
                    mutate_locked_candidate(&full_search_params, parent, asset, &mut local_seed)
                {
                    local_candidates.push(mutated);
                }
            }

            let mut unique_local = local_candidates
                .into_iter()
                .map(canonical_build_candidate)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            unique_local.sort_by_key(build_key_cache_string);
            let mut ranked = unique_local
                .par_iter()
                .map(|candidate| {
                    (
                        candidate.clone(),
                        full_score_for_search_type("coverage_stage", candidate),
                    )
                })
                .filter(|(_, score)| score.is_finite())
                .collect::<Vec<_>>();
            ranked.sort_by(|a, b| {
                b.1.partial_cmp(&a.1)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| build_key_cache_string(&a.0).cmp(&build_key_cache_string(&b.0)))
            });

            if !ranked.is_empty() {
                coverage_stage_diagnostics.assets_covered += 1;
                let diverse =
                    select_diverse_top_candidates(&ranked, 3, min_item_diff.max(1), 100.0);
                if diverse.is_empty() {
                    coverage_seed_candidates.push(ranked[0].0.clone());
                } else {
                    coverage_seed_candidates
                        .extend(diverse.into_iter().map(|(candidate, _)| candidate));
                }
            }

            let note = asset.display_label(&item_pool);
            status.emit(
                "coverage_stage",
                Some((asset_index + 1, coverage_assets.len())),
                None,
                Some(note.as_str()),
                false,
            );
        }

        coverage_stage_diagnostics.seed_candidates = coverage_seed_candidates.len();
        coverage_seed_candidates = coverage_seed_candidates
            .into_iter()
            .map(canonical_build_candidate)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        coverage_seed_candidates.sort_by_key(build_key_cache_string);
        coverage_stage_diagnostics.seed_candidates_unique = coverage_seed_candidates.len();
        coverage_stage_diagnostics.elapsed_seconds = coverage_start.elapsed().as_secs_f64();
        coverage_stage_diagnostics.coverage_incomplete =
            coverage_stage_diagnostics.assets_covered < coverage_stage_diagnostics.assets_total;
        if coverage_stage_diagnostics.coverage_incomplete {
            let reason = if coverage_stage_stopped_early {
                "coverage stage reached a timeout boundary before all assets were touched"
            } else {
                "coverage stage could not produce finite candidates for at least one locked asset"
            };
            coverage_stage_diagnostics.coverage_warning = format!(
                "Coverage incomplete: touched {}/{} assets; {}. Continuing search in degraded coverage mode.",
                coverage_stage_diagnostics.assets_covered,
                coverage_stage_diagnostics.assets_total,
                reason
            );
        }
    }

    if time_budget.is_some() && hard_deadline_value().is_none() {
        // Bootstrap one timed-phase simulation so staged search loops get a live deadline value.
        let mut bootstrap_seed = search_cfg.seed ^ 0xC0DE_DA7A_u64;
        let bootstrap_candidate = canonical_build_candidate(BuildKey {
            item_indices: random_valid_build(&item_pool, max_items, &mut bootstrap_seed),
            loadout_selection: controlled_champion_search_base_loadout_selection.clone(),
        });
        let bootstrap_search_type = format!("seed_search:{}", search_cfg.strategy);
        let _ = full_score_for_search_type(bootstrap_search_type.as_str(), &bootstrap_candidate);
        if hard_deadline_value().is_none() {
            arm_time_budget_deadline_if_unset(
                &hard_deadline_state,
                time_budget,
                defer_hard_budget_until_coverage,
                "seed_search:bootstrap",
            );
        }
    }

    let ensemble_seeds = search_cfg.ensemble_seeds.max(1);
    status.emit(
        "seed_search",
        Some((0, ensemble_seeds)),
        None,
        Some("running ensemble seeds"),
        true,
    );
    let mut seed_ranked = (0..ensemble_seeds)
        .into_par_iter()
        .map(|seed_idx| {
            if deadline_reached(current_deadline()) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                return (seed_idx, Vec::new());
            }
            let mut cfg = search_cfg.clone();
            cfg.seed = search_cfg.seed.wrapping_add(
                search_cfg
                    .ensemble_seed_stride
                    .wrapping_mul(seed_idx as u64),
            );
            cfg.ranked_limit = cfg.ranked_limit.max(64);
            let search_type = format!("seed_search:{}", cfg.strategy);
            let score_fn =
                |candidate: &BuildKey| full_score_for_search_type(search_type.as_str(), candidate);
            let ranked = build_search_ranked_full_loadout(
                &full_search_params,
                &cfg,
                &score_fn,
                current_deadline(),
            );
            (seed_idx, ranked)
        })
        .collect::<Vec<_>>();
    seed_ranked.sort_by_key(|(seed_idx, _)| *seed_idx);
    let seed_ranked = seed_ranked
        .into_iter()
        .map(|(_, ranked)| ranked)
        .collect::<Vec<_>>();
    status.emit(
        "seed_search",
        Some((seed_ranked.len().min(ensemble_seeds), ensemble_seeds)),
        None,
        None,
        false,
    );
    let strategy_elite_score_fn =
        |candidate: &BuildKey| full_score_for_search_type("strategy_elites", candidate);
    let mut strategy_elites = strategy_seed_elites_full_loadout(
        &full_search_params,
        &search_cfg,
        &active_strategies,
        &strategy_elite_score_fn,
        current_deadline(),
    );
    if !coverage_seed_candidates.is_empty() {
        let mut target_strategies = active_strategies.clone();
        if target_strategies.is_empty() {
            target_strategies.push(search_cfg.strategy.clone());
        }
        for (idx, candidate) in coverage_seed_candidates.iter().enumerate() {
            let strategy = target_strategies[idx % target_strategies.len()].clone();
            strategy_elites
                .entry(strategy)
                .or_default()
                .push(candidate.clone());
        }
        for candidates in strategy_elites.values_mut() {
            let mut unique = candidates
                .iter()
                .cloned()
                .map(canonical_build_candidate)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            unique.sort_by_key(build_key_cache_string);
            *candidates = unique;
        }
    }
    let adaptive_score_fn =
        |candidate: &BuildKey| full_score_for_search_type("adaptive_search", candidate);
    let adaptive_candidates = adaptive_strategy_candidates_full_loadout(
        &full_search_params,
        &search_cfg,
        &strategy_elites,
        &adaptive_score_fn,
        current_deadline(),
    );
    let bleed_candidates =
        generate_bleed_candidates_full_loadout(&full_search_params, &search_cfg, &strategy_elites);
    status.emit(
        "candidate_merge",
        None,
        None,
        Some("merging strict candidates"),
        true,
    );
    let bleed_candidate_count = bleed_candidates.len();
    let adaptive_candidate_count = adaptive_candidates.len();

    let mut candidate_keys = Vec::new();
    let mut best_seeded_candidate: Option<(BuildKey, f64)> = None;
    let mut seed_top_sets = Vec::new();
    for (seed_idx, ranked) in seed_ranked.iter().enumerate() {
        let mut seed_top = HashSet::new();
        for (ranked_idx, (candidate, score)) in ranked.iter().enumerate() {
            if score.is_finite() {
                let candidate_key = canonical_build_candidate(candidate.clone());
                let replace = best_seeded_candidate
                    .as_ref()
                    .map(|(best_key, best_score)| {
                        *score > *best_score
                            || ((*score - *best_score).abs() <= f64::EPSILON
                                && build_key_cache_string(&candidate_key)
                                    < build_key_cache_string(best_key))
                    })
                    .unwrap_or(true);
                if replace {
                    best_seeded_candidate = Some((candidate_key, *score));
                }
            }
            if candidate.item_indices.len() == max_items {
                candidate_keys.push(candidate.clone());
                if ranked_idx < search_cfg.ensemble_seed_top_k.max(1) {
                    seed_top.insert(candidate.clone());
                }
                continue;
            }
            if !score.is_finite() {
                continue;
            }
            let mut completion_seed =
                partial_candidate_completion_seed(search_cfg.seed, seed_idx, ranked_idx, candidate);
            let completed = complete_partial_candidate_to_full(
                candidate,
                &item_pool,
                max_items,
                &mut completion_seed,
            );
            if completed.item_indices.len() != max_items {
                continue;
            }
            candidate_keys.push(completed.clone());
            if ranked_idx < search_cfg.ensemble_seed_top_k.max(1) {
                seed_top.insert(completed);
            }
        }
        seed_top_sets.push(seed_top);
    }
    for candidate in &coverage_seed_candidates {
        candidate_keys.push(candidate.clone());
    }
    for k in bleed_candidates {
        candidate_keys.push(k);
    }
    for k in adaptive_candidates {
        candidate_keys.push(k);
    }
    let candidate_keys_generated = candidate_keys.len();
    let mut unique_candidate_keys = candidate_keys
        .into_iter()
        .map(canonical_build_candidate)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_candidate_keys.sort_by_key(build_key_cache_string);
    if unique_candidate_keys.is_empty() {
        let mut fallback_seed = search_cfg.seed ^ 0x9e37_79b9_7f4a_7c15;
        unique_candidate_keys.push(canonical_build_candidate(BuildKey {
            item_indices: random_valid_build(&item_pool, max_items, &mut fallback_seed),
            loadout_selection: random_loadout_selection(
                &controlled_champion_search_base_loadout_selection,
                search_loadout_domain.as_ref(),
                &mut fallback_seed,
            ),
        }));
    }
    let candidate_duplicates_pruned =
        candidate_keys_generated.saturating_sub(unique_candidate_keys.len());

    let mut strict_scores = HashMap::<BuildKey, f64>::new();
    for ranked in &seed_ranked {
        for (k, s) in ranked {
            if k.item_indices.len() != max_items {
                continue;
            }
            if !s.is_finite() {
                continue;
            }
            let entry = strict_scores.entry(k.clone()).or_insert(*s);
            if *s > *entry {
                *entry = *s;
            }
        }
    }

    let total_candidates = unique_candidate_keys.len();
    let strict_seed_scored_candidates = strict_scores.len().min(total_candidates);
    let mut processed_keys = strict_scores.keys().cloned().collect::<HashSet<_>>();
    let mut processed_candidates = processed_keys.len().min(total_candidates);
    let mut timed_out = timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    status.emit(
        "strict_full_ranking",
        Some((processed_candidates, total_candidates)),
        strict_scores
            .values()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
        Some("evaluating all generated candidates"),
        true,
    );
    let remaining_keys = unique_candidate_keys
        .iter()
        .filter(|key| !processed_keys.contains(*key))
        .cloned()
        .collect::<Vec<_>>();
    let strict_remaining_candidates = remaining_keys.len();
    let (remaining_keys, strict_random_promotions_done) =
        if search_cfg.strict_ranking_enable_heuristic_ordering {
            heuristic_sort_remaining_candidates_for_strict_ranking(
                remaining_keys,
                &strict_scores,
                item_pool.len(),
                search_cfg.strict_ranking_rune_signal_weight,
                search_cfg.strict_ranking_shard_signal_weight,
                search_cfg.seed,
                search_cfg.strict_ranking_exploration_promotions,
            )
        } else {
            (remaining_keys, 0)
        };
    let mut strict_non_finite_candidates = 0usize;
    let batch_size = 128usize;
    for batch in remaining_keys.chunks(batch_size) {
        if deadline_reached(current_deadline()) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            timed_out = true;
            break;
        }
        let scored_batch = batch
            .par_iter()
            .map(|key| {
                (
                    key.clone(),
                    full_score_for_search_type("strict_full_ranking", key),
                )
            })
            .collect::<Vec<_>>();
        for (key, score) in scored_batch {
            if score.is_finite() {
                strict_scores.insert(key.clone(), score);
            } else {
                strict_non_finite_candidates += 1;
            }
            processed_keys.insert(key);
            processed_candidates += 1;
            status.emit(
                "strict_full_ranking",
                Some((processed_candidates, total_candidates)),
                strict_scores
                    .values()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
                None,
                false,
            );
        }
    }

    if strict_scores.is_empty()
        && let Some(fallback_key) = best_seeded_candidate
            .as_ref()
            .map(|(candidate, _)| {
                if candidate.item_indices.len() == max_items {
                    return candidate.clone();
                }
                let mut completion_seed =
                    partial_candidate_completion_seed(search_cfg.seed, 0, 0, candidate);
                complete_partial_candidate_to_full(
                    candidate,
                    &item_pool,
                    max_items,
                    &mut completion_seed,
                )
            })
            .or_else(|| unique_candidate_keys.first().cloned())
        && let Some((key, fallback_score, fallback_outcome, fallback_loadout)) =
            evaluate_candidate_direct(&fallback_key)
    {
        strict_scores.insert(key.clone(), fallback_score);
        if let Ok(mut map) = best_outcome_by_candidate.lock() {
            map.insert(key.clone(), fallback_outcome);
        }
        if let Ok(mut map) = best_loadout_by_candidate.lock() {
            map.insert(key, fallback_loadout);
        }
    }

    let mut controlled_champion_ranked = strict_scores.into_iter().collect::<Vec<_>>();
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    let strict_candidates_skipped_timeout =
        total_candidates.saturating_sub(processed_candidates.min(total_candidates));
    let strict_completion_percent = if total_candidates > 0 {
        100.0 * (processed_candidates.min(total_candidates) as f64) / (total_candidates as f64)
    } else {
        100.0
    };
    let outcome_map_for_tiebreak = best_outcome_by_candidate
        .lock()
        .map(|m| m.clone())
        .unwrap_or_default();
    controlled_champion_ranked.sort_by(|a, b| {
        let by_score = b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal);
        if by_score != Ordering::Equal {
            return by_score;
        }
        let out_a = outcome_map_for_tiebreak.get(&a.0);
        let out_b = outcome_map_for_tiebreak.get(&b.0);
        let cap_a = out_a
            .map(|o| o.time_alive_seconds >= sim.max_time_seconds - 1e-6)
            .unwrap_or(false);
        let cap_b = out_b
            .map(|o| o.time_alive_seconds >= sim.max_time_seconds - 1e-6)
            .unwrap_or(false);
        if cap_a != cap_b {
            return cap_b.cmp(&cap_a);
        }
        let combo_a = out_a
            .map(|o| {
                objective_component_weights.damage * o.damage_dealt
                    + objective_component_weights.healing * o.healing_done
                    + objective_component_weights.enemy_kills * o.enemy_kills as f64
                    + objective_component_weights.invulnerable_seconds * o.invulnerable_seconds
            })
            .unwrap_or(0.0);
        let combo_b = out_b
            .map(|o| {
                objective_component_weights.damage * o.damage_dealt
                    + objective_component_weights.healing * o.healing_done
                    + objective_component_weights.enemy_kills * o.enemy_kills as f64
                    + objective_component_weights.invulnerable_seconds * o.invulnerable_seconds
            })
            .unwrap_or(0.0);
        let by_combo = combo_b.partial_cmp(&combo_a).unwrap_or(Ordering::Equal);
        if by_combo != Ordering::Equal {
            return by_combo;
        }
        build_key_cache_string(&a.0).cmp(&build_key_cache_string(&b.0))
    });

    let mut seed_best_scores = Vec::new();
    for ranked in &seed_ranked {
        if deadline_reached(current_deadline()) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let best = ranked
            .iter()
            .take(search_cfg.ensemble_seed_top_k.max(1))
            .map(|(_, s)| *s)
            .fold(f64::NEG_INFINITY, |acc, v| acc.max(v));
        if best.is_finite() {
            seed_best_scores.push(best);
        }
    }

    let mut seed_hits_by_key: HashMap<BuildKey, usize> = HashMap::new();
    for top in &seed_top_sets {
        for key in top {
            *seed_hits_by_key.entry(key.clone()).or_insert(0) += 1;
        }
    }

    if controlled_champion_ranked.is_empty() {
        return Err(anyhow!(
            "No valid full-build candidate remained after strict ranking. candidate_keys_generated={} unique_candidates={} strict_non_finite={} unmodeled_rune_rejected={} unmodeled_item_effect_rejected={} coverage_assets={}/{}. This run cannot produce a valid best build; adjust quality gates or increase modeled coverage.",
            candidate_keys_generated,
            total_candidates,
            strict_non_finite_candidates,
            unmodeled_rune_candidates_rejected.load(AtomicOrdering::Relaxed),
            unmodeled_item_effect_candidates_rejected.load(AtomicOrdering::Relaxed),
            coverage_stage_diagnostics.assets_covered,
            coverage_stage_diagnostics.assets_total
        ));
    }
    let controlled_champion_best_candidate = controlled_champion_ranked[0].0.clone();
    let controlled_champion_best_build =
        build_from_indices(&item_pool, &controlled_champion_best_candidate.item_indices);
    let controlled_champion_runtime_loadout_selection =
        controlled_champion_best_candidate.loadout_selection.clone();
    let controlled_champion_loadout = best_loadout_by_candidate
        .lock()
        .ok()
        .and_then(|m| m.get(&controlled_champion_best_candidate).cloned())
        .or_else(|| resolve_loadout_for_selection(&controlled_champion_runtime_loadout_selection))
        .unwrap_or_else(|| controlled_champion_base_loadout.clone());

    let controlled_champion_best_score = controlled_champion_ranked
        .first()
        .map(|(_, s)| *s)
        .unwrap_or(0.0);
    let controlled_champion_best_outcome = best_outcome_by_candidate
        .lock()
        .ok()
        .and_then(|m| m.get(&controlled_champion_best_candidate).copied())
        .unwrap_or_else(|| {
            aggregate_objective_score_and_outcome_with_loadout_selection(
                &objective_eval_ctx,
                &controlled_champion_best_build,
                &controlled_champion_loadout.bonus_stats,
                Some(&controlled_champion_runtime_loadout_selection),
            )
            .1
        });
    let (_, _, best_score_breakdown) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            &objective_eval_ctx,
            &controlled_champion_best_build,
            &controlled_champion_loadout.bonus_stats,
            Some(&controlled_champion_runtime_loadout_selection),
        );
    let best_cap_survivor =
        controlled_champion_best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    let progress_snapshot =
        progress_state
            .lock()
            .ok()
            .map(|state| *state)
            .unwrap_or(SignificantProgressState {
                best_overall_score: f64::NEG_INFINITY,
                best_significant_score: f64::NEG_INFINITY,
                significant_events: 0,
                last_significant_at: run_start,
            });
    let seconds_since_last_significant_improvement = Instant::now()
        .saturating_duration_since(progress_snapshot.last_significant_at)
        .as_secs_f64();
    let mut search_type_breakdown = snapshot_search_type_counters(search_type_counters.as_ref());
    search_type_breakdown.sort_by(|a, b| {
        b.new_simulations
            .cmp(&a.new_simulations)
            .then_with(|| b.score_requests.cmp(&a.score_requests))
            .then_with(|| a.name.cmp(&b.name))
    });
    let effective_threads = rayon::current_num_threads();
    let seed_orchestration_parallel = ensemble_seeds > 1;
    let portfolio_strategy_parallel =
        search_cfg.strategy == "portfolio" && active_strategies.len() > 1;
    let strategy_elites_parallel = active_strategies.len() > 1 || ensemble_seeds > 1;
    let estimated_total_candidate_space = {
        let item_space = estimated_legal_item_build_count(&item_pool, max_items);
        let loadout_space = estimated_legal_loadout_count(search_loadout_domain.as_ref());
        let total = item_space * loadout_space;
        (total.is_finite() && total > 0.0).then_some(total)
    };
    let unique_scored_candidates = unique_scored_candidate_keys.len();
    let loadout_candidates_count = unique_loadout_selection_count(&unique_candidate_keys);
    let loadout_finalists_count =
        unique_loadout_selection_count_from_ranked(&controlled_champion_ranked);
    let estimated_run_space_coverage_percent = estimated_total_candidate_space
        .map(|total| ((unique_scored_candidates as f64) / total * 100.0).clamp(0.0, 100.0));
    let (estimated_close_to_optimal_probability, estimated_close_to_optimal_probability_note) =
        estimate_close_to_optimal_probability(
            unique_scored_candidates,
            estimated_total_candidate_space,
        );

    println!("Enemy builds (URF preset defaults):");
    for (enemy, build, _) in &enemy_builds {
        println!(
            "- {}: {}",
            enemy.name,
            build
                .iter()
                .map(|i| i.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );
        if let Some(preset) = enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            println!(
                "  source: {} (last checked {})",
                preset.source_url, preset.last_checked
            );
        }
    }
    println!("\nEnemy derived combat profiles:");
    for profile in &enemy_derived_combat_stats {
        println!(
            "- {}: HP {:.1}, Armor {:.1}, MR {:.1}, AD {:.1}, AS {:.3} (interval {:.3}s), range {:.0}, move speed {:.1}, hit physical {:.1}, hit ability {:.1}, burst phys/magic/true {:.1}/{:.1}/{:.1}",
            profile.champion,
            profile.max_health,
            profile.armor,
            profile.magic_resist,
            profile.attack_damage,
            profile.attack_speed,
            profile.attack_interval_seconds,
            profile.attack_range,
            profile.move_speed,
            profile.physical_hit_damage,
            profile.ability_hit_damage,
            profile.burst_physical_damage,
            profile.burst_magic_damage,
            profile.burst_true_damage
        );
    }
    for note in &enemy_similarity_notes {
        println!("- Warning: {}", note);
    }

    println!(
        "\n{} best build (optimized for objective):",
        controlled_champion_name
    );
    println!(
        "- Search strategy: {}",
        search_strategy_summary(&search_cfg)
    );
    println!(
        "- Loadout candidates/finalists: {}/{}",
        loadout_candidates_count, loadout_finalists_count
    );
    println!("- Effective search seed: {}", search_cfg.seed);
    if coverage_stage_diagnostics.enabled {
        println!(
            "- Coverage stage (pre-budget): {:.2}s | assets covered {}/{} | seeded candidates {}/{}",
            coverage_stage_diagnostics.elapsed_seconds,
            coverage_stage_diagnostics.assets_covered,
            coverage_stage_diagnostics.assets_total,
            coverage_stage_diagnostics.seed_candidates_unique,
            coverage_stage_diagnostics.seed_candidates
        );
        if coverage_stage_diagnostics.coverage_incomplete
            && !coverage_stage_diagnostics.coverage_warning.is_empty()
        {
            println!(
                "- Coverage warning: {}",
                coverage_stage_diagnostics.coverage_warning
            );
        }
    }
    println!(
        "- Candidate evaluations (full): {}",
        full_eval_count.load(AtomicOrdering::Relaxed)
    );
    println!(
        "- In-memory full-evaluation cache (hits/misses/waits): {}/{}/{}",
        full_cache.hits(),
        full_cache.misses(),
        full_cache.waits()
    );
    println!("- Ensemble seeds: {}", ensemble_seeds);
    println!(
        "- Parallelism: threads {} | seed orchestration parallel {} | portfolio strategy parallel {} | strategy-elites parallel {}",
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel
    );
    println!(
        "- Enemy scenarios in objective: {}",
        enemy_build_scenarios.len()
    );
    println!(
        "- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): {:.2}/{:.2}/{:.2}/{:.2}/{:.2}",
        objective_component_weights.survival,
        objective_component_weights.damage,
        objective_component_weights.healing,
        objective_component_weights.enemy_kills,
        objective_component_weights.invulnerable_seconds
    );
    if let Some(budget) = time_budget {
        println!(
            "- Time budget: {:.1}s | elapsed: {:.1}s | timed_out: {} | progress: {}/{}",
            budget.as_secs_f64(),
            run_start.elapsed().as_secs_f64(),
            timed_out,
            processed_candidates,
            total_candidates
        );
    }
    if let Some(window) = popcorn_window {
        println!(
            "- Popcorn mode: window {:.1}s | significant threshold {:.2}% of last best score | significant events {} | seconds since last significant improvement {:.1}",
            window.as_secs_f64(),
            popcorn_min_relative_improvement_percent,
            progress_snapshot.significant_events,
            seconds_since_last_significant_improvement
        );
    }
    println!(
        "- Unique strict candidates: {}",
        unique_candidate_keys.len()
    );
    println!(
        "- Strict candidate ordering: heuristic {} (rune/shard weights {:.2}/{:.2}), exploration promotions {}",
        search_cfg.strict_ranking_enable_heuristic_ordering,
        search_cfg.strict_ranking_rune_signal_weight,
        search_cfg.strict_ranking_shard_signal_weight,
        strict_random_promotions_done
    );
    println!(
        "- Unmodeled rune gate: hard gate {} | penalty per rune {:.4} | rejected {} | penalized {}",
        search_cfg.unmodeled_rune_hard_gate,
        search_cfg.unmodeled_rune_penalty_per_rune,
        unmodeled_rune_candidates_rejected.load(AtomicOrdering::Relaxed),
        unmodeled_rune_candidates_penalized.load(AtomicOrdering::Relaxed)
    );
    println!(
        "- Unmodeled item-effect gate: hard gate {} | penalty per item {:.4} | rejected {} | penalized {}",
        search_cfg.unmodeled_item_effect_hard_gate,
        search_cfg.unmodeled_item_effect_penalty_per_item,
        unmodeled_item_effect_candidates_rejected.load(AtomicOrdering::Relaxed),
        unmodeled_item_effect_candidates_penalized.load(AtomicOrdering::Relaxed)
    );
    println!(
        "- Candidate keys generated / duplicates pruned: {}/{}",
        candidate_keys_generated, candidate_duplicates_pruned
    );
    println!(
        "- Strict completion: {:.1}% (processed {}/{}, timeout-skipped {}, non-finite {})",
        strict_completion_percent,
        processed_candidates.min(total_candidates),
        total_candidates,
        strict_candidates_skipped_timeout,
        strict_non_finite_candidates
    );
    println!(
        "- Unique scored candidates (all search stages): {}",
        unique_scored_candidates
    );
    if let Some(total) = estimated_total_candidate_space {
        println!("- Estimated total legal candidate space: {:.0}", total);
    }
    if let Some(run_coverage) = estimated_run_space_coverage_percent {
        println!(
            "- Estimated legal-space coverage (this run): {}",
            format_percent_display(run_coverage)
        );
    }
    if let Some(probability) = estimated_close_to_optimal_probability {
        println!(
            "- Estimated closeness probability (top 0.000001% heuristic): {:.2}% | {}",
            probability * 100.0,
            estimated_close_to_optimal_probability_note
        );
    }
    println!("- Bleed candidates injected: {}", bleed_candidate_count);
    println!(
        "- Adaptive candidates injected: {}",
        adaptive_candidate_count
    );
    if !search_type_breakdown.is_empty() {
        println!("- Search-type simulation breakdown:");
        for entry in &search_type_breakdown {
            println!(
                "  - {} => score requests {}, new simulations {}",
                entry.name, entry.score_requests, entry.new_simulations
            );
        }
    }
    println!(
        "- Items: {}",
        controlled_champion_best_build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Objective score: {:.4}", controlled_champion_best_score);
    println!(
        "- Time alive / damage dealt / healing done / enemy kills / invulnerable seconds: {:.2}s / {:.1} / {:.1} / {} / {:.2}",
        controlled_champion_best_outcome.time_alive_seconds,
        controlled_champion_best_outcome.damage_dealt,
        controlled_champion_best_outcome.healing_done,
        controlled_champion_best_outcome.enemy_kills,
        controlled_champion_best_outcome.invulnerable_seconds
    );
    println!("- Cap survivor: {}", best_cap_survivor);
    if !controlled_champion_loadout.selection_labels.is_empty() {
        println!("\n{} rune page:", controlled_champion_name);
        for s in &controlled_champion_loadout.selection_labels {
            println!("- {}", s);
        }
    }

    let diverse_top_raw = select_diverse_top_candidates(
        &controlled_champion_ranked,
        top_x,
        min_item_diff,
        max_relative_gap_percent,
    );
    let diverse_top_keys = diverse_top_raw
        .iter()
        .map(|(candidate, _)| candidate.clone())
        .collect::<Vec<_>>();
    let diverse_top_builds = diverse_top_raw
        .iter()
        .map(|(candidate, score)| {
            (
                build_from_indices(&item_pool, &candidate.item_indices),
                *score,
            )
        })
        .collect::<Vec<_>>();
    let resolved_by_candidate_snapshot = best_loadout_by_candidate
        .lock()
        .map(|map| map.clone())
        .unwrap_or_default();
    let mut metrics_by_key = HashMap::new();
    for (candidate, score) in &controlled_champion_ranked {
        let candidate_bonus_stats = resolved_by_candidate_snapshot
            .get(candidate)
            .map(|resolved| resolved.bonus_stats.clone())
            .or_else(|| {
                resolve_loadout_for_selection(&candidate.loadout_selection)
                    .map(|resolved| resolved.bonus_stats)
            })
            .unwrap_or_else(|| controlled_champion_base_loadout.bonus_stats.clone());
        metrics_by_key.insert(
            candidate.clone(),
            compute_build_metrics_for_candidate(
                candidate,
                &item_pool,
                &controlled_champion_base,
                &candidate_bonus_stats,
                &controlled_champion_stack_overrides,
                &sim,
                *score,
            ),
        );
    }
    let pareto_front = candidate_pareto_front_keys(&metrics_by_key);
    let build_confidence = controlled_champion_ranked
        .iter()
        .map(|(key, _)| {
            let hits = seed_hits_by_key.get(key).copied().unwrap_or(0);
            let hit_rate = hits as f64 / ensemble_seeds as f64;
            let robustness = if hit_rate >= search_cfg.robust_min_seed_hit_rate {
                "robust".to_string()
            } else {
                "fragile".to_string()
            };
            BuildConfidence {
                key: key.clone(),
                seed_hits: hits,
                seed_hit_rate: hit_rate,
                robustness,
            }
        })
        .collect::<Vec<_>>();
    let mut diagnostics = SearchDiagnostics {
        strategy_summary: search_strategy_summary(&search_cfg),
        search_quality_profile: match search_quality_profile {
            SearchQualityProfile::Fast => "fast".to_string(),
            SearchQualityProfile::Balanced => "balanced".to_string(),
            SearchQualityProfile::MaximumQuality => "maximum_quality".to_string(),
        },
        effective_seed: search_cfg.seed,
        ensemble_seeds,
        effective_threads,
        seed_orchestration_parallel,
        portfolio_strategy_parallel,
        strategy_elites_parallel,
        objective_survival_weight: objective_component_weights.survival,
        objective_damage_weight: objective_component_weights.damage,
        objective_healing_weight: objective_component_weights.healing,
        objective_enemy_kills_weight: objective_component_weights.enemy_kills,
        objective_invulnerable_seconds_weight: objective_component_weights.invulnerable_seconds,
        full_evaluations: full_eval_count.load(AtomicOrdering::Relaxed),
        full_cache_hits: full_cache.hits(),
        full_cache_misses: full_cache.misses(),
        full_cache_waits: full_cache.waits(),
        candidate_keys_generated,
        candidate_duplicates_pruned,
        unique_candidate_builds: unique_candidate_keys.len(),
        bleed_candidates_injected: bleed_candidate_count,
        adaptive_candidates_injected: adaptive_candidate_count,
        scenario_count: enemy_build_scenarios.len(),
        loadout_candidates: loadout_candidates_count,
        loadout_finalists: loadout_finalists_count,
        strict_seed_scored_candidates,
        strict_remaining_candidates,
        strict_non_finite_candidates,
        strict_candidates_skipped_timeout,
        strict_completion_percent,
        strict_heuristic_ordering_enabled: search_cfg.strict_ranking_enable_heuristic_ordering,
        strict_ranking_rune_signal_weight: search_cfg.strict_ranking_rune_signal_weight,
        strict_ranking_shard_signal_weight: search_cfg.strict_ranking_shard_signal_weight,
        strict_random_promotions_done,
        unmodeled_rune_hard_gate: search_cfg.unmodeled_rune_hard_gate,
        unmodeled_rune_penalty_per_rune: search_cfg.unmodeled_rune_penalty_per_rune,
        unmodeled_rune_candidates_rejected: unmodeled_rune_candidates_rejected
            .load(AtomicOrdering::Relaxed),
        unmodeled_rune_candidates_penalized: unmodeled_rune_candidates_penalized
            .load(AtomicOrdering::Relaxed),
        unmodeled_item_effect_hard_gate: search_cfg.unmodeled_item_effect_hard_gate,
        unmodeled_item_effect_penalty_per_item: search_cfg.unmodeled_item_effect_penalty_per_item,
        unmodeled_item_effect_candidates_rejected: unmodeled_item_effect_candidates_rejected
            .load(AtomicOrdering::Relaxed),
        unmodeled_item_effect_candidates_penalized: unmodeled_item_effect_candidates_penalized
            .load(AtomicOrdering::Relaxed),
        unique_scored_candidates,
        time_budget_seconds: time_budget.map(|d| d.as_secs_f64()),
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
        significant_improvement_events: progress_snapshot.significant_events,
        best_significant_score: progress_snapshot
            .best_significant_score
            .is_finite()
            .then_some(progress_snapshot.best_significant_score),
        seconds_since_last_significant_improvement: Some(
            seconds_since_last_significant_improvement,
        ),
        search_type_breakdown,
        estimated_total_candidate_space,
        estimated_run_space_coverage_percent,
        estimated_close_to_optimal_probability,
        estimated_close_to_optimal_probability_note,
        coverage_stage_enabled: coverage_stage_diagnostics.enabled,
        coverage_stage_elapsed_seconds: coverage_stage_diagnostics.elapsed_seconds,
        coverage_stage_assets_total: coverage_stage_diagnostics.assets_total,
        coverage_stage_assets_covered: coverage_stage_diagnostics.assets_covered,
        coverage_stage_seed_candidates: coverage_stage_diagnostics.seed_candidates,
        coverage_stage_seed_candidates_unique: coverage_stage_diagnostics.seed_candidates_unique,
        coverage_stage_incomplete: coverage_stage_diagnostics.coverage_incomplete,
        coverage_stage_warning: coverage_stage_diagnostics.coverage_warning.clone(),
        elapsed_seconds: run_start.elapsed().as_secs_f64(),
        total_run_seconds: 0.0,
        timed_out,
        processed_candidates,
        total_candidates,
        seed_best_scores,
    };
    let confidence_by_key = build_confidence
        .iter()
        .map(|c| (c.key.clone(), c.clone()))
        .collect::<HashMap<_, _>>();
    let mut order_input = diverse_top_raw
        .iter()
        .filter_map(|(candidate, _)| {
            let robust = confidence_by_key
                .get(candidate)
                .map(|c| c.robustness == "robust")
                .unwrap_or(false);
            let pareto = pareto_front.contains(candidate);
            if robust || pareto {
                Some((
                    candidate.clone(),
                    build_from_indices(&item_pool, &candidate.item_indices),
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if order_input.is_empty() {
        order_input = diverse_top_raw
            .iter()
            .take(2)
            .map(|(candidate, _)| {
                (
                    candidate.clone(),
                    build_from_indices(&item_pool, &candidate.item_indices),
                )
            })
            .collect::<Vec<_>>();
    }
    let build_order_results = order_input
        .iter()
        .map(|(candidate, build)| {
            let candidate_bonus_stats = resolved_by_candidate_snapshot
                .get(candidate)
                .map(|resolved| resolved.bonus_stats.clone())
                .or_else(|| {
                    resolve_loadout_for_selection(&candidate.loadout_selection)
                        .map(|resolved| resolved.bonus_stats)
                })
                .unwrap_or_else(|| controlled_champion_base_loadout.bonus_stats.clone());
            let build_order_ctx = BuildOrderEvalContext {
                controlled_champion_base_raw: &controlled_champion_base_raw,
                controlled_champion_bonus_stats: &candidate_bonus_stats,
                controlled_champion_stack_overrides: &controlled_champion_stack_overrides,
                enemy_build_scenarios: &enemy_build_scenarios,
                raw_enemy_bases: &raw_enemy_bases,
                sim: &sim,
                urf: &urf,
                objective_weights: objective_component_weights,
                multi_scenario_worst_weight: objective_worst_case_weight,
            };
            optimize_build_order(build, &build_order_ctx)
        })
        .collect::<Vec<_>>();
    let best_order_acquired_map = build_order_results
        .first()
        .map(|br| acquisition_level_map(&br.ordered_items, &br.acquired_levels));

    let best_effective_item_stats = compute_effective_item_stats_for_build(
        &controlled_champion_base,
        &controlled_champion_best_build,
        &controlled_champion_loadout.bonus_stats,
        &sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
        Some(&controlled_champion_stack_overrides),
    );
    let controlled_champion_end_stats =
        compute_champion_final_stats(&controlled_champion_base, &best_effective_item_stats);
    let stack_notes = build_stack_notes(
        &controlled_champion_best_build,
        &controlled_champion_base,
        &sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
        Some(&controlled_champion_stack_overrides),
    );

    println!("\nTop diverse builds:");
    if diverse_top_builds.is_empty() {
        println!(
            "- None found (try increasing --max-relative-gap-percent or lowering --min-item-diff)."
        );
    } else {
        for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
            println!(
                "- #{:02} score {:.4}: {}",
                idx + 1,
                score,
                item_names(build)
            );
        }
    }
    if !build_order_results.is_empty() {
        println!("\nBuild order optimization (levels spread from 5 to 20):");
        for (idx, br) in build_order_results.iter().enumerate() {
            println!(
                "- Build #{:02} best order (cumulative {:.2}): {}",
                idx + 1,
                br.cumulative_score,
                item_names(&br.ordered_items)
            );
            for (stage_idx, lvl) in br.levels.iter().enumerate() {
                let surv = br.stage_survival.get(stage_idx).copied().unwrap_or(0.0);
                let dmg = br.stage_damage.get(stage_idx).copied().unwrap_or(0.0);
                let heal = br.stage_healing.get(stage_idx).copied().unwrap_or(0.0);
                let stage_objective = br
                    .stage_objective_scores
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                println!(
                    "  - Stage {} @ level {} -> objective {:.3} | time {:.2}s | damage {:.1} | healing {:.1}",
                    stage_idx + 1,
                    lvl,
                    stage_objective,
                    surv,
                    dmg,
                    heal
                );
            }
        }
    }

    let default_output_directory = default_run_output_directory(
        search_quality_profile,
        max_runtime_seconds,
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
    );
    let report_path = report_path_override.map(PathBuf::from).unwrap_or_else(|| {
        default_output_directory.join(format!(
            "{}_run_report.md",
            to_norm_key(&controlled_champion_name)
        ))
    });
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }
    // Optional deterministic replay-style timeline for the optimized build run.
    let trace_markdown_path = report_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!(
            "{}_event_trace.md",
            to_norm_key(&controlled_champion_name)
        ));
    let trace_json_path = trace_markdown_path.with_extension("json");
    let mut best_trace_sim_cfg = sim.clone();
    best_trace_sim_cfg.collect_rune_proc_telemetry = true;
    let mut best_trace_sim =
        ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
            controlled_champion_base.clone(),
            &controlled_champion_best_build,
            &controlled_champion_loadout.bonus_stats,
            Some(&controlled_champion_runtime_loadout_selection),
            best_order_acquired_map.as_ref(),
            Some(&controlled_champion_stack_overrides),
            &enemy_builds,
            best_trace_sim_cfg,
            urf.clone(),
        );
    best_trace_sim.enable_trace();
    while best_trace_sim.step(1) {}
    let best_trace = best_trace_sim.trace_events().to_vec();
    let best_rune_proc_telemetry = best_trace_sim.controlled_champion_rune_proc_telemetry();

    let mut trace_md = String::new();
    trace_md.push_str(&format!("# {} Event Trace\n\n", controlled_champion_name));
    trace_md.push_str("## Rune Proc Telemetry\n");
    if best_rune_proc_telemetry.is_empty() {
        trace_md.push_str("- none\n\n");
    } else {
        append_rune_proc_telemetry_markdown_entries(
            &mut trace_md,
            "- ",
            "  ",
            &best_rune_proc_telemetry,
            controlled_champion_best_outcome.damage_dealt,
            controlled_champion_best_outcome.healing_done,
        );
        trace_md.push('\n');
    }
    trace_md.push_str("## Optimized Build Trace\n");
    for line in &best_trace {
        if let Some((header, details)) = line.split_once('\n') {
            trace_md.push_str("- ");
            trace_md.push_str(header);
            trace_md.push('\n');
            trace_md.push_str("  ```text\n");
            trace_md.push_str(details);
            if !details.ends_with('\n') {
                trace_md.push('\n');
            }
            trace_md.push_str("  ```\n");
        } else {
            trace_md.push_str("- ");
            trace_md.push_str(line);
            trace_md.push('\n');
        }
    }
    fs::write(&trace_markdown_path, trace_md)?;

    let trace_json = json!({
        "schema_version": CONTROLLED_CHAMPION_TRACE_JSON_SCHEMA_VERSION,
        "event_encoding": "structured",
        "rune_proc_telemetry": rune_proc_telemetry_json(
            &best_rune_proc_telemetry,
            controlled_champion_best_outcome.damage_dealt,
            controlled_champion_best_outcome.healing_done,
        ),
        "events": best_trace
            .iter()
            .map(|line| structured_trace_event(line))
            .collect::<Vec<_>>(),
    });
    fs::write(&trace_json_path, serde_json::to_string_pretty(&trace_json)?)?;

    diagnostics.total_run_seconds = run_start.elapsed().as_secs_f64();
    let report_data = ControlledChampionReportData {
        scenario_path,
        controlled_champion_name: &controlled_champion_name,
        sim: &sim,
        controlled_champion_base_level: &controlled_champion_base,
        controlled_champion_end_stats: &controlled_champion_end_stats,
        stack_notes: &stack_notes,
        controlled_champion_loadout: &controlled_champion_loadout,
        controlled_champion_loadout_selection: &controlled_champion_runtime_loadout_selection,
        enemy_loadout: &enemy_loadout,
        best_build: &controlled_champion_best_build,
        best_score: controlled_champion_best_score,
        best_outcome: &controlled_champion_best_outcome,
        best_rune_proc_telemetry: &best_rune_proc_telemetry,
        best_score_breakdown,
        enemy_builds: &enemy_builds,
        enemy_derived_combat_stats: &enemy_derived_combat_stats,
        enemy_similarity_notes: &enemy_similarity_notes,
        enemy_presets_used: &enemy_presets_used,
        diverse_top_builds: &diverse_top_builds,
        diverse_top_keys: &diverse_top_keys,
        build_confidence: &build_confidence,
        metrics_by_key: &metrics_by_key,
        pareto_front: &pareto_front,
        diagnostics: &diagnostics,
        build_orders: &build_order_results,
    };
    write_controlled_champion_report_markdown(&report_path, &report_data)?;
    let json_report_path = report_path.with_extension("json");
    write_controlled_champion_report_json(&json_report_path, &report_data)?;

    status.emit(
        "finalization",
        Some((processed_candidates, total_candidates)),
        Some(controlled_champion_best_score),
        Some("reports and trace outputs written"),
        true,
    );
    println!(
        "\nReport written: {}",
        format_repo_relative_path(&report_path)
    );
    println!(
        "Structured report written: {}",
        format_repo_relative_path(&json_report_path)
    );
    println!(
        "Trace report written: {}",
        format_repo_relative_path(&trace_markdown_path)
    );
    println!(
        "Trace json written: {}",
        format_repo_relative_path(&trace_json_path)
    );

    Ok(())
}

pub(super) fn run_controlled_champion_stepper(scenario_path: &Path, ticks: usize) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let simulation_config = scenario
        .get("simulation")
        .ok_or_else(|| anyhow!("Missing simulation"))?;
    let mut sim_cfg = parse_simulation_config(simulation_config)?;
    sim_cfg.collect_rune_proc_telemetry = false;
    let controlled_champion_config = parse_controlled_champion_config(
        &scenario,
        &champion_bases,
        sim_cfg.champion_level,
        &sim_cfg.stack_overrides,
    )?;
    let simulation_level_before_controlled_override = sim_cfg.champion_level;
    sim_cfg.champion_level = controlled_champion_config.level;
    apply_level_scaled_sim_defaults_after_controlled_level_override(
        &mut sim_cfg,
        simulation_config,
        simulation_level_before_controlled_override,
    );
    let controlled_champion_base =
        champion_at_level(&controlled_champion_config.base, sim_cfg.champion_level);
    sim_cfg.controlled_champion_script =
        resolve_controlled_champion_script(&controlled_champion_base.name);
    let controlled_champion_loadout_selection = controlled_champion_config.loadout_selection;
    let controlled_champion_stack_overrides = controlled_champion_config.stack_overrides;

    let enemy_encounters = parse_opponent_encounters(
        &scenario,
        &champion_bases,
        sim_cfg.champion_level,
        &sim_cfg.stack_overrides,
    )?;
    let (selected_encounter_name, _, selected_enemies_raw) = enemy_encounters
        .first()
        .cloned()
        .ok_or_else(|| anyhow!("opponents.encounters must include at least one encounter"))?;
    let enemies = selected_enemies_raw
        .into_iter()
        .map(|mut e| {
            e.base = champion_at_level(&e.base, e.level);
            e
        })
        .collect::<Vec<_>>();

    let loadout_domain = build_loadout_domain();
    let controlled_champion_loadout_selection =
        ensure_complete_loadout_selection(&controlled_champion_loadout_selection, &loadout_domain)?;
    let controlled_champion_loadout = resolve_loadout(
        &controlled_champion_loadout_selection,
        sim_cfg.champion_level,
        true,
    )?;
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain, &urf)?;

    let mut enemy_builds: Vec<(EnemyConfig, Vec<Item>, Stats)> = Vec::new();
    for enemy in &enemies {
        let key = to_norm_key(&enemy.name);
        let preset = enemy_presets.get(&key).ok_or_else(|| {
            anyhow!(
                "Missing URF preset for enemy champion '{}'. Add it to {}.",
                enemy.name,
                enemy_preset_data_path().display()
            )
        })?;
        let build = item_pool_from_names(&items, &preset.item_names)?;
        let bonus_stats =
            resolve_loadout(&enemy_loadout_from_preset(preset), enemy.level, false)?.bonus_stats;
        let mut enemy_with_loadout = enemy.clone();
        enemy_with_loadout.loadout_item_names = preset.item_names.clone();
        enemy_with_loadout.loadout_rune_names = preset.runes.clone();
        enemy_with_loadout.loadout_shards = preset.shards.clone();
        enemy_builds.push((enemy_with_loadout, build, bonus_stats));
    }
    let controlled_champion_items: Vec<Item> = Vec::new();

    let mut sim = ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
        controlled_champion_base,
        &controlled_champion_items,
        &controlled_champion_loadout.bonus_stats,
        Some(&controlled_champion_loadout_selection),
        None,
        Some(&controlled_champion_stack_overrides),
        &enemy_builds,
        sim_cfg.clone(),
        urf,
    );

    println!(
        "Server tick rate: {:.2} Hz ({:.5}s/tick)",
        sim_cfg.server_tick_rate_hz,
        sim.tick_seconds()
    );
    println!("Using opponent encounter: {}", selected_encounter_name);

    for tick in 0..ticks.max(1) {
        let alive = sim.step(1);
        let status = if alive { "alive" } else { "finished" };
        println!(
            "tick={} time={:.3}s health={:.2} targetable={} can_cast={} status={}",
            tick + 1,
            sim.current_time(),
            sim.current_health(),
            sim.is_targetable(),
            sim.can_cast(),
            status
        );
        if !alive {
            break;
        }
    }
    Ok(())
}

pub(super) fn run_stat_optimization(
    stat_key: &str,
    scenario_path: &Path,
    label: &str,
) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let scenario = load_json(scenario_path)?;
    let search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    let item_pool = default_item_pool(&items, &urf);

    let build_indices = choose_best_build_by_stat(
        &item_pool,
        stat_key,
        search_cfg.max_items,
        search_cfg.beam_width,
    );
    let build = build_from_indices(&item_pool, &build_indices);
    let stats = build_item_stats(&build);
    let value = stats.get_stat(stat_key);

    println!("Best build for {}:", label);
    println!(
        "- Items: {}",
        build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Total {}: {:.2}", label, value);
    Ok(())
}

fn build_enemy_similarity_notes(profiles: &[EnemyDerivedCombatStats]) -> Vec<String> {
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

#[cfg(test)]
#[path = "tests/scenario_runner_tests.rs"]
mod tests;

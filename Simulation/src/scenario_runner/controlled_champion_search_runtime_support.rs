use anyhow::Result;
use serde_json::Value;
use std::collections::HashSet;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::*;

mod coverage_locked_asset_candidate_generation;
mod search_runtime_reporting_projections;
mod search_seed_derivation;

pub(super) use self::coverage_locked_asset_candidate_generation::{
    complete_partial_candidate_to_full, coverage_locked_assets,
    filter_item_pool_to_modeled_runtime_effects, max_legal_build_size, mutate_locked_candidate,
    random_locked_candidate,
};
pub(super) use self::search_runtime_reporting_projections::{
    append_rune_proc_telemetry_markdown_entries, build_enemy_similarity_notes,
    deterministic_signature_json, rune_proc_telemetry_json, structured_trace_event,
    verify_deterministic_replay_signature_match,
};
pub(super) use self::search_seed_derivation::{
    fixed_sweep_keystone_seed_base, fixed_sweep_repeat_seed, partial_candidate_completion_seed,
};

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

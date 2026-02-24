use crate::data::EnemyUrfPreset;
use crate::engine::EnemyDerivedCombatStats;
use crate::scripts::champions::ChampionRuneProcTelemetryEntry;
use crate::simulation_contracts::{
    ChampionBase, EnemyConfig, Item, SimulationConfig, Stats, UrfBuffs,
};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone)]
pub(crate) struct BuildSearchConfig {
    pub(crate) strategy: String,
    pub(crate) beam_width: usize,
    pub(crate) max_items: usize,
    pub(crate) random_samples: usize,
    pub(crate) hill_climb_restarts: usize,
    pub(crate) hill_climb_steps: usize,
    pub(crate) hill_climb_neighbors: usize,
    pub(crate) genetic_population: usize,
    pub(crate) genetic_generations: usize,
    pub(crate) genetic_mutation_rate: f64,
    pub(crate) genetic_crossover_rate: f64,
    pub(crate) portfolio_strategies: Vec<String>,
    pub(crate) ranked_limit: usize,
    pub(crate) simulated_annealing_restarts: usize,
    pub(crate) simulated_annealing_iterations: usize,
    pub(crate) simulated_annealing_initial_temp: f64,
    pub(crate) simulated_annealing_cooling_rate: f64,
    pub(crate) mcts_iterations: usize,
    pub(crate) mcts_rollouts_per_expansion: usize,
    pub(crate) mcts_exploration: f64,
    pub(crate) ensemble_seeds: usize,
    pub(crate) ensemble_seed_stride: u64,
    pub(crate) ensemble_seed_top_k: usize,
    pub(crate) objective_survival_weight: f64,
    pub(crate) objective_damage_weight: f64,
    pub(crate) objective_healing_weight: f64,
    pub(crate) objective_enemy_kills_weight: f64,
    pub(crate) objective_invulnerable_seconds_weight: f64,
    pub(crate) robust_min_seed_hit_rate: f64,
    pub(crate) bleed_enabled: bool,
    pub(crate) bleed_budget: usize,
    pub(crate) bleed_mutation_rate: f64,
    pub(crate) multi_scenario_worst_weight: f64,
    pub(crate) strict_ranking_enable_heuristic_ordering: bool,
    pub(crate) strict_ranking_rune_signal_weight: f64,
    pub(crate) strict_ranking_shard_signal_weight: f64,
    pub(crate) strict_ranking_exploration_promotions: usize,
    pub(crate) unmodeled_rune_hard_gate: bool,
    pub(crate) unmodeled_rune_penalty_per_rune: f64,
    pub(crate) unmodeled_item_effect_hard_gate: bool,
    pub(crate) unmodeled_item_effect_penalty_per_item: f64,
    pub(crate) seed: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct BuildMetrics {
    pub(crate) objective: f64,
    pub(crate) ehp_mixed: f64,
    pub(crate) ap: f64,
    pub(crate) cost_timing: f64,
    pub(crate) total_cost: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct CombatOutcome {
    pub(crate) time_alive_seconds: f64,
    pub(crate) damage_dealt: f64,
    pub(crate) healing_done: f64,
    pub(crate) enemy_kills: usize,
    pub(crate) invulnerable_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ObjectiveComponentWeights {
    pub(crate) survival: f64,
    pub(crate) damage: f64,
    pub(crate) healing: f64,
    pub(crate) enemy_kills: f64,
    pub(crate) invulnerable_seconds: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ObjectiveComponentImpact {
    pub(crate) weight: f64,
    pub(crate) normalized_ratio: f64,
    pub(crate) contribution: f64,
    pub(crate) impact_percent: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ObjectiveScoreBreakdown {
    pub(crate) weighted_mean_score: f64,
    pub(crate) worst_case_score: f64,
    pub(crate) worst_case_weight: f64,
    pub(crate) final_score: f64,
    pub(crate) survival: ObjectiveComponentImpact,
    pub(crate) damage: ObjectiveComponentImpact,
    pub(crate) healing: ObjectiveComponentImpact,
    pub(crate) enemy_kills: ObjectiveComponentImpact,
    pub(crate) invulnerable_seconds: ObjectiveComponentImpact,
}

#[derive(Debug, Clone)]
pub(crate) struct BuildConfidence {
    pub(crate) key: BuildKey,
    pub(crate) seed_hits: usize,
    pub(crate) seed_hit_rate: f64,
    pub(crate) robustness: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SearchTypeBreakdown {
    pub(crate) name: String,
    pub(crate) score_requests: usize,
    pub(crate) new_simulations: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct SearchDiagnostics {
    pub(crate) strategy_summary: String,
    pub(crate) search_quality_profile: String,
    pub(crate) effective_seed: u64,
    pub(crate) ensemble_seeds: usize,
    pub(crate) effective_threads: usize,
    pub(crate) seed_orchestration_parallel: bool,
    pub(crate) portfolio_strategy_parallel: bool,
    pub(crate) strategy_elites_parallel: bool,
    pub(crate) objective_survival_weight: f64,
    pub(crate) objective_damage_weight: f64,
    pub(crate) objective_healing_weight: f64,
    pub(crate) objective_enemy_kills_weight: f64,
    pub(crate) objective_invulnerable_seconds_weight: f64,
    pub(crate) full_evaluations: usize,
    pub(crate) full_cache_hits: usize,
    pub(crate) full_cache_misses: usize,
    pub(crate) full_cache_waits: usize,
    pub(crate) candidate_keys_generated: usize,
    pub(crate) candidate_duplicates_pruned: usize,
    pub(crate) unique_candidate_builds: usize,
    pub(crate) bleed_candidates_injected: usize,
    pub(crate) adaptive_candidates_injected: usize,
    pub(crate) scenario_count: usize,
    pub(crate) loadout_candidates: usize,
    pub(crate) loadout_finalists: usize,
    pub(crate) strict_seed_scored_candidates: usize,
    pub(crate) strict_remaining_candidates: usize,
    pub(crate) strict_non_finite_candidates: usize,
    pub(crate) strict_candidates_skipped_timeout: usize,
    pub(crate) strict_completion_percent: f64,
    pub(crate) strict_heuristic_ordering_enabled: bool,
    pub(crate) strict_ranking_rune_signal_weight: f64,
    pub(crate) strict_ranking_shard_signal_weight: f64,
    pub(crate) strict_random_promotions_done: usize,
    pub(crate) unmodeled_rune_hard_gate: bool,
    pub(crate) unmodeled_rune_penalty_per_rune: f64,
    pub(crate) unmodeled_rune_candidates_rejected: usize,
    pub(crate) unmodeled_rune_candidates_penalized: usize,
    pub(crate) unmodeled_item_effect_hard_gate: bool,
    pub(crate) unmodeled_item_effect_penalty_per_item: f64,
    pub(crate) unmodeled_item_effect_candidates_rejected: usize,
    pub(crate) unmodeled_item_effect_candidates_penalized: usize,
    pub(crate) unique_scored_candidates: usize,
    pub(crate) time_budget_seconds: Option<f64>,
    pub(crate) popcorn_window_seconds: Option<f64>,
    pub(crate) popcorn_min_relative_improvement_percent: f64,
    pub(crate) significant_improvement_events: usize,
    pub(crate) best_significant_score: Option<f64>,
    pub(crate) seconds_since_last_significant_improvement: Option<f64>,
    pub(crate) search_type_breakdown: Vec<SearchTypeBreakdown>,
    pub(crate) estimated_total_candidate_space: Option<f64>,
    pub(crate) estimated_run_space_coverage_percent: Option<f64>,
    pub(crate) estimated_close_to_optimal_probability: Option<f64>,
    pub(crate) estimated_close_to_optimal_probability_note: String,
    pub(crate) coverage_stage_enabled: bool,
    pub(crate) coverage_stage_elapsed_seconds: f64,
    pub(crate) coverage_stage_assets_total: usize,
    pub(crate) coverage_stage_assets_covered: usize,
    pub(crate) coverage_stage_seed_candidates: usize,
    pub(crate) coverage_stage_seed_candidates_unique: usize,
    pub(crate) coverage_stage_incomplete: bool,
    pub(crate) coverage_stage_warning: String,
    pub(crate) elapsed_seconds: f64,
    pub(crate) total_run_seconds: f64,
    pub(crate) timed_out: bool,
    pub(crate) processed_candidates: usize,
    pub(crate) total_candidates: usize,
    pub(crate) seed_best_scores: Vec<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub(crate) struct LoadoutSelection {
    pub(crate) rune_names: Vec<String>,
    pub(crate) shard_stats: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ResolvedLoadout {
    pub(crate) selection_labels: Vec<String>,
    pub(crate) bonus_stats: Stats,
    pub(crate) applied_notes: Vec<String>,
    pub(crate) skipped_notes: Vec<String>,
    pub(crate) unmodeled_rune_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct BuildOrderResult {
    pub(crate) ordered_items: Vec<Item>,
    pub(crate) levels: Vec<usize>,
    pub(crate) acquired_levels: Vec<usize>,
    pub(crate) stage_survival: Vec<f64>,
    pub(crate) stage_damage: Vec<f64>,
    pub(crate) stage_healing: Vec<f64>,
    pub(crate) stage_objective_scores: Vec<f64>,
    pub(crate) cumulative_score: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct BuildCandidateKey {
    pub(crate) item_indices: Vec<usize>,
    pub(crate) loadout_selection: LoadoutSelection,
}

pub(crate) type BuildKey = BuildCandidateKey;
pub(crate) type EnemyBuildEntry = (EnemyConfig, Vec<Item>, Stats);
pub(crate) type EnemyBuildScenario = (String, f64, Vec<EnemyBuildEntry>);
pub(crate) type ResolvedByCandidateMap = HashMap<BuildKey, ResolvedLoadout>;
pub(crate) type OutcomeByCandidateMap = HashMap<BuildKey, CombatOutcome>;

pub(crate) struct ObjectiveEvalContext<'a> {
    pub(crate) controlled_champion_base: &'a ChampionBase,
    pub(crate) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(crate) enemy_build_scenarios: &'a [EnemyBuildScenario],
    pub(crate) sim: &'a SimulationConfig,
    pub(crate) urf: &'a UrfBuffs,
    pub(crate) scenario_reference_outcomes: &'a [CombatOutcome],
    pub(crate) weights: ObjectiveComponentWeights,
    pub(crate) worst_case_weight: f64,
}

pub(crate) struct BuildOrderEvalContext<'a> {
    pub(crate) controlled_champion_base_raw: &'a ChampionBase,
    pub(crate) controlled_champion_bonus_stats: &'a Stats,
    pub(crate) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(crate) enemy_build_scenarios: &'a [EnemyBuildScenario],
    pub(crate) raw_enemy_bases: &'a HashMap<String, ChampionBase>,
    pub(crate) sim: &'a SimulationConfig,
    pub(crate) urf: &'a UrfBuffs,
    pub(crate) objective_weights: ObjectiveComponentWeights,
    pub(crate) multi_scenario_worst_weight: f64,
}

pub(crate) struct ControlledChampionReportData<'a> {
    pub(crate) scenario_path: &'a Path,
    pub(crate) controlled_champion_name: &'a str,
    pub(crate) sim: &'a SimulationConfig,
    pub(crate) controlled_champion_base_level: &'a ChampionBase,
    pub(crate) controlled_champion_end_stats: &'a Stats,
    pub(crate) stack_notes: &'a [String],
    pub(crate) controlled_champion_loadout: &'a ResolvedLoadout,
    pub(crate) controlled_champion_loadout_selection: &'a LoadoutSelection,
    pub(crate) enemy_loadout: &'a ResolvedLoadout,
    pub(crate) best_build: &'a [Item],
    pub(crate) best_score: f64,
    pub(crate) best_outcome: &'a CombatOutcome,
    pub(crate) best_rune_proc_telemetry: &'a [ChampionRuneProcTelemetryEntry],
    pub(crate) best_score_breakdown: ObjectiveScoreBreakdown,
    pub(crate) enemy_builds: &'a [EnemyBuildEntry],
    pub(crate) enemy_derived_combat_stats: &'a [EnemyDerivedCombatStats],
    pub(crate) enemy_similarity_notes: &'a [String],
    pub(crate) enemy_presets_used: &'a HashMap<String, EnemyUrfPreset>,
    pub(crate) diverse_top_builds: &'a [(Vec<Item>, f64)],
    pub(crate) diverse_top_keys: &'a [BuildKey],
    pub(crate) build_confidence: &'a [BuildConfidence],
    pub(crate) metrics_by_key: &'a HashMap<BuildKey, BuildMetrics>,
    pub(crate) pareto_front: &'a HashSet<BuildKey>,
    pub(crate) diagnostics: &'a SearchDiagnostics,
    pub(crate) build_orders: &'a [BuildOrderResult],
}

use super::shared_parsing_primitives::as_str;
use super::*;

pub(crate) fn parse_build_search(data: &Value) -> Result<BuildSearchConfig> {
    let defaults = simulator_defaults();
    let search_defaults = &defaults.search_defaults;
    let portfolio_strategies = data
        .get("portfolio_strategies")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(BuildSearchConfig {
        strategy: as_str(data, "strategy")?.to_string(),
        beam_width: data
            .get("beam_width")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.beam_width as u64) as usize,
        max_items: data
            .get("max_items")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.max_items as u64) as usize,
        random_samples: data
            .get("random_samples")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.random_samples as u64) as usize,
        hill_climb_restarts: data
            .get("hill_climb_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.hill_climb_restarts as u64)
            as usize,
        hill_climb_steps: data
            .get("hill_climb_steps")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.hill_climb_steps as u64) as usize,
        hill_climb_neighbors: data
            .get("hill_climb_neighbors")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.hill_climb_neighbors as u64)
            as usize,
        genetic_population: data
            .get("genetic_population")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.genetic_population as u64)
            as usize,
        genetic_generations: data
            .get("genetic_generations")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.genetic_generations as u64)
            as usize,
        genetic_mutation_rate: data
            .get("genetic_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.genetic_mutation_rate),
        genetic_crossover_rate: data
            .get("genetic_crossover_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.genetic_crossover_rate),
        portfolio_strategies,
        ranked_limit: data
            .get("ranked_limit")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ranked_limit as u64) as usize,
        simulated_annealing_restarts: data
            .get("simulated_annealing_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.simulated_annealing_restarts as u64)
            as usize,
        simulated_annealing_iterations: data
            .get("simulated_annealing_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.simulated_annealing_iterations as u64)
            as usize,
        simulated_annealing_initial_temp: data
            .get("simulated_annealing_initial_temp")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.simulated_annealing_initial_temp),
        simulated_annealing_cooling_rate: data
            .get("simulated_annealing_cooling_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.simulated_annealing_cooling_rate),
        mcts_iterations: data
            .get("mcts_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.mcts_iterations as u64) as usize,
        mcts_rollouts_per_expansion: data
            .get("mcts_rollouts_per_expansion")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.mcts_rollouts_per_expansion as u64)
            as usize,
        mcts_exploration: data
            .get("mcts_exploration")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.mcts_exploration),
        ensemble_seeds: data
            .get("ensemble_seeds")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ensemble_seeds as u64) as usize,
        ensemble_seed_stride: data
            .get("ensemble_seed_stride")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ensemble_seed_stride),
        ensemble_seed_top_k: data
            .get("ensemble_seed_top_k")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.ensemble_seed_top_k as u64)
            as usize,
        objective_survival_weight: data
            .get("objective_survival_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_survival_weight),
        objective_damage_weight: data
            .get("objective_damage_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_damage_weight),
        objective_healing_weight: data
            .get("objective_healing_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_healing_weight),
        objective_enemy_kills_weight: data
            .get("objective_enemy_kills_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_enemy_kills_weight),
        objective_invulnerable_seconds_weight: data
            .get("objective_invulnerable_seconds_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.objective_invulnerable_seconds_weight),
        robust_min_seed_hit_rate: data
            .get("robust_min_seed_hit_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.robust_min_seed_hit_rate),
        bleed_enabled: data
            .get("bleed_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.bleed_enabled),
        bleed_budget: data
            .get("bleed_budget")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.bleed_budget as u64) as usize,
        bleed_mutation_rate: data
            .get("bleed_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.bleed_mutation_rate),
        multi_scenario_worst_weight: data
            .get("multi_scenario_worst_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.multi_scenario_worst_weight),
        strict_ranking_enable_heuristic_ordering: data
            .get("strict_ranking_enable_heuristic_ordering")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.strict_ranking_enable_heuristic_ordering),
        strict_ranking_rune_signal_weight: data
            .get("strict_ranking_rune_signal_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.strict_ranking_rune_signal_weight),
        strict_ranking_shard_signal_weight: data
            .get("strict_ranking_shard_signal_weight")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.strict_ranking_shard_signal_weight),
        strict_ranking_exploration_promotions: data
            .get("strict_ranking_exploration_promotions")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.strict_ranking_exploration_promotions as u64)
            as usize,
        unmodeled_rune_hard_gate: data
            .get("unmodeled_rune_hard_gate")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.unmodeled_rune_hard_gate),
        unmodeled_rune_penalty_per_rune: data
            .get("unmodeled_rune_penalty_per_rune")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.unmodeled_rune_penalty_per_rune),
        unmodeled_item_effect_hard_gate: data
            .get("unmodeled_item_effect_hard_gate")
            .and_then(Value::as_bool)
            .unwrap_or(search_defaults.unmodeled_item_effect_hard_gate),
        unmodeled_item_effect_penalty_per_item: data
            .get("unmodeled_item_effect_penalty_per_item")
            .and_then(Value::as_f64)
            .unwrap_or(search_defaults.unmodeled_item_effect_penalty_per_item),
        seed: data
            .get("seed")
            .and_then(Value::as_u64)
            .unwrap_or(search_defaults.seed),
    })
}

pub(crate) fn apply_search_quality_profile(
    search: &mut BuildSearchConfig,
    profile: SearchQualityProfile,
) {
    fn apply_profile_overrides(search: &mut BuildSearchConfig, preset: SearchQualityProfilePreset) {
        search.beam_width = preset.beam_width;
        search.random_samples = preset.random_samples;
        search.hill_climb_restarts = preset.hill_climb_restarts;
        search.hill_climb_steps = preset.hill_climb_steps;
        search.hill_climb_neighbors = preset.hill_climb_neighbors;
        search.genetic_population = preset.genetic_population;
        search.genetic_generations = preset.genetic_generations;
        search.simulated_annealing_restarts = preset.simulated_annealing_restarts;
        search.simulated_annealing_iterations = preset.simulated_annealing_iterations;
        search.mcts_iterations = preset.mcts_iterations;
        search.mcts_rollouts_per_expansion = preset.mcts_rollouts_per_expansion;
        search.ensemble_seeds = preset.ensemble_seeds;
        search.ensemble_seed_top_k = preset.ensemble_seed_top_k;
        search.ranked_limit = preset.ranked_limit;
        search.bleed_budget = preset.bleed_budget;
    }

    fn apply_profile_minimums(search: &mut BuildSearchConfig, preset: SearchQualityProfilePreset) {
        search.beam_width = search.beam_width.max(preset.beam_width);
        search.random_samples = search.random_samples.max(preset.random_samples);
        search.hill_climb_restarts = search.hill_climb_restarts.max(preset.hill_climb_restarts);
        search.hill_climb_steps = search.hill_climb_steps.max(preset.hill_climb_steps);
        search.hill_climb_neighbors = search.hill_climb_neighbors.max(preset.hill_climb_neighbors);
        search.genetic_population = search.genetic_population.max(preset.genetic_population);
        search.genetic_generations = search.genetic_generations.max(preset.genetic_generations);
        search.simulated_annealing_restarts = search
            .simulated_annealing_restarts
            .max(preset.simulated_annealing_restarts);
        search.simulated_annealing_iterations = search
            .simulated_annealing_iterations
            .max(preset.simulated_annealing_iterations);
        search.mcts_iterations = search.mcts_iterations.max(preset.mcts_iterations);
        search.mcts_rollouts_per_expansion = search
            .mcts_rollouts_per_expansion
            .max(preset.mcts_rollouts_per_expansion);
        search.ensemble_seeds = search.ensemble_seeds.max(preset.ensemble_seeds);
        search.ensemble_seed_top_k = search.ensemble_seed_top_k.max(preset.ensemble_seed_top_k);
        search.ranked_limit = search.ranked_limit.max(preset.ranked_limit);
        search.bleed_budget = search.bleed_budget.max(preset.bleed_budget);
    }

    let profile_defaults = &simulator_defaults().search_quality_profile_defaults;
    match profile {
        SearchQualityProfile::Fast => {
            apply_profile_overrides(search, profile_defaults.fast);
            search.unmodeled_rune_hard_gate = false;
            search.unmodeled_rune_penalty_per_rune =
                search.unmodeled_rune_penalty_per_rune.max(0.0);
            search.unmodeled_item_effect_hard_gate = false;
            search.unmodeled_item_effect_penalty_per_item =
                search.unmodeled_item_effect_penalty_per_item.max(0.0);
        }
        SearchQualityProfile::Balanced => {
            apply_profile_overrides(search, profile_defaults.balanced);
            search.unmodeled_rune_hard_gate = false;
            search.unmodeled_rune_penalty_per_rune =
                search.unmodeled_rune_penalty_per_rune.max(0.0);
            search.unmodeled_item_effect_hard_gate = false;
            search.unmodeled_item_effect_penalty_per_item =
                search.unmodeled_item_effect_penalty_per_item.max(0.0);
        }
        SearchQualityProfile::MaximumQuality => {
            apply_profile_minimums(search, profile_defaults.maximum_quality_minimums);
            search.unmodeled_rune_hard_gate = true;
            search.unmodeled_rune_penalty_per_rune = 0.0;
            search.unmodeled_item_effect_hard_gate = true;
            search.unmodeled_item_effect_penalty_per_item = 0.0;
        }
    }
}

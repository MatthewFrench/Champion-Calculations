use super::*;

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

pub(crate) fn apply_search_quality_profile(
    search: &mut BuildSearchConfig,
    profile: SearchQualityProfile,
) {
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

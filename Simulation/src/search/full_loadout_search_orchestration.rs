use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct FullLoadoutSearchParams<'a> {
    pub item_pool: &'a [Item],
    pub max_items: usize,
    pub loadout_domain: &'a LoadoutDomain,
    pub base_loadout: &'a LoadoutSelection,
}

pub(crate) fn build_search_ranked_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(BuildKey, f64)>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    if deadline_reached(deadline) {
        return Vec::new();
    }
    match search.strategy.as_str() {
        "greedy" => {
            let mut seed = search.seed;
            let mut candidate = BuildKey {
                item_indices: Vec::new(),
                loadout_selection: params.base_loadout.clone(),
            };
            for _ in 0..params.max_items {
                if deadline_reached(deadline) {
                    break;
                }
                let mut best: Option<BuildKey> = None;
                let mut best_score = f64::NEG_INFINITY;
                for item_idx in 0..params.item_pool.len() {
                    if !can_add_item_to_build(params.item_pool, &candidate.item_indices, item_idx) {
                        continue;
                    }
                    let mut next = candidate.clone();
                    next.item_indices.push(item_idx);
                    next.item_indices = canonical_key(&next.item_indices);
                    let loadout_variants =
                        candidate_loadout_variants(&next.loadout_selection, params, &mut seed, 4);
                    for loadout_selection in loadout_variants {
                        let mut probe = next.clone();
                        probe.loadout_selection = loadout_selection;
                        probe = canonical_build_candidate(probe);
                        let score = score_fn(&probe);
                        if score > best_score {
                            best_score = score;
                            best = Some(probe);
                        }
                    }
                }
                if let Some(next) = best {
                    candidate = next;
                } else {
                    break;
                }
            }
            vec![(candidate.clone(), score_fn(&candidate))]
        }
        "beam" => {
            beam_search_ranked_full(params, search.beam_width, search.seed, score_fn, deadline)
        }
        "random" => random_search_ranked_full(
            params,
            search.random_samples,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "hill_climb" => hill_climb_search_ranked_full(
            params,
            &HillClimbSearchConfig {
                restarts: search.hill_climb_restarts,
                steps: search.hill_climb_steps,
                neighbors_per_step: search.hill_climb_neighbors,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "genetic" => genetic_search_ranked_full(
            params,
            &GeneticSearchConfig {
                population_size: search.genetic_population,
                generations: search.genetic_generations,
                mutation_rate: search.genetic_mutation_rate,
                crossover_rate: search.genetic_crossover_rate,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "simulated_annealing" => simulated_annealing_search_ranked_full(
            params,
            &SimulatedAnnealingSearchConfig {
                restarts: search.simulated_annealing_restarts,
                iterations: search.simulated_annealing_iterations,
                initial_temp: search.simulated_annealing_initial_temp,
                cooling_rate: search.simulated_annealing_cooling_rate,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "mcts" => mcts_search_ranked_full(
            params,
            &MctsSearchConfig {
                iterations: search.mcts_iterations,
                rollouts_per_expansion: search.mcts_rollouts_per_expansion,
                exploration: search.mcts_exploration,
                seed: search.seed,
                limit: search.ranked_limit,
            },
            score_fn,
            deadline,
        ),
        "portfolio" => {
            let strategies = portfolio_strategy_list(search);
            let mut ranked_sets = strategies
                .par_iter()
                .enumerate()
                .map(|(idx, strategy)| {
                    if deadline_reached(deadline) {
                        return (idx, Vec::new());
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strategy.clone();
                    cfg.seed = search.seed.wrapping_add((idx as u64 + 1) * 1_000_003);
                    (
                        idx,
                        build_search_ranked_full_loadout(params, &cfg, score_fn, deadline),
                    )
                })
                .collect::<Vec<_>>();
            ranked_sets.sort_by_key(|(idx, _)| *idx);
            let merged = ranked_sets
                .into_iter()
                .flat_map(|(_, ranked)| ranked.into_iter().map(|(candidate, _)| candidate))
                .collect::<Vec<_>>();
            unique_ranked_full_candidates(merged, score_fn, search.ranked_limit, deadline)
        }
        _ => Vec::new(),
    }
}

pub(crate) fn strategy_seed_elites_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategies: &[String],
    score_fn: &F,
    deadline: Option<Instant>,
) -> HashMap<String, Vec<BuildKey>>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    let ensemble = search.ensemble_seeds.max(1);
    let top_k = search.ensemble_seed_top_k.max(1);
    let mut grouped = strategies
        .par_iter()
        .enumerate()
        .map(|(strategy_index, strategy)| {
            let mut aggregate = HashMap::<BuildKey, f64>::new();
            let seed_ranked = (0..ensemble)
                .into_par_iter()
                .map(|seed_idx| {
                    if deadline_reached(deadline) {
                        return Vec::new();
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strategy.clone();
                    cfg.seed = search.seed.wrapping_add(
                        ((strategy_index as u64 + 1) * 31 + seed_idx as u64 + 1)
                            * search.ensemble_seed_stride,
                    );
                    cfg.ranked_limit = top_k.max(64);
                    build_search_ranked_full_loadout(params, &cfg, score_fn, deadline)
                })
                .collect::<Vec<_>>();
            for ranked in seed_ranked {
                for (candidate, score) in ranked.into_iter().take(top_k) {
                    let entry = aggregate.entry(candidate).or_insert(score);
                    if score > *entry {
                        *entry = score;
                    }
                }
            }
            let mut entries = aggregate.into_iter().collect::<Vec<_>>();
            entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            let elites = entries
                .into_iter()
                .map(|(candidate, _)| candidate)
                .collect::<Vec<_>>();
            (strategy_index, strategy.clone(), elites)
        })
        .collect::<Vec<_>>();
    grouped.sort_by_key(|(idx, _, _)| *idx);
    grouped
        .into_iter()
        .map(|(_, strategy, elites)| (strategy, elites))
        .collect::<HashMap<_, _>>()
}

pub(crate) fn adaptive_strategy_candidates_full_loadout<F>(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<BuildKey>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<BuildKey>
where
    F: Fn(&BuildKey) -> f64 + Sync,
{
    if strategy_elites.is_empty() {
        return Vec::new();
    }
    let mut strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    strategies.sort_unstable();
    let contributions = strategies
        .iter()
        .map(|strategy| {
            let contribution = strategy_elites
                .get(strategy)
                .map(|candidates| candidates.len().max(1) as f64)
                .unwrap_or(1.0);
            (strategy.clone(), contribution)
        })
        .collect::<Vec<_>>();
    let total_contribution = contributions
        .iter()
        .map(|(_, contribution)| *contribution)
        .sum::<f64>()
        .max(1.0);
    let extra_runs_total = (search.ensemble_seeds.max(1) * strategies.len()).max(8);
    let per_strategy = contributions
        .into_iter()
        .map(|(strategy, contribution)| {
            let share = contribution / total_contribution;
            let runs = ((extra_runs_total as f64) * share).round() as usize;
            (strategy, runs.max(1))
        })
        .collect::<Vec<_>>();

    let mut out = HashSet::<BuildKey>::new();
    let gathered = per_strategy
        .par_iter()
        .enumerate()
        .map(|(strategy_idx, (strategy, runs))| {
            (0..*runs)
                .into_par_iter()
                .flat_map_iter(|run_idx| {
                    if deadline_reached(deadline) {
                        return Vec::<BuildKey>::new().into_iter();
                    }
                    let mut cfg = search.clone();
                    cfg.strategy = strategy.clone();
                    cfg.seed = search.seed.wrapping_add(
                        ((strategy_idx as u64 + 1) * 131 + run_idx as u64 + 1)
                            * search.ensemble_seed_stride,
                    );
                    cfg.ranked_limit = (search.ensemble_seed_top_k.max(1) * 2).max(50);
                    let ranked = build_search_ranked_full_loadout(params, &cfg, score_fn, deadline);
                    ranked
                        .into_iter()
                        .take(search.ensemble_seed_top_k.max(1))
                        .map(|(candidate, _)| candidate)
                        .collect::<Vec<_>>()
                        .into_iter()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for candidates in gathered {
        for candidate in candidates {
            out.insert(candidate);
        }
    }
    let mut out_vec = out.into_iter().collect::<Vec<_>>();
    out_vec.sort_by_key(candidate_order_key);
    out_vec
}

pub(crate) fn generate_bleed_candidates_full_loadout(
    params: &FullLoadoutSearchParams<'_>,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<BuildKey>>,
) -> Vec<BuildKey> {
    if !search.bleed_enabled {
        return Vec::new();
    }
    let mut seed = search.seed ^ 0xB1EED_u64;
    let mut out = Vec::new();
    let mut seen = HashSet::<BuildKey>::new();
    let mut strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    strategies.sort_unstable();
    let mut elite_pool = Vec::new();

    for strategy in &strategies {
        if let Some(candidates) = strategy_elites.get(strategy) {
            for candidate in candidates.iter().take(search.ensemble_seed_top_k.max(1)) {
                let canonical = canonical_build_candidate(candidate.clone());
                if seen.insert(canonical.clone()) {
                    out.push(canonical.clone());
                    elite_pool.push(canonical);
                }
            }
        }
    }
    if elite_pool.is_empty() {
        return out;
    }

    let bleed_budget = if search.bleed_budget > 0 {
        search.bleed_budget
    } else {
        search.ranked_limit.max(800)
    };
    let cross_budget = bleed_budget / 2;
    let mutate_budget = bleed_budget - cross_budget;
    let mutation_rate = search.bleed_mutation_rate.clamp(0.0, 1.0);

    for _ in 0..cross_budget {
        let parent_a = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        let child = if strategies.len() >= 2 {
            let strategy_a = rand_index(&mut seed, strategies.len());
            let mut strategy_b = rand_index(&mut seed, strategies.len());
            if strategy_b == strategy_a {
                strategy_b = (strategy_b + 1) % strategies.len();
            }
            let list_a = strategy_elites
                .get(&strategies[strategy_a])
                .unwrap_or(&elite_pool);
            let list_b = strategy_elites
                .get(&strategies[strategy_b])
                .unwrap_or(&elite_pool);
            let parent_a_candidate = list_a
                .get(rand_index(&mut seed, list_a.len()))
                .cloned()
                .unwrap_or(parent_a.clone());
            let parent_b_candidate = list_b
                .get(rand_index(&mut seed, list_b.len()))
                .cloned()
                .unwrap_or(parent_a.clone());
            let mut child = crossover_full_candidates(
                &parent_a_candidate,
                &parent_b_candidate,
                params,
                &mut seed,
            );
            mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
            canonical_build_candidate(child)
        } else {
            let mut child = parent_a.clone();
            mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
            canonical_build_candidate(child)
        };
        if seen.insert(child.clone()) {
            out.push(child);
        }
    }

    for _ in 0..mutate_budget {
        let mut child = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        mutate_full_candidate(params, &mut child, mutation_rate, &mut seed);
        repair_full_candidate(params, &mut child, &mut seed);
        let canonical = canonical_build_candidate(child);
        if seen.insert(canonical.clone()) {
            out.push(canonical);
        }
    }
    out.sort_by_key(candidate_order_key);
    out
}

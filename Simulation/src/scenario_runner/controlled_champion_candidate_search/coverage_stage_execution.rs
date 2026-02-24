use std::cmp::Ordering;

use super::*;

#[derive(Debug, Clone)]
pub(in crate::scenario_runner) struct CoverageStageExecution {
    pub(in crate::scenario_runner) diagnostics: CoverageStageDiagnostics,
    pub(in crate::scenario_runner) seed_candidates: Vec<BuildKey>,
}

pub(in crate::scenario_runner) struct CoverageStageRunContext<'a> {
    pub(in crate::scenario_runner) search_quality_profile: SearchQualityProfile,
    pub(in crate::scenario_runner) search_cfg: &'a BuildSearchConfig,
    pub(in crate::scenario_runner) min_item_diff: usize,
    pub(in crate::scenario_runner) item_pool: &'a [Item],
    pub(in crate::scenario_runner) search_loadout_domain: &'a crate::data::LoadoutDomain,
    pub(in crate::scenario_runner) full_search_params: FullLoadoutSearchParams<'a>,
    pub(in crate::scenario_runner) status: &'a mut StatusReporter,
    pub(in crate::scenario_runner) timeout_flag: &'a AtomicUsize,
    pub(in crate::scenario_runner) coverage_stage_deadline: &'a dyn Fn() -> Option<Instant>,
    pub(in crate::scenario_runner) full_score_for_search_type:
        &'a (dyn Fn(&str, &BuildKey) -> f64 + Sync),
}

pub(in crate::scenario_runner) fn run_maximum_quality_coverage_stage(
    context: CoverageStageRunContext<'_>,
) -> CoverageStageExecution {
    let CoverageStageRunContext {
        search_quality_profile,
        search_cfg,
        min_item_diff,
        item_pool,
        search_loadout_domain,
        full_search_params,
        status,
        timeout_flag,
        coverage_stage_deadline,
        full_score_for_search_type,
    } = context;

    let mut diagnostics = CoverageStageDiagnostics::default();
    let mut coverage_seed_candidates = Vec::<BuildKey>::new();
    if !matches!(search_quality_profile, SearchQualityProfile::MaximumQuality) {
        return CoverageStageExecution {
            diagnostics,
            seed_candidates: coverage_seed_candidates,
        };
    }

    diagnostics.enabled = true;
    let coverage_start = Instant::now();
    let coverage_assets = coverage_locked_assets(item_pool, search_loadout_domain);
    diagnostics.assets_total = coverage_assets.len();
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
            diagnostics.assets_covered += 1;
            let diverse = select_diverse_top_candidates(&ranked, 3, min_item_diff.max(1), 100.0);
            if diverse.is_empty() {
                coverage_seed_candidates.push(ranked[0].0.clone());
            } else {
                coverage_seed_candidates
                    .extend(diverse.into_iter().map(|(candidate, _)| candidate));
            }
        }

        let note = asset.display_label(item_pool);
        status.emit(
            "coverage_stage",
            Some((asset_index + 1, coverage_assets.len())),
            None,
            Some(note.as_str()),
            false,
        );
    }

    diagnostics.seed_candidates = coverage_seed_candidates.len();
    coverage_seed_candidates = coverage_seed_candidates
        .into_iter()
        .map(canonical_build_candidate)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    coverage_seed_candidates.sort_by_key(build_key_cache_string);
    diagnostics.seed_candidates_unique = coverage_seed_candidates.len();
    diagnostics.elapsed_seconds = coverage_start.elapsed().as_secs_f64();
    diagnostics.coverage_incomplete = diagnostics.assets_covered < diagnostics.assets_total;
    if diagnostics.coverage_incomplete {
        let reason = if coverage_stage_stopped_early {
            "coverage stage reached a timeout boundary before all assets were touched"
        } else {
            "coverage stage could not produce finite candidates for at least one locked asset"
        };
        diagnostics.coverage_warning = format!(
            "Coverage incomplete: touched {}/{} assets; {}. Continuing search in degraded coverage mode.",
            diagnostics.assets_covered, diagnostics.assets_total, reason
        );
    }

    CoverageStageExecution {
        diagnostics,
        seed_candidates: coverage_seed_candidates,
    }
}

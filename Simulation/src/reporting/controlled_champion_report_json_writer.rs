use super::*;
pub(super) fn report_rune_proc_telemetry_json(
    entries: &[crate::scripts::champions::ChampionRuneProcTelemetryEntry],
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

pub(crate) fn write_controlled_champion_report_json(
    report_path: &Path,
    data: &ControlledChampionReportData<'_>,
) -> Result<()> {
    fn deterministic_signature_json(
        signature: crate::engine::SimulationDeterminismSignature,
    ) -> Value {
        json!({
            "final_state_checksum_hex": format!("{:016x}", signature.final_state_checksum),
            "tick_state_checksum_hex": format!("{:016x}", signature.tick_state_checksum),
            "queue_checksum_hex": format!("{:016x}", signature.queue_checksum),
            "ticks_executed": signature.ticks_executed,
            "events_processed": signature.events_processed
        })
    }

    fn component_json(
        weight: f64,
        normalized_ratio: f64,
        contribution: f64,
        impact_percent: f64,
    ) -> Value {
        json!({
            "weight": weight,
            "normalized_ratio": normalized_ratio,
            "contribution": contribution,
            "impact_percent": impact_percent,
            "delta_vs_weight_percent_points": impact_percent - weight * 100.0
        })
    }
    fn objective_breakdown_json(breakdown: ObjectiveScoreBreakdown) -> Value {
        json!({
            "weighted_mean_score": breakdown.weighted_mean_score,
            "worst_case_score": breakdown.worst_case_score,
            "worst_case_weight": breakdown.worst_case_weight,
            "final_score": breakdown.final_score,
            "components": {
                "survival": component_json(
                    breakdown.survival.weight,
                    breakdown.survival.normalized_ratio,
                    breakdown.survival.contribution,
                    breakdown.survival.impact_percent
                ),
                "damage": component_json(
                    breakdown.damage.weight,
                    breakdown.damage.normalized_ratio,
                    breakdown.damage.contribution,
                    breakdown.damage.impact_percent
                ),
                "healing": component_json(
                    breakdown.healing.weight,
                    breakdown.healing.normalized_ratio,
                    breakdown.healing.contribution,
                    breakdown.healing.impact_percent
                ),
                "enemy_kills": component_json(
                    breakdown.enemy_kills.weight,
                    breakdown.enemy_kills.normalized_ratio,
                    breakdown.enemy_kills.contribution,
                    breakdown.enemy_kills.impact_percent
                ),
                "invulnerable_seconds": component_json(
                    breakdown.invulnerable_seconds.weight,
                    breakdown.invulnerable_seconds.normalized_ratio,
                    breakdown.invulnerable_seconds.contribution,
                    breakdown.invulnerable_seconds.impact_percent
                )
            }
        })
    }

    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let now = SystemTime::now();
    let generated_utc: DateTime<Utc> = now.into();
    let generated_local: DateTime<Local> = DateTime::from(now);
    let scenario_path = data.scenario_path;
    let scenario_path_display = format_repo_relative_path(scenario_path);
    let controlled_champion_name = data.controlled_champion_name;
    let sim = data.sim;
    let best_build = data.best_build;
    let best_score = data.best_score;
    let best_outcome = data.best_outcome;
    let best_rune_proc_telemetry = data.best_rune_proc_telemetry;
    let best_trace_determinism = data.best_trace_determinism;
    let best_score_breakdown = data.best_score_breakdown;
    let controlled_champion_unmodeled_item_effect_names =
        unmodeled_runtime_item_effect_names(best_build);
    let controlled_champion_loadout = data.controlled_champion_loadout;
    let enemy_builds = data.enemy_builds;
    let enemy_derived_combat_stats = data.enemy_derived_combat_stats;
    let enemy_similarity_notes = data.enemy_similarity_notes;
    let enemy_presets_used = data.enemy_presets_used;
    let diverse_top_builds = data.diverse_top_builds;
    let diagnostics = data.diagnostics;
    let build_orders = data.build_orders;
    validate_controlled_champion_selection_labels(
        controlled_champion_name,
        data.controlled_champion_loadout_selection,
        &controlled_champion_loadout.selection_labels,
        &controlled_champion_loadout.unmodeled_rune_names,
    )?;
    let total_score_requests = diagnostics
        .search_type_breakdown
        .iter()
        .map(|breakdown| breakdown.score_requests)
        .sum::<usize>();
    let json_value = json!({
        "schema_version": CONTROLLED_CHAMPION_RUN_REPORT_JSON_SCHEMA_VERSION,
        "generated_utc": generated_utc.to_rfc3339(),
        "generated_local": generated_local.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        "scenario_path": scenario_path_display,
        "controlled_champion_name": controlled_champion_name,
        "champion_level": sim.champion_level,
        "headline": {
            "best_objective_score": best_score,
            "best_outcome": {
                "time_alive_seconds": best_outcome.time_alive_seconds,
                "damage_dealt": best_outcome.damage_dealt,
                "healing_done": best_outcome.healing_done,
                "enemy_kills": best_outcome.enemy_kills,
                "invulnerable_seconds": best_outcome.invulnerable_seconds,
                "cap_survivor": best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
            },
            "best_objective_breakdown": objective_breakdown_json(best_score_breakdown),
        },
        "best_rune_proc_telemetry": report_rune_proc_telemetry_json(
            best_rune_proc_telemetry,
            best_outcome.damage_dealt,
            best_outcome.healing_done,
        ),
        "best_trace_determinism": deterministic_signature_json(best_trace_determinism),
        "best_build": best_build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
        "controlled_champion_loadout_labels": controlled_champion_loadout.selection_labels,
        "controlled_champion_unmodeled_runes": controlled_champion_loadout.unmodeled_rune_names,
        "controlled_champion_unmodeled_item_effects": controlled_champion_unmodeled_item_effect_names,
        "enemy_presets": enemy_builds.iter().map(|(enemy, build, _)| {
            let key = to_norm_key(&enemy.name);
            let preset = enemy_presets_used.get(&key);
            json!({
                "champion": enemy.name,
                "items": build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                "runes": preset.map(|p| p.runes.clone()).unwrap_or_default(),
                "shards": preset.map(|p| p.shards.clone()).unwrap_or_default(),
                "source_url": preset.map(|p| p.source_url.clone()).unwrap_or_default(),
                "last_checked": preset.map(|p| p.last_checked.clone()).unwrap_or_default(),
            })
        }).collect::<Vec<_>>(),
        "enemy_derived_combat_stats": enemy_derived_combat_stats.iter().map(|profile| {
            json!({
                "champion": profile.champion,
                "max_health": profile.max_health,
                "armor": profile.armor,
                "magic_resist": profile.magic_resist,
                "attack_damage": profile.attack_damage,
                "attack_speed": profile.attack_speed,
                "attack_interval_seconds": profile.attack_interval_seconds,
                "attack_range": profile.attack_range,
                "attack_projectile_speed": profile.attack_projectile_speed,
                "move_speed": profile.move_speed,
                "desired_combat_range": profile.desired_combat_range,
                "physical_hit_damage": profile.physical_hit_damage,
                "ability_hit_damage": profile.ability_hit_damage,
                "burst_physical_damage": profile.burst_physical_damage,
                "burst_magic_damage": profile.burst_magic_damage,
                "burst_true_damage": profile.burst_true_damage
            })
        }).collect::<Vec<_>>(),
        "enemy_similarity_notes": enemy_similarity_notes,
        "diverse_top_builds": diverse_top_builds.iter().map(|(build, score)| {
            json!({
                "objective_score": score,
                "items": build.iter().map(|i| i.name.clone()).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>(),
        "build_orders": build_orders.iter().map(|order| {
            json!({
                "cumulative_score": order.cumulative_score,
                "ordered_items": order.ordered_items.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                "levels": order.levels,
                "stage_survival_seconds": order.stage_survival,
                "stage_damage": order.stage_damage,
                "stage_healing": order.stage_healing,
                "stage_objective_scores": order.stage_objective_scores,
            })
        }).collect::<Vec<_>>(),
        "diagnostics": {
            "strategy_summary": diagnostics.strategy_summary,
            "search_quality_profile": diagnostics.search_quality_profile,
            "effective_seed": diagnostics.effective_seed,
            "ensemble_seeds": diagnostics.ensemble_seeds,
            "effective_threads": diagnostics.effective_threads,
            "seed_orchestration_parallel": diagnostics.seed_orchestration_parallel,
            "portfolio_strategy_parallel": diagnostics.portfolio_strategy_parallel,
            "strategy_elites_parallel": diagnostics.strategy_elites_parallel,
            "objective_survival_weight": diagnostics.objective_survival_weight,
            "objective_damage_weight": diagnostics.objective_damage_weight,
            "objective_healing_weight": diagnostics.objective_healing_weight,
            "objective_enemy_kills_weight": diagnostics.objective_enemy_kills_weight,
            "objective_invulnerable_seconds_weight": diagnostics.objective_invulnerable_seconds_weight,
            "full_evaluations": diagnostics.full_evaluations,
            "full_cache_hits": diagnostics.full_cache_hits,
            "full_cache_misses": diagnostics.full_cache_misses,
            "full_cache_waits": diagnostics.full_cache_waits,
            "candidate_keys_generated": diagnostics.candidate_keys_generated,
            "candidate_duplicates_pruned": diagnostics.candidate_duplicates_pruned,
            "unique_candidate_builds": diagnostics.unique_candidate_builds,
            "bleed_candidates_injected": diagnostics.bleed_candidates_injected,
            "adaptive_candidates_injected": diagnostics.adaptive_candidates_injected,
            "scenario_count": diagnostics.scenario_count,
            "loadout_candidates": diagnostics.loadout_candidates,
            "loadout_finalists": diagnostics.loadout_finalists,
            "strict_seed_scored_candidates": diagnostics.strict_seed_scored_candidates,
            "strict_remaining_candidates": diagnostics.strict_remaining_candidates,
            "strict_non_finite_candidates": diagnostics.strict_non_finite_candidates,
            "strict_candidates_skipped_timeout": diagnostics.strict_candidates_skipped_timeout,
            "strict_completion_percent": diagnostics.strict_completion_percent,
            "strict_heuristic_ordering_enabled": diagnostics.strict_heuristic_ordering_enabled,
            "strict_ranking_rune_signal_weight": diagnostics.strict_ranking_rune_signal_weight,
            "strict_ranking_shard_signal_weight": diagnostics.strict_ranking_shard_signal_weight,
            "strict_random_promotions_done": diagnostics.strict_random_promotions_done,
            "unmodeled_rune_hard_gate": diagnostics.unmodeled_rune_hard_gate,
            "unmodeled_rune_penalty_per_rune": diagnostics.unmodeled_rune_penalty_per_rune,
            "unmodeled_rune_candidates_rejected": diagnostics.unmodeled_rune_candidates_rejected,
            "unmodeled_rune_candidates_penalized": diagnostics.unmodeled_rune_candidates_penalized,
            "unmodeled_item_effect_hard_gate": diagnostics.unmodeled_item_effect_hard_gate,
            "unmodeled_item_effect_penalty_per_item": diagnostics.unmodeled_item_effect_penalty_per_item,
            "unmodeled_item_effect_candidates_rejected": diagnostics.unmodeled_item_effect_candidates_rejected,
            "unmodeled_item_effect_candidates_penalized": diagnostics.unmodeled_item_effect_candidates_penalized,
            "unique_scored_candidates": diagnostics.unique_scored_candidates,
            "time_budget_seconds": diagnostics.time_budget_seconds,
            "popcorn_window_seconds": diagnostics.popcorn_window_seconds,
            "popcorn_min_relative_improvement_percent": diagnostics.popcorn_min_relative_improvement_percent,
            "significant_improvement_events": diagnostics.significant_improvement_events,
            "best_significant_score": diagnostics.best_significant_score,
            "seconds_since_last_significant_improvement": diagnostics.seconds_since_last_significant_improvement,
            "search_type_breakdown": diagnostics.search_type_breakdown.iter().map(|breakdown| {
                json!({
                    "name": breakdown.name.clone(),
                    "score_requests": breakdown.score_requests,
                    "new_simulations": breakdown.new_simulations
                })
            }).collect::<Vec<_>>(),
            "estimated_total_candidate_space": diagnostics.estimated_total_candidate_space,
            "estimated_run_space_coverage_percent": diagnostics.estimated_run_space_coverage_percent,
            "estimated_close_to_optimal_probability": diagnostics.estimated_close_to_optimal_probability,
            "estimated_close_to_optimal_probability_note": diagnostics.estimated_close_to_optimal_probability_note,
            "coverage_stage_enabled": diagnostics.coverage_stage_enabled,
            "coverage_stage_elapsed_seconds": diagnostics.coverage_stage_elapsed_seconds,
            "coverage_stage_assets_total": diagnostics.coverage_stage_assets_total,
            "coverage_stage_assets_covered": diagnostics.coverage_stage_assets_covered,
            "coverage_stage_seed_candidates": diagnostics.coverage_stage_seed_candidates,
            "coverage_stage_seed_candidates_unique": diagnostics.coverage_stage_seed_candidates_unique,
            "coverage_stage_incomplete": diagnostics.coverage_stage_incomplete,
            "coverage_stage_warning": diagnostics.coverage_stage_warning,
            "elapsed_seconds": diagnostics.elapsed_seconds,
            "total_run_seconds": diagnostics.total_run_seconds,
            "timed_out": diagnostics.timed_out,
            "processed_candidates": diagnostics.processed_candidates,
            "total_candidates": diagnostics.total_candidates,
            "simulation_counts": {
                "new_full_simulations": diagnostics.full_evaluations,
                "unique_scored_candidates": diagnostics.unique_scored_candidates,
                "total_score_requests": total_score_requests
            }
        }
    });
    fs::write(report_path, serde_json::to_string_pretty(&json_value)?)
        .with_context(|| format!("Failed writing JSON report {}", report_path.display()))?;
    Ok(())
}

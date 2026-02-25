use super::controlled_champion_result_build_analysis::ControlledChampionBuildAnalysisOutput;
use super::*;
use serde_json::{Value, json};

fn deterministic_signature_json(signature: SimulationDeterminismSignature) -> Value {
    json!({
        "final_state_checksum_hex": format!("{:016x}", signature.final_state_checksum),
        "tick_state_checksum_hex": format!("{:016x}", signature.tick_state_checksum),
        "queue_checksum_hex": format!("{:016x}", signature.queue_checksum),
        "ticks_executed": signature.ticks_executed,
        "events_processed": signature.events_processed,
    })
}

pub(super) struct ControlledChampionResultArtifactWritingContext<'a> {
    pub(super) scenario_path: &'a Path,
    pub(super) controlled_champion_name: &'a str,
    pub(super) search_quality_profile: SearchQualityProfile,
    pub(super) max_runtime_seconds: Option<f64>,
    pub(super) popcorn_window_seconds: Option<f64>,
    pub(super) popcorn_min_relative_improvement_percent: f64,
    pub(super) report_path_override: Option<&'a str>,
    pub(super) controlled_champion_base: &'a ChampionBase,
    pub(super) controlled_champion_best_build: &'a [Item],
    pub(super) controlled_champion_best_score: f64,
    pub(super) controlled_champion_best_outcome: CombatOutcome,
    pub(super) controlled_champion_loadout: &'a ResolvedLoadout,
    pub(super) controlled_champion_runtime_loadout_selection: &'a LoadoutSelection,
    pub(super) controlled_champion_stack_overrides: &'a HashMap<String, f64>,
    pub(super) best_score_breakdown: ObjectiveScoreBreakdown,
    pub(super) enemy_builds: &'a [EnemyBuildEntry],
    pub(super) enemy_loadout: &'a ResolvedLoadout,
    pub(super) enemy_derived_combat_stats: &'a [EnemyDerivedCombatStats],
    pub(super) enemy_similarity_notes: &'a [String],
    pub(super) enemy_presets_used: &'a HashMap<String, EnemyUrfPreset>,
    pub(super) sim: &'a SimulationConfig,
    pub(super) urf: &'a UrfBuffs,
    pub(super) run_start: Instant,
    pub(super) status: &'a mut StatusReporter,
    pub(super) processed_candidates: usize,
    pub(super) total_candidates: usize,
}

pub(super) fn write_controlled_champion_result_artifacts(
    context: ControlledChampionResultArtifactWritingContext<'_>,
    analysis: &mut ControlledChampionBuildAnalysisOutput,
) -> Result<()> {
    let ControlledChampionResultArtifactWritingContext {
        scenario_path,
        controlled_champion_name,
        search_quality_profile,
        max_runtime_seconds,
        popcorn_window_seconds,
        popcorn_min_relative_improvement_percent,
        report_path_override,
        controlled_champion_base,
        controlled_champion_best_build,
        controlled_champion_best_score,
        controlled_champion_best_outcome,
        controlled_champion_loadout,
        controlled_champion_runtime_loadout_selection,
        controlled_champion_stack_overrides,
        best_score_breakdown,
        enemy_builds,
        enemy_loadout,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
        enemy_presets_used,
        sim,
        urf,
        run_start,
        status,
        processed_candidates,
        total_candidates,
    } = context;

    println!("\nTop diverse builds:");
    if analysis.diverse_top_builds.is_empty() {
        println!(
            "- None found (try increasing --max-relative-gap-percent or lowering --min-item-diff)."
        );
    } else {
        for (idx, (build, score)) in analysis.diverse_top_builds.iter().enumerate() {
            println!(
                "- #{:02} score {:.4}: {}",
                idx + 1,
                score,
                item_names(build)
            );
        }
    }

    if !analysis.build_order_results.is_empty() {
        println!("\nBuild order optimization (levels spread from 5 to 20):");
        for (idx, build_order_result) in analysis.build_order_results.iter().enumerate() {
            println!(
                "- Build #{:02} best order (cumulative {:.2}): {}",
                idx + 1,
                build_order_result.cumulative_score,
                item_names(&build_order_result.ordered_items)
            );
            for (stage_idx, level) in build_order_result.levels.iter().enumerate() {
                let stage_survival = build_order_result
                    .stage_survival
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                let stage_damage = build_order_result
                    .stage_damage
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                let stage_healing = build_order_result
                    .stage_healing
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                let stage_objective = build_order_result
                    .stage_objective_scores
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                println!(
                    "  - Stage {} @ level {} -> objective {:.3} | time {:.2}s | damage {:.1} | healing {:.1}",
                    stage_idx + 1,
                    level,
                    stage_objective,
                    stage_survival,
                    stage_damage,
                    stage_healing
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
            to_norm_key(controlled_champion_name)
        ))
    });
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let trace_markdown_path = report_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!(
            "{}_event_trace.md",
            to_norm_key(controlled_champion_name)
        ));
    let trace_json_path = trace_markdown_path.with_extension("json");
    let best_order_acquired_map = analysis
        .build_order_results
        .first()
        .map(|result| acquisition_level_map(&result.ordered_items, &result.acquired_levels));

    let mut best_trace_sim_cfg = sim.clone();
    best_trace_sim_cfg.collect_rune_proc_telemetry = true;
    let mut best_trace_sim =
        ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
            controlled_champion_base.clone(),
            controlled_champion_best_build,
            &controlled_champion_loadout.bonus_stats,
            Some(controlled_champion_runtime_loadout_selection),
            best_order_acquired_map.as_ref(),
            Some(controlled_champion_stack_overrides),
            enemy_builds,
            best_trace_sim_cfg,
            urf.clone(),
        );
    best_trace_sim.enable_trace();
    while best_trace_sim.step(1) {}
    let best_trace = best_trace_sim.trace_events().to_vec();
    let best_rune_proc_telemetry = best_trace_sim.controlled_champion_rune_proc_telemetry();
    let best_trace_determinism = best_trace_sim.deterministic_replay_signature();

    let mut trace_markdown = String::new();
    trace_markdown.push_str(&format!("# {} Event Trace\n\n", controlled_champion_name));
    trace_markdown.push_str("## Rune Proc Telemetry\n");
    if best_rune_proc_telemetry.is_empty() {
        trace_markdown.push_str("- none\n\n");
    } else {
        append_rune_proc_telemetry_markdown_entries(
            &mut trace_markdown,
            "- ",
            "  ",
            &best_rune_proc_telemetry,
            controlled_champion_best_outcome.damage_dealt,
            controlled_champion_best_outcome.healing_done,
        );
        trace_markdown.push('\n');
    }
    trace_markdown.push_str("## Optimized Build Trace\n");
    for line in &best_trace {
        if let Some((header, details)) = line.split_once('\n') {
            trace_markdown.push_str("- ");
            trace_markdown.push_str(header);
            trace_markdown.push('\n');
            trace_markdown.push_str("  ```text\n");
            trace_markdown.push_str(details);
            if !details.ends_with('\n') {
                trace_markdown.push('\n');
            }
            trace_markdown.push_str("  ```\n");
        } else {
            trace_markdown.push_str("- ");
            trace_markdown.push_str(line);
            trace_markdown.push('\n');
        }
    }
    fs::write(&trace_markdown_path, trace_markdown)?;

    let trace_json = json!({
        "schema_version": CONTROLLED_CHAMPION_TRACE_JSON_SCHEMA_VERSION,
        "event_encoding": "structured",
        "determinism": deterministic_signature_json(best_trace_determinism),
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

    analysis.diagnostics.total_run_seconds = run_start.elapsed().as_secs_f64();
    let report_data = ControlledChampionReportData {
        scenario_path,
        controlled_champion_name,
        sim,
        controlled_champion_base_level: controlled_champion_base,
        controlled_champion_end_stats: &analysis.controlled_champion_end_stats,
        stack_notes: &analysis.stack_notes,
        controlled_champion_loadout,
        controlled_champion_loadout_selection: controlled_champion_runtime_loadout_selection,
        enemy_loadout,
        best_build: controlled_champion_best_build,
        best_score: controlled_champion_best_score,
        best_outcome: &controlled_champion_best_outcome,
        best_rune_proc_telemetry: &best_rune_proc_telemetry,
        best_trace_determinism,
        best_score_breakdown,
        enemy_builds,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
        enemy_presets_used,
        diverse_top_builds: &analysis.diverse_top_builds,
        diverse_top_keys: &analysis.diverse_top_keys,
        build_confidence: &analysis.build_confidence,
        metrics_by_key: &analysis.metrics_by_key,
        pareto_front: &analysis.pareto_front,
        diagnostics: &analysis.diagnostics,
        build_orders: &analysis.build_order_results,
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

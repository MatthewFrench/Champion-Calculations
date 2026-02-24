use super::*;
mod header_and_objective_sections;
mod loadout_and_build_sections;
mod search_diagnostics_section;

use self::header_and_objective_sections::{
    HeaderAndObjectiveSectionInput, append_report_header_and_objective_sections,
    append_rune_proc_telemetry_section,
};
use self::loadout_and_build_sections::{
    append_base_stats_section, append_best_build_section, append_build_order_optimization_section,
    append_deeper_insights_section, append_diverse_top_builds_section, append_end_stats_section,
    append_enemy_builds_section, append_enemy_derived_combat_profiles_section,
    append_loadout_selection_and_effect_sections, append_stack_overrides_section,
};
use self::search_diagnostics_section::append_search_diagnostics_section;

pub(crate) fn write_controlled_champion_report_markdown(
    report_path: &Path,
    data: &ControlledChampionReportData<'_>,
) -> Result<()> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let scenario_path = data.scenario_path;
    let scenario_path_display = format_repo_relative_path(scenario_path);
    let controlled_champion_name = data.controlled_champion_name;
    let sim = data.sim;
    let controlled_champion_base_level = data.controlled_champion_base_level;
    let controlled_champion_end_stats = data.controlled_champion_end_stats;
    let stack_notes = data.stack_notes;
    let controlled_champion_loadout = data.controlled_champion_loadout;
    let enemy_loadout = data.enemy_loadout;
    let best_build = data.best_build;
    let best_score = data.best_score;
    let best_outcome = data.best_outcome;
    let best_rune_proc_telemetry = data.best_rune_proc_telemetry;
    let controlled_champion_unmodeled_item_effect_names =
        unmodeled_runtime_item_effect_names(best_build);
    let best_score_breakdown = data.best_score_breakdown;
    let enemy_builds = data.enemy_builds;
    let enemy_derived_combat_stats = data.enemy_derived_combat_stats;
    let enemy_similarity_notes = data.enemy_similarity_notes;
    let enemy_presets_used = data.enemy_presets_used;
    let diverse_top_builds = data.diverse_top_builds;
    let diverse_top_keys = data.diverse_top_keys;
    let build_confidence = data.build_confidence;
    let metrics_by_key = data.metrics_by_key;
    let pareto_front = data.pareto_front;
    let diagnostics = data.diagnostics;
    let build_orders = data.build_orders;

    validate_controlled_champion_selection_labels(
        controlled_champion_name,
        data.controlled_champion_loadout_selection,
        &controlled_champion_loadout.selection_labels,
        &controlled_champion_loadout.unmodeled_rune_names,
    )?;

    let now = SystemTime::now();
    let generated_utc: DateTime<Utc> = now.into();
    let generated_local: DateTime<Local> = DateTime::from(now);

    let mut content = String::new();
    append_report_header_and_objective_sections(
        &mut content,
        &HeaderAndObjectiveSectionInput {
            generated_local,
            generated_utc,
            scenario_path_display: &scenario_path_display,
            controlled_champion_name,
            best_score,
            best_outcome,
            champion_level: sim.champion_level,
            max_time_seconds: sim.max_time_seconds,
            best_score_breakdown,
        },
    );
    append_rune_proc_telemetry_section(&mut content, best_rune_proc_telemetry, best_outcome);
    append_search_diagnostics_section(&mut content, diagnostics);
    append_base_stats_section(
        &mut content,
        controlled_champion_name,
        controlled_champion_base_level,
    );
    append_loadout_selection_and_effect_sections(
        &mut content,
        controlled_champion_name,
        controlled_champion_loadout,
        enemy_loadout,
        &controlled_champion_unmodeled_item_effect_names,
    );
    append_best_build_section(&mut content, best_build);
    append_end_stats_section(
        &mut content,
        controlled_champion_name,
        controlled_champion_end_stats,
    );
    append_stack_overrides_section(&mut content, stack_notes);
    append_enemy_builds_section(&mut content, enemy_builds, enemy_presets_used);
    append_enemy_derived_combat_profiles_section(
        &mut content,
        enemy_derived_combat_stats,
        enemy_similarity_notes,
    );
    append_diverse_top_builds_section(
        &mut content,
        diverse_top_builds,
        diverse_top_keys,
        build_confidence,
        metrics_by_key,
        pareto_front,
        diagnostics,
    );
    append_build_order_optimization_section(&mut content, build_orders);
    append_deeper_insights_section(&mut content, diverse_top_builds);

    fs::write(report_path, content)
        .with_context(|| format!("Failed writing report {}", report_path.display()))?;
    Ok(())
}

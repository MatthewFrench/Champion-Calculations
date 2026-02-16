use anyhow::{Context, Result};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::search::item_names;

use super::{VladimirReportData, mean_std, simulation_dir, to_norm_key};

pub(super) fn default_report_path() -> PathBuf {
    simulation_dir()
        .join("output")
        .join("vladimir_run_report.md")
}

pub(super) fn write_vladimir_report_markdown(
    report_path: &Path,
    data: &VladimirReportData<'_>,
) -> Result<()> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let scenario_path = data.scenario_path;
    let sim = data.sim;
    let vlad_base_level = data.vlad_base_level;
    let vlad_end_stats = data.vlad_end_stats;
    let stack_notes = data.stack_notes;
    let vlad_loadout = data.vladimir_loadout;
    let enemy_loadout = data.enemy_loadout;
    let baseline_build = data.baseline_build;
    let baseline_score = data.baseline_score;
    let baseline_outcome = data.baseline_outcome;
    let best_build = data.best_build;
    let best_score = data.best_score;
    let best_outcome = data.best_outcome;
    let enemy_builds = data.enemy_builds;
    let enemy_presets_used = data.enemy_presets_used;
    let diverse_top_builds = data.diverse_top_builds;
    let diverse_top_keys = data.diverse_top_keys;
    let build_confidence = data.build_confidence;
    let metrics_by_key = data.metrics_by_key;
    let pareto_front = data.pareto_front;
    let diagnostics = data.diagnostics;
    let build_orders = data.build_orders;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let improvement = if baseline_score.abs() > f64::EPSILON {
        ((best_score - baseline_score) / baseline_score) * 100.0
    } else {
        0.0
    };

    let mut content = String::new();
    content.push_str("# Vladimir URF Run Report\n\n");
    content.push_str(&format!("- Generated (unix): `{}`\n", now));
    content.push_str(&format!("- Scenario: `{}`\n\n", scenario_path.display()));

    content.push_str("## Headline\n");
    content.push_str(&format!(
        "- Baseline objective score: **{:.4}**\n- Best objective score: **{:.4}**\n- Improvement: **{:+.2}%**\n- Baseline time alive / damage dealt / healing done / enemy kills: **{:.2}s / {:.1} / {:.1} / {}**\n- Best time alive / damage dealt / healing done / enemy kills: **{:.2}s / {:.1} / {:.1} / {}**\n- Baseline cap survivor: **{}**\n- Best cap survivor: **{}**\n\n",
        baseline_score,
        best_score,
        improvement,
        baseline_outcome.time_alive_seconds,
        baseline_outcome.damage_dealt,
        baseline_outcome.healing_done,
        baseline_outcome.enemy_kills,
        best_outcome.time_alive_seconds,
        best_outcome.damage_dealt,
        best_outcome.healing_done,
        best_outcome.enemy_kills,
        baseline_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
        best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
    ));

    content.push_str(&format!(
        "- Champion level assumption: **{}**\n\n",
        sim.champion_level
    ));

    let (seed_mean, seed_std) = mean_std(&diagnostics.seed_best_scores);
    content.push_str("## Search Diagnostics\n");
    content.push_str(&format!(
        "- Strategy: `{}`\n- Search quality profile: `{}`\n- Enemy scenarios: `{}`\n- Loadout candidates/finalists: `{}/{}`\n- Ensemble seeds: `{}`\n- Objective weights (survival/damage/healing): `{:.2}/{:.2}/{:.2}`\n- Full evaluations: `{}` (cache hits/misses/waits: `{}/{}/{}`)\n- Full persistent cache hits/entries: `{}/{}`\n- Unique candidate builds: `{}`\n- Bleed candidates injected: `{}`\n- Adaptive candidates injected: `{}`\n- Seed-best mean/stddev: `{:.2}` / `{:.3}`\n\n",
        diagnostics.strategy_summary,
        diagnostics.search_quality_profile,
        diagnostics.scenario_count,
        diagnostics.loadout_candidates,
        diagnostics.loadout_finalists,
        diagnostics.ensemble_seeds,
        diagnostics.objective_survival_weight,
        diagnostics.objective_damage_weight,
        diagnostics.objective_healing_weight,
        diagnostics.full_evaluations,
        diagnostics.full_cache_hits,
        diagnostics.full_cache_misses,
        diagnostics.full_cache_waits,
        diagnostics.full_persistent_cache_hits,
        diagnostics.full_persistent_cache_entries,
        diagnostics.unique_candidate_builds,
        diagnostics.bleed_candidates_injected,
        diagnostics.adaptive_candidates_injected,
        seed_mean,
        seed_std
    ));
    if let Some(budget) = diagnostics.time_budget_seconds {
        content.push_str(&format!(
            "- Time budget: `{:.1}s`; elapsed: `{:.1}s`; timed_out: `{}`; progress: `{}/{}`\n\n",
            budget,
            diagnostics.elapsed_seconds,
            diagnostics.timed_out,
            diagnostics.processed_candidates,
            diagnostics.total_candidates
        ));
    } else {
        content.push_str(&format!(
            "- Elapsed: `{:.1}s`; progress: `{}/{}`\n\n",
            diagnostics.elapsed_seconds,
            diagnostics.processed_candidates,
            diagnostics.total_candidates
        ));
    }

    content.push_str("## Vladimir Base Stats At Level\n");
    content.push_str(&format!(
        "- HP: {:.1}, Armor: {:.1}, MR: {:.1}, AD: {:.1}, AS: {:.3}, MS: {:.1}\n\n",
        vlad_base_level.base_health,
        vlad_base_level.base_armor,
        vlad_base_level.base_magic_resist,
        vlad_base_level.base_attack_damage,
        vlad_base_level.base_attack_speed,
        vlad_base_level.base_move_speed
    ));

    content.push_str("## Selected Runes/Masteries\n");
    if vlad_loadout.selection_labels.is_empty() {
        content.push_str("- Vladimir: none selected.\n");
    } else {
        content.push_str("- Vladimir:\n");
        for s in &vlad_loadout.selection_labels {
            content.push_str(&format!("  - {}\n", s));
        }
    }
    if enemy_loadout.selection_labels.is_empty() {
        content.push_str("- Enemies: none selected.\n\n");
    } else {
        content.push_str("- Enemies (applied to all):\n");
        for s in &enemy_loadout.selection_labels {
            content.push_str(&format!("  - {}\n", s));
        }
        content.push('\n');
    }
    if !vlad_loadout.applied_notes.is_empty() || !enemy_loadout.applied_notes.is_empty() {
        content.push_str("- Applied deterministic loadout effects:\n");
        for note in &vlad_loadout.applied_notes {
            content.push_str(&format!("  - Vladimir: {}\n", note));
        }
        for note in &enemy_loadout.applied_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    if !vlad_loadout.skipped_notes.is_empty() || !enemy_loadout.skipped_notes.is_empty() {
        content.push_str("- Skipped unsupported/non-deterministic effects:\n");
        for note in &vlad_loadout.skipped_notes {
            content.push_str(&format!("  - Vladimir: {}\n", note));
        }
        for note in &enemy_loadout.skipped_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    content.push('\n');

    content.push_str("## Baseline Build\n");
    content.push_str(&format!("- {}\n\n", item_names(baseline_build)));

    content.push_str("## Best Build\n");
    content.push_str(&format!("- {}\n\n", item_names(best_build)));

    content.push_str("## Vladimir End Stats (Best Build)\n");
    content.push_str(&format!(
        "- HP: {:.1}, Armor: {:.1}, MR: {:.1}, AP: {:.1}, AD: {:.1}, Ability Haste: {:.1}, Move Speed (flat bonus): {:.1}, Move Speed (% bonus): {:.1}\n\n",
        vlad_end_stats.health,
        vlad_end_stats.armor,
        vlad_end_stats.magic_resist,
        vlad_end_stats.ability_power,
        vlad_end_stats.attack_damage,
        vlad_end_stats.ability_haste,
        vlad_end_stats.move_speed_flat,
        vlad_end_stats.move_speed_percent
    ));

    content.push_str("## Stack Assumptions\n");
    if stack_notes.is_empty() {
        content.push_str(
            "- No explicit stack assumptions triggered for selected best build items.\n\n",
        );
    } else {
        for note in stack_notes {
            content.push_str(&format!("- {}\n", note));
        }
        content.push('\n');
    }

    content.push_str("## Enemy Builds (URF Presets)\n");
    for (enemy, build, _) in enemy_builds {
        content.push_str(&format!("- {}: {}\n", enemy.name, item_names(build)));
        if let Some(preset) = enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            content.push_str(&format!(
                "  - Source: {} (last checked {})\n",
                preset.source_url, preset.last_checked
            ));
            content.push_str(&format!("  - Runes: {}\n", preset.runes.join(", ")));
            content.push_str(&format!(
                "  - Masteries: {}\n",
                preset
                    .masteries
                    .iter()
                    .map(|m| format!("{} ({})", m.name, m.rank))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    content.push('\n');

    content.push_str("## Diverse Top Builds\n");
    if diverse_top_builds.is_empty() {
        content.push_str("- No diverse builds found under current thresholds.\n\n");
    } else {
        let best = diverse_top_builds[0].1;
        for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
            let delta = score - best;
            let key = diverse_top_keys.get(idx);
            let confidence = key
                .and_then(|k| build_confidence.iter().find(|c| c.key == *k))
                .map(|c| {
                    format!(
                        " | seed hits: {}/{} ({:.0}%) {}",
                        c.seed_hits,
                        diagnostics.ensemble_seeds,
                        c.seed_hit_rate * 100.0,
                        c.robustness
                    )
                })
                .unwrap_or_default();
            let pareto = key.map(|k| pareto_front.contains(k)).unwrap_or(false);
            let pareto_tag = if pareto { " | Pareto-front" } else { "" };
            content.push_str(&format!(
                "{}. `score {:.4}` ({:+.4} vs top): {}{}{}\n",
                idx + 1,
                score,
                delta,
                item_names(build),
                confidence,
                pareto_tag
            ));
            if let Some(k) = key
                && let Some(m) = metrics_by_key.get(k)
            {
                content.push_str(&format!(
                    "   - metrics: EHP~{:.1}, AP~{:.1}, timing score {:+.2}, total cost {:.0}\n",
                    m.ehp_mixed, m.ap, m.cost_timing, m.total_cost
                ));
            }
        }
        content.push('\n');
    }

    content.push_str("## Build Order Optimization\n");
    if build_orders.is_empty() {
        content.push_str("- No build-order optimization results available.\n\n");
    } else {
        for (idx, br) in build_orders.iter().enumerate() {
            content.push_str(&format!(
                "{}. Cumulative score: `{:.2}` | Order: {}\n",
                idx + 1,
                br.cumulative_score,
                item_names(&br.ordered_items)
            ));
            for (stage_idx, lvl) in br.levels.iter().enumerate() {
                let surv = br.stage_survival.get(stage_idx).copied().unwrap_or(0.0);
                let dmg = br.stage_damage.get(stage_idx).copied().unwrap_or(0.0);
                let heal = br.stage_healing.get(stage_idx).copied().unwrap_or(0.0);
                let stage_objective = br
                    .stage_objective_scores
                    .get(stage_idx)
                    .copied()
                    .unwrap_or(0.0);
                content.push_str(&format!(
                    "   - Stage {} (level {}): objective `{:.3}`, time alive `{:.2}s`, damage `{:.1}`, healing `{:.1}`\n",
                    stage_idx + 1,
                    lvl,
                    stage_objective,
                    surv,
                    dmg,
                    heal
                ));
            }
        }
        content.push('\n');
    }

    content.push_str("## Deeper Insights\n");
    if !diverse_top_builds.is_empty() {
        let mut item_counts: HashMap<String, usize> = HashMap::new();
        for (build, _) in diverse_top_builds {
            for item in build {
                *item_counts.entry(item.name.clone()).or_insert(0) += 1;
            }
        }
        let mut counts = item_counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let core_items = counts
            .iter()
            .filter(|(_, c)| *c == diverse_top_builds.len())
            .map(|(n, _)| n.clone())
            .collect::<Vec<_>>();
        let top_freq = counts
            .iter()
            .take(8)
            .map(|(n, c)| format!("{} ({}/{})", n, c, diverse_top_builds.len()))
            .collect::<Vec<_>>();

        if core_items.is_empty() {
            content.push_str("- No single item appears in every selected diverse top build.\n");
        } else {
            content.push_str(&format!(
                "- Common core across all selected top builds: {}.\n",
                core_items.join(", ")
            ));
        }
        content.push_str(&format!(
            "- Most frequent items in selected top set: {}.\n",
            top_freq.join(", ")
        ));
        content.push_str("- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.\n");
    } else {
        content.push_str("- Broaden thresholds (`--max-relative-gap-percent`) or lower diversity constraint (`--min-item-diff`) to surface more alternatives.\n");
    }

    fs::write(report_path, content)
        .with_context(|| format!("Failed writing report {}", report_path.display()))?;
    Ok(())
}

pub(super) fn write_vladimir_report_json(
    report_path: &Path,
    data: &VladimirReportData<'_>,
) -> Result<()> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let scenario_path = data.scenario_path;
    let sim = data.sim;
    let baseline_build = data.baseline_build;
    let baseline_score = data.baseline_score;
    let baseline_outcome = data.baseline_outcome;
    let best_build = data.best_build;
    let best_score = data.best_score;
    let best_outcome = data.best_outcome;
    let vladimir_loadout = data.vladimir_loadout;
    let enemy_builds = data.enemy_builds;
    let enemy_presets_used = data.enemy_presets_used;
    let diverse_top_builds = data.diverse_top_builds;
    let diagnostics = data.diagnostics;
    let build_orders = data.build_orders;

    let improvement_percent = if baseline_score.abs() > f64::EPSILON {
        ((best_score - baseline_score) / baseline_score) * 100.0
    } else {
        0.0
    };
    let json_value = json!({
        "scenario_path": scenario_path.display().to_string(),
        "champion_level": sim.champion_level,
        "headline": {
            "baseline_objective_score": baseline_score,
            "best_objective_score": best_score,
            "improvement_percent": improvement_percent,
            "baseline_outcome": {
                "time_alive_seconds": baseline_outcome.time_alive_seconds,
                "damage_dealt": baseline_outcome.damage_dealt,
                "healing_done": baseline_outcome.healing_done,
                "enemy_kills": baseline_outcome.enemy_kills,
                "cap_survivor": baseline_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
            },
            "best_outcome": {
                "time_alive_seconds": best_outcome.time_alive_seconds,
                "damage_dealt": best_outcome.damage_dealt,
                "healing_done": best_outcome.healing_done,
                "enemy_kills": best_outcome.enemy_kills,
                "cap_survivor": best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
            },
        },
        "baseline_build": baseline_build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
        "best_build": best_build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
        "vladimir_loadout_labels": vladimir_loadout.selection_labels,
        "enemy_presets": enemy_builds.iter().map(|(enemy, build, _)| {
            let key = to_norm_key(&enemy.name);
            let preset = enemy_presets_used.get(&key);
            json!({
                "champion": enemy.name,
                "items": build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                "runes": preset.map(|p| p.runes.clone()).unwrap_or_default(),
                "shards": preset.map(|p| p.shards.clone()).unwrap_or_default(),
                "masteries": preset.map(|p| p.masteries.iter().map(|m| json!({"name": m.name, "rank": m.rank})).collect::<Vec<_>>()).unwrap_or_default(),
                "source_url": preset.map(|p| p.source_url.clone()).unwrap_or_default(),
                "last_checked": preset.map(|p| p.last_checked.clone()).unwrap_or_default(),
            })
        }).collect::<Vec<_>>(),
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
            "ensemble_seeds": diagnostics.ensemble_seeds,
            "objective_survival_weight": diagnostics.objective_survival_weight,
            "objective_damage_weight": diagnostics.objective_damage_weight,
            "objective_healing_weight": diagnostics.objective_healing_weight,
            "full_evaluations": diagnostics.full_evaluations,
            "full_cache_hits": diagnostics.full_cache_hits,
            "full_cache_misses": diagnostics.full_cache_misses,
            "full_cache_waits": diagnostics.full_cache_waits,
            "full_persistent_cache_hits": diagnostics.full_persistent_cache_hits,
            "full_persistent_cache_entries": diagnostics.full_persistent_cache_entries,
            "unique_candidate_builds": diagnostics.unique_candidate_builds,
            "bleed_candidates_injected": diagnostics.bleed_candidates_injected,
            "adaptive_candidates_injected": diagnostics.adaptive_candidates_injected,
            "scenario_count": diagnostics.scenario_count,
            "loadout_candidates": diagnostics.loadout_candidates,
            "loadout_finalists": diagnostics.loadout_finalists,
            "time_budget_seconds": diagnostics.time_budget_seconds,
            "elapsed_seconds": diagnostics.elapsed_seconds,
            "timed_out": diagnostics.timed_out,
            "processed_candidates": diagnostics.processed_candidates,
            "total_candidates": diagnostics.total_candidates
        }
    });
    fs::write(report_path, serde_json::to_string_pretty(&json_value)?)
        .with_context(|| format!("Failed writing JSON report {}", report_path.display()))?;
    Ok(())
}

use super::*;
use std::collections::{HashMap, HashSet};

use crate::{BuildConfidence, BuildKey, BuildMetrics, BuildOrderResult, Item, SearchDiagnostics};

pub(crate) fn append_diverse_top_builds_section(
    content: &mut String,
    diverse_top_builds: &[(Vec<Item>, f64)],
    diverse_top_keys: &[BuildKey],
    build_confidence: &[BuildConfidence],
    metrics_by_key: &HashMap<BuildKey, BuildMetrics>,
    pareto_front: &HashSet<BuildKey>,
    diagnostics: &SearchDiagnostics,
) {
    content.push_str("## Diverse Top Builds\n");
    if diverse_top_builds.is_empty() {
        content.push_str("- No diverse builds found under current thresholds.\n\n");
        return;
    }
    let best_score = diverse_top_builds[0].1;
    for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
        let delta = score - best_score;
        let key = diverse_top_keys.get(idx);
        let confidence = key
            .and_then(|build_key| {
                build_confidence
                    .iter()
                    .find(|entry| entry.key == *build_key)
            })
            .map(|entry| {
                format!(
                    " | seed hits: {}/{} ({:.0}%) {}",
                    entry.seed_hits,
                    diagnostics.ensemble_seeds,
                    entry.seed_hit_rate * 100.0,
                    entry.robustness
                )
            })
            .unwrap_or_default();
        let pareto = key
            .map(|build_key| pareto_front.contains(build_key))
            .unwrap_or(false);
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
        if let Some(build_key) = key
            && let Some(metrics) = metrics_by_key.get(build_key)
        {
            content.push_str(&format!(
                "   - metrics: EHP~{}, AP~{}, timing score {:+.2}, total cost {}\n",
                format_f64_with_commas(metrics.ehp_mixed, 1),
                format_f64_with_commas(metrics.ap, 1),
                metrics.cost_timing,
                format_f64_with_commas(metrics.total_cost, 0)
            ));
        }
    }
    content.push('\n');
}

pub(crate) fn append_build_order_optimization_section(
    content: &mut String,
    build_orders: &[BuildOrderResult],
) {
    content.push_str("## Build Order Optimization\n");
    if build_orders.is_empty() {
        content.push_str("- No build-order optimization results available.\n\n");
        return;
    }
    for (idx, build_order) in build_orders.iter().enumerate() {
        content.push_str(&format!(
            "{}. Cumulative score: `{:.2}` | Order: {}\n",
            idx + 1,
            build_order.cumulative_score,
            item_names(&build_order.ordered_items)
        ));
        for (stage_idx, level) in build_order.levels.iter().enumerate() {
            let stage_survival = build_order
                .stage_survival
                .get(stage_idx)
                .copied()
                .unwrap_or(0.0);
            let stage_damage = build_order
                .stage_damage
                .get(stage_idx)
                .copied()
                .unwrap_or(0.0);
            let stage_healing = build_order
                .stage_healing
                .get(stage_idx)
                .copied()
                .unwrap_or(0.0);
            let stage_objective_score = build_order
                .stage_objective_scores
                .get(stage_idx)
                .copied()
                .unwrap_or(0.0);
            content.push_str(&format!(
                "   - Stage {} (level {}): objective `{:.3}`, time alive `{:.2}s`, damage `{}`, healing `{}`\n",
                stage_idx + 1,
                format_usize_with_commas(*level),
                stage_objective_score,
                stage_survival,
                format_f64_with_commas(stage_damage, 1),
                format_f64_with_commas(stage_healing, 1)
            ));
        }
    }
    content.push('\n');
}

pub(crate) fn append_deeper_insights_section(
    content: &mut String,
    diverse_top_builds: &[(Vec<Item>, f64)],
) {
    content.push_str("## Deeper Insights\n");
    if diverse_top_builds.is_empty() {
        content.push_str("- Broaden thresholds (`--max-relative-gap-percent`) or lower diversity constraint (`--min-item-diff`) to surface more alternatives.\n");
        return;
    }
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
        .filter(|(_, count)| *count == diverse_top_builds.len())
        .map(|(name, _)| name.clone())
        .collect::<Vec<_>>();
    let top_frequency = counts
        .iter()
        .take(8)
        .map(|(name, count)| format!("{} ({}/{})", name, count, diverse_top_builds.len()))
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
        top_frequency.join(", ")
    ));
    content.push_str("- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.\n");
}

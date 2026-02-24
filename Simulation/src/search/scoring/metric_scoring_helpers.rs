use crate::{BuildMetrics, Item};

pub(super) fn effective_health_points_mixed(health: f64, armor: f64, magic_resist: f64) -> f64 {
    let physical_multiplier = 1.0 + armor.max(0.0) / 100.0;
    let magic_multiplier = 1.0 + magic_resist.max(0.0) / 100.0;
    health.max(1.0) * 0.5 * (physical_multiplier + magic_multiplier)
}

pub(super) fn build_cost_timing_score(build: &[Item]) -> f64 {
    if build.is_empty() {
        return 0.0;
    }
    let mut weighted = 0.0;
    let mut total = 0.0;
    for (idx, item) in build.iter().enumerate() {
        let weight = 1.0 / (1.0 + idx as f64);
        weighted += weight * item.total_cost.max(0.0);
        total += item.total_cost.max(0.0);
    }
    // Higher is better. Penalize expensive early spikes more.
    -weighted - 0.1 * total
}

pub(super) fn build_metrics_dominates(a: &BuildMetrics, b: &BuildMetrics) -> bool {
    let greater_or_equal = a.objective >= b.objective
        && a.ehp_mixed >= b.ehp_mixed
        && a.ap >= b.ap
        && a.cost_timing >= b.cost_timing;
    let strictly_greater = a.objective > b.objective
        || a.ehp_mixed > b.ehp_mixed
        || a.ap > b.ap
        || a.cost_timing > b.cost_timing;
    greater_or_equal && strictly_greater
}

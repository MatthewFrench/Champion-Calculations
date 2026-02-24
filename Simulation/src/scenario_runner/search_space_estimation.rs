use crate::data::LoadoutDomain;

use super::*;

fn n_choose_k(n: usize, k: usize) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    if k == 0 {
        return 1;
    }
    let mut result = 1u128;
    for i in 1..=k {
        let numerator = (n - k + i) as u128;
        let denominator = i as u128;
        result = (result * numerator) / denominator;
    }
    result
}

pub(super) fn estimated_legal_item_build_count(item_pool: &[Item], max_items: usize) -> f64 {
    if max_items == 0 {
        return 1.0;
    }
    let boots_count = item_pool.iter().filter(|item| is_boots(item)).count();
    let non_boots_count = item_pool.len().saturating_sub(boots_count);
    let max_boots = boots_count.min(1).min(max_items);
    let mut total = 0u128;
    for boots_used in 0..=max_boots {
        let non_boots_used = max_items.saturating_sub(boots_used);
        if non_boots_used > non_boots_count {
            continue;
        }
        total = total.saturating_add(
            n_choose_k(boots_count, boots_used)
                .saturating_mul(n_choose_k(non_boots_count, non_boots_used)),
        );
    }
    total as f64
}

pub(super) fn estimated_legal_loadout_count(loadout_domain: &LoadoutDomain) -> f64 {
    if loadout_domain.rune_paths.len() < 2 {
        return 0.0;
    }
    let shard_count = loadout_domain
        .shard_slots
        .iter()
        .map(|slot| slot.len() as u128)
        .product::<u128>();
    if shard_count == 0 {
        return 0.0;
    }
    let mut rune_pages = 0u128;
    for (primary_index, primary_path) in loadout_domain.rune_paths.iter().enumerate() {
        if primary_path.slot_runes.len() < 4 {
            continue;
        }
        let primary_count = primary_path.slot_runes[..4]
            .iter()
            .map(|slot| slot.len() as u128)
            .product::<u128>();
        if primary_count == 0 {
            continue;
        }
        for (secondary_index, secondary_path) in loadout_domain.rune_paths.iter().enumerate() {
            if secondary_index == primary_index || secondary_path.slot_runes.len() < 4 {
                continue;
            }
            let secondary_pair_count = [(1usize, 2usize), (1usize, 3usize), (2usize, 3usize)]
                .iter()
                .map(|(slot_a, slot_b)| {
                    (secondary_path.slot_runes[*slot_a].len() as u128)
                        .saturating_mul(secondary_path.slot_runes[*slot_b].len() as u128)
                })
                .sum::<u128>();
            rune_pages =
                rune_pages.saturating_add(primary_count.saturating_mul(secondary_pair_count));
        }
    }
    rune_pages.saturating_mul(shard_count) as f64
}

pub(super) fn estimate_close_to_optimal_probability(
    evaluated_candidates: usize,
    total_candidate_space: Option<f64>,
) -> (Option<f64>, String) {
    let Some(total) = total_candidate_space else {
        return (
            None,
            "Unavailable: total legal candidate space estimate was not finite.".to_string(),
        );
    };
    if !total.is_finite() || total <= 0.0 {
        return (
            None,
            "Unavailable: total legal candidate space estimate was not positive.".to_string(),
        );
    }
    let draws = evaluated_candidates as f64;
    if draws <= 0.0 {
        return (
            Some(0.0),
            "0.0%: no unique candidates were scored in this run.".to_string(),
        );
    }
    let conservative_top_quantile = 0.00000001_f64; // top 0.000001%
    let minimum_quantile = (1.0 / total).clamp(0.0, 1.0);
    let hit_rate = conservative_top_quantile
        .max(minimum_quantile)
        .clamp(0.0, 1.0);
    let probability = if hit_rate >= 1.0 {
        1.0
    } else {
        1.0 - (1.0 - hit_rate).powf(draws)
    };
    let implied_top_candidate_count = (hit_rate * total).max(1.0).round();
    let note = format!(
        "Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = {:.9}% (about top {:.0} candidates in the legal space) and n = {} unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.",
        hit_rate * 100.0,
        implied_top_candidate_count,
        evaluated_candidates
    );
    (Some(probability.clamp(0.0, 1.0)), note)
}

pub(super) fn format_percent_display(percent: f64) -> String {
    if !percent.is_finite() {
        return percent.to_string();
    }
    if percent > 0.0 && percent < 0.000001 {
        format!("{percent:.3e}%")
    } else {
        format!("{percent:.6}%")
    }
}

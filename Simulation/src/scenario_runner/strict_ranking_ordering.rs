use std::cmp::Ordering;
use std::collections::HashMap;

use super::*;

pub(super) fn heuristic_sort_remaining_candidates_for_strict_ranking(
    mut remaining_keys: Vec<BuildKey>,
    strict_scores: &HashMap<BuildKey, f64>,
    item_pool_len: usize,
    rune_signal_weight: f64,
    shard_signal_weight: f64,
    seed: u64,
    exploration_promotions: usize,
) -> (Vec<BuildKey>, usize) {
    if remaining_keys.len() <= 1 {
        return (remaining_keys, 0);
    }

    let finite_scores = strict_scores
        .values()
        .copied()
        .filter(|score| score.is_finite())
        .collect::<Vec<_>>();
    if finite_scores.len() > 1 {
        let min_score = finite_scores.iter().copied().fold(f64::INFINITY, f64::min);
        let max_score = finite_scores
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        let span = max_score - min_score;
        if span > 1e-9 {
            let mut item_signals = vec![0.0_f64; item_pool_len];
            let mut rune_signals = HashMap::<String, f64>::new();
            let mut shard_signals = HashMap::<String, f64>::new();
            for (candidate, score) in strict_scores {
                if !score.is_finite() {
                    continue;
                }
                let centered = ((*score - min_score) / span) - 0.5;
                for &item_idx in &candidate.item_indices {
                    if let Some(signal) = item_signals.get_mut(item_idx) {
                        *signal += centered;
                    }
                }
                for rune_name in &candidate.loadout_selection.rune_names {
                    *rune_signals.entry(to_norm_key(rune_name)).or_insert(0.0) += centered;
                }
                for (slot_idx, shard_stat) in
                    candidate.loadout_selection.shard_stats.iter().enumerate()
                {
                    let shard_key = format!("{}:{}", slot_idx, to_norm_key(shard_stat));
                    *shard_signals.entry(shard_key).or_insert(0.0) += centered;
                }
            }

            let mut ranked = remaining_keys
                .into_iter()
                .map(|candidate| {
                    let item_score = candidate
                        .item_indices
                        .iter()
                        .filter_map(|idx| item_signals.get(*idx).copied())
                        .sum::<f64>();
                    let rune_score = candidate
                        .loadout_selection
                        .rune_names
                        .iter()
                        .map(|rune_name| {
                            rune_signals
                                .get(&to_norm_key(rune_name))
                                .copied()
                                .unwrap_or(0.0)
                        })
                        .sum::<f64>();
                    let shard_score = candidate
                        .loadout_selection
                        .shard_stats
                        .iter()
                        .enumerate()
                        .map(|(slot_idx, shard_stat)| {
                            let shard_key = format!("{}:{}", slot_idx, to_norm_key(shard_stat));
                            shard_signals.get(&shard_key).copied().unwrap_or(0.0)
                        })
                        .sum::<f64>();
                    let heuristic_score = item_score
                        + rune_signal_weight * rune_score
                        + shard_signal_weight * shard_score;
                    (candidate, heuristic_score)
                })
                .collect::<Vec<_>>();
            ranked.sort_by(|(a_key, a_score), (b_key, b_score)| {
                b_score
                    .partial_cmp(a_score)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| build_key_cache_string(a_key).cmp(&build_key_cache_string(b_key)))
            });
            remaining_keys = ranked.into_iter().map(|(candidate, _)| candidate).collect();
        }
    }

    let mut promotions_done = 0usize;
    let mut promotion_seed = seed ^ 0x4f6d_13aa_a31b_2f17;
    for _ in 0..exploration_promotions {
        if remaining_keys.len() <= 1 {
            break;
        }
        let idx = rand_index(&mut promotion_seed, remaining_keys.len());
        if idx > 0 {
            remaining_keys.swap(0, idx);
            promotions_done += 1;
        }
    }

    (remaining_keys, promotions_done)
}

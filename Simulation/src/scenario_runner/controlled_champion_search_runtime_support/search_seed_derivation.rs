use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use super::*;

pub(in crate::scenario_runner) fn fixed_sweep_keystone_seed_base(
    seed_base: u64,
    keystone: &str,
) -> u64 {
    let mut keystone_seed_hasher = DefaultHasher::new();
    to_norm_key(keystone).hash(&mut keystone_seed_hasher);
    seed_base.wrapping_add(keystone_seed_hasher.finish())
}

pub(in crate::scenario_runner) fn fixed_sweep_repeat_seed(
    keystone_seed_base: u64,
    repeat_idx: usize,
) -> u64 {
    keystone_seed_base
        .wrapping_add((repeat_idx as u64).wrapping_mul(FIXED_SWEEP_REPEAT_SEED_STRIDE))
}

pub(in crate::scenario_runner) fn partial_candidate_completion_seed(
    search_seed: u64,
    seed_index: usize,
    ranked_index: usize,
    candidate: &BuildKey,
) -> u64 {
    let mut hasher = DefaultHasher::new();
    candidate.hash(&mut hasher);
    search_seed
        ^ hasher.finish()
        ^ ((seed_index as u64 + 1).wrapping_mul(0x9e37_79b9_7f4a_7c15))
        ^ ((ranked_index as u64 + 1).wrapping_mul(0xbf58_476d_1ce4_e5b9))
}

use super::*;
use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn blocking_score_cache_reuses_computed_value_for_duplicate_key() {
    let cache = BlockingScoreCache::new();
    let compute_count = AtomicUsize::new(0);

    let first = cache.get_or_compute("same_key".to_string(), || {
        compute_count.fetch_add(1, Ordering::Relaxed);
        42.0
    });
    let second = cache.get_or_compute("same_key".to_string(), || {
        compute_count.fetch_add(1, Ordering::Relaxed);
        7.0
    });

    assert_eq!(first, 42.0);
    assert_eq!(second, 42.0);
    assert_eq!(
        compute_count.load(Ordering::Relaxed),
        1,
        "duplicate cache key should only compute once"
    );
    assert_eq!(cache.misses(), 1);
    assert_eq!(cache.hits(), 1);
}

#[test]
fn blocking_score_cache_recovers_from_poisoned_mutex_state() {
    let cache = BlockingScoreCache::new();
    let shard_idx = cache.shard_idx("poisoned_key");

    let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
        let _guard = cache.shards[shard_idx]
            .states
            .lock()
            .expect("test should acquire cache shard lock before poisoning");
        panic!("intentional poison for recovery path test");
    }));

    let value = cache.get_or_compute("poisoned_key".to_string(), || 5.0);
    assert_eq!(value, 5.0);
}

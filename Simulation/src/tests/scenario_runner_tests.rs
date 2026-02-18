use super::*;

#[test]
fn cache_seed_partition_uses_shared_bucket_for_runtime_random_seed() {
    let partition = persistent_cache_seed_partition(0, None, 987_654_321);
    assert_eq!(partition, 0);
}

#[test]
fn cache_seed_partition_uses_effective_seed_for_configured_seed() {
    let partition = persistent_cache_seed_partition(42, None, 42);
    assert_eq!(partition, 42);
}

#[test]
fn cache_seed_partition_uses_effective_seed_for_cli_override() {
    let partition = persistent_cache_seed_partition(0, Some(123), 123);
    assert_eq!(partition, 123);
}

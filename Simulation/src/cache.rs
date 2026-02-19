use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
enum CacheState {
    InFlight,
    Ready(f64),
}

#[derive(Debug)]
pub(crate) struct BlockingScoreCache {
    shards: Vec<CacheShard>,
    hits: AtomicUsize,
    misses: AtomicUsize,
    waits: AtomicUsize,
}

#[derive(Debug)]
struct CacheShard {
    states: Mutex<HashMap<String, CacheState>>,
    cv: Condvar,
}

impl BlockingScoreCache {
    pub(crate) fn new() -> Self {
        let shard_count = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(8)
            .next_power_of_two()
            .max(8);
        let shards = (0..shard_count)
            .map(|_| CacheShard {
                states: Mutex::new(HashMap::new()),
                cv: Condvar::new(),
            })
            .collect::<Vec<_>>();
        Self {
            shards,
            hits: AtomicUsize::new(0),
            misses: AtomicUsize::new(0),
            waits: AtomicUsize::new(0),
        }
    }

    fn shard_idx(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) & (self.shards.len() - 1)
    }

    pub(crate) fn get_or_compute<F>(&self, key: String, compute: F) -> f64
    where
        F: FnOnce() -> f64,
    {
        let shard_idx = self.shard_idx(&key);
        let shard = &self.shards[shard_idx];
        loop {
            let mut guard = shard.states.lock().expect("cache mutex poisoned");
            match guard.get(&key) {
                Some(CacheState::Ready(v)) => {
                    self.hits.fetch_add(1, AtomicOrdering::Relaxed);
                    return *v;
                }
                Some(CacheState::InFlight) => {
                    self.waits.fetch_add(1, AtomicOrdering::Relaxed);
                    guard = shard.cv.wait(guard).expect("cache condvar wait poisoned");
                    drop(guard);
                    continue;
                }
                None => {
                    self.misses.fetch_add(1, AtomicOrdering::Relaxed);
                    guard.insert(key.clone(), CacheState::InFlight);
                    drop(guard);
                    let value = compute();
                    let mut done = shard.states.lock().expect("cache mutex poisoned");
                    done.insert(key.clone(), CacheState::Ready(value));
                    shard.cv.notify_all();
                    return value;
                }
            }
        }
    }

    pub(crate) fn hits(&self) -> usize {
        self.hits.load(AtomicOrdering::Relaxed)
    }

    pub(crate) fn misses(&self) -> usize {
        self.misses.load(AtomicOrdering::Relaxed)
    }

    pub(crate) fn waits(&self) -> usize {
        self.waits.load(AtomicOrdering::Relaxed)
    }
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::event_type_catalog::EventType;
use super::queued_event_record::QueuedEvent;
use super::queued_projectile_impact_projection::{
    QueuedProjectileImpactKind, QueuedProjectileImpactProjection,
};

pub(in crate::engine) struct EventQueueScheduler {
    heap: BinaryHeap<QueuedEvent>,
    counter: u64,
}

impl EventQueueScheduler {
    pub(in crate::engine) fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            counter: 0,
        }
    }

    pub(in crate::engine) fn enqueue_event(
        &mut self,
        now_seconds: f64,
        delay_seconds: f64,
        priority: i32,
        kind: EventType,
        recurring_interval_seconds: Option<f64>,
    ) {
        self.push_event(QueuedEvent {
            time: now_seconds + delay_seconds.max(0.0),
            priority,
            seq: 0,
            recurring: recurring_interval_seconds,
            kind,
        });
    }

    pub(in crate::engine) fn peek_next(&self) -> Option<&QueuedEvent> {
        self.heap.peek()
    }

    pub(in crate::engine) fn pop_next(&mut self) -> Option<QueuedEvent> {
        self.heap.pop()
    }

    pub(in crate::engine) fn reschedule_recurring_event(
        &mut self,
        event_time_seconds: f64,
        priority: i32,
        recurring_interval_seconds: Option<f64>,
        kind: EventType,
    ) {
        let Some(interval_seconds) = recurring_interval_seconds else {
            return;
        };
        if interval_seconds <= 0.0 {
            return;
        }
        self.push_event(QueuedEvent {
            time: event_time_seconds + interval_seconds,
            priority,
            seq: 0,
            recurring: recurring_interval_seconds,
            kind,
        });
    }

    pub(in crate::engine) fn next_enemy_attack_ready_at(&self, enemy_index: usize) -> Option<f64> {
        self.heap
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::Attack(event_idx) if *event_idx == enemy_index => Some(queued.time),
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    pub(in crate::engine) fn next_enemy_attack_impact_at(&self, enemy_index: usize) -> Option<f64> {
        self.heap
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::AttackHit { idx, .. } if *idx == enemy_index => Some(queued.time),
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    pub(in crate::engine) fn next_controlled_champion_attack_ready_at(&self) -> Option<f64> {
        self.heap
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::ControlledChampionAttack => Some(queued.time),
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    pub(in crate::engine) fn next_controlled_champion_attack_impact_at(
        &self,
    ) -> Option<(usize, f64)> {
        self.heap
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::ControlledChampionAttackHit { idx, .. } => Some((*idx, queued.time)),
                _ => None,
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
    }

    pub(in crate::engine) fn queued_projectile_impact_projections(
        &self,
        now_seconds: f64,
    ) -> Vec<QueuedProjectileImpactProjection> {
        let mut entries = Vec::new();
        for queued in self.heap.iter() {
            if queued.time + 1e-9 < now_seconds {
                continue;
            }
            match &queued.kind {
                EventType::AttackHit { idx, .. } => {
                    entries.push(QueuedProjectileImpactProjection {
                        time_seconds: queued.time,
                        kind: QueuedProjectileImpactKind::EnemyAttack { enemy_index: *idx },
                    })
                }
                EventType::ControlledChampionOffensivePrimaryHit { idx, .. } => {
                    entries.push(QueuedProjectileImpactProjection {
                        time_seconds: queued.time,
                        kind: QueuedProjectileImpactKind::ControlledChampionOffensivePrimary {
                            enemy_index: *idx,
                        },
                    })
                }
                EventType::ControlledChampionAttackHit { idx, .. } => {
                    entries.push(QueuedProjectileImpactProjection {
                        time_seconds: queued.time,
                        kind: QueuedProjectileImpactKind::ControlledChampionAttack {
                            enemy_index: *idx,
                        },
                    })
                }
                _ => {}
            }
        }
        entries.sort_by(|a, b| {
            a.time_seconds
                .partial_cmp(&b.time_seconds)
                .unwrap_or(Ordering::Equal)
        });
        entries
    }

    pub(in crate::engine) fn queued_event_count(&self) -> usize {
        self.heap.len()
    }

    pub(in crate::engine) fn deterministic_replay_checksum(&self) -> u64 {
        fn mix(checksum: &mut u64, value: u64) {
            *checksum ^= value;
            *checksum = checksum.wrapping_mul(0x1000_0000_01B3);
            *checksum ^= *checksum >> 31;
        }

        fn mix_i32(checksum: &mut u64, value: i32) {
            mix(checksum, value as u32 as u64);
        }

        fn mix_f64(checksum: &mut u64, value: f64) {
            mix(checksum, value.to_bits());
        }

        let mut sorted = self.heap.iter().cloned().collect::<Vec<_>>();
        sorted.sort_by(|left, right| {
            left.time
                .partial_cmp(&right.time)
                .unwrap_or(Ordering::Equal)
                .then_with(|| left.priority.cmp(&right.priority))
                .then_with(|| left.seq.cmp(&right.seq))
        });

        let mut checksum = 0xcbf2_9ce4_8422_2325u64;
        mix(&mut checksum, self.counter);
        mix(&mut checksum, sorted.len() as u64);
        for queued in sorted {
            mix_f64(&mut checksum, queued.time);
            mix_i32(&mut checksum, queued.priority);
            mix(&mut checksum, queued.seq);
            match queued.recurring {
                Some(interval_seconds) => {
                    mix(&mut checksum, 1);
                    mix_f64(&mut checksum, interval_seconds);
                }
                None => mix(&mut checksum, 0),
            }
            mix(&mut checksum, queued.kind.deterministic_signature());
        }
        checksum
    }

    fn push_event(&mut self, mut queued_event: QueuedEvent) {
        self.counter = self.counter.wrapping_add(1);
        queued_event.seq = self.counter;
        self.heap.push(queued_event);
    }
}

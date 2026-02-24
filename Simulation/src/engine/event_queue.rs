pub(super) mod event_queue_ordering;
pub(super) mod event_queue_scheduler;
pub(super) mod event_type_catalog;
pub(super) mod queued_event_record;
pub(super) mod queued_projectile_impact_projection;

pub(super) use self::event_queue_scheduler::EventQueueScheduler;
pub(super) use self::event_type_catalog::EventType;
pub(super) use self::queued_event_record::QueuedEvent;
pub(super) use self::queued_projectile_impact_projection::QueuedProjectileImpactKind;

#[cfg(test)]
#[path = "event_queue/tests/event_queue_scheduler_tests.rs"]
mod tests;

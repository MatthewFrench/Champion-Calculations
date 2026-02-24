use super::event_type_catalog::EventType;

#[derive(Debug, Clone)]
pub(in crate::engine) struct QueuedEvent {
    pub(in crate::engine) time: f64,
    pub(in crate::engine) priority: i32,
    pub(in crate::engine) seq: u64,
    pub(in crate::engine) recurring: Option<f64>,
    pub(in crate::engine) kind: EventType,
}

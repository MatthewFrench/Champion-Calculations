#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::engine) enum QueuedProjectileImpactKind {
    EnemyAttack { enemy_index: usize },
    ControlledChampionOffensivePrimary { enemy_index: usize },
    ControlledChampionAttack { enemy_index: usize },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::engine) struct QueuedProjectileImpactProjection {
    pub(in crate::engine) time_seconds: f64,
    pub(in crate::engine) kind: QueuedProjectileImpactKind,
}

use crate::scripts::champions::ChampionScriptEvent;

use super::super::geometry::Vec2;

#[derive(Debug, Clone)]
pub(in crate::engine) enum EventType {
    Attack(usize),
    AttackWindup {
        idx: usize,
        token: u64,
    },
    AttackHit {
        idx: usize,
        token: u64,
        source: Vec2,
        target_at_release: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    ControlledChampionAttack,
    ControlledChampionAttackWindup {
        idx: usize,
        token: u64,
    },
    ControlledChampionAttackHit {
        idx: usize,
        token: u64,
        source: Vec2,
        target_at_release: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    ControlledChampionOffensivePrimaryHit {
        idx: usize,
        source: Vec2,
        target_at_cast: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    ControlledChampionOffensiveSecondaryHit,
    ControlledChampionOffensiveUltimateHit,
    ChampionScript(usize, ChampionScriptEvent, u64),
}

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

impl EventType {
    pub(in crate::engine) fn deterministic_signature(&self) -> u64 {
        fn mix(checksum: &mut u64, value: u64) {
            *checksum ^= value;
            *checksum = checksum.wrapping_mul(0x1000_0000_01B3);
            *checksum ^= *checksum >> 32;
        }

        fn mix_usize(checksum: &mut u64, value: usize) {
            mix(checksum, value as u64);
        }

        fn mix_f64(checksum: &mut u64, value: f64) {
            mix(checksum, value.to_bits());
        }

        fn mix_vec2(checksum: &mut u64, point: Vec2) {
            mix_f64(checksum, point.x);
            mix_f64(checksum, point.y);
        }

        let mut checksum = 0xcbf2_9ce4_8422_2325u64;
        match self {
            Self::Attack(idx) => {
                mix(&mut checksum, 1);
                mix_usize(&mut checksum, *idx);
            }
            Self::AttackWindup { idx, token } => {
                mix(&mut checksum, 2);
                mix_usize(&mut checksum, *idx);
                mix(&mut checksum, *token);
            }
            Self::AttackHit {
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                mix(&mut checksum, 3);
                mix_usize(&mut checksum, *idx);
                mix(&mut checksum, *token);
                mix_vec2(&mut checksum, *source);
                mix_vec2(&mut checksum, *target_at_release);
                mix_f64(&mut checksum, *projectile_speed);
                mix_f64(&mut checksum, *effect_hitbox_radius);
            }
            Self::ControlledChampionAttack => {
                mix(&mut checksum, 4);
            }
            Self::ControlledChampionAttackWindup { idx, token } => {
                mix(&mut checksum, 5);
                mix_usize(&mut checksum, *idx);
                mix(&mut checksum, *token);
            }
            Self::ControlledChampionAttackHit {
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                mix(&mut checksum, 6);
                mix_usize(&mut checksum, *idx);
                mix(&mut checksum, *token);
                mix_vec2(&mut checksum, *source);
                mix_vec2(&mut checksum, *target_at_release);
                mix_f64(&mut checksum, *projectile_speed);
                mix_f64(&mut checksum, *effect_hitbox_radius);
            }
            Self::ControlledChampionOffensivePrimaryHit {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                mix(&mut checksum, 7);
                mix_usize(&mut checksum, *idx);
                mix_vec2(&mut checksum, *source);
                mix_vec2(&mut checksum, *target_at_cast);
                mix_f64(&mut checksum, *projectile_speed);
                mix_f64(&mut checksum, *effect_hitbox_radius);
            }
            Self::ControlledChampionOffensiveSecondaryHit => {
                mix(&mut checksum, 8);
            }
            Self::ControlledChampionOffensiveUltimateHit => {
                mix(&mut checksum, 9);
            }
            Self::ChampionScript(idx, event, epoch) => {
                mix(&mut checksum, 10);
                mix_usize(&mut checksum, *idx);
                mix_usize(&mut checksum, *event as usize);
                mix(&mut checksum, *epoch);
            }
        }
        checksum
    }
}

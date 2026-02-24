use crate::defaults::simulator_defaults;
use crate::scripts::champions::ChampionBehaviorProfile;

use super::vector_2d_math::Vec2;

pub(in crate::engine) fn enemy_spawn_position(
    index: usize,
    total: usize,
    behavior: ChampionBehaviorProfile,
) -> Vec2 {
    let defaults = &simulator_defaults().engine_defaults;
    let angle = (index as f64 / total.max(1) as f64) * std::f64::consts::TAU;
    let radius = if behavior.attack_range <= defaults.melee_spawn_attack_range_threshold {
        defaults.melee_spawn_radius
    } else {
        (behavior.attack_range * defaults.ranged_spawn_radius_multiplier).clamp(
            defaults.ranged_spawn_radius_min,
            defaults.ranged_spawn_radius_max,
        )
    };
    Vec2 {
        x: radius * angle.cos(),
        y: radius * angle.sin(),
    }
}

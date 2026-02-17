use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::defaults::{
    AbilityExecutionProfile, champion_ability_execution_profile, champion_ai_profile,
    champion_hitbox_radius, default_ability_execution_profile,
    protoplasm_lifeline_cooldown_seconds_default, simulator_defaults,
};
use crate::scripts::champions::vladimir::{
    VladimirCastProfile, VladimirDefensiveAbilityDecisionInput, VladimirOffensiveDecisionInput,
    VladimirTargetSnapshot, decide_defensive_ability_activations, decide_offensive_casts,
    default_cast_profile,
};
use crate::scripts::champions::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptPoint, attack_speed_multiplier, behavior_profile,
    build_champion_loadout_runtime, champion_script_event_cooldown_seconds,
    clear_transient_combat_state, execute_champion_script_event, on_ability_bonus_damage,
    on_hit_bonus_damage, scripted_champion_events, tick_regen_heal,
};
use crate::scripts::items::hooks::controlled_champion_defensive_item_capabilities;
use crate::scripts::runtime::ability_slots::{
    AbilitySlotKey, ActorAbilityLoadout, default_champion_ability_loadout,
};
use crate::scripts::runtime::controlled_champion_loadout::{
    ControlledChampionAbilityRuntimeInput, ControlledChampionLoadoutRuntime,
    DefensiveItemActivationInput, ReviveEffectDecisionInput,
    build_controlled_champion_loadout_runtime, controlled_champion_damage_taken_multiplier,
    controlled_champion_heal_multiplier, decide_defensive_item_activations,
    on_controlled_champion_ability_bonus, on_controlled_champion_enemy_kill,
    should_trigger_revive_effect, tick_controlled_champion_regen_heal,
};

use super::*;

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn distance_to(self, other: Vec2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn enemy_spawn_position(index: usize, total: usize, behavior: ChampionBehaviorProfile) -> Vec2 {
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

fn projectile_travel_seconds(distance: f64, speed: f64) -> f64 {
    if speed <= 0.0 {
        0.0
    } else {
        (distance / speed).max(0.0)
    }
}

fn cross(a: Vec2, b: Vec2, c: Vec2) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn almost_zero(value: f64) -> bool {
    value.abs() <= 1e-9
}

fn on_segment(a: Vec2, b: Vec2, p: Vec2) -> bool {
    if !almost_zero(cross(a, b, p)) {
        return false;
    }
    let min_x = a.x.min(b.x) - 1e-9;
    let max_x = a.x.max(b.x) + 1e-9;
    let min_y = a.y.min(b.y) - 1e-9;
    let max_y = a.y.max(b.y) + 1e-9;
    p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
}

fn line_segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    let d1 = cross(a1, a2, b1);
    let d2 = cross(a1, a2, b2);
    let d3 = cross(b1, b2, a1);
    let d4 = cross(b1, b2, a2);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }

    if almost_zero(d1) && on_segment(a1, a2, b1) {
        return true;
    }
    if almost_zero(d2) && on_segment(a1, a2, b2) {
        return true;
    }
    if almost_zero(d3) && on_segment(b1, b2, a1) {
        return true;
    }
    if almost_zero(d4) && on_segment(b1, b2, a2) {
        return true;
    }

    false
}

fn dot(a: Vec2, b: Vec2) -> f64 {
    a.x * b.x + a.y * b.y
}

fn distance_point_to_segment(point: Vec2, seg_start: Vec2, seg_end: Vec2) -> f64 {
    let segment = Vec2 {
        x: seg_end.x - seg_start.x,
        y: seg_end.y - seg_start.y,
    };
    let len_sq = dot(segment, segment);
    if len_sq <= 1e-9 {
        return point.distance_to(seg_start);
    }
    let from_start = Vec2 {
        x: point.x - seg_start.x,
        y: point.y - seg_start.y,
    };
    let t = (dot(from_start, segment) / len_sq).clamp(0.0, 1.0);
    let projection = Vec2 {
        x: seg_start.x + segment.x * t,
        y: seg_start.y + segment.y * t,
    };
    point.distance_to(projection)
}

fn distance_segment_to_segment(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> f64 {
    if line_segments_intersect(a1, a2, b1, b2) {
        return 0.0;
    }
    distance_point_to_segment(a1, b1, b2)
        .min(distance_point_to_segment(a2, b1, b2))
        .min(distance_point_to_segment(b1, a1, a2))
        .min(distance_point_to_segment(b2, a1, a2))
}

fn path_hits_circle(
    source: Vec2,
    aim_point: Vec2,
    target_center: Vec2,
    target_hitbox_radius: f64,
    effect_hitbox_radius: f64,
) -> bool {
    let reach = target_hitbox_radius.max(0.0) + effect_hitbox_radius.max(0.0);
    if source.distance_to(aim_point) <= 1e-9 {
        return source.distance_to(target_center) <= reach;
    }
    distance_point_to_segment(target_center, source, aim_point) <= reach
}

fn within_reach_with_hitboxes(
    center_distance: f64,
    range: f64,
    source_hitbox_radius: f64,
    target_hitbox_radius: f64,
    effect_hitbox_radius: f64,
) -> bool {
    center_distance
        <= range.max(0.0)
            + source_hitbox_radius.max(0.0)
            + target_hitbox_radius.max(0.0)
            + effect_hitbox_radius.max(0.0)
}

fn uptime_window_active(enemy: &EnemyConfig, time: f64, enabled: bool) -> bool {
    if !enabled {
        return true;
    }
    let cycle = enemy.uptime_cycle_seconds;
    let active = enemy.uptime_active_seconds;
    if cycle <= 0.0 || active <= 0.0 || active >= cycle {
        return true;
    }
    let phase = enemy.uptime_phase_seconds.max(0.0);
    let t = (time + phase) % cycle;
    t <= active
}

struct EnemyState {
    enemy: EnemyConfig,
    movement_mode: OpponentMovementMode,
    behavior: ChampionBehaviorProfile,
    ability_execution: AbilityExecutionProfile,
    burst_execution: AbilityExecutionProfile,
    runtime: ChampionLoadoutRuntime,
    position: Vec2,
    spawn_position: Vec2,
    move_speed: f64,
    base_attack_speed: f64,
    physical_hit_damage: f64,
    ability_hit_damage: f64,
    burst_physical_damage: f64,
    burst_magic_damage: f64,
    burst_true_damage: f64,
    next_attack_bonus_physical: f64,
    next_attack_bonus_magic: f64,
    next_attack_bonus_true: f64,
    max_health: f64,
    health: f64,
    magic_multiplier: f64,
    respawn_at: Option<f64>,
    uptime_active: bool,
    script_epoch: u64,
    script_poll_interval_seconds: f64,
    script_event_ready_at: HashMap<ChampionScriptEvent, f64>,
    attack_sequence: u64,
    stunned_until: f64,
    untargetable_until: f64,
    stasis_until: f64,
    invulnerable_until: f64,
    hitbox_radius: f64,
}

#[derive(Debug, Clone)]
struct EnemyDerivedModel {
    behavior: ChampionBehaviorProfile,
    ability_execution: AbilityExecutionProfile,
    burst_execution: AbilityExecutionProfile,
    runtime: ChampionLoadoutRuntime,
    max_health: f64,
    armor: f64,
    magic_resist: f64,
    magic_multiplier: f64,
    attack_damage: f64,
    attack_speed: f64,
    attack_interval: f64,
    ability_interval: f64,
    ability_hit_damage: f64,
    burst_physical_damage: f64,
    burst_magic_damage: f64,
    burst_true_damage: f64,
    move_speed: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct EnemyDerivedCombatStats {
    pub champion: String,
    pub max_health: f64,
    pub armor: f64,
    pub magic_resist: f64,
    pub attack_damage: f64,
    pub attack_speed: f64,
    pub attack_interval_seconds: f64,
    pub attack_range: f64,
    pub attack_projectile_speed: f64,
    pub move_speed: f64,
    pub desired_combat_range: f64,
    pub physical_hit_damage: f64,
    pub ability_hit_damage: f64,
    pub burst_physical_damage: f64,
    pub burst_magic_damage: f64,
    pub burst_true_damage: f64,
}

#[derive(Debug, Clone)]
struct ProjectileBlockZone {
    start: Vec2,
    end: Vec2,
    half_width: f64,
    expires_at: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IncomingImpactOutcome {
    Applied,
    ProjectileBlocked,
    MissedHitbox,
    NullifiedUntargetable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DamageApplicationOutcome {
    Applied,
    NullifiedUntargetable,
    Ignored,
}

#[derive(Debug, Clone)]
enum EventType {
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
    Ability(usize),
    AbilityWindup {
        idx: usize,
        source: Vec2,
        target_at_cast: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    AbilityHit {
        idx: usize,
        source: Vec2,
        target_at_cast: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    Stun(usize),
    Burst(usize),
    BurstWindup {
        idx: usize,
        source: Vec2,
        target_at_cast: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    BurstHit {
        idx: usize,
        source: Vec2,
        target_at_cast: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    ControlledChampionQHit {
        idx: usize,
        source: Vec2,
        target_at_cast: Vec2,
        projectile_speed: f64,
        effect_hitbox_radius: f64,
    },
    ControlledChampionEHit,
    ControlledChampionRHit,
    ChampionScript(usize, ChampionScriptEvent, u64),
}

#[derive(Debug, Clone)]
struct QueuedEvent {
    time: f64,
    priority: i32,
    seq: u64,
    recurring: Option<f64>,
    kind: EventType,
}

impl PartialEq for QueuedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.priority == other.priority && self.seq == other.seq
    }
}

impl Eq for QueuedEvent {}

impl PartialOrd for QueuedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QueuedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .partial_cmp(&self.time)
            .unwrap_or(Ordering::Equal)
            .then_with(|| other.priority.cmp(&self.priority))
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

pub(super) struct ControlledChampionCombatSimulation {
    vlad_base: ChampionBase,
    sim: SimulationConfig,
    urf: UrfBuffs,

    tick_seconds: f64,
    time: f64,
    finished: bool,
    death_time: Option<f64>,
    damage_dealt_total: f64,
    healing_done_total: f64,
    enemy_kills_total: usize,
    invulnerable_seconds_total: f64,

    event_queue: BinaryHeap<QueuedEvent>,
    event_counter: u64,

    vlad_stats: Stats,
    controlled_champion_runtime: ControlledChampionLoadoutRuntime,
    controlled_champion_name: String,
    controlled_champion_hitbox_radius: f64,
    max_health: f64,
    health: f64,

    physical_multiplier: f64,
    magic_multiplier: f64,

    pool_cooldown: f64,
    pool_duration: f64,
    offensive_tuning: VladimirAbilityTuning,
    offensive_cooldowns: VladimirAbilityCooldowns,
    cast_profile: VladimirCastProfile,
    controlled_champion_ability_loadout: ActorAbilityLoadout,
    controlled_champion_ability_ready_at: HashMap<String, f64>,

    stasis_item_available: bool,
    revive_item_available: bool,
    emergency_shield_item_available: bool,

    revive_item_cooldown_seconds: f64,
    stasis_item_cooldown_seconds: f64,
    emergency_shield_item_cooldown_seconds: f64,

    stasis_item_ready_at: f64,
    revive_item_ready_at: f64,
    emergency_shield_item_ready_at: f64,

    pool_until: f64,
    stasis_until: f64,
    revive_lockout_until: f64,
    stunned_until: f64,
    combat_primitives: CombatPrimitivesState,

    emergency_shield_amount: f64,
    pool_heal_rate: f64,
    pool_heal_until: f64,
    emergency_heal_rate: f64,
    emergency_heal_until: f64,

    target_position: Vec2,
    enemy_state: Vec<EnemyState>,
    projectile_block_zones: Vec<ProjectileBlockZone>,
    trace_enabled: bool,
    trace_events: Vec<String>,
}

impl ControlledChampionCombatSimulation {
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        vlad_base: ChampionBase,
        vlad_build_items: &[Item],
        vlad_bonus_stats: &Stats,
        vlad_item_acquired_levels: Option<&HashMap<String, usize>>,
        vlad_stack_overrides: Option<&HashMap<String, f64>>,
        enemies: &[(EnemyConfig, Vec<Item>, Stats)],
        sim: SimulationConfig,
        urf: UrfBuffs,
    ) -> Self {
        Self::new_with_controlled_champion_loadout(
            vlad_base,
            vlad_build_items,
            vlad_bonus_stats,
            None,
            vlad_item_acquired_levels,
            vlad_stack_overrides,
            enemies,
            sim,
            urf,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn new_with_controlled_champion_loadout(
        controlled_champion_base: ChampionBase,
        controlled_champion_build_items: &[Item],
        controlled_champion_bonus_stats: &Stats,
        controlled_champion_loadout_selection: Option<&LoadoutSelection>,
        controlled_champion_item_acquired_levels: Option<&HashMap<String, usize>>,
        controlled_champion_stack_overrides: Option<&HashMap<String, f64>>,
        enemies: &[(EnemyConfig, Vec<Item>, Stats)],
        sim: SimulationConfig,
        urf: UrfBuffs,
    ) -> Self {
        let vlad_base = controlled_champion_base;
        let vlad_build_items = controlled_champion_build_items;
        let vlad_bonus_stats = controlled_champion_bonus_stats;
        let vlad_item_acquired_levels = controlled_champion_item_acquired_levels;
        let controlled_champion_name = vlad_base.name.clone();
        let controlled_champion_hitbox_radius = champion_hitbox_radius(&vlad_base.name);
        let mut vlad_item_stats = Stats::default();
        for item in vlad_build_items {
            vlad_item_stats.add(&item.stats);
        }
        vlad_item_stats.add(vlad_bonus_stats);
        apply_item_assumptions(
            &mut vlad_item_stats,
            &vlad_base,
            vlad_build_items,
            &sim,
            sim.champion_level,
            vlad_item_acquired_levels,
            controlled_champion_stack_overrides,
        );
        let vlad_stats = compute_champion_final_stats(&vlad_base, &vlad_item_stats);
        let controlled_champion_runtime = controlled_champion_loadout_selection
            .map(build_controlled_champion_loadout_runtime)
            .unwrap_or_default();

        let max_health = vlad_stats.health;
        let physical_multiplier = 100.0 / (100.0 + vlad_stats.armor.max(0.0));
        let magic_multiplier = 100.0 / (100.0 + vlad_stats.magic_resist.max(0.0));

        let ability_haste = vlad_item_stats.ability_haste + urf.ability_haste;
        let pool_base_cd = sim
            .vlad_pool_base_cooldown_seconds_by_rank
            .get(sim.vlad_pool_rank.saturating_sub(1))
            .copied()
            .or_else(|| sim.vlad_pool_base_cooldown_seconds_by_rank.last().copied())
            .unwrap_or(16.0);
        let pool_cooldown = cooldown_after_haste(pool_base_cd, ability_haste);
        let offensive_tuning = VladimirAbilityTuning {
            q_base_damage: sim.vlad_q_base_damage,
            q_ap_ratio: sim.vlad_q_ap_ratio,
            q_heal_ratio_of_damage: sim.vlad_q_heal_ratio_of_damage,
            q_base_cooldown_seconds: sim.vlad_q_base_cooldown_seconds,
            e_base_damage: sim.vlad_e_base_damage,
            e_ap_ratio: sim.vlad_e_ap_ratio,
            e_base_cooldown_seconds: sim.vlad_e_base_cooldown_seconds,
            r_base_damage: sim.vlad_r_base_damage,
            r_ap_ratio: sim.vlad_r_ap_ratio,
            r_base_cooldown_seconds: sim.vlad_r_base_cooldown_seconds,
        };
        let offensive_cooldowns = offensive_cooldowns_after_haste(offensive_tuning, ability_haste);
        let cast_profile = default_cast_profile();
        let mut controlled_champion_ability_loadout =
            default_champion_ability_loadout(&controlled_champion_name);
        // Ensure the actor can still cast core Vladimir abilities even if external bindings are missing.
        if controlled_champion_ability_loadout
            .slot_for_ability(&cast_profile.q_ability_id)
            .is_none()
        {
            controlled_champion_ability_loadout
                .assign_ability_to_slot(cast_profile.q_ability_id.clone(), AbilitySlotKey::Q);
        }
        if controlled_champion_ability_loadout
            .slot_for_ability(&cast_profile.pool_ability_id)
            .is_none()
        {
            controlled_champion_ability_loadout
                .assign_ability_to_slot(cast_profile.pool_ability_id.clone(), AbilitySlotKey::W);
        }
        if controlled_champion_ability_loadout
            .slot_for_ability(&cast_profile.e_ability_id)
            .is_none()
        {
            controlled_champion_ability_loadout
                .assign_ability_to_slot(cast_profile.e_ability_id.clone(), AbilitySlotKey::E);
        }
        if controlled_champion_ability_loadout
            .slot_for_ability(&cast_profile.r_ability_id)
            .is_none()
        {
            controlled_champion_ability_loadout
                .assign_ability_to_slot(cast_profile.r_ability_id.clone(), AbilitySlotKey::R);
        }
        let mut controlled_champion_ability_ready_at = HashMap::new();
        controlled_champion_ability_ready_at.insert(cast_profile.q_ability_id.clone(), 0.0);
        controlled_champion_ability_ready_at.insert(cast_profile.pool_ability_id.clone(), 0.0);
        controlled_champion_ability_ready_at.insert(cast_profile.e_ability_id.clone(), 0.0);
        controlled_champion_ability_ready_at.insert(cast_profile.r_ability_id.clone(), 0.0);

        let defensive_item_capabilities =
            controlled_champion_defensive_item_capabilities(vlad_build_items);
        let stasis_item_available = defensive_item_capabilities.has_stasis_item;
        let revive_item_available = defensive_item_capabilities.has_revive_item;
        let emergency_shield_item_available = defensive_item_capabilities.has_emergency_shield_item;

        let revive_item_cooldown_seconds =
            cooldown_after_haste(sim.ga_cooldown_seconds, urf.item_haste);
        let stasis_item_cooldown_seconds =
            cooldown_after_haste(sim.zhonya_cooldown_seconds, urf.item_haste);
        let emergency_shield_item_cooldown_seconds = cooldown_after_haste(
            protoplasm_lifeline_cooldown_seconds_default(),
            urf.item_haste,
        );

        let tick_seconds = if sim.server_tick_rate_hz > 0.0 {
            1.0 / sim.server_tick_rate_hz
        } else {
            sim.dt
        };

        let mut runner = Self {
            vlad_base,
            sim,
            urf,
            tick_seconds,
            time: 0.0,
            finished: false,
            death_time: None,
            damage_dealt_total: 0.0,
            healing_done_total: 0.0,
            enemy_kills_total: 0,
            invulnerable_seconds_total: 0.0,
            event_queue: BinaryHeap::new(),
            event_counter: 0,
            vlad_stats,
            controlled_champion_runtime,
            controlled_champion_name,
            controlled_champion_hitbox_radius,
            max_health,
            health: max_health,
            physical_multiplier,
            magic_multiplier,
            pool_cooldown,
            pool_duration: 0.0,
            offensive_tuning,
            offensive_cooldowns,
            cast_profile,
            controlled_champion_ability_loadout,
            controlled_champion_ability_ready_at,
            stasis_item_available,
            revive_item_available,
            emergency_shield_item_available,
            revive_item_cooldown_seconds,
            stasis_item_cooldown_seconds,
            emergency_shield_item_cooldown_seconds,
            stasis_item_ready_at: 0.0,
            revive_item_ready_at: 0.0,
            emergency_shield_item_ready_at: 0.0,
            pool_until: 0.0,
            stasis_until: 0.0,
            revive_lockout_until: 0.0,
            stunned_until: 0.0,
            combat_primitives: CombatPrimitivesState::default(),
            emergency_shield_amount: 0.0,
            pool_heal_rate: 0.0,
            pool_heal_until: 0.0,
            emergency_heal_rate: 0.0,
            emergency_heal_until: 0.0,
            target_position: Vec2 { x: 0.0, y: 0.0 },
            enemy_state: Vec::new(),
            projectile_block_zones: Vec::new(),
            trace_enabled: false,
            trace_events: Vec::new(),
        };

        runner.pool_duration = runner.sim.vlad_pool_untargetable_seconds;

        let enemy_count = enemies.len();
        for (idx, (enemy, build, enemy_bonus)) in enemies.iter().cloned().enumerate() {
            let model = derive_enemy_model(&enemy, &build, &enemy_bonus, &runner.sim, &runner.urf);
            let position = enemy
                .spawn_position_xy
                .map(|(x, y)| Vec2 { x, y })
                .unwrap_or_else(|| enemy_spawn_position(idx, enemy_count.max(1), model.behavior));
            let ai_profile = champion_ai_profile(&enemy.name, model.behavior.attack_range);
            let script_poll_interval_seconds = ai_profile.script_poll_interval_seconds.max(0.05);

            runner.enemy_state.push(EnemyState {
                enemy: enemy.clone(),
                movement_mode: enemy.movement_mode,
                behavior: model.behavior,
                ability_execution: model.ability_execution,
                burst_execution: model.burst_execution,
                runtime: model.runtime,
                position,
                spawn_position: position,
                move_speed: model.move_speed,
                base_attack_speed: model.attack_speed.max(0.001),
                physical_hit_damage: model.attack_damage,
                ability_hit_damage: model.ability_hit_damage,
                burst_physical_damage: model.burst_physical_damage,
                burst_magic_damage: model.burst_magic_damage,
                burst_true_damage: model.burst_true_damage,
                next_attack_bonus_physical: 0.0,
                next_attack_bonus_magic: 0.0,
                next_attack_bonus_true: 0.0,
                max_health: model.max_health,
                health: model.max_health,
                magic_multiplier: model.magic_multiplier,
                respawn_at: None,
                uptime_active: uptime_window_active(
                    &enemy,
                    runner.time,
                    runner.sim.enemy_uptime_model_enabled,
                ),
                script_epoch: 0,
                script_poll_interval_seconds,
                script_event_ready_at: HashMap::new(),
                attack_sequence: 0,
                stunned_until: 0.0,
                untargetable_until: 0.0,
                stasis_until: 0.0,
                invulnerable_until: 0.0,
                hitbox_radius: champion_hitbox_radius(&enemy.base.name),
            });

            runner.schedule_event(model.attack_interval, 30, EventType::Attack(idx), None);
            if model.ability_hit_damage > 0.0 {
                runner.schedule_event(
                    model.ability_interval,
                    40,
                    EventType::Ability(idx),
                    Some(model.ability_interval),
                );
            }
            if enemy.stun_interval_seconds > 0.0 {
                runner.schedule_event(
                    enemy.stun_interval_seconds,
                    20,
                    EventType::Stun(idx),
                    Some(enemy.stun_interval_seconds),
                );
            }
            if enemy.burst_interval_seconds > 0.0
                && (model.burst_physical_damage > 0.0
                    || model.burst_magic_damage > 0.0
                    || model.burst_true_damage > 0.0)
            {
                runner.schedule_event(
                    enemy.burst_start_offset_seconds.max(0.0),
                    10,
                    EventType::Burst(idx),
                    Some(enemy.burst_interval_seconds),
                );
            }
            for event in scripted_champion_events(&enemy.name) {
                runner.schedule_event(
                    0.0,
                    12,
                    EventType::ChampionScript(idx, event, 0),
                    Some(script_poll_interval_seconds),
                );
            }
        }

        runner
    }

    fn schedule_event(
        &mut self,
        delay: f64,
        priority: i32,
        kind: EventType,
        recurring: Option<f64>,
    ) {
        self.event_counter += 1;
        self.event_queue.push(QueuedEvent {
            time: self.time + delay.max(0.0),
            priority,
            seq: self.event_counter,
            recurring,
            kind,
        });
    }

    fn trace_event(&mut self, kind: &str, details: String) {
        if !self.trace_enabled {
            return;
        }
        self.trace_events
            .push(format!("{:.3}s [{}] {}", self.time, kind, details));
    }

    pub(super) fn is_targetable(&self) -> bool {
        self.time >= self.pool_until
            && self.time >= self.stasis_until
            && self.time >= self.revive_lockout_until
    }

    fn controlled_champion_is_stunned(&self) -> bool {
        self.time < self.stunned_until
            || self
                .combat_primitives
                .status_effects()
                .is_active(&StatusEffectKind::Stun)
    }

    fn controlled_champion_is_invulnerable_or_untargetable(&self) -> bool {
        !self.is_targetable()
            || self
                .combat_primitives
                .status_effects()
                .is_active(&StatusEffectKind::Stasis)
            || self
                .combat_primitives
                .status_effects()
                .is_active(&StatusEffectKind::Untargetable)
    }

    pub(super) fn can_cast(&self) -> bool {
        !self.controlled_champion_is_stunned()
            && !self.controlled_champion_is_invulnerable_or_untargetable()
    }

    fn controlled_champion_ability_ready_at(&self, ability_id: &str) -> f64 {
        self.controlled_champion_ability_ready_at
            .get(ability_id)
            .copied()
            .unwrap_or(0.0)
    }

    fn set_controlled_champion_ability_ready_at(&mut self, ability_id: &str, ready_at: f64) {
        self.controlled_champion_ability_ready_at
            .insert(ability_id.to_string(), ready_at);
    }

    fn apply_status_effect(&mut self, effect: StatusEffect) {
        self.combat_primitives.apply_status(effect);
    }

    fn apply_stun_window(&mut self, duration_seconds: f64) {
        if duration_seconds <= 0.0 {
            return;
        }
        self.apply_status_effect(StatusEffect::timed(
            StatusEffectKind::Stun,
            duration_seconds,
            1,
            StatusPersistence::RefreshDuration,
        ));
    }

    fn begin_cast_lock_window(&mut self, windup_seconds: f64, channel_seconds: f64, lockout: f64) {
        self.combat_primitives.begin_cast_lock(CastLockWindow::new(
            windup_seconds,
            channel_seconds,
            lockout,
        ));
    }

    fn enemy_respawn_delay_seconds(&self) -> f64 {
        respawn::urf_respawn_delay_seconds(
            self.sim.champion_level,
            self.time,
            respawn::UrfRespawnTuning {
                urf_flat_reduction_seconds: self.sim.urf_respawn_flat_reduction_seconds,
                extrapolation_per_level: self.sim.urf_respawn_extrapolation_per_level,
                time_scaling_enabled: self.sim.urf_respawn_time_scaling_enabled,
                time_scaling_start_seconds: self.sim.urf_respawn_time_scaling_start_seconds,
                time_scaling_per_minute_seconds: self
                    .sim
                    .urf_respawn_time_scaling_per_minute_seconds,
                time_scaling_cap_seconds: self.sim.urf_respawn_time_scaling_cap_seconds,
            },
        )
    }

    fn refresh_enemy_respawns(&mut self) {
        let mut respawned = Vec::new();
        for (idx, state) in self.enemy_state.iter_mut().enumerate() {
            let Some(respawn_at) = state.respawn_at else {
                continue;
            };
            if self.time >= respawn_at {
                state.health = state.max_health;
                state.respawn_at = None;
                state.position = state.spawn_position;
                clear_transient_combat_state(&mut state.runtime);
                state.next_attack_bonus_physical = 0.0;
                state.next_attack_bonus_magic = 0.0;
                state.next_attack_bonus_true = 0.0;
                state.script_epoch = state.script_epoch.wrapping_add(1);
                state.script_event_ready_at.clear();
                state.attack_sequence = state.attack_sequence.wrapping_add(1);
                state.stunned_until = 0.0;
                state.untargetable_until = 0.0;
                state.stasis_until = 0.0;
                state.invulnerable_until = 0.0;
                state.uptime_active = uptime_window_active(
                    &state.enemy,
                    self.time,
                    self.sim.enemy_uptime_model_enabled,
                );
                respawned.push((idx, state.enemy.name.clone(), state.script_epoch));
            }
        }
        for (idx, name, epoch) in respawned {
            let champion_name = self.enemy_state[idx].enemy.name.clone();
            let poll_interval = self.enemy_state[idx].script_poll_interval_seconds.max(0.05);
            for event in scripted_champion_events(&champion_name) {
                self.schedule_event(
                    0.0,
                    12,
                    EventType::ChampionScript(idx, event, epoch),
                    Some(poll_interval),
                );
            }
            self.trace_event("enemy_respawn", format!("{} respawned", name));
        }
    }

    fn enemy_is_alive(&self, idx: usize) -> bool {
        let state = &self.enemy_state[idx];
        state.respawn_at.is_none() && state.health > 0.0
    }

    fn enemy_is_active(&self, idx: usize) -> bool {
        if !self.enemy_is_alive(idx) {
            return false;
        }
        self.enemy_state[idx].uptime_active
    }

    fn refresh_uptime_transitions(&mut self) {
        if !self.sim.enemy_uptime_model_enabled {
            return;
        }
        let mut trace = Vec::new();
        let mut reschedule = Vec::new();
        for (idx, state) in self.enemy_state.iter_mut().enumerate() {
            if state.respawn_at.is_some() || state.health <= 0.0 {
                state.uptime_active = false;
                continue;
            }
            let active_now = uptime_window_active(&state.enemy, self.time, true);
            if active_now != state.uptime_active {
                clear_transient_combat_state(&mut state.runtime);
                state.next_attack_bonus_physical = 0.0;
                state.next_attack_bonus_magic = 0.0;
                state.next_attack_bonus_true = 0.0;
                state.attack_sequence = state.attack_sequence.wrapping_add(1);
                state.stunned_until = 0.0;
                state.untargetable_until = 0.0;
                state.stasis_until = 0.0;
                state.invulnerable_until = 0.0;
                let msg = if active_now {
                    format!("{} re-entered combat window", state.enemy.name)
                } else {
                    format!("{} left combat window", state.enemy.name)
                };
                trace.push(("enemy_uptime", msg));
                state.script_epoch = state.script_epoch.wrapping_add(1);
                state.script_event_ready_at.clear();
                if active_now {
                    reschedule.push((
                        idx,
                        state.script_epoch,
                        state.enemy.name.clone(),
                        state.script_poll_interval_seconds.max(0.05),
                    ));
                }
                state.uptime_active = active_now;
            }
        }
        for (idx, epoch, champion_name, poll_interval) in reschedule {
            for event in scripted_champion_events(&champion_name) {
                self.schedule_event(
                    0.0,
                    12,
                    EventType::ChampionScript(idx, event, epoch),
                    Some(poll_interval),
                );
            }
        }
        for (kind, msg) in trace {
            self.trace_event(kind, msg);
        }
    }

    fn distance_to_target(&self, idx: usize) -> f64 {
        self.enemy_state[idx]
            .position
            .distance_to(self.target_position)
    }

    fn enemy_in_attack_range(&self, idx: usize) -> bool {
        let state = &self.enemy_state[idx];
        within_reach_with_hitboxes(
            self.distance_to_target(idx),
            state.behavior.attack_range,
            state.hitbox_radius,
            self.controlled_champion_hitbox_radius,
            state.behavior.attack_effect_hitbox_radius,
        )
    }

    fn enemy_in_controlled_champion_range(
        &self,
        idx: usize,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> bool {
        let state = &self.enemy_state[idx];
        within_reach_with_hitboxes(
            self.distance_to_target(idx),
            range,
            self.controlled_champion_hitbox_radius,
            state.hitbox_radius,
            effect_hitbox_radius,
        )
    }

    fn enemy_projectile_delay_from_points(&self, source: Vec2, target: Vec2, speed: f64) -> f64 {
        projectile_travel_seconds(source.distance_to(target), speed)
    }

    fn cleanup_expired_projectile_blocks(&mut self) {
        self.projectile_block_zones
            .retain(|zone| zone.expires_at > self.time);
    }

    fn is_projectile_blocked(&self, source: Vec2, target: Vec2, projectile_radius: f64) -> bool {
        self.projectile_block_zones
            .iter()
            .filter(|zone| zone.expires_at > self.time)
            .any(|zone| {
                distance_segment_to_segment(source, target, zone.start, zone.end)
                    <= projectile_radius.max(0.0) + zone.half_width.max(0.0)
            })
    }

    fn enemy_is_stunned(&self, idx: usize) -> bool {
        self.time < self.enemy_state[idx].stunned_until
    }

    fn enemy_is_invulnerable_or_untargetable(&self, idx: usize) -> bool {
        let state = &self.enemy_state[idx];
        self.time < state.untargetable_until
            || self.time < state.stasis_until
            || self.time < state.invulnerable_until
    }

    fn enemy_can_take_actions(&self, idx: usize) -> bool {
        self.enemy_is_active(idx)
            && !self.enemy_is_stunned(idx)
            && !self.enemy_is_invulnerable_or_untargetable(idx)
    }

    fn first_active_enemy_in_controlled_champion_range(
        &self,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_is_active(idx)
                || !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius)
            {
                continue;
            }
            let dist = self.distance_to_target(idx);
            match best {
                Some((_, best_dist)) if dist >= best_dist => {}
                _ => best = Some((idx, dist)),
            }
        }
        best.map(|(idx, _)| idx)
    }

    fn max_enemy_distance_in_controlled_champion_range(
        &self,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> Option<f64> {
        let mut max_distance = None;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_is_active(idx)
                || !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius)
            {
                continue;
            }
            let distance = self.distance_to_target(idx);
            max_distance = Some(match max_distance {
                Some(current) => distance.max(current),
                None => distance,
            });
        }
        max_distance
    }

    fn active_enemy_count_in_controlled_champion_range(
        &self,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> usize {
        self.enemy_state
            .iter()
            .enumerate()
            .filter(|(idx, _)| {
                self.enemy_is_active(*idx)
                    && self.enemy_in_controlled_champion_range(*idx, range, effect_hitbox_radius)
            })
            .count()
    }

    fn schedule_next_attack(&mut self, idx: usize) {
        if idx >= self.enemy_state.len() {
            return;
        }
        let state = &self.enemy_state[idx];
        let attack_speed = state.base_attack_speed * attack_speed_multiplier(&state.runtime);
        let interval =
            1.0 / attack_speed.max(simulator_defaults().engine_defaults.minimum_attack_speed);
        self.schedule_event(interval, 30, EventType::Attack(idx), None);
    }

    fn script_point_from_vec2(point: Vec2) -> ChampionScriptPoint {
        ChampionScriptPoint {
            x: point.x,
            y: point.y,
        }
    }

    fn vec2_from_script_point(point: ChampionScriptPoint) -> Vec2 {
        Vec2 {
            x: point.x,
            y: point.y,
        }
    }

    fn apply_enemy_script_actions(
        &mut self,
        idx: usize,
        epoch: u64,
        actions: Vec<ChampionScriptAction>,
    ) {
        for action in actions {
            match action {
                ChampionScriptAction::AddNextAttackBonusPhysical {
                    amount,
                    trace_message,
                } => {
                    let enemy_name = {
                        let state = &mut self.enemy_state[idx];
                        state.next_attack_bonus_physical += amount;
                        state.enemy.name.clone()
                    };
                    self.trace_event("enemy_buff", format!("{} {}", enemy_name, trace_message));
                }
                ChampionScriptAction::ApplyDamage {
                    source,
                    projectile_speed,
                    hitbox,
                    physical,
                    magic,
                    true_damage,
                    stun_duration,
                } => {
                    let source = Self::vec2_from_script_point(source);
                    let effect_hitbox_radius = hitbox.radius();
                    let outcome = if projectile_speed > 0.0
                        && self.is_projectile_blocked(
                            source,
                            self.target_position,
                            effect_hitbox_radius,
                        ) {
                        IncomingImpactOutcome::ProjectileBlocked
                    } else {
                        let hit = path_hits_circle(
                            source,
                            self.target_position,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        );
                        if !hit {
                            IncomingImpactOutcome::MissedHitbox
                        } else {
                            match self.apply_damage(physical, magic, true_damage) {
                                DamageApplicationOutcome::Applied => IncomingImpactOutcome::Applied,
                                DamageApplicationOutcome::NullifiedUntargetable => {
                                    IncomingImpactOutcome::NullifiedUntargetable
                                }
                                DamageApplicationOutcome::Ignored => {
                                    IncomingImpactOutcome::MissedHitbox
                                }
                            }
                        }
                    };
                    if stun_duration > 0.0 && outcome == IncomingImpactOutcome::Applied {
                        self.stunned_until = self.stunned_until.max(self.time + stun_duration);
                        self.apply_stun_window(stun_duration);
                    }
                    match outcome {
                        IncomingImpactOutcome::Applied => {}
                        IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                            "projectile_blocked",
                            format!(
                                "{} scripted projectile blocked",
                                self.enemy_state[idx].enemy.name
                            ),
                        ),
                        IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                            "impact_nullified",
                            format!(
                                "{} scripted impact on {} was nullified by untargetable state",
                                self.enemy_state[idx].enemy.name, self.controlled_champion_name
                            ),
                        ),
                        IncomingImpactOutcome::MissedHitbox => self.trace_event(
                            "impact_missed",
                            format!(
                                "{} scripted impact missed {} hitbox",
                                self.enemy_state[idx].enemy.name, self.controlled_champion_name
                            ),
                        ),
                    }
                }
                ChampionScriptAction::ScheduleFollowup {
                    delay_seconds,
                    priority,
                    event,
                } => {
                    self.schedule_event(
                        delay_seconds,
                        priority,
                        EventType::ChampionScript(idx, event, epoch),
                        None,
                    );
                }
            }
        }
    }

    fn apply_magic_damage_to_enemy(&mut self, idx: usize, raw_magic_damage: f64) -> f64 {
        if raw_magic_damage <= 0.0 || !self.enemy_is_active(idx) {
            return 0.0;
        }
        let mitigated = {
            let state = &self.enemy_state[idx];
            raw_magic_damage * state.magic_multiplier
        };
        if mitigated <= 0.0 {
            return 0.0;
        }
        let respawn_delay = self.enemy_respawn_delay_seconds();
        let mut killed_name = None;
        let dealt = {
            let state = &mut self.enemy_state[idx];
            let d = mitigated.min(state.health.max(0.0));
            state.health -= d;
            if state.health <= 0.0 {
                state.health = 0.0;
                state.respawn_at = Some(self.time + respawn_delay);
                clear_transient_combat_state(&mut state.runtime);
                state.next_attack_bonus_physical = 0.0;
                state.next_attack_bonus_magic = 0.0;
                state.next_attack_bonus_true = 0.0;
                state.uptime_active = false;
                state.script_epoch = state.script_epoch.wrapping_add(1);
                state.script_event_ready_at.clear();
                state.attack_sequence = state.attack_sequence.wrapping_add(1);
                state.stunned_until = 0.0;
                state.untargetable_until = 0.0;
                state.stasis_until = 0.0;
                state.invulnerable_until = 0.0;
                killed_name = Some(state.enemy.name.clone());
            }
            d
        };
        if let Some(name) = killed_name {
            self.enemy_kills_total += 1;
            let runtime_kill_heal = on_controlled_champion_enemy_kill(
                &mut self.controlled_champion_runtime,
                self.max_health,
            );
            if runtime_kill_heal > 0.0 {
                let before = self.health;
                self.health = self.max_health.min(
                    self.health
                        + runtime_kill_heal
                            * controlled_champion_heal_multiplier(
                                &self.controlled_champion_runtime,
                            ),
                );
                self.healing_done_total += (self.health - before).max(0.0);
            }
            self.trace_event(
                "enemy_death",
                format!("{} died; respawn in {:.1}s", name, respawn_delay),
            );
        }
        dealt
    }

    fn apply_magic_damage_to_all_active_enemies(
        &mut self,
        raw_magic_damage: f64,
        effect_hitbox_radius: f64,
    ) -> f64 {
        self.apply_magic_damage_to_enemies_in_controlled_champion_range(
            raw_magic_damage,
            f64::INFINITY,
            effect_hitbox_radius,
        )
    }

    fn apply_magic_damage_to_enemies_in_controlled_champion_range(
        &mut self,
        raw_magic_damage: f64,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> f64 {
        if raw_magic_damage <= 0.0 {
            return 0.0;
        }
        let mut total = 0.0;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius) {
                continue;
            }
            total += self.apply_magic_damage_to_enemy(idx, raw_magic_damage);
        }
        total
    }

    fn apply_hot_effects(&mut self, to_time: f64) {
        if to_time <= self.time {
            return;
        }
        let delta = to_time - self.time;
        if delta > 0.0 {
            let invulnerable_until = self
                .pool_until
                .max(self.stasis_until)
                .max(self.revive_lockout_until);
            let invulnerable_overlap = (to_time.min(invulnerable_until) - self.time).max(0.0);
            self.invulnerable_seconds_total += invulnerable_overlap;
        }
        if self.pool_heal_until > self.time {
            let active = delta.min(self.pool_heal_until - self.time);
            let before = self.health;
            self.health = self
                .max_health
                .min(self.health + self.pool_heal_rate * active);
            self.healing_done_total += (self.health - before).max(0.0);
        }
        if self.emergency_heal_until > self.time {
            let active = delta.min(self.emergency_heal_until - self.time);
            let before = self.health;
            self.health = self
                .max_health
                .min(self.health + self.emergency_heal_rate * active);
            self.healing_done_total += (self.health - before).max(0.0);
        }
        let runtime_regen = tick_controlled_champion_regen_heal(
            &self.controlled_champion_runtime,
            self.health,
            self.max_health,
            delta,
        );
        if runtime_regen > 0.0 {
            let before = self.health;
            self.health = self.max_health.min(self.health + runtime_regen);
            self.healing_done_total += (self.health - before).max(0.0);
        }
        self.combat_primitives.tick(delta);
        self.update_actor_positions(delta);
        self.apply_enemy_regen(delta);
        self.time = to_time;
        self.refresh_uptime_transitions();
        self.cleanup_expired_projectile_blocks();
    }

    fn apply_enemy_regen(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        for state in &mut self.enemy_state {
            if state.respawn_at.is_some() || state.health <= 0.0 {
                continue;
            }
            let heal = tick_regen_heal(&state.runtime, state.health, state.max_health, delta);
            state.health = (state.health + heal).min(state.max_health);
        }
    }

    fn update_actor_positions(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        for (idx, state) in self.enemy_state.iter_mut().enumerate() {
            if state.respawn_at.is_some() || state.health <= 0.0 {
                continue;
            }
            if state.movement_mode == OpponentMovementMode::HoldPosition {
                continue;
            }
            let speed = state.move_speed * state.behavior.movement_speed_scale;
            let step = speed * delta;
            let mut radial = Vec2 {
                x: state.position.x - self.target_position.x,
                y: state.position.y - self.target_position.y,
            };
            let distance = radial.distance_to(Vec2 { x: 0.0, y: 0.0 }).max(1e-6);
            radial.x /= distance;
            radial.y /= distance;

            let desired = state.behavior.desired_combat_range.max(75.0);
            let radial_error = distance - desired;
            let radial_step = radial_error.clamp(-step, step);
            state.position.x -= radial.x * radial_step;
            state.position.y -= radial.y * radial_step;

            // Deterministic tangential orbiting to create realistic kiting/chasing arcs.
            let tangent_dir = if idx % 2 == 0 { 1.0 } else { -1.0 };
            let tangent = Vec2 {
                x: -radial.y * tangent_dir,
                y: radial.x * tangent_dir,
            };
            let tangential_step = step
                * if state.enemy.base.is_melee {
                    0.08
                } else {
                    0.20
                };
            state.position.x += tangent.x * tangential_step;
            state.position.y += tangent.y * tangential_step;
        }
    }

    fn apply_damage(
        &mut self,
        physical: f64,
        magic: f64,
        true_damage: f64,
    ) -> DamageApplicationOutcome {
        if self.finished || self.health <= 0.0 {
            return DamageApplicationOutcome::Ignored;
        }
        if !self.is_targetable() {
            return DamageApplicationOutcome::NullifiedUntargetable;
        }
        let mut damage =
            physical * self.physical_multiplier + magic * self.magic_multiplier + true_damage;
        let active_enemy_count = self
            .enemy_state
            .iter()
            .filter(|state| state.respawn_at.is_none() && state.health > 0.0 && state.uptime_active)
            .count();
        damage *= controlled_champion_damage_taken_multiplier(
            &self.controlled_champion_runtime,
            active_enemy_count,
        );
        if self.emergency_shield_amount > 0.0 && damage > 0.0 {
            let absorbed = self.emergency_shield_amount.min(damage);
            self.emergency_shield_amount -= absorbed;
            damage -= absorbed;
        }
        self.trace_event(
            "damage_in",
            format!(
                "physical {:.1}, magic {:.1}, true {:.1}, total {:.1}",
                physical, magic, true_damage, damage
            ),
        );
        self.health -= damage;
        if self.health <= 0.0 {
            self.handle_death();
        }
        DamageApplicationOutcome::Applied
    }

    fn handle_death(&mut self) {
        if should_trigger_revive_effect(ReviveEffectDecisionInput {
            available: self.revive_item_available,
            now_seconds: self.time,
            ready_at: self.revive_item_ready_at,
        }) {
            self.revive_item_ready_at = self.time + self.revive_item_cooldown_seconds;
            self.revive_lockout_until = self.time + self.sim.ga_revive_duration_seconds;
            self.health =
                1.0_f64.max(self.vlad_base.base_health * self.sim.ga_revive_base_health_ratio);
            self.trace_event(
                "revive_effect",
                format!("Revive item restored {}", self.controlled_champion_name),
            );
            return;
        }
        self.finished = true;
        self.death_time = Some(self.time);
        self.trace_event(
            "controlled_champion_death",
            format!("{} died", self.controlled_champion_name),
        );
    }

    fn maybe_cast_controlled_champion_abilities_and_defensives(&mut self) {
        if self.finished {
            return;
        }
        self.refresh_enemy_respawns();

        let defensive_ability =
            decide_defensive_ability_activations(VladimirDefensiveAbilityDecisionInput {
                now_seconds: self.time,
                can_cast: self.can_cast(),
                pool_ready_at: self
                    .controlled_champion_ability_ready_at(&self.cast_profile.pool_ability_id),
            });

        if defensive_ability.cast_pool {
            let pool_ability_id = self.cast_profile.pool_ability_id.clone();
            self.set_controlled_champion_ability_ready_at(
                &pool_ability_id,
                self.time + self.pool_cooldown,
            );
            self.pool_until = self.time + self.pool_duration;
            self.apply_status_effect(StatusEffect::timed(
                StatusEffectKind::Untargetable,
                self.pool_duration,
                1,
                StatusPersistence::RefreshDuration,
            ));
            let cost = self.health
                * self.sim.vlad_pool_cost_percent_current_health
                * self.urf.health_cost_multiplier;
            self.health -= cost;

            let mut pool_damage = self
                .sim
                .vlad_pool_base_damage_by_rank
                .get(self.sim.vlad_pool_rank.saturating_sub(1))
                .copied()
                .or_else(|| self.sim.vlad_pool_base_damage_by_rank.last().copied())
                .unwrap_or(0.0);
            pool_damage += self.sim.vlad_pool_bonus_health_ratio
                * (self.vlad_stats.health - self.vlad_base.base_health);
            let total_pool_damage = self.apply_magic_damage_to_all_active_enemies(pool_damage, 0.0);
            self.damage_dealt_total += total_pool_damage.max(0.0);
            let pool_heal = total_pool_damage * self.sim.vlad_pool_heal_ratio_of_damage;
            self.pool_heal_rate = if self.pool_duration > 0.0 {
                pool_heal / self.pool_duration
            } else {
                0.0
            };
            self.pool_heal_until = self.time + self.pool_duration;

            if self.health <= 0.0 {
                self.handle_death();
                return;
            }
        }

        // Script-owned cadence for controlled champion offensive spell scheduling.
        let can_cast = self.can_cast();
        let q_ability_ready_at =
            self.controlled_champion_ability_ready_at(&self.cast_profile.q_ability_id);
        let q_equipped = self
            .controlled_champion_ability_loadout
            .slot_for_ability(&self.cast_profile.q_ability_id)
            .is_some();
        let q_target = if can_cast && q_equipped && self.time >= q_ability_ready_at {
            self.first_active_enemy_in_controlled_champion_range(
                self.cast_profile.q_range,
                self.cast_profile.q_effect_hitbox_radius,
            )
            .map(|target_index| VladimirTargetSnapshot {
                target_index,
                distance: self.distance_to_target(target_index),
            })
        } else {
            None
        };
        let e_ability_ready_at =
            self.controlled_champion_ability_ready_at(&self.cast_profile.e_ability_id);
        let e_equipped = self
            .controlled_champion_ability_loadout
            .slot_for_ability(&self.cast_profile.e_ability_id)
            .is_some();
        let e_max_distance = if can_cast && e_equipped && self.time >= e_ability_ready_at {
            self.max_enemy_distance_in_controlled_champion_range(
                self.cast_profile.e_range,
                self.cast_profile.e_effect_hitbox_radius,
            )
        } else {
            None
        };
        let r_ability_ready_at =
            self.controlled_champion_ability_ready_at(&self.cast_profile.r_ability_id);
        let r_equipped = self
            .controlled_champion_ability_loadout
            .slot_for_ability(&self.cast_profile.r_ability_id)
            .is_some();
        let r_max_distance = if can_cast && r_equipped && self.time >= r_ability_ready_at {
            self.max_enemy_distance_in_controlled_champion_range(
                self.cast_profile.r_range,
                self.cast_profile.r_effect_hitbox_radius,
            )
        } else {
            None
        };
        let offensive = decide_offensive_casts(VladimirOffensiveDecisionInput {
            now_seconds: self.time,
            can_cast,
            q_ready_at: q_ability_ready_at,
            e_ready_at: e_ability_ready_at,
            r_ready_at: r_ability_ready_at,
            cooldowns: self.offensive_cooldowns,
            cast_profile: self.cast_profile.clone(),
            q_target,
            e_max_distance,
            r_max_distance,
        });

        if let Some(q) = offensive.q {
            self.set_controlled_champion_ability_ready_at(&q.ability_id, q.next_ready_at);
            self.begin_cast_lock_window(self.cast_profile.q_windup_seconds, 0.0, 0.0);
            let target_at_cast = self.enemy_state[q.target_index].position;
            self.schedule_event(
                q.impact_delay_seconds,
                50,
                EventType::ControlledChampionQHit {
                    idx: q.target_index,
                    source: self.target_position,
                    target_at_cast,
                    projectile_speed: self.cast_profile.q_projectile_speed,
                    effect_hitbox_radius: self.cast_profile.q_effect_hitbox_radius,
                },
                None,
            );
        }
        if let Some(e) = offensive.e {
            self.set_controlled_champion_ability_ready_at(&e.ability_id, e.next_ready_at);
            self.begin_cast_lock_window(self.cast_profile.e_windup_seconds, 0.0, 0.0);
            self.schedule_event(
                e.impact_delay_seconds,
                49,
                EventType::ControlledChampionEHit,
                None,
            );
        }
        if let Some(r) = offensive.r {
            self.set_controlled_champion_ability_ready_at(&r.ability_id, r.next_ready_at);
            self.begin_cast_lock_window(self.cast_profile.r_windup_seconds, 0.0, 0.0);
            self.schedule_event(
                r.impact_delay_seconds,
                48,
                EventType::ControlledChampionRHit,
                None,
            );
        }

        let defensive_items = decide_defensive_item_activations(DefensiveItemActivationInput {
            now_seconds: self.time,
            can_cast: self.can_cast(),
            health: self.health,
            max_health: self.max_health,
            stasis_available: self.stasis_item_available,
            stasis_ready_at: self.stasis_item_ready_at,
            stasis_trigger_health_percent: self.sim.zhonya_trigger_health_percent,
            untargetable_active_until: self.pool_until,
            revive_lock_active_until: self.revive_lockout_until,
            emergency_shield_available: self.emergency_shield_item_available,
            emergency_shield_ready_at: self.emergency_shield_item_ready_at,
            emergency_shield_trigger_health_percent: self.sim.protoplasm_trigger_health_percent,
        });

        if defensive_items.activate_stasis {
            self.stasis_item_ready_at = self.time + self.stasis_item_cooldown_seconds;
            self.stasis_until = self.time + self.sim.zhonya_duration_seconds;
            self.apply_status_effect(StatusEffect::timed(
                StatusEffectKind::Stasis,
                self.sim.zhonya_duration_seconds,
                1,
                StatusPersistence::RefreshDuration,
            ));
        }

        if defensive_items.activate_emergency_shield {
            self.emergency_shield_item_ready_at =
                self.time + self.emergency_shield_item_cooldown_seconds;
            self.emergency_shield_amount += self.sim.protoplasm_bonus_health;
            self.emergency_heal_rate =
                self.sim.protoplasm_heal_total / self.sim.protoplasm_duration_seconds.max(0.001);
            self.emergency_heal_until = self.time + self.sim.protoplasm_duration_seconds;
        }
    }

    fn process_event(&mut self, ev: &QueuedEvent) {
        match ev.kind {
            EventType::Attack(idx) => {
                if !self.enemy_can_take_actions(idx) || !self.enemy_in_attack_range(idx) {
                    self.schedule_next_attack(idx);
                    return;
                }
                let token = {
                    let state = &mut self.enemy_state[idx];
                    state.attack_sequence = state.attack_sequence.wrapping_add(1);
                    state.attack_sequence
                };
                self.trace_event(
                    "attack_start",
                    format!("{} begins auto attack", self.enemy_state[idx].enemy.name),
                );
                let windup = self.enemy_state[idx]
                    .behavior
                    .attack_windup_seconds
                    .max(0.0);
                self.schedule_event(windup, 35, EventType::AttackWindup { idx, token }, None);
            }
            EventType::AttackWindup { idx, token } => {
                if !self.enemy_is_active(idx)
                    || self.enemy_state[idx].attack_sequence != token
                    || !self.enemy_in_attack_range(idx)
                {
                    self.schedule_next_attack(idx);
                    return;
                }
                if !self.enemy_can_take_actions(idx) {
                    self.trace_event(
                        "attack_cancelled",
                        format!(
                            "{} auto attack cancelled during windup by crowd control or invulnerability",
                            self.enemy_state[idx].enemy.name
                        ),
                    );
                    self.schedule_next_attack(idx);
                    return;
                }
                let source = self.enemy_state[idx].position;
                let target_at_release = self.target_position;
                let projectile_speed = self.enemy_state[idx].behavior.attack_projectile_speed;
                let effect_hitbox_radius =
                    self.enemy_state[idx].behavior.attack_effect_hitbox_radius;
                let travel = self.enemy_projectile_delay_from_points(
                    source,
                    target_at_release,
                    projectile_speed,
                );
                self.schedule_event(
                    travel,
                    34,
                    EventType::AttackHit {
                        idx,
                        token,
                        source,
                        target_at_release,
                        projectile_speed,
                        effect_hitbox_radius,
                    },
                    None,
                );
            }
            EventType::AttackHit {
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if !self.enemy_is_active(idx) || self.enemy_state[idx].attack_sequence != token {
                    self.schedule_next_attack(idx);
                    return;
                }
                if projectile_speed <= 0.0 && !self.enemy_can_take_actions(idx) {
                    self.trace_event(
                        "attack_cancelled",
                        format!(
                            "{} melee attack cancelled before impact by crowd control or invulnerability",
                            self.enemy_state[idx].enemy.name
                        ),
                    );
                    self.schedule_next_attack(idx);
                    return;
                }
                let target_current = self.health.max(0.0);
                let target_max = self.max_health.max(1.0);
                let (physical, magic, true_damage) = {
                    let state = &mut self.enemy_state[idx];
                    let attack_damage =
                        state.physical_hit_damage + state.next_attack_bonus_physical;
                    let (extra_physical, extra_magic, extra_true) = on_hit_bonus_damage(
                        state.behavior,
                        &mut state.runtime,
                        attack_damage,
                        target_current,
                        target_max,
                        state.max_health,
                        self.time,
                    );
                    let out = (
                        attack_damage + extra_physical,
                        state.next_attack_bonus_magic + extra_magic,
                        state.next_attack_bonus_true + extra_true,
                    );
                    state.next_attack_bonus_physical = 0.0;
                    state.next_attack_bonus_magic = 0.0;
                    state.next_attack_bonus_true = 0.0;
                    out
                };
                let outcome = if projectile_speed > 0.0
                    && self.is_projectile_blocked(source, target_at_release, effect_hitbox_radius)
                {
                    IncomingImpactOutcome::ProjectileBlocked
                } else {
                    let hit = if projectile_speed > 0.0 {
                        path_hits_circle(
                            source,
                            target_at_release,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        )
                    } else {
                        path_hits_circle(
                            source,
                            source,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        )
                    };
                    if !hit {
                        IncomingImpactOutcome::MissedHitbox
                    } else {
                        match self.apply_damage(physical, magic, true_damage) {
                            DamageApplicationOutcome::Applied => IncomingImpactOutcome::Applied,
                            DamageApplicationOutcome::NullifiedUntargetable => {
                                IncomingImpactOutcome::NullifiedUntargetable
                            }
                            DamageApplicationOutcome::Ignored => {
                                IncomingImpactOutcome::MissedHitbox
                            }
                        }
                    }
                };
                match outcome {
                    IncomingImpactOutcome::Applied => self.trace_event(
                        "attack_hit",
                        format!(
                            "{} hit {} (phys {:.1}, magic {:.1}, true {:.1})",
                            self.enemy_state[idx].enemy.name,
                            self.controlled_champion_name,
                            physical,
                            magic,
                            true_damage
                        ),
                    ),
                    IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                        "projectile_blocked",
                        format!("{} auto attack blocked", self.enemy_state[idx].enemy.name),
                    ),
                    IncomingImpactOutcome::MissedHitbox => self.trace_event(
                        "attack_missed",
                        format!(
                            "{} auto attack missed {} hitbox",
                            self.enemy_state[idx].enemy.name, self.controlled_champion_name
                        ),
                    ),
                    IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                        "impact_nullified",
                        format!(
                            "{} auto attack impacted {} during untargetable state",
                            self.enemy_state[idx].enemy.name, self.controlled_champion_name
                        ),
                    ),
                }
                self.schedule_next_attack(idx);
            }
            EventType::Ability(idx) => {
                if !self.enemy_can_take_actions(idx) {
                    return;
                }
                let execution = self.enemy_state[idx].ability_execution;
                let source = self.enemy_state[idx].position;
                let target_at_cast = self.target_position;
                self.schedule_event(
                    execution.cast_windup_seconds.max(0.0),
                    46,
                    EventType::AbilityWindup {
                        idx,
                        source,
                        target_at_cast,
                        projectile_speed: execution.projectile_speed,
                        effect_hitbox_radius: execution.effect_hitbox_radius,
                    },
                    None,
                );
            }
            EventType::AbilityWindup {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if !self.enemy_can_take_actions(idx) {
                    self.trace_event(
                        "ability_cancelled",
                        format!(
                            "{} ability cancelled during windup by crowd control or invulnerability",
                            self.enemy_state[idx].enemy.name
                        ),
                    );
                    return;
                }
                let travel = self.enemy_projectile_delay_from_points(
                    source,
                    target_at_cast,
                    projectile_speed,
                );
                self.schedule_event(
                    travel,
                    45,
                    EventType::AbilityHit {
                        idx,
                        source,
                        target_at_cast,
                        projectile_speed,
                        effect_hitbox_radius,
                    },
                    None,
                );
            }
            EventType::AbilityHit {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let target_max = self.max_health.max(1.0);
                let (magic, true_damage) = {
                    let state = &mut self.enemy_state[idx];
                    let (extra_magic, extra_true) = on_ability_bonus_damage(
                        &mut state.runtime,
                        state.ability_hit_damage,
                        target_max,
                        self.time,
                    );
                    (state.ability_hit_damage + extra_magic, extra_true)
                };
                let outcome = if projectile_speed > 0.0
                    && self.is_projectile_blocked(source, target_at_cast, effect_hitbox_radius)
                {
                    IncomingImpactOutcome::ProjectileBlocked
                } else {
                    let hit = if projectile_speed > 0.0 {
                        path_hits_circle(
                            source,
                            target_at_cast,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        )
                    } else {
                        path_hits_circle(
                            source,
                            source,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        )
                    };
                    if !hit {
                        IncomingImpactOutcome::MissedHitbox
                    } else {
                        match self.apply_damage(0.0, magic, true_damage) {
                            DamageApplicationOutcome::Applied => IncomingImpactOutcome::Applied,
                            DamageApplicationOutcome::NullifiedUntargetable => {
                                IncomingImpactOutcome::NullifiedUntargetable
                            }
                            DamageApplicationOutcome::Ignored => {
                                IncomingImpactOutcome::MissedHitbox
                            }
                        }
                    }
                };
                match outcome {
                    IncomingImpactOutcome::Applied => self.trace_event(
                        "ability_hit",
                        format!(
                            "{} ability hit {} (magic {:.1}, true {:.1})",
                            self.enemy_state[idx].enemy.name,
                            self.controlled_champion_name,
                            magic,
                            true_damage
                        ),
                    ),
                    IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                        "projectile_blocked",
                        format!("{} ability blocked", self.enemy_state[idx].enemy.name),
                    ),
                    IncomingImpactOutcome::MissedHitbox => self.trace_event(
                        "ability_missed",
                        format!(
                            "{} ability missed {} hitbox",
                            self.enemy_state[idx].enemy.name, self.controlled_champion_name
                        ),
                    ),
                    IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                        "impact_nullified",
                        format!(
                            "{} ability impacted {} during untargetable state",
                            self.enemy_state[idx].enemy.name, self.controlled_champion_name
                        ),
                    ),
                }
            }
            EventType::Stun(idx) => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let enemy = &self.enemy_state[idx].enemy;
                if self.is_targetable() {
                    self.stunned_until = self
                        .stunned_until
                        .max(self.time + enemy.stun_duration_seconds);
                    self.apply_stun_window(enemy.stun_duration_seconds);
                }
            }
            EventType::Burst(idx) => {
                if !self.enemy_can_take_actions(idx) {
                    return;
                }
                let execution = self.enemy_state[idx].burst_execution;
                let source = self.enemy_state[idx].position;
                let target_at_cast = self.target_position;
                self.schedule_event(
                    execution.cast_windup_seconds.max(0.0),
                    26,
                    EventType::BurstWindup {
                        idx,
                        source,
                        target_at_cast,
                        projectile_speed: execution.projectile_speed,
                        effect_hitbox_radius: execution.effect_hitbox_radius,
                    },
                    None,
                );
            }
            EventType::BurstWindup {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if !self.enemy_can_take_actions(idx) {
                    self.trace_event(
                        "burst_cancelled",
                        format!(
                            "{} burst cancelled during windup by crowd control or invulnerability",
                            self.enemy_state[idx].enemy.name
                        ),
                    );
                    return;
                }
                let travel = self.enemy_projectile_delay_from_points(
                    source,
                    target_at_cast,
                    projectile_speed,
                );
                self.schedule_event(
                    travel,
                    25,
                    EventType::BurstHit {
                        idx,
                        source,
                        target_at_cast,
                        projectile_speed,
                        effect_hitbox_radius,
                    },
                    None,
                );
            }
            EventType::BurstHit {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let target_max = self.max_health.max(1.0);
                let (physical, magic, true_damage) = {
                    let state = &mut self.enemy_state[idx];
                    let (extra_magic, extra_true) = on_ability_bonus_damage(
                        &mut state.runtime,
                        state.burst_magic_damage,
                        target_max,
                        self.time,
                    );
                    (
                        state.burst_physical_damage,
                        state.burst_magic_damage + extra_magic,
                        state.burst_true_damage + extra_true,
                    )
                };
                let outcome = if projectile_speed > 0.0
                    && self.is_projectile_blocked(source, target_at_cast, effect_hitbox_radius)
                {
                    IncomingImpactOutcome::ProjectileBlocked
                } else {
                    let hit = if projectile_speed > 0.0 {
                        path_hits_circle(
                            source,
                            target_at_cast,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        )
                    } else {
                        path_hits_circle(
                            source,
                            source,
                            self.target_position,
                            self.controlled_champion_hitbox_radius,
                            effect_hitbox_radius,
                        )
                    };
                    if !hit {
                        IncomingImpactOutcome::MissedHitbox
                    } else {
                        match self.apply_damage(physical, magic, true_damage) {
                            DamageApplicationOutcome::Applied => IncomingImpactOutcome::Applied,
                            DamageApplicationOutcome::NullifiedUntargetable => {
                                IncomingImpactOutcome::NullifiedUntargetable
                            }
                            DamageApplicationOutcome::Ignored => {
                                IncomingImpactOutcome::MissedHitbox
                            }
                        }
                    }
                };
                match outcome {
                    IncomingImpactOutcome::Applied => self.trace_event(
                        "burst_hit",
                        format!(
                            "{} burst hit {} (phys {:.1}, magic {:.1}, true {:.1})",
                            self.enemy_state[idx].enemy.name,
                            self.controlled_champion_name,
                            physical,
                            magic,
                            true_damage
                        ),
                    ),
                    IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                        "projectile_blocked",
                        format!("{} burst blocked", self.enemy_state[idx].enemy.name),
                    ),
                    IncomingImpactOutcome::MissedHitbox => self.trace_event(
                        "burst_missed",
                        format!(
                            "{} burst missed {} hitbox",
                            self.enemy_state[idx].enemy.name, self.controlled_champion_name
                        ),
                    ),
                    IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                        "impact_nullified",
                        format!(
                            "{} burst impacted {} during untargetable state",
                            self.enemy_state[idx].enemy.name, self.controlled_champion_name
                        ),
                    ),
                }
            }
            EventType::ControlledChampionQHit {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if idx >= self.enemy_state.len() || !self.enemy_is_active(idx) {
                    return;
                }
                if projectile_speed > 0.0
                    && self.is_projectile_blocked(source, target_at_cast, effect_hitbox_radius)
                {
                    self.trace_event(
                        "projectile_blocked",
                        format!("{} Q blocked", self.controlled_champion_name),
                    );
                    return;
                }
                let enemy_position = self.enemy_state[idx].position;
                let enemy_hitbox_radius = self.enemy_state[idx].hitbox_radius;
                let hit = if projectile_speed > 0.0 {
                    path_hits_circle(
                        source,
                        target_at_cast,
                        enemy_position,
                        enemy_hitbox_radius,
                        effect_hitbox_radius,
                    )
                } else {
                    path_hits_circle(
                        source,
                        source,
                        enemy_position,
                        enemy_hitbox_radius,
                        effect_hitbox_radius,
                    )
                };
                if !hit {
                    self.trace_event(
                        "controlled_champion_q_miss",
                        format!(
                            "{} Q missed {}",
                            self.controlled_champion_name, self.enemy_state[idx].enemy.name
                        ),
                    );
                    return;
                }
                let q_raw_damage =
                    q_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let runtime_bonus = on_controlled_champion_ability_bonus(
                    &mut self.controlled_champion_runtime,
                    ControlledChampionAbilityRuntimeInput {
                        ability_power: self.vlad_stats.ability_power,
                        ability_ap_ratio: self.offensive_tuning.q_ap_ratio,
                        now_seconds: self.time,
                    },
                );
                let dealt = self.apply_magic_damage_to_enemy(idx, q_raw_damage)
                    + self.apply_magic_damage_to_enemy(idx, runtime_bonus.extra_magic_damage);
                self.damage_dealt_total += dealt.max(0.0);
                if dealt > 0.0 {
                    let before = self.health;
                    self.health = self.max_health.min(
                        self.health
                            + dealt
                                * self.offensive_tuning.q_heal_ratio_of_damage
                                * controlled_champion_heal_multiplier(
                                    &self.controlled_champion_runtime,
                                ),
                    );
                    self.healing_done_total += (self.health - before).max(0.0);
                }
                self.trace_event(
                    "controlled_champion_q_hit",
                    format!(
                        "{} Q hit {} for {:.1}",
                        self.controlled_champion_name, self.enemy_state[idx].enemy.name, dealt
                    ),
                );
            }
            EventType::ControlledChampionEHit => {
                let e_raw_damage =
                    e_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let target_count = self.active_enemy_count_in_controlled_champion_range(
                    self.cast_profile.e_range,
                    self.cast_profile.e_effect_hitbox_radius,
                );
                let runtime_bonus = on_controlled_champion_ability_bonus(
                    &mut self.controlled_champion_runtime,
                    ControlledChampionAbilityRuntimeInput {
                        ability_power: self.vlad_stats.ability_power,
                        ability_ap_ratio: self.offensive_tuning.e_ap_ratio,
                        now_seconds: self.time,
                    },
                );
                let runtime_bonus_per_target = if target_count > 0 {
                    runtime_bonus.extra_magic_damage / target_count as f64
                } else {
                    0.0
                };
                let dealt = self.apply_magic_damage_to_enemies_in_controlled_champion_range(
                    e_raw_damage,
                    self.cast_profile.e_range,
                    self.cast_profile.e_effect_hitbox_radius,
                ) + self.apply_magic_damage_to_enemies_in_controlled_champion_range(
                    runtime_bonus_per_target,
                    self.cast_profile.e_range,
                    self.cast_profile.e_effect_hitbox_radius,
                );
                self.damage_dealt_total += dealt.max(0.0);
                self.trace_event(
                    "controlled_champion_e_hit",
                    format!("{} E dealt {:.1}", self.controlled_champion_name, dealt),
                );
            }
            EventType::ControlledChampionRHit => {
                let r_raw_damage =
                    r_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let target_count = self.active_enemy_count_in_controlled_champion_range(
                    self.cast_profile.r_range,
                    self.cast_profile.r_effect_hitbox_radius,
                );
                let runtime_bonus = on_controlled_champion_ability_bonus(
                    &mut self.controlled_champion_runtime,
                    ControlledChampionAbilityRuntimeInput {
                        ability_power: self.vlad_stats.ability_power,
                        ability_ap_ratio: self.offensive_tuning.r_ap_ratio,
                        now_seconds: self.time,
                    },
                );
                let runtime_bonus_per_target = if target_count > 0 {
                    runtime_bonus.extra_magic_damage / target_count as f64
                } else {
                    0.0
                };
                let dealt = self.apply_magic_damage_to_enemies_in_controlled_champion_range(
                    r_raw_damage,
                    self.cast_profile.r_range,
                    self.cast_profile.r_effect_hitbox_radius,
                ) + self.apply_magic_damage_to_enemies_in_controlled_champion_range(
                    runtime_bonus_per_target,
                    self.cast_profile.r_range,
                    self.cast_profile.r_effect_hitbox_radius,
                );
                self.damage_dealt_total += dealt.max(0.0);
                self.trace_event(
                    "controlled_champion_r_hit",
                    format!("{} R dealt {:.1}", self.controlled_champion_name, dealt),
                );
            }
            EventType::ChampionScript(idx, script_event, epoch) => {
                if idx >= self.enemy_state.len()
                    || self.enemy_state[idx].script_epoch != epoch
                    || !self.enemy_can_take_actions(idx)
                {
                    return;
                }
                let script_ready_at = self.enemy_state[idx]
                    .script_event_ready_at
                    .get(&script_event)
                    .copied()
                    .unwrap_or(0.0);
                if self.time + 1e-9 < script_ready_at {
                    return;
                }
                let champion_name = self.enemy_state[idx].enemy.name.clone();
                let distance_to_target = self.distance_to_target(idx);
                let target_current_health = self.health;
                let target_max_health = self.max_health;
                let now = self.time;
                let actions = {
                    let state = &mut self.enemy_state[idx];
                    let input = ChampionScriptExecutionInput {
                        event: script_event,
                        actor_position: Self::script_point_from_vec2(state.position),
                        distance_to_target,
                        physical_hit_damage: state.physical_hit_damage,
                        burst_magic_damage: state.burst_magic_damage,
                        target_current_health,
                        target_max_health,
                        now,
                    };
                    execute_champion_script_event(input, &mut state.runtime)
                };
                if !actions.is_empty() {
                    self.trace_event(
                        "champion_script",
                        format!("{} executed {:?}", champion_name, script_event),
                    );
                    if let Some(cooldown_seconds) =
                        champion_script_event_cooldown_seconds(&champion_name, script_event)
                    {
                        let next_ready = self.time + cooldown_seconds.max(0.0);
                        if let Some(state) = self.enemy_state.get_mut(idx) {
                            state.script_event_ready_at.insert(script_event, next_ready);
                        }
                    }
                }
                self.apply_enemy_script_actions(idx, epoch, actions);
            }
        }
    }

    pub(super) fn step(&mut self, ticks: usize) -> bool {
        for _ in 0..ticks.max(1) {
            if self.finished || self.time >= self.sim.max_time_seconds {
                self.finished = true;
                return false;
            }

            let target_time = self.sim.max_time_seconds.min(self.time + self.tick_seconds);
            self.maybe_cast_controlled_champion_abilities_and_defensives();

            while let Some(top) = self.event_queue.peek().cloned() {
                if top.time > target_time || self.finished {
                    break;
                }
                self.event_queue.pop();
                self.apply_hot_effects(top.time);
                self.refresh_enemy_respawns();
                self.process_event(&top);
                let should_recur = match &top.kind {
                    EventType::ChampionScript(idx, _, epoch) => self
                        .enemy_state
                        .get(*idx)
                        .map(|state| {
                            state.script_epoch == *epoch
                                && state.respawn_at.is_none()
                                && state.health > 0.0
                                && state.uptime_active
                        })
                        .unwrap_or(false),
                    _ => true,
                };
                if let Some(recurring) = top.recurring
                    && recurring > 0.0
                    && !self.finished
                    && should_recur
                {
                    self.event_counter += 1;
                    self.event_queue.push(QueuedEvent {
                        time: top.time + recurring,
                        priority: top.priority,
                        seq: self.event_counter,
                        recurring: top.recurring,
                        kind: top.kind.clone(),
                    });
                }
                self.maybe_cast_controlled_champion_abilities_and_defensives();
            }

            self.apply_hot_effects(target_time);
            self.refresh_enemy_respawns();
            self.maybe_cast_controlled_champion_abilities_and_defensives();

            if self.health <= 0.0 && !self.finished {
                self.handle_death();
            }
            if self.finished {
                return false;
            }
        }
        true
    }

    fn run_until_end(&mut self) -> CombatOutcome {
        while self.step(1) {}
        CombatOutcome {
            time_alive_seconds: self
                .death_time
                .unwrap_or(self.time.min(self.sim.max_time_seconds)),
            damage_dealt: self.damage_dealt_total,
            healing_done: self.healing_done_total,
            enemy_kills: self.enemy_kills_total,
            invulnerable_seconds: self.invulnerable_seconds_total,
        }
    }

    pub(super) fn tick_seconds(&self) -> f64 {
        self.tick_seconds
    }

    pub(super) fn current_time(&self) -> f64 {
        self.time
    }

    pub(super) fn current_health(&self) -> f64 {
        self.health
    }

    pub(super) fn enable_trace(&mut self) {
        self.trace_enabled = true;
        self.trace_events.clear();
    }

    pub(super) fn trace_events(&self) -> &[String] {
        &self.trace_events
    }
}

fn compute_enemy_dps(enemy: &EnemyConfig, item_stats: &Stats, urf: &UrfBuffs) -> (f64, f64) {
    let attack_damage = enemy.base.base_attack_damage + item_stats.attack_damage;
    let attack_speed_bonus = item_stats.attack_speed_percent / 100.0;
    let mut attack_speed = enemy.base.base_attack_speed * (1.0 + attack_speed_bonus);
    attack_speed *= if enemy.base.is_melee {
        urf.bonus_attack_speed_multiplier_melee
    } else {
        urf.bonus_attack_speed_multiplier_ranged
    };
    let physical_dps = attack_damage * attack_speed;

    let mut ability_dps = enemy.ability_dps_flat;
    ability_dps += enemy.ability_dps_ad_ratio * attack_damage;
    ability_dps += enemy.ability_dps_ap_ratio * item_stats.ability_power;
    (physical_dps, ability_dps)
}

fn derive_enemy_model(
    enemy: &EnemyConfig,
    build: &[Item],
    enemy_bonus: &Stats,
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> EnemyDerivedModel {
    let mut enemy_stats = Stats::default();
    for item in build {
        enemy_stats.add(&item.stats);
    }
    enemy_stats.add(enemy_bonus);
    apply_item_assumptions(
        &mut enemy_stats,
        &enemy.base,
        build,
        sim,
        enemy.level,
        None,
        Some(&enemy.stack_overrides),
    );

    let (_physical_dps, magic_dps) = compute_enemy_dps(enemy, &enemy_stats, urf);
    let attack_damage = enemy.base.base_attack_damage + enemy_stats.attack_damage;
    let armor = (enemy.base.base_armor + enemy_stats.armor).max(0.0);
    let magic_resist = (enemy.base.base_magic_resist + enemy_stats.magic_resist).max(0.0);
    let max_health = (enemy.base.base_health + enemy_stats.health).max(1.0);
    let move_speed = ((enemy.base.base_move_speed + enemy_stats.move_speed_flat).max(150.0))
        * (1.0 + enemy_stats.move_speed_percent / 100.0);

    let attack_speed_bonus = enemy_stats.attack_speed_percent / 100.0;
    let mut attack_speed = enemy.base.base_attack_speed * (1.0 + attack_speed_bonus);
    attack_speed *= if enemy.base.is_melee {
        urf.bonus_attack_speed_multiplier_melee
    } else {
        urf.bonus_attack_speed_multiplier_ranged
    };
    let base_attack_speed = attack_speed.max(0.001);

    let runtime_item_names = if enemy.loadout_item_names.is_empty() {
        build
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>()
    } else {
        enemy.loadout_item_names.clone()
    };
    let runtime = build_champion_loadout_runtime(&runtime_item_names, &enemy.loadout_rune_names);
    attack_speed = base_attack_speed * attack_speed_multiplier(&runtime);

    let attack_interval = 1.0 / attack_speed.max(0.001);
    let ability_interval = enemy.ability_tick_interval_seconds.max(0.05);
    let ability_hit_damage = magic_dps * ability_interval;
    let burst_physical_damage = enemy.burst_physical_flat + enemy.burst_ad_ratio * attack_damage;
    let burst_magic_damage =
        enemy.burst_magic_flat + enemy.burst_ap_ratio * enemy_stats.ability_power;
    let burst_true_damage = enemy.burst_true_flat;
    let behavior = behavior_profile(
        &enemy.name,
        enemy.base.is_melee,
        enemy.base.base_attack_range,
        enemy.base.base_attack_projectile_speed,
    );
    let ability_execution =
        champion_ability_execution_profile(&enemy.name, "basic_ability_1", enemy.base.is_melee)
            .unwrap_or_else(|| default_ability_execution_profile(enemy.base.is_melee));
    let burst_execution =
        champion_ability_execution_profile(&enemy.name, "ultimate", enemy.base.is_melee)
            .unwrap_or_else(|| default_ability_execution_profile(enemy.base.is_melee));

    EnemyDerivedModel {
        behavior,
        ability_execution,
        burst_execution,
        runtime,
        max_health,
        armor,
        magic_resist,
        magic_multiplier: 100.0 / (100.0 + magic_resist),
        attack_damage,
        attack_speed,
        attack_interval,
        ability_interval,
        ability_hit_damage,
        burst_physical_damage,
        burst_magic_damage,
        burst_true_damage,
        move_speed,
    }
}

pub(crate) fn derive_enemy_combat_stats(
    enemy: &EnemyConfig,
    build: &[Item],
    enemy_bonus: &Stats,
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> EnemyDerivedCombatStats {
    let model = derive_enemy_model(enemy, build, enemy_bonus, sim, urf);
    EnemyDerivedCombatStats {
        champion: enemy.name.clone(),
        max_health: model.max_health,
        armor: model.armor,
        magic_resist: model.magic_resist,
        attack_damage: model.attack_damage,
        attack_speed: model.attack_speed,
        attack_interval_seconds: model.attack_interval,
        attack_range: model.behavior.attack_range,
        attack_projectile_speed: model.behavior.attack_projectile_speed,
        move_speed: model.move_speed,
        desired_combat_range: model.behavior.desired_combat_range,
        physical_hit_damage: model.attack_damage,
        ability_hit_damage: model.ability_hit_damage,
        burst_physical_damage: model.burst_physical_damage,
        burst_magic_damage: model.burst_magic_damage,
        burst_true_damage: model.burst_true_damage,
    }
}

#[allow(dead_code)]
pub(super) type VladCombatSimulation = ControlledChampionCombatSimulation;

#[allow(clippy::too_many_arguments)]
pub(super) fn simulate_controlled_champion_combat(
    controlled_champion_base: &ChampionBase,
    controlled_champion_build_items: &[Item],
    controlled_champion_bonus_stats: &Stats,
    controlled_champion_loadout_selection: Option<&LoadoutSelection>,
    controlled_champion_item_acquired_levels: Option<&HashMap<String, usize>>,
    controlled_champion_stack_overrides: Option<&HashMap<String, f64>>,
    enemies: &[(EnemyConfig, Vec<Item>, Stats)],
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> CombatOutcome {
    let mut runner = ControlledChampionCombatSimulation::new_with_controlled_champion_loadout(
        controlled_champion_base.clone(),
        controlled_champion_build_items,
        controlled_champion_bonus_stats,
        controlled_champion_loadout_selection,
        controlled_champion_item_acquired_levels,
        controlled_champion_stack_overrides,
        enemies,
        sim.clone(),
        urf.clone(),
    );
    runner.run_until_end()
}

#[allow(dead_code)]
pub(super) fn simulate_vlad_combat(
    vlad_base: &ChampionBase,
    vlad_build_items: &[Item],
    vlad_bonus_stats: &Stats,
    vlad_item_acquired_levels: Option<&HashMap<String, usize>>,
    enemies: &[(EnemyConfig, Vec<Item>, Stats)],
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> CombatOutcome {
    let mut runner = ControlledChampionCombatSimulation::new(
        vlad_base.clone(),
        vlad_build_items,
        vlad_bonus_stats,
        vlad_item_acquired_levels,
        None,
        enemies,
        sim.clone(),
        urf.clone(),
    );
    runner.run_until_end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projectile_travel_time_handles_instant_and_ranged() {
        assert_eq!(projectile_travel_seconds(400.0, 0.0), 0.0);
        assert!((projectile_travel_seconds(500.0, 2000.0) - 0.25).abs() < 1e-9);
    }

    #[test]
    fn spawn_positions_keep_melee_closer_than_ranged() {
        let melee = ChampionBehaviorProfile::default_for(true, 125.0, 0.0);
        let ranged = ChampionBehaviorProfile::default_for(false, 550.0, 2000.0);
        let melee_pos = enemy_spawn_position(0, 5, melee);
        let ranged_pos = enemy_spawn_position(0, 5, ranged);
        let origin = Vec2 { x: 0.0, y: 0.0 };
        assert!(melee_pos.distance_to(origin) < ranged_pos.distance_to(origin));
    }

    #[test]
    fn projectile_path_intersection_detects_blocks() {
        let source = Vec2 { x: 0.0, y: 0.0 };
        let target = Vec2 { x: 1000.0, y: 0.0 };
        let wall_start = Vec2 { x: 500.0, y: 200.0 };
        let wall_end = Vec2 {
            x: 500.0,
            y: -200.0,
        };
        assert!(line_segments_intersect(
            source, target, wall_start, wall_end
        ));

        let miss_start = Vec2 { x: 500.0, y: 300.0 };
        let miss_end = Vec2 { x: 500.0, y: 600.0 };
        assert!(!line_segments_intersect(
            source, target, miss_start, miss_end
        ));

        let colinear_disjoint_start = Vec2 { x: 1200.0, y: 0.0 };
        let colinear_disjoint_end = Vec2 { x: 1400.0, y: 0.0 };
        assert!(!line_segments_intersect(
            source,
            target,
            colinear_disjoint_start,
            colinear_disjoint_end
        ));
    }

    #[test]
    fn path_hits_circle_respects_effect_and_target_hitbox() {
        let source = Vec2 { x: 0.0, y: 0.0 };
        let aim = Vec2 { x: 1000.0, y: 0.0 };
        let near_target = Vec2 { x: 1000.0, y: 70.0 };
        let far_target = Vec2 {
            x: 1000.0,
            y: 120.0,
        };
        assert!(path_hits_circle(source, aim, near_target, 65.0, 10.0));
        assert!(!path_hits_circle(source, aim, far_target, 65.0, 10.0));
    }

    fn test_controlled_champion_base() -> ChampionBase {
        ChampionBase {
            name: "Vladimir".to_string(),
            base_health: 2000.0,
            health_per_level: 0.0,
            base_armor: 45.0,
            armor_per_level: 0.0,
            base_magic_resist: 45.0,
            magic_resist_per_level: 0.0,
            base_attack_damage: 60.0,
            attack_damage_per_level: 0.0,
            base_attack_speed: 0.658,
            attack_speed_per_level_percent: 0.0,
            base_attack_range: 450.0,
            base_attack_projectile_speed: 1600.0,
            base_move_speed: 335.0,
            is_melee: false,
        }
    }

    fn test_enemy_base(name: &str) -> ChampionBase {
        ChampionBase {
            name: name.to_string(),
            base_health: 2200.0,
            health_per_level: 0.0,
            base_armor: 35.0,
            armor_per_level: 0.0,
            base_magic_resist: 35.0,
            magic_resist_per_level: 0.0,
            base_attack_damage: 80.0,
            attack_damage_per_level: 0.0,
            base_attack_speed: 0.70,
            attack_speed_per_level_percent: 0.0,
            base_attack_range: 550.0,
            base_attack_projectile_speed: 1800.0,
            base_move_speed: 330.0,
            is_melee: false,
        }
    }

    fn test_enemy_base_with_role(name: &str, is_melee: bool) -> ChampionBase {
        let mut base = test_enemy_base(name);
        base.is_melee = is_melee;
        if is_melee {
            base.base_attack_range = 125.0;
            base.base_attack_projectile_speed = 0.0;
        }
        base
    }

    fn test_enemy(name: &str, ability_dps_flat: f64) -> EnemyConfig {
        EnemyConfig {
            id: name.to_string(),
            name: name.to_string(),
            level: 20,
            base: test_enemy_base(name),
            spawn_position_xy: None,
            movement_mode: OpponentMovementMode::MaintainCombatRange,
            ability_dps_flat,
            ability_dps_ad_ratio: 0.0,
            ability_dps_ap_ratio: 0.0,
            ability_tick_interval_seconds: 1.0,
            stun_interval_seconds: 0.0,
            stun_duration_seconds: 0.0,
            burst_interval_seconds: 0.0,
            burst_start_offset_seconds: 0.0,
            burst_magic_flat: 0.0,
            burst_physical_flat: 0.0,
            burst_true_flat: 0.0,
            burst_ad_ratio: 0.0,
            burst_ap_ratio: 0.0,
            uptime_cycle_seconds: 0.0,
            uptime_active_seconds: 0.0,
            uptime_phase_seconds: 0.0,
            loadout_item_names: Vec::new(),
            loadout_rune_names: Vec::new(),
            loadout_shards: Vec::new(),
            stack_overrides: HashMap::new(),
        }
    }

    fn test_enemy_with_role(name: &str, ability_dps_flat: f64, is_melee: bool) -> EnemyConfig {
        let mut enemy = test_enemy(name, ability_dps_flat);
        enemy.base = test_enemy_base_with_role(name, is_melee);
        enemy
    }

    fn test_simulation(max_time_seconds: f64, q_base_damage: f64) -> SimulationConfig {
        SimulationConfig {
            dt: 1.0 / 30.0,
            server_tick_rate_hz: 30.0,
            champion_level: 20,
            max_time_seconds,
            vlad_pool_rank: 5,
            vlad_pool_untargetable_seconds: 0.0,
            vlad_pool_cost_percent_current_health: 0.0,
            vlad_pool_heal_ratio_of_damage: 0.0,
            vlad_pool_base_damage_by_rank: vec![0.0, 0.0, 0.0, 0.0, 0.0],
            vlad_pool_base_cooldown_seconds_by_rank: vec![28.0, 25.0, 22.0, 19.0, 16.0],
            vlad_pool_bonus_health_ratio: 0.0,
            zhonya_duration_seconds: 2.5,
            zhonya_cooldown_seconds: 120.0,
            zhonya_trigger_health_percent: 0.0,
            ga_cooldown_seconds: 300.0,
            ga_revive_duration_seconds: 4.0,
            ga_revive_base_health_ratio: 0.3,
            protoplasm_trigger_health_percent: 0.0,
            protoplasm_bonus_health: 0.0,
            protoplasm_heal_total: 0.0,
            protoplasm_duration_seconds: 0.0,
            stack_overrides: HashMap::new(),
            enemy_uptime_model_enabled: false,
            urf_respawn_flat_reduction_seconds: 3.0,
            urf_respawn_extrapolation_per_level: 2.5,
            urf_respawn_time_scaling_enabled: true,
            urf_respawn_time_scaling_start_seconds: 300.0,
            urf_respawn_time_scaling_per_minute_seconds: 0.4,
            urf_respawn_time_scaling_cap_seconds: 20.0,
            vlad_q_base_damage: q_base_damage,
            vlad_q_ap_ratio: 0.6,
            vlad_q_heal_ratio_of_damage: 0.0,
            vlad_q_base_cooldown_seconds: 1.0,
            vlad_e_base_damage: 0.0,
            vlad_e_ap_ratio: 0.0,
            vlad_e_base_cooldown_seconds: 999.0,
            vlad_r_base_damage: 0.0,
            vlad_r_ap_ratio: 0.0,
            vlad_r_base_cooldown_seconds: 999.0,
        }
    }

    fn test_urf() -> UrfBuffs {
        UrfBuffs {
            ability_haste: 0.0,
            item_haste: 0.0,
            health_cost_multiplier: 1.0,
            bonus_attack_speed_multiplier_melee: 1.0,
            bonus_attack_speed_multiplier_ranged: 1.0,
            allowed_item_keys: Default::default(),
        }
    }

    #[test]
    fn controlled_champion_loadout_runtime_increases_spell_damage_when_selected() {
        let base = test_controlled_champion_base();
        let enemy = test_enemy("Target Dummy", 0.0);
        let enemies = vec![(enemy, Vec::new(), Stats::default())];
        let bonus_stats = Stats {
            ability_power: 250.0,
            ..Stats::default()
        };
        let sim = test_simulation(4.0, 200.0);
        let urf = test_urf();
        let outcome_without_runtime = simulate_controlled_champion_combat(
            &base,
            &[],
            &bonus_stats,
            None,
            None,
            None,
            &enemies,
            &sim,
            &urf,
        );
        let arcane_comet_selection = LoadoutSelection {
            rune_names: vec!["Arcane Comet".to_string()],
            shard_stats: Vec::new(),
        };
        let outcome_with_runtime = simulate_controlled_champion_combat(
            &base,
            &[],
            &bonus_stats,
            Some(&arcane_comet_selection),
            None,
            None,
            &enemies,
            &sim,
            &urf,
        );
        assert!(outcome_with_runtime.damage_dealt > outcome_without_runtime.damage_dealt);
    }

    #[test]
    fn controlled_champion_second_wind_runtime_adds_regeneration_ticks() {
        let base = test_controlled_champion_base();
        let enemy = test_enemy("Sona", 120.0);
        let enemies = vec![(enemy, Vec::new(), Stats::default())];
        let sim = test_simulation(12.0, 0.0);
        let urf = test_urf();
        let outcome_without_runtime = simulate_controlled_champion_combat(
            &base,
            &[],
            &Stats::default(),
            None,
            None,
            None,
            &enemies,
            &sim,
            &urf,
        );
        let second_wind_selection = LoadoutSelection {
            rune_names: vec!["Second Wind".to_string()],
            shard_stats: Vec::new(),
        };
        let outcome_with_runtime = simulate_controlled_champion_combat(
            &base,
            &[],
            &Stats::default(),
            Some(&second_wind_selection),
            None,
            None,
            &enemies,
            &sim,
            &urf,
        );
        assert!(outcome_with_runtime.healing_done > outcome_without_runtime.healing_done);
        assert!(
            outcome_with_runtime.time_alive_seconds >= outcome_without_runtime.time_alive_seconds
        );
    }

    #[test]
    fn melee_attack_is_cancelled_when_attacker_is_stunned_during_windup() {
        let controlled_champion = test_controlled_champion_base();
        let enemy = test_enemy_with_role("Melee Tester", 0.0, true);
        let enemies = vec![(enemy, Vec::new(), Stats::default())];
        let simulation = test_simulation(2.0, 0.0);
        let urf = test_urf();

        let mut runner = ControlledChampionCombatSimulation::new(
            controlled_champion,
            &[],
            &Stats::default(),
            None,
            None,
            &enemies,
            simulation,
            urf,
        );
        runner.enable_trace();
        runner.schedule_event(0.0, 30, EventType::Attack(0), None);
        let _ = runner.step(1);
        runner.enemy_state[0].stunned_until = runner.current_time() + 1.0;
        for _ in 0..30 {
            let _ = runner.step(1);
        }
        assert_eq!(runner.current_health(), runner.max_health);
        assert!(
            runner
                .trace_events()
                .iter()
                .any(|entry| entry.contains("cancelled during windup"))
        );
    }

    #[test]
    fn projectile_impact_on_stasis_is_nullified() {
        let controlled_champion = test_controlled_champion_base();
        let enemy = test_enemy("Sona", 0.0);
        let enemies = vec![(enemy, Vec::new(), Stats::default())];
        let simulation = test_simulation(1.0, 0.0);
        let urf = test_urf();

        let mut runner = ControlledChampionCombatSimulation::new(
            controlled_champion,
            &[],
            &Stats::default(),
            None,
            None,
            &enemies,
            simulation,
            urf,
        );
        runner.enable_trace();
        runner.schedule_event(0.0, 30, EventType::Attack(0), None);
        for _ in 0..8 {
            let _ = runner.step(1);
        }
        let health_before_stasis = runner.current_health();
        runner.stasis_until = runner.current_time() + 0.8;
        for _ in 0..16 {
            let _ = runner.step(1);
        }
        assert_eq!(runner.current_health(), health_before_stasis);
        assert!(
            runner
                .trace_events()
                .iter()
                .any(|entry| entry.contains("[impact_nullified]"))
        );
    }

    #[test]
    fn enemy_cannot_auto_attack_or_cast_while_invulnerable() {
        let controlled_champion = test_controlled_champion_base();
        let mut enemy = test_enemy("Sona", 120.0);
        enemy.ability_tick_interval_seconds = 0.25;
        let enemies = vec![(enemy, Vec::new(), Stats::default())];
        let simulation = test_simulation(1.0, 0.0);
        let urf = test_urf();

        let mut runner = ControlledChampionCombatSimulation::new(
            controlled_champion,
            &[],
            &Stats::default(),
            None,
            None,
            &enemies,
            simulation,
            urf,
        );
        runner.enable_trace();
        runner.enemy_state[0].invulnerable_until = runner.current_time() + 0.8;
        runner.schedule_event(0.0, 30, EventType::Attack(0), None);
        runner.schedule_event(0.0, 40, EventType::Ability(0), None);
        for _ in 0..24 {
            let _ = runner.step(1);
        }
        assert_eq!(runner.current_health(), runner.max_health);
        assert!(
            !runner
                .trace_events()
                .iter()
                .any(|entry| entry.contains("begins auto attack"))
        );
        assert!(
            !runner
                .trace_events()
                .iter()
                .any(|entry| entry.contains("ability hit"))
        );
    }
}

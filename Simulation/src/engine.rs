use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::defaults::{
    champion_ai_profile, champion_hitbox_radius, protoplasm_lifeline_cooldown_seconds_default,
    simulator_defaults,
};
use crate::scripts::champions::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionRuneProcTelemetryEntry,
    ChampionScriptAction, ChampionScriptEvent, ChampionScriptExecutionInput, ChampionScriptPoint,
    ControlledChampionAbilityCooldowns, ControlledChampionCastProfile,
    ControlledChampionDefensiveAbilityDecisionInput, ControlledChampionDefensiveAbilityTwoConfig,
    ControlledChampionOffensiveAbility, ControlledChampionOffensiveDecisionInput,
    ControlledChampionScriptHandle, ControlledChampionTargetSnapshot, attack_speed_multiplier,
    behavior_profile, build_champion_loadout_runtime, champion_script_event_cooldown_seconds,
    champion_script_event_label, clear_transient_combat_state,
    controlled_champion_default_cast_profile, controlled_champion_defensive_ability_two_config,
    controlled_champion_defensive_ability_two_raw_damage, controlled_champion_offensive_ap_ratio,
    controlled_champion_offensive_cooldowns_after_haste,
    controlled_champion_offensive_primary_heal_ratio, controlled_champion_offensive_raw_damage,
    controlled_champion_script_enabled, decide_controlled_champion_defensive_ability_activations,
    decide_controlled_champion_offensive_casts, describe_rune_proc_telemetry,
    describe_runtime_effect_cooldowns, describe_runtime_effect_stacks, enemy_kill_heal,
    execute_champion_script_event, incoming_damage_multipliers,
    initialize_controlled_champion_ability_slots, movement_speed_multiplier,
    on_ability_bonus_damage, on_hit_bonus_damage, on_immobilize_rune_damage, outgoing_damage_heal,
    scripted_champion_events, tick_regen_heal,
};
use crate::scripts::items::hooks::controlled_champion_defensive_item_capabilities;
use crate::scripts::runtime::ability_slots::{
    ActorAbilityLoadout, default_champion_ability_loadout,
};
use crate::scripts::runtime::controlled_champion_loadout::{
    DefensiveItemActivationInput, ReviveEffectDecisionInput,
    controlled_champion_damage_taken_multiplier, controlled_champion_heal_multiplier,
    decide_defensive_item_activations, describe_controlled_champion_runtime_cooldowns,
    should_trigger_revive_effect,
};
use crate::scripts::runtime::stat_resolution::{
    CooldownMetricSource, RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
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

fn hitbox_miss_reason(
    source: Vec2,
    aim_point: Vec2,
    target_center: Vec2,
    target_hitbox_radius: f64,
    effect_hitbox_radius: f64,
) -> String {
    let reach = target_hitbox_radius.max(0.0) + effect_hitbox_radius.max(0.0);
    let path_distance = if source.distance_to(aim_point) <= 1e-9 {
        source.distance_to(target_center)
    } else {
        distance_point_to_segment(target_center, source, aim_point)
    };
    format!(
        "target outside hitbox path (distance {:.1} > reach {:.1})",
        path_distance, reach
    )
}

struct EnemyState {
    enemy: EnemyConfig,
    movement_mode: OpponentMovementMode,
    behavior: ChampionBehaviorProfile,
    runtime: ChampionLoadoutRuntime,
    runtime_item_names: Vec<String>,
    runtime_rune_names: Vec<String>,
    position: Vec2,
    spawn_position: Vec2,
    move_speed: f64,
    base_attack_speed: f64,
    ability_haste: f64,
    physical_hit_damage: f64,
    ability_power: f64,
    armor: f64,
    magic_resist: f64,
    next_attack_bonus_physical: f64,
    next_attack_bonus_magic: f64,
    next_attack_bonus_true: f64,
    max_health: f64,
    health: f64,
    physical_multiplier: f64,
    magic_multiplier: f64,
    respawn_at: Option<f64>,
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
    runtime: ChampionLoadoutRuntime,
    runtime_item_names: Vec<String>,
    runtime_rune_names: Vec<String>,
    max_health: f64,
    armor: f64,
    magic_resist: f64,
    physical_multiplier: f64,
    magic_multiplier: f64,
    attack_damage: f64,
    ability_power: f64,
    ability_haste: f64,
    attack_speed: f64,
    attack_interval: f64,
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
    IgnoredTargetUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DamageApplicationOutcome {
    Applied,
    NullifiedUntargetable,
    Ignored,
}

#[derive(Debug, Clone)]
struct DamageSourceContext {
    champion_name: String,
    ability_name: String,
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
    controlled_champion_base: ChampionBase,
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

    controlled_champion_stats: Stats,
    controlled_champion_buffs: RuntimeBuffState,
    controlled_champion_combat_runtime: ChampionLoadoutRuntime,
    controlled_champion_behavior: ChampionBehaviorProfile,
    controlled_champion_base_attack_speed: f64,
    controlled_champion_attack_sequence: u64,
    controlled_champion_name: String,
    controlled_champion_item_names: Vec<String>,
    controlled_champion_rune_names: Vec<String>,
    controlled_champion_shard_names: Vec<String>,
    controlled_champion_hitbox_radius: f64,
    max_health: f64,
    health: f64,

    physical_multiplier: f64,
    magic_multiplier: f64,

    controlled_champion_script: Option<ControlledChampionScriptHandle>,
    pool_cooldown: f64,
    pool_duration: f64,
    pool_effect_range: f64,
    pool_damage_tick_interval_seconds: f64,
    controlled_champion_defensive_ability_two_cost_percent_current_health: f64,
    controlled_champion_defensive_ability_two_heal_ratio_of_damage: f64,
    controlled_champion_defensive_ability_two_damage_per_tick: f64,
    controlled_champion_defensive_ability_two_damage_per_tick_bonus_health_ratio: f64,
    offensive_cooldowns: ControlledChampionAbilityCooldowns,
    cast_profile: ControlledChampionCastProfile,
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
    pool_damage_until: f64,
    pool_next_damage_tick_at: f64,
    stasis_until: f64,
    revive_lockout_until: f64,
    stunned_until: f64,
    combat_primitives: CombatPrimitivesState,

    emergency_shield_amount: f64,
    emergency_heal_rate: f64,
    emergency_heal_until: f64,

    target_position: Vec2,
    enemy_state: Vec<EnemyState>,
    projectile_block_zones: Vec<ProjectileBlockZone>,
    trace_enabled: bool,
    trace_events: Vec<String>,
    trace_snapshot_interval_seconds: f64,
    trace_next_snapshot_at: f64,
}

impl ControlledChampionCombatSimulation {
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        controlled_champion_base: ChampionBase,
        controlled_champion_build_items: &[Item],
        controlled_champion_bonus_stats: &Stats,
        controlled_champion_item_acquired_levels: Option<&HashMap<String, usize>>,
        controlled_champion_stack_overrides: Option<&HashMap<String, f64>>,
        enemies: &[(EnemyConfig, Vec<Item>, Stats)],
        sim: SimulationConfig,
        urf: UrfBuffs,
    ) -> Self {
        Self::new_with_controlled_champion_loadout(
            controlled_champion_base,
            controlled_champion_build_items,
            controlled_champion_bonus_stats,
            None,
            controlled_champion_item_acquired_levels,
            controlled_champion_stack_overrides,
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
        let controlled_champion_name = controlled_champion_base.name.clone();
        let controlled_champion_item_names = controlled_champion_build_items
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>();
        let (controlled_champion_rune_names, controlled_champion_shard_names) =
            controlled_champion_loadout_selection
                .map(|selection| (selection.rune_names.clone(), selection.shard_stats.clone()))
                .unwrap_or_default();
        let controlled_champion_hitbox_radius =
            champion_hitbox_radius(&controlled_champion_base.name);
        let mut controlled_champion_item_stats = Stats::default();
        for item in controlled_champion_build_items {
            controlled_champion_item_stats.add(&item.stats);
        }
        controlled_champion_item_stats.add(controlled_champion_bonus_stats);
        apply_item_assumptions(
            &mut controlled_champion_item_stats,
            &controlled_champion_base,
            controlled_champion_build_items,
            &sim,
            sim.champion_level,
            controlled_champion_item_acquired_levels,
            controlled_champion_stack_overrides,
        );
        let controlled_champion_stats = compute_champion_final_stats(
            &controlled_champion_base,
            &controlled_champion_item_stats,
        );
        let controlled_champion_combat_runtime = build_champion_loadout_runtime(
            &controlled_champion_item_names,
            &controlled_champion_rune_names,
            urf.item_haste,
            controlled_champion_base.is_melee,
        );
        let controlled_champion_behavior = behavior_profile(
            &controlled_champion_name,
            controlled_champion_base.is_melee,
            controlled_champion_base.base_attack_range,
            controlled_champion_base.base_attack_projectile_speed,
        );
        let controlled_champion_attack_speed_bonus =
            (controlled_champion_stats.attack_speed_percent / 100.0).max(-0.99);
        let mut controlled_champion_base_attack_speed = controlled_champion_base.base_attack_speed
            * (1.0 + controlled_champion_attack_speed_bonus);
        controlled_champion_base_attack_speed *= if controlled_champion_base.is_melee {
            urf.bonus_attack_speed_multiplier_melee
        } else {
            urf.bonus_attack_speed_multiplier_ranged
        };
        controlled_champion_base_attack_speed = controlled_champion_base_attack_speed.max(0.001);

        let max_health = controlled_champion_stats.health;
        let physical_multiplier = 100.0 / (100.0 + controlled_champion_stats.armor.max(0.0));
        let magic_multiplier = 100.0 / (100.0 + controlled_champion_stats.magic_resist.max(0.0));

        let ability_haste = controlled_champion_item_stats.ability_haste + urf.ability_haste;
        let runtime_buffs = RuntimeBuffState {
            ability_haste,
            item_haste: urf.item_haste,
            cooldown_rate_multiplier: 1.0,
            incoming_damage_taken_multiplier: 1.0,
            healing_multiplier: 1.0,
            movement_speed_multiplier: 1.0,
            outgoing_ability_damage_multiplier: 1.0,
        };
        let controlled_champion_script = sim.controlled_champion_script.clone();
        let controlled_champion_defensive_ability_two =
            controlled_champion_defensive_ability_two_config(
                controlled_champion_script.as_ref(),
                ability_haste,
            );
        let offensive_cooldowns = controlled_champion_offensive_cooldowns_after_haste(
            controlled_champion_script.as_ref(),
            ability_haste,
        );
        let cast_profile =
            controlled_champion_default_cast_profile(controlled_champion_script.as_ref());
        let mut controlled_champion_ability_loadout =
            default_champion_ability_loadout(&controlled_champion_name);
        let mut controlled_champion_ability_ready_at = HashMap::new();
        initialize_controlled_champion_ability_slots(
            controlled_champion_script.as_ref(),
            &cast_profile,
            &mut controlled_champion_ability_loadout,
            &mut controlled_champion_ability_ready_at,
        );

        let defensive_item_capabilities =
            controlled_champion_defensive_item_capabilities(controlled_champion_build_items);
        let stasis_item_available = defensive_item_capabilities.has_stasis_item;
        let revive_item_available = defensive_item_capabilities.has_revive_item;
        let emergency_shield_item_available = defensive_item_capabilities.has_emergency_shield_item;

        let revive_item_cooldown_seconds = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: sim.ga_cooldown_seconds,
                source: CooldownMetricSource::Item,
            },
            runtime_buffs,
        );
        let stasis_item_cooldown_seconds = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: sim.zhonya_cooldown_seconds,
                source: CooldownMetricSource::Item,
            },
            runtime_buffs,
        );
        let emergency_shield_item_cooldown_seconds = resolve_stat(
            StatQuery::CooldownSeconds {
                base_seconds: protoplasm_lifeline_cooldown_seconds_default(),
                source: CooldownMetricSource::Item,
            },
            runtime_buffs,
        );

        let tick_seconds = if sim.server_tick_rate_hz > 0.0 {
            1.0 / sim.server_tick_rate_hz
        } else {
            sim.dt
        };
        let mut seeded_combat_rng = sim.combat_seed.map(|seed| seed ^ 0xC0B4_7EAF_D15C_A11E);

        let mut runner = Self {
            controlled_champion_base,
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
            controlled_champion_stats,
            controlled_champion_buffs: runtime_buffs,
            controlled_champion_combat_runtime,
            controlled_champion_behavior,
            controlled_champion_base_attack_speed,
            controlled_champion_attack_sequence: 0,
            controlled_champion_name,
            controlled_champion_item_names,
            controlled_champion_rune_names,
            controlled_champion_shard_names,
            controlled_champion_hitbox_radius,
            max_health,
            health: max_health,
            physical_multiplier,
            magic_multiplier,
            controlled_champion_script,
            pool_cooldown: controlled_champion_defensive_ability_two.cooldown_seconds,
            pool_duration: controlled_champion_defensive_ability_two.duration_seconds,
            pool_effect_range: controlled_champion_defensive_ability_two.effect_range,
            pool_damage_tick_interval_seconds: controlled_champion_defensive_ability_two
                .damage_tick_interval_seconds,
            controlled_champion_defensive_ability_two_cost_percent_current_health:
                controlled_champion_defensive_ability_two.cost_percent_current_health,
            controlled_champion_defensive_ability_two_heal_ratio_of_damage:
                controlled_champion_defensive_ability_two.heal_ratio_of_damage,
            controlled_champion_defensive_ability_two_damage_per_tick:
                controlled_champion_defensive_ability_two.damage_per_tick,
            controlled_champion_defensive_ability_two_damage_per_tick_bonus_health_ratio:
                controlled_champion_defensive_ability_two.damage_per_tick_bonus_health_ratio,
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
            pool_damage_until: 0.0,
            pool_next_damage_tick_at: f64::INFINITY,
            stasis_until: 0.0,
            revive_lockout_until: 0.0,
            stunned_until: 0.0,
            combat_primitives: CombatPrimitivesState::default(),
            emergency_shield_amount: 0.0,
            emergency_heal_rate: 0.0,
            emergency_heal_until: 0.0,
            target_position: Vec2 { x: 0.0, y: 0.0 },
            enemy_state: Vec::new(),
            projectile_block_zones: Vec::new(),
            trace_enabled: false,
            trace_events: Vec::new(),
            trace_snapshot_interval_seconds: 5.0,
            trace_next_snapshot_at: 0.0,
        };

        let mut enemy_entries = enemies.to_vec();
        if let Some(seed) = seeded_combat_rng.as_mut() {
            let mut order = (0..enemy_entries.len()).collect::<Vec<_>>();
            shuffle_usize(&mut order, seed);
            enemy_entries = order
                .into_iter()
                .map(|original_idx| enemy_entries[original_idx].clone())
                .collect::<Vec<_>>();
        }

        let enemy_count = enemy_entries.len();
        for (idx, (enemy, build, enemy_bonus)) in enemy_entries.into_iter().enumerate() {
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
                runtime: model.runtime,
                runtime_item_names: model.runtime_item_names,
                runtime_rune_names: model.runtime_rune_names,
                position,
                spawn_position: position,
                move_speed: model.move_speed,
                base_attack_speed: model.attack_speed.max(0.001),
                ability_haste: model.ability_haste,
                physical_hit_damage: model.attack_damage,
                ability_power: model.ability_power,
                armor: model.armor,
                magic_resist: model.magic_resist,
                next_attack_bonus_physical: 0.0,
                next_attack_bonus_magic: 0.0,
                next_attack_bonus_true: 0.0,
                max_health: model.max_health,
                health: model.max_health,
                physical_multiplier: model.physical_multiplier,
                magic_multiplier: model.magic_multiplier,
                respawn_at: None,
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

            let attack_delay_jitter = seeded_combat_rng
                .as_mut()
                .map(|seed| rand_f64(seed) * runner.tick_seconds)
                .unwrap_or(0.0);
            runner.schedule_event(
                model.attack_interval + attack_delay_jitter,
                30,
                EventType::Attack(idx),
                None,
            );
            for event in scripted_champion_events(&enemy.name) {
                runner.schedule_event(
                    0.0,
                    12,
                    EventType::ChampionScript(idx, event, 0),
                    Some(script_poll_interval_seconds),
                );
            }
        }

        runner.schedule_event(
            runner.controlled_champion_attack_interval_seconds(),
            31,
            EventType::ControlledChampionAttack,
            None,
        );

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

    fn trace_cooldown_status(now: f64, ready_at: f64) -> String {
        let remaining = (ready_at - now).max(0.0);
        if remaining <= 1e-9 {
            "ready".to_string()
        } else {
            format!("{remaining:.2}s")
        }
    }

    fn status_effect_kind_label(kind: &StatusEffectKind) -> String {
        match kind {
            StatusEffectKind::Stun => "Stun".to_string(),
            StatusEffectKind::Silence => "Silence".to_string(),
            StatusEffectKind::Root => "Root".to_string(),
            StatusEffectKind::Slow => "Slow".to_string(),
            StatusEffectKind::Untargetable => "Untargetable".to_string(),
            StatusEffectKind::Stasis => "Stasis".to_string(),
            StatusEffectKind::Custom(name) => (*name).to_string(),
        }
    }

    fn status_effect_summary(effect: &StatusEffect) -> String {
        let duration = match effect.duration {
            StatusDuration::Timed { remaining_seconds } => {
                let remaining = remaining_seconds.max(0.0);
                if remaining <= 1e-9 {
                    "expired".to_string()
                } else {
                    format!("{remaining:.2}s")
                }
            }
            StatusDuration::Persistent => "persistent".to_string(),
        };
        format!(
            "{} x{} ({})",
            Self::status_effect_kind_label(&effect.kind),
            effect.stacks,
            duration
        )
    }

    fn controlled_champion_status_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if self.time < self.stunned_until {
            lines.push(format!(
                "Stunned {:.2}s",
                (self.stunned_until - self.time).max(0.0)
            ));
        }
        if self.time < self.pool_until {
            lines.push(format!(
                "Pool untargetable {:.2}s",
                (self.pool_until - self.time).max(0.0)
            ));
        }
        if self.time < self.stasis_until {
            lines.push(format!(
                "Stasis {:.2}s",
                (self.stasis_until - self.time).max(0.0)
            ));
        }
        if self.time < self.revive_lockout_until {
            lines.push(format!(
                "Revive lockout {:.2}s",
                (self.revive_lockout_until - self.time).max(0.0)
            ));
        }
        if self.pool_damage_until > self.time {
            lines.push(format!(
                "Pool damage-over-time {:.2}s",
                (self.pool_damage_until - self.time).max(0.0)
            ));
        }
        if self.emergency_heal_until > self.time {
            lines.push(format!(
                "Emergency heal-over-time {:.2}s",
                (self.emergency_heal_until - self.time).max(0.0)
            ));
        }
        if self.emergency_shield_amount > 0.0 {
            lines.push(format!(
                "Emergency shield {:.1}",
                self.emergency_shield_amount
            ));
        }
        lines.extend(
            self.combat_primitives
                .status_effects()
                .effects()
                .iter()
                .map(Self::status_effect_summary),
        );
        if lines.is_empty() {
            lines.push("none".to_string());
        }
        lines
    }

    fn enemy_status_lines(&self, idx: usize) -> Vec<String> {
        let Some(state) = self.enemy_state.get(idx) else {
            return vec!["none".to_string()];
        };
        let mut lines = Vec::new();
        if let Some(respawn_at) = state.respawn_at {
            lines.push(format!(
                "Respawning in {:.2}s",
                (respawn_at - self.time).max(0.0)
            ));
        }
        if self.time < state.stunned_until {
            lines.push(format!(
                "Stunned {:.2}s",
                (state.stunned_until - self.time).max(0.0)
            ));
        }
        if self.time < state.untargetable_until {
            lines.push(format!(
                "Untargetable {:.2}s",
                (state.untargetable_until - self.time).max(0.0)
            ));
        }
        if self.time < state.stasis_until {
            lines.push(format!(
                "Stasis {:.2}s",
                (state.stasis_until - self.time).max(0.0)
            ));
        }
        if self.time < state.invulnerable_until {
            lines.push(format!(
                "Invulnerable {:.2}s",
                (state.invulnerable_until - self.time).max(0.0)
            ));
        }
        if lines.is_empty() {
            lines.push("none".to_string());
        }
        lines
    }

    fn enemy_next_attack_ready_at(&self, idx: usize) -> Option<f64> {
        self.event_queue
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::Attack(event_idx) if *event_idx == idx => Some(queued.time),
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    fn enemy_next_attack_impact_at(&self, idx: usize) -> Option<f64> {
        self.event_queue
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::AttackHit { idx: event_idx, .. } if *event_idx == idx => {
                    Some(queued.time)
                }
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    fn controlled_champion_next_attack_ready_at(&self) -> Option<f64> {
        self.event_queue
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::ControlledChampionAttack => Some(queued.time),
                _ => None,
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    fn controlled_champion_next_attack_impact_at(&self) -> Option<(usize, f64)> {
        self.event_queue
            .iter()
            .filter_map(|queued| match &queued.kind {
                EventType::ControlledChampionAttackHit { idx, .. } => Some((*idx, queued.time)),
                _ => None,
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal))
    }

    fn queued_projectile_lines(&self) -> Vec<String> {
        let mut entries = Vec::<(f64, String)>::new();
        for queued in &self.event_queue {
            if queued.time + 1e-9 < self.time {
                continue;
            }
            match &queued.kind {
                EventType::AttackHit { idx, .. } => {
                    if let Some(state) = self.enemy_state.get(*idx) {
                        entries.push((
                            queued.time,
                            format!(
                                "{} Auto Attack -> {} (impact in {:.2}s)",
                                state.enemy.name,
                                self.controlled_champion_name,
                                (queued.time - self.time).max(0.0)
                            ),
                        ));
                    }
                }
                EventType::ControlledChampionOffensivePrimaryHit { idx, .. } => {
                    if let Some(state) = self.enemy_state.get(*idx) {
                        entries.push((
                            queued.time,
                            format!(
                                "{} {} -> {} (impact in {:.2}s)",
                                self.controlled_champion_name,
                                self.cast_profile.offensive_primary_ability_id,
                                state.enemy.name,
                                (queued.time - self.time).max(0.0)
                            ),
                        ));
                    }
                }
                EventType::ControlledChampionAttackHit { idx, .. } => {
                    if let Some(state) = self.enemy_state.get(*idx) {
                        entries.push((
                            queued.time,
                            format!(
                                "{} Auto Attack -> {} (impact in {:.2}s)",
                                self.controlled_champion_name,
                                state.enemy.name,
                                (queued.time - self.time).max(0.0)
                            ),
                        ));
                    }
                }
                _ => {}
            }
        }
        entries.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
        if entries.is_empty() {
            return vec!["none".to_string()];
        }
        entries.into_iter().map(|(_, line)| line).collect()
    }

    fn collect_state_snapshot_summary(&self, checkpoint_seconds: f64) -> String {
        fn list_or_none(values: &[String]) -> String {
            if values.is_empty() {
                "none".to_string()
            } else {
                values.join(", ")
            }
        }

        fn join_or_none(values: &[String], separator: &str) -> String {
            if values.is_empty() {
                "none".to_string()
            } else {
                values.join(separator)
            }
        }

        let health_ratio = if self.max_health > 0.0 {
            (self.health / self.max_health).clamp(0.0, 1.0) * 100.0
        } else {
            0.0
        };

        let mut controlled_champion_cooldowns = Vec::new();
        if self.stasis_item_available {
            controlled_champion_cooldowns.push(format!(
                "Stasis item {}",
                Self::trace_cooldown_status(self.time, self.stasis_item_ready_at)
            ));
        }
        if self.revive_item_available {
            controlled_champion_cooldowns.push(format!(
                "Revive item {}",
                Self::trace_cooldown_status(self.time, self.revive_item_ready_at)
            ));
        }
        if self.emergency_shield_item_available {
            controlled_champion_cooldowns.push(format!(
                "Emergency shield item {}",
                Self::trace_cooldown_status(self.time, self.emergency_shield_item_ready_at)
            ));
        }
        let runtime_controlled_champion_cooldowns =
            describe_controlled_champion_runtime_cooldowns(self.time);
        let runtime_cooldowns_are_none = runtime_controlled_champion_cooldowns.len() == 1
            && runtime_controlled_champion_cooldowns[0] == "none";
        if !runtime_cooldowns_are_none {
            controlled_champion_cooldowns.extend(runtime_controlled_champion_cooldowns);
        }
        if controlled_champion_cooldowns.is_empty() {
            controlled_champion_cooldowns.push("none".to_string());
        }

        let mut controlled_champion_abilities = self
            .controlled_champion_ability_loadout
            .slot_bindings()
            .into_iter()
            .map(|(slot, ability_id)| {
                format!(
                    "{}:{} {}",
                    slot.label(),
                    ability_id,
                    Self::trace_cooldown_status(
                        self.time,
                        self.controlled_champion_ability_ready_at(ability_id)
                    )
                )
            })
            .collect::<Vec<_>>();
        if let Some((idx, impact_at)) = self.controlled_champion_next_attack_impact_at() {
            let target_name = self
                .enemy_state
                .get(idx)
                .map(|state| state.enemy.name.as_str())
                .unwrap_or("target");
            controlled_champion_abilities.push(format!(
                "Auto Attack in-flight -> {} ({:.2}s to impact)",
                target_name,
                (impact_at - self.time).max(0.0)
            ));
        } else if let Some(next_attack_ready_at) = self.controlled_champion_next_attack_ready_at() {
            controlled_champion_abilities.push(format!(
                "Auto Attack {}",
                Self::trace_cooldown_status(self.time, next_attack_ready_at)
            ));
        } else {
            controlled_champion_abilities.push("Auto Attack unavailable".to_string());
        }
        let controlled_runtime_effect_cooldowns =
            describe_runtime_effect_cooldowns(&self.controlled_champion_combat_runtime, self.time);
        let controlled_runtime_effect_stacks =
            describe_runtime_effect_stacks(&self.controlled_champion_combat_runtime);
        let controlled_champion_buffs = self.controlled_champion_status_lines();

        let mut lines = Vec::new();
        lines.push(format!(
            "checkpoint {:.1}s (captured_at {:.3}s)",
            checkpoint_seconds, self.time
        ));
        lines.push("controlled_champion:".to_string());
        lines.push(format!("  identity: {}", self.controlled_champion_name));
        lines.push(format!(
            "  core: pos=({:.1}, {:.1}) hp={:.1}/{:.1} ({:.1}%) armor={:.1} mr={:.1}",
            self.target_position.x,
            self.target_position.y,
            self.health.max(0.0),
            self.max_health,
            health_ratio,
            self.controlled_champion_stats.armor,
            self.controlled_champion_stats.magic_resist
        ));
        lines.push(format!(
            "  offense: ap={:.1} ah={:.1}",
            self.controlled_champion_stats.ability_power,
            self.controlled_champion_buffs.ability_haste
        ));
        lines.push(format!(
            "  loadout: items [{}] | runes [{}] | shards [{}]",
            list_or_none(&self.controlled_champion_item_names),
            list_or_none(&self.controlled_champion_rune_names),
            list_or_none(&self.controlled_champion_shard_names)
        ));
        lines.push(format!(
            "  cooldowns: {}",
            join_or_none(&controlled_champion_cooldowns, "; ")
        ));
        lines.push(format!(
            "  abilities: {}",
            join_or_none(&controlled_champion_abilities, "; ")
        ));
        lines.push(format!(
            "  runtime: cooldowns [{}] | stacks [{}]",
            join_or_none(&controlled_runtime_effect_cooldowns, "; "),
            join_or_none(&controlled_runtime_effect_stacks, "; ")
        ));
        lines.push(format!(
            "  buffs: {}",
            join_or_none(&controlled_champion_buffs, "; ")
        ));

        if self.enemy_state.is_empty() {
            lines.push("enemies: none".to_string());
        } else {
            lines.push("enemies:".to_string());
            for (idx, state) in self.enemy_state.iter().enumerate() {
                let attack_speed =
                    state.base_attack_speed * attack_speed_multiplier(&state.runtime, self.time);
                let attack_interval = 1.0 / attack_speed.max(0.001);

                let mut enemy_abilities = Vec::new();
                if let Some(impact_at) = self.enemy_next_attack_impact_at(idx) {
                    enemy_abilities.push(format!(
                        "Auto Attack in-flight ({:.2}s to impact)",
                        (impact_at - self.time).max(0.0)
                    ));
                } else if let Some(next_attack_ready_at) = self.enemy_next_attack_ready_at(idx) {
                    enemy_abilities.push(format!(
                        "Auto Attack {}",
                        Self::trace_cooldown_status(self.time, next_attack_ready_at)
                    ));
                } else {
                    enemy_abilities.push("Auto Attack unavailable".to_string());
                }
                for event in scripted_champion_events(&state.enemy.name) {
                    let ready_at = state
                        .script_event_ready_at
                        .get(&event)
                        .copied()
                        .unwrap_or(0.0);
                    enemy_abilities.push(format!(
                        "{} {}",
                        champion_script_event_label(event),
                        Self::trace_cooldown_status(self.time, ready_at)
                    ));
                }

                let runtime_cooldowns =
                    describe_runtime_effect_cooldowns(&state.runtime, self.time);
                let runtime_stacks = describe_runtime_effect_stacks(&state.runtime);
                let enemy_buffs = self.enemy_status_lines(idx);

                lines.push(format!("  {}:", state.enemy.name));
                lines.push(format!(
                    "    core: pos=({:.1}, {:.1}) hp={:.1}/{:.1} armor={:.1} mr={:.1}",
                    state.position.x,
                    state.position.y,
                    state.health.max(0.0),
                    state.max_health,
                    state.armor,
                    state.magic_resist
                ));
                lines.push(format!(
                    "    combat: ad={:.1} ap={:.1} as={:.3} (interval {:.3}s) ah={:.1}",
                    state.physical_hit_damage,
                    state.ability_power,
                    attack_speed,
                    attack_interval,
                    state.ability_haste
                ));
                lines.push(format!(
                    "    loadout: items [{}] | runes [{}]",
                    list_or_none(&state.runtime_item_names),
                    list_or_none(&state.runtime_rune_names)
                ));
                lines.push(format!(
                    "    abilities: {}",
                    join_or_none(&enemy_abilities, "; ")
                ));
                lines.push(format!(
                    "    runtime: cooldowns [{}] | stacks [{}]",
                    join_or_none(&runtime_cooldowns, "; "),
                    join_or_none(&runtime_stacks, "; ")
                ));
                lines.push(format!("    buffs: {}", join_or_none(&enemy_buffs, "; ")));
            }
        }

        lines.push("field:".to_string());
        let projectile_lines = self.queued_projectile_lines();
        if projectile_lines.len() == 1 && projectile_lines[0] == "none" {
            lines.push("  projectiles: none".to_string());
        } else {
            lines.push("  projectiles:".to_string());
            for projectile in projectile_lines {
                lines.push(format!("    - {projectile}"));
            }
        }
        if self.projectile_block_zones.is_empty() {
            lines.push("  projectile_block_zones: none".to_string());
        } else {
            lines.push("  projectile_block_zones:".to_string());
            for (idx, zone) in self.projectile_block_zones.iter().enumerate() {
                lines.push(format!(
                    "    - zone {}: start=({:.1}, {:.1}) end=({:.1}, {:.1}) width={:.1} expires_in={:.2}s",
                    idx + 1,
                    zone.start.x,
                    zone.start.y,
                    zone.end.x,
                    zone.end.y,
                    zone.half_width * 2.0,
                    (zone.expires_at - self.time).max(0.0)
                ));
            }
        }

        lines.join("\n")
    }

    fn emit_trace_snapshot(&mut self, checkpoint_seconds: f64) {
        if !self.trace_enabled {
            return;
        }
        let snapshot = self.collect_state_snapshot_summary(checkpoint_seconds);
        self.trace_event("state_snapshot", snapshot);
    }

    fn emit_trace_snapshots_due(&mut self) {
        if !self.trace_enabled {
            return;
        }
        let interval = self.trace_snapshot_interval_seconds.max(0.1);
        while self.time + 1e-9 >= self.trace_next_snapshot_at {
            let checkpoint = self.trace_next_snapshot_at;
            self.emit_trace_snapshot(checkpoint);
            self.trace_next_snapshot_at += interval;
        }
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
            && !self.combat_primitives.cast_lock().is_locked()
    }

    fn can_basic_attack(&self) -> bool {
        !self.controlled_champion_is_stunned()
            && !self.controlled_champion_is_invulnerable_or_untargetable()
            && !self.combat_primitives.cast_lock().is_locked()
    }

    fn controlled_champion_script_enabled(&self) -> bool {
        controlled_champion_script_enabled(self.controlled_champion_script.as_ref())
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

    fn enemy_respawn_delay_seconds(&self, enemy_level: usize) -> f64 {
        respawn::urf_respawn_delay_seconds(
            enemy_level,
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
        self.enemy_is_alive(idx)
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

    fn controlled_champion_attack_speed(&self) -> f64 {
        self.controlled_champion_base_attack_speed
            * attack_speed_multiplier(&self.controlled_champion_combat_runtime, self.time)
    }

    fn controlled_champion_attack_interval_seconds(&self) -> f64 {
        1.0 / self
            .controlled_champion_attack_speed()
            .max(simulator_defaults().engine_defaults.minimum_attack_speed)
    }

    fn controlled_champion_in_attack_range(&self, idx: usize) -> bool {
        let Some(state) = self.enemy_state.get(idx) else {
            return false;
        };
        within_reach_with_hitboxes(
            self.distance_to_target(idx),
            self.controlled_champion_behavior.attack_range,
            self.controlled_champion_hitbox_radius,
            state.hitbox_radius,
            self.controlled_champion_behavior
                .attack_effect_hitbox_radius,
        )
    }

    fn first_active_enemy_in_controlled_champion_attack_range(&self) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_is_active(idx) || !self.controlled_champion_in_attack_range(idx) {
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

    fn schedule_next_controlled_champion_attack(&mut self) {
        self.schedule_event(
            self.controlled_champion_attack_interval_seconds(),
            31,
            EventType::ControlledChampionAttack,
            None,
        );
    }

    fn schedule_next_attack(&mut self, idx: usize) {
        if idx >= self.enemy_state.len() {
            return;
        }
        let state = &self.enemy_state[idx];
        let attack_speed =
            state.base_attack_speed * attack_speed_multiplier(&state.runtime, self.time);
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
        script_event: ChampionScriptEvent,
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
                    let enemy_name = self.enemy_state[idx].enemy.name.clone();
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
                            match self.apply_damage(
                                DamageSourceContext {
                                    champion_name: enemy_name.clone(),
                                    ability_name: champion_script_event_label(script_event)
                                        .to_string(),
                                },
                                physical,
                                magic,
                                true_damage,
                            ) {
                                DamageApplicationOutcome::Applied => IncomingImpactOutcome::Applied,
                                DamageApplicationOutcome::NullifiedUntargetable => {
                                    IncomingImpactOutcome::NullifiedUntargetable
                                }
                                DamageApplicationOutcome::Ignored => {
                                    IncomingImpactOutcome::IgnoredTargetUnavailable
                                }
                            }
                        }
                    };
                    let mut aftershock_magic_damage = 0.0;
                    if stun_duration > 0.0 && outcome == IncomingImpactOutcome::Applied {
                        self.stunned_until = self.stunned_until.max(self.time + stun_duration);
                        self.apply_stun_window(stun_duration);
                        let (enemy_level, enemy_bonus_health) = {
                            let state = &self.enemy_state[idx];
                            (
                                state.enemy.level,
                                (state.max_health - state.enemy.base.base_health).max(0.0),
                            )
                        };
                        let state = &mut self.enemy_state[idx];
                        aftershock_magic_damage = on_immobilize_rune_damage(
                            &mut state.runtime,
                            self.time,
                            enemy_level,
                            enemy_bonus_health,
                        );
                    }
                    if aftershock_magic_damage > 0.0 {
                        match self.apply_damage(
                            DamageSourceContext {
                                champion_name: enemy_name.clone(),
                                ability_name: "Aftershock Shockwave".to_string(),
                            },
                            0.0,
                            aftershock_magic_damage,
                            0.0,
                        ) {
                            DamageApplicationOutcome::Applied => {
                                self.trace_event(
                                    "aftershock_hit",
                                    format!(
                                        "{} Aftershock shockwave dealt {:.1} magic damage",
                                        enemy_name, aftershock_magic_damage
                                    ),
                                );
                            }
                            DamageApplicationOutcome::NullifiedUntargetable => {
                                self.trace_event(
                                    "impact_nullified",
                                    format!(
                                        "{} Aftershock shockwave on {} was nullified by untargetable or stasis state",
                                        enemy_name, self.controlled_champion_name
                                    ),
                                );
                            }
                            DamageApplicationOutcome::Ignored => {
                                self.trace_event(
                                    "impact_ignored",
                                    format!(
                                        "{} Aftershock shockwave skipped because {} is unavailable",
                                        enemy_name, self.controlled_champion_name
                                    ),
                                );
                            }
                        }
                    }
                    match outcome {
                        IncomingImpactOutcome::Applied => {}
                        IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                            "projectile_blocked",
                            format!(
                                "{} {} projectile blocked by active projectile block zone",
                                enemy_name,
                                champion_script_event_label(script_event)
                            ),
                        ),
                        IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                            "impact_nullified",
                            format!(
                                "{} {} on {} was nullified by untargetable or stasis state",
                                enemy_name,
                                champion_script_event_label(script_event),
                                self.controlled_champion_name
                            ),
                        ),
                        IncomingImpactOutcome::MissedHitbox => self.trace_event(
                            "impact_missed",
                            format!(
                                "{} {} missed {} ({})",
                                enemy_name,
                                champion_script_event_label(script_event),
                                self.controlled_champion_name,
                                hitbox_miss_reason(
                                    source,
                                    self.target_position,
                                    self.target_position,
                                    self.controlled_champion_hitbox_radius,
                                    effect_hitbox_radius
                                )
                            ),
                        ),
                        IncomingImpactOutcome::IgnoredTargetUnavailable => self.trace_event(
                            "impact_ignored",
                            format!(
                                "{} {} skipped because {} is unavailable",
                                enemy_name,
                                champion_script_event_label(script_event),
                                self.controlled_champion_name
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

    fn apply_damage_to_enemy(
        &mut self,
        idx: usize,
        raw_physical_damage: f64,
        raw_magic_damage: f64,
        raw_true_damage: f64,
    ) -> f64 {
        if !self.enemy_is_active(idx) {
            return 0.0;
        }
        let (mitigated_physical, mitigated_magic, mitigated_true, enemy_level) = {
            let state = &self.enemy_state[idx];
            let bonus_armor = (state.armor - state.enemy.base.base_armor).max(0.0);
            let bonus_magic_resist =
                (state.magic_resist - state.enemy.base.base_magic_resist).max(0.0);
            let (aftershock_physical_multiplier, aftershock_magic_multiplier) =
                incoming_damage_multipliers(
                    &state.runtime,
                    self.time,
                    state.enemy.level,
                    state.armor,
                    state.magic_resist,
                    bonus_armor,
                    bonus_magic_resist,
                );
            (
                raw_physical_damage.max(0.0)
                    * state.physical_multiplier
                    * aftershock_physical_multiplier,
                raw_magic_damage.max(0.0) * state.magic_multiplier * aftershock_magic_multiplier,
                raw_true_damage.max(0.0),
                state.enemy.level,
            )
        };
        let mitigated = mitigated_physical + mitigated_magic + mitigated_true;
        if mitigated <= 0.0 {
            return 0.0;
        }
        let respawn_delay = self.enemy_respawn_delay_seconds(enemy_level);
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
            let runtime_kill_heal = enemy_kill_heal(
                &mut self.controlled_champion_combat_runtime,
                self.max_health,
            );
            if runtime_kill_heal > 0.0 {
                let script_heal_multiplier = controlled_champion_heal_multiplier();
                let resolved_heal = resolve_stat(
                    StatQuery::ScalarAmount {
                        base_amount: runtime_kill_heal * script_heal_multiplier,
                        source: ScalarMetricSource::Healing,
                        clamp_min_zero: true,
                    },
                    self.controlled_champion_buffs,
                );
                let before = self.health;
                self.health = self.max_health.min(self.health + resolved_heal);
                self.healing_done_total += (self.health - before).max(0.0);
            }
            self.trace_event(
                "enemy_death",
                format!("{} died; respawn in {:.1}s", name, respawn_delay),
            );
        }
        dealt
    }

    fn apply_magic_damage_to_enemy(&mut self, idx: usize, raw_magic_damage: f64) -> f64 {
        self.apply_damage_to_enemy(idx, 0.0, raw_magic_damage, 0.0)
    }

    fn apply_ability_bonus_damage_to_enemy(
        &mut self,
        idx: usize,
        ability_raw_damage: f64,
        ability_ap_ratio: f64,
        attacker_level: usize,
    ) -> f64 {
        if !self.enemy_is_active(idx) {
            return 0.0;
        }
        let target_current_health = self.enemy_state[idx].health.max(0.0);
        let target_max_health = self.enemy_state[idx].max_health.max(1.0);
        let (bonus_magic, bonus_true) = on_ability_bonus_damage(
            &mut self.controlled_champion_combat_runtime,
            ability_raw_damage,
            ability_ap_ratio,
            self.controlled_champion_stats.ability_power,
            self.controlled_champion_stats.attack_damage.max(0.0),
            target_current_health,
            target_max_health,
            self.time,
            Some(idx),
            attacker_level,
        );
        self.apply_damage_to_enemy(idx, 0.0, bonus_magic, bonus_true)
    }

    fn apply_ability_bonus_damage_to_enemies_in_controlled_champion_range(
        &mut self,
        ability_raw_damage: f64,
        ability_ap_ratio: f64,
        attacker_level: usize,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> (f64, usize) {
        let mut total = 0.0;
        let mut hit_count = 0usize;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius) {
                continue;
            }
            hit_count += 1;
            total += self.apply_ability_bonus_damage_to_enemy(
                idx,
                ability_raw_damage,
                ability_ap_ratio,
                attacker_level,
            );
        }
        (total, hit_count)
    }

    fn apply_controlled_champion_runtime_heal(&mut self, damage_dealt: f64) {
        if damage_dealt <= 0.0 {
            return;
        }
        let runtime_heal = outgoing_damage_heal(
            &mut self.controlled_champion_combat_runtime,
            damage_dealt,
            self.time,
        );
        if runtime_heal <= 0.0 {
            return;
        }
        let script_heal_multiplier = controlled_champion_heal_multiplier();
        let resolved_heal = resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: runtime_heal * script_heal_multiplier,
                source: ScalarMetricSource::Healing,
                clamp_min_zero: true,
            },
            self.controlled_champion_buffs,
        );
        let before = self.health;
        self.health = self.max_health.min(self.health + resolved_heal);
        self.healing_done_total += (self.health - before).max(0.0);
    }

    fn apply_magic_damage_to_enemies_in_controlled_champion_range(
        &mut self,
        raw_magic_damage: f64,
        range: f64,
        effect_hitbox_radius: f64,
    ) -> (f64, usize) {
        if raw_magic_damage <= 0.0 {
            return (0.0, 0);
        }
        let mut total = 0.0;
        let mut hit_count = 0usize;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_in_controlled_champion_range(idx, range, effect_hitbox_radius) {
                continue;
            }
            hit_count += 1;
            total += self.apply_magic_damage_to_enemy(idx, raw_magic_damage);
        }
        (total, hit_count)
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
        if self.pool_damage_until > self.time
            && self.pool_damage_tick_interval_seconds > 0.0
            && self.pool_next_damage_tick_at.is_finite()
        {
            while self.pool_next_damage_tick_at <= to_time + 1e-9
                && self.pool_next_damage_tick_at <= self.pool_damage_until + 1e-9
            {
                let (dealt, hit_count) = self
                    .apply_magic_damage_to_enemies_in_controlled_champion_range(
                        self.controlled_champion_defensive_ability_two_damage_per_tick,
                        self.pool_effect_range,
                        0.0,
                    );
                self.damage_dealt_total += dealt.max(0.0);
                if dealt > 0.0 {
                    let resolved_heal = resolve_stat(
                        StatQuery::ScalarAmount {
                            base_amount: dealt
                                * self
                                    .controlled_champion_defensive_ability_two_heal_ratio_of_damage,
                            source: ScalarMetricSource::Healing,
                            clamp_min_zero: true,
                        },
                        self.controlled_champion_buffs,
                    );
                    let before = self.health;
                    self.health = self.max_health.min(self.health + resolved_heal);
                    self.healing_done_total += (self.health - before).max(0.0);
                }
                self.trace_event(
                    "controlled_champion_pool_tick",
                    format!(
                        "{} {} tick dealt {:.1} to {} enemies in range",
                        self.controlled_champion_name,
                        self.cast_profile.defensive_ability_two_id,
                        dealt,
                        hit_count
                    ),
                );
                self.pool_next_damage_tick_at += self.pool_damage_tick_interval_seconds;
            }
            if self.pool_next_damage_tick_at > self.pool_damage_until + 1e-9 {
                self.pool_next_damage_tick_at = f64::INFINITY;
            }
        }
        if self.emergency_heal_until > self.time {
            let active = delta.min(self.emergency_heal_until - self.time);
            let resolved_heal = resolve_stat(
                StatQuery::ScalarAmount {
                    base_amount: self.emergency_heal_rate * active,
                    source: ScalarMetricSource::Healing,
                    clamp_min_zero: true,
                },
                self.controlled_champion_buffs,
            );
            let before = self.health;
            self.health = self.max_health.min(self.health + resolved_heal);
            self.healing_done_total += (self.health - before).max(0.0);
        }
        let runtime_regen = tick_regen_heal(
            &self.controlled_champion_combat_runtime,
            self.health,
            self.max_health,
            delta,
        );
        if runtime_regen > 0.0 {
            let resolved_regen = resolve_stat(
                StatQuery::ScalarAmount {
                    base_amount: runtime_regen,
                    source: ScalarMetricSource::Healing,
                    clamp_min_zero: true,
                },
                self.controlled_champion_buffs,
            );
            let before = self.health;
            self.health = self.max_health.min(self.health + resolved_regen);
            self.healing_done_total += (self.health - before).max(0.0);
        }
        self.combat_primitives.tick(delta);
        self.update_actor_positions(delta);
        self.apply_enemy_regen(delta);
        self.time = to_time;
        self.cleanup_expired_projectile_blocks();
        self.emit_trace_snapshots_due();
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
            let runtime_movement_speed_multiplier =
                movement_speed_multiplier(&state.runtime, self.time, state.enemy.level);
            let speed = resolve_stat(
                StatQuery::ScalarAmount {
                    base_amount: state.move_speed * state.behavior.movement_speed_scale,
                    source: ScalarMetricSource::MovementSpeed,
                    clamp_min_zero: true,
                },
                RuntimeBuffState {
                    movement_speed_multiplier: runtime_movement_speed_multiplier,
                    ..RuntimeBuffState::default()
                },
            );
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
        source: DamageSourceContext,
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
        let bonus_armor = (self.controlled_champion_stats.armor
            - self.controlled_champion_base.base_armor)
            .max(0.0);
        let bonus_magic_resist = (self.controlled_champion_stats.magic_resist
            - self.controlled_champion_base.base_magic_resist)
            .max(0.0);
        let (aftershock_physical_multiplier, aftershock_magic_multiplier) =
            incoming_damage_multipliers(
                &self.controlled_champion_combat_runtime,
                self.time,
                self.sim.champion_level,
                self.controlled_champion_stats.armor,
                self.controlled_champion_stats.magic_resist,
                bonus_armor,
                bonus_magic_resist,
            );
        let mut damage = physical * self.physical_multiplier * aftershock_physical_multiplier
            + magic * self.magic_multiplier * aftershock_magic_multiplier
            + true_damage;
        let active_enemy_count = self
            .enemy_state
            .iter()
            .filter(|state| state.respawn_at.is_none() && state.health > 0.0)
            .count();
        let script_damage_taken_multiplier =
            controlled_champion_damage_taken_multiplier(active_enemy_count);
        damage = resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: damage * script_damage_taken_multiplier,
                source: ScalarMetricSource::IncomingDamageTaken,
                clamp_min_zero: true,
            },
            self.controlled_champion_buffs,
        );
        if self.emergency_shield_amount > 0.0 && damage > 0.0 {
            let absorbed = self.emergency_shield_amount.min(damage);
            self.emergency_shield_amount -= absorbed;
            damage -= absorbed;
        }
        self.trace_event(
            "damage_in",
            format!(
                "{} {} -> {} | physical {:.1}, magic {:.1}, true {:.1}, total {:.1}",
                source.champion_name,
                source.ability_name,
                self.controlled_champion_name,
                physical,
                magic,
                true_damage,
                damage
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
            self.health = 1.0_f64.max(
                self.controlled_champion_base.base_health * self.sim.ga_revive_base_health_ratio,
            );
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

        if self.controlled_champion_script_enabled() {
            let can_cast_now = self.can_cast();
            let offensive_ultimate_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_ultimate_ability_id)
                .is_some();
            let offensive_ultimate_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_ultimate_ability_id,
            );
            let offensive_ultimate_has_viable_targets = can_cast_now
                && offensive_ultimate_equipped
                && self.time >= offensive_ultimate_ready_at
                && self
                    .max_enemy_distance_in_controlled_champion_range(
                        self.cast_profile.offensive_ultimate_range,
                        self.cast_profile.offensive_ultimate_effect_hitbox_radius,
                    )
                    .is_some();

            let defensive_ability = decide_controlled_champion_defensive_ability_activations(
                self.controlled_champion_script.as_ref(),
                ControlledChampionDefensiveAbilityDecisionInput {
                    now_seconds: self.time,
                    can_cast: can_cast_now,
                    defensive_ability_two_ready_at: self.controlled_champion_ability_ready_at(
                        &self.cast_profile.defensive_ability_two_id,
                    ),
                    offensive_ultimate_ready_at,
                    offensive_ultimate_has_viable_targets,
                },
            );

            if defensive_ability.cast_defensive_ability_two {
                let defensive_ability_two_id = self.cast_profile.defensive_ability_two_id.clone();
                self.set_controlled_champion_ability_ready_at(
                    &defensive_ability_two_id,
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
                    * self.controlled_champion_defensive_ability_two_cost_percent_current_health
                    * self.urf.health_cost_multiplier;
                self.health -= cost;

                let defensive_ability_two_config = ControlledChampionDefensiveAbilityTwoConfig {
                    cooldown_seconds: self.pool_cooldown,
                    duration_seconds: self.pool_duration,
                    effect_range: self.pool_effect_range,
                    damage_tick_interval_seconds: self.pool_damage_tick_interval_seconds,
                    cost_percent_current_health: self
                        .controlled_champion_defensive_ability_two_cost_percent_current_health,
                    damage_per_tick: self.controlled_champion_defensive_ability_two_damage_per_tick,
                    damage_per_tick_bonus_health_ratio: self
                        .controlled_champion_defensive_ability_two_damage_per_tick_bonus_health_ratio,
                    heal_ratio_of_damage: self
                        .controlled_champion_defensive_ability_two_heal_ratio_of_damage,
                };
                let pool_damage_per_tick = controlled_champion_defensive_ability_two_raw_damage(
                    self.controlled_champion_script.as_ref(),
                    defensive_ability_two_config,
                    &self.controlled_champion_stats,
                    &self.controlled_champion_base,
                );
                if self.pool_damage_tick_interval_seconds > 0.0 && self.pool_duration > 0.0 {
                    self.pool_damage_until = self.time + self.pool_duration;
                    self.pool_next_damage_tick_at =
                        self.time + self.pool_damage_tick_interval_seconds;
                } else {
                    self.pool_damage_until = self.time;
                    self.pool_next_damage_tick_at = f64::INFINITY;
                }
                self.controlled_champion_defensive_ability_two_damage_per_tick =
                    pool_damage_per_tick.max(0.0);
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} (untargetable {:.2}s, damage tick {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.defensive_ability_two_id,
                        self.pool_duration,
                        self.pool_damage_tick_interval_seconds
                    ),
                );

                if self.health <= 0.0 {
                    self.handle_death();
                    return;
                }
            }

            // Script-owned cadence for controlled champion offensive spell scheduling.
            let can_cast = self.can_cast();
            let offensive_primary_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_primary_ability_id,
            );
            let offensive_primary_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_primary_ability_id)
                .is_some();
            let offensive_primary_target = if can_cast
                && offensive_primary_equipped
                && self.time >= offensive_primary_ready_at
            {
                self.first_active_enemy_in_controlled_champion_range(
                    self.cast_profile.offensive_primary_range,
                    self.cast_profile.offensive_primary_effect_hitbox_radius,
                )
                .map(|target_index| ControlledChampionTargetSnapshot {
                    target_index,
                    distance: self.distance_to_target(target_index),
                })
            } else {
                None
            };
            let offensive_secondary_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_secondary_ability_id,
            );
            let offensive_secondary_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_secondary_ability_id)
                .is_some();
            let offensive_secondary_max_distance = if can_cast
                && offensive_secondary_equipped
                && self.time >= offensive_secondary_ready_at
            {
                self.max_enemy_distance_in_controlled_champion_range(
                    self.cast_profile.offensive_secondary_range,
                    self.cast_profile.offensive_secondary_effect_hitbox_radius,
                )
            } else {
                None
            };
            let offensive_ultimate_ready_at = self.controlled_champion_ability_ready_at(
                &self.cast_profile.offensive_ultimate_ability_id,
            );
            let offensive_ultimate_equipped = self
                .controlled_champion_ability_loadout
                .slot_for_ability(&self.cast_profile.offensive_ultimate_ability_id)
                .is_some();
            let offensive_ultimate_max_distance = if can_cast
                && offensive_ultimate_equipped
                && self.time >= offensive_ultimate_ready_at
            {
                self.max_enemy_distance_in_controlled_champion_range(
                    self.cast_profile.offensive_ultimate_range,
                    self.cast_profile.offensive_ultimate_effect_hitbox_radius,
                )
            } else {
                None
            };
            let offensive = decide_controlled_champion_offensive_casts(
                self.controlled_champion_script.as_ref(),
                ControlledChampionOffensiveDecisionInput {
                    now_seconds: self.time,
                    can_cast,
                    offensive_primary_ready_at,
                    offensive_secondary_ready_at,
                    offensive_ultimate_ready_at,
                    cooldowns: self.offensive_cooldowns,
                    cast_profile: self.cast_profile.clone(),
                    offensive_primary_target,
                    offensive_secondary_max_distance,
                    offensive_ultimate_max_distance,
                },
            );

            if let Some(offensive_primary) = offensive.offensive_primary {
                self.set_controlled_champion_ability_ready_at(
                    &offensive_primary.ability_id,
                    offensive_primary.next_ready_at,
                );
                self.begin_cast_lock_window(
                    self.cast_profile.offensive_primary_windup_seconds,
                    0.0,
                    0.0,
                );
                let target_at_cast = self.enemy_state[offensive_primary.target_index].position;
                let target_name = self.enemy_state[offensive_primary.target_index]
                    .enemy
                    .name
                    .clone();
                self.schedule_event(
                    offensive_primary.impact_delay_seconds,
                    50,
                    EventType::ControlledChampionOffensivePrimaryHit {
                        idx: offensive_primary.target_index,
                        source: self.target_position,
                        target_at_cast,
                        projectile_speed: self.cast_profile.offensive_primary_projectile_speed,
                        effect_hitbox_radius: self
                            .cast_profile
                            .offensive_primary_effect_hitbox_radius,
                    },
                    None,
                );
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} on {} (impact in {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_primary_ability_id,
                        target_name,
                        offensive_primary.impact_delay_seconds
                    ),
                );
            }
            if let Some(offensive_secondary) = offensive.offensive_secondary {
                self.set_controlled_champion_ability_ready_at(
                    &offensive_secondary.ability_id,
                    offensive_secondary.next_ready_at,
                );
                self.begin_cast_lock_window(
                    self.cast_profile.offensive_secondary_windup_seconds,
                    0.0,
                    0.0,
                );
                self.schedule_event(
                    offensive_secondary.impact_delay_seconds,
                    49,
                    EventType::ControlledChampionOffensiveSecondaryHit,
                    None,
                );
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} (impact in {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_secondary_ability_id,
                        offensive_secondary.impact_delay_seconds
                    ),
                );
            }
            if let Some(offensive_ultimate) = offensive.offensive_ultimate {
                self.set_controlled_champion_ability_ready_at(
                    &offensive_ultimate.ability_id,
                    offensive_ultimate.next_ready_at,
                );
                self.begin_cast_lock_window(
                    self.cast_profile.offensive_ultimate_windup_seconds,
                    0.0,
                    0.0,
                );
                self.schedule_event(
                    offensive_ultimate.impact_delay_seconds,
                    48,
                    EventType::ControlledChampionOffensiveUltimateHit,
                    None,
                );
                self.trace_event(
                    "controlled_champion_cast",
                    format!(
                        "{} cast {} (impact in {:.2}s)",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_ultimate_ability_id,
                        offensive_ultimate.impact_delay_seconds
                    ),
                );
            }
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
            self.trace_event(
                "controlled_champion_item_active",
                format!(
                    "{} activated stasis item for {:.2}s",
                    self.controlled_champion_name, self.sim.zhonya_duration_seconds
                ),
            );
        }

        if defensive_items.activate_emergency_shield {
            self.emergency_shield_item_ready_at =
                self.time + self.emergency_shield_item_cooldown_seconds;
            self.emergency_shield_amount += self.sim.protoplasm_bonus_health;
            self.emergency_heal_rate =
                self.sim.protoplasm_heal_total / self.sim.protoplasm_duration_seconds.max(0.001);
            self.emergency_heal_until = self.time + self.sim.protoplasm_duration_seconds;
            self.trace_event(
                "controlled_champion_item_active",
                format!(
                    "{} activated emergency shield ({:.1} shield, {:.1}s heal window)",
                    self.controlled_champion_name,
                    self.sim.protoplasm_bonus_health,
                    self.sim.protoplasm_duration_seconds
                ),
            );
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
                    let bonus_attack_damage =
                        (state.physical_hit_damage - state.enemy.base.base_attack_damage).max(0.0);
                    let (extra_physical, extra_magic, extra_true) = on_hit_bonus_damage(
                        state.behavior,
                        &mut state.runtime,
                        attack_damage,
                        state.ability_power,
                        bonus_attack_damage,
                        target_current,
                        target_max,
                        state.max_health,
                        self.time,
                        Some(0),
                        state.enemy.level,
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
                let enemy_name = self.enemy_state[idx].enemy.name.clone();
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
                        match self.apply_damage(
                            DamageSourceContext {
                                champion_name: enemy_name.clone(),
                                ability_name: "Auto Attack".to_string(),
                            },
                            physical,
                            magic,
                            true_damage,
                        ) {
                            DamageApplicationOutcome::Applied => IncomingImpactOutcome::Applied,
                            DamageApplicationOutcome::NullifiedUntargetable => {
                                IncomingImpactOutcome::NullifiedUntargetable
                            }
                            DamageApplicationOutcome::Ignored => {
                                IncomingImpactOutcome::IgnoredTargetUnavailable
                            }
                        }
                    }
                };
                match outcome {
                    IncomingImpactOutcome::Applied => self.trace_event(
                        "attack_hit",
                        format!(
                            "{} hit {} (phys {:.1}, magic {:.1}, true {:.1})",
                            enemy_name, self.controlled_champion_name, physical, magic, true_damage
                        ),
                    ),
                    IncomingImpactOutcome::ProjectileBlocked => self.trace_event(
                        "projectile_blocked",
                        format!(
                            "{} auto attack blocked by active projectile block zone",
                            enemy_name
                        ),
                    ),
                    IncomingImpactOutcome::MissedHitbox => self.trace_event(
                        "attack_missed",
                        format!(
                            "{} auto attack missed {} ({})",
                            enemy_name,
                            self.controlled_champion_name,
                            hitbox_miss_reason(
                                source,
                                if projectile_speed > 0.0 {
                                    target_at_release
                                } else {
                                    source
                                },
                                self.target_position,
                                self.controlled_champion_hitbox_radius,
                                effect_hitbox_radius
                            )
                        ),
                    ),
                    IncomingImpactOutcome::NullifiedUntargetable => self.trace_event(
                        "impact_nullified",
                        format!(
                            "{} auto attack on {} was nullified by untargetable or stasis state",
                            enemy_name, self.controlled_champion_name
                        ),
                    ),
                    IncomingImpactOutcome::IgnoredTargetUnavailable => self.trace_event(
                        "impact_ignored",
                        format!(
                            "{} auto attack skipped because {} is unavailable",
                            enemy_name, self.controlled_champion_name
                        ),
                    ),
                }
                self.schedule_next_attack(idx);
            }
            EventType::ControlledChampionAttack => {
                if !self.can_basic_attack() {
                    self.schedule_next_controlled_champion_attack();
                    return;
                }
                let Some(idx) = self.first_active_enemy_in_controlled_champion_attack_range()
                else {
                    self.schedule_next_controlled_champion_attack();
                    return;
                };
                self.controlled_champion_attack_sequence =
                    self.controlled_champion_attack_sequence.wrapping_add(1);
                let token = self.controlled_champion_attack_sequence;
                let enemy_name = self.enemy_state[idx].enemy.name.clone();
                self.trace_event(
                    "controlled_champion_attack_start",
                    format!(
                        "{} begins auto attack on {}",
                        self.controlled_champion_name, enemy_name
                    ),
                );
                let windup = self
                    .controlled_champion_behavior
                    .attack_windup_seconds
                    .max(0.0);
                self.schedule_event(
                    windup,
                    36,
                    EventType::ControlledChampionAttackWindup { idx, token },
                    None,
                );
            }
            EventType::ControlledChampionAttackWindup { idx, token } => {
                if token != self.controlled_champion_attack_sequence
                    || !self.enemy_is_active(idx)
                    || !self.controlled_champion_in_attack_range(idx)
                {
                    self.schedule_next_controlled_champion_attack();
                    return;
                }
                if !self.can_basic_attack() {
                    self.trace_event(
                        "controlled_champion_attack_cancelled",
                        format!(
                            "{} auto attack cancelled during windup by crowd control, cast lock, or invulnerability",
                            self.controlled_champion_name
                        ),
                    );
                    self.schedule_next_controlled_champion_attack();
                    return;
                }
                let source = self.target_position;
                let target_at_release = self.enemy_state[idx].position;
                let projectile_speed = self.controlled_champion_behavior.attack_projectile_speed;
                let effect_hitbox_radius = self
                    .controlled_champion_behavior
                    .attack_effect_hitbox_radius;
                let travel = self.enemy_projectile_delay_from_points(
                    source,
                    target_at_release,
                    projectile_speed,
                );
                self.schedule_event(
                    travel,
                    35,
                    EventType::ControlledChampionAttackHit {
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
            EventType::ControlledChampionAttackHit {
                idx,
                token,
                source,
                target_at_release,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if token != self.controlled_champion_attack_sequence || !self.enemy_is_active(idx) {
                    self.schedule_next_controlled_champion_attack();
                    return;
                }
                if projectile_speed <= 0.0 && !self.can_basic_attack() {
                    self.trace_event(
                        "controlled_champion_attack_cancelled",
                        format!(
                            "{} melee auto attack cancelled before impact by crowd control, cast lock, or invulnerability",
                            self.controlled_champion_name
                        ),
                    );
                    self.schedule_next_controlled_champion_attack();
                    return;
                }
                let enemy_name = self.enemy_state[idx].enemy.name.clone();
                if projectile_speed > 0.0
                    && self.is_projectile_blocked(source, target_at_release, effect_hitbox_radius)
                {
                    self.trace_event(
                        "projectile_blocked",
                        format!(
                            "{} auto attack blocked by active projectile block zone",
                            self.controlled_champion_name
                        ),
                    );
                    self.schedule_next_controlled_champion_attack();
                    return;
                }

                let enemy_position = self.enemy_state[idx].position;
                let enemy_hitbox_radius = self.enemy_state[idx].hitbox_radius;
                let hit = if projectile_speed > 0.0 {
                    path_hits_circle(
                        source,
                        target_at_release,
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
                        "controlled_champion_attack_missed",
                        format!(
                            "{} auto attack missed {} ({})",
                            self.controlled_champion_name,
                            enemy_name,
                            hitbox_miss_reason(
                                source,
                                if projectile_speed > 0.0 {
                                    target_at_release
                                } else {
                                    source
                                },
                                enemy_position,
                                enemy_hitbox_radius,
                                effect_hitbox_radius
                            )
                        ),
                    );
                    self.schedule_next_controlled_champion_attack();
                    return;
                }

                let target_current_health = self
                    .enemy_state
                    .get(idx)
                    .map(|state| state.health.max(0.0))
                    .unwrap_or(0.0);
                let target_max_health = self
                    .enemy_state
                    .get(idx)
                    .map(|state| state.max_health.max(1.0))
                    .unwrap_or(1.0);
                let attack_damage = self.controlled_champion_base.base_attack_damage
                    + self.controlled_champion_stats.attack_damage;
                let (extra_physical, extra_magic, extra_true) = on_hit_bonus_damage(
                    self.controlled_champion_behavior,
                    &mut self.controlled_champion_combat_runtime,
                    attack_damage,
                    self.controlled_champion_stats.ability_power,
                    self.controlled_champion_stats.attack_damage.max(0.0),
                    target_current_health,
                    target_max_health,
                    self.max_health,
                    self.time,
                    Some(idx),
                    self.sim.champion_level,
                );
                let physical = attack_damage + extra_physical;
                let magic = extra_magic;
                let true_damage = extra_true;
                let dealt = self.apply_damage_to_enemy(idx, physical, magic, true_damage);
                self.damage_dealt_total += dealt.max(0.0);
                self.apply_controlled_champion_runtime_heal(dealt);
                self.trace_event(
                    "controlled_champion_attack_hit",
                    format!(
                        "{} auto attacked {} (phys {:.1}, magic {:.1}, true {:.1}, dealt {:.1})",
                        self.controlled_champion_name,
                        enemy_name,
                        physical,
                        magic,
                        true_damage,
                        dealt
                    ),
                );
                self.schedule_next_controlled_champion_attack();
            }
            EventType::ControlledChampionOffensivePrimaryHit {
                idx,
                source,
                target_at_cast,
                projectile_speed,
                effect_hitbox_radius,
            } => {
                if !self.controlled_champion_script_enabled() {
                    return;
                }
                if idx >= self.enemy_state.len() || !self.enemy_is_active(idx) {
                    return;
                }
                let enemy_name = self.enemy_state[idx].enemy.name.clone();
                if projectile_speed > 0.0
                    && self.is_projectile_blocked(source, target_at_cast, effect_hitbox_radius)
                {
                    self.trace_event(
                        "projectile_blocked",
                        format!(
                            "{} {} blocked by active projectile block zone",
                            self.controlled_champion_name,
                            self.cast_profile.offensive_primary_ability_id
                        ),
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
                        "controlled_champion_primary_miss",
                        format!(
                            "{} {} missed {} ({})",
                            self.controlled_champion_name,
                            self.cast_profile.offensive_primary_ability_id,
                            enemy_name,
                            hitbox_miss_reason(
                                source,
                                if projectile_speed > 0.0 {
                                    target_at_cast
                                } else {
                                    source
                                },
                                enemy_position,
                                enemy_hitbox_radius,
                                effect_hitbox_radius
                            )
                        ),
                    );
                    return;
                }
                let q_raw_damage = controlled_champion_offensive_raw_damage(
                    self.controlled_champion_script.as_ref(),
                    ControlledChampionOffensiveAbility::Primary,
                    self.controlled_champion_stats.ability_power,
                );
                let q_ap_ratio = controlled_champion_offensive_ap_ratio(
                    self.controlled_champion_script.as_ref(),
                    ControlledChampionOffensiveAbility::Primary,
                );
                let generic_runtime_bonus = self.apply_ability_bonus_damage_to_enemy(
                    idx,
                    q_raw_damage,
                    q_ap_ratio,
                    self.sim.champion_level,
                );
                let dealt =
                    self.apply_magic_damage_to_enemy(idx, q_raw_damage) + generic_runtime_bonus;
                self.damage_dealt_total += dealt.max(0.0);
                self.apply_controlled_champion_runtime_heal(dealt);
                if dealt > 0.0 {
                    let script_heal_multiplier = controlled_champion_heal_multiplier();
                    let resolved_heal = resolve_stat(
                        StatQuery::ScalarAmount {
                            base_amount: dealt
                                * controlled_champion_offensive_primary_heal_ratio(
                                    self.controlled_champion_script.as_ref(),
                                )
                                * script_heal_multiplier,
                            source: ScalarMetricSource::Healing,
                            clamp_min_zero: true,
                        },
                        self.controlled_champion_buffs,
                    );
                    let before = self.health;
                    self.health = self.max_health.min(self.health + resolved_heal);
                    self.healing_done_total += (self.health - before).max(0.0);
                }
                self.trace_event(
                    "controlled_champion_primary_hit",
                    format!(
                        "{} {} hit {} for {:.1}",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_primary_ability_id,
                        enemy_name,
                        dealt
                    ),
                );
            }
            EventType::ControlledChampionOffensiveSecondaryHit => {
                if !self.controlled_champion_script_enabled() {
                    return;
                }
                let e_raw_damage = controlled_champion_offensive_raw_damage(
                    self.controlled_champion_script.as_ref(),
                    ControlledChampionOffensiveAbility::Secondary,
                    self.controlled_champion_stats.ability_power,
                );
                let e_ap_ratio = controlled_champion_offensive_ap_ratio(
                    self.controlled_champion_script.as_ref(),
                    ControlledChampionOffensiveAbility::Secondary,
                );
                let (base_dealt, hit_count) = self
                    .apply_magic_damage_to_enemies_in_controlled_champion_range(
                        e_raw_damage,
                        self.cast_profile.offensive_secondary_range,
                        self.cast_profile.offensive_secondary_effect_hitbox_radius,
                    );
                let (generic_runtime_bonus, _) = self
                    .apply_ability_bonus_damage_to_enemies_in_controlled_champion_range(
                        e_raw_damage,
                        e_ap_ratio,
                        self.sim.champion_level,
                        self.cast_profile.offensive_secondary_range,
                        self.cast_profile.offensive_secondary_effect_hitbox_radius,
                    );
                let dealt = base_dealt + generic_runtime_bonus;
                self.damage_dealt_total += dealt.max(0.0);
                self.apply_controlled_champion_runtime_heal(dealt);
                self.trace_event(
                    "controlled_champion_secondary_hit",
                    format!(
                        "{} {} dealt {:.1} to {} enemies in range",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_secondary_ability_id,
                        dealt,
                        hit_count
                    ),
                );
            }
            EventType::ControlledChampionOffensiveUltimateHit => {
                if !self.controlled_champion_script_enabled() {
                    return;
                }
                let r_raw_damage = controlled_champion_offensive_raw_damage(
                    self.controlled_champion_script.as_ref(),
                    ControlledChampionOffensiveAbility::Ultimate,
                    self.controlled_champion_stats.ability_power,
                );
                let r_ap_ratio = controlled_champion_offensive_ap_ratio(
                    self.controlled_champion_script.as_ref(),
                    ControlledChampionOffensiveAbility::Ultimate,
                );
                let (base_dealt, hit_count) = self
                    .apply_magic_damage_to_enemies_in_controlled_champion_range(
                        r_raw_damage,
                        self.cast_profile.offensive_ultimate_range,
                        self.cast_profile.offensive_ultimate_effect_hitbox_radius,
                    );
                let (generic_runtime_bonus, _) = self
                    .apply_ability_bonus_damage_to_enemies_in_controlled_champion_range(
                        r_raw_damage,
                        r_ap_ratio,
                        self.sim.champion_level,
                        self.cast_profile.offensive_ultimate_range,
                        self.cast_profile.offensive_ultimate_effect_hitbox_radius,
                    );
                let dealt = base_dealt + generic_runtime_bonus;
                self.damage_dealt_total += dealt.max(0.0);
                self.apply_controlled_champion_runtime_heal(dealt);
                self.trace_event(
                    "controlled_champion_ultimate_hit",
                    format!(
                        "{} {} dealt {:.1} to {} enemies in range",
                        self.controlled_champion_name,
                        self.cast_profile.offensive_ultimate_ability_id,
                        dealt,
                        hit_count
                    ),
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
                        actor_level: state.enemy.level,
                        distance_to_target,
                        physical_hit_damage: state.physical_hit_damage,
                        actor_ability_power: state.ability_power,
                        actor_bonus_attack_damage: (state.physical_hit_damage
                            - state.enemy.base.base_attack_damage)
                            .max(0.0),
                        target_current_health,
                        target_max_health,
                        now,
                    };
                    execute_champion_script_event(input, &mut state.runtime)
                };
                if !actions.is_empty() {
                    self.trace_event(
                        "champion_script",
                        format!(
                            "{} executed {}",
                            champion_name,
                            champion_script_event_label(script_event)
                        ),
                    );
                    if let Some(cooldown_seconds) =
                        champion_script_event_cooldown_seconds(&champion_name, script_event)
                    {
                        let ability_haste = self
                            .enemy_state
                            .get(idx)
                            .map(|state| state.ability_haste)
                            .unwrap_or(self.urf.ability_haste);
                        let resolved_cooldown = resolve_stat(
                            StatQuery::CooldownSeconds {
                                base_seconds: cooldown_seconds,
                                source: CooldownMetricSource::Ability,
                            },
                            RuntimeBuffState {
                                ability_haste,
                                item_haste: self.urf.item_haste,
                                cooldown_rate_multiplier: 1.0,
                                ..RuntimeBuffState::default()
                            },
                        );
                        let next_ready = self.time + resolved_cooldown.max(0.0);
                        if let Some(state) = self.enemy_state.get_mut(idx) {
                            state.script_event_ready_at.insert(script_event, next_ready);
                        }
                    }
                }
                self.apply_enemy_script_actions(idx, script_event, epoch, actions);
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
        self.trace_next_snapshot_at = 0.0;
        self.emit_trace_snapshots_due();
    }

    pub(super) fn trace_events(&self) -> &[String] {
        &self.trace_events
    }

    pub(super) fn controlled_champion_rune_proc_telemetry(
        &self,
    ) -> Vec<ChampionRuneProcTelemetryEntry> {
        describe_rune_proc_telemetry(&self.controlled_champion_combat_runtime)
    }
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

    let attack_damage = enemy.base.base_attack_damage + enemy_stats.attack_damage;
    let ability_power = enemy_stats.ability_power.max(0.0);
    let ability_haste = enemy_stats.ability_haste + urf.ability_haste;
    let runtime_buffs = RuntimeBuffState {
        ability_haste,
        item_haste: urf.item_haste,
        cooldown_rate_multiplier: 1.0,
        ..RuntimeBuffState::default()
    };
    let armor = (enemy.base.base_armor + enemy_stats.armor).max(0.0);
    let magic_resist = (enemy.base.base_magic_resist + enemy_stats.magic_resist).max(0.0);
    let physical_multiplier = 100.0 / (100.0 + armor);
    let max_health = (enemy.base.base_health + enemy_stats.health).max(1.0);
    let move_speed = resolve_stat(
        StatQuery::MovementSpeedUnits {
            base_units: enemy.base.base_move_speed,
            flat_bonus_units: enemy_stats.move_speed_flat,
            percent_bonus: enemy_stats.move_speed_percent,
            minimum_units: 150.0,
        },
        runtime_buffs,
    );

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
    let runtime_rune_names = enemy.loadout_rune_names.clone();
    let runtime = build_champion_loadout_runtime(
        &runtime_item_names,
        &runtime_rune_names,
        urf.item_haste,
        enemy.base.is_melee,
    );
    attack_speed = base_attack_speed * attack_speed_multiplier(&runtime, 0.0);

    let attack_interval = 1.0 / attack_speed.max(0.001);
    let behavior = behavior_profile(
        &enemy.name,
        enemy.base.is_melee,
        enemy.base.base_attack_range,
        enemy.base.base_attack_projectile_speed,
    );

    EnemyDerivedModel {
        behavior,
        runtime,
        runtime_item_names,
        runtime_rune_names,
        max_health,
        armor,
        magic_resist,
        physical_multiplier,
        magic_multiplier: 100.0 / (100.0 + magic_resist),
        attack_damage,
        ability_power,
        ability_haste,
        attack_speed,
        attack_interval,
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
        ability_hit_damage: 0.0,
        burst_physical_damage: 0.0,
        burst_magic_damage: 0.0,
        burst_true_damage: 0.0,
    }
}

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

#[cfg(test)]
#[path = "tests/engine_tests.rs"]
mod tests;

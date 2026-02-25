use std::collections::{HashMap, HashSet, VecDeque};

use crate::champion_control_harness::{
    ChampionActionDecisionPolicy, ChampionActionStatusReport, ChampionControllerIdentity,
    ChampionControllerKind,
};
use crate::defaults::{
    champion_ai_profile, champion_hitbox_radius,
    controlled_champion_controller_vision_radius_default,
    controlled_champion_request_fixed_tick_delay_default,
    protoplasm_lifeline_cooldown_seconds_default, simulator_defaults, world_lifecycle_defaults,
};
use crate::scripts::champions::{
    ChampionBehaviorProfile, ChampionLoadoutRuntime, ChampionRuneProcTelemetryEntry,
    ChampionScriptAction, ChampionScriptEvent, ChampionScriptExecutionInput,
    ControlledChampionAbilityCooldowns, ControlledChampionCastProfile,
    ControlledChampionDefensiveAbilityDecisionInput, ControlledChampionDefensiveAbilityTwoConfig,
    ControlledChampionOffensiveAbility, ControlledChampionOffensiveDecisionInput,
    ControlledChampionScriptHandle, ControlledChampionTargetSnapshot, attack_speed_multiplier,
    behavior_profile, build_champion_loadout_runtime, champion_script_event_cast_range,
    champion_script_event_cooldown_seconds, champion_script_event_for_ability_id,
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
use crate::scripts::items::hooks::{
    controlled_champion_defensive_item_capabilities, defensive_item_capabilities_from_item_names,
};
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
use crate::world::{
    WorldActorAllegiance, WorldActorClass, WorldActorPosition, WorldLifecycleState, WorldState,
    default_urf_world_map_state, seed_static_world_ecology_anchors,
};

mod actor_state;
mod combat_timing_and_targeting;
mod controlled_champion_controller_channels;
mod enemy_combat_stat_modeling;
mod event_queue;
mod event_resolution;
mod geometry;
mod script_point_coordinate_conversions;
mod simulation_step;
mod trace_snapshot_reporting;

pub(crate) use self::enemy_combat_stat_modeling::derive_enemy_combat_stats;
use self::enemy_combat_stat_modeling::derive_enemy_model;
use self::event_queue::{EventQueueScheduler, EventType, QueuedEvent, QueuedProjectileImpactKind};
use self::geometry::{
    Vec2, distance_segment_to_segment, enemy_spawn_position, hitbox_miss_reason, path_hits_circle,
    projectile_travel_seconds, update_enemy_orbit_position, within_reach_with_hitboxes,
};
use self::script_point_coordinate_conversions::vec2_from_champion_script_point;
use super::*;

#[cfg(test)]
fn line_segments_intersect(a1: Vec2, a2: Vec2, b1: Vec2, b2: Vec2) -> bool {
    geometry::segment_intersection_checks::line_segments_intersect(a1, a2, b1, b2)
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
    stasis_item_available: bool,
    stasis_item_ready_at: f64,
    emergency_shield_item_available: bool,
    emergency_shield_item_ready_at: f64,
    emergency_shield_amount: f64,
    emergency_heal_rate: f64,
    emergency_heal_until: f64,
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

const CONTROLLED_CHAMPION_WORLD_ACTOR_ID: &str = "controlled_champion";

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

    event_queue: EventQueueScheduler,

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
    controlled_champion_controller_identity: ChampionControllerIdentity,
    controlled_champion_controller_policy: Option<Box<dyn ChampionActionDecisionPolicy>>,
    controlled_champion_manual_control_mode: bool,
    controlled_champion_controller_vision_radius: f64,
    controlled_champion_request_fixed_tick_delay: u64,
    controlled_champion_pending_action_requests:
        VecDeque<controlled_champion_controller_channels::QueuedActorActionRequest>,
    controlled_champion_next_action_request_sequence: u64,
    controlled_champion_current_tick_index: u64,
    controlled_champion_recent_action_status_reports: VecDeque<ChampionActionStatusReport>,
    controlled_champion_pending_move_target_position: Option<Vec2>,
    controlled_champion_basic_attack_target_actor_id: Option<String>,
    manually_controlled_enemy_actor_ids: HashSet<String>,
    enemy_pending_move_target_position_by_actor_id: HashMap<String, Vec2>,
    enemy_basic_attack_target_actor_id_by_actor_id: HashMap<String, String>,

    target_position: Vec2,
    enemy_state: Vec<EnemyState>,
    world_state: WorldState,
    world_lifecycle_state: WorldLifecycleState,
    controlled_champion_world_actor_id: String,
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
            sim.collect_rune_proc_telemetry,
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
            event_queue: EventQueueScheduler::new(),
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
            controlled_champion_controller_identity: ChampionControllerIdentity {
                controller_id: "runtime_default_controller".to_string(),
                controller_kind: ChampionControllerKind::ArtificialIntelligence,
            },
            controlled_champion_controller_policy: None,
            controlled_champion_manual_control_mode: false,
            controlled_champion_controller_vision_radius:
                controlled_champion_controller_vision_radius_default(),
            controlled_champion_request_fixed_tick_delay:
                controlled_champion_request_fixed_tick_delay_default(),
            controlled_champion_pending_action_requests: VecDeque::new(),
            controlled_champion_next_action_request_sequence: 0,
            controlled_champion_current_tick_index: 0,
            controlled_champion_recent_action_status_reports: VecDeque::new(),
            controlled_champion_pending_move_target_position: None,
            controlled_champion_basic_attack_target_actor_id: None,
            manually_controlled_enemy_actor_ids: HashSet::new(),
            enemy_pending_move_target_position_by_actor_id: HashMap::new(),
            enemy_basic_attack_target_actor_id_by_actor_id: HashMap::new(),
            target_position: Vec2 { x: 0.0, y: 0.0 },
            enemy_state: Vec::new(),
            world_state: WorldState::new(default_urf_world_map_state()),
            world_lifecycle_state: WorldLifecycleState::new(world_lifecycle_defaults()),
            controlled_champion_world_actor_id: CONTROLLED_CHAMPION_WORLD_ACTOR_ID.to_string(),
            projectile_block_zones: Vec::new(),
            trace_enabled: false,
            trace_events: Vec::new(),
            trace_snapshot_interval_seconds: 5.0,
            trace_next_snapshot_at: 0.0,
        };
        seed_static_world_ecology_anchors(&mut runner.world_state);
        runner.world_state.upsert_actor_position_clamped(
            &runner.controlled_champion_world_actor_id,
            WorldActorClass::Champion,
            WorldActorAllegiance::ControlledChampionTeam,
            WorldActorPosition {
                x: runner.target_position.x,
                y: runner.target_position.y,
            },
        );

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
            let enemy_defensive_item_capabilities =
                defensive_item_capabilities_from_item_names(&model.runtime_item_names);
            let enemy_has_stasis_item = enemy_defensive_item_capabilities.has_stasis_item;
            let enemy_has_emergency_shield_item =
                enemy_defensive_item_capabilities.has_emergency_shield_item;

            let clamped_position = runner.world_state.upsert_actor_position_clamped(
                &enemy.id,
                WorldActorClass::Champion,
                WorldActorAllegiance::OpponentTeam,
                WorldActorPosition {
                    x: position.x,
                    y: position.y,
                },
            );
            let position = Vec2 {
                x: clamped_position.x,
                y: clamped_position.y,
            };

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
                stasis_item_available: enemy_has_stasis_item,
                stasis_item_ready_at: 0.0,
                emergency_shield_item_available: enemy_has_emergency_shield_item,
                emergency_shield_item_ready_at: 0.0,
                emergency_shield_amount: 0.0,
                emergency_heal_rate: 0.0,
                emergency_heal_until: 0.0,
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

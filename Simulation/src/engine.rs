use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::scripts::enemies::{
    EnemyBehaviorProfile, EnemyLoadoutRuntime, EnemyScriptEvent, attack_speed_multiplier,
    behavior_profile, build_enemy_loadout_runtime, clear_transient_combat_state,
    on_ability_bonus_damage, on_hit_bonus_damage, scripted_event_schedules, tick_regen_heal,
};
use crate::scripts::vladimir::{VladimirCastProfile, default_cast_profile};

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

fn enemy_spawn_position(index: usize, total: usize, behavior: EnemyBehaviorProfile) -> Vec2 {
    let angle = (index as f64 / total.max(1) as f64) * std::f64::consts::TAU;
    let radius = if behavior.attack_range <= 200.0 {
        160.0
    } else {
        (behavior.attack_range * 0.80).clamp(360.0, 520.0)
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
    behavior: EnemyBehaviorProfile,
    runtime: EnemyLoadoutRuntime,
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
}

#[derive(Debug, Clone)]
struct EnemyDerivedModel {
    behavior: EnemyBehaviorProfile,
    runtime: EnemyLoadoutRuntime,
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
    expires_at: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventType {
    Attack(usize),
    AttackWindup(usize),
    AttackHit(usize),
    Ability(usize),
    AbilityHit(usize),
    Stun(usize),
    Burst(usize),
    BurstHit(usize),
    VladQHit(usize),
    VladEHit,
    VladRHit,
    EnemyScript(usize, EnemyScriptEvent, u64),
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

pub(super) struct VladCombatSimulation {
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

    event_queue: BinaryHeap<QueuedEvent>,
    event_counter: u64,

    vlad_stats: Stats,
    max_health: f64,
    health: f64,

    physical_multiplier: f64,
    magic_multiplier: f64,

    pool_cooldown: f64,
    pool_duration: f64,
    offensive_tuning: VladimirAbilityTuning,
    offensive_cooldowns: VladimirAbilityCooldowns,
    cast_profile: VladimirCastProfile,

    zhonya_available: bool,
    ga_available: bool,
    protoplasm_available: bool,

    ga_cooldown: f64,
    zhonya_cooldown: f64,
    protoplasm_cooldown: f64,

    zhonya_cd: f64,
    ga_cd: f64,
    pool_cd: f64,
    q_cd: f64,
    e_cd: f64,
    r_cd: f64,
    protoplasm_cd: f64,

    pool_until: f64,
    stasis_until: f64,
    ga_res_until: f64,
    stunned_until: f64,

    protoplasm_shield: f64,
    pool_heal_rate: f64,
    pool_heal_until: f64,
    protoplasm_hot_rate: f64,
    protoplasm_hot_until: f64,

    vlad_position: Vec2,
    enemy_state: Vec<EnemyState>,
    projectile_block_zones: Vec<ProjectileBlockZone>,
    trace_enabled: bool,
    trace_events: Vec<String>,
}

impl VladCombatSimulation {
    pub(super) fn new(
        vlad_base: ChampionBase,
        vlad_build_items: &[Item],
        vlad_bonus_stats: &Stats,
        vlad_item_acquired_levels: Option<&HashMap<String, usize>>,
        enemies: &[(EnemyConfig, Vec<Item>, Stats)],
        sim: SimulationConfig,
        urf: UrfBuffs,
    ) -> Self {
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
        );
        let vlad_stats = compute_vlad_stats(&vlad_base, &vlad_item_stats);

        let max_health = vlad_stats.health;
        let physical_multiplier = 100.0 / (100.0 + vlad_stats.armor.max(0.0));
        let magic_multiplier = 100.0 / (100.0 + vlad_stats.magic_resist.max(0.0));

        let ability_haste = vlad_item_stats.ability_haste + urf.ability_haste;
        let pool_base_cd = [28.0, 25.0, 22.0, 19.0, 16.0][sim.vlad_pool_rank - 1];
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

        let zhonya_available = vlad_build_items
            .iter()
            .any(|i| i.name == "Zhonya's Hourglass");
        let ga_available = vlad_build_items.iter().any(|i| i.name == "Guardian Angel");
        let protoplasm_available = vlad_build_items
            .iter()
            .any(|i| i.name == "Protoplasm Harness");

        let ga_cooldown = cooldown_after_haste(sim.ga_cooldown_seconds, urf.item_haste);
        let zhonya_cooldown = cooldown_after_haste(sim.zhonya_cooldown_seconds, urf.item_haste);

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
            event_queue: BinaryHeap::new(),
            event_counter: 0,
            vlad_stats,
            max_health,
            health: max_health,
            physical_multiplier,
            magic_multiplier,
            pool_cooldown,
            pool_duration: 0.0,
            offensive_tuning,
            offensive_cooldowns,
            cast_profile,
            zhonya_available,
            ga_available,
            protoplasm_available,
            ga_cooldown,
            zhonya_cooldown,
            protoplasm_cooldown: 120.0,
            zhonya_cd: 0.0,
            ga_cd: 0.0,
            pool_cd: 0.0,
            q_cd: 0.0,
            e_cd: 0.0,
            r_cd: 0.0,
            protoplasm_cd: 0.0,
            pool_until: 0.0,
            stasis_until: 0.0,
            ga_res_until: 0.0,
            stunned_until: 0.0,
            protoplasm_shield: 0.0,
            pool_heal_rate: 0.0,
            pool_heal_until: 0.0,
            protoplasm_hot_rate: 0.0,
            protoplasm_hot_until: 0.0,
            vlad_position: Vec2 { x: 0.0, y: 0.0 },
            enemy_state: Vec::new(),
            projectile_block_zones: Vec::new(),
            trace_enabled: false,
            trace_events: Vec::new(),
        };

        runner.pool_duration = runner.sim.vlad_pool_untargetable_seconds;

        let enemy_count = enemies.len();
        for (idx, (enemy, build, enemy_bonus)) in enemies.iter().cloned().enumerate() {
            let model = derive_enemy_model(&enemy, &build, &enemy_bonus, &runner.sim, &runner.urf);
            let position = enemy_spawn_position(idx, enemy_count.max(1), model.behavior);

            runner.enemy_state.push(EnemyState {
                enemy: enemy.clone(),
                behavior: model.behavior,
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
            for spec in scripted_event_schedules(&enemy.name) {
                runner.schedule_event(
                    spec.start_offset_seconds.max(0.0),
                    12,
                    EventType::EnemyScript(idx, spec.event, 0),
                    Some(spec.interval_seconds.max(0.1)),
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
            && self.time >= self.ga_res_until
    }

    pub(super) fn can_cast(&self) -> bool {
        self.is_targetable() && self.time >= self.stunned_until
    }

    fn enemy_respawn_delay_seconds(&self) -> f64 {
        respawn::urf_respawn_delay_seconds(
            self.sim.champion_level,
            self.sim.urf_respawn_flat_reduction_seconds,
            self.sim.urf_respawn_extrapolation_per_level,
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
                state.uptime_active = uptime_window_active(
                    &state.enemy,
                    self.time,
                    self.sim.enemy_uptime_model_enabled,
                );
                respawned.push((idx, state.enemy.name.clone(), state.script_epoch));
            }
        }
        for (idx, name, epoch) in respawned {
            for spec in scripted_event_schedules(&self.enemy_state[idx].enemy.name) {
                self.schedule_event(
                    spec.start_offset_seconds.max(0.0),
                    12,
                    EventType::EnemyScript(idx, spec.event, epoch),
                    Some(spec.interval_seconds.max(0.1)),
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
                let msg = if active_now {
                    format!("{} re-entered combat window", state.enemy.name)
                } else {
                    format!("{} left combat window", state.enemy.name)
                };
                trace.push(("enemy_uptime", msg));
                state.script_epoch = state.script_epoch.wrapping_add(1);
                if active_now {
                    reschedule.push((idx, state.script_epoch, state.enemy.name.clone()));
                }
                state.uptime_active = active_now;
            }
        }
        for (idx, epoch, champion_name) in reschedule {
            for spec in scripted_event_schedules(&champion_name) {
                self.schedule_event(
                    spec.start_offset_seconds.max(0.0),
                    12,
                    EventType::EnemyScript(idx, spec.event, epoch),
                    Some(spec.interval_seconds.max(0.1)),
                );
            }
        }
        for (kind, msg) in trace {
            self.trace_event(kind, msg);
        }
    }

    fn enemy_distance_to_vlad(&self, idx: usize) -> f64 {
        self.enemy_state[idx]
            .position
            .distance_to(self.vlad_position)
    }

    fn enemy_in_attack_range(&self, idx: usize) -> bool {
        self.enemy_distance_to_vlad(idx) <= self.enemy_state[idx].behavior.attack_range
    }

    fn enemy_in_vlad_range(&self, idx: usize, range: f64) -> bool {
        self.enemy_distance_to_vlad(idx) <= range
    }

    fn enemy_projectile_delay(&self, idx: usize, speed: f64) -> f64 {
        projectile_travel_seconds(self.enemy_distance_to_vlad(idx), speed)
    }

    fn cleanup_expired_projectile_blocks(&mut self) {
        self.projectile_block_zones
            .retain(|zone| zone.expires_at > self.time);
    }

    fn is_projectile_blocked(&self, source: Vec2, target: Vec2) -> bool {
        self.projectile_block_zones
            .iter()
            .filter(|zone| zone.expires_at > self.time)
            .any(|zone| line_segments_intersect(source, target, zone.start, zone.end))
    }

    fn first_active_enemy_in_vlad_range(&self, range: f64) -> Option<usize> {
        let mut best: Option<(usize, f64)> = None;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_is_active(idx) || !self.enemy_in_vlad_range(idx, range) {
                continue;
            }
            let dist = self.enemy_distance_to_vlad(idx);
            match best {
                Some((_, best_dist)) if dist >= best_dist => {}
                _ => best = Some((idx, dist)),
            }
        }
        best.map(|(idx, _)| idx)
    }

    fn max_enemy_distance_in_vlad_range(&self, range: f64) -> Option<f64> {
        let mut max_distance = None;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_is_active(idx) || !self.enemy_in_vlad_range(idx, range) {
                continue;
            }
            let distance = self.enemy_distance_to_vlad(idx);
            max_distance = Some(match max_distance {
                Some(current) => distance.max(current),
                None => distance,
            });
        }
        max_distance
    }

    fn schedule_next_attack(&mut self, idx: usize) {
        if idx >= self.enemy_state.len() {
            return;
        }
        let state = &self.enemy_state[idx];
        let attack_speed = state.base_attack_speed * attack_speed_multiplier(&state.runtime);
        let interval = 1.0 / attack_speed.max(0.25);
        self.schedule_event(interval, 30, EventType::Attack(idx), None);
    }

    fn add_projectile_block_zone(&mut self, start: Vec2, end: Vec2, duration: f64) {
        self.projectile_block_zones.push(ProjectileBlockZone {
            start,
            end,
            expires_at: self.time + duration.max(0.0),
        });
        self.trace_event(
            "projectile_block",
            format!(
                "barrier created from ({:.1},{:.1}) to ({:.1},{:.1}) for {:.2}s",
                start.x,
                start.y,
                end.x,
                end.y,
                duration.max(0.0)
            ),
        );
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
                killed_name = Some(state.enemy.name.clone());
            }
            d
        };
        if let Some(name) = killed_name {
            self.enemy_kills_total += 1;
            self.trace_event(
                "enemy_death",
                format!("{} died; respawn in {:.1}s", name, respawn_delay),
            );
        }
        dealt
    }

    fn apply_magic_damage_to_all_active_enemies(&mut self, raw_magic_damage: f64) -> f64 {
        self.apply_magic_damage_to_enemies_in_vlad_range(raw_magic_damage, f64::INFINITY)
    }

    fn apply_magic_damage_to_enemies_in_vlad_range(
        &mut self,
        raw_magic_damage: f64,
        range: f64,
    ) -> f64 {
        if raw_magic_damage <= 0.0 {
            return 0.0;
        }
        let mut total = 0.0;
        for idx in 0..self.enemy_state.len() {
            if !self.enemy_in_vlad_range(idx, range) {
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
        if self.pool_heal_until > self.time {
            let active = delta.min(self.pool_heal_until - self.time);
            let before = self.health;
            self.health = self
                .max_health
                .min(self.health + self.pool_heal_rate * active);
            self.healing_done_total += (self.health - before).max(0.0);
        }
        if self.protoplasm_hot_until > self.time {
            let active = delta.min(self.protoplasm_hot_until - self.time);
            let before = self.health;
            self.health = self
                .max_health
                .min(self.health + self.protoplasm_hot_rate * active);
            self.healing_done_total += (self.health - before).max(0.0);
        }
        self.update_enemy_positions(delta);
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

    fn update_enemy_positions(&mut self, delta: f64) {
        if delta <= 0.0 {
            return;
        }
        for (idx, state) in self.enemy_state.iter_mut().enumerate() {
            if state.respawn_at.is_some() || state.health <= 0.0 {
                continue;
            }
            let speed = state.move_speed * state.behavior.movement_speed_scale;
            let step = speed * delta;
            let mut radial = Vec2 {
                x: state.position.x - self.vlad_position.x,
                y: state.position.y - self.vlad_position.y,
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

    fn apply_damage(&mut self, physical: f64, magic: f64, true_damage: f64) {
        if self.finished || self.health <= 0.0 || !self.is_targetable() {
            return;
        }
        let mut damage =
            physical * self.physical_multiplier + magic * self.magic_multiplier + true_damage;
        if self.protoplasm_shield > 0.0 && damage > 0.0 {
            let absorbed = self.protoplasm_shield.min(damage);
            self.protoplasm_shield -= absorbed;
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
    }

    fn handle_death(&mut self) {
        if self.ga_available && self.time >= self.ga_cd {
            self.ga_cd = self.time + self.ga_cooldown;
            self.ga_res_until = self.time + self.sim.ga_revive_duration_seconds;
            self.health =
                1.0_f64.max(self.vlad_base.base_health * self.sim.ga_revive_base_health_ratio);
            self.trace_event("ga_revive", "Guardian Angel revived Vladimir".to_string());
            return;
        }
        self.finished = true;
        self.death_time = Some(self.time);
        self.trace_event("vladimir_death", "Vladimir died".to_string());
    }

    fn maybe_cast_vlad_defensives(&mut self) {
        if self.finished {
            return;
        }
        self.refresh_enemy_respawns();

        if self.time >= self.pool_cd && self.can_cast() {
            self.pool_cd = self.time + self.pool_cooldown;
            self.pool_until = self.time + self.pool_duration;
            let cost = self.health
                * self.sim.vlad_pool_cost_percent_current_health
                * self.urf.health_cost_multiplier;
            self.health -= cost;

            let mut pool_damage =
                self.sim.vlad_pool_base_damage_by_rank[self.sim.vlad_pool_rank - 1];
            pool_damage += self.sim.vlad_pool_bonus_health_ratio
                * (self.vlad_stats.health - self.vlad_base.base_health);
            let total_pool_damage = self.apply_magic_damage_to_all_active_enemies(pool_damage);
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

        // Scripted offensive cadence for Vladimir abilities.
        if self.can_cast() {
            if self.time >= self.q_cd
                && let Some(target_idx) =
                    self.first_active_enemy_in_vlad_range(self.cast_profile.q_range)
            {
                self.q_cd = self.time + self.offensive_cooldowns.q_seconds;
                let travel = projectile_travel_seconds(
                    self.enemy_distance_to_vlad(target_idx),
                    self.cast_profile.q_projectile_speed,
                );
                self.schedule_event(
                    self.cast_profile.q_windup_seconds + travel,
                    50,
                    EventType::VladQHit(target_idx),
                    None,
                );
            }

            if self.time >= self.e_cd
                && let Some(max_distance) =
                    self.max_enemy_distance_in_vlad_range(self.cast_profile.e_range)
            {
                self.e_cd = self.time + self.offensive_cooldowns.e_seconds;
                let travel =
                    projectile_travel_seconds(max_distance, self.cast_profile.e_projectile_speed);
                self.schedule_event(
                    self.cast_profile.e_windup_seconds + travel,
                    49,
                    EventType::VladEHit,
                    None,
                );
            }

            if self.time >= self.r_cd
                && let Some(max_distance) =
                    self.max_enemy_distance_in_vlad_range(self.cast_profile.r_range)
            {
                self.r_cd = self.time + self.offensive_cooldowns.r_seconds;
                let travel =
                    projectile_travel_seconds(max_distance, self.cast_profile.r_projectile_speed);
                self.schedule_event(
                    self.cast_profile.r_windup_seconds + travel,
                    48,
                    EventType::VladRHit,
                    None,
                );
            }
        }

        if self.zhonya_available
            && self.time >= self.zhonya_cd
            && self.health <= self.max_health * self.sim.zhonya_trigger_health_percent
            && self.time >= self.pool_until
            && self.time >= self.ga_res_until
        {
            self.zhonya_cd = self.time + self.zhonya_cooldown;
            self.stasis_until = self.time + self.sim.zhonya_duration_seconds;
        }

        if self.protoplasm_available
            && self.time >= self.protoplasm_cd
            && self.health <= self.max_health * self.sim.protoplasm_trigger_health_percent
        {
            self.protoplasm_cd = self.time + self.protoplasm_cooldown;
            self.protoplasm_shield += self.sim.protoplasm_bonus_health;
            self.protoplasm_hot_rate =
                self.sim.protoplasm_heal_total / self.sim.protoplasm_duration_seconds.max(0.001);
            self.protoplasm_hot_until = self.time + self.sim.protoplasm_duration_seconds;
        }
    }

    fn process_event(&mut self, ev: &QueuedEvent) {
        match ev.kind {
            EventType::Attack(idx) => {
                if !self.enemy_is_active(idx) || !self.enemy_in_attack_range(idx) {
                    self.schedule_next_attack(idx);
                    return;
                }
                self.trace_event(
                    "attack_start",
                    format!("{} begins auto attack", self.enemy_state[idx].enemy.name),
                );
                let windup = self.enemy_state[idx]
                    .behavior
                    .attack_windup_seconds
                    .max(0.0);
                self.schedule_event(windup, 35, EventType::AttackWindup(idx), None);
            }
            EventType::AttackWindup(idx) => {
                if !self.enemy_is_active(idx) || !self.enemy_in_attack_range(idx) {
                    self.schedule_next_attack(idx);
                    return;
                }
                let travel = self.enemy_projectile_delay(
                    idx,
                    self.enemy_state[idx].behavior.attack_projectile_speed,
                );
                self.schedule_event(travel, 34, EventType::AttackHit(idx), None);
            }
            EventType::AttackHit(idx) => {
                if !self.enemy_is_active(idx) {
                    self.schedule_next_attack(idx);
                    return;
                }
                let target_current = self.health.max(0.0);
                let target_max = self.max_health.max(1.0);
                let (source, projectile_speed, physical, magic, true_damage) = {
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
                        state.position,
                        state.behavior.attack_projectile_speed,
                        attack_damage + extra_physical,
                        state.next_attack_bonus_magic + extra_magic,
                        state.next_attack_bonus_true + extra_true,
                    );
                    state.next_attack_bonus_physical = 0.0;
                    state.next_attack_bonus_magic = 0.0;
                    state.next_attack_bonus_true = 0.0;
                    out
                };
                if projectile_speed > 0.0 && self.is_projectile_blocked(source, self.vlad_position)
                {
                    self.trace_event(
                        "projectile_blocked",
                        format!("{} auto attack blocked", self.enemy_state[idx].enemy.name),
                    );
                    self.schedule_next_attack(idx);
                    return;
                }
                self.apply_damage(physical, magic, true_damage);
                self.trace_event(
                    "attack_hit",
                    format!(
                        "{} hit Vladimir (phys {:.1}, magic {:.1}, true {:.1})",
                        self.enemy_state[idx].enemy.name, physical, magic, true_damage
                    ),
                );
                self.schedule_next_attack(idx);
            }
            EventType::Ability(idx) => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let behavior = self.enemy_state[idx].behavior;
                let travel = self.enemy_projectile_delay(idx, behavior.ability_projectile_speed);
                self.schedule_event(
                    behavior.ability_windup_seconds.max(0.0) + travel,
                    45,
                    EventType::AbilityHit(idx),
                    None,
                );
            }
            EventType::AbilityHit(idx) => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let target_max = self.max_health.max(1.0);
                let (source, projectile_speed, magic, true_damage) = {
                    let state = &mut self.enemy_state[idx];
                    let (extra_magic, extra_true) = on_ability_bonus_damage(
                        &mut state.runtime,
                        state.ability_hit_damage,
                        target_max,
                        self.time,
                    );
                    (
                        state.position,
                        state.behavior.ability_projectile_speed,
                        state.ability_hit_damage + extra_magic,
                        extra_true,
                    )
                };
                if projectile_speed > 0.0 && self.is_projectile_blocked(source, self.vlad_position)
                {
                    self.trace_event(
                        "projectile_blocked",
                        format!("{} ability blocked", self.enemy_state[idx].enemy.name),
                    );
                    return;
                }
                self.apply_damage(0.0, magic, true_damage);
                self.trace_event(
                    "ability_hit",
                    format!(
                        "{} ability hit Vladimir (magic {:.1}, true {:.1})",
                        self.enemy_state[idx].enemy.name, magic, true_damage
                    ),
                );
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
                }
            }
            EventType::Burst(idx) => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let behavior = self.enemy_state[idx].behavior;
                let travel = self.enemy_projectile_delay(idx, behavior.burst_projectile_speed);
                self.schedule_event(
                    behavior.burst_windup_seconds.max(0.0) + travel,
                    25,
                    EventType::BurstHit(idx),
                    None,
                );
            }
            EventType::BurstHit(idx) => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let target_max = self.max_health.max(1.0);
                let (source, projectile_speed, physical, magic, true_damage) = {
                    let state = &mut self.enemy_state[idx];
                    let (extra_magic, extra_true) = on_ability_bonus_damage(
                        &mut state.runtime,
                        state.burst_magic_damage,
                        target_max,
                        self.time,
                    );
                    (
                        state.position,
                        state.behavior.burst_projectile_speed,
                        state.burst_physical_damage,
                        state.burst_magic_damage + extra_magic,
                        state.burst_true_damage + extra_true,
                    )
                };
                if projectile_speed > 0.0 && self.is_projectile_blocked(source, self.vlad_position)
                {
                    self.trace_event(
                        "projectile_blocked",
                        format!("{} burst blocked", self.enemy_state[idx].enemy.name),
                    );
                    return;
                }
                self.apply_damage(physical, magic, true_damage);
                self.trace_event(
                    "burst_hit",
                    format!(
                        "{} burst hit Vladimir (phys {:.1}, magic {:.1}, true {:.1})",
                        self.enemy_state[idx].enemy.name, physical, magic, true_damage
                    ),
                );
            }
            EventType::VladQHit(idx) => {
                if idx >= self.enemy_state.len() || !self.enemy_is_active(idx) {
                    return;
                }
                if self.cast_profile.q_projectile_speed > 0.0
                    && self
                        .is_projectile_blocked(self.vlad_position, self.enemy_state[idx].position)
                {
                    self.trace_event("projectile_blocked", "Vladimir Q blocked".to_string());
                    return;
                }
                let q_raw_damage =
                    q_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let dealt = self.apply_magic_damage_to_enemy(idx, q_raw_damage);
                self.damage_dealt_total += dealt.max(0.0);
                if dealt > 0.0 {
                    let before = self.health;
                    self.health = self
                        .max_health
                        .min(self.health + dealt * self.offensive_tuning.q_heal_ratio_of_damage);
                    self.healing_done_total += (self.health - before).max(0.0);
                }
                self.trace_event(
                    "vladimir_q_hit",
                    format!(
                        "Vladimir Q hit {} for {:.1}",
                        self.enemy_state[idx].enemy.name, dealt
                    ),
                );
            }
            EventType::VladEHit => {
                let e_raw_damage =
                    e_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let dealt = self.apply_magic_damage_to_enemies_in_vlad_range(
                    e_raw_damage,
                    self.cast_profile.e_range,
                );
                self.damage_dealt_total += dealt.max(0.0);
                self.trace_event("vladimir_e_hit", format!("Vladimir E dealt {:.1}", dealt));
            }
            EventType::VladRHit => {
                let r_raw_damage =
                    r_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let dealt = self.apply_magic_damage_to_enemies_in_vlad_range(
                    r_raw_damage,
                    self.cast_profile.r_range,
                );
                self.damage_dealt_total += dealt.max(0.0);
                self.trace_event("vladimir_r_hit", format!("Vladimir R dealt {:.1}", dealt));
            }
            EventType::EnemyScript(idx, script_event, epoch) => {
                if idx >= self.enemy_state.len()
                    || self.enemy_state[idx].script_epoch != epoch
                    || !self.enemy_is_active(idx)
                {
                    return;
                }
                self.trace_event(
                    "enemy_script",
                    format!(
                        "{} executed {:?}",
                        self.enemy_state[idx].enemy.name, script_event
                    ),
                );
                match script_event {
                    EnemyScriptEvent::WarwickInfiniteDuress => {
                        if !self.enemy_in_vlad_range(idx, 700.0) {
                            return;
                        }
                        let (physical, magic) = {
                            let state = &self.enemy_state[idx];
                            (
                                state.physical_hit_damage * 1.8,
                                80.0 + 0.25 * state.physical_hit_damage,
                            )
                        };
                        self.apply_damage(physical, magic, 0.0);
                        if self.is_targetable() {
                            self.stunned_until = self.stunned_until.max(self.time + 1.4);
                        }
                    }
                    EnemyScriptEvent::VayneTumbleEmpower => {
                        let enemy_name = {
                            let state = &mut self.enemy_state[idx];
                            state.next_attack_bonus_physical += 0.75 * state.physical_hit_damage;
                            state.enemy.name.clone()
                        };
                        self.trace_event(
                            "enemy_buff",
                            format!("{} empowered next attack", enemy_name),
                        );
                    }
                    EnemyScriptEvent::MorganaDarkBinding => {
                        if !self.enemy_in_vlad_range(idx, 1300.0) {
                            return;
                        }
                        let (source, projectile_speed, magic, true_damage) = {
                            let state = &mut self.enemy_state[idx];
                            let raw_magic = 140.0 + 0.25 * state.burst_magic_damage.max(0.0);
                            let (extra_magic, extra_true) = on_ability_bonus_damage(
                                &mut state.runtime,
                                raw_magic,
                                self.max_health,
                                self.time,
                            );
                            (
                                state.position,
                                state.behavior.ability_projectile_speed,
                                raw_magic + extra_magic,
                                extra_true,
                            )
                        };
                        if projectile_speed > 0.0
                            && self.is_projectile_blocked(source, self.vlad_position)
                        {
                            return;
                        }
                        self.apply_damage(0.0, magic, true_damage);
                        if self.is_targetable() {
                            self.stunned_until = self.stunned_until.max(self.time + 2.0);
                        }
                    }
                    EnemyScriptEvent::MorganaSoulShackles => {
                        if !self.enemy_in_vlad_range(idx, 650.0) {
                            return;
                        }
                        self.apply_damage(0.0, 70.0, 0.0);
                        self.schedule_event(
                            2.5,
                            11,
                            EventType::EnemyScript(
                                idx,
                                EnemyScriptEvent::MorganaSoulShacklesDetonate,
                                epoch,
                            ),
                            None,
                        );
                    }
                    EnemyScriptEvent::MorganaSoulShacklesDetonate => {
                        if !self.enemy_in_vlad_range(idx, 700.0) {
                            return;
                        }
                        self.apply_damage(0.0, 170.0, 0.0);
                        if self.is_targetable() {
                            self.stunned_until = self.stunned_until.max(self.time + 1.5);
                        }
                    }
                    EnemyScriptEvent::SonaCrescendo => {
                        if !self.enemy_in_vlad_range(idx, 1000.0) {
                            return;
                        }
                        let (source, projectile_speed, magic, true_damage) = {
                            let state = &mut self.enemy_state[idx];
                            let raw_magic = 190.0 + 0.20 * state.burst_magic_damage.max(0.0);
                            let (extra_magic, extra_true) = on_ability_bonus_damage(
                                &mut state.runtime,
                                raw_magic,
                                self.max_health,
                                self.time,
                            );
                            (
                                state.position,
                                state.behavior.burst_projectile_speed,
                                raw_magic + extra_magic,
                                extra_true,
                            )
                        };
                        if projectile_speed > 0.0
                            && self.is_projectile_blocked(source, self.vlad_position)
                        {
                            return;
                        }
                        self.apply_damage(0.0, magic, true_damage);
                        if self.is_targetable() {
                            self.stunned_until = self.stunned_until.max(self.time + 1.5);
                        }
                    }
                    EnemyScriptEvent::DrMundoInfectedCleaver => {
                        if !self.enemy_in_vlad_range(idx, 1050.0) {
                            return;
                        }
                        let (source, projectile_speed, magic, true_damage) = {
                            let state = &mut self.enemy_state[idx];
                            let raw_magic = (0.15 * self.health.max(0.0)).clamp(80.0, 320.0) + 20.0;
                            let (extra_magic, extra_true) = on_ability_bonus_damage(
                                &mut state.runtime,
                                raw_magic,
                                self.max_health,
                                self.time,
                            );
                            (
                                state.position,
                                state.behavior.ability_projectile_speed,
                                raw_magic + extra_magic,
                                extra_true,
                            )
                        };
                        if projectile_speed > 0.0
                            && self.is_projectile_blocked(source, self.vlad_position)
                        {
                            return;
                        }
                        self.apply_damage(0.0, magic, true_damage);
                    }
                    EnemyScriptEvent::YasuoWindWall => {
                        let state = &self.enemy_state[idx];
                        let to_vlad = Vec2 {
                            x: self.vlad_position.x - state.position.x,
                            y: self.vlad_position.y - state.position.y,
                        };
                        let length = to_vlad.distance_to(Vec2 { x: 0.0, y: 0.0 }).max(1e-6);
                        let nx = to_vlad.x / length;
                        let ny = to_vlad.y / length;
                        let center = Vec2 {
                            x: state.position.x + nx * 180.0,
                            y: state.position.y + ny * 180.0,
                        };
                        let tangent = Vec2 { x: -ny, y: nx };
                        let half_width = 180.0;
                        let start = Vec2 {
                            x: center.x + tangent.x * half_width,
                            y: center.y + tangent.y * half_width,
                        };
                        let end = Vec2 {
                            x: center.x - tangent.x * half_width,
                            y: center.y - tangent.y * half_width,
                        };
                        self.add_projectile_block_zone(start, end, 4.0);
                    }
                }
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
            self.maybe_cast_vlad_defensives();

            while let Some(top) = self.event_queue.peek().cloned() {
                if top.time > target_time || self.finished {
                    break;
                }
                self.event_queue.pop();
                self.apply_hot_effects(top.time);
                self.refresh_enemy_respawns();
                self.process_event(&top);
                let should_recur = match &top.kind {
                    EventType::EnemyScript(idx, _, epoch) => self
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
                self.maybe_cast_vlad_defensives();
            }

            self.apply_hot_effects(target_time);
            self.refresh_enemy_respawns();
            self.maybe_cast_vlad_defensives();

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
        sim.champion_level,
        None,
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
    let runtime = build_enemy_loadout_runtime(
        &runtime_item_names,
        &enemy.loadout_rune_names,
        &enemy.loadout_masteries,
    );
    attack_speed = base_attack_speed * attack_speed_multiplier(&runtime);

    let attack_interval = 1.0 / attack_speed.max(0.001);
    let ability_interval = enemy.ability_tick_interval_seconds.max(0.05);
    let ability_hit_damage = magic_dps * ability_interval;
    let burst_physical_damage = enemy.burst_physical_flat + enemy.burst_ad_ratio * attack_damage;
    let burst_magic_damage =
        enemy.burst_magic_flat + enemy.burst_ap_ratio * enemy_stats.ability_power;
    let burst_true_damage = enemy.burst_true_flat;
    let behavior = behavior_profile(&enemy.name, enemy.base.is_melee);

    EnemyDerivedModel {
        behavior,
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

pub(super) fn simulate_vlad_combat(
    vlad_base: &ChampionBase,
    vlad_build_items: &[Item],
    vlad_bonus_stats: &Stats,
    vlad_item_acquired_levels: Option<&HashMap<String, usize>>,
    enemies: &[(EnemyConfig, Vec<Item>, Stats)],
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> CombatOutcome {
    let mut runner = VladCombatSimulation::new(
        vlad_base.clone(),
        vlad_build_items,
        vlad_bonus_stats,
        vlad_item_acquired_levels,
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
        let melee = EnemyBehaviorProfile::default_for(true);
        let ranged = EnemyBehaviorProfile::default_for(false);
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
}

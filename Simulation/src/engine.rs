use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::scripts::enemies::{EnemyBehaviorProfile, behavior_profile, on_hit_bonus_damage};
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

struct EnemyState {
    enemy: EnemyConfig,
    behavior: EnemyBehaviorProfile,
    position: Vec2,
    physical_hit_damage: f64,
    ability_hit_damage: f64,
    burst_physical_damage: f64,
    burst_magic_damage: f64,
    burst_true_damage: f64,
    attacks_landed: usize,
    max_health: f64,
    health: f64,
    magic_multiplier: f64,
    respawn_at: Option<f64>,
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
        };

        runner.pool_duration = runner.sim.vlad_pool_untargetable_seconds;

        let enemy_count = enemies.len();
        for (idx, (enemy, build, enemy_bonus)) in enemies.iter().cloned().enumerate() {
            let mut enemy_stats = Stats::default();
            for item in &build {
                enemy_stats.add(&item.stats);
            }
            enemy_stats.add(&enemy_bonus);
            apply_item_assumptions(
                &mut enemy_stats,
                &enemy.base,
                &build,
                &runner.sim,
                runner.sim.champion_level,
                None,
            );
            let (_physical_dps, magic_dps) = compute_enemy_dps(&enemy, &enemy_stats, &runner.urf);
            let attack_damage = enemy.base.base_attack_damage + enemy_stats.attack_damage;
            let magic_resist = enemy.base.base_magic_resist + enemy_stats.magic_resist;
            let max_health = (enemy.base.base_health + enemy_stats.health).max(1.0);
            let attack_speed_bonus = enemy_stats.attack_speed_percent / 100.0;
            let mut attack_speed = enemy.base.base_attack_speed * (1.0 + attack_speed_bonus);
            attack_speed *= if enemy.base.is_melee {
                runner.urf.bonus_attack_speed_multiplier_melee
            } else {
                runner.urf.bonus_attack_speed_multiplier_ranged
            };
            let attack_interval = 1.0 / attack_speed.max(0.001);
            let ability_interval = enemy.ability_tick_interval_seconds.max(0.05);
            let ability_hit_damage = magic_dps * ability_interval;
            let burst_physical_damage =
                enemy.burst_physical_flat + enemy.burst_ad_ratio * attack_damage;
            let burst_magic_damage =
                enemy.burst_magic_flat + enemy.burst_ap_ratio * enemy_stats.ability_power;
            let burst_true_damage = enemy.burst_true_flat;
            let behavior = behavior_profile(&enemy.name, enemy.base.is_melee);
            let position = enemy_spawn_position(idx, enemy_count.max(1), behavior);

            runner.enemy_state.push(EnemyState {
                enemy: enemy.clone(),
                behavior,
                position,
                physical_hit_damage: attack_damage,
                ability_hit_damage,
                burst_physical_damage,
                burst_magic_damage,
                burst_true_damage,
                attacks_landed: 0,
                max_health,
                health: max_health,
                magic_multiplier: 100.0 / (100.0 + magic_resist.max(0.0)),
                respawn_at: None,
            });

            runner.schedule_event(
                attack_interval,
                30,
                EventType::Attack(idx),
                Some(attack_interval),
            );
            if ability_hit_damage > 0.0 {
                runner.schedule_event(
                    ability_interval,
                    40,
                    EventType::Ability(idx),
                    Some(ability_interval),
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
                && (burst_physical_damage > 0.0
                    || burst_magic_damage > 0.0
                    || burst_true_damage > 0.0)
            {
                runner.schedule_event(
                    enemy.burst_start_offset_seconds.max(0.0),
                    10,
                    EventType::Burst(idx),
                    Some(enemy.burst_interval_seconds),
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
        for state in &mut self.enemy_state {
            let Some(respawn_at) = state.respawn_at else {
                continue;
            };
            if self.time >= respawn_at {
                state.health = state.max_health;
                state.respawn_at = None;
            }
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
        if !self.sim.enemy_uptime_model_enabled {
            return true;
        }
        let state = &self.enemy_state[idx];
        let cycle = state.enemy.uptime_cycle_seconds;
        let active = state.enemy.uptime_active_seconds;
        if cycle <= 0.0 || active <= 0.0 || active >= cycle {
            return true;
        }
        let phase = state.enemy.uptime_phase_seconds.max(0.0);
        let t = (self.time + phase) % cycle;
        t <= active
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
        let mut killed = false;
        let dealt = {
            let state = &mut self.enemy_state[idx];
            let d = mitigated.min(state.health.max(0.0));
            state.health -= d;
            if state.health <= 0.0 {
                state.health = 0.0;
                state.respawn_at = Some(self.time + respawn_delay);
                killed = true;
            }
            d
        };
        if killed {
            self.enemy_kills_total += 1;
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
        self.time = to_time;
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
            return;
        }
        self.finished = true;
        self.death_time = Some(self.time);
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
                    return;
                }
                let windup = self.enemy_state[idx]
                    .behavior
                    .attack_windup_seconds
                    .max(0.0);
                self.schedule_event(windup, 35, EventType::AttackWindup(idx), None);
            }
            EventType::AttackWindup(idx) => {
                if !self.enemy_is_active(idx) || !self.enemy_in_attack_range(idx) {
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
                    return;
                }
                let (physical, magic, true_damage) = {
                    let state = &mut self.enemy_state[idx];
                    state.attacks_landed += 1;
                    let (magic_bonus, true_bonus) = on_hit_bonus_damage(
                        state.behavior,
                        state.attacks_landed,
                        state.physical_hit_damage,
                        self.max_health,
                    );
                    (state.physical_hit_damage, magic_bonus, true_bonus)
                };
                self.apply_damage(physical, magic, true_damage);
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
                let state = &self.enemy_state[idx];
                self.apply_damage(0.0, state.ability_hit_damage, 0.0);
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
                let state = &self.enemy_state[idx];
                self.apply_damage(
                    state.burst_physical_damage,
                    state.burst_magic_damage,
                    state.burst_true_damage,
                );
            }
            EventType::VladQHit(idx) => {
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
            }
            EventType::VladEHit => {
                let e_raw_damage =
                    e_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let dealt = self.apply_magic_damage_to_enemies_in_vlad_range(
                    e_raw_damage,
                    self.cast_profile.e_range,
                );
                self.damage_dealt_total += dealt.max(0.0);
            }
            EventType::VladRHit => {
                let r_raw_damage =
                    r_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let dealt = self.apply_magic_damage_to_enemies_in_vlad_range(
                    r_raw_damage,
                    self.cast_profile.r_range,
                );
                self.damage_dealt_total += dealt.max(0.0);
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
                if let Some(recurring) = top.recurring
                    && recurring > 0.0
                    && !self.finished
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
}

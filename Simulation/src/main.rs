use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, ValueEnum};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use serde_json::{Value, json};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

mod cache;
mod respawn;
mod scripts;
mod status;

use crate::cache::{BlockingScoreCache, PersistentScoreCache};
use crate::scripts::vladimir::{
    VladimirAbilityCooldowns, VladimirAbilityTuning, e_damage_raw, offensive_cooldowns_after_haste,
    q_damage_raw, r_damage_raw,
};
use crate::status::{StatusReporter, deadline_reached};

const EXCLUDED_RANKS: &[&str] = &["CONSUMABLE", "TRINKET"];
const LEGENDARY_RANK: &str = "LEGENDARY";
const ITEM_EVOLUTION_REPLACEMENTS: &[(&str, &str)] = &[
    ("Manamune", "Muramana"),
    ("Archangel's Staff", "Seraph's Embrace"),
];

#[derive(Debug, Clone, Default)]
struct Stats {
    ability_power: f64,
    health: f64,
    armor: f64,
    magic_resist: f64,
    attack_damage: f64,
    attack_speed_percent: f64,
    ability_haste: f64,
    move_speed_flat: f64,
    move_speed_percent: f64,
    crit_chance_percent: f64,
}

impl Stats {
    fn add(&mut self, other: &Stats) {
        self.ability_power += other.ability_power;
        self.health += other.health;
        self.armor += other.armor;
        self.magic_resist += other.magic_resist;
        self.attack_damage += other.attack_damage;
        self.attack_speed_percent += other.attack_speed_percent;
        self.ability_haste += other.ability_haste;
        self.move_speed_flat += other.move_speed_flat;
        self.move_speed_percent += other.move_speed_percent;
        self.crit_chance_percent += other.crit_chance_percent;
    }

    fn get_stat(&self, key: &str) -> f64 {
        match key {
            "ability_power" => self.ability_power,
            "health" => self.health,
            "armor" => self.armor,
            "magic_resist" => self.magic_resist,
            "attack_damage" => self.attack_damage,
            "attack_speed_percent" => self.attack_speed_percent,
            "ability_haste" => self.ability_haste,
            "move_speed_flat" => self.move_speed_flat,
            "move_speed_percent" => self.move_speed_percent,
            "crit_chance_percent" => self.crit_chance_percent,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    stats: Stats,
    rank: Vec<String>,
    shop_purchasable: bool,
    total_cost: f64,
    passive_effects_text: Vec<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ChampionBase {
    name: String,
    base_health: f64,
    health_per_level: f64,
    base_armor: f64,
    armor_per_level: f64,
    base_magic_resist: f64,
    magic_resist_per_level: f64,
    base_attack_damage: f64,
    attack_damage_per_level: f64,
    base_attack_speed: f64,
    attack_speed_per_level_percent: f64,
    base_move_speed: f64,
    is_melee: bool,
}

#[derive(Debug, Clone)]
struct EnemyConfig {
    name: String,
    base: ChampionBase,
    ability_dps_flat: f64,
    ability_dps_ad_ratio: f64,
    ability_dps_ap_ratio: f64,
    ability_tick_interval_seconds: f64,
    stun_interval_seconds: f64,
    stun_duration_seconds: f64,
    burst_interval_seconds: f64,
    burst_start_offset_seconds: f64,
    burst_magic_flat: f64,
    burst_physical_flat: f64,
    burst_true_flat: f64,
    burst_ad_ratio: f64,
    burst_ap_ratio: f64,
    uptime_cycle_seconds: f64,
    uptime_active_seconds: f64,
    uptime_phase_seconds: f64,
}

#[derive(Debug, Clone)]
struct SimulationConfig {
    dt: f64,
    server_tick_rate_hz: f64,
    champion_level: usize,
    max_time_seconds: f64,
    vlad_pool_rank: usize,
    vlad_pool_untargetable_seconds: f64,
    vlad_pool_cost_percent_current_health: f64,
    vlad_pool_heal_ratio_of_damage: f64,
    vlad_pool_base_damage_by_rank: Vec<f64>,
    vlad_pool_bonus_health_ratio: f64,
    zhonya_duration_seconds: f64,
    zhonya_cooldown_seconds: f64,
    zhonya_trigger_health_percent: f64,
    ga_cooldown_seconds: f64,
    ga_revive_duration_seconds: f64,
    ga_revive_base_health_ratio: f64,
    protoplasm_trigger_health_percent: f64,
    protoplasm_bonus_health: f64,
    protoplasm_heal_total: f64,
    protoplasm_duration_seconds: f64,
    heartsteel_assumed_stacks_at_8m: f64,
    enemy_uptime_model_enabled: bool,
    urf_respawn_flat_reduction_seconds: f64,
    urf_respawn_extrapolation_per_level: f64,
    vlad_q_base_damage: f64,
    vlad_q_ap_ratio: f64,
    vlad_q_heal_ratio_of_damage: f64,
    vlad_q_base_cooldown_seconds: f64,
    vlad_e_base_damage: f64,
    vlad_e_ap_ratio: f64,
    vlad_e_base_cooldown_seconds: f64,
    vlad_r_base_damage: f64,
    vlad_r_ap_ratio: f64,
    vlad_r_base_cooldown_seconds: f64,
}

#[derive(Debug, Clone)]
struct UrfBuffs {
    ability_haste: f64,
    item_haste: f64,
    health_cost_multiplier: f64,
    bonus_attack_speed_multiplier_melee: f64,
    bonus_attack_speed_multiplier_ranged: f64,
}

#[derive(Debug, Clone)]
struct BuildSearchConfig {
    strategy: String,
    beam_width: usize,
    max_items: usize,
    random_samples: usize,
    hill_climb_restarts: usize,
    hill_climb_steps: usize,
    hill_climb_neighbors: usize,
    genetic_population: usize,
    genetic_generations: usize,
    genetic_mutation_rate: f64,
    genetic_crossover_rate: f64,
    portfolio_strategies: Vec<String>,
    ranked_limit: usize,
    simulated_annealing_restarts: usize,
    simulated_annealing_iterations: usize,
    simulated_annealing_initial_temp: f64,
    simulated_annealing_cooling_rate: f64,
    mcts_iterations: usize,
    mcts_rollouts_per_expansion: usize,
    mcts_exploration: f64,
    ensemble_seeds: usize,
    ensemble_seed_stride: u64,
    ensemble_seed_top_k: usize,
    objective_survival_weight: f64,
    objective_damage_weight: f64,
    objective_healing_weight: f64,
    robust_min_seed_hit_rate: f64,
    bleed_enabled: bool,
    bleed_budget: usize,
    bleed_mutation_rate: f64,
    multi_scenario_worst_weight: f64,
    seed: u64,
}

#[derive(Debug, Clone)]
struct BuildMetrics {
    objective: f64,
    ehp_mixed: f64,
    ap: f64,
    cost_timing: f64,
    total_cost: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct CombatOutcome {
    time_alive_seconds: f64,
    damage_dealt: f64,
    healing_done: f64,
    enemy_kills: usize,
}

#[derive(Debug, Clone, Copy)]
struct ObjectiveComponentWeights {
    survival: f64,
    damage: f64,
    healing: f64,
}

#[derive(Debug, Clone)]
struct BuildConfidence {
    key: Vec<usize>,
    seed_hits: usize,
    seed_hit_rate: f64,
    robustness: String,
}

#[derive(Debug, Clone)]
struct SearchDiagnostics {
    strategy_summary: String,
    search_quality_profile: String,
    ensemble_seeds: usize,
    objective_survival_weight: f64,
    objective_damage_weight: f64,
    objective_healing_weight: f64,
    full_evaluations: usize,
    full_cache_hits: usize,
    full_cache_misses: usize,
    full_cache_waits: usize,
    full_persistent_cache_hits: usize,
    full_persistent_cache_entries: usize,
    unique_candidate_builds: usize,
    bleed_candidates_injected: usize,
    adaptive_candidates_injected: usize,
    scenario_count: usize,
    loadout_candidates: usize,
    loadout_finalists: usize,
    time_budget_seconds: Option<f64>,
    elapsed_seconds: f64,
    timed_out: bool,
    processed_candidates: usize,
    total_candidates: usize,
    seed_best_scores: Vec<f64>,
}

#[derive(Debug, Clone, Default)]
struct MasterySelection {
    name: String,
    rank: usize,
}

#[derive(Debug, Clone, Default)]
struct LoadoutSelection {
    rune_ids: Vec<i64>,
    rune_names: Vec<String>,
    shard_stats: Vec<String>,
    masteries: Vec<MasterySelection>,
}

#[derive(Debug, Clone, Default)]
struct ResolvedLoadout {
    selection_labels: Vec<String>,
    bonus_stats: Stats,
    applied_notes: Vec<String>,
    skipped_notes: Vec<String>,
}

#[derive(Debug, Clone)]
struct BuildOrderResult {
    ordered_items: Vec<Item>,
    levels: Vec<usize>,
    acquired_levels: Vec<usize>,
    stage_survival: Vec<f64>,
    cumulative_score: f64,
}

#[derive(Debug, Clone)]
struct EnemyState {
    enemy: EnemyConfig,
    physical_hit_damage: f64,
    ability_hit_damage: f64,
    burst_physical_damage: f64,
    burst_magic_damage: f64,
    burst_true_damage: f64,
    max_health: f64,
    health: f64,
    magic_multiplier: f64,
    respawn_at: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventType {
    EnemyAttack(usize),
    EnemyAbility(usize),
    EnemyStun(usize),
    EnemyBurst(usize),
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

struct VladCombatSimulation {
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

    enemy_state: Vec<EnemyState>,
}

impl VladCombatSimulation {
    fn new(
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
            enemy_state: Vec::new(),
        };

        runner.pool_duration = runner.sim.vlad_pool_untargetable_seconds;

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

            runner.enemy_state.push(EnemyState {
                enemy: enemy.clone(),
                physical_hit_damage: attack_damage,
                ability_hit_damage,
                burst_physical_damage,
                burst_magic_damage,
                burst_true_damage,
                max_health,
                health: max_health,
                magic_multiplier: 100.0 / (100.0 + magic_resist.max(0.0)),
                respawn_at: None,
            });

            runner.schedule_event(
                attack_interval,
                30,
                EventType::EnemyAttack(idx),
                Some(attack_interval),
            );
            if ability_hit_damage > 0.0 {
                runner.schedule_event(
                    ability_interval,
                    40,
                    EventType::EnemyAbility(idx),
                    Some(ability_interval),
                );
            }
            if enemy.stun_interval_seconds > 0.0 {
                runner.schedule_event(
                    enemy.stun_interval_seconds,
                    20,
                    EventType::EnemyStun(idx),
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
                    EventType::EnemyBurst(idx),
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

    fn is_targetable(&self) -> bool {
        self.time >= self.pool_until
            && self.time >= self.stasis_until
            && self.time >= self.ga_res_until
    }

    fn can_cast(&self) -> bool {
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
        if raw_magic_damage <= 0.0 {
            return 0.0;
        }
        let mut total = 0.0;
        for idx in 0..self.enemy_state.len() {
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
            if self.time >= self.q_cd {
                self.q_cd = self.time + self.offensive_cooldowns.q_seconds;
                let q_raw_damage =
                    q_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let mut dealt = 0.0;
                if let Some(target_idx) =
                    (0..self.enemy_state.len()).find(|idx| self.enemy_is_active(*idx))
                {
                    dealt = self.apply_magic_damage_to_enemy(target_idx, q_raw_damage);
                }
                self.damage_dealt_total += dealt.max(0.0);
                if dealt > 0.0 {
                    let before = self.health;
                    self.health = self
                        .max_health
                        .min(self.health + dealt * self.offensive_tuning.q_heal_ratio_of_damage);
                    self.healing_done_total += (self.health - before).max(0.0);
                }
            }

            if self.time >= self.e_cd {
                self.e_cd = self.time + self.offensive_cooldowns.e_seconds;
                let e_raw_damage =
                    e_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let dealt = self.apply_magic_damage_to_all_active_enemies(e_raw_damage);
                self.damage_dealt_total += dealt.max(0.0);
            }

            if self.time >= self.r_cd {
                self.r_cd = self.time + self.offensive_cooldowns.r_seconds;
                let r_raw_damage =
                    r_damage_raw(self.offensive_tuning, self.vlad_stats.ability_power);
                let dealt = self.apply_magic_damage_to_all_active_enemies(r_raw_damage);
                self.damage_dealt_total += dealt.max(0.0);
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
            EventType::EnemyAttack(idx) => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let state = &self.enemy_state[idx];
                self.apply_damage(state.physical_hit_damage, 0.0, 0.0);
            }
            EventType::EnemyAbility(idx) => {
                if !self.enemy_is_active(idx) {
                    return;
                }
                let state = &self.enemy_state[idx];
                self.apply_damage(0.0, state.ability_hit_damage, 0.0);
            }
            EventType::EnemyStun(idx) => {
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
            EventType::EnemyBurst(idx) => {
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
        }
    }

    fn step(&mut self, ticks: usize) -> bool {
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
                if let Some(recurring) = top.recurring {
                    if recurring > 0.0 && !self.finished {
                        self.event_counter += 1;
                        self.event_queue.push(QueuedEvent {
                            time: top.time + recurring,
                            priority: top.priority,
                            seq: self.event_counter,
                            recurring: top.recurring,
                            kind: top.kind.clone(),
                        });
                    }
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
}

#[derive(Debug, Clone, Parser)]
#[command(about = "URF Vladimir objective simulator")]
struct Cli {
    #[arg(long)]
    scenario: String,
    #[arg(long, value_enum, default_value_t = Mode::Vladimir)]
    mode: Mode,
    #[arg(long, default_value_t = 30)]
    ticks: usize,
    #[arg(long, default_value_t = 8)]
    top_x: usize,
    #[arg(long, default_value_t = 2)]
    min_item_diff: usize,
    #[arg(long, default_value_t = 5.0)]
    max_relative_gap_percent: f64,
    #[arg(long)]
    report_path: Option<String>,
    #[arg(long)]
    threads: Option<usize>,
    #[arg(long)]
    max_runtime_seconds: Option<f64>,
    #[arg(long, default_value_t = 10.0)]
    status_every_seconds: f64,
    #[arg(long, value_enum, default_value_t = SearchQualityProfile::MaximumQuality)]
    search_quality_profile: SearchQualityProfile,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    #[value(name = "vladimir", alias = "vlad")]
    Vladimir,
    #[value(name = "vladimir_step", alias = "vlad_step")]
    VladimirStep,
    #[value(name = "taric_as")]
    TaricAs,
    #[value(name = "hecarim_ms")]
    HecarimMs,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SearchQualityProfile {
    #[value(name = "fast")]
    Fast,
    #[value(name = "balanced")]
    Balanced,
    #[value(name = "maximum_quality")]
    MaximumQuality,
}

fn simulation_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn items_dir() -> PathBuf {
    simulation_dir().join("..").join("Items")
}

fn game_mode_dir() -> PathBuf {
    simulation_dir().join("..").join("Game Mode")
}

fn characters_dir() -> PathBuf {
    simulation_dir().join("..").join("Characters")
}

fn masteries_dir() -> PathBuf {
    simulation_dir().join("..").join("Masteries")
}

fn simulation_data_dir() -> PathBuf {
    simulation_dir().join("data")
}

fn load_json(path: &Path) -> Result<Value> {
    let text =
        fs::read_to_string(path).with_context(|| format!("Failed reading {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("Failed parsing {}", path.display()))
}

fn to_norm_key(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn value_from_effect(effect: &Value, rank: usize, level: usize) -> Option<f64> {
    if let Some(v) = effect
        .get("formula")
        .and_then(|f| f.get("value"))
        .and_then(Value::as_f64)
    {
        return Some(v);
    }
    if let Some(v) = effect.get("value").and_then(Value::as_f64) {
        return Some(v);
    }
    if let Some(values) = effect.get("values").and_then(Value::as_array) {
        let idx = rank.saturating_sub(1).min(values.len().saturating_sub(1));
        if let Some(v) = values.get(idx).and_then(Value::as_f64) {
            return Some(v);
        }
    }
    if let Some(vr) = effect.get("value_range").and_then(Value::as_object) {
        if let (Some(min), Some(max)) = (
            vr.get("min").and_then(Value::as_f64),
            vr.get("max").and_then(Value::as_f64),
        ) {
            let t = ((level.max(1) as f64 - 1.0) / 29.0).clamp(0.0, 1.0);
            return Some(min + (max - min) * t);
        }
    }
    None
}

fn apply_stat_bonus(
    stats: &mut Stats,
    stat: &str,
    value: f64,
    is_percent_unit: bool,
    for_vlad: bool,
) -> bool {
    match stat {
        "health" => {
            stats.health += value;
            true
        }
        "armor" => {
            stats.armor += value;
            true
        }
        "magic_resist" | "mr" => {
            stats.magic_resist += value;
            true
        }
        "attack_damage" => {
            stats.attack_damage += value;
            true
        }
        "ability_power" => {
            stats.ability_power += value;
            true
        }
        "ability_haste" => {
            stats.ability_haste += value;
            true
        }
        "attack_speed" => {
            if is_percent_unit {
                stats.attack_speed_percent += value;
            } else {
                stats.attack_speed_percent += value;
            }
            true
        }
        "movement_speed" => {
            if is_percent_unit {
                stats.move_speed_percent += value;
            } else {
                stats.move_speed_flat += value;
            }
            true
        }
        "adaptive" => {
            if for_vlad {
                stats.ability_power += value;
                true
            } else {
                false
            }
        }
        "cooldown" => {
            // Approximate CDR% to AH for deterministic use.
            let pct = if is_percent_unit {
                value / 100.0
            } else {
                value
            };
            let pct = pct.clamp(0.0, 0.95);
            let ah = 100.0 * pct / (1.0 - pct);
            stats.ability_haste += ah;
            true
        }
        _ => false,
    }
}

fn apply_structured_effect(
    effect: &Value,
    rank: usize,
    level: usize,
    for_vlad: bool,
    stats: &mut Stats,
) -> Result<bool> {
    let effect_type = effect
        .get("effect_type")
        .and_then(Value::as_str)
        .unwrap_or("");
    if effect_type != "stat_modifier" && effect_type != "cooldown" {
        return Ok(false);
    }
    let trigger = effect
        .get("trigger")
        .and_then(Value::as_str)
        .unwrap_or("passive");
    let unconditional_trigger = matches!(trigger, "" | "passive" | "on_equip" | "always");
    if !unconditional_trigger {
        return Ok(false);
    }
    let stat = effect.get("stat").and_then(Value::as_str).unwrap_or("");
    if stat.is_empty() {
        return Ok(false);
    }
    let Some(value) = value_from_effect(effect, rank, level) else {
        return Ok(false);
    };
    let unit = effect.get("unit").and_then(Value::as_str).unwrap_or("");
    let is_percent_unit = unit.contains("percent") || unit == "ratio";
    Ok(apply_stat_bonus(
        stats,
        stat,
        value,
        is_percent_unit,
        for_vlad,
    ))
}

fn resolve_loadout(
    selection: &LoadoutSelection,
    level: usize,
    for_vlad: bool,
) -> Result<ResolvedLoadout> {
    let runes_data = load_json(&masteries_dir().join("RunesReforged.json"))?;
    let masteries_data = load_json(&masteries_dir().join("Season2016.json"))?;

    let mut runes_by_id: HashMap<i64, Value> = HashMap::new();
    let mut runes_by_name: HashMap<String, Value> = HashMap::new();
    if let Some(paths) = runes_data.get("paths").and_then(Value::as_array) {
        for path in paths {
            if let Some(slots) = path.get("slots").and_then(Value::as_array) {
                for slot in slots {
                    if let Some(runes) = slot.get("runes").and_then(Value::as_array) {
                        for rune in runes {
                            if let Some(id) = rune.get("id").and_then(Value::as_i64) {
                                runes_by_id.insert(id, rune.clone());
                            }
                            if let Some(name) = rune.get("name").and_then(Value::as_str) {
                                runes_by_name.insert(to_norm_key(name), rune.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    let mastery_by_name = masteries_data
        .get("masteries")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    m.get("display_name")
                        .and_then(Value::as_str)
                        .map(|name| (to_norm_key(name), m.clone()))
                })
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default();

    let mut out = ResolvedLoadout::default();

    for id in &selection.rune_ids {
        if let Some(rune) = runes_by_id.get(id) {
            let name = rune
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("unknown rune");
            out.selection_labels.push(format!("Rune: {}", name));
            for effect in rune
                .get("effects_structured")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
            {
                if apply_structured_effect(&effect, 1, level, for_vlad, &mut out.bonus_stats)? {
                    out.applied_notes
                        .push(format!("Applied rune stat effect from {}.", name));
                }
            }
        } else {
            out.skipped_notes
                .push(format!("Rune id {} not found in RunesReforged.", id));
        }
    }

    for name in &selection.rune_names {
        let key = to_norm_key(name);
        if let Some(rune) = runes_by_name.get(&key) {
            let real_name = rune.get("name").and_then(Value::as_str).unwrap_or(name);
            out.selection_labels.push(format!("Rune: {}", real_name));
            for effect in rune
                .get("effects_structured")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
            {
                if apply_structured_effect(&effect, 1, level, for_vlad, &mut out.bonus_stats)? {
                    out.applied_notes
                        .push(format!("Applied rune stat effect from {}.", real_name));
                }
            }
        } else {
            out.skipped_notes
                .push(format!("Rune '{}' not found in RunesReforged.", name));
        }
    }

    if let Some(shards) = runes_data.get("stat_shards").and_then(Value::as_array) {
        for (idx, shard_key) in selection.shard_stats.iter().enumerate() {
            let Some(slot) = shards.get(idx) else {
                out.skipped_notes.push(format!(
                    "Shard '{}' ignored: slot {} does not exist.",
                    shard_key,
                    idx + 1
                ));
                continue;
            };
            let key = to_norm_key(shard_key);
            let mut applied = false;
            for option in slot
                .get("options")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default()
            {
                let stat = option.get("stat").and_then(Value::as_str).unwrap_or("");
                if to_norm_key(stat) != key {
                    continue;
                }
                let mut val = option
                    .get("numbers_extracted")
                    .and_then(Value::as_array)
                    .and_then(|a| a.first())
                    .and_then(Value::as_f64)
                    .unwrap_or(0.0);
                if stat == "health" {
                    // health shard scales with level using extracted [min, max]
                    if let Some(nums) = option.get("numbers_extracted").and_then(Value::as_array) {
                        if nums.len() >= 2 {
                            if let (Some(min), Some(max)) = (nums[0].as_f64(), nums[1].as_f64()) {
                                let t = ((level.max(1) as f64 - 1.0) / 29.0).clamp(0.0, 1.0);
                                val = min + (max - min) * t;
                            }
                        }
                    }
                }
                let is_percent = option
                    .get("unit_hint")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .contains("percent");
                if apply_stat_bonus(&mut out.bonus_stats, stat, val, is_percent, for_vlad) {
                    out.selection_labels
                        .push(format!("Shard {}: {}", idx + 1, shard_key));
                    out.applied_notes.push(format!(
                        "Applied shard '{}' in slot {}.",
                        shard_key,
                        idx + 1
                    ));
                    applied = true;
                    break;
                }
            }
            if !applied {
                out.skipped_notes.push(format!(
                    "Shard '{}' in slot {} not applicable in current stat model.",
                    shard_key,
                    idx + 1
                ));
            }
        }
    }

    for mastery in &selection.masteries {
        let key = to_norm_key(&mastery.name);
        let Some(m) = mastery_by_name.get(&key) else {
            out.skipped_notes.push(format!(
                "Mastery '{}' not found in Season2016.",
                mastery.name
            ));
            continue;
        };
        let max_ranks = m.get("ranks").and_then(Value::as_u64).unwrap_or(1) as usize;
        let rank = mastery.rank.clamp(1, max_ranks);
        let name = m
            .get("display_name")
            .and_then(Value::as_str)
            .unwrap_or(&mastery.name);
        out.selection_labels
            .push(format!("Mastery: {} ({}/{})", name, rank, max_ranks));
        for effect in m
            .get("effects_structured")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
        {
            if apply_structured_effect(&effect, rank, level, for_vlad, &mut out.bonus_stats)? {
                out.applied_notes
                    .push(format!("Applied mastery stat effect from {}.", name));
            }
        }
    }

    Ok(out)
}

fn as_f64(obj: &Value, key: &str) -> Result<f64> {
    obj.get(key)
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing f64 key: {}", key))
}

fn as_str<'a>(obj: &'a Value, key: &str) -> Result<&'a str> {
    obj.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Missing string key: {}", key))
}

fn parse_simulation_config(data: &Value) -> Result<SimulationConfig> {
    let server_tick_rate_hz = data
        .get("server_tick_rate_hz")
        .and_then(Value::as_f64)
        .unwrap_or(30.0);
    let dt = data.get("dt").and_then(Value::as_f64).unwrap_or_else(|| {
        if server_tick_rate_hz > 0.0 {
            1.0 / server_tick_rate_hz
        } else {
            0.05
        }
    });

    let base_damage = data
        .get("vlad_pool_base_damage_by_rank")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing vlad_pool_base_damage_by_rank"))?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| anyhow!("Invalid base damage")))
        .collect::<Result<Vec<_>>>()?;

    Ok(SimulationConfig {
        dt,
        server_tick_rate_hz,
        champion_level: data
            .get("champion_level")
            .and_then(Value::as_u64)
            .unwrap_or(20) as usize,
        max_time_seconds: as_f64(data, "max_time_seconds")?,
        vlad_pool_rank: data
            .get("vlad_pool_rank")
            .and_then(Value::as_u64)
            .ok_or_else(|| anyhow!("Missing vlad_pool_rank"))? as usize,
        vlad_pool_untargetable_seconds: as_f64(data, "vlad_pool_untargetable_seconds")?,
        vlad_pool_cost_percent_current_health: as_f64(
            data,
            "vlad_pool_cost_percent_current_health",
        )?,
        vlad_pool_heal_ratio_of_damage: as_f64(data, "vlad_pool_heal_ratio_of_damage")?,
        vlad_pool_base_damage_by_rank: base_damage,
        vlad_pool_bonus_health_ratio: as_f64(data, "vlad_pool_bonus_health_ratio")?,
        zhonya_duration_seconds: as_f64(data, "zhonya_duration_seconds")?,
        zhonya_cooldown_seconds: as_f64(data, "zhonya_cooldown_seconds")?,
        zhonya_trigger_health_percent: as_f64(data, "zhonya_trigger_health_percent")?,
        ga_cooldown_seconds: as_f64(data, "ga_cooldown_seconds")?,
        ga_revive_duration_seconds: as_f64(data, "ga_revive_duration_seconds")?,
        ga_revive_base_health_ratio: as_f64(data, "ga_revive_base_health_ratio")?,
        protoplasm_trigger_health_percent: as_f64(data, "protoplasm_trigger_health_percent")?,
        protoplasm_bonus_health: as_f64(data, "protoplasm_bonus_health")?,
        protoplasm_heal_total: as_f64(data, "protoplasm_heal_total")?,
        protoplasm_duration_seconds: as_f64(data, "protoplasm_duration_seconds")?,
        heartsteel_assumed_stacks_at_8m: data
            .get("heartsteel_assumed_stacks_at_8m")
            .and_then(Value::as_f64)
            .unwrap_or(20.0),
        enemy_uptime_model_enabled: data
            .get("enemy_uptime_model_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        urf_respawn_flat_reduction_seconds: data
            .get("urf_respawn_flat_reduction_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(3.0),
        urf_respawn_extrapolation_per_level: data
            .get("urf_respawn_extrapolation_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(2.5),
        vlad_q_base_damage: data
            .get("vlad_q_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(220.0),
        vlad_q_ap_ratio: data
            .get("vlad_q_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.60),
        vlad_q_heal_ratio_of_damage: data
            .get("vlad_q_heal_ratio_of_damage")
            .and_then(Value::as_f64)
            .unwrap_or(0.30),
        vlad_q_base_cooldown_seconds: data
            .get("vlad_q_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(4.0),
        vlad_e_base_damage: data
            .get("vlad_e_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(180.0),
        vlad_e_ap_ratio: data
            .get("vlad_e_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.50),
        vlad_e_base_cooldown_seconds: data
            .get("vlad_e_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(8.0),
        vlad_r_base_damage: data
            .get("vlad_r_base_damage")
            .and_then(Value::as_f64)
            .unwrap_or(350.0),
        vlad_r_ap_ratio: data
            .get("vlad_r_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.70),
        vlad_r_base_cooldown_seconds: data
            .get("vlad_r_base_cooldown_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(90.0),
    })
}

fn parse_champion_base(data: &Value) -> Result<ChampionBase> {
    Ok(ChampionBase {
        name: as_str(data, "name")?.to_string(),
        base_health: as_f64(data, "base_health")?,
        health_per_level: data
            .get("health_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_armor: as_f64(data, "base_armor")?,
        armor_per_level: data
            .get("armor_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_magic_resist: as_f64(data, "base_magic_resist")?,
        magic_resist_per_level: data
            .get("magic_resist_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_attack_damage: as_f64(data, "base_attack_damage")?,
        attack_damage_per_level: data
            .get("attack_damage_per_level")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_attack_speed: as_f64(data, "base_attack_speed")?,
        attack_speed_per_level_percent: data
            .get("attack_speed_per_level_percent")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        base_move_speed: as_f64(data, "base_move_speed")?,
        is_melee: data
            .get("is_melee")
            .and_then(Value::as_bool)
            .ok_or_else(|| anyhow!("Missing is_melee"))?,
    })
}

fn parse_enemy_config(
    data: &Value,
    champion_bases: &HashMap<String, ChampionBase>,
) -> Result<EnemyConfig> {
    let base = if let Some(champion) = data.get("champion").and_then(Value::as_str) {
        lookup_champion_base(champion_bases, champion)?
    } else if let Some(legacy) = data.get("base") {
        parse_champion_base(legacy)?
    } else {
        bail!("Enemy requires champion or base field");
    };
    Ok(EnemyConfig {
        name: data
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or(&base.name)
            .to_string(),
        base,
        ability_dps_flat: data
            .get("ability_dps_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_dps_ad_ratio: data
            .get("ability_dps_ad_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_dps_ap_ratio: data
            .get("ability_dps_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        ability_tick_interval_seconds: data
            .get("ability_tick_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        stun_interval_seconds: data
            .get("stun_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        stun_duration_seconds: data
            .get("stun_duration_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_interval_seconds: data
            .get("burst_interval_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_start_offset_seconds: data
            .get("burst_start_offset_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_magic_flat: data
            .get("burst_magic_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_physical_flat: data
            .get("burst_physical_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_true_flat: data
            .get("burst_true_flat")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_ad_ratio: data
            .get("burst_ad_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        burst_ap_ratio: data
            .get("burst_ap_ratio")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_cycle_seconds: data
            .get("uptime_cycle_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_active_seconds: data
            .get("uptime_active_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        uptime_phase_seconds: data
            .get("uptime_phase_seconds")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
    })
}

fn parse_build_search(data: &Value) -> Result<BuildSearchConfig> {
    let portfolio_strategies = data
        .get("portfolio_strategies")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(Value::as_str)
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(BuildSearchConfig {
        strategy: as_str(data, "strategy")?.to_string(),
        beam_width: data.get("beam_width").and_then(Value::as_u64).unwrap_or(20) as usize,
        max_items: data.get("max_items").and_then(Value::as_u64).unwrap_or(6) as usize,
        random_samples: data
            .get("random_samples")
            .and_then(Value::as_u64)
            .unwrap_or(200) as usize,
        hill_climb_restarts: data
            .get("hill_climb_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(64) as usize,
        hill_climb_steps: data
            .get("hill_climb_steps")
            .and_then(Value::as_u64)
            .unwrap_or(20) as usize,
        hill_climb_neighbors: data
            .get("hill_climb_neighbors")
            .and_then(Value::as_u64)
            .unwrap_or(24) as usize,
        genetic_population: data
            .get("genetic_population")
            .and_then(Value::as_u64)
            .unwrap_or(80) as usize,
        genetic_generations: data
            .get("genetic_generations")
            .and_then(Value::as_u64)
            .unwrap_or(30) as usize,
        genetic_mutation_rate: data
            .get("genetic_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.18),
        genetic_crossover_rate: data
            .get("genetic_crossover_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.90),
        portfolio_strategies,
        ranked_limit: data
            .get("ranked_limit")
            .and_then(Value::as_u64)
            .unwrap_or(400) as usize,
        simulated_annealing_restarts: data
            .get("simulated_annealing_restarts")
            .and_then(Value::as_u64)
            .unwrap_or(32) as usize,
        simulated_annealing_iterations: data
            .get("simulated_annealing_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(220) as usize,
        simulated_annealing_initial_temp: data
            .get("simulated_annealing_initial_temp")
            .and_then(Value::as_f64)
            .unwrap_or(1.2),
        simulated_annealing_cooling_rate: data
            .get("simulated_annealing_cooling_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.985),
        mcts_iterations: data
            .get("mcts_iterations")
            .and_then(Value::as_u64)
            .unwrap_or(5000) as usize,
        mcts_rollouts_per_expansion: data
            .get("mcts_rollouts_per_expansion")
            .and_then(Value::as_u64)
            .unwrap_or(2) as usize,
        mcts_exploration: data
            .get("mcts_exploration")
            .and_then(Value::as_f64)
            .unwrap_or(1.2),
        ensemble_seeds: data
            .get("ensemble_seeds")
            .and_then(Value::as_u64)
            .unwrap_or(3) as usize,
        ensemble_seed_stride: data
            .get("ensemble_seed_stride")
            .and_then(Value::as_u64)
            .unwrap_or(1_000_003),
        ensemble_seed_top_k: data
            .get("ensemble_seed_top_k")
            .and_then(Value::as_u64)
            .unwrap_or(25) as usize,
        objective_survival_weight: data
            .get("objective_survival_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.55),
        objective_damage_weight: data
            .get("objective_damage_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.30),
        objective_healing_weight: data
            .get("objective_healing_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.15),
        robust_min_seed_hit_rate: data
            .get("robust_min_seed_hit_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.5),
        bleed_enabled: data
            .get("bleed_enabled")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        bleed_budget: data
            .get("bleed_budget")
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize,
        bleed_mutation_rate: data
            .get("bleed_mutation_rate")
            .and_then(Value::as_f64)
            .unwrap_or(0.35),
        multi_scenario_worst_weight: data
            .get("multi_scenario_worst_weight")
            .and_then(Value::as_f64)
            .unwrap_or(0.35),
        seed: data.get("seed").and_then(Value::as_u64).unwrap_or(1337),
    })
}

fn apply_search_quality_profile(search: &mut BuildSearchConfig, profile: SearchQualityProfile) {
    match profile {
        SearchQualityProfile::Fast => {
            search.beam_width = 24;
            search.random_samples = 192;
            search.hill_climb_restarts = 24;
            search.hill_climb_steps = 12;
            search.hill_climb_neighbors = 12;
            search.genetic_population = 48;
            search.genetic_generations = 14;
            search.simulated_annealing_restarts = 12;
            search.simulated_annealing_iterations = 96;
            search.mcts_iterations = 1200;
            search.mcts_rollouts_per_expansion = 1;
            search.ensemble_seeds = 1;
            search.ensemble_seed_top_k = 10;
            search.ranked_limit = 200;
            search.bleed_budget = 120;
        }
        SearchQualityProfile::Balanced => {
            search.beam_width = 36;
            search.random_samples = 360;
            search.hill_climb_restarts = 48;
            search.hill_climb_steps = 18;
            search.hill_climb_neighbors = 20;
            search.genetic_population = 72;
            search.genetic_generations = 24;
            search.simulated_annealing_restarts = 24;
            search.simulated_annealing_iterations = 180;
            search.mcts_iterations = 2600;
            search.mcts_rollouts_per_expansion = 2;
            search.ensemble_seeds = 2;
            search.ensemble_seed_top_k = 18;
            search.ranked_limit = 320;
            search.bleed_budget = 300;
        }
        SearchQualityProfile::MaximumQuality => {
            search.beam_width = search.beam_width.max(64);
            search.random_samples = search.random_samples.max(900);
            search.hill_climb_restarts = search.hill_climb_restarts.max(128);
            search.hill_climb_steps = search.hill_climb_steps.max(28);
            search.hill_climb_neighbors = search.hill_climb_neighbors.max(30);
            search.genetic_population = search.genetic_population.max(140);
            search.genetic_generations = search.genetic_generations.max(52);
            search.simulated_annealing_restarts = search.simulated_annealing_restarts.max(56);
            search.simulated_annealing_iterations = search.simulated_annealing_iterations.max(340);
            search.mcts_iterations = search.mcts_iterations.max(9000);
            search.mcts_rollouts_per_expansion = search.mcts_rollouts_per_expansion.max(3);
            search.ensemble_seeds = search.ensemble_seeds.max(4);
            search.ensemble_seed_top_k = search.ensemble_seed_top_k.max(36);
            search.ranked_limit = search.ranked_limit.max(640);
            search.bleed_budget = search.bleed_budget.max(1200);
        }
    }
}

fn parse_loadout_selection(data: Option<&Value>) -> LoadoutSelection {
    let mut out = LoadoutSelection::default();
    let Some(obj) = data.and_then(Value::as_object) else {
        return out;
    };

    if let Some(runes_obj) = obj.get("runes_reforged").and_then(Value::as_object) {
        if let Some(ids) = runes_obj.get("rune_ids").and_then(Value::as_array) {
            out.rune_ids = ids.iter().filter_map(Value::as_i64).collect();
        }
        if let Some(names) = runes_obj.get("rune_names").and_then(Value::as_array) {
            out.rune_names = names
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect();
        }
        if let Some(shards) = runes_obj.get("shard_stats").and_then(Value::as_array) {
            out.shard_stats = shards
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect();
        }
    }

    if let Some(masteries) = obj.get("season2016_masteries").and_then(Value::as_array) {
        for entry in masteries {
            if let Some(name) = entry.as_str() {
                out.masteries.push(MasterySelection {
                    name: name.to_string(),
                    rank: 1,
                });
                continue;
            }
            let Some(mo) = entry.as_object() else {
                continue;
            };
            let Some(name) = mo.get("name").and_then(Value::as_str) else {
                continue;
            };
            let rank = mo.get("rank").and_then(Value::as_u64).unwrap_or(1) as usize;
            out.masteries.push(MasterySelection {
                name: name.to_string(),
                rank,
            });
        }
    }
    out
}

fn loadout_selection_key(sel: &LoadoutSelection) -> String {
    let mut runes = sel.rune_names.clone();
    runes.sort();
    let mut shards = sel.shard_stats.clone();
    shards.sort();
    let mut masteries = sel
        .masteries
        .iter()
        .map(|m| format!("{}:{}", m.name, m.rank))
        .collect::<Vec<_>>();
    masteries.sort();
    format!(
        "r={}|s={}|m={}",
        runes.join(","),
        shards.join(","),
        masteries.join(",")
    )
}

#[derive(Debug, Clone)]
struct RunePathDomain {
    slot_runes: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
struct MasteryOptionDomain {
    name: String,
    max_rank: usize,
    points_required_in_tree: usize,
}

#[derive(Debug, Clone)]
struct MasteryTierDomain {
    points_required: usize,
    points_available: usize,
    is_keystone_tier: bool,
    options: Vec<MasteryOptionDomain>,
}

#[derive(Debug, Clone)]
struct MasteryTreeDomain {
    tiers: Vec<MasteryTierDomain>,
}

#[derive(Debug, Clone)]
struct LoadoutDomain {
    rune_paths: Vec<RunePathDomain>,
    shard_slots: [Vec<String>; 3],
    mastery_trees: Vec<MasteryTreeDomain>,
    mastery_primary_points: usize,
    mastery_secondary_points: usize,
    mastery_keystone_requirement: usize,
}

fn build_loadout_domain() -> LoadoutDomain {
    let runes_data = load_json(&masteries_dir().join("RunesReforged.json")).unwrap_or(Value::Null);
    let masteries_data = load_json(&masteries_dir().join("Season2016.json")).unwrap_or(Value::Null);

    let rune_paths = runes_data
        .get("paths")
        .and_then(Value::as_array)
        .map(|paths| {
            paths
                .iter()
                .filter_map(|path| {
                    let slots = path.get("slots").and_then(Value::as_array)?;
                    let slot_runes = slots
                        .iter()
                        .map(|slot| {
                            slot.get("runes")
                                .and_then(Value::as_array)
                                .map(|runes| {
                                    runes
                                        .iter()
                                        .filter_map(|r| r.get("name").and_then(Value::as_str))
                                        .map(ToOwned::to_owned)
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default()
                        })
                        .collect::<Vec<_>>();
                    if slot_runes.len() >= 4 {
                        Some(RunePathDomain { slot_runes })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let shard_slots = {
        let slots = runes_data
            .get("stat_shards")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let read_slot = |idx: usize| -> Vec<String> {
            slots
                .get(idx)
                .and_then(|s| s.get("options"))
                .and_then(Value::as_array)
                .map(|options| {
                    options
                        .iter()
                        .filter_map(|o| o.get("stat").and_then(Value::as_str))
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        [read_slot(0), read_slot(1), read_slot(2)]
    };

    let mastery_lookup = masteries_data
        .get("masteries")
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    let name = m.get("display_name").and_then(Value::as_str)?;
                    let ranks = m.get("ranks").and_then(Value::as_u64).unwrap_or(1) as usize;
                    let points_required_in_tree = m
                        .get("points_required_in_tree")
                        .and_then(Value::as_u64)
                        .unwrap_or(0) as usize;
                    Some((
                        to_norm_key(name),
                        MasteryOptionDomain {
                            name: name.to_string(),
                            max_rank: ranks.max(1),
                            points_required_in_tree,
                        },
                    ))
                })
                .collect::<HashMap<_, _>>()
        })
        .unwrap_or_default();

    let mastery_trees = masteries_data
        .get("trees")
        .and_then(Value::as_array)
        .map(|trees| {
            trees
                .iter()
                .map(|tree| {
                    let tiers = tree
                        .get("tiers")
                        .and_then(Value::as_array)
                        .map(|tier_arr| {
                            tier_arr
                                .iter()
                                .map(|tier| {
                                    let options = tier
                                        .get("masteries")
                                        .and_then(Value::as_array)
                                        .map(|names| {
                                            names
                                                .iter()
                                                .filter_map(Value::as_str)
                                                .filter_map(|name| {
                                                    mastery_lookup.get(&to_norm_key(name)).cloned()
                                                })
                                                .collect::<Vec<_>>()
                                        })
                                        .unwrap_or_default();
                                    MasteryTierDomain {
                                        points_required: tier
                                            .get("points_in_tree_required")
                                            .and_then(Value::as_u64)
                                            .unwrap_or(0)
                                            as usize,
                                        points_available: tier
                                            .get("points_available")
                                            .and_then(Value::as_u64)
                                            .unwrap_or(5)
                                            as usize,
                                        is_keystone_tier: tier
                                            .get("is_keystone_tier")
                                            .and_then(Value::as_bool)
                                            .unwrap_or(false),
                                        options,
                                    }
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    MasteryTreeDomain { tiers }
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let rules = masteries_data
        .get("selection_rules")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    LoadoutDomain {
        rune_paths,
        shard_slots,
        mastery_trees,
        mastery_primary_points: rules
            .get("primary_tree_points")
            .and_then(Value::as_u64)
            .unwrap_or(18) as usize,
        mastery_secondary_points: rules
            .get("secondary_tree_points")
            .and_then(Value::as_u64)
            .unwrap_or(12) as usize,
        mastery_keystone_requirement: rules
            .get("keystone_requirement_points_in_tree")
            .and_then(Value::as_u64)
            .unwrap_or(17) as usize,
    }
}

fn random_tree_masteries(
    tree: &MasteryTreeDomain,
    target_points: usize,
    keystone_requirement: usize,
    seed: &mut u64,
) -> Option<Vec<MasterySelection>> {
    if tree.tiers.is_empty() || target_points == 0 {
        return Some(Vec::new());
    }
    for _ in 0..128 {
        let mut points = 0usize;
        let mut tier_spent = vec![0usize; tree.tiers.len()];
        let mut ranks = tree
            .tiers
            .iter()
            .map(|t| vec![0usize; t.options.len()])
            .collect::<Vec<_>>();

        while points < target_points {
            let mut choices = Vec::new();
            for (tier_idx, tier) in tree.tiers.iter().enumerate() {
                if tier.options.is_empty()
                    || points < tier.points_required
                    || tier_spent[tier_idx] >= tier.points_available
                {
                    continue;
                }
                if tier.is_keystone_tier && points < keystone_requirement {
                    continue;
                }
                for (opt_idx, opt) in tier.options.iter().enumerate() {
                    if ranks[tier_idx][opt_idx] >= opt.max_rank
                        || points < opt.points_required_in_tree
                    {
                        continue;
                    }
                    choices.push((tier_idx, opt_idx));
                }
            }
            if choices.is_empty() {
                break;
            }
            let (tier_idx, opt_idx) = choices[rand_index(seed, choices.len())];
            ranks[tier_idx][opt_idx] += 1;
            tier_spent[tier_idx] += 1;
            points += 1;
        }

        if points != target_points {
            continue;
        }
        let mut out = Vec::new();
        for (tier_idx, tier) in tree.tiers.iter().enumerate() {
            for (opt_idx, opt) in tier.options.iter().enumerate() {
                let rank = ranks[tier_idx][opt_idx];
                if rank > 0 {
                    out.push(MasterySelection {
                        name: opt.name.clone(),
                        rank,
                    });
                }
            }
        }
        return Some(out);
    }
    None
}

fn random_loadout_selection(
    base: &LoadoutSelection,
    domain: &LoadoutDomain,
    seed: &mut u64,
) -> LoadoutSelection {
    let mut out = base.clone();

    if domain.rune_paths.len() >= 2
        && domain.shard_slots.iter().all(|s| !s.is_empty())
        && domain.rune_paths.iter().all(|p| p.slot_runes.len() >= 4)
    {
        let primary_idx = rand_index(seed, domain.rune_paths.len());
        let secondary_choices = (0..domain.rune_paths.len())
            .filter(|idx| *idx != primary_idx)
            .collect::<Vec<_>>();
        if !secondary_choices.is_empty() {
            let secondary_idx = secondary_choices[rand_index(seed, secondary_choices.len())];
            let primary = &domain.rune_paths[primary_idx];
            let secondary = &domain.rune_paths[secondary_idx];
            if primary.slot_runes[..4].iter().all(|slot| !slot.is_empty()) {
                let secondary_slots = (1..=3)
                    .filter(|slot| {
                        secondary
                            .slot_runes
                            .get(*slot)
                            .map(|r| !r.is_empty())
                            .unwrap_or(false)
                    })
                    .collect::<Vec<_>>();
                if secondary_slots.len() >= 2 {
                    let mut picks = secondary_slots.clone();
                    shuffle_usize(&mut picks, seed);
                    let sa = picks[0];
                    let sb = picks[1];
                    out.rune_names = vec![
                        primary.slot_runes[0][rand_index(seed, primary.slot_runes[0].len())]
                            .clone(),
                        primary.slot_runes[1][rand_index(seed, primary.slot_runes[1].len())]
                            .clone(),
                        primary.slot_runes[2][rand_index(seed, primary.slot_runes[2].len())]
                            .clone(),
                        primary.slot_runes[3][rand_index(seed, primary.slot_runes[3].len())]
                            .clone(),
                        secondary.slot_runes[sa][rand_index(seed, secondary.slot_runes[sa].len())]
                            .clone(),
                        secondary.slot_runes[sb][rand_index(seed, secondary.slot_runes[sb].len())]
                            .clone(),
                    ];
                    out.rune_ids.clear();
                    out.shard_stats = domain
                        .shard_slots
                        .iter()
                        .map(|slot| slot[rand_index(seed, slot.len())].clone())
                        .collect::<Vec<_>>();
                }
            }
        }
    }

    if domain.mastery_trees.len() >= 2 {
        let primary_idx = rand_index(seed, domain.mastery_trees.len());
        let secondary_choices = (0..domain.mastery_trees.len())
            .filter(|idx| *idx != primary_idx)
            .collect::<Vec<_>>();
        if !secondary_choices.is_empty() {
            let secondary_idx = secondary_choices[rand_index(seed, secondary_choices.len())];
            let primary = random_tree_masteries(
                &domain.mastery_trees[primary_idx],
                domain.mastery_primary_points,
                domain.mastery_keystone_requirement,
                seed,
            );
            let secondary = random_tree_masteries(
                &domain.mastery_trees[secondary_idx],
                domain.mastery_secondary_points,
                domain.mastery_keystone_requirement,
                seed,
            );
            if let (Some(mut p), Some(s)) = (primary, secondary) {
                p.extend(s);
                out.masteries = p;
            }
        }
    }

    out
}

fn loadout_eval_budget(search: &BuildSearchConfig, profile: SearchQualityProfile) -> usize {
    let base = search
        .random_samples
        .max(search.beam_width * 8)
        .max(search.hill_climb_restarts * 4)
        .max(search.genetic_population * 2)
        .max(128);
    match profile {
        SearchQualityProfile::Fast => base.min(256),
        SearchQualityProfile::Balanced => base.min(1024),
        SearchQualityProfile::MaximumQuality => base.min(4096),
    }
}

fn normalize_name(input: &str) -> String {
    input
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_lowercase())
        .collect()
}

fn champion_base_from_character_data(
    character: &Value,
    fallback_name: &str,
) -> Result<ChampionBase> {
    let base_stats = character
        .get("base_stats")
        .ok_or_else(|| anyhow!("Missing base_stats for {}", fallback_name))?;

    let attack_speed = base_stats
        .get("attack_speed")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_speed.base for {}", fallback_name))?;
    let armor = base_stats
        .get("armor")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing armor.base for {}", fallback_name))?;
    let magic_resist = base_stats
        .get("magic_resist")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing magic_resist.base for {}", fallback_name))?;
    let attack_damage = base_stats
        .get("attack_damage")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing attack_damage.base for {}", fallback_name))?;
    let health = base_stats
        .get("health")
        .and_then(|v| v.get("base"))
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing health.base for {}", fallback_name))?;
    let move_speed = base_stats
        .get("move_speed")
        .and_then(Value::as_f64)
        .ok_or_else(|| anyhow!("Missing move_speed for {}", fallback_name))?;
    let attack_speed_per_level_percent = base_stats
        .get("attack_speed")
        .and_then(|v| v.get("per_level_percent"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let armor_per_level = base_stats
        .get("armor")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let magic_resist_per_level = base_stats
        .get("magic_resist")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let attack_damage_per_level = base_stats
        .get("attack_damage")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    let health_per_level = base_stats
        .get("health")
        .and_then(|v| v.get("per_level"))
        .and_then(Value::as_f64)
        .unwrap_or(0.0);

    let attack_type = character
        .get("basic_attack")
        .and_then(|v| v.get("attack_type"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_lowercase();
    let is_melee = attack_type == "melee";

    let champion_name = character
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or(fallback_name)
        .to_string();

    Ok(ChampionBase {
        name: champion_name,
        base_health: health,
        health_per_level,
        base_armor: armor,
        armor_per_level,
        base_magic_resist: magic_resist,
        magic_resist_per_level,
        base_attack_damage: attack_damage,
        attack_damage_per_level,
        base_attack_speed: attack_speed,
        attack_speed_per_level_percent,
        base_move_speed: move_speed,
        is_melee,
    })
}

fn load_champion_bases() -> Result<HashMap<String, ChampionBase>> {
    let mut out = HashMap::new();
    let mut entries = fs::read_dir(characters_dir())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    entries.sort();

    for path in entries {
        let data = load_json(&path)?;
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Invalid character filename"))?;
        let base = champion_base_from_character_data(&data, stem)?;
        out.insert(normalize_name(stem), base.clone());
        out.insert(normalize_name(&base.name), base);
    }
    Ok(out)
}

fn lookup_champion_base(
    champion_bases: &HashMap<String, ChampionBase>,
    champion_name: &str,
) -> Result<ChampionBase> {
    champion_bases
        .get(&normalize_name(champion_name))
        .cloned()
        .ok_or_else(|| anyhow!("Champion not found: {}", champion_name))
}

fn load_urf_buffs() -> Result<UrfBuffs> {
    let path = game_mode_dir().join("URF.json");
    let data = load_json(&path)?;
    let buffs = data.get("global_buffs").cloned().unwrap_or(Value::Null);

    Ok(UrfBuffs {
        ability_haste: buffs
            .get("ability_haste")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        item_haste: buffs
            .get("item_haste")
            .and_then(Value::as_f64)
            .unwrap_or(0.0),
        health_cost_multiplier: buffs
            .get("resource_costs")
            .and_then(|v| v.get("health_cost_multiplier"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        bonus_attack_speed_multiplier_melee: buffs
            .get("bonus_attack_speed_multiplier")
            .and_then(|v| v.get("melee"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
        bonus_attack_speed_multiplier_ranged: buffs
            .get("bonus_attack_speed_multiplier")
            .and_then(|v| v.get("ranged"))
            .and_then(Value::as_f64)
            .unwrap_or(1.0),
    })
}

fn stat_key_map(key: &str) -> Option<&'static str> {
    match key {
        "abilityPower" => Some("ability_power"),
        "health" => Some("health"),
        "armor" => Some("armor"),
        "magicResist" => Some("magic_resist"),
        "attackDamage" => Some("attack_damage"),
        "attackSpeed" => Some("attack_speed_percent"),
        "movespeed" | "moveSpeed" | "movementSpeed" => Some("move_speed_flat"),
        "abilityHaste" => Some("ability_haste"),
        "critChance" => Some("crit_chance_percent"),
        _ => None,
    }
}

fn load_items() -> Result<HashMap<String, Item>> {
    let mut items = HashMap::new();
    let mut entries = fs::read_dir(items_dir())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    entries.sort();

    for path in entries {
        let data = load_json(&path)?;
        let rank = data
            .get("rank")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let purchasable = data
            .get("shop")
            .and_then(|v| v.get("purchasable"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let total_cost = data
            .get("shop")
            .and_then(|v| v.get("prices"))
            .and_then(|v| v.get("total"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0);
        if rank.iter().any(|r| EXCLUDED_RANKS.contains(&r.as_str())) {
            continue;
        }

        let mut stats = Stats::default();
        if let Some(stats_obj) = data.get("stats").and_then(Value::as_object) {
            for (raw_key, raw_values) in stats_obj {
                let Some(stat_key) = stat_key_map(raw_key) else {
                    continue;
                };

                if let Some(flat) = raw_values.get("flat").and_then(Value::as_f64) {
                    add_stat_value(&mut stats, stat_key, flat);
                }
                if let Some(percent) = raw_values.get("percent").and_then(Value::as_f64) {
                    if stat_key == "move_speed_flat" {
                        stats.move_speed_flat += percent;
                    } else {
                        add_stat_value(&mut stats, stat_key, percent);
                    }
                }
            }
        }

        let passive_effects_text = data
            .get("passives")
            .and_then(Value::as_array)
            .map(|arr| {
                arr.iter()
                    .filter_map(|p| p.get("effects").and_then(Value::as_str))
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let Some(name) = data
            .get("name")
            .and_then(Value::as_str)
            .map(|s| s.to_string())
        else {
            continue;
        };
        items.insert(
            name.clone(),
            Item {
                name,
                stats,
                rank,
                shop_purchasable: purchasable,
                total_cost,
                passive_effects_text,
            },
        );
    }

    Ok(items)
}

fn add_stat_value(stats: &mut Stats, stat_key: &str, value: f64) {
    match stat_key {
        "ability_power" => stats.ability_power += value,
        "health" => stats.health += value,
        "armor" => stats.armor += value,
        "magic_resist" => stats.magic_resist += value,
        "attack_damage" => stats.attack_damage += value,
        "attack_speed_percent" => stats.attack_speed_percent += value,
        "ability_haste" => stats.ability_haste += value,
        "move_speed_flat" => stats.move_speed_flat += value,
        "crit_chance_percent" => stats.crit_chance_percent += value,
        _ => {}
    }
}

fn is_boots(item: &Item) -> bool {
    item.rank.iter().any(|r| r == "BOOTS")
}

fn cooldown_after_haste(base_seconds: f64, haste: f64) -> f64 {
    base_seconds * (100.0 / (100.0 + haste))
}

fn champion_at_level(base: &ChampionBase, level: usize) -> ChampionBase {
    let lvl = level.max(1) as f64;
    let growth_levels = (lvl - 1.0).max(0.0);
    ChampionBase {
        name: base.name.clone(),
        base_health: base.base_health + base.health_per_level * growth_levels,
        health_per_level: base.health_per_level,
        base_armor: base.base_armor + base.armor_per_level * growth_levels,
        armor_per_level: base.armor_per_level,
        base_magic_resist: base.base_magic_resist + base.magic_resist_per_level * growth_levels,
        magic_resist_per_level: base.magic_resist_per_level,
        base_attack_damage: base.base_attack_damage + base.attack_damage_per_level * growth_levels,
        attack_damage_per_level: base.attack_damage_per_level,
        base_attack_speed: base.base_attack_speed
            * (1.0 + (base.attack_speed_per_level_percent / 100.0) * growth_levels),
        attack_speed_per_level_percent: base.attack_speed_per_level_percent,
        base_move_speed: base.base_move_speed,
        is_melee: base.is_melee,
    }
}

fn assumed_heartsteel_bonus_health(base_max_health: f64, stacks_at_8m: f64) -> f64 {
    if stacks_at_8m <= 0.0 {
        return 0.0;
    }
    // Approximate permanent health gained by repeatedly proccing Heartsteel:
    // per proc ~= 8% * (70 + 6% max_health) = 5.6 + 0.0048 * max_health.
    // Use an iterative approximation because max_health grows as stacks are gained.
    let procs = stacks_at_8m.max(0.0).round() as usize;
    let mut max_health = base_max_health;
    let mut gained = 0.0;
    for _ in 0..procs {
        let delta = 5.6 + 0.0048 * max_health;
        gained += delta;
        max_health += delta;
    }
    gained
}

fn assumed_heartsteel_stacks_by_level(
    full_stacks_at_level_20: f64,
    acquired_level: usize,
    current_level: usize,
) -> f64 {
    let ref_start: f64 = 5.0;
    let ref_end: f64 = 20.0;
    let acquired = acquired_level as f64;
    let current = current_level as f64;
    let elapsed = (current - acquired).max(0.0);
    let reference_window = (ref_end - ref_start).max(1.0_f64);
    (full_stacks_at_level_20 * (elapsed / reference_window)).clamp(0.0, full_stacks_at_level_20)
}

fn get_item_acquired_level(
    build_items: &[Item],
    item_name: &str,
    acquired_levels: Option<&HashMap<String, usize>>,
    default_level: usize,
) -> usize {
    if build_items.iter().any(|i| i.name == item_name) {
        if let Some(map) = acquired_levels {
            if let Some(level) = map.get(item_name) {
                return *level;
            }
        }
        return default_level;
    }
    default_level
}

fn apply_item_assumptions(
    stats: &mut Stats,
    base: &ChampionBase,
    build_items: &[Item],
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
) {
    if build_items.iter().any(|i| i.name == "Heartsteel") {
        let acquired_level = get_item_acquired_level(build_items, "Heartsteel", acquired_levels, 5);
        let stacks = assumed_heartsteel_stacks_by_level(
            sim.heartsteel_assumed_stacks_at_8m,
            acquired_level,
            current_level,
        );
        let base_max_health = base.base_health + stats.health;
        stats.health += assumed_heartsteel_bonus_health(base_max_health, stacks);
    }
}

fn compute_effective_item_stats_for_build(
    base: &ChampionBase,
    build_items: &[Item],
    bonus_stats: &Stats,
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
) -> Stats {
    let mut stats = build_item_stats(build_items);
    stats.add(bonus_stats);
    apply_item_assumptions(
        &mut stats,
        base,
        build_items,
        sim,
        current_level,
        acquired_levels,
    );
    stats
}

fn build_stack_notes(
    build_items: &[Item],
    base: &ChampionBase,
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
) -> Vec<String> {
    let mut notes = Vec::new();
    for item in build_items {
        if item.name == "Heartsteel" {
            let mut base_stats = build_item_stats(build_items);
            // Remove Heartsteel's own flat health so the estimate is anchored to pre-heartsteel max HP.
            base_stats.health -= item.stats.health;
            let base_max_hp = base.base_health + base_stats.health.max(0.0);
            let acquired_level =
                get_item_acquired_level(build_items, "Heartsteel", acquired_levels, 5);
            let stacks = assumed_heartsteel_stacks_by_level(
                sim.heartsteel_assumed_stacks_at_8m,
                acquired_level,
                current_level,
            );
            let bonus = assumed_heartsteel_bonus_health(base_max_hp, stacks);
            notes.push(format!(
                "Heartsteel estimated stacks by level {}: {:.1} (acquired at level {}, reference full-at-20 stack target {:.0}, estimated permanent bonus health: +{:.1}).",
                current_level, stacks, acquired_level, sim.heartsteel_assumed_stacks_at_8m, bonus
            ));
            continue;
        }

        let has_stack_text = item
            .passive_effects_text
            .iter()
            .any(|t| t.to_ascii_lowercase().contains("stack"));
        if has_stack_text {
            notes.push(format!(
                "{} has stack-based passive text in item data; currently treated as baseline/implicit unless explicitly modeled.",
                item.name
            ));
        }
    }
    notes
}

fn compute_vlad_stats(base: &ChampionBase, item_stats: &Stats) -> Stats {
    let ap_items = item_stats.ability_power;
    let bonus_health_items = item_stats.health;
    // Crimson Pact should not self-recursively amplify:
    // - AP gained from bonus health does not grant extra health again
    // - Health gained from AP does not grant extra AP again
    let bonus_health = bonus_health_items + 1.6 * ap_items;
    let ability_power = ap_items + 0.033 * bonus_health_items;

    let mut stats = Stats {
        ability_power,
        health: bonus_health,
        armor: item_stats.armor,
        magic_resist: item_stats.magic_resist,
        attack_damage: item_stats.attack_damage,
        attack_speed_percent: item_stats.attack_speed_percent,
        ability_haste: item_stats.ability_haste,
        move_speed_flat: item_stats.move_speed_flat,
        move_speed_percent: item_stats.move_speed_percent,
        crit_chance_percent: item_stats.crit_chance_percent,
    };
    stats.health += base.base_health;
    stats.armor += base.base_armor;
    stats.magic_resist += base.base_magic_resist;
    stats
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

fn simulate_vlad_combat(
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

fn simulate_vlad_survival(
    vlad_base: &ChampionBase,
    vlad_build_items: &[Item],
    vlad_bonus_stats: &Stats,
    vlad_item_acquired_levels: Option<&HashMap<String, usize>>,
    enemies: &[(EnemyConfig, Vec<Item>, Stats)],
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> f64 {
    simulate_vlad_combat(
        vlad_base,
        vlad_build_items,
        vlad_bonus_stats,
        vlad_item_acquired_levels,
        enemies,
        sim,
        urf,
    )
    .time_alive_seconds
}

fn normalized_objective_weights(
    survival: f64,
    damage: f64,
    healing: f64,
) -> ObjectiveComponentWeights {
    let mut s = survival.max(0.0);
    let mut d = damage.max(0.0);
    let mut h = healing.max(0.0);
    let sum = s + d + h;
    if sum <= 0.0 {
        s = 1.0;
        d = 0.0;
        h = 0.0;
    } else {
        s /= sum;
        d /= sum;
        h /= sum;
    }
    ObjectiveComponentWeights {
        survival: s,
        damage: d,
        healing: h,
    }
}

fn objective_score_from_outcome(
    outcome: CombatOutcome,
    reference: CombatOutcome,
    weights: ObjectiveComponentWeights,
) -> f64 {
    let survival_ref = reference.time_alive_seconds.max(0.01);
    let damage_ref = reference.damage_dealt.max(1.0);
    let healing_ref = reference.healing_done.max(1.0);
    weights.survival * (outcome.time_alive_seconds / survival_ref)
        + weights.damage * (outcome.damage_dealt / damage_ref)
        + weights.healing * (outcome.healing_done / healing_ref)
}

fn aggregate_objective_score_and_outcome(
    vlad_base: &ChampionBase,
    build_items: &[Item],
    bonus_stats: &Stats,
    enemy_build_scenarios: &[(String, f64, Vec<(EnemyConfig, Vec<Item>, Stats)>)],
    sim: &SimulationConfig,
    urf: &UrfBuffs,
    scenario_reference_outcomes: &[CombatOutcome],
    weights: ObjectiveComponentWeights,
    worst_case_weight: f64,
) -> (f64, CombatOutcome) {
    let mut weighted_score_sum = 0.0;
    let mut weighted_time_sum = 0.0;
    let mut weighted_damage_sum = 0.0;
    let mut weighted_healing_sum = 0.0;
    let mut weighted_kills_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut worst = f64::INFINITY;

    for (idx, (_, weight, enemy_builds_s)) in enemy_build_scenarios.iter().enumerate() {
        let w = (*weight).max(0.0);
        if w <= 0.0 {
            continue;
        }
        let outcome = simulate_vlad_combat(
            vlad_base,
            build_items,
            bonus_stats,
            None,
            enemy_builds_s,
            sim,
            urf,
        );
        let reference = scenario_reference_outcomes
            .get(idx)
            .copied()
            .unwrap_or(CombatOutcome {
                time_alive_seconds: sim.max_time_seconds.max(1.0),
                damage_dealt: 1.0,
                healing_done: 1.0,
                enemy_kills: 0,
            });
        let scenario_score = objective_score_from_outcome(outcome, reference, weights);
        weighted_score_sum += w * scenario_score;
        weighted_time_sum += w * outcome.time_alive_seconds;
        weighted_damage_sum += w * outcome.damage_dealt;
        weighted_healing_sum += w * outcome.healing_done;
        weighted_kills_sum += w * outcome.enemy_kills as f64;
        weight_sum += w;
        worst = worst.min(scenario_score);
    }

    if weight_sum <= 0.0 {
        return (0.0, CombatOutcome::default());
    }

    let mean_score = weighted_score_sum / weight_sum;
    let blended_score = if worst.is_finite() {
        let ww = worst_case_weight.clamp(0.0, 1.0);
        (1.0 - ww) * mean_score + ww * worst
    } else {
        mean_score
    };
    let mean_outcome = CombatOutcome {
        time_alive_seconds: weighted_time_sum / weight_sum,
        damage_dealt: weighted_damage_sum / weight_sum,
        healing_done: weighted_healing_sum / weight_sum,
        enemy_kills: (weighted_kills_sum / weight_sum).round() as usize,
    };
    (blended_score, mean_outcome)
}

fn build_item_stats(items: &[Item]) -> Stats {
    let mut stats = Stats::default();
    for item in items {
        stats.add(&item.stats);
    }
    stats
}

fn build_from_indices(item_pool: &[Item], build: &[usize]) -> Vec<Item> {
    build.iter().map(|&idx| item_pool[idx].clone()).collect()
}

fn canonical_key(build: &[usize]) -> Vec<usize> {
    let mut key = build.to_vec();
    key.sort_unstable();
    key
}

fn next_u64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}

fn rand_index(seed: &mut u64, upper: usize) -> usize {
    if upper <= 1 {
        return 0;
    }
    (next_u64(seed) as usize) % upper
}

fn rand_f64(seed: &mut u64) -> f64 {
    let bits = next_u64(seed) >> 11;
    (bits as f64) / ((1u64 << 53) as f64)
}

fn shuffle_usize(slice: &mut [usize], seed: &mut u64) {
    if slice.len() <= 1 {
        return;
    }
    for i in (1..slice.len()).rev() {
        let j = rand_index(seed, i + 1);
        slice.swap(i, j);
    }
}

fn can_add_item_to_build(item_pool: &[Item], build: &[usize], item_idx: usize) -> bool {
    if build.contains(&item_idx) {
        return false;
    }
    if is_boots(&item_pool[item_idx]) && build.iter().any(|&i| is_boots(&item_pool[i])) {
        return false;
    }
    true
}

fn random_valid_build(item_pool: &[Item], max_items: usize, seed: &mut u64) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..item_pool.len()).collect();
    shuffle_usize(&mut indices, seed);
    let mut build = Vec::with_capacity(max_items);
    for item_idx in indices {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &build, item_idx) {
            build.push(item_idx);
        }
    }
    build
}

fn repair_build(item_pool: &[Item], build: &mut Vec<usize>, max_items: usize, seed: &mut u64) {
    let mut deduped = Vec::with_capacity(max_items);
    for &item_idx in build.iter() {
        if deduped.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &deduped, item_idx) {
            deduped.push(item_idx);
        }
    }
    *build = deduped;

    if build.len() >= max_items {
        return;
    }
    let mut all_indices: Vec<usize> = (0..item_pool.len()).collect();
    shuffle_usize(&mut all_indices, seed);
    for item_idx in all_indices {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, build, item_idx) {
            build.push(item_idx);
        }
    }
}

fn unique_ranked_from_candidates<F>(
    candidates: Vec<Vec<usize>>,
    score_fn: &F,
    limit: usize,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let scored = score_candidates(candidates, score_fn, deadline);
    let mut ranked = Vec::new();
    let mut seen = HashSet::new();
    for (_, key, score) in scored {
        if !score.is_finite() {
            continue;
        }
        if seen.insert(key.clone()) {
            ranked.push((key, score));
            if ranked.len() >= limit.max(1) {
                break;
            }
        }
    }
    ranked
}

fn score_candidates<F>(
    candidates: Vec<Vec<usize>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    if candidates.is_empty() || deadline_reached(deadline) {
        return Vec::new();
    }
    let unique_keys: HashSet<Vec<usize>> = candidates.iter().map(|c| canonical_key(c)).collect();
    let mut key_list = unique_keys.into_iter().collect::<Vec<_>>();
    key_list.sort_unstable();

    let score_pairs = key_list
        .par_iter()
        .map(|key| {
            if deadline_reached(deadline) {
                (key.clone(), f64::NEG_INFINITY)
            } else {
                (key.clone(), score_fn(key))
            }
        })
        .collect::<Vec<_>>();
    let score_map = score_pairs
        .into_iter()
        .collect::<HashMap<Vec<usize>, f64>>();

    let mut scored = candidates
        .into_iter()
        .map(|candidate| {
            let key = canonical_key(&candidate);
            let score = score_map.get(&key).copied().unwrap_or(f64::NEG_INFINITY);
            (candidate, key, score)
        })
        .collect::<Vec<_>>();

    scored.sort_by(|a, b| {
        b.2.partial_cmp(&a.2)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.1.cmp(&b.1))
            .then_with(|| a.0.cmp(&b.0))
    });
    scored
}

fn beam_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    beam_width: usize,
    score_fn: F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    let mut final_scored: Vec<(Vec<usize>, Vec<usize>, f64)> = vec![];

    for _ in 0..max_items {
        if deadline_reached(deadline) {
            break;
        }
        let mut next_candidates = Vec::new();
        for build in &candidates {
            let has_boots = build.iter().any(|&i| is_boots(&item_pool[i]));
            let used = build.iter().copied().collect::<HashSet<_>>();
            for item_idx in 0..item_pool.len() {
                if used.contains(&item_idx) {
                    continue;
                }
                if is_boots(&item_pool[item_idx]) && has_boots {
                    continue;
                }
                let mut next = build.clone();
                next.push(item_idx);
                next_candidates.push(next);
            }
        }

        let scored = score_candidates(next_candidates, &score_fn, deadline);
        candidates = scored
            .iter()
            .take(beam_width)
            .map(|(candidate, _, _)| candidate.clone())
            .collect();
        final_scored = scored;
    }

    let mut ranked = Vec::new();
    let mut seen = HashSet::new();
    for (_, key, score) in final_scored {
        if seen.insert(key.clone()) {
            ranked.push((key, score));
        }
    }
    ranked
}

fn random_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    random_samples: usize,
    seed: u64,
    limit: usize,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = seed;
    let mut candidates = Vec::with_capacity(random_samples.max(1));
    for _ in 0..random_samples.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        candidates.push(random_valid_build(item_pool, max_items, &mut s));
    }
    unique_ranked_from_candidates(candidates, score_fn, limit, deadline)
}

fn hill_climb_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    restarts: usize,
    steps: usize,
    neighbors_per_step: usize,
    seed: u64,
    limit: usize,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = seed;
    let mut candidates = Vec::new();

    for _ in 0..restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_valid_build(item_pool, max_items, &mut s);
        let mut current_score = score_fn(&canonical_key(&current));
        candidates.push(current.clone());

        for _ in 0..steps {
            if deadline_reached(deadline) {
                break;
            }
            let mut neighbor_builds = Vec::new();
            for _ in 0..neighbors_per_step.max(1) {
                if current.is_empty() {
                    break;
                }
                let mut neighbor = current.clone();
                let swap_idx = rand_index(&mut s, neighbor.len());
                let mut proposed = rand_index(&mut s, item_pool.len());
                let mut tries = 0usize;
                while tries < item_pool.len()
                    && (!can_add_item_to_build(item_pool, &neighbor, proposed)
                        || proposed == neighbor[swap_idx])
                {
                    proposed = rand_index(&mut s, item_pool.len());
                    tries += 1;
                }
                if tries < item_pool.len() {
                    neighbor[swap_idx] = proposed;
                    repair_build(item_pool, &mut neighbor, max_items, &mut s);
                    neighbor_builds.push(neighbor);
                }
            }
            if neighbor_builds.is_empty() {
                break;
            }
            let ranked_neighbors = unique_ranked_from_candidates(
                neighbor_builds,
                score_fn,
                neighbors_per_step.max(1),
                deadline,
            );
            let Some((best_neighbor, best_score)) = ranked_neighbors.first().cloned() else {
                break;
            };
            if best_score > current_score {
                current = best_neighbor;
                current_score = best_score;
                candidates.push(current.clone());
            } else {
                break;
            }
        }
    }

    unique_ranked_from_candidates(candidates, score_fn, limit, deadline)
}

fn tournament_parent(
    scored_population: &[(Vec<usize>, f64)],
    seed: &mut u64,
    tournament_size: usize,
) -> Vec<usize> {
    let mut best_idx = rand_index(seed, scored_population.len());
    for _ in 1..tournament_size.max(1) {
        let idx = rand_index(seed, scored_population.len());
        if scored_population[idx].1 > scored_population[best_idx].1 {
            best_idx = idx;
        }
    }
    scored_population[best_idx].0.clone()
}

fn crossover_builds(
    parent_a: &[usize],
    parent_b: &[usize],
    item_pool: &[Item],
    max_items: usize,
    seed: &mut u64,
) -> Vec<usize> {
    let mut merged = parent_a.to_vec();
    for &idx in parent_b {
        if !merged.contains(&idx) {
            merged.push(idx);
        }
    }
    shuffle_usize(&mut merged, seed);
    let mut child = Vec::with_capacity(max_items);
    for idx in merged {
        if child.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &child, idx) {
            child.push(idx);
        }
    }
    repair_build(item_pool, &mut child, max_items, seed);
    child
}

fn mutate_build(
    build: &mut Vec<usize>,
    item_pool: &[Item],
    max_items: usize,
    mutation_rate: f64,
    seed: &mut u64,
) {
    if build.is_empty() || rand_f64(seed) > mutation_rate.clamp(0.0, 1.0) {
        return;
    }
    let slot = rand_index(seed, build.len());
    let mut tries = 0usize;
    while tries < item_pool.len() {
        let candidate = rand_index(seed, item_pool.len());
        if candidate != build[slot] {
            let old = build[slot];
            build[slot] = candidate;
            if can_add_item_to_build(item_pool, &build[..slot], build[slot])
                && !build[(slot + 1)..].contains(&build[slot])
            {
                repair_build(item_pool, build, max_items, seed);
                return;
            }
            build[slot] = old;
        }
        tries += 1;
    }
    repair_build(item_pool, build, max_items, seed);
}

fn genetic_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    population_size: usize,
    generations: usize,
    mutation_rate: f64,
    crossover_rate: f64,
    seed: u64,
    limit: usize,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let pop_n = population_size.max(8);
    let mut s = seed;
    let mut population = Vec::with_capacity(pop_n);
    for _ in 0..pop_n {
        if deadline_reached(deadline) {
            break;
        }
        population.push(random_valid_build(item_pool, max_items, &mut s));
    }

    let mut all_seen = population.clone();
    for _ in 0..generations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut scored =
            unique_ranked_from_candidates(population.clone(), score_fn, pop_n, deadline);
        if scored.is_empty() {
            break;
        }
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        let elite_count = (pop_n / 8).max(1).min(scored.len());
        let mut next_population = scored
            .iter()
            .take(elite_count)
            .map(|(b, _)| b.clone())
            .collect::<Vec<_>>();

        while next_population.len() < pop_n {
            if deadline_reached(deadline) {
                break;
            }
            let parent_a = tournament_parent(&scored, &mut s, 3);
            let parent_b = tournament_parent(&scored, &mut s, 3);
            let mut child = if rand_f64(&mut s) <= crossover_rate.clamp(0.0, 1.0) {
                crossover_builds(&parent_a, &parent_b, item_pool, max_items, &mut s)
            } else {
                parent_a
            };
            mutate_build(&mut child, item_pool, max_items, mutation_rate, &mut s);
            repair_build(item_pool, &mut child, max_items, &mut s);
            next_population.push(child);
        }

        all_seen.extend(next_population.clone());
        population = next_population;
    }

    unique_ranked_from_candidates(all_seen, score_fn, limit, deadline)
}

fn simulated_annealing_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    restarts: usize,
    iterations: usize,
    initial_temp: f64,
    cooling_rate: f64,
    seed: u64,
    limit: usize,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = seed;
    let mut candidates = Vec::new();

    for _ in 0..restarts.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut current = random_valid_build(item_pool, max_items, &mut s);
        let mut current_score = score_fn(&canonical_key(&current));
        let mut best = current.clone();
        let mut best_score = current_score;
        let mut temp = initial_temp.max(0.0001);
        candidates.push(current.clone());

        for _ in 0..iterations.max(1) {
            if deadline_reached(deadline) {
                break;
            }
            let mut next = current.clone();
            if !next.is_empty() {
                let slot = rand_index(&mut s, next.len());
                let candidate = rand_index(&mut s, item_pool.len());
                next[slot] = candidate;
                repair_build(item_pool, &mut next, max_items, &mut s);
                let next_key = canonical_key(&next);
                let next_score = score_fn(&next_key);
                let delta = next_score - current_score;
                let accept = delta >= 0.0 || rand_f64(&mut s) < (delta / temp).exp();
                if accept {
                    current = next;
                    current_score = next_score;
                    candidates.push(current.clone());
                    if current_score > best_score {
                        best_score = current_score;
                        best = current.clone();
                    }
                }
            }
            temp = (temp * cooling_rate.clamp(0.8, 0.9999)).max(0.0001);
        }
        candidates.push(best);
    }

    unique_ranked_from_candidates(candidates, score_fn, limit, deadline)
}

#[derive(Debug, Clone)]
struct MctsNode {
    build: Vec<usize>,
    parent: Option<usize>,
    action_from_parent: Option<usize>,
    children: Vec<usize>,
    untried_actions: Vec<usize>,
    visits: usize,
    value_sum: f64,
}

fn available_actions(item_pool: &[Item], build: &[usize]) -> Vec<usize> {
    (0..item_pool.len())
        .filter(|&idx| can_add_item_to_build(item_pool, build, idx))
        .collect()
}

fn rollout_completion<F>(
    item_pool: &[Item],
    max_items: usize,
    start_build: &[usize],
    seed: &mut u64,
    score_fn: &F,
    deadline: Option<Instant>,
) -> (Vec<usize>, f64)
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut build = start_build.to_vec();
    let mut actions = available_actions(item_pool, &build);
    shuffle_usize(&mut actions, seed);
    for action in actions {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &build, action) {
            build.push(action);
        }
    }
    repair_build(item_pool, &mut build, max_items, seed);
    let key = canonical_key(&build);
    let score = if deadline_reached(deadline) {
        f64::NEG_INFINITY
    } else {
        score_fn(&key)
    };
    (key, score)
}

fn mcts_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    iterations: usize,
    rollouts_per_expansion: usize,
    exploration: f64,
    seed: u64,
    limit: usize,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut s = seed;
    let mut nodes = vec![MctsNode {
        build: vec![],
        parent: None,
        action_from_parent: None,
        children: vec![],
        untried_actions: available_actions(item_pool, &[]),
        visits: 0,
        value_sum: 0.0,
    }];
    let mut all_rollout_keys = Vec::new();

    for _ in 0..iterations.max(1) {
        if deadline_reached(deadline) {
            break;
        }
        let mut node_idx = 0usize;
        loop {
            if nodes[node_idx].build.len() >= max_items {
                break;
            }
            if !nodes[node_idx].untried_actions.is_empty() {
                break;
            }
            if nodes[node_idx].children.is_empty() {
                break;
            }
            let parent_visits = nodes[node_idx].visits.max(1) as f64;
            let mut best_child = nodes[node_idx].children[0];
            let mut best_uct = f64::NEG_INFINITY;
            for &child_idx in &nodes[node_idx].children {
                let child = &nodes[child_idx];
                let exploit = if child.visits == 0 {
                    0.0
                } else {
                    child.value_sum / child.visits as f64
                };
                let explore =
                    exploration * ((parent_visits.ln() / (child.visits.max(1) as f64)).sqrt());
                let uct = exploit + explore;
                if uct > best_uct {
                    best_uct = uct;
                    best_child = child_idx;
                }
            }
            node_idx = best_child;
        }

        if !nodes[node_idx].untried_actions.is_empty() && nodes[node_idx].build.len() < max_items {
            let action_pos = rand_index(&mut s, nodes[node_idx].untried_actions.len());
            let action = nodes[node_idx].untried_actions.swap_remove(action_pos);
            let mut child_build = nodes[node_idx].build.clone();
            child_build.push(action);
            repair_build(item_pool, &mut child_build, max_items, &mut s);
            let child_idx = nodes.len();
            nodes.push(MctsNode {
                build: child_build.clone(),
                parent: Some(node_idx),
                action_from_parent: Some(action),
                children: vec![],
                untried_actions: available_actions(item_pool, &child_build),
                visits: 0,
                value_sum: 0.0,
            });
            nodes[node_idx].children.push(child_idx);
            node_idx = child_idx;
        }

        let mut rollout_scores = Vec::new();
        let rollouts = rollouts_per_expansion.max(1);
        for _ in 0..rollouts {
            if deadline_reached(deadline) {
                break;
            }
            let (key, score) = rollout_completion(
                item_pool,
                max_items,
                &nodes[node_idx].build,
                &mut s,
                score_fn,
                deadline,
            );
            all_rollout_keys.push(key);
            rollout_scores.push(score);
        }
        if rollout_scores.is_empty() {
            break;
        }
        let mean_score = rollout_scores.iter().sum::<f64>() / rollout_scores.len() as f64;

        let mut back = Some(node_idx);
        while let Some(idx) = back {
            nodes[idx].visits += 1;
            nodes[idx].value_sum += mean_score;
            back = nodes[idx].parent;
        }
    }

    let _used_actions = nodes.iter().filter_map(|n| n.action_from_parent).count();
    unique_ranked_from_candidates(all_rollout_keys, score_fn, limit, deadline)
}

fn symmetric_diff_count(a: &[usize], b: &[usize]) -> usize {
    let sa = a.iter().copied().collect::<HashSet<_>>();
    let sb = b.iter().copied().collect::<HashSet<_>>();
    sa.symmetric_difference(&sb).count()
}

fn select_diverse_top_builds(
    ranked: &[(Vec<usize>, f64)],
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
) -> Vec<(Vec<usize>, f64)> {
    if ranked.is_empty() || top_x == 0 {
        return vec![];
    }

    let best_score = ranked[0].1;
    let min_allowed = best_score * (1.0 - (max_relative_gap_percent / 100.0));

    let mut selected: Vec<(Vec<usize>, f64)> = Vec::new();
    for (build, score) in ranked {
        if *score < min_allowed {
            continue;
        }
        if selected
            .iter()
            .all(|(chosen, _)| symmetric_diff_count(chosen, build) >= min_item_diff)
        {
            selected.push((build.clone(), *score));
            if selected.len() >= top_x {
                break;
            }
        }
    }
    selected
}

fn item_names(items: &[Item]) -> String {
    items
        .iter()
        .map(|i| i.name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}

fn build_level_milestones(item_count: usize, start_level: usize, end_level: usize) -> Vec<usize> {
    if item_count == 0 {
        return vec![];
    }
    if item_count == 1 {
        return vec![end_level.max(start_level)];
    }
    let start = start_level as f64;
    let end = end_level as f64;
    let denom = (item_count - 1) as f64;
    (0..item_count)
        .map(|i| {
            let t = (i as f64) / denom;
            (start + (end - start) * t).round().max(1.0) as usize
        })
        .collect()
}

fn acquisition_level_map(items: &[Item], levels: &[usize]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for (item, lvl) in items.iter().zip(levels.iter()) {
        map.insert(item.name.clone(), *lvl);
    }
    map
}

fn level_scaled_enemy_builds(
    level: usize,
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    raw_enemy_bases: &HashMap<String, ChampionBase>,
) -> Vec<(EnemyConfig, Vec<Item>, Stats)> {
    enemy_builds
        .iter()
        .map(|(enemy_cfg, build, bonus_stats)| {
            let raw_base = raw_enemy_bases
                .get(&enemy_cfg.name)
                .cloned()
                .unwrap_or_else(|| enemy_cfg.base.clone());
            let mut scaled_cfg = enemy_cfg.clone();
            scaled_cfg.base = champion_at_level(&raw_base, level);
            (scaled_cfg, build.clone(), bonus_stats.clone())
        })
        .collect()
}

fn score_build_order(
    ordered_items: &[Item],
    levels: &[usize],
    vlad_base_raw: &ChampionBase,
    vlad_bonus_stats: &Stats,
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    raw_enemy_bases: &HashMap<String, ChampionBase>,
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> BuildOrderResult {
    let mut stage_survival = Vec::new();
    let mut cumulative_score = 0.0;
    for (idx, level) in levels.iter().enumerate() {
        let prefix = &ordered_items[..=idx];
        let prefix_levels = &levels[..=idx];
        let acquired_map = acquisition_level_map(prefix, prefix_levels);
        let vlad_base_level = champion_at_level(vlad_base_raw, *level);
        let enemy_level_builds = level_scaled_enemy_builds(*level, enemy_builds, raw_enemy_bases);
        let mut sim_at_level = sim.clone();
        sim_at_level.champion_level = *level;
        let t = simulate_vlad_survival(
            &vlad_base_level,
            prefix,
            vlad_bonus_stats,
            Some(&acquired_map),
            &enemy_level_builds,
            &sim_at_level,
            urf,
        );
        stage_survival.push(t);
        cumulative_score += t;
    }
    BuildOrderResult {
        ordered_items: ordered_items.to_vec(),
        levels: levels.to_vec(),
        acquired_levels: levels.to_vec(),
        stage_survival,
        cumulative_score,
    }
}

fn optimize_build_order(
    build_items: &[Item],
    vlad_base_raw: &ChampionBase,
    vlad_bonus_stats: &Stats,
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    raw_enemy_bases: &HashMap<String, ChampionBase>,
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> BuildOrderResult {
    let levels = build_level_milestones(build_items.len(), 5, 20);
    let mut best = score_build_order(
        build_items,
        &levels,
        vlad_base_raw,
        vlad_bonus_stats,
        enemy_builds,
        raw_enemy_bases,
        sim,
        urf,
    );
    if build_items.len() <= 1 {
        return best;
    }

    let beam_width = 40usize;
    let mut frontier = vec![Vec::<usize>::new()];
    for depth in 0..build_items.len() {
        let mut expanded = Vec::<(Vec<usize>, f64)>::new();
        for partial in &frontier {
            let mut used = partial.iter().copied().collect::<HashSet<_>>();
            for idx in 0..build_items.len() {
                if used.contains(&idx) {
                    continue;
                }
                let mut candidate = partial.clone();
                candidate.push(idx);
                used.insert(idx);
                let ordered = candidate
                    .iter()
                    .map(|i| build_items[*i].clone())
                    .collect::<Vec<_>>();
                let partial_levels = levels[..candidate.len()].to_vec();
                let current = score_build_order(
                    &ordered,
                    &partial_levels,
                    vlad_base_raw,
                    vlad_bonus_stats,
                    enemy_builds,
                    raw_enemy_bases,
                    sim,
                    urf,
                );
                let optimistic_upper_bound = current.cumulative_score
                    + (build_items.len() - candidate.len()) as f64 * sim.max_time_seconds;
                expanded.push((candidate, optimistic_upper_bound));
            }
        }
        expanded.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        frontier = expanded
            .into_iter()
            .take(beam_width)
            .map(|(candidate, _)| candidate)
            .collect::<Vec<_>>();
        if frontier.is_empty() {
            break;
        }
        if depth + 1 == build_items.len() {
            for order_idx in &frontier {
                let ordered = order_idx
                    .iter()
                    .map(|i| build_items[*i].clone())
                    .collect::<Vec<_>>();
                let scored = score_build_order(
                    &ordered,
                    &levels,
                    vlad_base_raw,
                    vlad_bonus_stats,
                    enemy_builds,
                    raw_enemy_bases,
                    sim,
                    urf,
                );
                if scored.cumulative_score > best.cumulative_score {
                    best = scored;
                }
            }
        }
    }

    best
}

fn default_report_path() -> PathBuf {
    simulation_dir()
        .join("output")
        .join("vladimir_run_report.md")
}

fn write_vladimir_report_markdown(
    report_path: &Path,
    scenario_path: &Path,
    sim: &SimulationConfig,
    vlad_base_level: &ChampionBase,
    vlad_end_stats: &Stats,
    stack_notes: &[String],
    vlad_loadout: &ResolvedLoadout,
    enemy_loadout: &ResolvedLoadout,
    baseline_build: &[Item],
    baseline_score: f64,
    baseline_outcome: &CombatOutcome,
    best_build: &[Item],
    best_score: f64,
    best_outcome: &CombatOutcome,
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    enemy_presets_used: &HashMap<String, EnemyUrfPreset>,
    diverse_top_builds: &[(Vec<Item>, f64)],
    diverse_top_keys: &[Vec<usize>],
    build_confidence: &[BuildConfidence],
    metrics_by_key: &HashMap<Vec<usize>, BuildMetrics>,
    pareto_front: &HashSet<Vec<usize>>,
    diagnostics: &SearchDiagnostics,
    build_orders: &[BuildOrderResult],
) -> Result<()> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let improvement = if baseline_score.abs() > f64::EPSILON {
        ((best_score - baseline_score) / baseline_score) * 100.0
    } else {
        0.0
    };

    let mut content = String::new();
    content.push_str("# Vladimir URF Run Report\n\n");
    content.push_str(&format!("- Generated (unix): `{}`\n", now));
    content.push_str(&format!("- Scenario: `{}`\n\n", scenario_path.display()));

    content.push_str("## Headline\n");
    content.push_str(&format!(
        "- Baseline objective score: **{:.4}**\n- Best objective score: **{:.4}**\n- Improvement: **{:+.2}%**\n- Baseline time alive / damage dealt / healing done / enemy kills: **{:.2}s / {:.1} / {:.1} / {}**\n- Best time alive / damage dealt / healing done / enemy kills: **{:.2}s / {:.1} / {:.1} / {}**\n- Baseline cap survivor: **{}**\n- Best cap survivor: **{}**\n\n",
        baseline_score,
        best_score,
        improvement,
        baseline_outcome.time_alive_seconds,
        baseline_outcome.damage_dealt,
        baseline_outcome.healing_done,
        baseline_outcome.enemy_kills,
        best_outcome.time_alive_seconds,
        best_outcome.damage_dealt,
        best_outcome.healing_done,
        best_outcome.enemy_kills,
        baseline_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
        best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
    ));

    content.push_str(&format!(
        "- Champion level assumption: **{}**\n\n",
        sim.champion_level
    ));

    let (seed_mean, seed_std) = mean_std(&diagnostics.seed_best_scores);
    content.push_str("## Search Diagnostics\n");
    content.push_str(&format!(
        "- Strategy: `{}`\n- Search quality profile: `{}`\n- Enemy scenarios: `{}`\n- Loadout candidates/finalists: `{}/{}`\n- Ensemble seeds: `{}`\n- Objective weights (survival/damage/healing): `{:.2}/{:.2}/{:.2}`\n- Full evaluations: `{}` (cache hits/misses/waits: `{}/{}/{}`)\n- Full persistent cache hits/entries: `{}/{}`\n- Unique candidate builds: `{}`\n- Bleed candidates injected: `{}`\n- Adaptive candidates injected: `{}`\n- Seed-best mean/stddev: `{:.2}` / `{:.3}`\n\n",
        diagnostics.strategy_summary,
        diagnostics.search_quality_profile,
        diagnostics.scenario_count,
        diagnostics.loadout_candidates,
        diagnostics.loadout_finalists,
        diagnostics.ensemble_seeds,
        diagnostics.objective_survival_weight,
        diagnostics.objective_damage_weight,
        diagnostics.objective_healing_weight,
        diagnostics.full_evaluations,
        diagnostics.full_cache_hits,
        diagnostics.full_cache_misses,
        diagnostics.full_cache_waits,
        diagnostics.full_persistent_cache_hits,
        diagnostics.full_persistent_cache_entries,
        diagnostics.unique_candidate_builds,
        diagnostics.bleed_candidates_injected,
        diagnostics.adaptive_candidates_injected,
        seed_mean,
        seed_std
    ));
    if let Some(budget) = diagnostics.time_budget_seconds {
        content.push_str(&format!(
            "- Time budget: `{:.1}s`; elapsed: `{:.1}s`; timed_out: `{}`; progress: `{}/{}`\n\n",
            budget,
            diagnostics.elapsed_seconds,
            diagnostics.timed_out,
            diagnostics.processed_candidates,
            diagnostics.total_candidates
        ));
    } else {
        content.push_str(&format!(
            "- Elapsed: `{:.1}s`; progress: `{}/{}`\n\n",
            diagnostics.elapsed_seconds,
            diagnostics.processed_candidates,
            diagnostics.total_candidates
        ));
    }

    content.push_str("## Vladimir Base Stats At Level\n");
    content.push_str(&format!(
        "- HP: {:.1}, Armor: {:.1}, MR: {:.1}, AD: {:.1}, AS: {:.3}, MS: {:.1}\n\n",
        vlad_base_level.base_health,
        vlad_base_level.base_armor,
        vlad_base_level.base_magic_resist,
        vlad_base_level.base_attack_damage,
        vlad_base_level.base_attack_speed,
        vlad_base_level.base_move_speed
    ));

    content.push_str("## Selected Runes/Masteries\n");
    if vlad_loadout.selection_labels.is_empty() {
        content.push_str("- Vladimir: none selected.\n");
    } else {
        content.push_str("- Vladimir:\n");
        for s in &vlad_loadout.selection_labels {
            content.push_str(&format!("  - {}\n", s));
        }
    }
    if enemy_loadout.selection_labels.is_empty() {
        content.push_str("- Enemies: none selected.\n\n");
    } else {
        content.push_str("- Enemies (applied to all):\n");
        for s in &enemy_loadout.selection_labels {
            content.push_str(&format!("  - {}\n", s));
        }
        content.push('\n');
    }
    if !vlad_loadout.applied_notes.is_empty() || !enemy_loadout.applied_notes.is_empty() {
        content.push_str("- Applied deterministic loadout effects:\n");
        for note in &vlad_loadout.applied_notes {
            content.push_str(&format!("  - Vladimir: {}\n", note));
        }
        for note in &enemy_loadout.applied_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    if !vlad_loadout.skipped_notes.is_empty() || !enemy_loadout.skipped_notes.is_empty() {
        content.push_str("- Skipped unsupported/non-deterministic effects:\n");
        for note in &vlad_loadout.skipped_notes {
            content.push_str(&format!("  - Vladimir: {}\n", note));
        }
        for note in &enemy_loadout.skipped_notes {
            content.push_str(&format!("  - Enemies: {}\n", note));
        }
    }
    content.push('\n');

    content.push_str("## Baseline Build\n");
    content.push_str(&format!("- {}\n\n", item_names(baseline_build)));

    content.push_str("## Best Build\n");
    content.push_str(&format!("- {}\n\n", item_names(best_build)));

    content.push_str("## Vladimir End Stats (Best Build)\n");
    content.push_str(&format!(
        "- HP: {:.1}, Armor: {:.1}, MR: {:.1}, AP: {:.1}, AD: {:.1}, Ability Haste: {:.1}, Move Speed (flat bonus): {:.1}, Move Speed (% bonus): {:.1}\n\n",
        vlad_end_stats.health,
        vlad_end_stats.armor,
        vlad_end_stats.magic_resist,
        vlad_end_stats.ability_power,
        vlad_end_stats.attack_damage,
        vlad_end_stats.ability_haste,
        vlad_end_stats.move_speed_flat,
        vlad_end_stats.move_speed_percent
    ));

    content.push_str("## Stack Assumptions\n");
    if stack_notes.is_empty() {
        content.push_str(
            "- No explicit stack assumptions triggered for selected best build items.\n\n",
        );
    } else {
        for note in stack_notes {
            content.push_str(&format!("- {}\n", note));
        }
        content.push('\n');
    }

    content.push_str("## Enemy Builds (URF Presets)\n");
    for (enemy, build, _) in enemy_builds {
        content.push_str(&format!("- {}: {}\n", enemy.name, item_names(build)));
        if let Some(preset) = enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            content.push_str(&format!(
                "  - Source: {} (last checked {})\n",
                preset.source_url, preset.last_checked
            ));
            content.push_str(&format!("  - Runes: {}\n", preset.runes.join(", ")));
            content.push_str(&format!(
                "  - Masteries: {}\n",
                preset
                    .masteries
                    .iter()
                    .map(|m| format!("{} ({})", m.name, m.rank))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }
    content.push('\n');

    content.push_str("## Diverse Top Builds\n");
    if diverse_top_builds.is_empty() {
        content.push_str("- No diverse builds found under current thresholds.\n\n");
    } else {
        let best = diverse_top_builds[0].1;
        for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
            let delta = score - best;
            let key = diverse_top_keys.get(idx);
            let confidence = key
                .and_then(|k| build_confidence.iter().find(|c| c.key == *k))
                .map(|c| {
                    format!(
                        " | seed hits: {}/{} ({:.0}%) {}",
                        c.seed_hits,
                        diagnostics.ensemble_seeds,
                        c.seed_hit_rate * 100.0,
                        c.robustness
                    )
                })
                .unwrap_or_default();
            let pareto = key.map(|k| pareto_front.contains(k)).unwrap_or(false);
            let pareto_tag = if pareto { " | Pareto-front" } else { "" };
            content.push_str(&format!(
                "{}. `score {:.4}` ({:+.4} vs top): {}{}{}\n",
                idx + 1,
                score,
                delta,
                item_names(build),
                confidence,
                pareto_tag
            ));
            if let Some(k) = key {
                if let Some(m) = metrics_by_key.get(k) {
                    content.push_str(&format!(
                        "   - metrics: EHP~{:.1}, AP~{:.1}, timing score {:+.2}, total cost {:.0}\n",
                        m.ehp_mixed, m.ap, m.cost_timing, m.total_cost
                    ));
                }
            }
        }
        content.push('\n');
    }

    content.push_str("## Build Order Optimization\n");
    if build_orders.is_empty() {
        content.push_str("- No build-order optimization results available.\n\n");
    } else {
        for (idx, br) in build_orders.iter().enumerate() {
            content.push_str(&format!(
                "{}. Cumulative score: `{:.2}` | Order: {}\n",
                idx + 1,
                br.cumulative_score,
                item_names(&br.ordered_items)
            ));
            for (stage_idx, (lvl, surv)) in
                br.levels.iter().zip(br.stage_survival.iter()).enumerate()
            {
                content.push_str(&format!(
                    "   - Stage {} (level {}): `{:.2}s`\n",
                    stage_idx + 1,
                    lvl,
                    surv
                ));
            }
        }
        content.push('\n');
    }

    content.push_str("## Deeper Insights\n");
    if !diverse_top_builds.is_empty() {
        let mut item_counts: HashMap<String, usize> = HashMap::new();
        for (build, _) in diverse_top_builds {
            for item in build {
                *item_counts.entry(item.name.clone()).or_insert(0) += 1;
            }
        }
        let mut counts = item_counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let core_items = counts
            .iter()
            .filter(|(_, c)| *c == diverse_top_builds.len())
            .map(|(n, _)| n.clone())
            .collect::<Vec<_>>();
        let top_freq = counts
            .iter()
            .take(8)
            .map(|(n, c)| format!("{} ({}/{})", n, c, diverse_top_builds.len()))
            .collect::<Vec<_>>();

        if core_items.is_empty() {
            content.push_str("- No single item appears in every selected diverse top build.\n");
        } else {
            content.push_str(&format!(
                "- Common core across all selected top builds: {}.\n",
                core_items.join(", ")
            ));
        }
        content.push_str(&format!(
            "- Most frequent items in selected top set: {}.\n",
            top_freq.join(", ")
        ));
        content.push_str("- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.\n");
    } else {
        content.push_str("- Broaden thresholds (`--max-relative-gap-percent`) or lower diversity constraint (`--min-item-diff`) to surface more alternatives.\n");
    }

    fs::write(report_path, content)
        .with_context(|| format!("Failed writing report {}", report_path.display()))?;
    Ok(())
}

fn write_vladimir_report_json(
    report_path: &Path,
    scenario_path: &Path,
    sim: &SimulationConfig,
    baseline_build: &[Item],
    baseline_score: f64,
    baseline_outcome: &CombatOutcome,
    best_build: &[Item],
    best_score: f64,
    best_outcome: &CombatOutcome,
    vladimir_loadout: &ResolvedLoadout,
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    enemy_presets_used: &HashMap<String, EnemyUrfPreset>,
    diverse_top_builds: &[(Vec<Item>, f64)],
    diagnostics: &SearchDiagnostics,
    build_orders: &[BuildOrderResult],
) -> Result<()> {
    if let Some(parent) = report_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed creating report directory {}", parent.display()))?;
    }
    let improvement_percent = if baseline_score.abs() > f64::EPSILON {
        ((best_score - baseline_score) / baseline_score) * 100.0
    } else {
        0.0
    };
    let json_value = json!({
        "scenario_path": scenario_path.display().to_string(),
        "champion_level": sim.champion_level,
        "headline": {
            "baseline_objective_score": baseline_score,
            "best_objective_score": best_score,
            "improvement_percent": improvement_percent,
            "baseline_outcome": {
                "time_alive_seconds": baseline_outcome.time_alive_seconds,
                "damage_dealt": baseline_outcome.damage_dealt,
                "healing_done": baseline_outcome.healing_done,
                "enemy_kills": baseline_outcome.enemy_kills,
                "cap_survivor": baseline_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
            },
            "best_outcome": {
                "time_alive_seconds": best_outcome.time_alive_seconds,
                "damage_dealt": best_outcome.damage_dealt,
                "healing_done": best_outcome.healing_done,
                "enemy_kills": best_outcome.enemy_kills,
                "cap_survivor": best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6,
            },
        },
        "baseline_build": baseline_build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
        "best_build": best_build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
        "vladimir_loadout_labels": vladimir_loadout.selection_labels,
        "enemy_presets": enemy_builds.iter().map(|(enemy, build, _)| {
            let key = to_norm_key(&enemy.name);
            let preset = enemy_presets_used.get(&key);
            json!({
                "champion": enemy.name,
                "items": build.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                "runes": preset.map(|p| p.runes.clone()).unwrap_or_default(),
                "shards": preset.map(|p| p.shards.clone()).unwrap_or_default(),
                "masteries": preset.map(|p| p.masteries.iter().map(|m| json!({"name": m.name, "rank": m.rank})).collect::<Vec<_>>()).unwrap_or_default(),
                "source_url": preset.map(|p| p.source_url.clone()).unwrap_or_default(),
                "last_checked": preset.map(|p| p.last_checked.clone()).unwrap_or_default(),
            })
        }).collect::<Vec<_>>(),
        "diverse_top_builds": diverse_top_builds.iter().map(|(build, score)| {
            json!({
                "objective_score": score,
                "items": build.iter().map(|i| i.name.clone()).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>(),
        "build_orders": build_orders.iter().map(|order| {
            json!({
                "cumulative_score": order.cumulative_score,
                "ordered_items": order.ordered_items.iter().map(|i| i.name.clone()).collect::<Vec<_>>(),
                "levels": order.levels,
                "stage_survival_seconds": order.stage_survival,
            })
        }).collect::<Vec<_>>(),
        "diagnostics": {
            "strategy_summary": diagnostics.strategy_summary,
            "search_quality_profile": diagnostics.search_quality_profile,
            "ensemble_seeds": diagnostics.ensemble_seeds,
            "objective_survival_weight": diagnostics.objective_survival_weight,
            "objective_damage_weight": diagnostics.objective_damage_weight,
            "objective_healing_weight": diagnostics.objective_healing_weight,
            "full_evaluations": diagnostics.full_evaluations,
            "full_cache_hits": diagnostics.full_cache_hits,
            "full_cache_misses": diagnostics.full_cache_misses,
            "full_cache_waits": diagnostics.full_cache_waits,
            "full_persistent_cache_hits": diagnostics.full_persistent_cache_hits,
            "full_persistent_cache_entries": diagnostics.full_persistent_cache_entries,
            "unique_candidate_builds": diagnostics.unique_candidate_builds,
            "bleed_candidates_injected": diagnostics.bleed_candidates_injected,
            "adaptive_candidates_injected": diagnostics.adaptive_candidates_injected,
            "scenario_count": diagnostics.scenario_count,
            "loadout_candidates": diagnostics.loadout_candidates,
            "loadout_finalists": diagnostics.loadout_finalists,
            "time_budget_seconds": diagnostics.time_budget_seconds,
            "elapsed_seconds": diagnostics.elapsed_seconds,
            "timed_out": diagnostics.timed_out,
            "processed_candidates": diagnostics.processed_candidates,
            "total_candidates": diagnostics.total_candidates
        }
    });
    fs::write(report_path, serde_json::to_string_pretty(&json_value)?)
        .with_context(|| format!("Failed writing JSON report {}", report_path.display()))?;
    Ok(())
}

fn choose_best_build_by_stat(
    item_pool: &[Item],
    stat_key: &str,
    max_items: usize,
    beam_width: usize,
) -> Vec<usize> {
    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    for _ in 0..max_items {
        let mut next_candidates = Vec::new();
        for build in &candidates {
            let has_boots = build.iter().any(|&i| is_boots(&item_pool[i]));
            let used = build.iter().copied().collect::<HashSet<_>>();
            for item_idx in 0..item_pool.len() {
                if used.contains(&item_idx) {
                    continue;
                }
                if is_boots(&item_pool[item_idx]) && has_boots {
                    continue;
                }
                let mut next = build.clone();
                next.push(item_idx);
                next_candidates.push(next);
            }
        }
        next_candidates.sort_by(|a, b| {
            let sa = build_item_stats(&build_from_indices(item_pool, a)).get_stat(stat_key);
            let sb = build_item_stats(&build_from_indices(item_pool, b)).get_stat(stat_key);
            sb.partial_cmp(&sa).unwrap_or(Ordering::Equal)
        });
        next_candidates.truncate(beam_width);
        candidates = next_candidates;
    }
    candidates.into_iter().next().unwrap_or_default()
}

fn build_search_ranked<F>(
    item_pool: &[Item],
    max_items: usize,
    search: &BuildSearchConfig,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    if deadline_reached(deadline) {
        return Vec::new();
    }
    match search.strategy.as_str() {
        "greedy" => {
            let mut build = Vec::new();
            for _ in 0..max_items {
                if deadline_reached(deadline) {
                    break;
                }
                let mut best: Option<usize> = None;
                let mut best_score = f64::NEG_INFINITY;
                for item_idx in 0..item_pool.len() {
                    if !can_add_item_to_build(item_pool, &build, item_idx) {
                        continue;
                    }
                    let mut candidate = build.clone();
                    candidate.push(item_idx);
                    let score = score_fn(&canonical_key(&candidate));
                    if score > best_score {
                        best_score = score;
                        best = Some(item_idx);
                    }
                }
                if let Some(item_idx) = best {
                    build.push(item_idx);
                } else {
                    break;
                }
            }
            let key = canonical_key(&build);
            vec![(key.clone(), score_fn(&key))]
        }
        "beam" => beam_search_ranked(item_pool, max_items, search.beam_width, score_fn, deadline),
        "random" => random_search_ranked(
            item_pool,
            max_items,
            search.random_samples,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "hill_climb" => hill_climb_search_ranked(
            item_pool,
            max_items,
            search.hill_climb_restarts,
            search.hill_climb_steps,
            search.hill_climb_neighbors,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "genetic" => genetic_search_ranked(
            item_pool,
            max_items,
            search.genetic_population,
            search.genetic_generations,
            search.genetic_mutation_rate,
            search.genetic_crossover_rate,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "simulated_annealing" => simulated_annealing_search_ranked(
            item_pool,
            max_items,
            search.simulated_annealing_restarts,
            search.simulated_annealing_iterations,
            search.simulated_annealing_initial_temp,
            search.simulated_annealing_cooling_rate,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "mcts" => mcts_search_ranked(
            item_pool,
            max_items,
            search.mcts_iterations,
            search.mcts_rollouts_per_expansion,
            search.mcts_exploration,
            search.seed,
            search.ranked_limit,
            score_fn,
            deadline,
        ),
        "portfolio" => {
            let strategies = portfolio_strategy_list(search);
            let mut ranked_sets = Vec::new();
            for (idx, strat) in strategies.iter().enumerate() {
                if deadline_reached(deadline) {
                    break;
                }
                let mut cfg = search.clone();
                cfg.strategy = strat.clone();
                cfg.seed = search.seed.wrapping_add((idx as u64 + 1) * 1_000_003);
                ranked_sets.push(build_search_ranked(
                    item_pool, max_items, &cfg, score_fn, deadline,
                ));
            }
            let mut merged_candidates = Vec::new();
            for ranked in ranked_sets {
                for (build, _) in ranked {
                    merged_candidates.push(build);
                }
            }
            unique_ranked_from_candidates(
                merged_candidates,
                score_fn,
                search.ranked_limit,
                deadline,
            )
        }
        _ => vec![],
    }
}

fn portfolio_strategy_list(search: &BuildSearchConfig) -> Vec<String> {
    if search.strategy != "portfolio" {
        return vec![search.strategy.clone()];
    }
    let mut strategies = if search.portfolio_strategies.is_empty() {
        vec![
            "beam".to_string(),
            "hill_climb".to_string(),
            "genetic".to_string(),
            "simulated_annealing".to_string(),
            "mcts".to_string(),
            "random".to_string(),
            "greedy".to_string(),
        ]
    } else {
        search.portfolio_strategies.clone()
    };
    strategies.retain(|s| s != "portfolio");
    if strategies.is_empty() {
        strategies.push("beam".to_string());
    }
    strategies
}

fn search_strategy_summary(search: &BuildSearchConfig) -> String {
    if search.strategy == "portfolio" {
        let strategies = portfolio_strategy_list(search);
        format!("portfolio({})", strategies.join(", "))
    } else {
        search.strategy.clone()
    }
}

fn strategy_seed_elites<F>(
    item_pool: &[Item],
    max_items: usize,
    search: &BuildSearchConfig,
    strategies: &[String],
    score_fn: &F,
    deadline: Option<Instant>,
) -> HashMap<String, Vec<Vec<usize>>>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let ensemble = search.ensemble_seeds.max(1);
    let top_k = search.ensemble_seed_top_k.max(1);

    let grouped = strategies
        .iter()
        .enumerate()
        .map(|(sidx, strategy)| {
            let mut aggregate = HashMap::<Vec<usize>, f64>::new();
            for seed_idx in 0..ensemble {
                if deadline_reached(deadline) {
                    break;
                }
                let mut cfg = search.clone();
                cfg.strategy = strategy.clone();
                cfg.seed = search.seed.wrapping_add(
                    ((sidx as u64 + 1) * 31 + seed_idx as u64 + 1) * search.ensemble_seed_stride,
                );
                cfg.ranked_limit = top_k.max(64);
                let ranked = build_search_ranked(item_pool, max_items, &cfg, score_fn, deadline);
                for (key, score) in ranked.into_iter().take(top_k) {
                    let e = aggregate.entry(key).or_insert(score);
                    if score > *e {
                        *e = score;
                    }
                }
            }
            let mut items = aggregate.into_iter().collect::<Vec<_>>();
            items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
            let keys = items.into_iter().map(|(k, _)| k).collect::<Vec<_>>();
            (strategy.clone(), keys)
        })
        .collect::<Vec<_>>();

    grouped.into_iter().collect::<HashMap<_, _>>()
}

fn generate_bleed_candidates(
    item_pool: &[Item],
    max_items: usize,
    strategy_elites: &HashMap<String, Vec<Vec<usize>>>,
    search: &BuildSearchConfig,
) -> Vec<Vec<usize>> {
    if !search.bleed_enabled {
        return Vec::new();
    }
    let mut seed = search.seed ^ 0xB1EEDu64;
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    let mut elite_pool = Vec::new();

    for builds in strategy_elites.values() {
        for key in builds.iter().take(search.ensemble_seed_top_k.max(1)) {
            let canon = canonical_key(key);
            if seen.insert(canon.clone()) {
                out.push(canon.clone());
                elite_pool.push(canon);
            }
        }
    }
    if elite_pool.is_empty() {
        return out;
    }

    let bleed_budget = if search.bleed_budget > 0 {
        search.bleed_budget
    } else {
        // Max-quality default: at least ranked candidate pool size, with a reasonable floor.
        search.ranked_limit.max(800)
    };
    let cross_budget = bleed_budget / 2;
    let mutate_budget = bleed_budget - cross_budget;
    let mutation_rate = search.bleed_mutation_rate.clamp(0.0, 1.0);

    for _ in 0..cross_budget {
        let a = rand_index(&mut seed, elite_pool.len());
        let b = if strategies.len() >= 2 {
            let sa = rand_index(&mut seed, strategies.len());
            let mut sb = rand_index(&mut seed, strategies.len());
            if sb == sa {
                sb = (sb + 1) % strategies.len();
            }
            let list_a = strategy_elites.get(&strategies[sa]).unwrap_or(&elite_pool);
            let list_b = strategy_elites.get(&strategies[sb]).unwrap_or(&elite_pool);
            let pa = list_a
                .get(rand_index(&mut seed, list_a.len()))
                .cloned()
                .unwrap_or_else(|| elite_pool[a].clone());
            let pb = list_b
                .get(rand_index(&mut seed, list_b.len()))
                .cloned()
                .unwrap_or_else(|| elite_pool[a].clone());
            let mut child = crossover_builds(&pa, &pb, item_pool, max_items, &mut seed);
            mutate_build(&mut child, item_pool, max_items, mutation_rate, &mut seed);
            canonical_key(&child)
        } else {
            let mut child = elite_pool[a].clone();
            mutate_build(&mut child, item_pool, max_items, mutation_rate, &mut seed);
            canonical_key(&child)
        };
        if seen.insert(b.clone()) {
            out.push(b);
        }
    }

    for _ in 0..mutate_budget {
        let mut child = elite_pool[rand_index(&mut seed, elite_pool.len())].clone();
        mutate_build(&mut child, item_pool, max_items, mutation_rate, &mut seed);
        repair_build(item_pool, &mut child, max_items, &mut seed);
        let key = canonical_key(&child);
        if seen.insert(key.clone()) {
            out.push(key);
        }
    }

    out
}

fn adaptive_strategy_candidates<F>(
    item_pool: &[Item],
    max_items: usize,
    search: &BuildSearchConfig,
    strategy_elites: &HashMap<String, Vec<Vec<usize>>>,
    score_fn: &F,
    deadline: Option<Instant>,
) -> Vec<Vec<usize>>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    if strategy_elites.is_empty() {
        return Vec::new();
    }
    let strategies = strategy_elites.keys().cloned().collect::<Vec<_>>();
    let contributions = strategies
        .iter()
        .map(|s| {
            let c = strategy_elites
                .get(s)
                .map(|v| v.len().max(1) as f64)
                .unwrap_or(1.0);
            (s.clone(), c)
        })
        .collect::<Vec<_>>();
    let total_contrib = contributions.iter().map(|(_, c)| *c).sum::<f64>().max(1.0);
    let extra_runs_total = (search.ensemble_seeds.max(1) * strategies.len()).max(8);
    let per_strategy = contributions
        .into_iter()
        .map(|(s, c)| {
            let share = c / total_contrib;
            let runs = ((extra_runs_total as f64) * share).round() as usize;
            (s, runs.max(1))
        })
        .collect::<Vec<_>>();

    let gathered = per_strategy
        .iter()
        .enumerate()
        .map(|(sidx, (strategy, runs))| {
            let mut local = Vec::new();
            for ridx in 0..*runs {
                if deadline_reached(deadline) {
                    break;
                }
                let mut cfg = search.clone();
                cfg.strategy = strategy.clone();
                cfg.seed = search.seed.wrapping_add(
                    ((sidx as u64 + 1) * 131 + ridx as u64 + 1) * search.ensemble_seed_stride,
                );
                cfg.ranked_limit = (search.ensemble_seed_top_k.max(1) * 2).max(50);
                let ranked = build_search_ranked(item_pool, max_items, &cfg, score_fn, deadline);
                for (k, _) in ranked.into_iter().take(search.ensemble_seed_top_k.max(1)) {
                    local.push(k);
                }
            }
            local
        })
        .collect::<Vec<_>>();

    gathered
        .into_iter()
        .flatten()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

fn effective_hp_mixed(health: f64, armor: f64, magic_resist: f64) -> f64 {
    let phys_mult = 1.0 + armor.max(0.0) / 100.0;
    let magic_mult = 1.0 + magic_resist.max(0.0) / 100.0;
    health.max(1.0) * 0.5 * (phys_mult + magic_mult)
}

fn build_cost_timing_score(build: &[Item]) -> f64 {
    if build.is_empty() {
        return 0.0;
    }
    let mut weighted = 0.0;
    let mut total = 0.0;
    for (idx, item) in build.iter().enumerate() {
        let w = 1.0 / (1.0 + idx as f64);
        weighted += w * item.total_cost.max(0.0);
        total += item.total_cost.max(0.0);
    }
    // Higher is better. Penalize expensive early spikes more.
    -weighted - 0.1 * total
}

fn compute_build_metrics(
    key: &[usize],
    item_pool: &[Item],
    vlad_base: &ChampionBase,
    vlad_bonus_stats: &Stats,
    sim: &SimulationConfig,
    objective: f64,
) -> BuildMetrics {
    let build = build_from_indices(item_pool, key);
    let item_stats = compute_effective_item_stats_for_build(
        vlad_base,
        &build,
        vlad_bonus_stats,
        sim,
        sim.champion_level,
        None,
    );
    let stats = compute_vlad_stats(vlad_base, &item_stats);
    let ehp = effective_hp_mixed(stats.health, stats.armor, stats.magic_resist);
    let total_cost = build.iter().map(|i| i.total_cost).sum::<f64>();
    BuildMetrics {
        objective,
        ehp_mixed: ehp,
        ap: stats.ability_power,
        cost_timing: build_cost_timing_score(&build),
        total_cost,
    }
}

fn dominates(a: &BuildMetrics, b: &BuildMetrics) -> bool {
    let ge = a.objective >= b.objective
        && a.ehp_mixed >= b.ehp_mixed
        && a.ap >= b.ap
        && a.cost_timing >= b.cost_timing;
    let gt = a.objective > b.objective
        || a.ehp_mixed > b.ehp_mixed
        || a.ap > b.ap
        || a.cost_timing > b.cost_timing;
    ge && gt
}

fn pareto_front_keys(metrics_by_key: &HashMap<Vec<usize>, BuildMetrics>) -> HashSet<Vec<usize>> {
    let keys = metrics_by_key.keys().cloned().collect::<Vec<_>>();
    let mut front = HashSet::new();
    for key_a in &keys {
        let Some(a) = metrics_by_key.get(key_a) else {
            continue;
        };
        let dominated = keys.iter().any(|key_b| {
            if key_a == key_b {
                return false;
            }
            let Some(b) = metrics_by_key.get(key_b) else {
                return false;
            };
            dominates(b, a)
        });
        if !dominated {
            front.insert(key_a.clone());
        }
    }
    front
}

fn mean_std(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|v| {
            let d = *v - mean;
            d * d
        })
        .sum::<f64>()
        / values.len() as f64;
    (mean, var.sqrt())
}

fn item_pool_from_names(items: &HashMap<String, Item>, names: &[String]) -> Result<Vec<Item>> {
    let mut out = Vec::new();
    for name in names {
        let resolved = resolve_evolved_item_name(items, name);
        out.push(
            items
                .get(&resolved)
                .cloned()
                .ok_or_else(|| anyhow!("Item not found: {}", resolved))?,
        );
    }
    Ok(out)
}

fn resolve_evolved_item_name(items: &HashMap<String, Item>, name: &str) -> String {
    for (source, evolved) in ITEM_EVOLUTION_REPLACEMENTS {
        if *source == name && items.contains_key(*evolved) {
            return (*evolved).to_string();
        }
    }
    name.to_string()
}

fn is_legendary(item: &Item) -> bool {
    item.rank.iter().any(|r| r == LEGENDARY_RANK)
}

fn is_pre_evolution_item(items: &HashMap<String, Item>, item_name: &str) -> bool {
    ITEM_EVOLUTION_REPLACEMENTS
        .iter()
        .any(|(source, evolved)| *source == item_name && items.contains_key(*evolved))
}

fn is_evolution_target(item_name: &str) -> bool {
    ITEM_EVOLUTION_REPLACEMENTS
        .iter()
        .any(|(_, evolved)| *evolved == item_name)
}

fn looks_arena_or_non_summoners_rift(item: &Item) -> bool {
    // Conservative guard: these naming patterns are commonly Arena/distributed-only.
    // We already constrain to LEGENDARY for search; this helps future-proof odd imports.
    let lower = item.name.to_ascii_lowercase();
    let arena_like_tokens = [
        "dragonheart",
        "hemomancer",
        "runecarver",
        "gambler",
        "golden spatula",
        "black hole gauntlet",
        "reaper",
        "demon king",
        "pyromancer",
        "molten stone",
        "wooglet",
        "entropy",
        "decapitator",
        "regicide",
        "lucky dice",
    ];
    arena_like_tokens.iter().any(|t| lower.contains(t))
}

fn default_item_pool(items: &HashMap<String, Item>) -> Vec<Item> {
    let mut pool = items
        .values()
        .filter(|item| item.shop_purchasable || is_evolution_target(&item.name))
        .filter(|item| is_legendary(item))
        .filter(|item| !is_pre_evolution_item(items, &item.name))
        .filter(|item| !looks_arena_or_non_summoners_rift(item))
        .cloned()
        .collect::<Vec<_>>();
    pool.sort_by(|a, b| a.name.cmp(&b.name));
    pool
}

#[derive(Debug, Clone)]
struct EnemyUrfPreset {
    champion: String,
    source_url: String,
    last_checked: String,
    item_names: Vec<String>,
    runes: Vec<String>,
    shards: Vec<String>,
    masteries: Vec<MasterySelection>,
}

fn enemy_preset_data_path() -> PathBuf {
    simulation_data_dir().join("enemy_urf_presets.json")
}

fn load_enemy_urf_presets() -> Result<HashMap<String, EnemyUrfPreset>> {
    let data = load_json(&enemy_preset_data_path())?;
    let defaults = data.get("defaults").and_then(Value::as_object);
    let default_source_url = defaults
        .and_then(|o| o.get("source_url"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let default_last_checked = defaults
        .and_then(|o| o.get("last_checked"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let presets = data
        .get("presets")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            anyhow!(
                "Missing presets array in {}",
                enemy_preset_data_path().display()
            )
        })?;

    let mut by_champion = HashMap::new();
    for preset in presets {
        let champion = preset
            .get("champion")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Missing preset champion field"))?
            .to_string();
        let item_names = preset
            .get("item_names")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing item_names for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let runes = preset
            .get("runes")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing runes for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let shards = preset
            .get("shards")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing shards for preset {}", champion))?
            .iter()
            .filter_map(Value::as_str)
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let masteries = preset
            .get("masteries")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing masteries for preset {}", champion))?
            .iter()
            .filter_map(|m| {
                let name = m.get("name").and_then(Value::as_str)?;
                let rank = m.get("rank").and_then(Value::as_u64).unwrap_or(1) as usize;
                Some(MasterySelection {
                    name: name.to_string(),
                    rank,
                })
            })
            .collect::<Vec<_>>();

        let source_url = preset
            .get("source_url")
            .and_then(Value::as_str)
            .unwrap_or(&default_source_url)
            .to_string();
        let last_checked = preset
            .get("last_checked")
            .and_then(Value::as_str)
            .unwrap_or(&default_last_checked)
            .to_string();

        let loaded = EnemyUrfPreset {
            champion: champion.clone(),
            source_url,
            last_checked,
            item_names,
            runes,
            shards,
            masteries,
        };
        by_champion.insert(to_norm_key(&champion), loaded);
    }
    Ok(by_champion)
}

fn validate_enemy_urf_presets(
    presets: &HashMap<String, EnemyUrfPreset>,
    items: &HashMap<String, Item>,
    loadout_domain: &LoadoutDomain,
) -> Result<()> {
    let all_runes = loadout_domain
        .rune_paths
        .iter()
        .flat_map(|p| p.slot_runes.iter())
        .flat_map(|slot| slot.iter())
        .map(|s| to_norm_key(s))
        .collect::<HashSet<_>>();
    let all_masteries = loadout_domain
        .mastery_trees
        .iter()
        .flat_map(|t| t.tiers.iter())
        .flat_map(|tier| tier.options.iter())
        .map(|m| to_norm_key(&m.name))
        .collect::<HashSet<_>>();
    for preset in presets.values() {
        if preset.item_names.len() != 6 {
            bail!(
                "Enemy preset for {} must contain exactly six full items",
                preset.champion
            );
        }
        if preset.runes.len() != 6 {
            bail!(
                "Enemy preset for {} must contain exactly six runes",
                preset.champion
            );
        }
        if preset.shards.len() != 3 {
            bail!(
                "Enemy preset for {} must contain exactly three shards",
                preset.champion
            );
        }
        for item_name in &preset.item_names {
            if !items.contains_key(item_name) {
                bail!(
                    "Enemy preset item '{}' for {} is not present in Items/",
                    item_name,
                    preset.champion
                );
            }
        }
        for rune_name in &preset.runes {
            if !all_runes.contains(&to_norm_key(rune_name)) {
                bail!(
                    "Enemy preset rune '{}' for {} is not present in RunesReforged",
                    rune_name,
                    preset.champion
                );
            }
        }
        for (idx, shard) in preset.shards.iter().enumerate() {
            let valid = loadout_domain
                .shard_slots
                .get(idx)
                .map(|slot| slot.iter().any(|s| to_norm_key(s) == to_norm_key(shard)))
                .unwrap_or(false);
            if !valid {
                bail!(
                    "Enemy preset shard '{}' in slot {} for {} is invalid",
                    shard,
                    idx + 1,
                    preset.champion
                );
            }
        }
        for mastery in &preset.masteries {
            if !all_masteries.contains(&to_norm_key(&mastery.name)) {
                bail!(
                    "Enemy preset mastery '{}' for {} is not present in Season2016",
                    mastery.name,
                    preset.champion
                );
            }
        }
    }
    Ok(())
}

fn enemy_loadout_from_preset(preset: &EnemyUrfPreset) -> LoadoutSelection {
    LoadoutSelection {
        rune_ids: Vec::new(),
        rune_names: preset.runes.clone(),
        shard_stats: preset.shards.clone(),
        masteries: preset.masteries.clone(),
    }
}

fn run_vladimir_scenario(
    scenario_path: &Path,
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
    report_path_override: Option<&str>,
    max_runtime_seconds: Option<f64>,
    status_every_seconds: f64,
    search_quality_profile: SearchQualityProfile,
) -> Result<()> {
    let run_start = Instant::now();
    let time_budget = max_runtime_seconds
        .filter(|s| *s > 0.0)
        .map(Duration::from_secs_f64);
    let deadline = time_budget.map(|d| run_start + d);
    let status_every = Duration::from_secs_f64(status_every_seconds.max(1.0));
    let mut status = StatusReporter::new(run_start, status_every);
    let timeout_flag = Arc::new(AtomicUsize::new(0));
    status.emit("initialization", None, None, Some("starting"), true);
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;
    status.emit("initialization", None, None, Some("core data loaded"), true);

    let sim = parse_simulation_config(
        scenario
            .get("simulation")
            .ok_or_else(|| anyhow!("Missing simulation"))?,
    )?;
    if deadline_reached(deadline) {
        timeout_flag.store(1, AtomicOrdering::Relaxed);
    }

    let vlad_base_raw =
        if let Some(champion) = scenario.get("vladimir_champion").and_then(Value::as_str) {
            lookup_champion_base(&champion_bases, champion)?
        } else {
            parse_champion_base(
                scenario
                    .get("vladimir_base")
                    .ok_or_else(|| anyhow!("Missing vladimir_champion/vladimir_base"))?,
            )?
        };
    let vlad_base = champion_at_level(&vlad_base_raw, sim.champion_level);

    let mut enemy_scenarios_raw: Vec<(String, f64, Vec<EnemyConfig>)> = Vec::new();
    if let Some(groups) = scenario.get("enemy_scenarios").and_then(Value::as_array) {
        for (idx, group) in groups.iter().enumerate() {
            let enemies_arr = group
                .get("enemies")
                .and_then(Value::as_array)
                .ok_or_else(|| anyhow!("enemy_scenarios[{}].enemies missing", idx))?;
            let enemies_cfg = enemies_arr
                .iter()
                .map(|e| parse_enemy_config(e, &champion_bases))
                .collect::<Result<Vec<_>>>()?;
            let name = group
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("scenario")
                .to_string();
            let weight = group.get("weight").and_then(Value::as_f64).unwrap_or(1.0);
            enemy_scenarios_raw.push((name, weight.max(0.0), enemies_cfg));
        }
    }
    if enemy_scenarios_raw.is_empty() {
        let enemies_raw = scenario
            .get("enemies")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("Missing enemies"))?
            .iter()
            .map(|e| parse_enemy_config(e, &champion_bases))
            .collect::<Result<Vec<_>>>()?;
        enemy_scenarios_raw.push(("default".to_string(), 1.0, enemies_raw));
    }
    let primary_enemy_raw = enemy_scenarios_raw
        .first()
        .map(|(_, _, v)| v.clone())
        .unwrap_or_default();
    let raw_enemy_bases = primary_enemy_raw
        .iter()
        .map(|e| (e.name.clone(), e.base.clone()))
        .collect::<HashMap<_, _>>();
    let enemy_scenarios = enemy_scenarios_raw
        .iter()
        .map(|(name, weight, enemies)| {
            let scaled = enemies
                .iter()
                .cloned()
                .map(|mut e| {
                    e.base = champion_at_level(&e.base, sim.champion_level);
                    e
                })
                .collect::<Vec<_>>();
            (name.clone(), *weight, scaled)
        })
        .collect::<Vec<_>>();

    let mut search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    apply_search_quality_profile(&mut search_cfg, search_quality_profile);
    let vlad_loadout_selection = parse_loadout_selection(scenario.get("vladimir_loadout"));
    let loadout_domain = Arc::new(build_loadout_domain());
    let loadout_eval_budget = loadout_eval_budget(&search_cfg, search_quality_profile);
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain)?;
    let max_items = search_cfg.max_items;
    let item_pool = default_item_pool(&items);
    status.emit(
        "configuration",
        None,
        None,
        Some("search profile and enemy presets ready"),
        true,
    );

    let baseline_fixed_names = scenario
        .get("vladimir_baseline_fixed")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing vladimir_baseline_fixed"))?
        .iter()
        .map(|v| v.as_str().ok_or_else(|| anyhow!("Invalid baseline item")))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let baseline_fixed_build = item_pool_from_names(&items, &baseline_fixed_names)?;

    let mut enemy_presets_used: HashMap<String, EnemyUrfPreset> = HashMap::new();
    let mut enemy_build_scenarios = Vec::new();
    for (name, weight, enemies) in &enemy_scenarios {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let mut builds = Vec::new();
        for enemy in enemies {
            let preset_key = to_norm_key(&enemy.name);
            let preset = enemy_presets.get(&preset_key).ok_or_else(|| {
                anyhow!(
                    "Missing URF preset for enemy champion '{}'. Add it to {}.",
                    enemy.name,
                    enemy_preset_data_path().display()
                )
            })?;
            let build_items = item_pool_from_names(&items, &preset.item_names)?;
            let resolved_enemy_loadout = resolve_loadout(
                &enemy_loadout_from_preset(preset),
                sim.champion_level,
                false,
            )?;
            enemy_presets_used.insert(preset_key, preset.clone());
            builds.push((
                enemy.clone(),
                build_items,
                resolved_enemy_loadout.bonus_stats,
            ));
        }
        enemy_build_scenarios.push((name.clone(), *weight, builds));
    }
    let enemy_builds = enemy_build_scenarios
        .first()
        .map(|(_, _, b)| b.clone())
        .unwrap_or_default();
    let enemy_loadout = ResolvedLoadout::default();
    status.emit(
        "enemy_setup",
        None,
        None,
        Some("enemy preset builds loaded"),
        true,
    );

    let vlad_base_loadout = resolve_loadout(&vlad_loadout_selection, sim.champion_level, true)?;
    let resolve_cache: Arc<Mutex<HashMap<String, ResolvedLoadout>>> =
        Arc::new(Mutex::new(HashMap::from([(
            loadout_selection_key(&vlad_loadout_selection),
            vlad_base_loadout.clone(),
        )])));
    let best_loadout_by_item: Arc<Mutex<HashMap<Vec<usize>, (LoadoutSelection, ResolvedLoadout)>>> =
        Arc::new(Mutex::new(HashMap::new()));
    let best_outcome_by_item: Arc<Mutex<HashMap<Vec<usize>, CombatOutcome>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let objective_worst_case_weight = search_cfg.multi_scenario_worst_weight.clamp(0.0, 1.0);
    let objective_component_weights = normalized_objective_weights(
        search_cfg.objective_survival_weight,
        search_cfg.objective_damage_weight,
        search_cfg.objective_healing_weight,
    );
    let scenario_reference_outcomes = enemy_build_scenarios
        .iter()
        .map(|(_, _, enemy_builds_s)| {
            simulate_vlad_combat(
                &vlad_base,
                &baseline_fixed_build,
                &vlad_base_loadout.bonus_stats,
                None,
                enemy_builds_s,
                &sim,
                &urf,
            )
        })
        .collect::<Vec<_>>();
    let evaluate_build_with_bonus = |build_items: &[Item], bonus_stats: &Stats| {
        aggregate_objective_score_and_outcome(
            &vlad_base,
            build_items,
            bonus_stats,
            &enemy_build_scenarios,
            &sim,
            &urf,
            &scenario_reference_outcomes,
            objective_component_weights,
            objective_worst_case_weight,
        )
    };
    let score_build_with_bonus = |build_items: &[Item], bonus_stats: &Stats| {
        evaluate_build_with_bonus(build_items, bonus_stats).0
    };

    let loadout_candidates_count = loadout_eval_budget;
    let loadout_finalists_count = 1usize;
    let optimize_loadout_for_build = |build_key: &[usize], build_items: &[Item]| {
        let mut hasher = DefaultHasher::new();
        build_key.hash(&mut hasher);
        let mut seed = search_cfg.seed ^ hasher.finish();
        let mut seen = HashSet::new();

        let mut best_sel = vlad_loadout_selection.clone();
        let mut best_resolved = vlad_base_loadout.clone();
        let (mut best_score, mut best_outcome) =
            evaluate_build_with_bonus(build_items, &best_resolved.bonus_stats);
        seen.insert(loadout_selection_key(&best_sel));

        let mut evaluated = 0usize;
        while evaluated < loadout_eval_budget {
            if deadline_reached(deadline) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                break;
            }
            let candidate =
                random_loadout_selection(&vlad_loadout_selection, &loadout_domain, &mut seed);
            let key = loadout_selection_key(&candidate);
            if !seen.insert(key.clone()) {
                continue;
            }

            let resolved = if let Ok(map) = resolve_cache.lock() {
                map.get(&key).cloned()
            } else {
                None
            }
            .or_else(|| {
                resolve_loadout(&candidate, sim.champion_level, true)
                    .ok()
                    .inspect(|resolved| {
                        if let Ok(mut map) = resolve_cache.lock() {
                            map.insert(key.clone(), resolved.clone());
                        }
                    })
            });

            let Some(resolved) = resolved else {
                continue;
            };
            let (score, outcome) = evaluate_build_with_bonus(build_items, &resolved.bonus_stats);
            if score > best_score {
                best_score = score;
                best_sel = candidate;
                best_resolved = resolved;
                best_outcome = outcome;
            }
            evaluated += 1;
        }
        (best_score, best_outcome, best_sel, best_resolved)
    };

    let full_eval_count = AtomicUsize::new(0);
    let full_cache = Arc::new(BlockingScoreCache::new());
    let mut scenario_hasher = DefaultHasher::new();
    scenario.to_string().hash(&mut scenario_hasher);
    search_strategy_summary(&search_cfg).hash(&mut scenario_hasher);
    search_cfg.seed.hash(&mut scenario_hasher);
    loadout_eval_budget.hash(&mut scenario_hasher);
    let persistent_full_cache_path = simulation_dir().join("output").join("cache").join(format!(
        "vladimir_full_scores_{:016x}.json",
        scenario_hasher.finish()
    ));
    let persistent_full_cache = Arc::new(PersistentScoreCache::load(persistent_full_cache_path));
    let full_score_fn = |build_idx: &[usize]| {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            return f64::NEG_INFINITY;
        }
        let key = canonical_key(build_idx);
        if let Some(score) = persistent_full_cache.get(&key) {
            return score;
        }
        let cache = Arc::clone(&full_cache);
        cache.get_or_compute(key.clone(), || {
            if deadline_reached(deadline) {
                timeout_flag.store(1, AtomicOrdering::Relaxed);
                return f64::NEG_INFINITY;
            }
            if let Some(score) = persistent_full_cache.get(&key) {
                return score;
            }
            full_eval_count.fetch_add(1, AtomicOrdering::Relaxed);
            let build_items = build_from_indices(&item_pool, &key);
            let (score, outcome, best_sel, best_resolved) =
                optimize_loadout_for_build(&key, &build_items);
            if let Ok(mut map) = best_loadout_by_item.lock() {
                map.insert(key.clone(), (best_sel, best_resolved));
            }
            if let Ok(mut map) = best_outcome_by_item.lock() {
                map.insert(key.clone(), outcome);
            }
            if score.is_finite() {
                persistent_full_cache.insert(&key, score);
            }
            score
        })
    };

    let ensemble_seeds = search_cfg.ensemble_seeds.max(1);
    let active_strategies = portfolio_strategy_list(&search_cfg);
    status.emit(
        "seed_search",
        Some((0, ensemble_seeds)),
        None,
        Some("running ensemble seeds"),
        true,
    );
    let mut seed_ranked = Vec::new();
    for seed_idx in 0..ensemble_seeds {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        seed_ranked.push({
            let mut cfg = search_cfg.clone();
            cfg.seed = search_cfg.seed.wrapping_add(
                search_cfg
                    .ensemble_seed_stride
                    .wrapping_mul(seed_idx as u64),
            );
            cfg.ranked_limit = cfg.ranked_limit.max(64);
            build_search_ranked(&item_pool, max_items, &cfg, &full_score_fn, deadline)
        });
        status.emit(
            "seed_search",
            Some((seed_idx + 1, ensemble_seeds)),
            None,
            None,
            false,
        );
    }
    let strategy_elites = strategy_seed_elites(
        &item_pool,
        max_items,
        &search_cfg,
        &active_strategies,
        &full_score_fn,
        deadline,
    );
    let adaptive_candidates = adaptive_strategy_candidates(
        &item_pool,
        max_items,
        &search_cfg,
        &strategy_elites,
        &full_score_fn,
        deadline,
    );
    let bleed_candidates =
        generate_bleed_candidates(&item_pool, max_items, &strategy_elites, &search_cfg);
    status.emit(
        "candidate_merge",
        None,
        None,
        Some("merging strict candidates"),
        true,
    );
    let bleed_candidate_count = bleed_candidates.len();
    let adaptive_candidate_count = adaptive_candidates.len();

    let mut candidate_keys = Vec::new();
    let mut seed_top_sets = Vec::new();
    for ranked in &seed_ranked {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let seed_top = ranked
            .iter()
            .take(search_cfg.ensemble_seed_top_k.max(1))
            .map(|(k, _)| k.clone())
            .collect::<HashSet<_>>();
        seed_top_sets.push(seed_top);
        for (k, _) in ranked {
            candidate_keys.push(k.clone());
        }
    }
    for k in bleed_candidates {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        candidate_keys.push(k);
    }
    for k in adaptive_candidates {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        candidate_keys.push(k);
    }
    let mut unique_candidate_keys = candidate_keys
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    unique_candidate_keys.sort_unstable();
    if unique_candidate_keys.is_empty() {
        let baseline_key = canonical_key(
            &baseline_fixed_build
                .iter()
                .filter_map(|item| item_pool.iter().position(|p| p.name == item.name))
                .collect::<Vec<_>>(),
        );
        unique_candidate_keys.push(baseline_key);
    }

    let mut strict_scores = HashMap::<Vec<usize>, f64>::new();
    for ranked in &seed_ranked {
        for (k, s) in ranked {
            if !s.is_finite() {
                continue;
            }
            let entry = strict_scores.entry(k.clone()).or_insert(*s);
            if *s > *entry {
                *entry = *s;
            }
        }
    }

    let total_candidates = unique_candidate_keys.len();
    let mut processed_keys = strict_scores.keys().cloned().collect::<HashSet<_>>();
    let mut processed_candidates = processed_keys.len().min(total_candidates);
    let mut timed_out = timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    status.emit(
        "strict_full_ranking",
        Some((processed_candidates, total_candidates)),
        strict_scores
            .values()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
        Some("evaluating all generated candidates"),
        true,
    );
    let remaining_keys = unique_candidate_keys
        .iter()
        .filter(|key| !processed_keys.contains(*key))
        .cloned()
        .collect::<Vec<_>>();
    let batch_size = 128usize;
    for batch in remaining_keys.chunks(batch_size) {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            timed_out = true;
            break;
        }
        let scored_batch = batch
            .par_iter()
            .map(|key| (key.clone(), full_score_fn(key)))
            .collect::<Vec<_>>();
        for (key, score) in scored_batch {
            if score.is_finite() {
                strict_scores.insert(key.clone(), score);
            }
            processed_keys.insert(key);
            processed_candidates += 1;
            status.emit(
                "strict_full_ranking",
                Some((processed_candidates, total_candidates)),
                strict_scores
                    .values()
                    .copied()
                    .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)),
                None,
                false,
            );
        }
    }

    if strict_scores.is_empty() {
        let baseline_key = canonical_key(
            &baseline_fixed_build
                .iter()
                .filter_map(|item| item_pool.iter().position(|p| p.name == item.name))
                .collect::<Vec<_>>(),
        );
        let baseline_score =
            score_build_with_bonus(&baseline_fixed_build, &vlad_base_loadout.bonus_stats);
        strict_scores.insert(baseline_key, baseline_score);
    }

    let mut vlad_ranked = strict_scores.into_iter().collect::<Vec<_>>();
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;
    let outcome_map_for_tiebreak = best_outcome_by_item
        .lock()
        .map(|m| m.clone())
        .unwrap_or_default();
    vlad_ranked.sort_by(|a, b| {
        let by_score = b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal);
        if by_score != Ordering::Equal {
            return by_score;
        }
        let out_a = outcome_map_for_tiebreak.get(&a.0);
        let out_b = outcome_map_for_tiebreak.get(&b.0);
        let cap_a = out_a
            .map(|o| o.time_alive_seconds >= sim.max_time_seconds - 1e-6)
            .unwrap_or(false);
        let cap_b = out_b
            .map(|o| o.time_alive_seconds >= sim.max_time_seconds - 1e-6)
            .unwrap_or(false);
        if cap_a && cap_b {
            let combo_a = out_a
                .map(|o| {
                    objective_component_weights.damage * o.damage_dealt
                        + objective_component_weights.healing * o.healing_done
                })
                .unwrap_or(0.0);
            let combo_b = out_b
                .map(|o| {
                    objective_component_weights.damage * o.damage_dealt
                        + objective_component_weights.healing * o.healing_done
                })
                .unwrap_or(0.0);
            return combo_b.partial_cmp(&combo_a).unwrap_or(Ordering::Equal);
        }
        Ordering::Equal
    });

    let mut seed_best_scores = Vec::new();
    for ranked in &seed_ranked {
        if deadline_reached(deadline) {
            timeout_flag.store(1, AtomicOrdering::Relaxed);
            break;
        }
        let best = ranked
            .iter()
            .take(search_cfg.ensemble_seed_top_k.max(1))
            .map(|(_, s)| *s)
            .fold(f64::NEG_INFINITY, |acc, v| acc.max(v));
        if best.is_finite() {
            seed_best_scores.push(best);
        }
    }

    let mut seed_hits_by_key: HashMap<Vec<usize>, usize> = HashMap::new();
    for top in &seed_top_sets {
        for key in top {
            *seed_hits_by_key.entry(key.clone()).or_insert(0) += 1;
        }
    }

    let vlad_best_indices = vlad_ranked
        .first()
        .map(|(build, _)| build.clone())
        .unwrap_or_default();
    let vlad_best_build = build_from_indices(&item_pool, &vlad_best_indices);
    let vlad_loadout = best_loadout_by_item
        .lock()
        .ok()
        .and_then(|m| m.get(&vlad_best_indices).cloned())
        .map(|(_, resolved)| resolved)
        .unwrap_or_else(|| vlad_base_loadout.clone());

    let baseline_fixed_indices = baseline_fixed_build
        .iter()
        .filter_map(|item| item_pool.iter().position(|p| p.name == item.name))
        .collect::<Vec<_>>();
    let baseline_fixed_score = if deadline_reached(deadline) {
        score_build_with_bonus(&baseline_fixed_build, &vlad_base_loadout.bonus_stats)
    } else {
        full_score_fn(&baseline_fixed_indices)
    };
    let baseline_fixed_key = canonical_key(&baseline_fixed_indices);
    let baseline_loadout = best_loadout_by_item
        .lock()
        .ok()
        .and_then(|m| m.get(&baseline_fixed_key).cloned())
        .map(|(_, resolved)| resolved)
        .unwrap_or_else(|| vlad_base_loadout.clone());
    let (_, baseline_fixed_outcome) = aggregate_objective_score_and_outcome(
        &vlad_base,
        &baseline_fixed_build,
        &baseline_loadout.bonus_stats,
        &enemy_build_scenarios,
        &sim,
        &urf,
        &scenario_reference_outcomes,
        objective_component_weights,
        objective_worst_case_weight,
    );
    let vlad_best_score = vlad_ranked.first().map(|(_, s)| *s).unwrap_or(0.0);
    let (_, vlad_best_outcome) = aggregate_objective_score_and_outcome(
        &vlad_base,
        &vlad_best_build,
        &vlad_loadout.bonus_stats,
        &enemy_build_scenarios,
        &sim,
        &urf,
        &scenario_reference_outcomes,
        objective_component_weights,
        objective_worst_case_weight,
    );
    let baseline_cap_survivor =
        baseline_fixed_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;
    let best_cap_survivor = vlad_best_outcome.time_alive_seconds >= sim.max_time_seconds - 1e-6;
    timed_out = timed_out || timeout_flag.load(AtomicOrdering::Relaxed) > 0;

    println!("Enemy builds (URF preset defaults):");
    for (enemy, build, _) in &enemy_builds {
        println!(
            "- {}: {}",
            enemy.name,
            build
                .iter()
                .map(|i| i.name.clone())
                .collect::<Vec<_>>()
                .join(", ")
        );
        if let Some(preset) = enemy_presets_used.get(&to_norm_key(&enemy.name)) {
            println!(
                "  source: {} (last checked {})",
                preset.source_url, preset.last_checked
            );
        }
    }

    println!("\nVladimir baseline build (fixed):");
    println!(
        "- Items: {}",
        baseline_fixed_build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Objective score: {:.4}", baseline_fixed_score);
    println!(
        "- Time alive / damage dealt / healing done / enemy kills: {:.2}s / {:.1} / {:.1} / {}",
        baseline_fixed_outcome.time_alive_seconds,
        baseline_fixed_outcome.damage_dealt,
        baseline_fixed_outcome.healing_done,
        baseline_fixed_outcome.enemy_kills
    );
    println!("- Cap survivor: {}", baseline_cap_survivor);

    println!("\nVladimir best build (optimized for objective):");
    println!(
        "- Search strategy: {}",
        search_strategy_summary(&search_cfg)
    );
    println!(
        "- Loadout candidates/finalists: {}/{}",
        loadout_candidates_count, loadout_finalists_count
    );
    println!(
        "- Candidate evaluations (full): {}",
        full_eval_count.load(AtomicOrdering::Relaxed)
    );
    println!("- Cache hits (full): {}", full_cache.hits());
    println!(
        "- Persistent full cache hits/entries: {}/{}",
        persistent_full_cache.hits(),
        persistent_full_cache.len()
    );
    println!("- Cache waits (full): {}", full_cache.waits());
    println!("- Ensemble seeds: {}", ensemble_seeds);
    println!(
        "- Enemy scenarios in objective: {}",
        enemy_build_scenarios.len()
    );
    println!(
        "- Objective weights (survival/damage/healing): {:.2}/{:.2}/{:.2}",
        objective_component_weights.survival,
        objective_component_weights.damage,
        objective_component_weights.healing
    );
    if let Some(budget) = time_budget {
        println!(
            "- Time budget: {:.1}s | elapsed: {:.1}s | timed_out: {} | progress: {}/{}",
            budget.as_secs_f64(),
            run_start.elapsed().as_secs_f64(),
            timed_out,
            processed_candidates,
            total_candidates
        );
    }
    println!(
        "- Unique strict candidates: {}",
        unique_candidate_keys.len()
    );
    println!("- Bleed candidates injected: {}", bleed_candidate_count);
    println!(
        "- Adaptive candidates injected: {}",
        adaptive_candidate_count
    );
    println!(
        "- Items: {}",
        vlad_best_build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Objective score: {:.4}", vlad_best_score);
    println!(
        "- Time alive / damage dealt / healing done / enemy kills: {:.2}s / {:.1} / {:.1} / {}",
        vlad_best_outcome.time_alive_seconds,
        vlad_best_outcome.damage_dealt,
        vlad_best_outcome.healing_done,
        vlad_best_outcome.enemy_kills
    );
    println!("- Cap survivor: {}", best_cap_survivor);
    if !vlad_loadout.selection_labels.is_empty() {
        println!("\nVladimir runes/masteries:");
        for s in &vlad_loadout.selection_labels {
            println!("- {}", s);
        }
    }
    if !enemy_loadout.selection_labels.is_empty() {
        println!("\nEnemy runes/masteries (applied to all enemies):");
        for s in &enemy_loadout.selection_labels {
            println!("- {}", s);
        }
    }

    let diverse_top_raw =
        select_diverse_top_builds(&vlad_ranked, top_x, min_item_diff, max_relative_gap_percent);
    let diverse_top_keys = diverse_top_raw
        .iter()
        .map(|(indices, _)| indices.clone())
        .collect::<Vec<_>>();
    let diverse_top_builds = diverse_top_raw
        .iter()
        .map(|(indices, score)| (build_from_indices(&item_pool, indices), *score))
        .collect::<Vec<_>>();
    let mut metrics_by_key = HashMap::new();
    for (key, score) in &vlad_ranked {
        metrics_by_key.insert(
            key.clone(),
            compute_build_metrics(
                key,
                &item_pool,
                &vlad_base,
                &vlad_loadout.bonus_stats,
                &sim,
                *score,
            ),
        );
    }
    let pareto_front = pareto_front_keys(&metrics_by_key);
    let build_confidence = vlad_ranked
        .iter()
        .map(|(key, _)| {
            let hits = seed_hits_by_key.get(key).copied().unwrap_or(0);
            let hit_rate = hits as f64 / ensemble_seeds as f64;
            let robustness = if hit_rate >= search_cfg.robust_min_seed_hit_rate {
                "robust".to_string()
            } else {
                "fragile".to_string()
            };
            BuildConfidence {
                key: key.clone(),
                seed_hits: hits,
                seed_hit_rate: hit_rate,
                robustness,
            }
        })
        .collect::<Vec<_>>();
    let diagnostics = SearchDiagnostics {
        strategy_summary: search_strategy_summary(&search_cfg),
        search_quality_profile: match search_quality_profile {
            SearchQualityProfile::Fast => "fast".to_string(),
            SearchQualityProfile::Balanced => "balanced".to_string(),
            SearchQualityProfile::MaximumQuality => "maximum_quality".to_string(),
        },
        ensemble_seeds,
        objective_survival_weight: objective_component_weights.survival,
        objective_damage_weight: objective_component_weights.damage,
        objective_healing_weight: objective_component_weights.healing,
        full_evaluations: full_eval_count.load(AtomicOrdering::Relaxed),
        full_cache_hits: full_cache.hits(),
        full_cache_misses: full_cache.misses(),
        full_cache_waits: full_cache.waits(),
        full_persistent_cache_hits: persistent_full_cache.hits(),
        full_persistent_cache_entries: persistent_full_cache.len(),
        unique_candidate_builds: unique_candidate_keys.len(),
        bleed_candidates_injected: bleed_candidate_count,
        adaptive_candidates_injected: adaptive_candidate_count,
        scenario_count: enemy_build_scenarios.len(),
        loadout_candidates: loadout_candidates_count,
        loadout_finalists: loadout_finalists_count,
        time_budget_seconds: time_budget.map(|d| d.as_secs_f64()),
        elapsed_seconds: run_start.elapsed().as_secs_f64(),
        timed_out,
        processed_candidates,
        total_candidates,
        seed_best_scores,
    };
    let confidence_by_key = build_confidence
        .iter()
        .map(|c| (c.key.clone(), c.clone()))
        .collect::<HashMap<_, _>>();
    let mut order_input = diverse_top_builds
        .iter()
        .enumerate()
        .filter_map(|(idx, (build, _))| {
            let key = diverse_top_keys.get(idx)?;
            let robust = confidence_by_key
                .get(key)
                .map(|c| c.robustness == "robust")
                .unwrap_or(false);
            let pareto = pareto_front.contains(key);
            if robust || pareto {
                Some(build.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    if order_input.is_empty() {
        order_input = diverse_top_builds
            .iter()
            .take(2)
            .map(|(b, _)| b.clone())
            .collect::<Vec<_>>();
    }
    let build_order_results = order_input
        .iter()
        .map(|build| {
            optimize_build_order(
                build,
                &vlad_base_raw,
                &vlad_loadout.bonus_stats,
                &enemy_builds,
                &raw_enemy_bases,
                &sim,
                &urf,
            )
        })
        .collect::<Vec<_>>();
    let best_order_acquired_map = build_order_results
        .first()
        .map(|br| acquisition_level_map(&br.ordered_items, &br.acquired_levels));

    let best_effective_item_stats = compute_effective_item_stats_for_build(
        &vlad_base,
        &vlad_best_build,
        &vlad_loadout.bonus_stats,
        &sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
    );
    let vlad_end_stats = compute_vlad_stats(&vlad_base, &best_effective_item_stats);
    let stack_notes = build_stack_notes(
        &vlad_best_build,
        &vlad_base,
        &sim,
        sim.champion_level,
        best_order_acquired_map.as_ref(),
    );

    println!("\nTop diverse builds:");
    if diverse_top_builds.is_empty() {
        println!(
            "- None found (try increasing --max-relative-gap-percent or lowering --min-item-diff)."
        );
    } else {
        for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
            println!(
                "- #{:02} score {:.4}: {}",
                idx + 1,
                score,
                item_names(build)
            );
        }
    }
    if !build_order_results.is_empty() {
        println!("\nBuild order optimization (levels spread from 5 to 20):");
        for (idx, br) in build_order_results.iter().enumerate() {
            println!(
                "- Build #{:02} best order (cumulative {:.2}): {}",
                idx + 1,
                br.cumulative_score,
                item_names(&br.ordered_items)
            );
            for (stage_idx, (lvl, surv)) in
                br.levels.iter().zip(br.stage_survival.iter()).enumerate()
            {
                println!(
                    "  - Stage {} @ level {} -> {:.2}s",
                    stage_idx + 1,
                    lvl,
                    surv
                );
            }
        }
    }

    let report_path = report_path_override
        .map(PathBuf::from)
        .unwrap_or_else(default_report_path);
    write_vladimir_report_markdown(
        &report_path,
        scenario_path,
        &sim,
        &vlad_base,
        &vlad_end_stats,
        &stack_notes,
        &vlad_loadout,
        &enemy_loadout,
        &baseline_fixed_build,
        baseline_fixed_score,
        &baseline_fixed_outcome,
        &vlad_best_build,
        vlad_best_score,
        &vlad_best_outcome,
        &enemy_builds,
        &enemy_presets_used,
        &diverse_top_builds,
        &diverse_top_keys,
        &build_confidence,
        &metrics_by_key,
        &pareto_front,
        &diagnostics,
        &build_order_results,
    )?;
    let json_report_path = report_path.with_extension("json");
    write_vladimir_report_json(
        &json_report_path,
        scenario_path,
        &sim,
        &baseline_fixed_build,
        baseline_fixed_score,
        &baseline_fixed_outcome,
        &vlad_best_build,
        vlad_best_score,
        &vlad_best_outcome,
        &vlad_loadout,
        &enemy_builds,
        &enemy_presets_used,
        &diverse_top_builds,
        &diagnostics,
        &build_order_results,
    )?;
    persistent_full_cache.flush()?;
    status.emit(
        "finalization",
        Some((processed_candidates, total_candidates)),
        Some(vlad_best_score),
        Some("reports and persistent cache written"),
        true,
    );
    println!("\nReport written: {}", report_path.display());
    println!("Structured report written: {}", json_report_path.display());

    Ok(())
}

fn run_vladimir_stepper(scenario_path: &Path, ticks: usize) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let sim_cfg = parse_simulation_config(
        scenario
            .get("simulation")
            .ok_or_else(|| anyhow!("Missing simulation"))?,
    )?;
    let vlad_base_raw =
        if let Some(champion) = scenario.get("vladimir_champion").and_then(Value::as_str) {
            lookup_champion_base(&champion_bases, champion)?
        } else {
            parse_champion_base(
                scenario
                    .get("vladimir_base")
                    .ok_or_else(|| anyhow!("Missing vladimir_champion/vladimir_base"))?,
            )?
        };
    let vlad_base = champion_at_level(&vlad_base_raw, sim_cfg.champion_level);

    let enemies_raw = scenario
        .get("enemies")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing enemies"))?
        .iter()
        .map(|e| parse_enemy_config(e, &champion_bases))
        .collect::<Result<Vec<_>>>()?;
    let enemies = enemies_raw
        .into_iter()
        .map(|mut e| {
            e.base = champion_at_level(&e.base, sim_cfg.champion_level);
            e
        })
        .collect::<Vec<_>>();

    let vlad_loadout_selection = parse_loadout_selection(scenario.get("vladimir_loadout"));
    let enemy_loadout_selection = parse_loadout_selection(scenario.get("enemy_loadout"));
    let vlad_loadout = resolve_loadout(&vlad_loadout_selection, sim_cfg.champion_level, true)?;
    let enemy_loadout = resolve_loadout(&enemy_loadout_selection, sim_cfg.champion_level, false)?;
    let loadout_domain = build_loadout_domain();
    let enemy_presets = load_enemy_urf_presets()?;
    validate_enemy_urf_presets(&enemy_presets, &items, &loadout_domain)?;

    let mut enemy_builds: Vec<(EnemyConfig, Vec<Item>, Stats)> = Vec::new();
    for enemy in &enemies {
        let key = to_norm_key(&enemy.name);
        let preset = enemy_presets.get(&key).ok_or_else(|| {
            anyhow!(
                "Missing URF preset for enemy champion '{}'. Add it to {}.",
                enemy.name,
                enemy_preset_data_path().display()
            )
        })?;
        let build = item_pool_from_names(&items, &preset.item_names)?;
        let mut bonus_stats = resolve_loadout(
            &enemy_loadout_from_preset(preset),
            sim_cfg.champion_level,
            false,
        )?
        .bonus_stats;
        bonus_stats.add(&enemy_loadout.bonus_stats);
        enemy_builds.push((enemy.clone(), build, bonus_stats));
    }

    let baseline_fixed_names = scenario
        .get("vladimir_baseline_fixed")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing vladimir_baseline_fixed"))?
        .iter()
        .map(|v| v.as_str().ok_or_else(|| anyhow!("Invalid baseline item")))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let baseline_fixed_build = item_pool_from_names(&items, &baseline_fixed_names)?;

    let mut sim = VladCombatSimulation::new(
        vlad_base,
        &baseline_fixed_build,
        &vlad_loadout.bonus_stats,
        None,
        &enemy_builds,
        sim_cfg.clone(),
        urf,
    );

    println!(
        "Server tick rate: {:.2} Hz ({:.5}s/tick)",
        sim_cfg.server_tick_rate_hz, sim.tick_seconds
    );

    for tick in 0..ticks.max(1) {
        let alive = sim.step(1);
        let status = if alive { "alive" } else { "finished" };
        println!(
            "tick={} time={:.3}s health={:.2} targetable={} can_cast={} status={}",
            tick + 1,
            sim.time,
            sim.health,
            sim.is_targetable(),
            sim.can_cast(),
            status
        );
        if !alive {
            break;
        }
    }
    Ok(())
}

fn run_stat_optimization(stat_key: &str, scenario_path: &Path, label: &str) -> Result<()> {
    let items = load_items()?;
    let scenario = load_json(scenario_path)?;
    let search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    let item_pool = default_item_pool(&items);

    let build_indices = choose_best_build_by_stat(
        &item_pool,
        stat_key,
        search_cfg.max_items,
        search_cfg.beam_width,
    );
    let build = build_from_indices(&item_pool, &build_indices);
    let stats = build_item_stats(&build);
    let value = stats.get_stat(stat_key);

    println!("Best build for {}:", label);
    println!(
        "- Items: {}",
        build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Total {}: {:.2}", label, value);
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let available = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let default_threads = available.saturating_sub(1).max(1);
    let threads = cli.threads.unwrap_or(default_threads).max(1);
    let _ = ThreadPoolBuilder::new().num_threads(threads).build_global();

    let scenario_path = PathBuf::from(cli.scenario);
    match cli.mode {
        Mode::Vladimir => run_vladimir_scenario(
            &scenario_path,
            cli.top_x,
            cli.min_item_diff,
            cli.max_relative_gap_percent,
            cli.report_path.as_deref(),
            cli.max_runtime_seconds,
            cli.status_every_seconds,
            cli.search_quality_profile,
        ),
        Mode::VladimirStep => run_vladimir_stepper(&scenario_path, cli.ticks),
        Mode::TaricAs => {
            run_stat_optimization("attack_speed_percent", &scenario_path, "attack speed")
        }
        Mode::HecarimMs => run_stat_optimization("move_speed_flat", &scenario_path, "move speed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loadout_selection_key_is_order_independent() {
        let a = LoadoutSelection {
            rune_ids: vec![],
            rune_names: vec!["Triumph".to_string(), "Lethal Tempo".to_string()],
            shard_stats: vec!["adaptive".to_string(), "health".to_string()],
            masteries: vec![
                MasterySelection {
                    name: "Fervor of Battle".to_string(),
                    rank: 1,
                },
                MasterySelection {
                    name: "Perseverance".to_string(),
                    rank: 2,
                },
            ],
        };
        let b = LoadoutSelection {
            rune_ids: vec![],
            rune_names: vec!["Lethal Tempo".to_string(), "Triumph".to_string()],
            shard_stats: vec!["health".to_string(), "adaptive".to_string()],
            masteries: vec![
                MasterySelection {
                    name: "Perseverance".to_string(),
                    rank: 2,
                },
                MasterySelection {
                    name: "Fervor of Battle".to_string(),
                    rank: 1,
                },
            ],
        };
        assert_eq!(loadout_selection_key(&a), loadout_selection_key(&b));
    }

    #[test]
    fn compute_vladimir_stats_does_not_recursively_reapply_conversions() {
        let base = ChampionBase {
            name: "Vladimir".to_string(),
            base_health: 1000.0,
            health_per_level: 0.0,
            base_armor: 30.0,
            armor_per_level: 0.0,
            base_magic_resist: 30.0,
            magic_resist_per_level: 0.0,
            base_attack_damage: 60.0,
            attack_damage_per_level: 0.0,
            base_attack_speed: 0.658,
            attack_speed_per_level_percent: 0.0,
            base_move_speed: 340.0,
            is_melee: false,
        };
        let item_stats = Stats {
            ability_power: 100.0,
            health: 200.0,
            ..Stats::default()
        };
        let out = compute_vlad_stats(&base, &item_stats);
        let expected_ap = 100.0 + 0.033 * 200.0;
        let expected_health = 1000.0 + 200.0 + 1.6 * 100.0;
        assert!((out.ability_power - expected_ap).abs() < 1e-9);
        assert!((out.health - expected_health).abs() < 1e-9);
    }

    #[test]
    fn enemy_preset_data_validates_against_local_data() {
        let presets = load_enemy_urf_presets().expect("enemy presets should load");
        let items = load_items().expect("items should load");
        let domain = build_loadout_domain();
        validate_enemy_urf_presets(&presets, &items, &domain)
            .expect("enemy preset validation should pass");
    }

    #[test]
    fn random_loadout_generation_produces_legal_shapes() {
        let domain = build_loadout_domain();
        assert!(domain.rune_paths.len() >= 2);
        assert!(domain.shard_slots.iter().all(|s| !s.is_empty()));
        assert!(domain.mastery_trees.len() >= 2);

        let base = LoadoutSelection::default();
        let mut seed = 1337u64;
        let mut produced_mastery_page = false;
        for _ in 0..64 {
            let sample = random_loadout_selection(&base, &domain, &mut seed);
            assert_eq!(sample.rune_names.len(), 6);
            assert_eq!(sample.shard_stats.len(), 3);
            if !sample.masteries.is_empty() {
                let points = sample.masteries.iter().map(|m| m.rank).sum::<usize>();
                assert_eq!(points, 30);
                produced_mastery_page = true;
                break;
            }
        }
        assert!(
            produced_mastery_page,
            "expected to produce at least one legal mastery page"
        );
    }

    #[test]
    fn objective_weights_and_scoring_are_normalized() {
        let w = normalized_objective_weights(0.55, 0.30, 0.15);
        assert!((w.survival + w.damage + w.healing - 1.0).abs() < 1e-9);

        let reference = CombatOutcome {
            time_alive_seconds: 20.0,
            damage_dealt: 8000.0,
            healing_done: 2000.0,
            enemy_kills: 0,
        };
        let baseline_score = objective_score_from_outcome(reference, reference, w);
        assert!((baseline_score - 1.0).abs() < 1e-9);

        let better = CombatOutcome {
            time_alive_seconds: 22.0,
            damage_dealt: 8800.0,
            healing_done: 2400.0,
            enemy_kills: 0,
        };
        assert!(objective_score_from_outcome(better, reference, w) > baseline_score);
    }

    #[test]
    fn urf_respawn_timer_scales_with_level() {
        let mut prev = 0.0;
        for lvl in 1..=30 {
            let t = respawn::urf_respawn_delay_seconds(lvl, 3.0, 2.5);
            assert!(t >= 1.0);
            assert!(t >= prev);
            prev = t;
        }
        assert!((respawn::urf_respawn_delay_seconds(1, 3.0, 2.5) - 7.0).abs() < 1e-9);
    }
}

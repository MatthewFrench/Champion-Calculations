use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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
    seed: u64,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventType {
    EnemyAttack(usize),
    EnemyAbility(usize),
    EnemyStun(usize),
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
    enemy_count: usize,
    sim: SimulationConfig,
    urf: UrfBuffs,

    tick_seconds: f64,
    time: f64,
    finished: bool,
    death_time: Option<f64>,

    event_queue: BinaryHeap<QueuedEvent>,
    event_counter: u64,

    vlad_stats: Stats,
    max_health: f64,
    health: f64,

    physical_multiplier: f64,
    magic_multiplier: f64,

    pool_cooldown: f64,
    pool_duration: f64,

    zhonya_available: bool,
    ga_available: bool,
    protoplasm_available: bool,

    ga_cooldown: f64,
    zhonya_cooldown: f64,
    protoplasm_cooldown: f64,

    zhonya_cd: f64,
    ga_cd: f64,
    pool_cd: f64,
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
            enemy_count: enemies.len(),
            sim,
            urf,
            tick_seconds,
            time: 0.0,
            finished: false,
            death_time: None,
            event_queue: BinaryHeap::new(),
            event_counter: 0,
            vlad_stats,
            max_health,
            health: max_health,
            physical_multiplier,
            magic_multiplier,
            pool_cooldown,
            pool_duration: 0.0,
            zhonya_available,
            ga_available,
            protoplasm_available,
            ga_cooldown,
            zhonya_cooldown,
            protoplasm_cooldown: 120.0,
            zhonya_cd: 0.0,
            ga_cd: 0.0,
            pool_cd: 0.0,
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

            runner.enemy_state.push(EnemyState {
                enemy: enemy.clone(),
                physical_hit_damage: attack_damage,
                ability_hit_damage,
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

    fn apply_hot_effects(&mut self, to_time: f64) {
        if to_time <= self.time {
            return;
        }
        let delta = to_time - self.time;
        if self.pool_heal_until > self.time {
            let active = delta.min(self.pool_heal_until - self.time);
            self.health = self
                .max_health
                .min(self.health + self.pool_heal_rate * active);
        }
        if self.protoplasm_hot_until > self.time {
            let active = delta.min(self.protoplasm_hot_until - self.time);
            self.health = self
                .max_health
                .min(self.health + self.protoplasm_hot_rate * active);
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
            let total_pool_damage = pool_damage * self.enemy_count as f64;
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
                let state = &self.enemy_state[idx];
                self.apply_damage(state.physical_hit_damage, 0.0, 0.0);
            }
            EventType::EnemyAbility(idx) => {
                let state = &self.enemy_state[idx];
                self.apply_damage(0.0, state.ability_hit_damage, 0.0);
            }
            EventType::EnemyStun(idx) => {
                let enemy = &self.enemy_state[idx].enemy;
                if self.is_targetable() {
                    self.stunned_until = self
                        .stunned_until
                        .max(self.time + enemy.stun_duration_seconds);
                }
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

    fn run_until_end(&mut self) -> f64 {
        while self.step(1) {}
        self.death_time
            .unwrap_or(self.time.min(self.sim.max_time_seconds))
    }
}

#[derive(Debug, Clone, Parser)]
#[command(about = "URF Vladimir survival simulator")]
struct Cli {
    #[arg(long)]
    scenario: String,
    #[arg(long, value_enum, default_value_t = Mode::Vlad)]
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
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    #[value(name = "vlad")]
    Vlad,
    #[value(name = "vlad_step")]
    VladStep,
    #[value(name = "taric_as")]
    TaricAs,
    #[value(name = "hecarim_ms")]
    HecarimMs,
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
    let runes_data = load_json(&PathBuf::from("Masteries/RunesReforged.json"))?;
    let masteries_data = load_json(&PathBuf::from("Masteries/Season2016.json"))?;

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
    })
}

fn parse_build_search(data: &Value) -> Result<BuildSearchConfig> {
    Ok(BuildSearchConfig {
        strategy: as_str(data, "strategy")?.to_string(),
        beam_width: data.get("beam_width").and_then(Value::as_u64).unwrap_or(20) as usize,
        max_items: data.get("max_items").and_then(Value::as_u64).unwrap_or(6) as usize,
        random_samples: data
            .get("random_samples")
            .and_then(Value::as_u64)
            .unwrap_or(200) as usize,
        seed: data.get("seed").and_then(Value::as_u64).unwrap_or(1337),
    })
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

fn simulate_vlad_survival(
    vlad_base: &ChampionBase,
    vlad_build_items: &[Item],
    vlad_bonus_stats: &Stats,
    vlad_item_acquired_levels: Option<&HashMap<String, usize>>,
    enemies: &[(EnemyConfig, Vec<Item>, Stats)],
    sim: &SimulationConfig,
    urf: &UrfBuffs,
) -> f64 {
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

fn score_candidates<F>(
    candidates: Vec<Vec<usize>>,
    score_fn: &F,
) -> Vec<(Vec<usize>, Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let unique_keys: HashSet<Vec<usize>> = candidates.iter().map(|c| canonical_key(c)).collect();
    let mut key_list = unique_keys.into_iter().collect::<Vec<_>>();
    key_list.sort_unstable();

    let score_pairs = key_list
        .par_iter()
        .map(|key| (key.clone(), score_fn(key)))
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
) -> Vec<(Vec<usize>, f64)>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    let mut candidates: Vec<Vec<usize>> = vec![vec![]];
    let mut final_scored: Vec<(Vec<usize>, Vec<usize>, f64)> = vec![];

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

        let scored = score_candidates(next_candidates, &score_fn);
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

fn generate_permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    fn permute<T: Clone>(arr: &mut Vec<T>, l: usize, out: &mut Vec<Vec<T>>) {
        if l == arr.len() {
            out.push(arr.clone());
            return;
        }
        for i in l..arr.len() {
            arr.swap(l, i);
            permute(arr, l + 1, out);
            arr.swap(l, i);
        }
    }
    let mut arr = items.to_vec();
    let mut out = Vec::new();
    permute(&mut arr, 0, &mut out);
    out
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
    let permutations = generate_permutations(build_items);
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
    for perm in permutations {
        let current = score_build_order(
            &perm,
            &levels,
            vlad_base_raw,
            vlad_bonus_stats,
            enemy_builds,
            raw_enemy_bases,
            sim,
            urf,
        );
        if current.cumulative_score > best.cumulative_score {
            best = current;
        }
    }
    best
}

fn default_report_path() -> PathBuf {
    simulation_dir().join("output").join("vlad_run_report.md")
}

fn write_vlad_report(
    report_path: &Path,
    scenario_path: &Path,
    sim: &SimulationConfig,
    vlad_base_level: &ChampionBase,
    vlad_end_stats: &Stats,
    stack_notes: &[String],
    vlad_loadout: &ResolvedLoadout,
    enemy_loadout: &ResolvedLoadout,
    baseline_build: &[Item],
    baseline_time: f64,
    best_build: &[Item],
    best_time: f64,
    enemy_builds: &[(EnemyConfig, Vec<Item>, Stats)],
    diverse_top_builds: &[(Vec<Item>, f64)],
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
    let improvement = if baseline_time > 0.0 {
        ((best_time - baseline_time) / baseline_time) * 100.0
    } else {
        0.0
    };

    let mut content = String::new();
    content.push_str("# Vladimir URF Run Report\n\n");
    content.push_str(&format!("- Generated (unix): `{}`\n", now));
    content.push_str(&format!("- Scenario: `{}`\n\n", scenario_path.display()));

    content.push_str("## Headline\n");
    content.push_str(&format!(
        "- Baseline time alive: **{:.2}s**\n- Best time alive: **{:.2}s**\n- Improvement: **{:+.2}%**\n\n",
        baseline_time, best_time, improvement
    ));

    content.push_str(&format!(
        "- Champion level assumption: **{}**\n\n",
        sim.champion_level
    ));

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

    content.push_str("## Enemy Builds (DPS-Optimized)\n");
    for (enemy, build, _) in enemy_builds {
        content.push_str(&format!("- {}: {}\n", enemy.name, item_names(build)));
    }
    content.push('\n');

    content.push_str("## Diverse Top Builds\n");
    if diverse_top_builds.is_empty() {
        content.push_str("- No diverse builds found under current thresholds.\n\n");
    } else {
        let best = diverse_top_builds[0].1;
        for (idx, (build, score)) in diverse_top_builds.iter().enumerate() {
            let delta = score - best;
            content.push_str(&format!(
                "{}. `{:.2}s` ({:+.2}s vs top): {}\n",
                idx + 1,
                score,
                delta,
                item_names(build)
            ));
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

fn build_search<F>(
    item_pool: &[Item],
    max_items: usize,
    search: &BuildSearchConfig,
    score_fn: F,
) -> Vec<usize>
where
    F: Fn(&[usize]) -> f64 + Sync,
{
    match search.strategy.as_str() {
        "greedy" => {
            let mut build = Vec::new();
            for _ in 0..max_items {
                let mut best: Option<usize> = None;
                let mut best_score = f64::NEG_INFINITY;
                for item_idx in 0..item_pool.len() {
                    if build.contains(&item_idx) {
                        continue;
                    }
                    if is_boots(&item_pool[item_idx])
                        && build.iter().any(|&i| is_boots(&item_pool[i]))
                    {
                        continue;
                    }
                    let mut candidate = build.clone();
                    candidate.push(item_idx);
                    let score = score_fn(&candidate);
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
            build
        }
        "beam" => beam_search_ranked(item_pool, max_items, search.beam_width, score_fn)
            .into_iter()
            .next()
            .map(|(build, _)| build)
            .unwrap_or_default(),
        "random" => {
            // Lightweight deterministic PRNG to avoid extra crate dependency.
            let mut seed = search.seed;
            let mut best_build = Vec::new();
            let mut best_score = f64::NEG_INFINITY;
            for _ in 0..search.random_samples {
                let mut indices: Vec<usize> = (0..item_pool.len()).collect();
                for i in (1..indices.len()).rev() {
                    seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                    let j = (seed as usize) % (i + 1);
                    indices.swap(i, j);
                }

                let mut build = Vec::new();
                for item_idx in indices {
                    if build.len() >= max_items {
                        break;
                    }
                    if is_boots(&item_pool[item_idx])
                        && build.iter().any(|&i| is_boots(&item_pool[i]))
                    {
                        continue;
                    }
                    build.push(item_idx);
                }

                let score = score_fn(&build);
                if score > best_score {
                    best_score = score;
                    best_build = build;
                }
            }
            best_build
        }
        _ => vec![],
    }
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

fn run_vlad_scenario(
    scenario_path: &Path,
    top_x: usize,
    min_item_diff: usize,
    max_relative_gap_percent: f64,
    report_path_override: Option<&str>,
) -> Result<()> {
    let items = load_items()?;
    let urf = load_urf_buffs()?;
    let champion_bases = load_champion_bases()?;
    let scenario = load_json(scenario_path)?;

    let sim = parse_simulation_config(
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
    let vlad_base = champion_at_level(&vlad_base_raw, sim.champion_level);

    let enemies_raw = scenario
        .get("enemies")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Missing enemies"))?
        .iter()
        .map(|e| parse_enemy_config(e, &champion_bases))
        .collect::<Result<Vec<_>>>()?;
    let raw_enemy_bases = enemies_raw
        .iter()
        .map(|e| (e.name.clone(), e.base.clone()))
        .collect::<HashMap<_, _>>();
    let enemies = enemies_raw
        .iter()
        .cloned()
        .map(|mut e| {
            e.base = champion_at_level(&e.base, sim.champion_level);
            e
        })
        .collect::<Vec<_>>();

    let search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    let vlad_loadout_selection = parse_loadout_selection(scenario.get("vladimir_loadout"));
    let enemy_loadout_selection = parse_loadout_selection(scenario.get("enemy_loadout"));
    let vlad_loadout = resolve_loadout(&vlad_loadout_selection, sim.champion_level, true)?;
    let enemy_loadout = resolve_loadout(&enemy_loadout_selection, sim.champion_level, false)?;
    let max_items = search_cfg.max_items;
    let item_pool = default_item_pool(&items);

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

    let enemy_builds: Vec<(EnemyConfig, Vec<Item>, Stats)> = enemies
        .par_iter()
        .map(|enemy| {
            let enemy_copy = enemy.clone();
            let build_indices = build_search(&item_pool, max_items, &search_cfg, |build_idx| {
                let build_items = build_from_indices(&item_pool, build_idx);
                let mut stats = build_item_stats(&build_items);
                stats.add(&enemy_loadout.bonus_stats);
                apply_item_assumptions(
                    &mut stats,
                    &enemy_copy.base,
                    &build_items,
                    &sim,
                    sim.champion_level,
                    None,
                );
                let (physical_dps, magic_dps) = compute_enemy_dps(&enemy_copy, &stats, &urf);
                physical_dps + magic_dps
            });
            (
                enemy.clone(),
                build_from_indices(&item_pool, &build_indices),
                enemy_loadout.bonus_stats.clone(),
            )
        })
        .collect();

    let vlad_ranked = if search_cfg.strategy == "beam" {
        beam_search_ranked(&item_pool, max_items, search_cfg.beam_width, |build_idx| {
            let build_items = build_from_indices(&item_pool, build_idx);
            simulate_vlad_survival(
                &vlad_base,
                &build_items,
                &vlad_loadout.bonus_stats,
                None,
                &enemy_builds,
                &sim,
                &urf,
            )
        })
    } else {
        let best = build_search(&item_pool, max_items, &search_cfg, |build_idx| {
            let build_items = build_from_indices(&item_pool, build_idx);
            simulate_vlad_survival(
                &vlad_base,
                &build_items,
                &vlad_loadout.bonus_stats,
                None,
                &enemy_builds,
                &sim,
                &urf,
            )
        });
        let best_items = build_from_indices(&item_pool, &best);
        let best_score = simulate_vlad_survival(
            &vlad_base,
            &best_items,
            &vlad_loadout.bonus_stats,
            None,
            &enemy_builds,
            &sim,
            &urf,
        );
        vec![(canonical_key(&best), best_score)]
    };
    let vlad_best_indices = vlad_ranked
        .first()
        .map(|(build, _)| build.clone())
        .unwrap_or_default();
    let vlad_best_build = build_from_indices(&item_pool, &vlad_best_indices);

    let baseline_fixed_time = simulate_vlad_survival(
        &vlad_base,
        &baseline_fixed_build,
        &vlad_loadout.bonus_stats,
        None,
        &enemy_builds,
        &sim,
        &urf,
    );
    let vlad_best_time = simulate_vlad_survival(
        &vlad_base,
        &vlad_best_build,
        &vlad_loadout.bonus_stats,
        None,
        &enemy_builds,
        &sim,
        &urf,
    );

    println!("Enemy builds (optimized for DPS):");
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
    println!("- Time alive: {:.2}s", baseline_fixed_time);

    println!("\nVladimir best build (optimized for survival):");
    println!(
        "- Items: {}",
        vlad_best_build
            .iter()
            .map(|i| i.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("- Time alive: {:.2}s", vlad_best_time);
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
    let diverse_top_builds = diverse_top_raw
        .iter()
        .map(|(indices, score)| (build_from_indices(&item_pool, indices), *score))
        .collect::<Vec<_>>();
    let build_order_results = diverse_top_builds
        .iter()
        .map(|(build, _)| {
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
            println!("- #{:02} {:.2}s: {}", idx + 1, score, item_names(build));
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
    write_vlad_report(
        &report_path,
        scenario_path,
        &sim,
        &vlad_base,
        &vlad_end_stats,
        &stack_notes,
        &vlad_loadout,
        &enemy_loadout,
        &baseline_fixed_build,
        baseline_fixed_time,
        &vlad_best_build,
        vlad_best_time,
        &enemy_builds,
        &diverse_top_builds,
        &build_order_results,
    )?;
    println!("\nReport written: {}", report_path.display());

    Ok(())
}

fn run_vlad_stepper(scenario_path: &Path, ticks: usize) -> Result<()> {
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

    let search_cfg = parse_build_search(
        scenario
            .get("search")
            .ok_or_else(|| anyhow!("Missing search"))?,
    )?;
    let vlad_loadout_selection = parse_loadout_selection(scenario.get("vladimir_loadout"));
    let enemy_loadout_selection = parse_loadout_selection(scenario.get("enemy_loadout"));
    let vlad_loadout = resolve_loadout(&vlad_loadout_selection, sim_cfg.champion_level, true)?;
    let enemy_loadout = resolve_loadout(&enemy_loadout_selection, sim_cfg.champion_level, false)?;
    let item_pool = default_item_pool(&items);

    let enemy_builds: Vec<(EnemyConfig, Vec<Item>, Stats)> = enemies
        .par_iter()
        .map(|enemy| {
            let enemy_copy = enemy.clone();
            let build_indices =
                build_search(&item_pool, search_cfg.max_items, &search_cfg, |build_idx| {
                    let build_items = build_from_indices(&item_pool, build_idx);
                    let mut stats = build_item_stats(&build_items);
                    stats.add(&enemy_loadout.bonus_stats);
                    apply_item_assumptions(
                        &mut stats,
                        &enemy_copy.base,
                        &build_items,
                        &sim_cfg,
                        sim_cfg.champion_level,
                        None,
                    );
                    let (physical_dps, magic_dps) = compute_enemy_dps(&enemy_copy, &stats, &urf);
                    physical_dps + magic_dps
                });
            (
                enemy.clone(),
                build_from_indices(&item_pool, &build_indices),
                enemy_loadout.bonus_stats.clone(),
            )
        })
        .collect();

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
        Mode::Vlad => run_vlad_scenario(
            &scenario_path,
            cli.top_x,
            cli.min_item_diff,
            cli.max_relative_gap_percent,
            cli.report_path.as_deref(),
        ),
        Mode::VladStep => run_vlad_stepper(&scenario_path, cli.ticks),
        Mode::TaricAs => {
            run_stat_optimization("attack_speed_percent", &scenario_path, "attack speed")
        }
        Mode::HecarimMs => run_stat_optimization("move_speed_flat", &scenario_path, "move speed"),
    }
}

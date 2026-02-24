use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub(crate) struct Stats {
    pub(crate) ability_power: f64,
    pub(crate) health: f64,
    pub(crate) armor: f64,
    pub(crate) magic_resist: f64,
    pub(crate) attack_damage: f64,
    pub(crate) attack_speed_percent: f64,
    pub(crate) ability_haste: f64,
    pub(crate) move_speed_flat: f64,
    pub(crate) move_speed_percent: f64,
    pub(crate) crit_chance_percent: f64,
    pub(crate) tenacity_percent: f64,
}

impl Stats {
    pub(crate) fn add(&mut self, other: &Stats) {
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
        self.tenacity_percent += other.tenacity_percent;
    }

    pub(crate) fn get_stat(&self, key: &str) -> f64 {
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
            "tenacity_percent" => self.tenacity_percent,
            _ => 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Item {
    pub(crate) name: String,
    pub(crate) stats: Stats,
    pub(crate) rank: Vec<String>,
    pub(crate) shop_purchasable: bool,
    pub(crate) total_cost: f64,
    pub(crate) passive_effects_text: Vec<String>,
    pub(crate) has_active_effect: bool,
    pub(crate) structured_effect_count: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct ChampionBase {
    pub(crate) name: String,
    pub(crate) base_health: f64,
    pub(crate) health_per_level: f64,
    pub(crate) base_armor: f64,
    pub(crate) armor_per_level: f64,
    pub(crate) base_magic_resist: f64,
    pub(crate) magic_resist_per_level: f64,
    pub(crate) base_attack_damage: f64,
    pub(crate) attack_damage_per_level: f64,
    pub(crate) base_attack_speed: f64,
    pub(crate) attack_speed_per_level_percent: f64,
    pub(crate) base_attack_range: f64,
    pub(crate) base_attack_projectile_speed: f64,
    pub(crate) base_move_speed: f64,
    pub(crate) is_melee: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum OpponentMovementMode {
    HoldPosition,
    #[default]
    MaintainCombatRange,
}

#[derive(Debug, Clone)]
pub(crate) struct EnemyConfig {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) level: usize,
    pub(crate) base: ChampionBase,
    pub(crate) spawn_position_xy: Option<(f64, f64)>,
    pub(crate) movement_mode: OpponentMovementMode,
    pub(crate) loadout_item_names: Vec<String>,
    pub(crate) loadout_rune_names: Vec<String>,
    pub(crate) loadout_shards: Vec<String>,
    pub(crate) stack_overrides: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub(crate) struct SimulationConfig {
    pub(crate) dt: f64,
    pub(crate) server_tick_rate_hz: f64,
    pub(crate) champion_level: usize,
    pub(crate) max_time_seconds: f64,
    pub(crate) combat_seed: Option<u64>,
    pub(crate) collect_rune_proc_telemetry: bool,
    pub(crate) controlled_champion_script:
        Option<crate::scripts::champions::ControlledChampionScriptHandle>,
    pub(crate) zhonya_duration_seconds: f64,
    pub(crate) zhonya_cooldown_seconds: f64,
    pub(crate) zhonya_trigger_health_percent: f64,
    pub(crate) ga_cooldown_seconds: f64,
    pub(crate) ga_revive_duration_seconds: f64,
    pub(crate) ga_revive_base_health_ratio: f64,
    pub(crate) protoplasm_trigger_health_percent: f64,
    pub(crate) protoplasm_bonus_health: f64,
    pub(crate) protoplasm_heal_total: f64,
    pub(crate) protoplasm_duration_seconds: f64,
    pub(crate) stack_overrides: HashMap<String, f64>,
    pub(crate) urf_respawn_flat_reduction_seconds: f64,
    pub(crate) urf_respawn_extrapolation_per_level: f64,
    pub(crate) urf_respawn_time_scaling_enabled: bool,
    pub(crate) urf_respawn_time_scaling_start_seconds: f64,
    pub(crate) urf_respawn_time_scaling_per_minute_seconds: f64,
    pub(crate) urf_respawn_time_scaling_cap_seconds: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct UrfBuffs {
    pub(crate) ability_haste: f64,
    pub(crate) item_haste: f64,
    pub(crate) health_cost_multiplier: f64,
    pub(crate) bonus_attack_speed_multiplier_melee: f64,
    pub(crate) bonus_attack_speed_multiplier_ranged: f64,
    pub(crate) allowed_item_keys: HashSet<String>,
}

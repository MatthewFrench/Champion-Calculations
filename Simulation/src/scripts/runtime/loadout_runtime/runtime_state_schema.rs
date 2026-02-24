use std::collections::HashMap;

use crate::defaults::LevelScalingRange;

use super::rune_proc_telemetry::{MODELED_RUNE_TELEMETRY_KEY_COUNT, RuneProcTelemetryTotals};

#[derive(Debug, Clone, Copy)]
pub(crate) struct OnHitEffectProfile {
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct PressTheAttackTargetState {
    pub(super) stacks: usize,
    pub(super) stack_expires_at: f64,
    pub(super) vulnerable_until: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct HitWindowTargetState {
    pub(super) stacks: usize,
    pub(super) expires_at: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutRuntimeState {
    pub(super) has_lethal_tempo: bool,
    pub(super) has_grasp: bool,
    pub(super) has_kraken: bool,
    pub(super) has_blade_of_the_ruined_king: bool,
    pub(super) has_heartsteel: bool,
    pub(super) has_liandry: bool,
    pub(super) has_luden: bool,
    pub(super) has_guinsoo: bool,
    pub(super) has_second_wind: bool,
    pub(super) has_press_the_attack: bool,
    pub(super) has_fleet_footwork: bool,
    pub(super) has_conqueror: bool,
    pub(super) has_aftershock: bool,
    pub(super) has_electrocute: bool,
    pub(super) has_first_strike: bool,
    pub(super) has_phase_rush: bool,
    pub(super) has_arcane_comet: bool,
    pub(super) has_summon_aery: bool,
    pub(super) has_hail_of_blades: bool,
    pub(super) has_dark_harvest: bool,
    pub(super) has_triumph: bool,
    pub(super) has_gathering_storm: bool,
    pub(super) owner_is_melee: bool,
    pub(super) rune_proc_telemetry_enabled: bool,

    pub attacks_landed: usize,
    pub lethal_tempo_stacks: usize,
    pub hail_of_blades_remaining_attacks: usize,
    pub guinsoo_stacks: usize,
    pub conqueror_stacks: usize,
    pub dark_harvest_souls: usize,
    pub grasp_cooldown_seconds: f64,
    pub heartsteel_cooldown_seconds: f64,
    pub luden_cooldown_seconds: f64,
    pub grasp_ready_at: f64,
    pub heartsteel_ready_at: f64,
    pub luden_ready_at: f64,
    pub conqueror_expires_at: f64,
    pub fleet_ready_at: f64,
    pub aftershock_ready_at: f64,
    pub aftershock_active_until: f64,
    pub electrocute_ready_at: f64,
    pub first_strike_ready_at: f64,
    pub first_strike_window_until: f64,
    pub phase_rush_ready_at: f64,
    pub phase_rush_active_until: f64,
    pub arcane_comet_ready_at: f64,
    pub summon_aery_ready_at: f64,
    pub hail_of_blades_ready_at: f64,
    pub hail_of_blades_expires_at: f64,
    pub dark_harvest_ready_at: f64,
    pub pending_fleet_heal: f64,
    pub(super) press_the_attack_targets: HashMap<usize, PressTheAttackTargetState>,
    pub(super) electrocute_targets: HashMap<usize, HitWindowTargetState>,
    pub(super) phase_rush_targets: HashMap<usize, HitWindowTargetState>,
    pub(super) rune_proc_telemetry_totals:
        [RuneProcTelemetryTotals; MODELED_RUNE_TELEMETRY_KEY_COUNT],
}

impl Default for LoadoutRuntimeState {
    fn default() -> Self {
        Self {
            has_lethal_tempo: false,
            has_grasp: false,
            has_kraken: false,
            has_blade_of_the_ruined_king: false,
            has_heartsteel: false,
            has_liandry: false,
            has_luden: false,
            has_guinsoo: false,
            has_second_wind: false,
            has_press_the_attack: false,
            has_fleet_footwork: false,
            has_conqueror: false,
            has_aftershock: false,
            has_electrocute: false,
            has_first_strike: false,
            has_phase_rush: false,
            has_arcane_comet: false,
            has_summon_aery: false,
            has_hail_of_blades: false,
            has_dark_harvest: false,
            has_triumph: false,
            has_gathering_storm: false,
            owner_is_melee: false,
            rune_proc_telemetry_enabled: true,
            attacks_landed: 0,
            lethal_tempo_stacks: 0,
            hail_of_blades_remaining_attacks: 0,
            guinsoo_stacks: 0,
            conqueror_stacks: 0,
            dark_harvest_souls: 0,
            grasp_cooldown_seconds: 4.0,
            heartsteel_cooldown_seconds: 0.0,
            luden_cooldown_seconds: 0.0,
            grasp_ready_at: 0.0,
            heartsteel_ready_at: 0.0,
            luden_ready_at: 0.0,
            conqueror_expires_at: 0.0,
            fleet_ready_at: 0.0,
            aftershock_ready_at: 0.0,
            aftershock_active_until: 0.0,
            electrocute_ready_at: 0.0,
            first_strike_ready_at: 0.0,
            first_strike_window_until: 0.0,
            phase_rush_ready_at: 0.0,
            phase_rush_active_until: 0.0,
            arcane_comet_ready_at: 0.0,
            summon_aery_ready_at: 0.0,
            hail_of_blades_ready_at: 0.0,
            hail_of_blades_expires_at: 0.0,
            dark_harvest_ready_at: 0.0,
            pending_fleet_heal: 0.0,
            press_the_attack_targets: HashMap::new(),
            electrocute_targets: HashMap::new(),
            phase_rush_targets: HashMap::new(),
            rune_proc_telemetry_totals: [RuneProcTelemetryTotals::default();
                MODELED_RUNE_TELEMETRY_KEY_COUNT],
        }
    }
}

fn level_scaled_value(level: usize, min: f64, max: f64) -> f64 {
    let clamped_level = level.clamp(1, 18);
    let t = (clamped_level as f64 - 1.0) / 17.0;
    min + (max - min) * t
}

pub(super) fn level_scaled_range_value(level: usize, range: LevelScalingRange) -> f64 {
    level_scaled_value(level, range.min, range.max)
}

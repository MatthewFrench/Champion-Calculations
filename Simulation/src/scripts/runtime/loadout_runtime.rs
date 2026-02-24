use crate::to_norm_key;
use crate::{
    defaults::{
        LevelScalingRange, heartsteel_colossal_consumption_cooldown_seconds_default,
        luden_echo_cooldown_seconds_default, rune_runtime_defaults,
    },
    scripts::runtime::stat_resolution::{
        CooldownMetricSource, RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
    },
};
use std::collections::HashMap;

mod combat_bonus_resolution;
mod rune_proc_telemetry;

pub(crate) type RuneProcTelemetryEntry = self::rune_proc_telemetry::RuneProcTelemetryEntry;
#[cfg(test)]
pub(crate) type RuneProcTelemetrySourceEntry =
    self::rune_proc_telemetry::RuneProcTelemetrySourceEntry;

#[cfg(test)]
use self::rune_proc_telemetry::{MODELED_RUNE_TELEMETRY_KEYS, rune_telemetry_index};

use self::combat_bonus_resolution::{
    calculate_ability_bonus_damage_impl, calculate_on_hit_bonus_damage_impl,
    decay_expired_conqueror_stacks,
};
use self::rune_proc_telemetry::{
    MODELED_RUNE_TELEMETRY_KEY_COUNT, RuneProcTelemetryTotals, RuneProcTriggerSource,
    build_rune_proc_telemetry_entries, record_rune_proc, record_rune_proc_attempt,
    record_rune_proc_eligibility,
};

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
struct PressTheAttackTargetState {
    stacks: usize,
    stack_expires_at: f64,
    vulnerable_until: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct HitWindowTargetState {
    stacks: usize,
    expires_at: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct LoadoutRuntimeState {
    has_lethal_tempo: bool,
    has_grasp: bool,
    has_kraken: bool,
    has_blade_of_the_ruined_king: bool,
    has_heartsteel: bool,
    has_liandry: bool,
    has_luden: bool,
    has_guinsoo: bool,
    has_second_wind: bool,
    has_press_the_attack: bool,
    has_fleet_footwork: bool,
    has_conqueror: bool,
    has_aftershock: bool,
    has_electrocute: bool,
    has_first_strike: bool,
    has_phase_rush: bool,
    has_arcane_comet: bool,
    has_summon_aery: bool,
    has_hail_of_blades: bool,
    has_dark_harvest: bool,
    has_triumph: bool,
    has_gathering_storm: bool,
    owner_is_melee: bool,
    rune_proc_telemetry_enabled: bool,

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
    press_the_attack_targets: HashMap<usize, PressTheAttackTargetState>,
    electrocute_targets: HashMap<usize, HitWindowTargetState>,
    phase_rush_targets: HashMap<usize, HitWindowTargetState>,
    rune_proc_telemetry_totals: [RuneProcTelemetryTotals; MODELED_RUNE_TELEMETRY_KEY_COUNT],
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

fn level_scaled_range_value(level: usize, range: LevelScalingRange) -> f64 {
    level_scaled_value(level, range.min, range.max)
}

#[cfg(test)]
pub(crate) fn build_loadout_runtime_state(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
) -> LoadoutRuntimeState {
    build_loadout_runtime_state_with_telemetry(
        item_names,
        rune_names,
        item_haste,
        owner_is_melee,
        true,
    )
}

pub(crate) fn build_loadout_runtime_state_with_telemetry(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
    rune_proc_telemetry_enabled: bool,
) -> LoadoutRuntimeState {
    let rune_defaults = rune_runtime_defaults();
    let mut runtime = LoadoutRuntimeState {
        owner_is_melee,
        rune_proc_telemetry_enabled,
        ..LoadoutRuntimeState::default()
    };
    let clamped_item_haste = item_haste.max(-99.0);
    let item_buff_state = RuntimeBuffState {
        item_haste: clamped_item_haste,
        ..RuntimeBuffState::default()
    };
    runtime.heartsteel_cooldown_seconds = resolve_stat(
        StatQuery::CooldownSeconds {
            base_seconds: heartsteel_colossal_consumption_cooldown_seconds_default(),
            source: CooldownMetricSource::Item,
        },
        item_buff_state,
    );
    runtime.luden_cooldown_seconds = resolve_stat(
        StatQuery::CooldownSeconds {
            base_seconds: luden_echo_cooldown_seconds_default(),
            source: CooldownMetricSource::Item,
        },
        item_buff_state,
    );
    runtime.grasp_cooldown_seconds = resolve_stat(
        StatQuery::CooldownSeconds {
            base_seconds: rune_defaults.grasp_of_the_undying.cooldown_seconds,
            source: CooldownMetricSource::Neutral,
        },
        RuntimeBuffState::default(),
    );

    for item in item_names {
        match to_norm_key(item).as_str() {
            "krakenslayer" => runtime.has_kraken = true,
            "bladeoftheruinedking" => runtime.has_blade_of_the_ruined_king = true,
            "heartsteel" => runtime.has_heartsteel = true,
            "liandrystorment" => runtime.has_liandry = true,
            "ludensecho" => runtime.has_luden = true,
            "guinsoosrageblade" => runtime.has_guinsoo = true,
            _ => {}
        }
    }

    for rune in rune_names {
        match to_norm_key(rune).as_str() {
            "lethaltempo" => runtime.has_lethal_tempo = true,
            "graspoftheundying" => runtime.has_grasp = true,
            "secondwind" => runtime.has_second_wind = true,
            "presstheattack" => runtime.has_press_the_attack = true,
            "fleetfootwork" => runtime.has_fleet_footwork = true,
            "conqueror" => runtime.has_conqueror = true,
            "aftershock" => runtime.has_aftershock = true,
            "electrocute" => runtime.has_electrocute = true,
            "firststrike" => runtime.has_first_strike = true,
            "phaserush" => runtime.has_phase_rush = true,
            "arcanecomet" => runtime.has_arcane_comet = true,
            "summonaery" => runtime.has_summon_aery = true,
            "hailofblades" => runtime.has_hail_of_blades = true,
            "darkharvest" => runtime.has_dark_harvest = true,
            "triumph" => runtime.has_triumph = true,
            "gatheringstorm" => runtime.has_gathering_storm = true,
            _ => {}
        }
    }

    runtime
}

pub(crate) fn loadout_attack_speed_multiplier(runtime: &LoadoutRuntimeState, now: f64) -> f64 {
    let rune_defaults = rune_runtime_defaults();
    let lethal_tempo_bonus = if runtime.has_lethal_tempo {
        rune_defaults.lethal_tempo.attack_speed_per_stack.max(0.0)
            * runtime.lethal_tempo_stacks as f64
    } else {
        0.0
    };
    let guinsoo_bonus = if runtime.has_guinsoo {
        0.02 * runtime.guinsoo_stacks as f64
    } else {
        0.0
    };
    let hail_of_blades_bonus = if runtime.has_hail_of_blades
        && runtime.hail_of_blades_remaining_attacks > 0
        && now <= runtime.hail_of_blades_expires_at
    {
        if runtime.owner_is_melee {
            rune_defaults
                .hail_of_blades
                .bonus_attack_speed_ratio_melee
                .max(0.0)
        } else {
            rune_defaults
                .hail_of_blades
                .bonus_attack_speed_ratio_ranged
                .max(0.0)
        }
    } else {
        0.0
    };
    1.0 + lethal_tempo_bonus + guinsoo_bonus + hail_of_blades_bonus
}

pub(crate) fn reset_transient_loadout_state(runtime: &mut LoadoutRuntimeState) {
    runtime.attacks_landed = 0;
    runtime.lethal_tempo_stacks = 0;
    runtime.hail_of_blades_remaining_attacks = 0;
    runtime.hail_of_blades_expires_at = 0.0;
    runtime.guinsoo_stacks = 0;
    runtime.conqueror_stacks = 0;
    runtime.conqueror_expires_at = 0.0;
    runtime.pending_fleet_heal = 0.0;
    runtime.press_the_attack_targets.clear();
    runtime.electrocute_targets.clear();
    runtime.phase_rush_targets.clear();
    runtime.rune_proc_telemetry_totals =
        [RuneProcTelemetryTotals::default(); MODELED_RUNE_TELEMETRY_KEY_COUNT];
    runtime.aftershock_active_until = 0.0;
    runtime.first_strike_window_until = 0.0;
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn calculate_on_hit_bonus_damage(
    profile: OnHitEffectProfile,
    runtime: &mut LoadoutRuntimeState,
    attack_damage: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64, f64) {
    calculate_on_hit_bonus_damage_impl(
        profile,
        runtime,
        attack_damage,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        target_current_health,
        target_max_health,
        attacker_max_health,
        now,
        target_id,
        attacker_level,
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn calculate_ability_bonus_damage(
    runtime: &mut LoadoutRuntimeState,
    ability_raw_damage: f64,
    ability_ap_ratio: f64,
    attacker_ability_power: f64,
    attacker_bonus_attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64) {
    calculate_ability_bonus_damage_impl(
        runtime,
        ability_raw_damage,
        ability_ap_ratio,
        attacker_ability_power,
        attacker_bonus_attack_damage,
        target_current_health,
        target_max_health,
        now,
        target_id,
        attacker_level,
    )
}

pub(crate) fn on_outgoing_damage_heal(
    runtime: &mut LoadoutRuntimeState,
    damage_dealt: f64,
    now: f64,
) -> f64 {
    let defaults = rune_runtime_defaults();
    decay_expired_conqueror_stacks(runtime, now);
    if runtime.has_fleet_footwork {
        record_rune_proc_attempt(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
    }
    if runtime.has_conqueror {
        record_rune_proc_attempt(runtime, "conqueror", RuneProcTriggerSource::OnHit);
    }
    let mut heal = runtime.pending_fleet_heal.max(0.0);
    runtime.pending_fleet_heal = 0.0;
    if heal > 0.0 {
        record_rune_proc_eligibility(runtime, "fleetfootwork", RuneProcTriggerSource::OnHit);
        record_rune_proc(
            runtime,
            "fleetfootwork",
            RuneProcTriggerSource::OnHit,
            0.0,
            heal,
        );
    }
    if runtime.has_conqueror
        && runtime.conqueror_stacks >= defaults.conqueror.max_stacks.max(1)
        && now <= runtime.conqueror_expires_at
        && damage_dealt > 0.0
    {
        record_rune_proc_eligibility(runtime, "conqueror", RuneProcTriggerSource::OnHit);
        let conqueror_heal_ratio = if runtime.owner_is_melee {
            defaults.conqueror.melee_heal_ratio.max(0.0)
        } else {
            defaults.conqueror.ranged_heal_ratio.max(0.0)
        };
        let conqueror_heal = damage_dealt.max(0.0) * conqueror_heal_ratio;
        heal += conqueror_heal;
        if conqueror_heal > 0.0 {
            record_rune_proc(
                runtime,
                "conqueror",
                RuneProcTriggerSource::OnHit,
                0.0,
                conqueror_heal,
            );
        }
    }
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: heal,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn on_enemy_kill_heal(runtime: &mut LoadoutRuntimeState, max_health: f64) -> f64 {
    let defaults = rune_runtime_defaults();
    if !runtime.has_triumph || max_health <= 0.0 {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "triumph", RuneProcTriggerSource::EnemyKill);
    let heal = defaults.triumph.heal_max_health_ratio.max(0.0) * max_health.max(0.0);
    if heal <= 0.0 {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "triumph", RuneProcTriggerSource::EnemyKill);
    record_rune_proc(
        runtime,
        "triumph",
        RuneProcTriggerSource::EnemyKill,
        0.0,
        heal,
    );
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: heal,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn trigger_immobilize_rune_damage(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    actor_bonus_health: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().aftershock;
    if !runtime.has_aftershock {
        return 0.0;
    }
    record_rune_proc_attempt(runtime, "aftershock", RuneProcTriggerSource::Immobilize);
    if now < runtime.aftershock_ready_at {
        return 0.0;
    }
    record_rune_proc_eligibility(runtime, "aftershock", RuneProcTriggerSource::Immobilize);
    runtime.aftershock_ready_at = now + defaults.cooldown_seconds.max(0.0);
    runtime.aftershock_active_until = now + defaults.active_duration_seconds.max(0.0);
    let shockwave_magic =
        level_scaled_range_value(actor_level, defaults.shockwave_magic_damage_by_level)
            + defaults.shockwave_bonus_health_ratio.max(0.0) * actor_bonus_health.max(0.0);
    record_rune_proc(
        runtime,
        "aftershock",
        RuneProcTriggerSource::Immobilize,
        shockwave_magic,
        0.0,
    );
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: shockwave_magic,
            source: ScalarMetricSource::OutgoingAbilityDamage,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn loadout_incoming_damage_multipliers(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    current_armor: f64,
    current_magic_resist: f64,
    bonus_armor: f64,
    bonus_magic_resist: f64,
) -> (f64, f64) {
    let defaults = &rune_runtime_defaults().aftershock;
    if !runtime.has_aftershock || now > runtime.aftershock_active_until {
        return (1.0, 1.0);
    }
    let cap = level_scaled_range_value(actor_level, defaults.resist_cap_by_level);
    let bonus_armor_gain =
        (defaults.resist_base + defaults.resist_bonus_ratio * bonus_armor.max(0.0)).min(cap);
    let bonus_magic_resist_gain =
        (defaults.resist_base + defaults.resist_bonus_ratio * bonus_magic_resist.max(0.0)).min(cap);

    let armor = current_armor.max(0.0);
    let magic_resist = current_magic_resist.max(0.0);
    let physical_multiplier_before = 100.0 / (100.0 + armor);
    let magic_multiplier_before = 100.0 / (100.0 + magic_resist);
    let physical_multiplier_after = 100.0 / (100.0 + armor + bonus_armor_gain.max(0.0));
    let magic_multiplier_after = 100.0 / (100.0 + magic_resist + bonus_magic_resist_gain.max(0.0));

    (
        (physical_multiplier_after / physical_multiplier_before).clamp(0.0, 1.0),
        (magic_multiplier_after / magic_multiplier_before).clamp(0.0, 1.0),
    )
}

pub(crate) fn loadout_movement_speed_multiplier(
    runtime: &LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
) -> f64 {
    if !runtime.has_phase_rush || now > runtime.phase_rush_active_until {
        return 1.0;
    }
    1.0 + level_scaled_range_value(
        actor_level,
        rune_runtime_defaults()
            .phase_rush
            .movement_speed_bonus_ratio_by_level,
    )
}

pub(crate) fn rune_proc_telemetry(runtime: &LoadoutRuntimeState) -> Vec<RuneProcTelemetryEntry> {
    build_rune_proc_telemetry_entries(runtime)
}

pub(crate) fn tick_loadout_regeneration(
    runtime: &LoadoutRuntimeState,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    let defaults = &rune_runtime_defaults().second_wind;
    if !runtime.has_second_wind || max_health <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let health_ratio = (current_health / max_health).clamp(0.0, 1.0);
    let base_regen = defaults.base_regen_max_health_ratio_per_second.max(0.0) * max_health * dt;
    let bonus = if health_ratio <= defaults.low_health_threshold_ratio {
        defaults
            .low_health_bonus_regen_max_health_ratio_per_second
            .max(0.0)
            * max_health
            * dt
    } else {
        0.0
    };
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: base_regen + bonus,
            source: ScalarMetricSource::Healing,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

fn cooldown_status(now: f64, ready_at: f64) -> String {
    let remaining = (ready_at - now).max(0.0);
    if remaining <= 1e-9 {
        "ready".to_string()
    } else {
        format!("{remaining:.2}s")
    }
}

pub(crate) fn describe_runtime_cooldowns(runtime: &LoadoutRuntimeState, now: f64) -> Vec<String> {
    let mut lines = Vec::new();

    if runtime.has_grasp {
        lines.push(format!(
            "Grasp of the Undying: {} (cooldown {:.2}s)",
            cooldown_status(now, runtime.grasp_ready_at),
            runtime.grasp_cooldown_seconds
        ));
    }
    if runtime.has_heartsteel {
        lines.push(format!(
            "Heartsteel Colossal Consumption: {} (cooldown {:.2}s)",
            cooldown_status(now, runtime.heartsteel_ready_at),
            runtime.heartsteel_cooldown_seconds
        ));
    }
    if runtime.has_luden {
        lines.push(format!(
            "Luden's Echo: {} (cooldown {:.2}s)",
            cooldown_status(now, runtime.luden_ready_at),
            runtime.luden_cooldown_seconds
        ));
    }
    if runtime.has_fleet_footwork {
        lines.push(format!(
            "Fleet Footwork: {}",
            cooldown_status(now, runtime.fleet_ready_at)
        ));
    }
    if runtime.has_aftershock {
        lines.push(format!(
            "Aftershock: {} (active {})",
            cooldown_status(now, runtime.aftershock_ready_at),
            if now <= runtime.aftershock_active_until {
                "yes"
            } else {
                "no"
            }
        ));
    }
    if runtime.has_electrocute {
        lines.push(format!(
            "Electrocute: {}",
            cooldown_status(now, runtime.electrocute_ready_at)
        ));
    }
    if runtime.has_first_strike {
        lines.push(format!(
            "First Strike: {} (window active {})",
            cooldown_status(now, runtime.first_strike_ready_at),
            if now <= runtime.first_strike_window_until {
                "yes"
            } else {
                "no"
            }
        ));
    }
    if runtime.has_phase_rush {
        lines.push(format!(
            "Phase Rush: {} (active {})",
            cooldown_status(now, runtime.phase_rush_ready_at),
            if now <= runtime.phase_rush_active_until {
                "yes"
            } else {
                "no"
            }
        ));
    }
    if runtime.has_arcane_comet {
        lines.push(format!(
            "Arcane Comet: {}",
            cooldown_status(now, runtime.arcane_comet_ready_at)
        ));
    }
    if runtime.has_summon_aery {
        lines.push(format!(
            "Summon Aery: {}",
            cooldown_status(now, runtime.summon_aery_ready_at)
        ));
    }
    if runtime.has_hail_of_blades {
        lines.push(format!(
            "Hail of Blades: {} (remaining attacks {})",
            cooldown_status(now, runtime.hail_of_blades_ready_at),
            runtime.hail_of_blades_remaining_attacks
        ));
    }
    if runtime.has_dark_harvest {
        lines.push(format!(
            "Dark Harvest: {}",
            cooldown_status(now, runtime.dark_harvest_ready_at)
        ));
    }

    if lines.is_empty() {
        lines.push("none".to_string());
    }
    lines
}

pub(crate) fn describe_runtime_stacks(runtime: &LoadoutRuntimeState) -> Vec<String> {
    let defaults = rune_runtime_defaults();
    let mut lines = Vec::new();
    if runtime.has_lethal_tempo {
        lines.push(format!(
            "Lethal Tempo stacks: {}/{}",
            runtime.lethal_tempo_stacks,
            defaults.lethal_tempo.max_stacks.max(1)
        ));
    }
    if runtime.has_guinsoo {
        lines.push(format!("Guinsoo stacks: {}/8", runtime.guinsoo_stacks));
    }
    if runtime.has_conqueror {
        lines.push(format!(
            "Conqueror stacks: {}/{}",
            runtime.conqueror_stacks,
            defaults.conqueror.max_stacks.max(1)
        ));
    }
    if runtime.has_hail_of_blades {
        lines.push(format!(
            "Hail of Blades empowered attacks remaining: {}",
            runtime.hail_of_blades_remaining_attacks
        ));
    }
    if runtime.has_dark_harvest {
        lines.push(format!(
            "Dark Harvest souls: {}",
            runtime.dark_harvest_souls
        ));
    }
    if runtime.has_press_the_attack {
        let vulnerable_targets = runtime
            .press_the_attack_targets
            .values()
            .filter(|state| state.vulnerable_until > 0.0)
            .count();
        lines.push(format!(
            "Press the Attack tracked targets: {}",
            vulnerable_targets
        ));
    }
    if runtime.has_electrocute {
        let primed_targets = runtime
            .electrocute_targets
            .values()
            .filter(|state| state.stacks > 0)
            .count();
        lines.push(format!("Electrocute primed targets: {}", primed_targets));
    }
    if runtime.has_phase_rush {
        let tracked_targets = runtime
            .phase_rush_targets
            .values()
            .filter(|state| state.stacks > 0)
            .count();
        lines.push(format!("Phase Rush tracked targets: {}", tracked_targets));
    }
    if runtime.has_kraken || runtime.has_blade_of_the_ruined_king {
        lines.push(format!("Attacks landed: {}", runtime.attacks_landed));
    }
    if lines.is_empty() {
        lines.push("none".to_string());
    }
    lines
}

#[cfg(test)]
#[path = "tests/loadout_runtime_tests.rs"]
mod tests;

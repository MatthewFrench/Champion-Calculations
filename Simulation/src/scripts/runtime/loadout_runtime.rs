use crate::to_norm_key;
use crate::{
    defaults::{
        heartsteel_colossal_consumption_cooldown_seconds_default,
        luden_echo_cooldown_seconds_default,
    },
    scripts::runtime::stat_resolution::{
        CooldownMetricSource, RuntimeBuffState, ScalarMetricSource, StatQuery, resolve_stat,
    },
};
use std::collections::HashMap;

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
    owner_is_melee: bool,

    pub attacks_landed: usize,
    pub lethal_tempo_stacks: usize,
    pub guinsoo_stacks: usize,
    pub conqueror_stacks: usize,
    pub grasp_cooldown_seconds: f64,
    pub heartsteel_cooldown_seconds: f64,
    pub luden_cooldown_seconds: f64,
    pub grasp_ready_at: f64,
    pub heartsteel_ready_at: f64,
    pub luden_ready_at: f64,
    pub conqueror_expires_at: f64,
    pub fleet_ready_at: f64,
    pub aftershock_ready_at: f64,
    pub pending_heal: f64,
    press_the_attack_targets: HashMap<usize, PressTheAttackTargetState>,
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
            owner_is_melee: false,
            attacks_landed: 0,
            lethal_tempo_stacks: 0,
            guinsoo_stacks: 0,
            conqueror_stacks: 0,
            grasp_cooldown_seconds: 4.0,
            heartsteel_cooldown_seconds: 0.0,
            luden_cooldown_seconds: 0.0,
            grasp_ready_at: 0.0,
            heartsteel_ready_at: 0.0,
            luden_ready_at: 0.0,
            conqueror_expires_at: 0.0,
            fleet_ready_at: 0.0,
            aftershock_ready_at: 0.0,
            pending_heal: 0.0,
            press_the_attack_targets: HashMap::new(),
        }
    }
}

fn level_scaled_value(level: usize, min: f64, max: f64) -> f64 {
    let clamped_level = level.clamp(1, 18);
    let t = (clamped_level as f64 - 1.0) / 17.0;
    min + (max - min) * t
}

fn decay_expired_conqueror_stacks(runtime: &mut LoadoutRuntimeState, now: f64) {
    if runtime.has_conqueror && now > runtime.conqueror_expires_at {
        runtime.conqueror_stacks = 0;
    }
}

fn add_conqueror_stacks(runtime: &mut LoadoutRuntimeState, stacks: usize, now: f64) {
    if !runtime.has_conqueror || stacks == 0 {
        return;
    }
    decay_expired_conqueror_stacks(runtime, now);
    runtime.conqueror_stacks = (runtime.conqueror_stacks + stacks).min(12);
    runtime.conqueror_expires_at = now + 5.0;
}

fn press_the_attack_damage_multiplier(
    runtime: &LoadoutRuntimeState,
    target_id: Option<usize>,
    now: f64,
) -> f64 {
    if !runtime.has_press_the_attack {
        return 0.0;
    }
    target_id
        .and_then(|idx| runtime.press_the_attack_targets.get(&idx))
        .filter(|state| now <= state.vulnerable_until)
        .map(|_| 0.08)
        .unwrap_or(0.0)
}

pub(crate) fn build_loadout_runtime_state(
    item_names: &[String],
    rune_names: &[String],
    item_haste: f64,
    owner_is_melee: bool,
) -> LoadoutRuntimeState {
    let mut runtime = LoadoutRuntimeState {
        owner_is_melee,
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
            base_seconds: 4.0,
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
            _ => {}
        }
    }

    runtime
}

pub(crate) fn loadout_attack_speed_multiplier(runtime: &LoadoutRuntimeState) -> f64 {
    let lethal_tempo_bonus = if runtime.has_lethal_tempo {
        0.04 * runtime.lethal_tempo_stacks as f64
    } else {
        0.0
    };
    let guinsoo_bonus = if runtime.has_guinsoo {
        0.02 * runtime.guinsoo_stacks as f64
    } else {
        0.0
    };
    1.0 + lethal_tempo_bonus + guinsoo_bonus
}

pub(crate) fn reset_transient_loadout_state(runtime: &mut LoadoutRuntimeState) {
    runtime.attacks_landed = 0;
    runtime.lethal_tempo_stacks = 0;
    runtime.guinsoo_stacks = 0;
    runtime.conqueror_stacks = 0;
    runtime.conqueror_expires_at = 0.0;
    runtime.pending_heal = 0.0;
    runtime.press_the_attack_targets.clear();
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn calculate_on_hit_bonus_damage(
    profile: OnHitEffectProfile,
    runtime: &mut LoadoutRuntimeState,
    attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64, f64) {
    runtime.attacks_landed += 1;
    if runtime.has_lethal_tempo {
        runtime.lethal_tempo_stacks = (runtime.lethal_tempo_stacks + 1).min(6);
    }
    if runtime.has_guinsoo {
        runtime.guinsoo_stacks = (runtime.guinsoo_stacks + 1).min(8);
    }

    let magic = profile.on_hit_magic_flat + profile.on_hit_magic_ad_ratio * attack_damage;
    let mut extra_physical = 0.0;
    let mut extra_magic = magic.max(0.0);
    let mut extra_true = 0.0;
    let pta_multiplier = press_the_attack_damage_multiplier(runtime, target_id, now);
    if pta_multiplier > 0.0 {
        extra_true += pta_multiplier * attack_damage.max(0.0);
    }

    if profile.periodic_true_hit_every > 0
        && runtime
            .attacks_landed
            .is_multiple_of(profile.periodic_true_hit_every)
    {
        extra_true += profile.periodic_true_hit_base
            + profile.periodic_true_hit_ad_ratio * attack_damage
            + profile.periodic_true_hit_target_max_health_ratio * target_max_health;
    }

    if runtime.has_blade_of_the_ruined_king {
        extra_physical += 0.06 * target_current_health.max(0.0);
    }

    if runtime.has_kraken && runtime.attacks_landed.is_multiple_of(3) {
        extra_true += 65.0 + 0.45 * attack_damage;
    }

    if runtime.has_grasp && now >= runtime.grasp_ready_at {
        extra_magic += 8.0 + 0.035 * target_max_health.max(0.0);
        runtime.grasp_ready_at = now + runtime.grasp_cooldown_seconds;
    }

    if runtime.has_heartsteel && now >= runtime.heartsteel_ready_at {
        extra_physical += 70.0 + 0.06 * attacker_max_health.max(0.0);
        runtime.heartsteel_ready_at = now + runtime.heartsteel_cooldown_seconds;
    }
    if runtime.has_fleet_footwork && now >= runtime.fleet_ready_at {
        runtime.pending_heal +=
            level_scaled_value(attacker_level, 10.0, 148.0) + 0.10 * attack_damage.max(0.0);
        runtime.fleet_ready_at = now + 6.0;
    }
    if runtime.has_press_the_attack
        && let Some(target_idx) = target_id
    {
        let state = runtime
            .press_the_attack_targets
            .entry(target_idx)
            .or_default();
        if now > state.stack_expires_at {
            state.stacks = 0;
        }
        state.stacks = (state.stacks + 1).min(3);
        state.stack_expires_at = now + 4.0;
        if state.stacks >= 3 {
            extra_magic += level_scaled_value(attacker_level, 40.0, 174.0);
            state.stacks = 0;
            state.vulnerable_until = now + 5.0;
        }
    }
    if runtime.has_conqueror {
        let basic_attack_stacks = if runtime.owner_is_melee { 2 } else { 1 };
        add_conqueror_stacks(runtime, basic_attack_stacks, now);
    }

    (
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_physical,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_magic,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_true,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
    )
}

pub(crate) fn calculate_ability_bonus_damage(
    runtime: &mut LoadoutRuntimeState,
    ability_raw_damage: f64,
    ability_ap_ratio: f64,
    target_max_health: f64,
    now: f64,
    target_id: Option<usize>,
    attacker_level: usize,
) -> (f64, f64) {
    decay_expired_conqueror_stacks(runtime, now);
    let mut extra_magic = 0.0;
    let mut extra_true = 0.0_f64;
    let pta_multiplier = press_the_attack_damage_multiplier(runtime, target_id, now);
    if pta_multiplier > 0.0 {
        extra_true += pta_multiplier * ability_raw_damage.max(0.0);
    }
    if runtime.has_conqueror {
        add_conqueror_stacks(runtime, 2, now);
        let adaptive_ability_power =
            level_scaled_value(attacker_level, 1.8, 4.26) * runtime.conqueror_stacks as f64;
        extra_magic += adaptive_ability_power * ability_ap_ratio.max(0.0);
    }

    if runtime.has_liandry {
        extra_magic += 0.04 * target_max_health.max(0.0);
    }

    if runtime.has_luden && now >= runtime.luden_ready_at {
        extra_magic += 90.0 + 0.10 * ability_raw_damage.max(0.0);
        runtime.luden_ready_at = now + runtime.luden_cooldown_seconds;
    }

    (
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_magic,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
        resolve_stat(
            StatQuery::ScalarAmount {
                base_amount: extra_true,
                source: ScalarMetricSource::OutgoingAbilityDamage,
                clamp_min_zero: true,
            },
            RuntimeBuffState::default(),
        ),
    )
}

pub(crate) fn on_outgoing_damage_heal(
    runtime: &mut LoadoutRuntimeState,
    damage_dealt: f64,
    now: f64,
) -> f64 {
    decay_expired_conqueror_stacks(runtime, now);
    let mut heal = runtime.pending_heal.max(0.0);
    runtime.pending_heal = 0.0;
    if runtime.has_conqueror
        && runtime.conqueror_stacks >= 12
        && now <= runtime.conqueror_expires_at
        && damage_dealt > 0.0
    {
        let conqueror_heal_ratio = if runtime.owner_is_melee { 0.08 } else { 0.05 };
        heal += damage_dealt.max(0.0) * conqueror_heal_ratio;
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

pub(crate) fn trigger_immobilize_rune_damage(
    runtime: &mut LoadoutRuntimeState,
    now: f64,
    actor_level: usize,
    actor_bonus_health: f64,
) -> f64 {
    if !runtime.has_aftershock || now < runtime.aftershock_ready_at {
        return 0.0;
    }
    runtime.aftershock_ready_at = now + 20.0;
    let shockwave_magic =
        level_scaled_value(actor_level, 25.0, 120.0) + 0.08 * actor_bonus_health.max(0.0);
    resolve_stat(
        StatQuery::ScalarAmount {
            base_amount: shockwave_magic,
            source: ScalarMetricSource::OutgoingAbilityDamage,
            clamp_min_zero: true,
        },
        RuntimeBuffState::default(),
    )
}

pub(crate) fn tick_loadout_regeneration(
    runtime: &LoadoutRuntimeState,
    current_health: f64,
    max_health: f64,
    dt: f64,
) -> f64 {
    if !runtime.has_second_wind || max_health <= 0.0 || dt <= 0.0 {
        return 0.0;
    }
    let health_ratio = (current_health / max_health).clamp(0.0, 1.0);
    let base_regen = 0.0015 * max_health * dt;
    let bonus = if health_ratio <= 0.35 {
        0.0030 * max_health * dt
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
            "Aftershock: {}",
            cooldown_status(now, runtime.aftershock_ready_at)
        ));
    }

    if lines.is_empty() {
        lines.push("none".to_string());
    }
    lines
}

pub(crate) fn describe_runtime_stacks(runtime: &LoadoutRuntimeState) -> Vec<String> {
    let mut lines = Vec::new();
    if runtime.has_lethal_tempo {
        lines.push(format!(
            "Lethal Tempo stacks: {}/6",
            runtime.lethal_tempo_stacks
        ));
    }
    if runtime.has_guinsoo {
        lines.push(format!("Guinsoo stacks: {}/8", runtime.guinsoo_stacks));
    }
    if runtime.has_conqueror {
        lines.push(format!("Conqueror stacks: {}/12", runtime.conqueror_stacks));
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

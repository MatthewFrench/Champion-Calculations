use super::LoadoutRuntimeState;
use super::rune_proc_telemetry::{MODELED_RUNE_TELEMETRY_KEY_COUNT, RuneProcTelemetryTotals};
use crate::defaults::{
    heartsteel_colossal_consumption_cooldown_seconds_default, luden_echo_cooldown_seconds_default,
    rune_runtime_defaults,
};
use crate::scripts::runtime::stat_resolution::{
    CooldownMetricSource, RuntimeBuffState, StatQuery, resolve_stat,
};
use crate::to_norm_key;

fn set_item_runtime_flag(runtime: &mut LoadoutRuntimeState, item_name: &str) {
    match to_norm_key(item_name).as_str() {
        "krakenslayer" => runtime.has_kraken = true,
        "bladeoftheruinedking" => runtime.has_blade_of_the_ruined_king = true,
        "heartsteel" => runtime.has_heartsteel = true,
        "liandrystorment" => runtime.has_liandry = true,
        "ludensecho" => runtime.has_luden = true,
        "guinsoosrageblade" => runtime.has_guinsoo = true,
        _ => {}
    }
}

fn set_rune_runtime_flag(runtime: &mut LoadoutRuntimeState, rune_name: &str) {
    match to_norm_key(rune_name).as_str() {
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

pub(super) fn build_loadout_runtime_state_with_telemetry_impl(
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

    for item_name in item_names {
        set_item_runtime_flag(&mut runtime, item_name);
    }
    for rune_name in rune_names {
        set_rune_runtime_flag(&mut runtime, rune_name);
    }

    runtime
}

pub(super) fn reset_transient_loadout_state_impl(runtime: &mut LoadoutRuntimeState) {
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

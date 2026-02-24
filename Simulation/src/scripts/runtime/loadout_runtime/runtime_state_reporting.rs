use super::LoadoutRuntimeState;
use crate::defaults::rune_runtime_defaults;

fn cooldown_status(now: f64, ready_at: f64) -> String {
    let remaining = (ready_at - now).max(0.0);
    if remaining <= 1e-9 {
        "ready".to_string()
    } else {
        format!("{remaining:.2}s")
    }
}

pub(super) fn describe_runtime_cooldowns_impl(
    runtime: &LoadoutRuntimeState,
    now: f64,
) -> Vec<String> {
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

pub(super) fn describe_runtime_stacks_impl(runtime: &LoadoutRuntimeState) -> Vec<String> {
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

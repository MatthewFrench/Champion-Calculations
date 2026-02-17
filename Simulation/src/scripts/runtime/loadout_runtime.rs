use crate::to_norm_key;

#[derive(Debug, Clone, Copy)]
pub(crate) struct OnHitEffectProfile {
    pub on_hit_magic_flat: f64,
    pub on_hit_magic_ad_ratio: f64,
    pub periodic_true_hit_every: usize,
    pub periodic_true_hit_base: f64,
    pub periodic_true_hit_ad_ratio: f64,
    pub periodic_true_hit_target_max_health_ratio: f64,
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

    pub attacks_landed: usize,
    pub lethal_tempo_stacks: usize,
    pub guinsoo_stacks: usize,
    pub grasp_ready_at: f64,
    pub heartsteel_ready_at: f64,
    pub luden_ready_at: f64,
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
            attacks_landed: 0,
            lethal_tempo_stacks: 0,
            guinsoo_stacks: 0,
            grasp_ready_at: 0.0,
            heartsteel_ready_at: 0.0,
            luden_ready_at: 0.0,
        }
    }
}

pub(crate) fn build_loadout_runtime_state(
    item_names: &[String],
    rune_names: &[String],
) -> LoadoutRuntimeState {
    let mut runtime = LoadoutRuntimeState::default();

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
}

pub(crate) fn calculate_on_hit_bonus_damage(
    profile: OnHitEffectProfile,
    runtime: &mut LoadoutRuntimeState,
    attack_damage: f64,
    target_current_health: f64,
    target_max_health: f64,
    attacker_max_health: f64,
    now: f64,
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
        runtime.grasp_ready_at = now + 4.0;
    }

    if runtime.has_heartsteel && now >= runtime.heartsteel_ready_at {
        extra_physical += 70.0 + 0.06 * attacker_max_health.max(0.0);
        runtime.heartsteel_ready_at = now + 5.0;
    }

    (
        extra_physical.max(0.0),
        extra_magic.max(0.0),
        extra_true.max(0.0),
    )
}

pub(crate) fn calculate_ability_bonus_damage(
    runtime: &mut LoadoutRuntimeState,
    ability_raw_damage: f64,
    target_max_health: f64,
    now: f64,
) -> (f64, f64) {
    let mut extra_magic = 0.0;
    let extra_true = 0.0_f64;

    if runtime.has_liandry {
        extra_magic += 0.04 * target_max_health.max(0.0);
    }

    if runtime.has_luden && now >= runtime.luden_ready_at {
        extra_magic += 90.0 + 0.10 * ability_raw_damage.max(0.0);
        runtime.luden_ready_at = now + 8.0;
    }

    (extra_magic.max(0.0), extra_true.max(0.0))
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
    base_regen + bonus
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_on_hit_profile() -> OnHitEffectProfile {
        OnHitEffectProfile {
            on_hit_magic_flat: 0.0,
            on_hit_magic_ad_ratio: 0.0,
            periodic_true_hit_every: 0,
            periodic_true_hit_base: 0.0,
            periodic_true_hit_ad_ratio: 0.0,
            periodic_true_hit_target_max_health_ratio: 0.0,
        }
    }

    #[test]
    fn builder_marks_item_and_rune_runtime_flags() {
        let runtime = build_loadout_runtime_state(
            &[
                "Kraken Slayer".to_string(),
                "Guinsoo's Rageblade".to_string(),
                "Heartsteel".to_string(),
            ],
            &[
                "Lethal Tempo".to_string(),
                "Grasp of the Undying".to_string(),
                "Second Wind".to_string(),
            ],
        );

        assert!(runtime.has_kraken);
        assert!(runtime.has_guinsoo);
        assert!(runtime.has_heartsteel);
        assert!(runtime.has_lethal_tempo);
        assert!(runtime.has_grasp);
        assert!(runtime.has_second_wind);
    }

    #[test]
    fn lethal_tempo_and_kraken_stack_runtime() {
        let mut runtime = build_loadout_runtime_state(
            &["Kraken Slayer".to_string()],
            &["Lethal Tempo".to_string()],
        );
        assert!(loadout_attack_speed_multiplier(&runtime) >= 1.0);

        let profile = default_on_hit_profile();
        let _ = calculate_on_hit_bonus_damage(
            profile,
            &mut runtime,
            180.0,
            2500.0,
            3000.0,
            1600.0,
            1.0,
        );
        let _ = calculate_on_hit_bonus_damage(
            profile,
            &mut runtime,
            180.0,
            2500.0,
            3000.0,
            1600.0,
            2.0,
        );
        let (_, _, true_damage) = calculate_on_hit_bonus_damage(
            profile,
            &mut runtime,
            180.0,
            2500.0,
            3000.0,
            1600.0,
            3.0,
        );
        assert!(runtime.lethal_tempo_stacks > 0);
        assert!(true_damage > 0.0);
    }

    #[test]
    fn clear_transient_state_resets_stack_counters() {
        let mut runtime = build_loadout_runtime_state(
            &[
                "Kraken Slayer".to_string(),
                "Guinsoo's Rageblade".to_string(),
            ],
            &["Lethal Tempo".to_string()],
        );
        runtime.attacks_landed = 7;
        runtime.lethal_tempo_stacks = 6;
        runtime.guinsoo_stacks = 8;
        reset_transient_loadout_state(&mut runtime);
        assert_eq!(runtime.attacks_landed, 0);
        assert_eq!(runtime.lethal_tempo_stacks, 0);
        assert_eq!(runtime.guinsoo_stacks, 0);
    }

    #[test]
    fn ability_bonus_damage_respects_luden_cooldown() {
        let mut runtime = build_loadout_runtime_state(&["Luden's Echo".to_string()], &[]);
        let (magic_a, true_a) = calculate_ability_bonus_damage(&mut runtime, 300.0, 2500.0, 0.0);
        let (magic_b, true_b) = calculate_ability_bonus_damage(&mut runtime, 300.0, 2500.0, 1.0);
        assert!(magic_a > magic_b);
        assert_eq!(true_a, true_b);
    }

    #[test]
    fn second_wind_regen_scales_when_low_health() {
        let runtime = build_loadout_runtime_state(&[], &["Second Wind".to_string()]);
        let high = tick_loadout_regeneration(&runtime, 2800.0, 3000.0, 1.0);
        let low = tick_loadout_regeneration(&runtime, 900.0, 3000.0, 1.0);
        assert!(high > 0.0);
        assert!(low > high);
    }
}

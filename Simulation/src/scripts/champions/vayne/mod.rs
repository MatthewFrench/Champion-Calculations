use super::{
    ChampionBehaviorProfile, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput, ChampionScriptSchedule,
};

pub(crate) const CHAMPION_KEY: &str = "vayne";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    profile.attack_range = 550.0;
    profile.attack_windup_seconds = 0.17;
    profile.attack_projectile_speed = 2500.0;
    profile.periodic_true_hit_every = 3;
    profile.periodic_true_hit_base = 40.0;
    profile.periodic_true_hit_ad_ratio = 0.25;
    profile.periodic_true_hit_target_max_health_ratio = 0.04;
    profile.ability_projectile_speed = 2200.0;
    profile.burst_projectile_speed = 2200.0;
    profile.desired_combat_range = 520.0;
    profile.movement_speed_scale = 1.10;
}

pub(crate) fn schedules() -> Vec<ChampionScriptSchedule> {
    vec![ChampionScriptSchedule {
        event: ChampionScriptEvent::VayneTumbleEmpower,
        start_offset_seconds: 2.2,
        interval_seconds: 5.0,
    }]
}

pub(crate) fn execute_tumble_empower(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    vec![ChampionScriptAction::AddNextAttackBonusPhysical {
        amount: 0.75 * input.physical_hit_damage,
        trace_message: "empowered next attack",
    }]
}

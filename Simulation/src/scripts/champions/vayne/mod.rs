use super::{
    ChampionBehaviorProfile, ChampionScriptAction, ChampionScriptEvent,
    ChampionScriptExecutionInput,
};
use crate::defaults::{vayne_silver_bolts_ability_defaults, vayne_tumble_ability_defaults};

pub(crate) const CHAMPION_KEY: &str = "vayne";

pub(crate) fn apply_behavior(profile: &mut ChampionBehaviorProfile) {
    let silver_bolts = vayne_silver_bolts_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vayne.json abilities.basic_ability_2"));
    profile.periodic_true_hit_every = silver_bolts.periodic_true_hit_every;
    profile.periodic_true_hit_base = silver_bolts.periodic_true_hit_base;
    profile.periodic_true_hit_ad_ratio = 0.0;
    profile.periodic_true_hit_target_max_health_ratio =
        silver_bolts.periodic_true_hit_target_max_health_ratio;
    super::apply_behavior_override(CHAMPION_KEY, profile);
}

pub(crate) fn event_cooldown_seconds(event: ChampionScriptEvent) -> Option<f64> {
    if event != ChampionScriptEvent::VayneTumbleEmpower {
        return None;
    }
    Some(
        vayne_tumble_ability_defaults(CHAMPION_KEY)
            .unwrap_or_else(|| panic!("Missing Characters/Vayne.json abilities.basic_ability_1"))
            .tumble_cooldown_seconds,
    )
}

pub(crate) fn execute_tumble_empower(
    input: ChampionScriptExecutionInput,
) -> Vec<ChampionScriptAction> {
    let ability_defaults = vayne_tumble_ability_defaults(CHAMPION_KEY)
        .unwrap_or_else(|| panic!("Missing Characters/Vayne.json abilities.basic_ability_1"));
    vec![ChampionScriptAction::AddNextAttackBonusPhysical {
        amount: ability_defaults.tumble_bonus_physical_attack_damage_ratio
            * input.physical_hit_damage
            + ability_defaults.tumble_bonus_physical_ability_power_ratio
                * input.burst_magic_damage.max(0.0),
        trace_message: "empowered next attack",
    }]
}

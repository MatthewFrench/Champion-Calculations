pub(crate) mod abilities;
pub(crate) mod decisions;
pub(crate) mod hook;

pub(crate) use abilities::{
    VladimirAbilityCooldowns, VladimirAbilityTuning, e_damage_raw, offensive_cooldowns_after_haste,
    q_damage_raw, r_damage_raw,
};
pub(crate) use decisions::{
    VladimirCastProfile, VladimirDefensiveAbilityDecisionInput, VladimirOffensiveDecisionInput,
    VladimirTargetSnapshot, decide_defensive_ability_activations, decide_offensive_casts,
    default_cast_profile,
};
pub(crate) use hook::VLADIMIR_HOOK;

#[cfg(test)]
#[path = "tests/vladimir_tests.rs"]
mod tests;

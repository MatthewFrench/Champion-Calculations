#![allow(dead_code)]
#![allow(unused_imports)]

#[allow(dead_code)]
mod champion_control_action_validation_channels;
#[allow(dead_code)]
mod champion_control_contracts;
#[allow(dead_code)]
mod champion_control_decision_policy_channels;
#[allow(dead_code)]
mod champion_control_observation_channels;

pub(crate) use self::champion_control_action_validation_channels::validate_champion_action_request;
pub(crate) use self::champion_control_contracts::{
    ChampionAbilitySlotReadiness, ChampionActionDecisionPolicy, ChampionActionRequest,
    ChampionActionRuntimeState, ChampionActionStatus, ChampionActionStatusReport,
    ChampionActorControlSnapshot, ChampionBasicAttackReadiness,
    ChampionControlPerspectiveBuildInput, ChampionControllerIdentity, ChampionControllerKind,
    ChampionItemActiveReadiness, ChampionPerspectiveView, ChampionVisibleActor,
    ChampionVisibleActorTargetingProjection,
};
pub(crate) use self::champion_control_decision_policy_channels::{
    GenericChampionControllerDecisionPolicy, LayeredChampionControllerDecisionPolicy,
};
pub(crate) use self::champion_control_observation_channels::build_champion_perspective_view;

#[cfg(test)]
#[path = "champion_control_harness/tests/champion_control_harness_tests.rs"]
mod tests;

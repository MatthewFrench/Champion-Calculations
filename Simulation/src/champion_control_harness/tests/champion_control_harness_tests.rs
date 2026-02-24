use std::collections::HashMap;

use crate::champion_control_harness::{
    ChampionActionDecisionPolicy, ChampionActionRequest, ChampionActionRuntimeState,
    ChampionActionStatus, ChampionActorControlSnapshot, ChampionControlPerspectiveBuildInput,
    ChampionControllerIdentity, ChampionControllerKind, GenericChampionControllerDecisionPolicy,
    LayeredChampionControllerDecisionPolicy, build_champion_perspective_view,
    validate_champion_action_request,
};
use crate::scripts::runtime::ability_slots::{AbilitySlotKey, ActorAbilityLoadout};
use crate::world::{
    WorldActorAllegiance, WorldActorClass, WorldActorPosition, WorldState,
    default_urf_world_map_state,
};

fn test_world_state() -> WorldState {
    let mut world_state = WorldState::new(default_urf_world_map_state());
    world_state
        .register_actor_position_with_allegiance(
            "controlled",
            WorldActorClass::Champion,
            WorldActorAllegiance::ControlledChampionTeam,
            WorldActorPosition { x: 0.0, y: 0.0 },
        )
        .expect("controlled actor registration should succeed");
    world_state
        .register_actor_position_with_allegiance(
            "enemy_close",
            WorldActorClass::Champion,
            WorldActorAllegiance::OpponentTeam,
            WorldActorPosition { x: 350.0, y: 0.0 },
        )
        .expect("near enemy registration should succeed");
    world_state
        .register_actor_position_with_allegiance(
            "enemy_far",
            WorldActorClass::Champion,
            WorldActorAllegiance::OpponentTeam,
            WorldActorPosition { x: 1500.0, y: 0.0 },
        )
        .expect("far enemy registration should succeed");
    world_state
}

fn test_loadout() -> ActorAbilityLoadout {
    let mut loadout = ActorAbilityLoadout::default();
    loadout.assign_ability_to_slot("Transfusion".to_string(), AbilitySlotKey::Q);
    loadout.assign_ability_to_slot("Sanguine Pool".to_string(), AbilitySlotKey::W);
    loadout
}

fn test_runtime_state() -> ChampionActionRuntimeState {
    let mut ability_ready_at_seconds_by_id = HashMap::new();
    ability_ready_at_seconds_by_id.insert("Transfusion".to_string(), 0.0);
    ability_ready_at_seconds_by_id.insert("Sanguine Pool".to_string(), 5.0);

    let mut ability_cast_range_by_id = HashMap::new();
    ability_cast_range_by_id.insert("Transfusion".to_string(), 600.0);
    ability_cast_range_by_id.insert("Sanguine Pool".to_string(), 0.0);

    let mut item_active_ready_at_seconds_by_id = HashMap::new();
    item_active_ready_at_seconds_by_id.insert("zhonya_time_stop".to_string(), 0.0);

    let mut item_active_cast_range_by_id = HashMap::new();
    item_active_cast_range_by_id.insert("zhonya_time_stop".to_string(), 0.0);

    ChampionActionRuntimeState {
        ability_ready_at_seconds_by_id,
        ability_cast_range_by_id,
        item_active_ready_at_seconds_by_id,
        item_active_cast_range_by_id,
        basic_attack_ready_at_seconds: 0.0,
        basic_attack_range: 450.0,
        movement_locked_until_seconds: 0.0,
        cast_locked_until_seconds: 0.0,
    }
}

fn build_view(
    controller_kind: ChampionControllerKind,
) -> crate::champion_control_harness::ChampionPerspectiveView {
    build_champion_perspective_view(ChampionControlPerspectiveBuildInput {
        now_seconds: 1.0,
        controller_identity: ChampionControllerIdentity {
            controller_id: "controller".to_string(),
            controller_kind,
        },
        controlled_actor_id: "controlled",
        controlled_actor_snapshot: ChampionActorControlSnapshot {
            position: WorldActorPosition { x: 0.0, y: 0.0 },
            health_ratio: 1.0,
            vision_radius: 1000.0,
        },
        controlled_actor_ability_loadout: &test_loadout(),
        controlled_actor_runtime_state: &test_runtime_state(),
        world_state: &test_world_state(),
    })
}

#[test]
fn perspective_view_only_contains_visible_actors_in_vision_radius() {
    let view = build_view(ChampionControllerKind::HumanPlayer);
    let visible_ids = view
        .visible_actors
        .iter()
        .map(|actor| actor.actor_id.as_str())
        .collect::<Vec<_>>();
    assert!(
        visible_ids.contains(&"enemy_close"),
        "near actor should be visible"
    );
    assert!(
        !visible_ids.contains(&"enemy_far"),
        "far actor should not be visible"
    );
}

#[test]
fn action_validation_is_identical_for_human_and_ai_controllers() {
    let human_view = build_view(ChampionControllerKind::HumanPlayer);
    let ai_view = build_view(ChampionControllerKind::ArtificialIntelligence);
    let request = ChampionActionRequest::CastAbilityBySlot {
        ability_slot: AbilitySlotKey::W,
        target_actor_id: None,
        target_position: None,
    };

    let human_status = validate_champion_action_request(&human_view, request.clone()).status;
    let ai_status = validate_champion_action_request(&ai_view, request).status;

    match (human_status, ai_status) {
        (
            ChampionActionStatus::RejectedAbilityOnCooldown {
                ability_id: left_ability_id,
                remaining_seconds: left_remaining_seconds,
            },
            ChampionActionStatus::RejectedAbilityOnCooldown {
                ability_id: right_ability_id,
                remaining_seconds: right_remaining_seconds,
            },
        ) => {
            assert_eq!(left_ability_id, right_ability_id);
            assert!((left_remaining_seconds - right_remaining_seconds).abs() < 1e-9);
        }
        _ => panic!("expected equal cooldown rejection status for human and ai"),
    }
}

#[test]
fn action_validation_rejects_out_of_range_targets() {
    let view = build_view(ChampionControllerKind::HumanPlayer);
    let report = validate_champion_action_request(
        &view,
        ChampionActionRequest::StartBasicAttack {
            target_actor_id: "enemy_far".to_string(),
        },
    );
    match report.status {
        ChampionActionStatus::RejectedTargetNotVisible { target_actor_id } => {
            assert_eq!(target_actor_id, "enemy_far");
        }
        _ => panic!("expected not-visible rejection for out-of-vision target"),
    }
}

#[test]
fn generic_ai_policy_prefers_in_range_basic_attack() {
    let mut policy = GenericChampionControllerDecisionPolicy;
    let view = build_view(ChampionControllerKind::ArtificialIntelligence);
    let action = policy.choose_action(&view);
    match action {
        Some(ChampionActionRequest::StartBasicAttack { target_actor_id }) => {
            assert_eq!(target_actor_id, "enemy_close")
        }
        _ => panic!("expected basic attack action against nearest visible opponent"),
    }
}

#[derive(Debug)]
struct ChampionSpecificOverridePolicy;

impl ChampionActionDecisionPolicy for ChampionSpecificOverridePolicy {
    fn choose_action(
        &mut self,
        _view: &crate::champion_control_harness::ChampionPerspectiveView,
    ) -> Option<ChampionActionRequest> {
        Some(ChampionActionRequest::StopCurrentAction)
    }
}

#[test]
fn layered_policy_uses_champion_specific_policy_before_fallback() {
    let mut policy =
        LayeredChampionControllerDecisionPolicy::new(Box::new(ChampionSpecificOverridePolicy));
    let view = build_view(ChampionControllerKind::ArtificialIntelligence);
    let action = policy.choose_action(&view);
    assert!(
        matches!(action, Some(ChampionActionRequest::StopCurrentAction)),
        "champion-specific policy should take precedence over generic fallback policy"
    );
}

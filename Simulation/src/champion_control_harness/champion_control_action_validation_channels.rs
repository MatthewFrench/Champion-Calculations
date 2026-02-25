use super::{
    ChampionAbilitySlotReadiness, ChampionActionRequest, ChampionActionStatus,
    ChampionActionStatusReport, ChampionItemActiveReadiness, ChampionPerspectiveView,
    ChampionVisibleActorTargetingProjection,
};

fn visible_actor_targeting_projection(
    view: &ChampionPerspectiveView,
    target_actor_id: &str,
) -> Option<ChampionVisibleActorTargetingProjection> {
    view.visible_actors
        .iter()
        .find(|actor| actor.actor_id == target_actor_id)
        .map(|actor| ChampionVisibleActorTargetingProjection {
            distance_to_controlled_actor: actor.distance_to_controlled_actor,
        })
}

fn ability_slot_readiness(
    view: &ChampionPerspectiveView,
    ability_slot: crate::scripts::runtime::ability_slots::AbilitySlotKey,
) -> Option<&ChampionAbilitySlotReadiness> {
    view.ability_slot_readiness
        .iter()
        .find(|readiness| readiness.ability_slot == ability_slot)
}

fn item_active_readiness<'view>(
    view: &'view ChampionPerspectiveView,
    item_active_id: &str,
) -> Option<&'view ChampionItemActiveReadiness> {
    view.item_active_readiness
        .iter()
        .find(|readiness| readiness.item_active_id == item_active_id)
}

// Action validation returns explicit status outcomes for every request so controllers can
// react to legal and illegal outcomes without hidden engine-side privileges.
pub(crate) fn validate_champion_action_request(
    view: &ChampionPerspectiveView,
    request: ChampionActionRequest,
) -> ChampionActionStatusReport {
    let status = match &request {
        ChampionActionRequest::MoveToPosition { .. } => {
            if view.movement_locked_remaining_seconds > 0.0 {
                ChampionActionStatus::RejectedMovementLocked {
                    remaining_seconds: view.movement_locked_remaining_seconds,
                }
            } else {
                ChampionActionStatus::AcceptedQueued
            }
        }
        ChampionActionRequest::CastAbilityBySlot {
            ability_slot,
            target_actor_id,
            target_position: _,
        } => {
            if view.cast_locked_remaining_seconds > 0.0 {
                ChampionActionStatus::RejectedCastLocked {
                    remaining_seconds: view.cast_locked_remaining_seconds,
                }
            } else if let Some(readiness) = ability_slot_readiness(view, *ability_slot) {
                if readiness.remaining_cooldown_seconds > 0.0 {
                    ChampionActionStatus::RejectedAbilityOnCooldown {
                        ability_id: readiness.ability_id.clone(),
                        remaining_seconds: readiness.remaining_cooldown_seconds,
                    }
                } else if let Some(target_actor_id) = target_actor_id {
                    if let Some(target_projection) =
                        visible_actor_targeting_projection(view, target_actor_id)
                    {
                        if readiness.cast_range > 0.0
                            && target_projection.distance_to_controlled_actor
                                > readiness.cast_range + 1e-9
                        {
                            ChampionActionStatus::RejectedTargetOutOfRange {
                                target_actor_id: target_actor_id.clone(),
                                required_range: readiness.cast_range,
                                distance_to_target: target_projection.distance_to_controlled_actor,
                            }
                        } else {
                            ChampionActionStatus::AcceptedQueued
                        }
                    } else {
                        ChampionActionStatus::RejectedTargetNotVisible {
                            target_actor_id: target_actor_id.clone(),
                        }
                    }
                } else {
                    ChampionActionStatus::AcceptedQueued
                }
            } else {
                ChampionActionStatus::RejectedAbilitySlotUnbound {
                    ability_slot: *ability_slot,
                }
            }
        }
        ChampionActionRequest::StartBasicAttack { target_actor_id } => {
            // StartBasicAttack is a target-intent command. Cooldown gating is enforced by the
            // runtime attack scheduler, while validation enforces perspective legality.
            if let Some(target_projection) =
                visible_actor_targeting_projection(view, target_actor_id)
            {
                if target_projection.distance_to_controlled_actor
                    <= view.basic_attack_readiness.attack_range + 1e-9
                {
                    ChampionActionStatus::AcceptedQueued
                } else {
                    ChampionActionStatus::RejectedTargetOutOfRange {
                        target_actor_id: target_actor_id.clone(),
                        required_range: view.basic_attack_readiness.attack_range,
                        distance_to_target: target_projection.distance_to_controlled_actor,
                    }
                }
            } else {
                ChampionActionStatus::RejectedTargetNotVisible {
                    target_actor_id: target_actor_id.clone(),
                }
            }
        }
        ChampionActionRequest::UseItemActive {
            item_active_id,
            target_actor_id,
            target_position: _,
        } => {
            if view.cast_locked_remaining_seconds > 0.0 {
                ChampionActionStatus::RejectedCastLocked {
                    remaining_seconds: view.cast_locked_remaining_seconds,
                }
            } else if let Some(readiness) = item_active_readiness(view, item_active_id) {
                if readiness.remaining_cooldown_seconds > 0.0 {
                    ChampionActionStatus::RejectedItemActiveOnCooldown {
                        item_active_id: item_active_id.clone(),
                        remaining_seconds: readiness.remaining_cooldown_seconds,
                    }
                } else if let Some(target_actor_id) = target_actor_id {
                    if let Some(target_projection) =
                        visible_actor_targeting_projection(view, target_actor_id)
                    {
                        if readiness.cast_range > 0.0
                            && target_projection.distance_to_controlled_actor
                                > readiness.cast_range + 1e-9
                        {
                            ChampionActionStatus::RejectedTargetOutOfRange {
                                target_actor_id: target_actor_id.clone(),
                                required_range: readiness.cast_range,
                                distance_to_target: target_projection.distance_to_controlled_actor,
                            }
                        } else {
                            ChampionActionStatus::AcceptedQueued
                        }
                    } else {
                        ChampionActionStatus::RejectedTargetNotVisible {
                            target_actor_id: target_actor_id.clone(),
                        }
                    }
                } else {
                    ChampionActionStatus::AcceptedQueued
                }
            } else {
                ChampionActionStatus::RejectedUnknownItemActive {
                    item_active_id: item_active_id.clone(),
                }
            }
        }
        ChampionActionRequest::StopCurrentAction => ChampionActionStatus::AcceptedQueued,
    };

    ChampionActionStatusReport {
        request,
        status,
        server_time_seconds: view.now_seconds,
    }
}

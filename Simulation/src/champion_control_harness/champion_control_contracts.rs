use std::collections::HashMap;

use crate::scripts::runtime::ability_slots::{AbilitySlotKey, ActorAbilityLoadout};
use crate::world::{WorldActorAllegiance, WorldActorClass, WorldActorPosition, WorldState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChampionControllerKind {
    HumanPlayer,
    ArtificialIntelligence,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionControllerIdentity {
    pub controller_id: String,
    pub controller_kind: ChampionControllerKind,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionActionRuntimeState {
    pub ability_ready_at_seconds_by_id: HashMap<String, f64>,
    pub ability_cast_range_by_id: HashMap<String, f64>,
    pub item_active_ready_at_seconds_by_id: HashMap<String, f64>,
    pub item_active_cast_range_by_id: HashMap<String, f64>,
    pub basic_attack_ready_at_seconds: f64,
    pub basic_attack_range: f64,
    pub movement_locked_until_seconds: f64,
    pub cast_locked_until_seconds: f64,
}

impl Default for ChampionActionRuntimeState {
    fn default() -> Self {
        Self {
            ability_ready_at_seconds_by_id: HashMap::new(),
            ability_cast_range_by_id: HashMap::new(),
            item_active_ready_at_seconds_by_id: HashMap::new(),
            item_active_cast_range_by_id: HashMap::new(),
            basic_attack_ready_at_seconds: 0.0,
            basic_attack_range: 0.0,
            movement_locked_until_seconds: 0.0,
            cast_locked_until_seconds: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionActorControlSnapshot {
    pub position: WorldActorPosition,
    pub health_ratio: f64,
    pub vision_radius: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionControlPerspectiveBuildInput<'a> {
    pub now_seconds: f64,
    pub controller_identity: ChampionControllerIdentity,
    pub controlled_actor_id: &'a str,
    pub controlled_actor_snapshot: ChampionActorControlSnapshot,
    pub controlled_actor_ability_loadout: &'a ActorAbilityLoadout,
    pub controlled_actor_runtime_state: &'a ChampionActionRuntimeState,
    pub world_state: &'a WorldState,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionVisibleActor {
    pub actor_id: String,
    pub actor_class: WorldActorClass,
    pub actor_allegiance: WorldActorAllegiance,
    pub position: WorldActorPosition,
    pub distance_to_controlled_actor: f64,
    pub health_ratio: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionVisibleActorTargetingProjection {
    pub distance_to_controlled_actor: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionAbilitySlotReadiness {
    pub ability_slot: AbilitySlotKey,
    pub ability_id: String,
    pub cast_range: f64,
    pub remaining_cooldown_seconds: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionItemActiveReadiness {
    pub item_active_id: String,
    pub cast_range: f64,
    pub remaining_cooldown_seconds: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChampionBasicAttackReadiness {
    pub attack_range: f64,
    pub remaining_cooldown_seconds: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionPerspectiveView {
    pub now_seconds: f64,
    pub controller_identity: ChampionControllerIdentity,
    pub controlled_actor_id: String,
    pub controlled_actor_snapshot: ChampionActorControlSnapshot,
    pub movement_locked_remaining_seconds: f64,
    pub cast_locked_remaining_seconds: f64,
    pub ability_slot_readiness: Vec<ChampionAbilitySlotReadiness>,
    pub item_active_readiness: Vec<ChampionItemActiveReadiness>,
    pub basic_attack_readiness: ChampionBasicAttackReadiness,
    pub visible_actors: Vec<ChampionVisibleActor>,
}

#[derive(Debug, Clone)]
pub(crate) enum ChampionActionRequest {
    MoveToPosition {
        target_position: WorldActorPosition,
    },
    CastAbilityBySlot {
        ability_slot: AbilitySlotKey,
        target_actor_id: Option<String>,
        target_position: Option<WorldActorPosition>,
    },
    StartBasicAttack {
        target_actor_id: String,
    },
    UseItemActive {
        item_active_id: String,
        target_actor_id: Option<String>,
        target_position: Option<WorldActorPosition>,
    },
    StopCurrentAction,
}

#[derive(Debug, Clone)]
pub(crate) enum ChampionActionStatus {
    AcceptedQueued,
    RejectedMovementLocked {
        remaining_seconds: f64,
    },
    RejectedCastLocked {
        remaining_seconds: f64,
    },
    RejectedAbilitySlotUnbound {
        ability_slot: AbilitySlotKey,
    },
    RejectedAbilityOnCooldown {
        ability_id: String,
        remaining_seconds: f64,
    },
    RejectedItemActiveOnCooldown {
        item_active_id: String,
        remaining_seconds: f64,
    },
    RejectedTargetNotVisible {
        target_actor_id: String,
    },
    RejectedTargetOutOfRange {
        target_actor_id: String,
        required_range: f64,
        distance_to_target: f64,
    },
    RejectedUnknownItemActive {
        item_active_id: String,
    },
    RejectedTargetInvalidForAction {
        target_actor_id: String,
        reason: String,
    },
    RejectedUnsupportedAction {
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct ChampionActionStatusReport {
    pub request: ChampionActionRequest,
    pub status: ChampionActionStatus,
    pub server_time_seconds: f64,
}

pub(crate) trait ChampionActionDecisionPolicy: std::fmt::Debug + Send {
    fn choose_action(&mut self, view: &ChampionPerspectiveView) -> Option<ChampionActionRequest>;
}

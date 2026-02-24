use super::{ChampionActionDecisionPolicy, ChampionActionRequest, ChampionPerspectiveView};

#[derive(Debug, Default)]
pub(crate) struct GenericChampionControllerDecisionPolicy;

impl GenericChampionControllerDecisionPolicy {
    fn nearest_visible_opponent(
        view: &ChampionPerspectiveView,
    ) -> Option<&crate::champion_control_harness::ChampionVisibleActor> {
        view.visible_actors
            .iter()
            .filter(|actor| {
                actor.actor_allegiance == crate::world::WorldActorAllegiance::OpponentTeam
            })
            .min_by(|first, second| {
                first
                    .distance_to_controlled_actor
                    .total_cmp(&second.distance_to_controlled_actor)
                    .then_with(|| first.actor_id.cmp(&second.actor_id))
            })
    }
}

// Generic AI policy intentionally uses only controller-visible inputs, so it has no access
// advantage over a human-player controller acting through the same harness interface.
impl ChampionActionDecisionPolicy for GenericChampionControllerDecisionPolicy {
    fn choose_action(&mut self, view: &ChampionPerspectiveView) -> Option<ChampionActionRequest> {
        let nearest_opponent = Self::nearest_visible_opponent(view)?;

        if view.basic_attack_readiness.remaining_cooldown_seconds <= 0.0
            && nearest_opponent.distance_to_controlled_actor
                <= view.basic_attack_readiness.attack_range + 1e-9
        {
            return Some(ChampionActionRequest::StartBasicAttack {
                target_actor_id: nearest_opponent.actor_id.clone(),
            });
        }

        if view.cast_locked_remaining_seconds <= 0.0 {
            for readiness in &view.ability_slot_readiness {
                if readiness.remaining_cooldown_seconds > 0.0 {
                    continue;
                }
                if readiness.cast_range > 0.0
                    && nearest_opponent.distance_to_controlled_actor > readiness.cast_range + 1e-9
                {
                    continue;
                }
                return Some(ChampionActionRequest::CastAbilityBySlot {
                    ability_slot: readiness.ability_slot,
                    target_actor_id: Some(nearest_opponent.actor_id.clone()),
                    target_position: Some(nearest_opponent.position),
                });
            }
        }

        if view.movement_locked_remaining_seconds <= 0.0 {
            return Some(ChampionActionRequest::MoveToPosition {
                target_position: nearest_opponent.position,
            });
        }

        Some(ChampionActionRequest::StopCurrentAction)
    }
}

#[derive(Debug)]
pub(crate) struct LayeredChampionControllerDecisionPolicy {
    champion_specific_policy: Box<dyn ChampionActionDecisionPolicy>,
    fallback_policy: GenericChampionControllerDecisionPolicy,
}

impl LayeredChampionControllerDecisionPolicy {
    pub(crate) fn new(champion_specific_policy: Box<dyn ChampionActionDecisionPolicy>) -> Self {
        Self {
            champion_specific_policy,
            fallback_policy: GenericChampionControllerDecisionPolicy,
        }
    }
}

impl ChampionActionDecisionPolicy for LayeredChampionControllerDecisionPolicy {
    fn choose_action(&mut self, view: &ChampionPerspectiveView) -> Option<ChampionActionRequest> {
        if let Some(action) = self.champion_specific_policy.choose_action(view) {
            return Some(action);
        }
        self.fallback_policy.choose_action(view)
    }
}

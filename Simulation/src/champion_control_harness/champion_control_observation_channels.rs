use super::{
    ChampionAbilitySlotReadiness, ChampionBasicAttackReadiness,
    ChampionControlPerspectiveBuildInput, ChampionItemActiveReadiness, ChampionPerspectiveView,
    ChampionVisibleActor,
};

fn distance_between(
    a: crate::world::WorldActorPosition,
    b: crate::world::WorldActorPosition,
) -> f64 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

// Observation channels intentionally expose only perspective-visible state. Both human-player
// and artificial-intelligence controllers use the same projection path and visibility bounds.
pub(crate) fn build_champion_perspective_view(
    input: ChampionControlPerspectiveBuildInput<'_>,
) -> ChampionPerspectiveView {
    let movement_locked_remaining_seconds = (input
        .controlled_actor_runtime_state
        .movement_locked_until_seconds
        - input.now_seconds)
        .max(0.0);
    let cast_locked_remaining_seconds = (input
        .controlled_actor_runtime_state
        .cast_locked_until_seconds
        - input.now_seconds)
        .max(0.0);

    let ability_slot_readiness = input
        .controlled_actor_ability_loadout
        .slot_bindings()
        .into_iter()
        .map(|(ability_slot, ability_id)| {
            let ready_at = input
                .controlled_actor_runtime_state
                .ability_ready_at_seconds_by_id
                .get(ability_id)
                .copied()
                .unwrap_or(0.0);
            ChampionAbilitySlotReadiness {
                ability_slot,
                ability_id: ability_id.to_string(),
                cast_range: input
                    .controlled_actor_runtime_state
                    .ability_cast_range_by_id
                    .get(ability_id)
                    .copied()
                    .unwrap_or(0.0),
                remaining_cooldown_seconds: (ready_at - input.now_seconds).max(0.0),
            }
        })
        .collect::<Vec<_>>();

    let mut item_active_readiness = input
        .controlled_actor_runtime_state
        .item_active_ready_at_seconds_by_id
        .iter()
        .map(|(item_active_id, ready_at)| ChampionItemActiveReadiness {
            item_active_id: item_active_id.clone(),
            cast_range: input
                .controlled_actor_runtime_state
                .item_active_cast_range_by_id
                .get(item_active_id)
                .copied()
                .unwrap_or(0.0),
            remaining_cooldown_seconds: (*ready_at - input.now_seconds).max(0.0),
        })
        .collect::<Vec<_>>();
    item_active_readiness.sort_by(|a, b| a.item_active_id.cmp(&b.item_active_id));

    let basic_attack_readiness = ChampionBasicAttackReadiness {
        attack_range: input
            .controlled_actor_runtime_state
            .basic_attack_range
            .max(0.0),
        remaining_cooldown_seconds: (input
            .controlled_actor_runtime_state
            .basic_attack_ready_at_seconds
            - input.now_seconds)
            .max(0.0),
    };

    let mut visible_actors = input
        .world_state
        .actor_snapshot_entries()
        .into_iter()
        .filter_map(|(actor_id, snapshot)| {
            if actor_id == input.controlled_actor_id {
                return None;
            }
            let distance_to_controlled_actor =
                distance_between(input.controlled_actor_snapshot.position, snapshot.position);
            if distance_to_controlled_actor > input.controlled_actor_snapshot.vision_radius {
                return None;
            }
            Some(ChampionVisibleActor {
                actor_id,
                actor_class: snapshot.actor_class,
                actor_allegiance: snapshot.actor_allegiance,
                position: snapshot.position,
                distance_to_controlled_actor,
                health_ratio: None,
            })
        })
        .collect::<Vec<_>>();
    visible_actors.sort_by(|first, second| {
        first
            .distance_to_controlled_actor
            .total_cmp(&second.distance_to_controlled_actor)
            .then_with(|| first.actor_id.cmp(&second.actor_id))
    });

    ChampionPerspectiveView {
        now_seconds: input.now_seconds,
        controller_identity: input.controller_identity,
        controlled_actor_id: input.controlled_actor_id.to_string(),
        controlled_actor_snapshot: input.controlled_actor_snapshot,
        movement_locked_remaining_seconds,
        cast_locked_remaining_seconds,
        ability_slot_readiness,
        item_active_readiness,
        basic_attack_readiness,
        visible_actors,
    }
}

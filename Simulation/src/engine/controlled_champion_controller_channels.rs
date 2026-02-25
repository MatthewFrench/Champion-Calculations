use std::collections::HashMap;

use crate::champion_control_harness::{
    ChampionActionDecisionPolicy, ChampionActionRequest, ChampionActionRuntimeState,
    ChampionActionStatus, ChampionActionStatusReport, ChampionActorControlSnapshot,
    ChampionControlPerspectiveBuildInput, ChampionControllerIdentity, ChampionPerspectiveView,
    build_champion_perspective_view, validate_champion_action_request,
};
use crate::scripts::runtime::ability_slots::{
    AbilitySlotKey, ActorAbilityLoadout, default_champion_ability_loadout,
};
use crate::world::WorldActorPosition;

use super::*;

pub(in crate::engine) const STASIS_ITEM_ACTIVE_ID: &str = "stasis_item";
pub(in crate::engine) const EMERGENCY_SHIELD_ITEM_ACTIVE_ID: &str = "emergency_shield_item";

const ACTION_STATUS_REPORT_BUFFER_LIMIT: usize = 512;

#[derive(Debug, Clone)]
pub(in crate::engine) struct QueuedActorActionRequest {
    pub sequence_id: u64,
    pub execute_at_tick: u64,
    pub controlled_actor_id: String,
    pub controller_identity: ChampionControllerIdentity,
    pub request: ChampionActionRequest,
}

impl ControlledChampionCombatSimulation {
    #[allow(dead_code)]
    pub(crate) fn set_controlled_champion_controller_policy(
        &mut self,
        controller_identity: ChampionControllerIdentity,
        policy: Box<dyn ChampionActionDecisionPolicy>,
    ) {
        self.controlled_champion_controller_identity = controller_identity;
        self.controlled_champion_controller_policy = Some(policy);
        // Once harness control is enabled, script auto-cast channels stay disabled.
        self.controlled_champion_manual_control_mode = true;
    }

    #[allow(dead_code)]
    pub(crate) fn clear_controlled_champion_controller_policy(&mut self) {
        self.controlled_champion_controller_policy = None;
        // Manual-control mode remains enabled so script cadence cannot silently resume.
    }

    #[allow(dead_code)]
    pub(crate) fn queue_controlled_champion_action_request(
        &mut self,
        controller_identity: ChampionControllerIdentity,
        request: ChampionActionRequest,
    ) -> ChampionActionStatusReport {
        let controlled_actor_id = self.controlled_champion_world_actor_id.clone();
        self.queue_actor_action_request(controller_identity, &controlled_actor_id, request)
    }

    pub(crate) fn queue_actor_action_request(
        &mut self,
        controller_identity: ChampionControllerIdentity,
        controlled_actor_id: &str,
        request: ChampionActionRequest,
    ) -> ChampionActionStatusReport {
        let Some(perspective_view) =
            self.build_actor_perspective_view(controller_identity.clone(), controlled_actor_id)
        else {
            let status_report = ChampionActionStatusReport {
                request,
                status: ChampionActionStatus::RejectedControlledActorNotFound {
                    controlled_actor_id: controlled_actor_id.to_string(),
                },
                server_time_seconds: self.time,
            };
            self.record_controlled_champion_action_status_report(status_report.clone());
            return status_report;
        };

        if controlled_actor_id == self.controlled_champion_world_actor_id {
            self.controlled_champion_manual_control_mode = true;
        }

        let status_report = validate_champion_action_request(&perspective_view, request);
        let status_report =
            self.apply_runtime_actor_action_support_constraints(controlled_actor_id, status_report);
        self.record_controlled_champion_action_status_report(status_report.clone());

        if matches!(status_report.status, ChampionActionStatus::AcceptedQueued) {
            if controlled_actor_id != self.controlled_champion_world_actor_id {
                self.manually_controlled_enemy_actor_ids
                    .insert(controlled_actor_id.to_string());
            }
            let sequence_id = self.next_controlled_champion_action_request_sequence();
            let execute_at_tick = self.next_controlled_champion_action_request_execute_tick();
            self.controlled_champion_pending_action_requests
                .push_back(QueuedActorActionRequest {
                    sequence_id,
                    execute_at_tick,
                    controlled_actor_id: controlled_actor_id.to_string(),
                    controller_identity,
                    request: status_report.request.clone(),
                });
        }
        status_report
    }

    #[allow(dead_code)]
    pub(crate) fn drain_controlled_champion_action_status_reports(
        &mut self,
    ) -> Vec<ChampionActionStatusReport> {
        self.controlled_champion_recent_action_status_reports
            .drain(..)
            .collect()
    }

    pub(in crate::engine) fn controlled_champion_manual_control_mode_enabled(&self) -> bool {
        self.controlled_champion_manual_control_mode
    }

    pub(in crate::engine) fn enqueue_controller_policy_action_request_for_tick(&mut self) {
        let controller_identity = self.controlled_champion_controller_identity.clone();
        let Some(view) = self.build_actor_perspective_view(
            controller_identity.clone(),
            &self.controlled_champion_world_actor_id,
        ) else {
            return;
        };
        let next_action = self
            .controlled_champion_controller_policy
            .as_mut()
            .and_then(|policy| policy.choose_action(&view));
        if let Some(action_request) = next_action {
            let controlled_actor_id = self.controlled_champion_world_actor_id.clone();
            let _ = self.queue_actor_action_request(
                controller_identity,
                &controlled_actor_id,
                action_request,
            );
        }
    }

    pub(in crate::engine) fn process_pending_controlled_champion_action_requests(&mut self) {
        while self
            .controlled_champion_pending_action_requests
            .front()
            .map(|queued_action| queued_action.execute_at_tick)
            .is_some_and(|execute_at_tick| {
                execute_at_tick <= self.controlled_champion_current_tick_index
            })
        {
            let Some(queued_action_request) =
                self.controlled_champion_pending_action_requests.pop_front()
            else {
                return;
            };
            if queued_action_request.controlled_actor_id == self.controlled_champion_world_actor_id
            {
                self.execute_controlled_champion_action_request(queued_action_request);
            } else {
                self.execute_enemy_actor_action_request(queued_action_request);
            }
        }
    }

    pub(in crate::engine) fn controlled_champion_basic_attack_target_index(&self) -> Option<usize> {
        let actor_id = self
            .controlled_champion_basic_attack_target_actor_id
            .as_deref()?;
        self.resolve_enemy_index_by_actor_id(actor_id)
            .filter(|idx| self.enemy_is_active(*idx))
    }

    pub(in crate::engine) fn apply_move_to_position_command(
        &mut self,
        target_position: WorldActorPosition,
    ) {
        let (clamped_x, clamped_y) = self
            .world_state
            .map
            .bounds
            .clamp(target_position.x, target_position.y);
        self.controlled_champion_pending_move_target_position = Some(Vec2 {
            x: clamped_x,
            y: clamped_y,
        });
    }

    pub(in crate::engine) fn clear_controlled_champion_move_command(&mut self) {
        self.controlled_champion_pending_move_target_position = None;
    }

    pub(in crate::engine) fn controlled_champion_pending_move_target_position(
        &self,
    ) -> Option<Vec2> {
        self.controlled_champion_pending_move_target_position
    }

    pub(in crate::engine) fn enemy_actor_manual_control_mode_enabled(
        &self,
        actor_id: &str,
    ) -> bool {
        self.manually_controlled_enemy_actor_ids.contains(actor_id)
    }

    pub(in crate::engine) fn enemy_pending_move_target_position(
        &self,
        actor_id: &str,
    ) -> Option<Vec2> {
        self.enemy_pending_move_target_position_by_actor_id
            .get(actor_id)
            .copied()
    }

    pub(in crate::engine) fn apply_enemy_move_to_position_command(
        &mut self,
        actor_id: &str,
        target_position: WorldActorPosition,
    ) {
        let (clamped_x, clamped_y) = self
            .world_state
            .map
            .bounds
            .clamp(target_position.x, target_position.y);
        self.enemy_pending_move_target_position_by_actor_id.insert(
            actor_id.to_string(),
            Vec2 {
                x: clamped_x,
                y: clamped_y,
            },
        );
    }

    pub(in crate::engine) fn clear_enemy_move_command(&mut self, actor_id: &str) {
        self.enemy_pending_move_target_position_by_actor_id
            .remove(actor_id);
    }

    fn apply_runtime_actor_action_support_constraints(
        &self,
        controlled_actor_id: &str,
        mut status_report: ChampionActionStatusReport,
    ) -> ChampionActionStatusReport {
        if !matches!(status_report.status, ChampionActionStatus::AcceptedQueued) {
            return status_report;
        }

        if !self.actor_supports_action_request(controlled_actor_id, &status_report.request) {
            status_report.status = ChampionActionStatus::RejectedUnsupportedAction {
                reason: format!(
                    "action is not currently supported by runtime channels for actor `{}`",
                    controlled_actor_id
                ),
            };
            return status_report;
        }

        if controlled_actor_id == self.controlled_champion_world_actor_id
            && let ChampionActionRequest::StartBasicAttack { target_actor_id } =
                &status_report.request
            && self
                .resolve_enemy_index_by_actor_id(target_actor_id)
                .is_none()
        {
            status_report.status = ChampionActionStatus::RejectedTargetInvalidForAction {
                target_actor_id: target_actor_id.clone(),
                reason: "target actor is not a supported opponent champion target".to_string(),
            };
        }

        status_report
    }

    fn actor_supports_action_request(
        &self,
        controlled_actor_id: &str,
        request: &ChampionActionRequest,
    ) -> bool {
        if controlled_actor_id == self.controlled_champion_world_actor_id {
            return self.controlled_champion_supports_action_request(request);
        }
        self.enemy_actor_supports_action_request(controlled_actor_id, request)
    }

    fn controlled_champion_supports_action_request(&self, request: &ChampionActionRequest) -> bool {
        match request {
            ChampionActionRequest::MoveToPosition { .. }
            | ChampionActionRequest::StartBasicAttack { .. }
            | ChampionActionRequest::StopCurrentAction => true,
            ChampionActionRequest::CastAbilityBySlot { ability_slot, .. } => self
                .ability_id_for_slot(*ability_slot)
                .map(|ability_id| self.controlled_champion_supports_ability_id(ability_id))
                .unwrap_or(false),
            ChampionActionRequest::UseItemActive { item_active_id, .. } => {
                (item_active_id == STASIS_ITEM_ACTIVE_ID && self.stasis_item_available)
                    || (item_active_id == EMERGENCY_SHIELD_ITEM_ACTIVE_ID
                        && self.emergency_shield_item_available)
            }
        }
    }

    fn enemy_actor_supports_action_request(
        &self,
        controlled_actor_id: &str,
        request: &ChampionActionRequest,
    ) -> bool {
        if self
            .resolve_enemy_index_by_actor_id(controlled_actor_id)
            .is_none()
        {
            return false;
        }
        matches!(
            request,
            ChampionActionRequest::MoveToPosition { .. } | ChampionActionRequest::StopCurrentAction
        )
    }

    fn controlled_champion_supports_ability_id(&self, ability_id: &str) -> bool {
        ability_id == self.cast_profile.offensive_primary_ability_id
            || ability_id == self.cast_profile.offensive_secondary_ability_id
            || ability_id == self.cast_profile.offensive_ultimate_ability_id
            || ability_id == self.cast_profile.defensive_ability_two_id
    }

    fn execute_controlled_champion_action_request(
        &mut self,
        queued_action_request: QueuedActorActionRequest,
    ) {
        match queued_action_request.request {
            ChampionActionRequest::MoveToPosition { target_position } => {
                self.apply_move_to_position_command(target_position);
                self.trace_event(
                    "controlled_champion_command",
                    format!(
                        "{} queued move command to ({:.1}, {:.1}) by {} #{}",
                        self.controlled_champion_name,
                        target_position.x,
                        target_position.y,
                        queued_action_request.controller_identity.controller_id,
                        queued_action_request.sequence_id
                    ),
                );
            }
            ChampionActionRequest::CastAbilityBySlot {
                ability_slot,
                target_actor_id,
                target_position,
            } => self.execute_cast_ability_by_slot_action_request(
                ability_slot,
                target_actor_id.as_deref(),
                target_position,
            ),
            ChampionActionRequest::StartBasicAttack { target_actor_id } => {
                self.controlled_champion_basic_attack_target_actor_id = Some(target_actor_id);
            }
            ChampionActionRequest::UseItemActive { item_active_id, .. } => {
                self.execute_item_active_action_request(&item_active_id);
            }
            ChampionActionRequest::StopCurrentAction => {
                self.clear_controlled_champion_move_command();
                self.controlled_champion_basic_attack_target_actor_id = None;
                // Invalidate queued windup/hit branches so stop command behaves as cancellation.
                self.controlled_champion_attack_sequence =
                    self.controlled_champion_attack_sequence.wrapping_add(1);
            }
        }
    }

    fn execute_enemy_actor_action_request(
        &mut self,
        queued_action_request: QueuedActorActionRequest,
    ) {
        match queued_action_request.request {
            ChampionActionRequest::MoveToPosition { target_position } => {
                self.apply_enemy_move_to_position_command(
                    &queued_action_request.controlled_actor_id,
                    target_position,
                );
                self.trace_event(
                    "enemy_actor_command",
                    format!(
                        "{} queued move command to ({:.1}, {:.1}) by {} #{}",
                        queued_action_request.controlled_actor_id,
                        target_position.x,
                        target_position.y,
                        queued_action_request.controller_identity.controller_id,
                        queued_action_request.sequence_id
                    ),
                );
            }
            ChampionActionRequest::StopCurrentAction => {
                self.clear_enemy_move_command(&queued_action_request.controlled_actor_id);
            }
            ChampionActionRequest::CastAbilityBySlot { .. }
            | ChampionActionRequest::StartBasicAttack { .. }
            | ChampionActionRequest::UseItemActive { .. } => {}
        }
    }

    fn execute_cast_ability_by_slot_action_request(
        &mut self,
        ability_slot: AbilitySlotKey,
        target_actor_id: Option<&str>,
        target_position: Option<WorldActorPosition>,
    ) {
        let Some(ability_id) = self.ability_id_for_slot(ability_slot).map(str::to_string) else {
            return;
        };

        if ability_id == self.cast_profile.defensive_ability_two_id {
            let _ = self.activate_controlled_champion_defensive_ability_two(&ability_id);
            return;
        }

        if ability_id == self.cast_profile.offensive_primary_ability_id {
            let target_index = target_actor_id
                .and_then(|actor_id| self.resolve_enemy_index_by_actor_id(actor_id))
                .or_else(|| {
                    self.first_active_enemy_in_controlled_champion_range(
                        self.cast_profile.offensive_primary_range,
                        self.cast_profile.offensive_primary_effect_hitbox_radius,
                    )
                });
            let Some(target_index) = target_index else {
                return;
            };
            let Some(target_at_cast) = self.enemy_position(target_index) else {
                return;
            };
            let impact_delay_seconds = self.enemy_projectile_delay_from_points(
                self.target_position,
                target_at_cast,
                self.cast_profile.offensive_primary_projectile_speed,
            );
            let next_ready_at = self.time + self.offensive_cooldowns.offensive_primary_seconds;
            let _ = self.schedule_controlled_champion_offensive_primary_cast(
                &ability_id,
                target_index,
                impact_delay_seconds,
                next_ready_at,
            );
            return;
        }

        if ability_id == self.cast_profile.offensive_secondary_ability_id {
            let impact_delay_seconds = self.projectile_impact_delay_from_requested_target(
                target_actor_id,
                target_position,
                self.cast_profile.offensive_secondary_projectile_speed,
            );
            let next_ready_at = self.time + self.offensive_cooldowns.offensive_secondary_seconds;
            let _ = self.schedule_controlled_champion_offensive_secondary_cast(
                &ability_id,
                impact_delay_seconds,
                next_ready_at,
            );
            return;
        }

        if ability_id == self.cast_profile.offensive_ultimate_ability_id {
            let impact_delay_seconds = self.projectile_impact_delay_from_requested_target(
                target_actor_id,
                target_position,
                self.cast_profile.offensive_ultimate_projectile_speed,
            );
            let next_ready_at = self.time + self.offensive_cooldowns.offensive_ultimate_seconds;
            let _ = self.schedule_controlled_champion_offensive_ultimate_cast(
                &ability_id,
                impact_delay_seconds,
                next_ready_at,
            );
        }
    }

    fn execute_item_active_action_request(&mut self, item_active_id: &str) {
        match item_active_id {
            STASIS_ITEM_ACTIVE_ID => {
                let _ = self.activate_controlled_champion_stasis_item_active(item_active_id);
            }
            EMERGENCY_SHIELD_ITEM_ACTIVE_ID => {
                let _ =
                    self.activate_controlled_champion_emergency_shield_item_active(item_active_id);
            }
            _ => {}
        }
    }

    fn projectile_impact_delay_from_requested_target(
        &self,
        target_actor_id: Option<&str>,
        target_position: Option<WorldActorPosition>,
        projectile_speed: f64,
    ) -> f64 {
        let target = target_position
            .map(|position| Vec2 {
                x: position.x,
                y: position.y,
            })
            .or_else(|| {
                target_actor_id
                    .and_then(|actor_id| self.world_state.actor_position(actor_id))
                    .map(|position| Vec2 {
                        x: position.x,
                        y: position.y,
                    })
            })
            .unwrap_or(self.target_position);
        self.enemy_projectile_delay_from_points(self.target_position, target, projectile_speed)
    }

    fn build_actor_perspective_view(
        &self,
        controller_identity: ChampionControllerIdentity,
        controlled_actor_id: &str,
    ) -> Option<ChampionPerspectiveView> {
        let controlled_actor_snapshot = self.actor_control_snapshot(controlled_actor_id)?;
        let action_runtime_state = self.actor_action_runtime_state(controlled_actor_id)?;
        if controlled_actor_id == self.controlled_champion_world_actor_id {
            return Some(build_champion_perspective_view(
                ChampionControlPerspectiveBuildInput {
                    now_seconds: self.time,
                    controller_identity,
                    controlled_actor_id,
                    controlled_actor_snapshot,
                    controlled_actor_ability_loadout: &self.controlled_champion_ability_loadout,
                    controlled_actor_runtime_state: &action_runtime_state,
                    world_state: &self.world_state,
                },
            ));
        }

        let enemy_ability_loadout = self.enemy_actor_ability_loadout(controlled_actor_id)?;
        Some(build_champion_perspective_view(
            ChampionControlPerspectiveBuildInput {
                now_seconds: self.time,
                controller_identity,
                controlled_actor_id,
                controlled_actor_snapshot,
                controlled_actor_ability_loadout: &enemy_ability_loadout,
                controlled_actor_runtime_state: &action_runtime_state,
                world_state: &self.world_state,
            },
        ))
    }

    fn actor_control_snapshot(
        &self,
        controlled_actor_id: &str,
    ) -> Option<ChampionActorControlSnapshot> {
        if controlled_actor_id == self.controlled_champion_world_actor_id {
            return Some(self.controlled_champion_control_snapshot());
        }
        self.enemy_actor_control_snapshot(controlled_actor_id)
    }

    fn actor_action_runtime_state(
        &self,
        controlled_actor_id: &str,
    ) -> Option<ChampionActionRuntimeState> {
        if controlled_actor_id == self.controlled_champion_world_actor_id {
            return Some(self.controlled_champion_action_runtime_state());
        }
        self.enemy_actor_action_runtime_state(controlled_actor_id)
    }

    fn controlled_champion_control_snapshot(&self) -> ChampionActorControlSnapshot {
        let world_position = self
            .world_state
            .actor_position(&self.controlled_champion_world_actor_id)
            .unwrap_or(WorldActorPosition {
                x: self.target_position.x,
                y: self.target_position.y,
            });
        ChampionActorControlSnapshot {
            position: world_position,
            health_ratio: if self.max_health > 0.0 {
                (self.health / self.max_health).clamp(0.0, 1.0)
            } else {
                0.0
            },
            vision_radius: self.controlled_champion_controller_vision_radius,
        }
    }

    fn enemy_actor_control_snapshot(
        &self,
        controlled_actor_id: &str,
    ) -> Option<ChampionActorControlSnapshot> {
        let enemy_index = self.resolve_enemy_index_by_actor_id(controlled_actor_id)?;
        let enemy_state = self.enemy_state.get(enemy_index)?;
        let world_position = self
            .world_state
            .actor_position(controlled_actor_id)
            .unwrap_or(WorldActorPosition {
                x: enemy_state.position.x,
                y: enemy_state.position.y,
            });
        Some(ChampionActorControlSnapshot {
            position: world_position,
            health_ratio: if enemy_state.max_health > 0.0 {
                (enemy_state.health / enemy_state.max_health).clamp(0.0, 1.0)
            } else {
                0.0
            },
            vision_radius: self.controlled_champion_controller_vision_radius,
        })
    }

    fn controlled_champion_action_runtime_state(&self) -> ChampionActionRuntimeState {
        let mut ability_cast_range_by_id = HashMap::new();
        if !self.cast_profile.offensive_primary_ability_id.is_empty() {
            ability_cast_range_by_id.insert(
                self.cast_profile.offensive_primary_ability_id.clone(),
                self.cast_profile.offensive_primary_range.max(0.0),
            );
        }
        if !self.cast_profile.offensive_secondary_ability_id.is_empty() {
            ability_cast_range_by_id.insert(
                self.cast_profile.offensive_secondary_ability_id.clone(),
                self.cast_profile.offensive_secondary_range.max(0.0),
            );
        }
        if !self.cast_profile.offensive_ultimate_ability_id.is_empty() {
            ability_cast_range_by_id.insert(
                self.cast_profile.offensive_ultimate_ability_id.clone(),
                self.cast_profile.offensive_ultimate_range.max(0.0),
            );
        }
        if !self.cast_profile.defensive_ability_two_id.is_empty() {
            ability_cast_range_by_id.insert(
                self.cast_profile.defensive_ability_two_id.clone(),
                self.pool_effect_range.max(0.0),
            );
        }

        let mut item_active_ready_at_seconds_by_id = HashMap::new();
        let mut item_active_cast_range_by_id = HashMap::new();
        if self.stasis_item_available {
            item_active_ready_at_seconds_by_id
                .insert(STASIS_ITEM_ACTIVE_ID.to_string(), self.stasis_item_ready_at);
            item_active_cast_range_by_id.insert(STASIS_ITEM_ACTIVE_ID.to_string(), 0.0);
        }
        if self.emergency_shield_item_available {
            item_active_ready_at_seconds_by_id.insert(
                EMERGENCY_SHIELD_ITEM_ACTIVE_ID.to_string(),
                self.emergency_shield_item_ready_at,
            );
            item_active_cast_range_by_id.insert(EMERGENCY_SHIELD_ITEM_ACTIVE_ID.to_string(), 0.0);
        }

        let cast_lock_remaining = self.combat_primitives.cast_lock().remaining();
        let lock_remaining_seconds = cast_lock_remaining
            .windup_seconds
            .max(cast_lock_remaining.channel_seconds)
            .max(cast_lock_remaining.lockout_seconds)
            .max(0.0);
        let lock_until_seconds = (self.time + lock_remaining_seconds)
            .max(self.stunned_until)
            .max(self.pool_until)
            .max(self.stasis_until)
            .max(self.revive_lockout_until);

        ChampionActionRuntimeState {
            ability_ready_at_seconds_by_id: self.controlled_champion_ability_ready_at.clone(),
            ability_cast_range_by_id,
            item_active_ready_at_seconds_by_id,
            item_active_cast_range_by_id,
            basic_attack_ready_at_seconds: self
                .controlled_champion_next_attack_ready_at()
                .unwrap_or(self.time),
            basic_attack_range: self.controlled_champion_behavior.attack_range.max(0.0),
            movement_locked_until_seconds: lock_until_seconds,
            cast_locked_until_seconds: lock_until_seconds,
        }
    }

    fn enemy_actor_action_runtime_state(
        &self,
        controlled_actor_id: &str,
    ) -> Option<ChampionActionRuntimeState> {
        let enemy_index = self.resolve_enemy_index_by_actor_id(controlled_actor_id)?;
        let enemy_state = self.enemy_state.get(enemy_index)?;
        let mut action_runtime_state = ChampionActionRuntimeState {
            basic_attack_ready_at_seconds: self.time,
            basic_attack_range: enemy_state.behavior.attack_range.max(0.0),
            ..ChampionActionRuntimeState::default()
        };

        let lock_until_seconds = enemy_state
            .respawn_at
            .unwrap_or(self.time)
            .max(enemy_state.stunned_until)
            .max(enemy_state.stasis_until);
        action_runtime_state.movement_locked_until_seconds = lock_until_seconds;
        action_runtime_state.cast_locked_until_seconds = lock_until_seconds;
        Some(action_runtime_state)
    }

    fn enemy_actor_ability_loadout(
        &self,
        controlled_actor_id: &str,
    ) -> Option<ActorAbilityLoadout> {
        let enemy_index = self.resolve_enemy_index_by_actor_id(controlled_actor_id)?;
        let enemy_state = self.enemy_state.get(enemy_index)?;
        Some(default_champion_ability_loadout(
            &enemy_state.enemy.base.name,
        ))
    }

    fn resolve_enemy_index_by_actor_id(&self, actor_id: &str) -> Option<usize> {
        self.enemy_state
            .iter()
            .position(|enemy_state| enemy_state.enemy.id == actor_id)
    }

    fn ability_id_for_slot(&self, ability_slot: AbilitySlotKey) -> Option<&str> {
        self.controlled_champion_ability_loadout
            .slot_bindings()
            .into_iter()
            .find(|(slot, _)| *slot == ability_slot)
            .map(|(_, ability_id)| ability_id)
    }

    fn next_controlled_champion_action_request_sequence(&mut self) -> u64 {
        self.controlled_champion_next_action_request_sequence = self
            .controlled_champion_next_action_request_sequence
            .wrapping_add(1);
        self.controlled_champion_next_action_request_sequence
    }

    // Controller requests are applied on a fixed deterministic tick delay so fast-forward and
    // replay channels preserve authoritative command cadence.
    fn next_controlled_champion_action_request_execute_tick(&self) -> u64 {
        self.controlled_champion_current_tick_index
            .saturating_add(self.controlled_champion_request_fixed_tick_delay)
    }

    fn record_controlled_champion_action_status_report(
        &mut self,
        status_report: ChampionActionStatusReport,
    ) {
        self.controlled_champion_recent_action_status_reports
            .push_back(status_report);
        while self.controlled_champion_recent_action_status_reports.len()
            > ACTION_STATUS_REPORT_BUFFER_LIMIT
        {
            self.controlled_champion_recent_action_status_reports
                .pop_front();
        }
    }
}

use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(in crate::engine) fn resolve_enemy_champion_script_event(
        &mut self,
        idx: usize,
        script_event: ChampionScriptEvent,
        epoch: u64,
    ) {
        self.resolve_enemy_champion_script_event_internal(idx, script_event, epoch, false);
    }

    pub(in crate::engine) fn resolve_enemy_champion_script_event_for_manual_command(
        &mut self,
        idx: usize,
        script_event: ChampionScriptEvent,
        epoch: u64,
    ) {
        self.resolve_enemy_champion_script_event_internal(idx, script_event, epoch, true);
    }

    // Manual-control actors must not execute autonomous script cadence. The dedicated manual
    // command channel can still invoke the same resolver to preserve cooldown/timing semantics.
    fn resolve_enemy_champion_script_event_internal(
        &mut self,
        idx: usize,
        script_event: ChampionScriptEvent,
        epoch: u64,
        allow_manual_control_override: bool,
    ) {
        if idx >= self.enemy_count()
            || !self.enemy_script_epoch_matches(idx, epoch)
            || !self.enemy_can_take_actions(idx)
        {
            return;
        }
        let Some(enemy_actor_id) = self
            .enemy_state
            .get(idx)
            .map(|state| state.enemy.id.as_str())
        else {
            return;
        };
        if !allow_manual_control_override
            && self.enemy_actor_manual_control_mode_enabled(enemy_actor_id)
        {
            return;
        }
        let script_ready_at = self.enemy_script_event_ready_at_or_zero(idx, script_event);
        if self.time + 1e-9 < script_ready_at {
            return;
        }
        let Some(champion_name) = self.enemy_name(idx) else {
            return;
        };
        let distance_to_target = self.distance_to_target(idx);
        let target_current_health = self.health;
        let target_max_health = self.max_health;
        let now = self.time;
        let actions = self.execute_enemy_script_event_actions(
            idx,
            script_event,
            distance_to_target,
            target_current_health,
            target_max_health,
            now,
        );
        if !actions.is_empty() {
            self.trace_event(
                "champion_script",
                format!(
                    "{} executed {}",
                    champion_name,
                    champion_script_event_label(script_event)
                ),
            );
            if let Some(cooldown_seconds) =
                champion_script_event_cooldown_seconds(&champion_name, script_event)
            {
                let ability_haste = self.enemy_ability_haste_or_urf_default(idx);
                let resolved_cooldown = resolve_stat(
                    StatQuery::CooldownSeconds {
                        base_seconds: cooldown_seconds,
                        source: CooldownMetricSource::Ability,
                    },
                    RuntimeBuffState {
                        ability_haste,
                        item_haste: self.urf.item_haste,
                        cooldown_rate_multiplier: 1.0,
                        ..RuntimeBuffState::default()
                    },
                );
                let next_ready = self.time + resolved_cooldown.max(0.0);
                self.set_enemy_script_event_ready_at(idx, script_event, next_ready);
            }
        }
        self.apply_enemy_script_actions(idx, script_event, epoch, actions);
    }
}

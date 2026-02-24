use super::super::*;

impl ControlledChampionCombatSimulation {
    pub(crate) fn enable_trace(&mut self) {
        self.trace_enabled = true;
        self.trace_events.clear();
        self.trace_next_snapshot_at = 0.0;
        self.emit_trace_snapshots_due();
    }

    pub(crate) fn trace_events(&self) -> &[String] {
        &self.trace_events
    }

    pub(crate) fn controlled_champion_rune_proc_telemetry(
        &self,
    ) -> Vec<ChampionRuneProcTelemetryEntry> {
        describe_rune_proc_telemetry(&self.controlled_champion_combat_runtime)
    }

    pub(in crate::engine) fn trace_event(&mut self, kind: &str, details: String) {
        if !self.trace_enabled {
            return;
        }
        self.trace_events
            .push(format!("{:.3}s [{}] {}", self.time, kind, details));
    }

    fn emit_trace_snapshot(&mut self, checkpoint_seconds: f64) {
        if !self.trace_enabled {
            return;
        }
        let snapshot = self.collect_state_snapshot_summary(checkpoint_seconds);
        self.trace_event("state_snapshot", snapshot);
    }

    pub(in crate::engine) fn emit_trace_snapshots_due(&mut self) {
        if !self.trace_enabled {
            return;
        }
        let interval = self.trace_snapshot_interval_seconds.max(0.1);
        while self.time + 1e-9 >= self.trace_next_snapshot_at {
            let checkpoint = self.trace_next_snapshot_at;
            self.emit_trace_snapshot(checkpoint);
            self.trace_next_snapshot_at += interval;
        }
    }
}

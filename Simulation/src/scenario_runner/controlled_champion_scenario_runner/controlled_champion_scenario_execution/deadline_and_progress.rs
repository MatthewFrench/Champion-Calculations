use super::*;

pub(super) struct ControlledChampionRuntimeDeadlines {
    run_start: Instant,
    popcorn_window: Option<Duration>,
    popcorn_min_relative_improvement: f64,
    hard_deadline_state: Arc<Mutex<Option<Instant>>>,
    progress_state: Arc<Mutex<SignificantProgressState>>,
}

impl ControlledChampionRuntimeDeadlines {
    pub(super) fn new(
        run_start: Instant,
        popcorn_window: Option<Duration>,
        popcorn_min_relative_improvement: f64,
    ) -> Self {
        Self {
            run_start,
            popcorn_window,
            popcorn_min_relative_improvement,
            hard_deadline_state: Arc::new(Mutex::new(None)),
            progress_state: Arc::new(Mutex::new(SignificantProgressState {
                best_overall_score: f64::NEG_INFINITY,
                best_significant_score: f64::NEG_INFINITY,
                significant_events: 0,
                last_significant_at: run_start,
            })),
        }
    }

    pub(super) fn hard_deadline_state(&self) -> Arc<Mutex<Option<Instant>>> {
        Arc::clone(&self.hard_deadline_state)
    }

    pub(super) fn progress_state(&self) -> Arc<Mutex<SignificantProgressState>> {
        Arc::clone(&self.progress_state)
    }

    pub(super) fn coverage_stage_deadline(&self) -> Option<Instant> {
        self.hard_deadline_value()
    }

    pub(super) fn current_deadline(&self) -> Option<Instant> {
        let hard_deadline = self.hard_deadline_value();
        let progress_deadline = self.popcorn_window.map(|window| {
            let last_significant_at = self
                .progress_state
                .lock()
                .ok()
                .map(|state| state.last_significant_at)
                .unwrap_or(self.run_start);
            last_significant_at + window
        });
        match (hard_deadline, progress_deadline) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    pub(super) fn deadline_for_search_type(&self, search_type: &str) -> Option<Instant> {
        if search_type == "coverage_stage" {
            self.coverage_stage_deadline()
        } else {
            self.current_deadline()
        }
    }

    pub(super) fn record_score_progress(&self, score: f64) {
        if !score.is_finite() {
            return;
        }
        if let Ok(mut state) = self.progress_state.lock() {
            let previous_best_overall = state.best_overall_score;
            if score > state.best_overall_score {
                state.best_overall_score = score;
            }
            let significant = if !state.best_significant_score.is_finite() {
                true
            } else {
                let previous_best = previous_best_overall;
                let delta = if previous_best.is_finite() {
                    score - previous_best
                } else {
                    score - state.best_significant_score
                };
                if delta <= 0.0 {
                    false
                } else {
                    let threshold_base = if previous_best.is_finite() {
                        previous_best.abs().max(1e-9)
                    } else {
                        state.best_significant_score.abs().max(1e-9)
                    };
                    let threshold = threshold_base * self.popcorn_min_relative_improvement;
                    delta >= threshold
                }
            };
            if significant {
                state.best_significant_score = score;
                state.last_significant_at = Instant::now();
                state.significant_events += 1;
            }
        }
    }

    fn hard_deadline_value(&self) -> Option<Instant> {
        self.hard_deadline_state
            .lock()
            .ok()
            .and_then(|state| *state)
    }
}

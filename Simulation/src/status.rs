use std::time::{Duration, Instant};

pub(crate) fn deadline_reached(deadline: Option<Instant>) -> bool {
    deadline.is_some_and(|d| Instant::now() >= d)
}

#[derive(Debug)]
pub(crate) struct StatusReporter {
    run_start: Instant,
    interval: Duration,
    next_emit_at: Instant,
}

impl StatusReporter {
    pub(crate) fn new(run_start: Instant, interval: Duration) -> Self {
        Self {
            run_start,
            interval,
            next_emit_at: run_start,
        }
    }

    pub(crate) fn emit(
        &mut self,
        phase: &str,
        progress: Option<(usize, usize)>,
        best_score: Option<f64>,
        note: Option<&str>,
        force: bool,
    ) {
        if !force && Instant::now() < self.next_emit_at {
            return;
        }
        let elapsed = self.run_start.elapsed().as_secs_f64();
        let progress_str = progress
            .map(|(done, total)| {
                if total > 0 {
                    format!(
                        "{} / {} ({:.1}%)",
                        done,
                        total,
                        done as f64 * 100.0 / total as f64
                    )
                } else {
                    format!("{} / {}", done, total)
                }
            })
            .unwrap_or_else(|| "n/a".to_string());
        let best_str = best_score
            .map(|v| format!("{:.4}", v))
            .unwrap_or_else(|| "n/a".to_string());
        let note_str = note.unwrap_or("");
        println!(
            "[status] elapsed={:.1}s phase={} progress={} best={} {}",
            elapsed, phase, progress_str, best_str, note_str
        );
        while self.next_emit_at <= Instant::now() {
            self.next_emit_at += self.interval;
        }
    }
}

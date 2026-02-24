use std::path::{Path, PathBuf};

use super::*;

pub(super) fn format_repo_relative_path(path: &Path) -> String {
    if !path.is_absolute() {
        return path.display().to_string();
    }
    let repository_root = simulation_dir()
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(simulation_dir);
    path.strip_prefix(&repository_root)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

pub(super) fn search_quality_profile_key(
    search_quality_profile: SearchQualityProfile,
) -> &'static str {
    match search_quality_profile {
        SearchQualityProfile::Fast => "fast",
        SearchQualityProfile::Balanced => "balanced",
        SearchQualityProfile::MaximumQuality => "maximum_quality",
    }
}

fn runtime_budget_key(max_runtime_seconds: Option<f64>) -> String {
    match max_runtime_seconds {
        Some(seconds) if seconds > 0.0 => {
            let rounded = seconds.round();
            if (seconds - rounded).abs() < 1e-9 {
                format!("{rounded:.0}s")
            } else {
                format!("{seconds:.1}s")
            }
        }
        _ => "no_hard_cap".to_string(),
    }
}

fn format_seconds_key(seconds: f64) -> String {
    let rounded = seconds.round();
    if (seconds - rounded).abs() < 1e-9 {
        format!("{rounded:.0}s")
    } else {
        format!("{seconds:.1}s")
    }
}

fn format_percent_key(percent: f64) -> String {
    let clamped = percent.max(0.0);
    let rounded = clamped.round();
    let rendered = if (clamped - rounded).abs() < 1e-9 {
        format!("{rounded:.0}")
    } else {
        format!("{clamped:.2}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    };
    rendered.replace('.', "_")
}

fn runtime_stop_key(
    max_runtime_seconds: Option<f64>,
    popcorn_window_seconds: Option<f64>,
    popcorn_min_relative_improvement_percent: f64,
) -> String {
    let budget = runtime_budget_key(max_runtime_seconds);
    match popcorn_window_seconds {
        Some(seconds) if seconds > 0.0 => {
            let min_improvement = format_percent_key(popcorn_min_relative_improvement_percent);
            let popcorn_window = format_seconds_key(seconds);
            if popcorn_window == budget {
                format!("{budget}__popcorn__min_improvement_{min_improvement}pct")
            } else {
                format!("{budget}__popcorn_{popcorn_window}__min_improvement_{min_improvement}pct")
            }
        }
        _ => budget,
    }
}

pub(super) fn default_run_output_directory(
    search_quality_profile: SearchQualityProfile,
    max_runtime_seconds: Option<f64>,
    popcorn_window_seconds: Option<f64>,
    popcorn_min_relative_improvement_percent: f64,
) -> PathBuf {
    simulation_dir()
        .join("output")
        .join("runs")
        .join("controlled_champion")
        .join(search_quality_profile_key(search_quality_profile))
        .join(runtime_stop_key(
            max_runtime_seconds,
            popcorn_window_seconds,
            popcorn_min_relative_improvement_percent,
        ))
}

pub(super) fn default_fixed_loadout_output_directory(
    search_quality_profile: SearchQualityProfile,
    run_label: &str,
) -> PathBuf {
    simulation_dir()
        .join("output")
        .join("runs")
        .join("controlled_champion")
        .join("fixed_loadout")
        .join(search_quality_profile_key(search_quality_profile))
        .join(to_norm_key(run_label))
}

pub(super) fn default_fixed_loadout_rune_sweep_output_directory(
    search_quality_profile: SearchQualityProfile,
    run_label: &str,
) -> PathBuf {
    simulation_dir()
        .join("output")
        .join("runs")
        .join("controlled_champion")
        .join("fixed_loadout_rune_sweep")
        .join(search_quality_profile_key(search_quality_profile))
        .join(to_norm_key(run_label))
}

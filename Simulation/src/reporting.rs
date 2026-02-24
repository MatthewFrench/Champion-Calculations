use anyhow::{Context, Result, bail};
use chrono::{DateTime, Local, Utc};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use std::time::SystemTime;

use crate::scripts::coverage::unmodeled_runtime_item_effect_names;
use crate::search::item_names;

use super::{
    ControlledChampionReportData, LoadoutSelection, ObjectiveScoreBreakdown, mean_std,
    simulation_dir, to_norm_key,
};

const CONTROLLED_CHAMPION_RUN_REPORT_JSON_SCHEMA_VERSION: u32 = 2;

#[cfg(test)]
pub(super) fn default_report_path_for_champion(champion_name: &str) -> PathBuf {
    simulation_dir()
        .join("output")
        .join(format!("{}_run_report.md", to_norm_key(champion_name)))
}

fn format_repo_relative_path(path: &Path) -> String {
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

fn comma_separated_digits(digits: &str) -> String {
    let len = digits.len();
    if len <= 3 {
        return digits.to_string();
    }
    let mut out = String::with_capacity(len + len / 3);
    for (idx, ch) in digits.chars().enumerate() {
        if idx > 0 && (len - idx).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

fn format_usize_with_commas(value: usize) -> String {
    comma_separated_digits(&value.to_string())
}

fn format_f64_with_commas(value: f64, decimals: usize) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let sign = if value.is_sign_negative() { "-" } else { "" };
    let rendered = format!("{:.decimals$}", value.abs());
    if let Some((integer, fraction)) = rendered.split_once('.') {
        format!("{}{}.{}", sign, comma_separated_digits(integer), fraction)
    } else {
        format!("{}{}", sign, comma_separated_digits(&rendered))
    }
}

fn format_percent_display(percent: f64) -> String {
    if !percent.is_finite() {
        return percent.to_string();
    }
    if percent > 0.0 && percent < 0.000001 {
        format!("{percent:.3e}%")
    } else {
        format!("{percent:.6}%")
    }
}

pub(super) fn validate_controlled_champion_selection_labels(
    controlled_champion_name: &str,
    selected_loadout: &LoadoutSelection,
    selection_labels: &[String],
    unmodeled_rune_names: &[String],
) -> Result<()> {
    let expected_runes = selected_loadout.rune_names.len();
    let expected_shards = selected_loadout.shard_stats.len();
    if expected_runes < 6 || expected_shards < 3 {
        bail!(
            "Controlled champion '{}' report invariant violation: expected legal loadout shape with >=6 runes and >=3 shards, got runes={} shards={}",
            controlled_champion_name,
            expected_runes,
            expected_shards
        );
    }
    let rune_count = selection_labels
        .iter()
        .filter(|label| label.starts_with("Rune: "))
        .count();
    let shard_count = selection_labels
        .iter()
        .filter(|label| label.starts_with("Shard "))
        .count();
    let accounted_runes = rune_count + unmodeled_rune_names.len();
    if accounted_runes < expected_runes {
        bail!(
            "Controlled champion '{}' report invariant violation: expected {} runes accounted between labels and unmodeled list, got labels={} unmodeled={} (selection labels {}).",
            controlled_champion_name,
            expected_runes,
            rune_count,
            unmodeled_rune_names.len(),
            selection_labels.len()
        );
    }
    if shard_count > expected_shards {
        bail!(
            "Controlled champion '{}' report invariant violation: shard labels exceed selected shards (labels={} selected={}).",
            controlled_champion_name,
            shard_count,
            expected_shards
        );
    }
    Ok(())
}

mod controlled_champion_report_json_writer;
mod controlled_champion_report_markdown_writer;

#[cfg(test)]
use self::controlled_champion_report_json_writer::report_rune_proc_telemetry_json;
pub(super) use self::controlled_champion_report_json_writer::write_controlled_champion_report_json;
pub(super) use self::controlled_champion_report_markdown_writer::write_controlled_champion_report_markdown;

#[cfg(test)]
#[path = "tests/reporting_tests.rs"]
mod tests;

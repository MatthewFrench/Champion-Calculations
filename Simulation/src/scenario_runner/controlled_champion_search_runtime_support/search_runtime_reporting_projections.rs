use serde_json::{Value, json};

use super::*;

pub(in crate::scenario_runner) fn append_rune_proc_telemetry_markdown_entries(
    content: &mut String,
    entry_prefix: &str,
    detail_prefix: &str,
    entries: &[ChampionRuneProcTelemetryEntry],
    total_damage: f64,
    total_healing: f64,
) {
    fn share_percent(part: f64, total: f64) -> f64 {
        if total > 0.0 {
            (part.max(0.0) / total) * 100.0
        } else {
            0.0
        }
    }

    if entries.is_empty() {
        content.push_str(&format!("{entry_prefix}none\n"));
        return;
    }
    for entry in entries {
        let damage_share_percent = share_percent(entry.bonus_damage, total_damage);
        let healing_share_percent = share_percent(entry.bonus_healing, total_healing);
        content.push_str(&format!(
            "{entry_prefix}{}:\n{detail_prefix}- Procs: `{}`\n{detail_prefix}- Attempts: `{}`\n{detail_prefix}- Eligible: `{}`\n{detail_prefix}- Proc rate (vs attempts): `{:.1}%`\n{detail_prefix}- Proc rate (vs eligible): `{:.1}%`\n{detail_prefix}- Bonus damage: `{:.2}` ({:.2}% share)\n{detail_prefix}- Bonus healing: `{:.2}` ({:.2}% share)\n",
            entry.rune_name,
            entry.proc_count,
            entry.attempt_count,
            entry.eligible_count,
            entry.proc_attempt_rate * 100.0,
            entry.proc_eligible_rate * 100.0,
            entry.bonus_damage,
            damage_share_percent,
            entry.bonus_healing,
            healing_share_percent
        ));
        if !entry.source_breakdown.is_empty() {
            content.push_str(&format!("{detail_prefix}- Sources:\n"));
            let source_prefix = format!("{detail_prefix}  ");
            let source_detail_prefix = format!("{source_prefix}  ");
            for source in &entry.source_breakdown {
                content.push_str(&format!(
                    "{}- {}:\n{}- Procs: `{}`\n{}- Attempts: `{}`\n{}- Eligible: `{}`\n{}- Proc rate (vs attempts): `{:.1}%`\n{}- Proc rate (vs eligible): `{:.1}%`\n{}- Bonus damage: `{:.2}`\n{}- Bonus healing: `{:.2}`\n",
                    source_prefix,
                    source.source,
                    source_detail_prefix,
                    source.proc_count,
                    source_detail_prefix,
                    source.attempt_count,
                    source_detail_prefix,
                    source.eligible_count,
                    source_detail_prefix,
                    source.proc_attempt_rate * 100.0,
                    source_detail_prefix,
                    source.proc_eligible_rate * 100.0,
                    source_detail_prefix,
                    source.bonus_damage,
                    source_detail_prefix,
                    source.bonus_healing
                ));
            }
        }
    }
}

pub(in crate::scenario_runner) fn rune_proc_telemetry_json(
    entries: &[ChampionRuneProcTelemetryEntry],
    total_damage: f64,
    total_healing: f64,
) -> Vec<Value> {
    entries
        .iter()
        .map(|entry| {
            let damage_share = if total_damage > 0.0 {
                entry.bonus_damage.max(0.0) / total_damage
            } else {
                0.0
            };
            let healing_share = if total_healing > 0.0 {
                entry.bonus_healing.max(0.0) / total_healing
            } else {
                0.0
            };
            json!({
                "rune_name": entry.rune_name,
                "proc_count": entry.proc_count,
                "attempt_count": entry.attempt_count,
                "eligible_count": entry.eligible_count,
                "proc_attempt_rate": entry.proc_attempt_rate,
                "proc_eligible_rate": entry.proc_eligible_rate,
                "opportunity_count": entry.eligible_count,
                "proc_opportunity_rate": entry.proc_eligible_rate,
                "bonus_damage": entry.bonus_damage,
                "bonus_damage_share": damage_share,
                "bonus_healing": entry.bonus_healing,
                "bonus_healing_share": healing_share,
                "source_breakdown": entry.source_breakdown.iter().map(|source| {
                    json!({
                        "source": source.source,
                        "proc_count": source.proc_count,
                        "attempt_count": source.attempt_count,
                        "eligible_count": source.eligible_count,
                        "proc_attempt_rate": source.proc_attempt_rate,
                        "proc_eligible_rate": source.proc_eligible_rate,
                        "opportunity_count": source.eligible_count,
                        "proc_opportunity_rate": source.proc_eligible_rate,
                        "bonus_damage": source.bonus_damage,
                        "bonus_healing": source.bonus_healing
                    })
                }).collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>()
}

pub(in crate::scenario_runner) fn structured_trace_event(line: &str) -> Value {
    let (header, multiline_details) = match line.split_once('\n') {
        Some((head, details)) => (head, Some(details)),
        None => (line, None),
    };

    let mut timestamp_seconds = None::<f64>;
    let mut event_type = "unknown".to_string();
    let mut details = header.to_string();

    if let Some((time_part, rest)) = header.split_once("s [") {
        timestamp_seconds = time_part.parse::<f64>().ok();
        if let Some((kind, event_details)) = rest.split_once("] ") {
            event_type = kind.to_string();
            details = event_details.to_string();
        } else if let Some((kind, event_details)) = rest.split_once(']') {
            event_type = kind.to_string();
            details = event_details.trim_start().to_string();
        }
    }

    if let Some(extra) = multiline_details {
        if details.is_empty() {
            details = extra.to_string();
        } else {
            details.push('\n');
            details.push_str(extra);
        }
    }

    json!({
        "timestamp_seconds": timestamp_seconds,
        "event_type": event_type,
        "details": details,
        "raw": line,
    })
}

pub(in crate::scenario_runner) fn build_enemy_similarity_notes(
    profiles: &[EnemyDerivedCombatStats],
) -> Vec<String> {
    let mut pair_notes = Vec::new();
    for i in 0..profiles.len() {
        for j in (i + 1)..profiles.len() {
            let a = &profiles[i];
            let b = &profiles[j];
            let attack_damage_close = (a.attack_damage - b.attack_damage).abs() <= 8.0;
            let interval_close =
                (a.attack_interval_seconds - b.attack_interval_seconds).abs() <= 0.10;
            let range_close = (a.attack_range - b.attack_range).abs() <= 40.0;
            if attack_damage_close && interval_close && range_close {
                pair_notes.push(format!(
                    "{} and {} have very similar auto profiles (AD {:.1}/{:.1}, interval {:.3}/{:.3}, range {:.0}/{:.0}).",
                    a.champion,
                    b.champion,
                    a.attack_damage,
                    b.attack_damage,
                    a.attack_interval_seconds,
                    b.attack_interval_seconds,
                    a.attack_range,
                    b.attack_range
                ));
            }
        }
    }

    if pair_notes.is_empty() {
        return Vec::new();
    }

    let mut out = Vec::new();
    out.push(format!(
        "Detected {} pair(s) of enemy auto profiles that are unusually similar; verify presets/loadout ingestion if this looks incorrect.",
        pair_notes.len()
    ));
    out.extend(pair_notes.into_iter().take(8));
    out
}

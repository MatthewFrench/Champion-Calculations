use super::*;
use crate::{CombatOutcome, ObjectiveScoreBreakdown};
use chrono::{DateTime, Local, Utc};

pub(super) struct HeaderAndObjectiveSectionInput<'a> {
    pub(super) generated_local: DateTime<Local>,
    pub(super) generated_utc: DateTime<Utc>,
    pub(super) scenario_path_display: &'a str,
    pub(super) controlled_champion_name: &'a str,
    pub(super) best_score: f64,
    pub(super) best_outcome: &'a CombatOutcome,
    pub(super) champion_level: usize,
    pub(super) max_time_seconds: f64,
    pub(super) best_score_breakdown: ObjectiveScoreBreakdown,
}

fn append_objective_score_breakdown_block(
    content: &mut String,
    title: &str,
    breakdown: ObjectiveScoreBreakdown,
) {
    let push_component = |content: &mut String,
                          label: &str,
                          weight: f64,
                          normalized_ratio: f64,
                          contribution: f64,
                          impact_percent: f64| {
        let delta_vs_weight = impact_percent - weight * 100.0;
        content.push_str(&format!(
            "- {}: weight `{:.2}` | normalized `{:.4}` | contribution `{:.4}` | impact `{:.2}%` | delta vs weight `{:+.2}pp`\n",
            label,
            weight,
            normalized_ratio,
            contribution,
            impact_percent,
            delta_vs_weight
        ));
    };

    content.push_str(&format!("### {}\n", title));
    content.push_str(&format!(
        "- Weighted-mean score: `{:.4}`\n- Worst-case scenario score: `{:.4}`\n- Worst-case blend weight: `{:.2}`\n- Final blended objective score: `{:.4}`\n",
        breakdown.weighted_mean_score,
        breakdown.worst_case_score,
        breakdown.worst_case_weight,
        breakdown.final_score
    ));
    push_component(
        content,
        "survival",
        breakdown.survival.weight,
        breakdown.survival.normalized_ratio,
        breakdown.survival.contribution,
        breakdown.survival.impact_percent,
    );
    push_component(
        content,
        "damage",
        breakdown.damage.weight,
        breakdown.damage.normalized_ratio,
        breakdown.damage.contribution,
        breakdown.damage.impact_percent,
    );
    push_component(
        content,
        "healing",
        breakdown.healing.weight,
        breakdown.healing.normalized_ratio,
        breakdown.healing.contribution,
        breakdown.healing.impact_percent,
    );
    push_component(
        content,
        "enemy_kills",
        breakdown.enemy_kills.weight,
        breakdown.enemy_kills.normalized_ratio,
        breakdown.enemy_kills.contribution,
        breakdown.enemy_kills.impact_percent,
    );
    push_component(
        content,
        "invulnerable_seconds",
        breakdown.invulnerable_seconds.weight,
        breakdown.invulnerable_seconds.normalized_ratio,
        breakdown.invulnerable_seconds.contribution,
        breakdown.invulnerable_seconds.impact_percent,
    );
    content.push('\n');
}

pub(super) fn append_report_header_and_objective_sections(
    content: &mut String,
    input: &HeaderAndObjectiveSectionInput<'_>,
) {
    content.push_str(&format!(
        "# {} URF Run Report\n\n",
        input.controlled_champion_name
    ));
    content.push_str(&format!(
        "- Generated (local): `{}`\n",
        input.generated_local.format("%Y-%m-%d %H:%M:%S %Z")
    ));
    content.push_str(&format!(
        "- Generated (UTC): `{}`\n",
        input.generated_utc.to_rfc3339()
    ));
    content.push_str(&format!(
        "- Scenario: `{}`\n\n",
        input.scenario_path_display
    ));

    content.push_str("## Headline\n");
    let best_damage = format_f64_with_commas(input.best_outcome.damage_dealt, 1);
    let best_healing = format_f64_with_commas(input.best_outcome.healing_done, 1);
    let best_invulnerable_seconds =
        format_f64_with_commas(input.best_outcome.invulnerable_seconds, 2);
    content.push_str(&format!(
        "- Best objective score: **{:.4}**\n- Best outcome:\n  - Time alive: **{:.2}s**\n  - Damage dealt: **{}**\n  - Healing done: **{}**\n  - Enemy kills: **{}**\n  - Invulnerable seconds: **{}s**\n- Best cap survivor: **{}**\n\n",
        input.best_score,
        input.best_outcome.time_alive_seconds,
        best_damage,
        best_healing,
        input.best_outcome.enemy_kills,
        best_invulnerable_seconds,
        input.best_outcome.time_alive_seconds >= input.max_time_seconds - 1e-6,
    ));

    content.push_str(&format!(
        "- Champion level assumption: **{}**\n\n",
        input.champion_level
    ));
    content.push_str("## Objective Score Breakdown\n");
    append_objective_score_breakdown_block(content, "Best Build", input.best_score_breakdown);
}

pub(super) fn append_rune_proc_telemetry_section(
    content: &mut String,
    best_rune_proc_telemetry: &[crate::scripts::champions::ChampionRuneProcTelemetryEntry],
    best_outcome: &CombatOutcome,
) {
    content.push_str("## Rune Proc Telemetry (Best Trace)\n");
    if best_rune_proc_telemetry.is_empty() {
        content.push_str("- No rune procs were recorded during the best-trace replay.\n\n");
        return;
    }
    for entry in best_rune_proc_telemetry {
        let damage_share_percent = if best_outcome.damage_dealt > 0.0 {
            (entry.bonus_damage.max(0.0) / best_outcome.damage_dealt) * 100.0
        } else {
            0.0
        };
        let healing_share_percent = if best_outcome.healing_done > 0.0 {
            (entry.bonus_healing.max(0.0) / best_outcome.healing_done) * 100.0
        } else {
            0.0
        };
        content.push_str(&format!(
            "- {}:\n  - Procs: `{}`\n  - Attempts: `{}`\n  - Eligible: `{}`\n  - Proc rate (vs attempts): `{:.1}%`\n  - Proc rate (vs eligible): `{:.1}%`\n  - Bonus damage: `{:.2}` ({:.2}% share)\n  - Bonus healing: `{:.2}` ({:.2}% share)\n",
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
            content.push_str("  - Sources:\n");
            for source in &entry.source_breakdown {
                content.push_str(&format!(
                    "    - {}:\n      - Procs: `{}`\n      - Attempts: `{}`\n      - Eligible: `{}`\n      - Proc rate (vs attempts): `{:.1}%`\n      - Proc rate (vs eligible): `{:.1}%`\n      - Bonus damage: `{:.2}`\n      - Bonus healing: `{:.2}`\n",
                    source.source,
                    source.proc_count,
                    source.attempt_count,
                    source.eligible_count,
                    source.proc_attempt_rate * 100.0,
                    source.proc_eligible_rate * 100.0,
                    source.bonus_damage,
                    source.bonus_healing
                ));
            }
        }
    }
    content.push('\n');
}

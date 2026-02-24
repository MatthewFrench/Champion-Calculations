use super::*;

#[derive(Debug, Clone)]
pub(super) struct RuneSweepEntry {
    pub(super) keystone_name: String,
    pub(super) loadout_selection: LoadoutSelection,
    pub(super) objective_score: f64,
    pub(super) outcome: CombatOutcome,
    pub(super) objective_breakdown: ObjectiveScoreBreakdown,
    pub(super) rune_proc_telemetry: Vec<crate::scripts::champions::ChampionRuneProcTelemetryEntry>,
    pub(super) seed_repeat_scores: Vec<f64>,
    pub(super) seed_repeat_values: Vec<u64>,
}

pub(super) fn average_combat_outcomes(outcomes: &[CombatOutcome]) -> CombatOutcome {
    if outcomes.is_empty() {
        return CombatOutcome::default();
    }
    let outcome_count = outcomes.len() as f64;
    CombatOutcome {
        time_alive_seconds: outcomes
            .iter()
            .map(|outcome| outcome.time_alive_seconds)
            .sum::<f64>()
            / outcome_count,
        damage_dealt: outcomes
            .iter()
            .map(|outcome| outcome.damage_dealt)
            .sum::<f64>()
            / outcome_count,
        healing_done: outcomes
            .iter()
            .map(|outcome| outcome.healing_done)
            .sum::<f64>()
            / outcome_count,
        enemy_kills: ((outcomes
            .iter()
            .map(|outcome| outcome.enemy_kills as f64)
            .sum::<f64>()
            / outcome_count)
            .round()
            .max(0.0)) as usize,
        invulnerable_seconds: outcomes
            .iter()
            .map(|outcome| outcome.invulnerable_seconds)
            .sum::<f64>()
            / outcome_count,
    }
}

fn average_component_impacts(impacts: &[ObjectiveComponentImpact]) -> ObjectiveComponentImpact {
    if impacts.is_empty() {
        return ObjectiveComponentImpact::default();
    }
    let impact_count = impacts.len() as f64;
    ObjectiveComponentImpact {
        weight: impacts.iter().map(|impact| impact.weight).sum::<f64>() / impact_count,
        normalized_ratio: impacts
            .iter()
            .map(|impact| impact.normalized_ratio)
            .sum::<f64>()
            / impact_count,
        contribution: impacts
            .iter()
            .map(|impact| impact.contribution)
            .sum::<f64>()
            / impact_count,
        impact_percent: impacts
            .iter()
            .map(|impact| impact.impact_percent)
            .sum::<f64>()
            / impact_count,
    }
}

pub(super) fn average_objective_breakdowns(
    breakdowns: &[ObjectiveScoreBreakdown],
) -> ObjectiveScoreBreakdown {
    if breakdowns.is_empty() {
        return ObjectiveScoreBreakdown::default();
    }
    let breakdown_count = breakdowns.len() as f64;
    ObjectiveScoreBreakdown {
        weighted_mean_score: breakdowns
            .iter()
            .map(|breakdown| breakdown.weighted_mean_score)
            .sum::<f64>()
            / breakdown_count,
        worst_case_score: breakdowns
            .iter()
            .map(|breakdown| breakdown.worst_case_score)
            .sum::<f64>()
            / breakdown_count,
        worst_case_weight: breakdowns
            .iter()
            .map(|breakdown| breakdown.worst_case_weight)
            .sum::<f64>()
            / breakdown_count,
        final_score: breakdowns
            .iter()
            .map(|breakdown| breakdown.final_score)
            .sum::<f64>()
            / breakdown_count,
        survival: average_component_impacts(
            &breakdowns
                .iter()
                .map(|breakdown| breakdown.survival)
                .collect::<Vec<_>>(),
        ),
        damage: average_component_impacts(
            &breakdowns
                .iter()
                .map(|breakdown| breakdown.damage)
                .collect::<Vec<_>>(),
        ),
        healing: average_component_impacts(
            &breakdowns
                .iter()
                .map(|breakdown| breakdown.healing)
                .collect::<Vec<_>>(),
        ),
        enemy_kills: average_component_impacts(
            &breakdowns
                .iter()
                .map(|breakdown| breakdown.enemy_kills)
                .collect::<Vec<_>>(),
        ),
        invulnerable_seconds: average_component_impacts(
            &breakdowns
                .iter()
                .map(|breakdown| breakdown.invulnerable_seconds)
                .collect::<Vec<_>>(),
        ),
    }
}

use crate::engine::simulate_controlled_champion_combat;

use super::{
    CombatOutcome, Item, LoadoutSelection, ObjectiveComponentImpact, ObjectiveComponentWeights,
    ObjectiveEvalContext, ObjectiveScoreBreakdown, Stats,
};

pub(crate) fn normalized_objective_weights(
    survival: f64,
    damage: f64,
    healing: f64,
    enemy_kills: f64,
    invulnerable_seconds: f64,
) -> ObjectiveComponentWeights {
    let mut s = survival.max(0.0);
    let mut d = damage.max(0.0);
    let mut h = healing.max(0.0);
    let mut k = enemy_kills.max(0.0);
    let mut i = invulnerable_seconds.max(0.0);
    let sum = s + d + h + k + i;
    if sum <= 0.0 {
        s = 1.0;
        d = 0.0;
        h = 0.0;
        k = 0.0;
        i = 0.0;
    } else {
        s /= sum;
        d /= sum;
        h /= sum;
        k /= sum;
        i /= sum;
    }
    ObjectiveComponentWeights {
        survival: s,
        damage: d,
        healing: h,
        enemy_kills: k,
        invulnerable_seconds: i,
    }
}

pub(crate) fn objective_score_from_outcome(
    outcome: CombatOutcome,
    reference: CombatOutcome,
    weights: ObjectiveComponentWeights,
) -> f64 {
    let survival_ref = reference.time_alive_seconds.max(0.01);
    let damage_ref = reference.damage_dealt.max(1.0);
    let healing_ref = reference.healing_done.max(1.0);
    let kills_ref = reference.enemy_kills.max(1) as f64;
    let invulnerable_ref = reference.invulnerable_seconds.max(0.01);
    let survival_component = weights.survival * (outcome.time_alive_seconds / survival_ref);
    let damage_component = weights.damage * (outcome.damage_dealt / damage_ref);
    let healing_component = weights.healing * (outcome.healing_done / healing_ref);
    let enemy_kills_component = weights.enemy_kills * (outcome.enemy_kills as f64 / kills_ref);
    let invulnerable_component =
        weights.invulnerable_seconds * (outcome.invulnerable_seconds / invulnerable_ref);
    survival_component
        + damage_component
        + healing_component
        + enemy_kills_component
        + invulnerable_component
}

#[allow(dead_code)]
pub(crate) fn aggregate_objective_score_and_outcome(
    ctx: &ObjectiveEvalContext<'_>,
    build_items: &[Item],
    bonus_stats: &Stats,
) -> (f64, CombatOutcome) {
    let (score, outcome, _) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            ctx,
            build_items,
            bonus_stats,
            None,
        );
    (score, outcome)
}

pub(crate) fn aggregate_objective_score_and_outcome_with_loadout_selection(
    ctx: &ObjectiveEvalContext<'_>,
    build_items: &[Item],
    bonus_stats: &Stats,
    controlled_champion_loadout_selection: Option<&LoadoutSelection>,
) -> (f64, CombatOutcome) {
    let (score, outcome, _) =
        aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
            ctx,
            build_items,
            bonus_stats,
            controlled_champion_loadout_selection,
        );
    (score, outcome)
}

pub(crate) fn aggregate_objective_score_and_outcome_with_breakdown_and_loadout_selection(
    ctx: &ObjectiveEvalContext<'_>,
    build_items: &[Item],
    bonus_stats: &Stats,
    controlled_champion_loadout_selection: Option<&LoadoutSelection>,
) -> (f64, CombatOutcome, ObjectiveScoreBreakdown) {
    fn component_impact(
        weight: f64,
        contribution: f64,
        final_score: f64,
    ) -> ObjectiveComponentImpact {
        let normalized_ratio = if weight > 0.0 {
            contribution / weight
        } else {
            0.0
        };
        let impact_percent = if final_score.abs() > f64::EPSILON {
            (contribution / final_score) * 100.0
        } else {
            0.0
        };
        ObjectiveComponentImpact {
            weight,
            normalized_ratio,
            contribution,
            impact_percent,
        }
    }

    let mut weighted_score_sum = 0.0;
    let mut weighted_time_sum = 0.0;
    let mut weighted_damage_sum = 0.0;
    let mut weighted_healing_sum = 0.0;
    let mut weighted_kills_sum = 0.0;
    let mut weighted_invulnerable_seconds_sum = 0.0;
    let mut weighted_survival_component_sum = 0.0;
    let mut weighted_damage_component_sum = 0.0;
    let mut weighted_healing_component_sum = 0.0;
    let mut weighted_enemy_kills_component_sum = 0.0;
    let mut weighted_invulnerable_component_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut worst = f64::INFINITY;
    let mut worst_survival_component = 0.0;
    let mut worst_damage_component = 0.0;
    let mut worst_healing_component = 0.0;
    let mut worst_enemy_kills_component = 0.0;
    let mut worst_invulnerable_component = 0.0;
    let mut has_worst = false;

    for (idx, (_, weight, enemy_builds_s)) in ctx.enemy_build_scenarios.iter().enumerate() {
        let w = (*weight).max(0.0);
        if w <= 0.0 {
            continue;
        }
        let outcome = simulate_controlled_champion_combat(
            ctx.controlled_champion_base,
            build_items,
            bonus_stats,
            controlled_champion_loadout_selection,
            None,
            Some(ctx.controlled_champion_stack_overrides),
            enemy_builds_s,
            ctx.sim,
            ctx.urf,
        );
        let reference =
            ctx.scenario_reference_outcomes
                .get(idx)
                .copied()
                .unwrap_or(CombatOutcome {
                    time_alive_seconds: ctx.sim.max_time_seconds.max(1.0),
                    damage_dealt: 1.0,
                    healing_done: 1.0,
                    enemy_kills: 0,
                    invulnerable_seconds: 0.0,
                });
        let survival_ref = reference.time_alive_seconds.max(0.01);
        let damage_ref = reference.damage_dealt.max(1.0);
        let healing_ref = reference.healing_done.max(1.0);
        let kills_ref = reference.enemy_kills.max(1) as f64;
        let invulnerable_ref = reference.invulnerable_seconds.max(0.01);
        let survival_component = ctx.weights.survival * (outcome.time_alive_seconds / survival_ref);
        let damage_component = ctx.weights.damage * (outcome.damage_dealt / damage_ref);
        let healing_component = ctx.weights.healing * (outcome.healing_done / healing_ref);
        let enemy_kills_component =
            ctx.weights.enemy_kills * (outcome.enemy_kills as f64 / kills_ref);
        let invulnerable_component =
            ctx.weights.invulnerable_seconds * (outcome.invulnerable_seconds / invulnerable_ref);
        let scenario_score = survival_component
            + damage_component
            + healing_component
            + enemy_kills_component
            + invulnerable_component;
        weighted_score_sum += w * scenario_score;
        weighted_time_sum += w * outcome.time_alive_seconds;
        weighted_damage_sum += w * outcome.damage_dealt;
        weighted_healing_sum += w * outcome.healing_done;
        weighted_kills_sum += w * outcome.enemy_kills as f64;
        weighted_invulnerable_seconds_sum += w * outcome.invulnerable_seconds;
        weighted_survival_component_sum += w * survival_component;
        weighted_damage_component_sum += w * damage_component;
        weighted_healing_component_sum += w * healing_component;
        weighted_enemy_kills_component_sum += w * enemy_kills_component;
        weighted_invulnerable_component_sum += w * invulnerable_component;
        weight_sum += w;
        if scenario_score < worst {
            worst = scenario_score;
            worst_survival_component = survival_component;
            worst_damage_component = damage_component;
            worst_healing_component = healing_component;
            worst_enemy_kills_component = enemy_kills_component;
            worst_invulnerable_component = invulnerable_component;
            has_worst = true;
        }
    }

    if weight_sum <= 0.0 {
        return (
            0.0,
            CombatOutcome::default(),
            ObjectiveScoreBreakdown {
                worst_case_weight: ctx.worst_case_weight.clamp(0.0, 1.0),
                ..ObjectiveScoreBreakdown::default()
            },
        );
    }

    let mean_score = weighted_score_sum / weight_sum;
    let mean_survival_component = weighted_survival_component_sum / weight_sum;
    let mean_damage_component = weighted_damage_component_sum / weight_sum;
    let mean_healing_component = weighted_healing_component_sum / weight_sum;
    let mean_enemy_kills_component = weighted_enemy_kills_component_sum / weight_sum;
    let mean_invulnerable_component = weighted_invulnerable_component_sum / weight_sum;
    let blended_score = if worst.is_finite() {
        let ww = ctx.worst_case_weight.clamp(0.0, 1.0);
        (1.0 - ww) * mean_score + ww * worst
    } else {
        mean_score
    };
    let mean_outcome = CombatOutcome {
        time_alive_seconds: weighted_time_sum / weight_sum,
        damage_dealt: weighted_damage_sum / weight_sum,
        healing_done: weighted_healing_sum / weight_sum,
        enemy_kills: (weighted_kills_sum / weight_sum).round() as usize,
        invulnerable_seconds: weighted_invulnerable_seconds_sum / weight_sum,
    };
    let worst_case_weight = if has_worst {
        ctx.worst_case_weight.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let blended_survival_component = if has_worst {
        (1.0 - worst_case_weight) * mean_survival_component
            + worst_case_weight * worst_survival_component
    } else {
        mean_survival_component
    };
    let blended_damage_component = if has_worst {
        (1.0 - worst_case_weight) * mean_damage_component
            + worst_case_weight * worst_damage_component
    } else {
        mean_damage_component
    };
    let blended_healing_component = if has_worst {
        (1.0 - worst_case_weight) * mean_healing_component
            + worst_case_weight * worst_healing_component
    } else {
        mean_healing_component
    };
    let blended_enemy_kills_component = if has_worst {
        (1.0 - worst_case_weight) * mean_enemy_kills_component
            + worst_case_weight * worst_enemy_kills_component
    } else {
        mean_enemy_kills_component
    };
    let blended_invulnerable_component = if has_worst {
        (1.0 - worst_case_weight) * mean_invulnerable_component
            + worst_case_weight * worst_invulnerable_component
    } else {
        mean_invulnerable_component
    };
    let breakdown = ObjectiveScoreBreakdown {
        weighted_mean_score: mean_score,
        worst_case_score: if has_worst { worst } else { mean_score },
        worst_case_weight,
        final_score: blended_score,
        survival: component_impact(
            ctx.weights.survival,
            blended_survival_component,
            blended_score,
        ),
        damage: component_impact(ctx.weights.damage, blended_damage_component, blended_score),
        healing: component_impact(
            ctx.weights.healing,
            blended_healing_component,
            blended_score,
        ),
        enemy_kills: component_impact(
            ctx.weights.enemy_kills,
            blended_enemy_kills_component,
            blended_score,
        ),
        invulnerable_seconds: component_impact(
            ctx.weights.invulnerable_seconds,
            blended_invulnerable_component,
            blended_score,
        ),
    };
    (blended_score, mean_outcome, breakdown)
}

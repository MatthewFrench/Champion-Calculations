use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::engine::simulate_controlled_champion_combat;
use crate::scripts::registry::hooks::{
    ChampionStatContext, ItemAssumptionContext, StackNoteContext, apply_item_assumption_hooks,
    finalize_champion_stats_with_hooks, stack_notes_from_hooks,
};

use super::{
    BuildKey, ChampionBase, CombatOutcome, Item, LoadoutSelection, ObjectiveComponentImpact,
    ObjectiveComponentWeights, ObjectiveEvalContext, ObjectiveScoreBreakdown, SimulationConfig,
    Stats, loadout_selection_key,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub(crate) enum StatusEffectKind {
    Stun,
    Silence,
    Root,
    Slow,
    Untargetable,
    Stasis,
    Custom(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum StatusDuration {
    Timed { remaining_seconds: f64 },
    Persistent,
}

impl StatusDuration {
    #[allow(dead_code)]
    pub(crate) fn timed(seconds: f64) -> Self {
        Self::Timed {
            remaining_seconds: seconds.max(0.0),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remaining_seconds(self) -> Option<f64> {
        match self {
            Self::Timed { remaining_seconds } => Some(remaining_seconds),
            Self::Persistent => None,
        }
    }

    fn refresh_from(&mut self, incoming: StatusDuration) {
        match (*self, incoming) {
            (Self::Persistent, _) | (_, Self::Persistent) => *self = Self::Persistent,
            (
                Self::Timed {
                    remaining_seconds: current,
                },
                Self::Timed {
                    remaining_seconds: incoming,
                },
            ) => {
                *self = Self::Timed {
                    remaining_seconds: current.max(incoming),
                };
            }
        }
    }

    fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        if let Self::Timed { remaining_seconds } = self {
            *remaining_seconds = (*remaining_seconds - delta_seconds).max(0.0);
        }
    }

    fn is_expired(self) -> bool {
        matches!(
            self,
            Self::Timed { remaining_seconds } if remaining_seconds <= 0.0
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum StatusPersistence {
    Replace,
    RefreshDuration,
    StackRefreshDuration,
    Independent,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StatusEffect {
    pub kind: StatusEffectKind,
    pub duration: StatusDuration,
    pub stacks: u32,
    pub max_stacks: u32,
    pub persistence: StatusPersistence,
}

impl StatusEffect {
    #[allow(dead_code)]
    pub(crate) fn timed(
        kind: StatusEffectKind,
        duration_seconds: f64,
        stacks: u32,
        persistence: StatusPersistence,
    ) -> Self {
        let mut effect = Self {
            kind,
            duration: StatusDuration::timed(duration_seconds),
            stacks: stacks.max(1),
            max_stacks: u32::MAX,
            persistence,
        };
        effect.clamp_stacks();
        effect
    }

    #[allow(dead_code)]
    pub(crate) fn persistent(
        kind: StatusEffectKind,
        stacks: u32,
        persistence: StatusPersistence,
    ) -> Self {
        let mut effect = Self {
            kind,
            duration: StatusDuration::Persistent,
            stacks: stacks.max(1),
            max_stacks: u32::MAX,
            persistence,
        };
        effect.clamp_stacks();
        effect
    }

    #[allow(dead_code)]
    pub(crate) fn with_max_stacks(mut self, max_stacks: u32) -> Self {
        self.max_stacks = max_stacks.max(1);
        self.clamp_stacks();
        self
    }

    fn clamp_stacks(&mut self) {
        self.max_stacks = self.max_stacks.max(1);
        self.stacks = self.stacks.clamp(1, self.max_stacks);
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct StatusEffectSet {
    effects: Vec<StatusEffect>,
}

impl StatusEffectSet {
    #[allow(dead_code)]
    pub(crate) fn effects(&self) -> &[StatusEffect] {
        &self.effects
    }

    #[allow(dead_code)]
    pub(crate) fn is_active(&self, kind: &StatusEffectKind) -> bool {
        self.effects.iter().any(|effect| &effect.kind == kind)
    }

    #[allow(dead_code)]
    pub(crate) fn total_stacks(&self, kind: &StatusEffectKind) -> u32 {
        self.effects
            .iter()
            .filter(|effect| &effect.kind == kind)
            .map(|effect| effect.stacks)
            .sum()
    }

    pub(crate) fn apply(&mut self, mut incoming: StatusEffect) {
        incoming.clamp_stacks();
        match incoming.persistence {
            StatusPersistence::Independent => self.effects.push(incoming),
            StatusPersistence::Replace => {
                self.effects.retain(|effect| effect.kind != incoming.kind);
                self.effects.push(incoming);
            }
            StatusPersistence::RefreshDuration => {
                if let Some(existing) = self
                    .effects
                    .iter_mut()
                    .find(|effect| effect.kind == incoming.kind)
                {
                    existing.max_stacks = existing.max_stacks.max(incoming.max_stacks);
                    existing.stacks = existing.stacks.max(incoming.stacks);
                    existing.clamp_stacks();
                    existing.duration.refresh_from(incoming.duration);
                } else {
                    self.effects.push(incoming);
                }
            }
            StatusPersistence::StackRefreshDuration => {
                if let Some(existing) = self
                    .effects
                    .iter_mut()
                    .find(|effect| effect.kind == incoming.kind)
                {
                    existing.max_stacks = existing.max_stacks.max(incoming.max_stacks);
                    existing.stacks = existing.stacks.saturating_add(incoming.stacks);
                    existing.clamp_stacks();
                    existing.duration.refresh_from(incoming.duration);
                } else {
                    self.effects.push(incoming);
                }
            }
        }
    }

    pub(crate) fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        for effect in &mut self.effects {
            effect.duration.tick(delta_seconds);
        }
        self.effects.retain(|effect| !effect.duration.is_expired());
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CastLockWindow {
    pub windup_seconds: f64,
    pub channel_seconds: f64,
    pub lockout_seconds: f64,
}

impl CastLockWindow {
    pub(crate) fn new(windup_seconds: f64, channel_seconds: f64, lockout_seconds: f64) -> Self {
        Self {
            windup_seconds: windup_seconds.max(0.0),
            channel_seconds: channel_seconds.max(0.0),
            lockout_seconds: lockout_seconds.max(0.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum CastLockPhase {
    Idle,
    Windup,
    Channel,
    Lockout,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct CastLockState {
    windup_remaining_seconds: f64,
    channel_remaining_seconds: f64,
    lockout_remaining_seconds: f64,
}

impl CastLockState {
    #[allow(dead_code)]
    pub(crate) fn begin(&mut self, window: CastLockWindow) {
        self.windup_remaining_seconds = self.windup_remaining_seconds.max(window.windup_seconds);
        self.channel_remaining_seconds = self.channel_remaining_seconds.max(window.channel_seconds);
        self.lockout_remaining_seconds = self.lockout_remaining_seconds.max(window.lockout_seconds);
    }

    #[allow(dead_code)]
    pub(crate) fn phase(self) -> CastLockPhase {
        if self.windup_remaining_seconds > 0.0 {
            CastLockPhase::Windup
        } else if self.channel_remaining_seconds > 0.0 {
            CastLockPhase::Channel
        } else if self.lockout_remaining_seconds > 0.0 {
            CastLockPhase::Lockout
        } else {
            CastLockPhase::Idle
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_locked(self) -> bool {
        self.phase() != CastLockPhase::Idle
    }

    #[allow(dead_code)]
    pub(crate) fn remaining(self) -> CastLockWindow {
        CastLockWindow {
            windup_seconds: self.windup_remaining_seconds,
            channel_seconds: self.channel_remaining_seconds,
            lockout_seconds: self.lockout_remaining_seconds,
        }
    }

    pub(crate) fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        let mut remaining = delta_seconds;
        let windup_spent = self.windup_remaining_seconds.min(remaining);
        self.windup_remaining_seconds -= windup_spent;
        remaining -= windup_spent;

        let channel_spent = self.channel_remaining_seconds.min(remaining);
        self.channel_remaining_seconds -= channel_spent;
        remaining -= channel_spent;

        let lockout_spent = self.lockout_remaining_seconds.min(remaining);
        self.lockout_remaining_seconds -= lockout_spent;
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CombatPrimitivesState {
    status_effects: StatusEffectSet,
    cast_lock: CastLockState,
}

impl CombatPrimitivesState {
    #[allow(dead_code)]
    pub(crate) fn status_effects(&self) -> &StatusEffectSet {
        &self.status_effects
    }

    pub(crate) fn apply_status(&mut self, effect: StatusEffect) {
        self.status_effects.apply(effect);
    }

    #[allow(dead_code)]
    pub(crate) fn cast_lock(&self) -> CastLockState {
        self.cast_lock
    }

    pub(crate) fn begin_cast_lock(&mut self, window: CastLockWindow) {
        self.cast_lock.begin(window);
    }

    pub(crate) fn tick(&mut self, delta_seconds: f64) {
        if delta_seconds <= 0.0 {
            return;
        }
        self.status_effects.tick(delta_seconds);
        self.cast_lock.tick(delta_seconds);
    }
}

pub(crate) fn is_boots(item: &Item) -> bool {
    item.rank.iter().any(|r| r == "BOOTS")
}

pub(crate) fn cooldown_after_haste(base_seconds: f64, haste: f64) -> f64 {
    base_seconds * (100.0 / (100.0 + haste))
}

pub(crate) fn champion_at_level(base: &ChampionBase, level: usize) -> ChampionBase {
    let lvl = level.max(1) as f64;
    let growth_levels = (lvl - 1.0).max(0.0);
    ChampionBase {
        name: base.name.clone(),
        base_health: base.base_health + base.health_per_level * growth_levels,
        health_per_level: base.health_per_level,
        base_armor: base.base_armor + base.armor_per_level * growth_levels,
        armor_per_level: base.armor_per_level,
        base_magic_resist: base.base_magic_resist + base.magic_resist_per_level * growth_levels,
        magic_resist_per_level: base.magic_resist_per_level,
        base_attack_damage: base.base_attack_damage + base.attack_damage_per_level * growth_levels,
        attack_damage_per_level: base.attack_damage_per_level,
        base_attack_speed: base.base_attack_speed
            * (1.0 + (base.attack_speed_per_level_percent / 100.0) * growth_levels),
        attack_speed_per_level_percent: base.attack_speed_per_level_percent,
        base_attack_range: base.base_attack_range,
        base_attack_projectile_speed: base.base_attack_projectile_speed,
        base_move_speed: base.base_move_speed,
        is_melee: base.is_melee,
    }
}

pub(crate) fn apply_item_assumptions(
    stats: &mut Stats,
    base: &ChampionBase,
    build_items: &[Item],
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
    stack_overrides: Option<&HashMap<String, f64>>,
) {
    let ctx = ItemAssumptionContext {
        champion: base,
        build_items,
        sim,
        current_level,
        acquired_levels,
        stack_overrides,
    };
    apply_item_assumption_hooks(&ctx, stats);
}

pub(crate) fn compute_effective_item_stats_for_build(
    base: &ChampionBase,
    build_items: &[Item],
    bonus_stats: &Stats,
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
    stack_overrides: Option<&HashMap<String, f64>>,
) -> Stats {
    let mut stats = build_item_stats(build_items);
    stats.add(bonus_stats);
    apply_item_assumptions(
        &mut stats,
        base,
        build_items,
        sim,
        current_level,
        acquired_levels,
        stack_overrides,
    );
    stats
}

pub(crate) fn build_stack_notes(
    build_items: &[Item],
    base: &ChampionBase,
    sim: &SimulationConfig,
    current_level: usize,
    acquired_levels: Option<&HashMap<String, usize>>,
    stack_overrides: Option<&HashMap<String, f64>>,
) -> Vec<String> {
    let mut notes = Vec::new();
    for item in build_items {
        let hook_ctx = StackNoteContext {
            champion: base,
            build_items,
            item,
            sim,
            current_level,
            acquired_levels,
            stack_overrides,
        };
        let hook_notes = stack_notes_from_hooks(&hook_ctx);
        let has_explicit_item_note = !hook_notes.is_empty();
        notes.extend(hook_notes);

        if has_explicit_item_note {
            continue;
        }

        let has_stack_text = item
            .passive_effects_text
            .iter()
            .any(|t| t.to_ascii_lowercase().contains("stack"));
        if has_stack_text {
            notes.push(format!(
                "{} has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.",
                item.name
            ));
        }
    }
    notes
}

pub(crate) fn compute_champion_final_stats(base: &ChampionBase, item_stats: &Stats) -> Stats {
    let mut stats = item_stats.clone();
    let hook_ctx = ChampionStatContext {
        champion: base,
        item_stats,
    };
    finalize_champion_stats_with_hooks(&hook_ctx, &mut stats);
    stats.health += base.base_health;
    stats.armor += base.base_armor;
    stats.magic_resist += base.base_magic_resist;
    stats
}

#[allow(dead_code)]
pub(crate) fn compute_vlad_stats(base: &ChampionBase, item_stats: &Stats) -> Stats {
    compute_champion_final_stats(base, item_stats)
}

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

pub(crate) fn build_item_stats(items: &[Item]) -> Stats {
    let mut stats = Stats::default();
    for item in items {
        stats.add(&item.stats);
    }
    stats
}

pub(crate) fn build_from_indices(item_pool: &[Item], build: &[usize]) -> Vec<Item> {
    build.iter().map(|&idx| item_pool[idx].clone()).collect()
}

pub(crate) fn canonical_key(build: &[usize]) -> Vec<usize> {
    let mut key = build.to_vec();
    key.sort_unstable();
    key
}

pub(crate) fn canonical_build_candidate(mut candidate: BuildKey) -> BuildKey {
    candidate.item_indices.sort_unstable();
    candidate
}

pub(crate) fn build_key_cache_string(key: &BuildKey) -> String {
    let items = key
        .item_indices
        .iter()
        .map(|idx| idx.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let loadout = loadout_selection_key(&key.loadout_selection);
    format!("i={items}|l={loadout}")
}

pub(crate) fn next_u64(seed: &mut u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    *seed
}

pub(crate) fn runtime_random_seed() -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let mut hasher = DefaultHasher::new();
    now.as_secs().hash(&mut hasher);
    now.subsec_nanos().hash(&mut hasher);
    now.as_nanos().hash(&mut hasher);
    std::process::id().hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    let stack_entropy = (&now as *const _) as usize;
    stack_entropy.hash(&mut hasher);
    hasher.finish().max(1)
}

pub(crate) fn rand_index(seed: &mut u64, upper: usize) -> usize {
    if upper <= 1 {
        return 0;
    }
    (next_u64(seed) as usize) % upper
}

pub(crate) fn rand_f64(seed: &mut u64) -> f64 {
    let bits = next_u64(seed) >> 11;
    (bits as f64) / ((1u64 << 53) as f64)
}

pub(crate) fn shuffle_usize(slice: &mut [usize], seed: &mut u64) {
    if slice.len() <= 1 {
        return;
    }
    for i in (1..slice.len()).rev() {
        let j = rand_index(seed, i + 1);
        slice.swap(i, j);
    }
}

pub(crate) fn can_add_item_to_build(item_pool: &[Item], build: &[usize], item_idx: usize) -> bool {
    if build.contains(&item_idx) {
        return false;
    }
    if is_boots(&item_pool[item_idx]) && build.iter().any(|&i| is_boots(&item_pool[i])) {
        return false;
    }
    true
}

pub(crate) fn random_valid_build(
    item_pool: &[Item],
    max_items: usize,
    seed: &mut u64,
) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..item_pool.len()).collect();
    shuffle_usize(&mut indices, seed);
    let mut build = Vec::with_capacity(max_items);
    for item_idx in indices {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &build, item_idx) {
            build.push(item_idx);
        }
    }
    build
}

pub(crate) fn repair_build(
    item_pool: &[Item],
    build: &mut Vec<usize>,
    max_items: usize,
    seed: &mut u64,
) {
    let mut deduped = Vec::with_capacity(max_items);
    for &item_idx in build.iter() {
        if deduped.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, &deduped, item_idx) {
            deduped.push(item_idx);
        }
    }
    *build = deduped;

    if build.len() >= max_items {
        return;
    }
    let mut all_indices: Vec<usize> = (0..item_pool.len()).collect();
    shuffle_usize(&mut all_indices, seed);
    for item_idx in all_indices {
        if build.len() >= max_items {
            break;
        }
        if can_add_item_to_build(item_pool, build, item_idx) {
            build.push(item_idx);
        }
    }
}

pub(crate) fn mean_std(values: &[f64]) -> (f64, f64) {
    if values.is_empty() {
        return (0.0, 0.0);
    }
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let var = values
        .iter()
        .map(|v| {
            let d = *v - mean;
            d * d
        })
        .sum::<f64>()
        / values.len() as f64;
    (mean, var.sqrt())
}

#[cfg(test)]
#[path = "tests/core_tests.rs"]
mod tests;

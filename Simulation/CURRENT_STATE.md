# Current State Snapshot (2026-02-19)

This file is a concise handoff for developers and AI agents.

## Primary Intent
- Keep the simulation engine generic and data-driven.
- Keep champion/item/rune-specific behavior in scripts and canonical data files.
- Use multi-algorithm search to find high-value item + rune loadouts and build orders.

## What Is Implemented Now
- Rust simulation engine with fixed server-tick stepping (URF default 30 Hz).
- Generic actor/champion abstractions for controlled champion and opponents.
- Champion script dispatch under `src/scripts/champions/`.
- Engine-facing controlled champion script facade under `src/scripts/champions/controlled_champion.rs`.
- Controlled champion basic attacks now execute through recurring start/windup/hit events (hitbox/projectile-aware), using shared runtime attack-speed/on-hit effect paths.
- Shared loadout runtime now includes generic combat-time rune trigger hooks (on-hit, ability-hit, outgoing-damage healing, immobilize-triggered effects).
- Combat-time keystone coverage now includes Press the Attack, Fleet Footwork, Conqueror, and Aftershock.
- CLI primary modes are `controlled_champion`, `controlled_champion_fixed_loadout`, and `controlled_champion_step` (`vladimir`/`vladimir_step` aliases still accepted).
- Item and runtime loadout script hooks under `src/scripts/items/` and `src/scripts/runtime/`.
- Shared runtime stat-query resolution for cooldowns and scalar combat metrics (incoming damage taken, healing, movement speed, outgoing bonus-ability damage) from base data + runtime buff state.
- Strict scenario schema and minimal scenario setup under `Simulation/scenarios/`.
- URF item allow-list restrictions and enemy URF preset loadouts.
- Parallelized search/ranking with persistent score cache.
- Top-level search orchestration is parallelized for ensemble seeds and portfolio strategies, and strategy-elite/adaptive generation is parallelized with deterministic merge ordering.
- Report and trace outputs are optimized-build only (baseline comparison path removed).
- Trace JSON output is schema-versioned and structured for downstream tooling.

## Search Behavior (Important)
- Seed behavior:
  - default is runtime-random seed (`search.seed: 0`).
  - deterministic reproducibility is explicit (`--seed <u64>` or scenario `search.seed`).
  - reports include the effective seed.
  - persistent full-score cache partitioning ignores runtime-random default seeds to preserve cache reuse across default runs.
- `maximum_quality` behavior:
  - runs a pre-budget coverage stage that explicitly touches each legal item/rune/shard asset.
  - retains top diverse candidates per locked asset and injects them into main search.
  - starts runtime budget accounting only after coverage stage completes.
  - popcorn progress-window timeout does not interrupt this coverage stage.
  - if coverage cannot complete, search proceeds in explicit degraded mode and diagnostics include coverage warning flags.
- Runtime-budget behavior:
  - timed budget arms on first timed-phase simulation evaluation (not during setup/report generation).
  - for `maximum_quality`, timed budget can only arm after coverage-stage completion.
- Candidate scoring behavior:
  - generation-time strategy ranking can score partial candidates (improves greedy/beam branching quality before full builds are complete).
  - strict final ranking remains full-candidate only.
  - if time limits interrupt seed search, finite partial candidates are deterministically completed to full candidates before strict fallback selection.
  - full-loadout `beam` and `greedy` now co-optimize loadout selection (runes/shards) during item search.
  - adaptive/bleed strategy-key ordering is normalized before index-based seed math for fixed-seed reproducibility.
  - strict ranking can heuristic-order remaining candidates using strict-stage item/rune/shard signals, with configurable random exploration promotions.
  - unmodeled-rune quality gating is explicit and configurable:
    - optional hard gate (`unmodeled_rune_hard_gate`)
    - optional per-rune score penalty (`unmodeled_rune_penalty_per_rune`)
    - diagnostics include rejected/penalized candidate counts.
  - diagnostics now report effective thread count and parallel-mode flags for orchestration phases.

## Data/Runtime Correctness Updates
- `simulation.protoplasm_trigger_health_percent` is honored when set.
- If controlled champion level overrides simulation fallback level, Protoplasm level-scaled defaults are recalculated to the effective controlled level unless explicitly overridden.
- Legacy loadout keys now fail fast:
  - `loadout.runes_reforged.rune_ids`
  - `loadout.season2016_masteries`
- Legacy simulation tuning keys now fail fast:
  - `simulation.vlad_*`
- Deterministic loadout stat parsing no longer maps cooldown values expressed in seconds to ability haste.
- Scenario objective invulnerable-seconds normalization now references scenario horizon (instead of a fixed one-second baseline).
- Enemy respawn delay now uses each enemy actor's own level.
- Pareto/EHP/AP metric diagnostics now apply controlled champion stack overrides, matching objective simulation assumptions.
- Report metrics/build-order diagnostics re-resolve candidate loadout stats on persistent-cache hits (avoids base-loadout fallback skew).
- Opponent encounters now require at least one positive encounter weight; all-zero-weight scenario sets are rejected.
- Vladimir Sanguine Pool is modeled as per-tick area damage-over-time with range checks on each tick; trace events now include enemy-hit counts for area spells and pool ticks.
- Controlled champion cast gating now enforces cast-lock state (windup/channel/lockout), preventing same-tick spell stacking from engine scheduling.
- Controlled champion offensive-ultimate-before-defensive-ability-two policy now loads from `Characters/Vladimir.json` simulation policy data (script-owned; not engine hardcoded).
- Reports now explicitly list controlled champion runes that currently have no modeled deterministic or combat-time runtime effect.
- Shared runtime rune effects now apply combat-time behavior for Press the Attack, Fleet Footwork, Conqueror, and Aftershock rather than leaving them as unmodeled placeholders.

## Recent Observed Runtime Characteristic
- Coverage stage is currently the dominant fixed cost in short runs.
- Example: a 1-second maximum-quality budget can still take tens of seconds wall-clock due to pre-budget coverage.

## Current Known Tradeoff
- Coverage breadth floor is strong, but short-iteration latency is higher than ideal.

## Highest-Value Next Work
1. Reduce coverage-stage latency by constructing legal locked rune pages directly (instead of random rejection sampling).
2. Persist and reuse coverage-stage seed corpus across runs.
3. Add explicit coverage tuning controls (enable/disable, trials-per-asset, top-per-asset).
4. Add guardrail tests for:
   - asset coverage guarantee
   - post-coverage time-budget start behavior.

## Where To Look First
- Main orchestration:
  - `src/scenario_runner.rs`
- Search algorithms:
  - `src/search.rs`
- Data/schema parsing:
  - `src/data.rs`
- Reports:
  - `src/reporting.rs`
- Contributor rules:
  - repository root `AGENTS.md`

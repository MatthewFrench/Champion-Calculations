# Current State Snapshot (2026-02-17)

This file is a concise handoff for developers and AI agents.

## Primary Intent
- Keep the simulation engine generic and data-driven.
- Keep champion/item/rune-specific behavior in scripts and canonical data files.
- Use multi-algorithm search to find high-value item + rune loadouts and build orders.

## What Is Implemented Now
- Rust simulation engine with fixed server-tick stepping (URF default 30 Hz).
- Generic actor/champion abstractions for controlled champion and opponents.
- Champion script dispatch under `src/scripts/champions/`.
- Item and runtime loadout script hooks under `src/scripts/items/` and `src/scripts/runtime/`.
- Shared runtime stat-query resolution for cooldowns and scalar combat metrics (incoming damage taken, healing, movement speed, outgoing bonus-ability damage) from base data + runtime buff state.
- Strict scenario schema and minimal scenario setup under `Simulation/scenarios/`.
- URF item allow-list restrictions and enemy URF preset loadouts.
- Parallelized search/ranking with persistent score cache.
- Report and trace outputs are optimized-build only (baseline comparison path removed).

## Search Behavior (Important)
- Seed behavior:
  - default is runtime-random seed (`search.seed: 0`).
  - deterministic reproducibility is explicit (`--seed <u64>` or scenario `search.seed`).
  - reports include the effective seed.
- `maximum_quality` behavior:
  - runs a pre-budget coverage stage that explicitly touches each legal item/rune/shard asset.
  - retains top diverse candidates per locked asset and injects them into main search.
  - starts runtime budget accounting only after coverage stage completes.

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

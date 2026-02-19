# Champion Calculations (League of Legends Simulator)

This repository contains a data-driven combat simulator focused on URF team-fight optimization, with Vladimir as the current controlled champion benchmark scenario.

## Project Goal
- Build a generic, reusable simulation engine that can evaluate champion + item + rune loadouts with realistic combat behavior.
- Keep the engine generic and move champion/item/rune-specific mechanics into script modules and canonical data files.
- Use search algorithms to find strong full-build outcomes and strong build orders.

## Current State (Important)
- Runtime implementation is Rust (`Simulation/`).
- Search is parallelized and supports multiple algorithms (`beam`, `hill_climb`, `genetic`, `simulated_annealing`, `mcts`, `random`, `portfolio`).
- Controlled-champion and opponent simulation use shared generic abstractions (actors/champions), not enemy-only core paths.
- Runtime metrics are resolved from canonical base data plus active buff state through shared stat queries:
  - cooldown metrics (ability/item/neutral)
  - scalar combat metrics (incoming damage taken, healing, movement speed, and outgoing bonus-ability damage)
- Controlled champion now runs explicit basic-attack start/windup/hit events (hitbox/projectile-aware) and uses shared runtime attack-speed/on-hit effect paths.
- Controlled champion cast-lock state now gates cast permission, preventing same-tick spell stacking.
- Controlled champion reports and trace outputs focus on the optimized build outcome (no baseline comparison workflow).
- Reports explicitly flag controlled champion rune selections that are currently unmodeled in deterministic/runtime combat logic.
- Shared combat-time rune triggers now model runtime effects for:
  - Press the Attack, Fleet Footwork, Conqueror, Aftershock
  - Electrocute, First Strike, Phase Rush
  - Arcane Comet, Summon Aery, Hail of Blades, Dark Harvest
  - Triumph, Gathering Storm, Second Wind
- Controlled champion and enemy actors now execute rune combat logic through the same shared runtime path.
- Aftershock resist-window mitigation is now applied during the active window for both controlled champion and enemy actors.
- Search scoring now supports an explicit unmodeled-rune quality gate policy (hard gate or per-rune score penalty) to avoid rewarding placeholder loadouts.
- Search quality profiles now enforce profile-aware unmodeled-rune policy:
  - `maximum_quality` uses hard rejection for unmodeled rune candidates
  - `fast`/`balanced` keep penalty mode
- Search scoring now also supports explicit unmodeled-item-effect quality gating (hard gate or per-item penalty), with profile-aware defaults.
- Added a direct fixed-loadout evaluation mode for controlled champion A/B comparisons without search (`controlled_champion_fixed_loadout`).
- Added a fixed-loadout keystone comparison mode (`controlled_champion_fixed_loadout_rune_sweep`) for direct one-build rune sweeps.
- Reports and traces now include rune proc telemetry with trigger-source attribution and calibration metrics (opportunity counts, proc-opportunity rates, and damage/healing share).
- Optional `simulation.combat_seed` enables deterministic combat-variation runs (enemy init order + initial attack jitter); fixed-loadout rune sweep repeats now use distinct combat seeds per repeat.
- Scenarios are strict/minimal and reference canonical data from:
  - `Characters/`
  - `Items/`
  - `Game Mode/`
  - `Masteries/`
- Enemy presets for the default URF scenario are loaded from:
  - `Simulation/data/enemy_urf_presets.json`

## Search And Seed Policy
- Default search seed is runtime-random.
- Deterministic reproducibility is available via fixed seed override:
  - CLI: `--seed <u64>`
  - Scenario: `search.seed`
- Reports always include the effective seed used.
- In `maximum_quality`, a pre-budget coverage stage runs before timed optimization:
  - every legal item/rune/shard asset is explicitly touched at least once
  - top diverse seeds from that stage are injected into main search
  - runtime budget starts after coverage stage completes
  - popcorn progress-window timeout is applied after coverage (coverage itself is protected from popcorn early-stop checks)
  - if coverage is incomplete (timeout boundary or non-finite candidate gaps), search continues in explicit degraded mode and reports a coverage warning flag
- Runtime budget for timed search now arms on first timed-phase simulation evaluation (not during setup/wrap-up).
- Persistent full-score cache partitioning now ignores runtime-random default seed values:
  - deterministic seeds still partition cache explicitly
  - default random-seed runs reuse a shared cache partition
- Full-loadout `beam` and `greedy` now co-optimize loadout selection with item expansion.
- Adaptive/bleed strategy-key ordering is normalized before seed-index derivation for fixed-seed reproducibility.
- Seed-stage partial candidates are deterministically completed before strict full-ranking fallback in short-budget runs.
- Strict full-ranking can heuristic-order remaining candidates (item/rune/shard signals) with configurable random exploration promotions.

## Directory Overview
- `Simulation/`: Rust simulator, scenarios, reports, docs, and search pipeline.
- `Characters/`: champion canonical gameplay data and defaults.
- `Items/`: item data.
- `Game Mode/`: mode rules and defaults (for example URF).
- `Masteries/`: rune/stat shard data.
- `From Online/`: external-source ingestion and normalization material.

## Key Docs
- High-detail simulator docs:
  - `Simulation/README.md`
- Roadmap and status:
  - `Simulation/IMPLEMENTATION_ROADMAP.md`
  - `Simulation/IMPROVEMENT_TRACKER.md`
- Current implementation snapshot:
  - `Simulation/CURRENT_STATE.md`
  - `Simulation/COVERAGE_GAPS.md`
- Data authoring and coverage workflow:
  - `Simulation/DATA_AUTHORING_GUIDE.md`
  - `Simulation/COVERAGE_CHECKLIST.md`
- Contributor/agent rules:
  - `AGENTS.md`

## Quick Run
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode controlled_champion
```
`vladimir` remains accepted as a compatibility alias for `controlled_champion`.

## License And Notices
- Code and original repository content: `AGPL-3.0-or-later` (`LICENSE`).
- Third-party notices: `THIRD_PARTY_NOTICES.md`.
- Contributor agreement: `CLA.md` and `CONTRIBUTING.md`.

## Disclaimer
- This project is not affiliated with or endorsed by Riot Games.
- League of Legends and Riot Games names, marks, and game IP belong to Riot Games, Inc.

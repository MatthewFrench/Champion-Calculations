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
- Controlled champion reports and trace outputs focus on the optimized build outcome (no baseline comparison workflow).
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
- Contributor/agent rules:
  - `AGENTS.md`

## Quick Run
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode vladimir
```

## License And Notices
- Code and original repository content: `AGPL-3.0-or-later` (`LICENSE`).
- Third-party notices: `THIRD_PARTY_NOTICES.md`.
- Contributor agreement: `CLA.md` and `CONTRIBUTING.md`.

## Disclaimer
- This project is not affiliated with or endorsed by Riot Games.
- League of Legends and Riot Games names, marks, and game IP belong to Riot Games, Inc.

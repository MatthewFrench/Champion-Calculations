# Data Authoring Guide

This guide defines how to add or update champion, item, rune, and shard data so it stays aligned with the simulator architecture.

## Current Data Reality
- We already have broad source data in:
  - `From Online/champions/`
  - `From Online/items/`
  - `Masteries/RunesReforged.json`
- We do **not** yet have full runtime-mechanic coverage for all assets.
- The canonical gap tracker is `Simulation/COVERAGE_GAPS.md`.

## Canonical Ownership Map
- Champion canonical gameplay data:
  - `Characters/<Champion>.json`
- Champion AI policy (spacing/movement/script polling/priority overrides):
  - `Simulation/data/champion_ai_profiles.json`
- Item canonical data:
  - `Items/<Item>.json`
- Rune and shard canonical data:
  - `Masteries/RunesReforged.json`
- Mode defaults (for example URF respawn):
  - `Game Mode/<Mode>.json`
- Global simulator/search/engine defaults:
  - `Simulation/data/simulator_defaults.json`

Do not move data across these ownership boundaries.

## Best Complete Examples
- Champion canonical + script integration:
  - `Characters/Vladimir.json`
  - `Simulation/src/scripts/champions/vladimir/`
  - `Simulation/src/scripts/champions/controlled_champion.rs`
- Modeled survivability item effects:
  - `Items/Zhonyas Hourglass.json`
  - `Items/Guardian Angel.json`
  - `Items/Protoplasm Harness.json`
  - `Simulation/src/scripts/items/hooks.rs`
  - `Simulation/src/scripts/runtime/controlled_champion_loadout.rs`
- Modeled combat-time rune behavior:
  - `Masteries/RunesReforged.json` (for canonical rune definitions)
  - `Simulation/src/scripts/runes/effects.rs`
  - `Simulation/src/scripts/runtime/loadout_runtime.rs`
- Coverage registry and tests:
  - `Simulation/src/scripts/coverage.rs`
  - `Simulation/src/scripts/tests/coverage_tests.rs`
  - `Simulation/src/scripts/runtime/tests/loadout_runtime_tests.rs`

## How We Got The Complete Examples There
### Step 1: Start from source data
- Champion/item raw source is kept under `From Online/`.
- Item normalization pipeline:
  - `From Online/run_item_pipeline.py`
  - `From Online/normalize_items.py`
  - `From Online/make_sample_set.py`

### Step 2: Canonicalize into domain-owned files
- Champion gameplay values go in `Characters/<Champion>.json` (`base_stats`, `abilities`, `basic_attack`).
- Item gameplay values go in `Items/<Item>.json` with structured effects (`effects_structured`) and stable effect IDs.
- Runes/shards stay in `Masteries/RunesReforged.json`.

### Step 3: Load defaults from canonical data, not hardcoded literals
- Add or reuse loader paths in `Simulation/src/defaults.rs`.

### Step 4: Implement runtime behavior in scripts/runtime modules
- Champion-specific behavior in `Simulation/src/scripts/champions/<champion>/`.
- Shared loadout/item/rune behavior in `Simulation/src/scripts/runtime/` and `Simulation/src/scripts/items/`.

### Step 5: Register modeled coverage explicitly
- Add modeled asset keys in `Simulation/src/scripts/coverage.rs` and `Simulation/src/scripts/runes/effects.rs` as needed.

### Step 6: Add tests for behavior and regression
- Add unit tests near module `tests/` folders.

### Step 7: Update tracking docs
- Update `Simulation/COVERAGE_GAPS.md`.
- Update `Simulation/IMPROVEMENT_TRACKER.md`.
- Update docs if architecture/ownership changed.

## Champion Authoring Playbook
### Step 1: Author canonical champion file
- Required base shape:
  - `name`, `data_version`, `base_stats`, `basic_attack`, `abilities`.
- Put gameplay numbers in `abilities` and ability `effects`.
- Put ability timing/geometry in `abilities.<ability_key>.execution`.
- Keep `simulation` policy minimal and only for simulator-specific knobs that do not belong in canonical gameplay fields.

### Step 2: Keep AI controller policy separate
- Put movement/cadence/priority policy in `Simulation/data/champion_ai_profiles.json`.

### Step 3: Wire script behavior
- Controlled champion capability: `Simulation/src/scripts/champions/controlled_champion.rs`.
- Champion-specific behavior module: `Simulation/src/scripts/champions/<champion>/`.

### Step 4: Add tests
- Champion script tests in `Simulation/src/scripts/champions/tests/` or `Simulation/src/scripts/champions/<champion>/tests/`.

## Item Authoring Playbook
### Step 1: Start from source item JSON
- Use `From Online/items/` (and normalized outputs when useful).

### Step 2: Ensure canonical item data is complete
- accurate `stats`
- passive/active raw text
- `effects_structured` entries with stable `id` and useful trigger/cooldown/duration/scaling fields
- `schema_notes.effects_structured_reviewed` date update

### Step 3: Implement runtime behavior when effect impacts combat
- For stat assumptions and notes: `Simulation/src/scripts/items/hooks.rs`.
- For combat-time behavior: `Simulation/src/scripts/runtime/loadout_runtime.rs` and related runtime hooks.

### Step 4: Register modeled status
- Add item key in `Simulation/src/scripts/coverage.rs` when runtime modeled.

## Rune And Shard Authoring Playbook
### Step 1: Use canonical rune/shard source data
- Source of truth: `Masteries/RunesReforged.json`.

### Step 2: Wire deterministic stat effects
- Ensure effect shape is parseable by `Simulation/src/data.rs` (`apply_structured_effect`, `apply_stat_bonus`).
- Add support for new stat keys in `apply_stat_bonus` when needed.

### Step 3: Wire dynamic combat-time rune effects
- Add rune key to `Simulation/src/scripts/runes/effects.rs`.
- Implement runtime behavior in `Simulation/src/scripts/runtime/loadout_runtime.rs`.
- Add tuning defaults in `Simulation/data/simulator_defaults.json` and typed loader fields in `Simulation/src/defaults.rs`.

### Step 4: Legacy mastery note
- Legacy `Season2016` masteries are intentionally retired from runtime support.

## Data Authoring Rules That Must Hold
- Do not hardcode champion/item/rune behavior in shared core modules:
  - `Simulation/src/engine.rs`
  - `Simulation/src/core.rs`
  - `Simulation/src/search.rs`
  - `Simulation/src/reporting.rs`
- Use explicit domain naming (for example `Vladimir`, not `Vlad`).
- Avoid direct raw metric mutation when generic runtime apply/resolve paths exist.
- If mechanics are uncertain, record ambiguity in `Simulation/CONFIDENCE_REVIEW.md`.

## Definition Of Done
Before calling data work complete, run the checklist in:
- `Simulation/COVERAGE_CHECKLIST.md`

and required validation:
- `cargo fmt --manifest-path Simulation/Cargo.toml`
- `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
- `cargo test --release --manifest-path Simulation/Cargo.toml`

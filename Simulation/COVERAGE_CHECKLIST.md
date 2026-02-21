# Coverage Checklist

Use this checklist whenever you add or modify champion/item/rune/shard data.

Reference: `Simulation/COVERAGE_IMPLEMENTATION_PLAYBOOK.md` defines the quality bar and Definition of Done for each coverage lane.

## 1) Ownership And Schema Placement
- [ ] The field is stored in its canonical domain owner.
- [ ] No champion/item/rune-specific logic was added to shared core modules.
- [ ] Champion files keep canonical gameplay data; AI policy is in `Simulation/data/champion_ai_profiles.json`.
- [ ] No deprecated schema keys were reintroduced.

## 2) Champion Coverage
- [ ] `Characters/<Champion>.json` has complete canonical sections:
  - `base_stats`
  - `basic_attack`
  - `abilities`
- [ ] Ability execution geometry/timing is on `abilities.<ability_key>.execution`.
- [ ] `sources` and `schema_notes` are updated.
- [ ] If champion behavior changed at runtime, script logic is implemented in `Simulation/src/scripts/champions/<champion>/` and follows the champion coverage standard from the playbook.
- [ ] If controlled champion behavior was added, `Simulation/src/scripts/champions/controlled_champion.rs` wiring is updated.
- [ ] Champion runtime behavior has tests.

## 3) Item Coverage
- [ ] `Items/<Item>.json` includes accurate `stats`, passive/active text, and `effects_structured`.
- [ ] Structured effects use stable effect IDs and include trigger/cooldown/duration/scaling where applicable.
- [ ] `schema_notes.effects_structured_reviewed` is updated.
- [ ] If the item affects combat/runtime, behavior is implemented in scripts/runtime (not core engine).
- [ ] If runtime-modeled, the item key is added to `Simulation/src/scripts/coverage.rs` only after runtime behavior + tests land.
- [ ] Item behavior has regression tests.

## 4) Rune And Shard Coverage
- [ ] Rune canonical data in `Masteries/RunesReforged.json` is valid and ordered by legal slots.
- [ ] Deterministic stat effects are parseable by `Simulation/src/data.rs`.
- [ ] New deterministic stat keys are handled in `apply_stat_bonus` when needed.
- [ ] Dynamic rune behavior is added to `Simulation/src/scripts/runes/effects.rs` only after runtime logic + tests are implemented.
- [ ] Runtime rune tuning defaults are added to `Simulation/data/simulator_defaults.json` and `Simulation/src/defaults.rs`.
- [ ] Rune runtime behavior has tests.
- [ ] Shard stat keys are supported or explicitly documented as unmodeled.

## 5) Quality Gates And Coverage Tracking
- [ ] Unmodeled-rune and unmodeled-item behavior is handled intentionally (modeled, gated, or tracked as gap).
- [ ] `Simulation/COVERAGE_GAPS.md` is updated for any modeled/unmodeled coverage change, including what remains intentionally deferred.
- [ ] `Simulation/IMPROVEMENT_TRACKER.md` is updated with landed work.
- [ ] `Simulation/README.md` and root `README.md` are updated if contributor workflow changed.

## 6) Confidence And Research
- [ ] Uncertain mechanics were verified from authoritative sources when possible.
- [ ] Remaining ambiguity/assumptions are recorded in `Simulation/CONFIDENCE_REVIEW.md`.

## 7) Tracker Sync
- [ ] `Simulation/scripts/generate_coverage_trackers.py` was run and generated tracker files were committed when coverage status changed.

## 8) Validation Gates (Required)
- [ ] `cargo fmt --manifest-path Simulation/Cargo.toml`
- [ ] `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
- [ ] `cargo test --release --manifest-path Simulation/Cargo.toml`

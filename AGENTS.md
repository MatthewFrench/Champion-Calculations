# AGENTS Instructions

These instructions apply to the entire repository.

## Canonical Location And Discovery
- The canonical repository-wide agent instruction file is this file at repository root: `AGENTS.md`.
- Quick discovery command from this repository (or child paths):
  - `find .. -name AGENTS.md -print`
- Contributor documentation should reference this file directly when pointing to agent rules.

## Coverage Documentation Index
- Coverage status snapshot (complete vs incomplete):
  - `Simulation/COVERAGE_GAPS.md`
- Exemplar-derived standards for new coverage:
  - `Simulation/COVERAGE_STANDARDS.md`
- Required completion checklist:
  - `Simulation/COVERAGE_CHECKLIST.md`
- Data authoring and source/provenance workflow:
  - `Simulation/DATA_AUTHORING_GUIDE.md`

## Repository Structure Map (Non-Exhaustive)
- This is a high-value navigation map, not a complete file listing.
- Use it to quickly locate ownership boundaries and likely edit locations.

```text
/
  AGENTS.md                         # Repository-wide rules and engineering standards
  README.md                         # Project-level architecture/status summary
  Characters/                       # Canonical champion gameplay + simulation defaults
  Items/                            # Canonical item data and structured effects
  Game Mode/                        # Mode-owned defaults (for example URF)
  Masteries/                        # Rune/mastery domain data inputs
  Simulation/
    README.md                       # Simulator-specific behavior and module ownership notes
    ARCHITECTURE_STANDARDS.md       # Source of truth for module boundaries/owner channels
    ARCHITECTURE_TRANSFORMATION_PLAN.md
                                   # Architecture migration status and friction tracking
    IMPLEMENTATION_ROADMAP.md       # Feature and architecture milestone status
    IMPROVEMENT_TRACKER.md          # Chronological landed improvements
    COVERAGE_GAPS.md                # Known realism/data/runtime gaps
    src/
      main.rs                       # Thin entrypoint orchestration
      simulation_contracts.rs       # Shared contracts exposed at crate root
      engine.rs                     # Engine facade (delegates to engine/*)
      search.rs                     # Search facade (delegates to search/*)
      scenario_runner.rs            # Scenario facade (delegates to scenario_runner/*)
      defaults.rs                   # Defaults facade (delegates to defaults/*)
      data.rs                       # Data facade (delegates to data/*)
      reporting.rs                  # Reporting facade (delegates to reporting/*)
      scripts/                      # Champion/item/rune/runtime script ownership
      tests/                        # Cross-module/integration-style regression tests
```

## Important Files (Non-Exhaustive)
- `AGENTS.md`: mandatory working rules, ownership boundaries, validation requirements.
- `README.md`: top-level status and architecture summary for contributors.
- `Simulation/ARCHITECTURE_STANDARDS.md`: canonical architecture and owner-channel standards.
- `Simulation/ARCHITECTURE_TRANSFORMATION_PLAN.md`: architecture progress, status indicators, friction.
- `Simulation/IMPLEMENTATION_ROADMAP.md`: in-progress/planned simulator work.
- `Simulation/IMPROVEMENT_TRACKER.md`: landed improvements and historical context.
- `Simulation/COVERAGE_GAPS.md`: realism and fidelity gaps that still need work.
- `Simulation/src/engine.rs`, `Simulation/src/search.rs`, `Simulation/src/scenario_runner.rs`: subsystem facades; implementation should generally be pushed into submodules.
- `Simulation/src/scripts/`: champion/item/rune/runtime specialization modules.
- `Simulation/src/tests/`: high-value regression tests spanning major workflows.

## Core Architecture Standard
- Keep the simulator generic and replaceable.
- Do not add champion-specific, item-specific, rune-specific, or mastery-specific behavior to shared core modules when it can be scripted.
- Shared core modules include:
  - `Simulation/src/engine.rs`
  - `Simulation/src/core.rs`
  - `Simulation/src/search.rs`
  - `Simulation/src/reporting.rs`
- Place specialized behavior in script modules under `Simulation/src/scripts/` with clean interfaces.

## Abstraction-First Scripting Policy
- Avoid champion-and-item pair hardcoding (for example champion module types/functions that directly encode one specific item behavior).
- Item behavior should be expressed through item/runtime script capabilities and consumed through generic actor/target interfaces.
- Champion scripts should describe champion kit behavior only; shared defensive/offensive item activation logic belongs in generic runtime or item script modules.
- When a special interaction is needed, prefer introducing a reusable capability abstraction first, then attach it through scripts.
- New shared interfaces must use role-neutral terminology (`actor`, `target`, `opponent`, `controlled champion`) rather than role-locked naming.

## Champion And Enemy Symmetry
- Use the same abstractions for the controlled champion and enemies where practical.
- Avoid one-off logic paths that only exist for one champion unless represented as a script capability that any champion can use.

## Naming And Terminology
- Do not abbreviate champion names or major domain terms in new code, docs, or user-facing output.
- Examples:
  - Use `Vladimir`, not `Vlad`.
  - Use `loadout`, not short aliases that reduce clarity.
- Prefer explicit naming even if it is longer.

## Scenario And Data Shape
- Scenario files should stay minimal and reference canonical data from:
  - `Characters/`
  - `Items/`
  - `Game Mode/`
  - `Masteries/`
- Do not duplicate base data in scenario JSON unless it is scenario-specific behavior.

## AI Controller Policy Ownership
- Champion canonical data files (`Characters/*.json`) must not store AI controller cadence/policy fields (for example scripted cast schedules).
- AI behavior policy (combat spacing, movement scaling, cast polling cadence, non-canonical cooldown overrides) belongs in dedicated AI policy data under `Simulation/data/`.
- Champion files may keep only champion-owned gameplay data and per-ability execution geometry/routing overrides required by scripts.

## Data-Driven Defaults Policy
- Do not scatter fallback tuning numbers through Rust modules.
- Default ownership must follow domain boundaries:
  - global simulator/search/engine defaults: `Simulation/data/simulator_defaults.json`
  - game-mode defaults (for example URF respawn behavior): `Game Mode/<mode>.json`
  - champion-specific simulation defaults (behavior, script constants, slot bindings): `Characters/<Champion>.json`
- `Simulation/src/defaults.rs` is a loader layer and must read from the owning data file above.
- Do not move champion-specific or mode-specific values into `simulator_defaults.json`.
- Shared modules should read defaults via loader helpers instead of hardcoded profile constants.
- This rule applies to:
  - simulation/search defaults and quality profile presets (global)
  - mode-specific simulation defaults (mode file)
  - champion behavior/script defaults and ability-slot mapping (champion file)
  - loadout-generation fallback defaults (global)
- Inline literals are acceptable only for obvious structural values (for example `0.0`, `1.0`) when they are not a tunable gameplay assumption.

## Schema Ownership And Placement Rules
- Do not use catch-all containers to hold data that already has a canonical domain object.
- In `Characters/<Champion>.json`:
  - canonical champion stats belong in `base_stats`
  - canonical gameplay ability data belongs in `abilities`
  - simulator-only policy knobs may exist in `simulation`, but must be minimal and keyed by stable ability/mechanic identity
- Avoid champion-prefixed keys inside generic structures when an ability/mechanic identity key can be used instead.
- Do not duplicate the same gameplay value across sections unless one is explicitly declared a derived/runtime value.
- If duplication is unavoidable, document source-of-truth and derivation path in the file notes/schema notes.

## Mandatory Pre-Edit Review For Data Changes
- Before changing any JSON schema or moving values:
  - read the full destination object and immediate sibling objects
  - list top-level keys and check whether the target value already has a canonical home
  - verify ownership boundary (global vs mode vs champion vs item vs rune vs mastery)
- If field placement is ambiguous, stop and ask the user before writing changes.
- Do not perform structural edits based on partial-file assumptions.

## Mandatory Post-Edit Self-Audit
- After any schema/data refactor:
  - verify no old schema keys remain unless intentionally preserved for compatibility
  - verify loaders and error messages reference the new canonical path names
  - update `Simulation/README.md`, `Simulation/IMPLEMENTATION_ROADMAP.md`, and `Simulation/IMPROVEMENT_TRACKER.md` when architecture or schema ownership changed
- In the user report, explicitly state:
  - what moved
  - why it belongs there
  - what was intentionally left unchanged

## Ability Swapping And Slot Architecture
- Keep ability identity separate from cast slot (`Q`, `W`, `E`, `R`, `D`, `F`).
- Slot bindings must be treated as runtime/data state, not baked into engine branches.
- Shared engine logic must cast/track cooldowns by ability identity and runtime mapping, not by champion-specific fixed slot fields.
- Champion/item/rune/mastery special cases should be implemented in scripts/capabilities, not in `engine.rs`.

## Mechanics Research And Confidence
- If behavior is uncertain, research online using authoritative sources before locking assumptions.
- Prefer Riot/Data Dragon/official patch notes first; use wiki/community sources as secondary validation.
- Record unresolved ambiguity or low-confidence assumptions in `Simulation/CONFIDENCE_REVIEW.md`.

## Change Hygiene
- When adding or changing architecture behavior:
  - Update repository-root `README.md` for high-level project state changes.
  - Update `Simulation/README.md` if behavior or extension points changed.
  - Update `Simulation/IMPLEMENTATION_ROADMAP.md` status when milestones move.
  - Update `Simulation/IMPROVEMENT_TRACKER.md` when meaningful work lands.
- If behavior or search semantics changed materially, add/update a current-state handoff note under `Simulation/` so future contributors have a concise snapshot.

## Search And Reproducibility Policy
- Search seed policy:
  - default behavior is runtime-random seed (breadth-first exploration).
  - deterministic reproducibility must be explicitly enabled with a fixed seed override (for example CLI `--seed` or scenario `search.seed`).
  - reports must include the effective seed used by the run.
- Maximum-quality search policy:
  - includes a pre-budget coverage stage that must touch each legal item/rune/shard asset at least once.
  - time budgets (for example `--max-runtime-seconds`) are applied after coverage completes.
  - if changing this behavior, update docs and diagnostics sections together.

## Validation Requirements
- Run and pass before finishing:
  - `cargo fmt --manifest-path Simulation/Cargo.toml`
  - `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
  - `cargo test --release --manifest-path Simulation/Cargo.toml`

## Rust Test Layout Standard
- Do not place inline Rust test modules directly inside production `.rs` files.
- Keep Rust test code in dedicated test files, referenced from production modules via `#[cfg(test)]` + `#[path = "..."] mod tests;` when module-private access is required.
- Use clear, explicit test file names (for example `engine_tests.rs`, `stat_resolution_tests.rs`).
- Place integration/overall tests under the crate root `tests/` directory.
- Place unit tests in co-located relative `tests/` directories near the module they validate (for example `src/scripts/runtime/tests/loadout_runtime_tests.rs`).

## Function Change Test Requirement
- When modifying an existing code function, first evaluate whether current test coverage is sufficient for the behavior being changed.
- Add or update high-value unit tests for the function when the change affects logic, invariants, side effects, or edge-case handling.
- Tests should be pragmatic and necessary:
  - include the primary behavior path
  - include high-risk edge/corner behavior where regressions are likely
  - avoid low-value or redundant tests
- If no new test is added for a functional change, explicitly justify why in the user-facing update.

## Function And Module Comment Standard
- When modifying a function/module, ensure comments are high-value and necessary, not decorative.
- Add/update a top comment when it materially improves maintainability for non-obvious behavior.
- Good comments should capture relevant context such as:
  - purpose and intent
  - important assumptions/invariants
  - edge cases/caveats
  - tribal knowledge needed for safe modification
- Do not add comments that merely restate obvious code.
- Keep comments synchronized with behavior changes; stale comments are treated as defects.

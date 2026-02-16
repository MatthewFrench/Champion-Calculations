# AGENTS Instructions

These instructions apply to the entire repository.

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

## Data-Driven Defaults Policy
- Do not scatter fallback tuning numbers through Rust modules.
- New default tuning values must be added to:
  - `Simulation/data/simulator_defaults.json`
  - `Simulation/src/defaults.rs` (typed schema/loader)
- Shared modules should read defaults through `simulator_defaults()` instead of hardcoding profile constants.
- This rule applies to:
  - simulation/search defaults
  - quality profile presets
  - champion behavior/script defaults
  - loadout-generation fallback defaults
- Inline literals are acceptable only for obvious structural values (for example `0.0`, `1.0`) when they are not a tunable gameplay assumption.

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
  - Update `Simulation/README.md` if behavior or extension points changed.
  - Update `Simulation/IMPLEMENTATION_ROADMAP.md` status when milestones move.
  - Update `Simulation/IMPROVEMENT_TRACKER.md` when meaningful work lands.
- Keep deterministic behavior unless explicitly asked otherwise.

## Validation Requirements
- Run and pass before finishing:
  - `cargo fmt --manifest-path Simulation/Cargo.toml`
  - `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
  - `cargo test --release --manifest-path Simulation/Cargo.toml`

# Improvement Tracker

## Done
- Enforced runtime budget checks across all major phases and search loops.
- Added phase-aware periodic status updates from initialization through finalization.
- Moved enemy URF presets into a data file:
  - `Simulation/data/enemy_urf_presets.json`
  - Includes source links and last checked date.
- Added strict startup validation for enemy preset items, runes, shards, and masteries.
- Added structured run output JSON:
  - default path mirrors markdown report with `.json` extension.
- Added search quality profiles:
  - `fast`
  - `balanced`
  - `maximum_quality`
- Replaced full permutation build order search with beam plus optimistic bound pruning.
- Added regression tests for legality and key rules.
- Added persistent full-score cache across runs under:
  - `Simulation/output/cache/`
- Added first-pass module split for simulation extensions:
  - `src/respawn.rs`
  - `src/scripts/vladimir.rs`
- Added additional modular extraction for orchestration support:
  - `src/cache.rs`
  - `src/status.rs`
- Added enemy lifecycle simulation:
  - enemies can die from Vladimir damage
  - enemies respawn using URF-scaled death timer logic
- Added first-pass scripted Vladimir offensive cadence:
  - `Q`, `E`, and `R` damage/heal behavior integrated into the event loop
- Added strict final ranking parallel batches using Rayon for faster full candidate scoring.
- Added cap-survivor handling in output and tie-break:
  - cap-survivor labeling in reports/output
  - tie-break uses existing objective damage/healing weights
- Added repository automation workflows:
  - pull request and main branch continuous integration in `.github/workflows/continuous-integration.yml`
  - tag-based release generation with findings in `.github/workflows/release.yml`

## Not Done
- [P0] Full-fidelity Vladimir kit simulation (`Q`, `E`, `R`, passives)
  - Goal: upgrade first-pass scripted abilities to closer in-game behavior fidelity.
  - Scope:
    - Add full spell-state nuances, empowered states, and target-selection details.
    - Model cast times/windups and expected hit geometry with position model.
    - Preserve deterministic tick/event behavior.
  - Success criteria:
    - Offensive outcomes track expected ability scaling and timing behavior more closely.
    - Unit tests cover ability cooldown and cast ordering invariants.

- [P1] Build-order scoring alignment to composite objective
  - Goal: make build order optimize the same objective as end-state search.
  - Scope:
    - Replace stage survival-only accumulation with stage objective accumulation:
      - time alive
      - damage dealt
      - healing done
    - Continue stack progression by acquisition timing.
  - Success criteria:
    - Build-order ranking can differ from survival-only ranking.
    - Output includes per-stage objective components and totals.

- [P1] Composite objective refinement and guardrails
  - Goal: improve build quality under mixed metrics while avoiding pathological results.
  - Scope:
    - Add survivability floor gating for obviously non-viable builds.
    - Add optional diminishing returns/soft caps for damage and healing components.
    - Emit per-scenario component breakdown in reports.
  - Success criteria:
    - Top builds satisfy survivability floor.
    - Reports clearly explain why a build scored highly.

- [P2] Script hooks for non-generic item/champion mechanics
  - Goal: support behavior not expressible by static data.
  - Scope:
    - Define idiomatic Rust hook points for bespoke mechanic scripts.
    - Keep hooks deterministic and compatible with search parallelism.
  - Success criteria:
    - At least one complex mechanic implemented through the hook API.
    - No regression to strict candidate search stability.

- [P2] Robustness and stability sweeps
  - Goal: distinguish stable winners from seed/weight-sensitive outliers.
  - Scope:
    - Run multi-seed and multi-weight preset sweeps.
    - Summarize robust versus fragile builds in report output.
  - Success criteria:
    - Report explicitly identifies high-confidence stable loadouts.
    - Repeat runs show reduced variance in recommended top builds.


- [P0] Action timeline realism (windup, projectile travel, blocking, position)
  - Goal: improve combat fidelity beyond instant-hit abstractions.
  - Scope:
    - Add melee auto-attack windup and hit frame timing.
    - Add ranged projectile travel time with per-projectile hit resolution.
    - Add position state and movement/spacing assumptions.
    - Add support for projectile-blocking interactions where applicable.
  - Success criteria:
    - Time-to-damage differs by range/position and projectile speed.
    - Replays/debug traces show action start, launch, travel, and hit events.

- [P1] Enemy ability simulation and champion behavior scripts
  - Goal: model champion kits with deterministic scripted execution.
  - Scope:
    - Expand enemy simulation beyond generic periodic damage.
    - Implement script modules for per-champion ability usage patterns.
    - Keep scripts outside the core engine loop and data-driven where possible.
  - Success criteria:
    - Enemy champions no longer feel equivalent under same stat profile.
    - Script modules are testable independently from the core simulation engine.

- [P1] Preset/build correctness audit (enemy autos and itemization variance)
  - Goal: explain and fix cases where enemy champions appear to use overly similar auto-attack behavior/build outcomes.
  - Scope:
    - Audit enemy preset ingestion and per-champion stat application.
    - Verify item/rune/mastery effects are applied distinctly per champion.
    - Add validation/report checks to flag suspiciously similar enemy profiles.
  - Success criteria:
    - Per-enemy derived combat stats are printed in diagnostics.
    - Champion-to-champion auto attack profiles materially differ where expected.

- [P1] Codebase modularization and script-friendly architecture
  - Goal: eliminate single-file bottleneck and keep champion/item logic out of `main.rs`.
  - Scope:
    - Split core into modules (engine, search, reporting, data loading, scripts, CLI).
    - Move item/champion/mastery special cases into dedicated script modules/hooks.
    - Define clear interfaces for extending champion/item behavior.
  - Success criteria:
    - `main.rs` becomes a thin CLI orchestration entrypoint.
    - New champion/item behavior can be added without touching core engine files.

- [P1] CI/CD for pull requests, main branch, and releases
  - Goal: automate quality checks and release artifacts with run findings.
  - Scope:
    - Add CI workflows for pull requests and main:
      - build
      - test
      - formatting/lint gates
    - Add release workflow that packages binary and attaches generated findings/report summary.
  - Success criteria:
    - PRs and main branch enforce green checks.
    - Releases include artifacts plus generated run report in release notes/attachments.

- [P2] Repository quality gates (linting, style, module boundaries)
  - Goal: enforce maintainable structure and coding standards automatically.
  - Scope:
    - Add `rustfmt` and `clippy` checks in CI.
    - Add deny/warn policy for common maintainability issues.
    - Add lightweight module size/ownership conventions in docs.
  - Success criteria:
    - New changes consistently conform to style and lint rules.
    - Core simulation logic stays split across coherent modules over time.

## Open Questions
- Enemy respawn behavior:
  - Implement using URF death timer scaling by level (not a fixed `20.0s`).
  - Decision:
    - Use documented SR level scaling through level 18, then extrapolate smoothly for levels 19-30 with conservative clamping.
    - Apply URF modifier (minus 3 seconds) on top of the scaled timer.
    - Keep the respawn timer model configurable so we can swap in a verified full table later.
    - cooldown timers continue while dead.
    - buffs/debuffs that do not persist through death are removed.
    - persist-through-death effects remain.
- Position model scope:
  - Decision:
    - Start with 1D range bands and deterministic spacing assumptions.
    - Expand to 2D coordinates in a follow-up after projectile travel/blocking is stable.
- Cap-survivor ranking:
  - Decision:
    - Use combined `damage dealt + healing done` tie-break metric.
    - Reuse existing objective weights (no separate custom weighting logic).

## Research Notes (2026-02-15)
- URF death timer patch history:
  - `V13.1b` entry explicitly states: "Death timers reduced by 3 seconds at all levels."
  - No newer URF patch-history entry found that explicitly reverts this.
  - Sources:
    - https://www.leagueoflegends.com/en-us/news/game-updates/patch-13-1b-notes/
    - https://wiki.leagueoflegends.com/en-us/Ultra_Rapid_Fire/Patch_and_Buff_History
- Baseline death timer model (SR reference):
  - Base Respawn Wait by level (1-18): `10, 10, 12, 12, 14, 16, 20, 25, 28, 32.5, 35, 37.5, 40, 42.5, 45, 47.5, 50, 52.5` seconds.
  - Time Increase Factor applies by game time after 15 minutes (with cap).
  - Sources:
    - https://wiki.leagueoflegends.com/en-us/Death
    - https://leagueoflegends.fandom.com/wiki/Death
- Death-state effects:
  - Documented behavior indicates many buffs/debuffs are removed on death unless marked to persist through death.
  - Cooldown behavior should be treated as continuing through death unless contradicted by targeted in-client validation.
  - Sources:
    - https://wiki.leagueoflegends.com/en-us/Death
    - https://leagueoflegends.fandom.com/wiki/Death

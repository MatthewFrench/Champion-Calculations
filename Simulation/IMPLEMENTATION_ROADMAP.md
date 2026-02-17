# Implementation Roadmap

This file tracks all high-value follow-up work requested for simulator realism, architecture, search quality, and correctness.

## Status Legend
- `DONE`: implemented and validated
- `IN_PROGRESS`: currently being implemented
- `PLANNED`: approved and queued
- `BLOCKED`: waiting on data/rules clarification

## P0 Realism And Architecture

1. `DONE` Move all Vladimir combat behavior out of engine into script modules.
- Scope:
  - offensive rotation ownership in scripts
  - defensive logic and survivability item activation ownership in scripts
- Success criteria:
  - engine no longer hardcodes Vladimir-specific combat sequencing.

2. `DONE` Move enemy script event handling out of engine into champion-specific script handlers.
- Scope:
  - central dispatch in scripts module
  - per-champion behavior modules are isolated from core loop
- Success criteria:
  - engine processes generic events and delegates champion specifics.

3. `DONE` Normalize cross-module interfaces to controlled-champion terminology.
- Scope:
  - remove legacy `for_vlad` hook context fields
  - use generic controlled-champion names in enemy script execution input
  - keep compatibility wrappers where needed for external call stability
- Success criteria:
  - shared script interfaces do not hardcode Vladimir-specific identifiers.

3.1 `DONE` Separate champion AI controller policy from champion canonical data.
- Scope:
  - move script cast cadence policy out of `Characters/*.json`
  - move AI movement/spacing policy (`desired_combat_range`, `movement_speed_scale`) into dedicated AI data
  - move per-ability execution timing/projectile/hitbox ownership to `abilities.<ability_key>.execution` with role defaults in `Characters/ChampionDefaults.json`
  - introduce cooldown-ready script polling (`cast when ready`) for scripted enemy abilities
- Success criteria:
  - champion files contain canonical champion gameplay data (including `abilities.<ability_key>.execution`) while controller policy lives in AI profiles.

3.2 `DONE` Remove remaining champion `behavior` and `scripts` gameplay overrides from roster files.
- Scope:
  - migrate Vayne Silver Bolts periodic true-hit tuning to canonical ability effects
  - migrate Warwick Eternal Hunger and Infinite Duress scaling to canonical passive/ultimate effects
  - remove Yasuo champion script module from simulation roster
- Success criteria:
  - roster champion files keep canonical gameplay data only; no leftover simulator-only gameplay constants in champion JSON.

3.3 `DONE` Remove champion top-level `ability_slot_bindings` and derive default bindings from canonical ability data.
- Scope:
  - derive runtime slot-to-ability defaults from `abilities.<ability>.slot` / `default_keybinding`
  - remove top-level `ability_slot_bindings` from champion JSON
- Success criteria:
  - slot binding defaults are canonical and do not duplicate data shape in champion root objects.

3.4 `DONE` Replace legacy scenario schema with strict canonical scenario ownership.
- Scope:
  - remove legacy scenario aliases (`vladimir_*`, top-level `enemies`, `enemy_scenarios`, `enemy_loadout`)
  - require canonical scenario shape under `controlled_champion` and `opponents.encounters`
  - move opponent movement/placement to actor-level scenario placement policy and support explicit hold-position behavior
  - apply optional `opponents.shared_loadout` on top of preset opponent loadouts
- Success criteria:
  - all scenario modes parse a single canonical shape and fail fast on invalid/unknown movement policy values.
  - actor IDs are stable and used by staged build-order scaling logic.

3.5 `DONE` Minimize scenario simulation block and load gameplay defaults from canonical data owners.
- Scope:
  - move Vladimir Sanguine Pool defaults to `Characters/Vladimir.json` loader usage
  - move Zhonya/Guardian Angel/Protoplasm default gameplay values to item-data loader usage
  - keep only controlled champion stasis activation policy in AI defaults (`data/champion_ai_profiles.json`)
  - keep passive lifeline trigger thresholds in canonical item data
  - keep URF respawn defaults in `Game Mode/URF.json` with optional scenario override only
- Success criteria:
  - default scenario simulation block only needs scenario-owned knobs plus optional explicit overrides.

3.6 `DONE` Add actor-level level overrides and generic stack override ownership.
- Scope:
  - add `controlled_champion.level`, `opponents.default_level`, and `opponents.encounters[].actors[].level`
  - add generic stack override maps: `simulation.stack_overrides`, `controlled_champion.stack_overrides`, `opponents.stack_overrides`, and actor-level overrides
  - add `simulation.time_limit_seconds` parsing and enforce 20-minute hard cap
- Success criteria:
  - scenario can independently set controlled and opponent levels, stack overrides are generic/per-actor, runtime horizon is bounded by validated time limit, and legacy keys are rejected.
  - opponent actors are minimal setup objects and do not carry proxy combat cadence fields.

4. `IN_PROGRESS` Build a generic buff/debuff/status system.
- Scope:
  - duration, stacks, refresh policy
  - persist-through-death flags
  - deterministic ticking
- Success criteria:
  - combat effects represented as status instances, not ad hoc booleans.

5. `IN_PROGRESS` Build a generic cast system.
- Scope:
  - windup, backswing, channel, cancel rules
  - cast lock and interrupt windows
- Success criteria:
  - all spells and attack phases use a single cast-state model.

6. `IN_PROGRESS` Build a generic projectile system.
- Scope:
  - spawn, travel, hit resolution, block/reflect, expiration
  - collision layer and shape support
- Success criteria:
  - projectile behavior is data/script-driven across champions.

7. `DONE` Add hitbox-aware impact resolution and interruption outcomes.
- Scope:
  - actor and effect hitbox radius checks for attacks and scripted effects
  - explicit impact outcomes (applied, blocked, miss, untargetable nullification)
  - melee auto-attack cancellation on attacker stun during windup
- Success criteria:
  - event traces and combat outcomes differentiate blocked/missed/nullified hits from applied damage.

8. `PLANNED` Upgrade position simulation to command-based movement.
- Scope:
  - chase, kite, hold, retreat commands
  - deterministic path updates and range-maintain behavior
- Success criteria:
  - positioning changes materially affect damage windows.

9. `IN_PROGRESS` Introduce slot-agnostic ability architecture.
- Scope:
  - decouple ability identity from key binding slots (`Q`,`W`,`E`,`R`)
  - model runtime slot-to-ability mapping as data/state instead of hardcoded champion fields
  - support runtime ability remapping/swapping (for example stolen/captured abilities)
  - keep champion-specific interaction rules in ability scripts, not engine branches
- Success criteria:
  - controlled champion and enemy actors cast through the same generic ability-instance interfaces.
  - stolen/swapped ability execution does not require core engine conditionals by champion.

10. `PLANNED` Implement target-selection logic for both sides.
- Scope:
  - closest, lowest health, priority target, in-range fallback
- Success criteria:
  - target selection is configurable and scriptable.

11. `PLANNED` Expand Vladimir kit fidelity to full in-game nuance.
- Scope:
  - empowered states, conditional effects, timing edge cases
- Success criteria:
  - ability outcomes/timing align with documented in-game behavior.

12. `PLANNED` Expand enemy kit fidelity for preset champions.
- Scope:
  - beyond first-pass constants into richer state interactions
- Success criteria:
  - each champion exhibits clearly distinct kit timelines.

12a. `DONE` Opponent-first realism sequencing.
- Scope:
  - replaced opponent DPS/crowd-control proxy fields with script- and data-driven behavior.
  - removed scenario actor `combat` blocks and removed `opponents.uptime_windows_enabled`.
  - stage improvements champion-by-champion with deterministic validation fixtures.
- Success criteria:
  - realism gains are measurable and land in high-impact order.
  - opponent behavior dominates fewer outcomes through proxy approximations.

13. `IN_PROGRESS` Full combat-time rune system.
- Scope:
  - move dynamic rune behavior from notes/simplifications into scripts
- Success criteria:
  - selected runes contribute in real time when conditions trigger.

13a. `PLANNED` Legal rune-page domain enumeration/reporting.
- Scope:
  - generate deterministic counts/tables for all legal rune pages from current selection constraints.
  - include grouped counts by path pair and optional shard-expanded totals.
- Success criteria:
  - legal rune-page domain size is explicit, versionable, and easy to diff across data changes.

14. `DONE` Retire legacy mastery system (rune pages only).
- Scope:
  - remove mastery parsing, preset fields, docs, and script modules
  - enforce modern rune-page legality and shard-slot legality as the only loadout page system
- Success criteria:
  - no mastery fields remain in scenario/preset/runtime schemas.
  - invalid rune pages are rejected before simulation.

15. `PLANNED` Item script coverage for all high-frequency legendary items.
- Scope:
  - script mechanics for passives that materially alter combat outcomes
- Success criteria:
  - top-search item effects are represented in combat-time logic.

16. `PLANNED` Explicit item mode-availability model.
- Scope:
  - URF legality field and validation
- Success criteria:
  - item pool generation is mode-correct without heuristic filters.

17. `DONE` Full death-timer model including game-time scaling.
- Scope:
  - include time increase factor and URF modifiers
- Success criteria:
  - respawn timing tracks expected rules across level and game time.

18. `PLANNED` Death-state cleanup rules by effect tags.
- Scope:
  - buffs/debuffs dropped or persisted based on metadata
- Success criteria:
  - death transitions behave consistently with effect persistence semantics.

19. `PLANNED` Cooldown behavior policy validation on death/respawn.
- Scope:
  - verify cooldown progression rules for scripted abilities/effects
- Success criteria:
  - cooldown handling is explicit and tested.

## P1 Data Correctness, Calibration, Validation

20. `IN_PROGRESS` Source-backed constants dataset with provenance.
- Scope:
  - maintain sources, dates, and override policy for tuned constants
- Recent progress:
  - moved Morgana Soul Shackles detonation delay ownership to `Characters/Morgana.json` `abilities.ultimate.effects[id=tether_duration]` and removed duplicated script storage
- Success criteria:
  - constants are traceable and auditable.

21. `PLANNED` Calibration tests versus expected interactions.
- Scope:
  - scenario fixtures with expected ranges/outcomes
- Success criteria:
  - deviations are caught automatically.

22. `PLANNED` Golden regression suite.
- Scope:
  - lock key scenario outputs with tolerances
- Success criteria:
  - unintended model drift is detected in CI.

23. `PLANNED` Property-based tests for invariants.
- Scope:
  - deterministic replay, non-negative health constraints, event ordering
- Success criteria:
  - invariant violations are reproducibly surfaced.

## P1 Performance And Search Quality

24. `PLANNED` Performance profiling workflow and flamegraphs.
- Scope:
  - repeatable profiling command and report artifacts
- Success criteria:
  - hotspot regressions become actionable quickly.

25. `PLANNED` Reduce synchronization overhead in caches/scoring.
- Scope:
  - minimize lock contention and shared hot-map pressure
- Success criteria:
  - higher evaluations/second at same quality settings.

26. `PLANNED` Two-stage evaluation pipeline.
- Scope:
  - approximate fast scorer, strict rerank by full simulation
- Success criteria:
  - maintains top quality while expanding effective search width.

27. `PLANNED` Adaptive algorithm budget allocation.
- Scope:
  - allocate iterations based on marginal gain and novelty contribution
- Success criteria:
  - better objective outcomes under fixed runtime budget.

28. `PLANNED` Smarter cross-algorithm elite exchange rounds.
- Scope:
  - novelty pressure plus dedupe-aware injections
- Success criteria:
  - more diverse high-quality candidates without wasted full evaluations.

29. `PLANNED` Confidence intervals and robustness ranking.
- Scope:
  - confidence bands beyond seed-hit labels
- Success criteria:
  - report separates stable winners from fragile outliers.

30. `PLANNED` First-class multi-scenario robust optimization.
- Scope:
  - weighted mean, worst-case, and risk-sensitive options
- Success criteria:
  - recommended builds remain strong across scenario variations.

31. `PLANNED` Build-order optimization with economy/timing model.
- Scope:
  - include gold flow and purchase times
- Success criteria:
  - order recommendations align with realistic progression constraints.

## P2 Product And Tooling

32. `PLANNED` Optional summoner spell simulation.
- Scope:
  - include only when materially changing outcomes
- Success criteria:
  - scenario supports realistic spell-driven survivability/damage spikes.

33. `PLANNED` Richer report explainability.
- Scope:
  - per-component and per-enemy contribution breakdowns
- Success criteria:
  - report explains why a build wins in concrete terms.

34. `PLANNED` Resume/checkpoint support for long searches.
- Scope:
  - save and continue in-progress runs
- Success criteria:
  - interrupted runs can resume without restarting search from scratch.

35. `PLANNED` Benchmark gates in CI.
- Scope:
  - runtime and throughput trend checks
- Success criteria:
  - performance regressions fail checks or raise warnings.

36. `IN_PROGRESS` Enforce strict module boundaries to prevent core logic sprawl.
- Scope:
  - keep champion/item/rune specifics outside engine core
- Success criteria:
  - architecture remains extendable and maintainable.

37. `PLANNED` Dead-code/config cleanup with strict warning policy.
- Scope:
  - remove stale paths and enforce lint rigor
- Success criteria:
  - cleaner build and lower maintenance risk.

38. `IN_PROGRESS` Full codebase structure audit and abstraction-driven reorganization plan.
- Scope:
  - review the entire `Simulation/src` layout for abstraction boundaries and long-term extensibility
  - identify and prioritize high-friction files/modules for relocation or split
  - define a migration map with incremental compatibility checkpoints
- Success criteria:
  - a concrete target module layout exists with phased migration steps and acceptance criteria.

39. `IN_PROGRESS` Domain-oriented script hierarchy for champions, items, and runes.
- Scope:
  - replace flat script layout with domain folders (for example `scripts/champions/`, `scripts/items/`, `scripts/runes/`)
  - move per-entity script logic into dedicated files (for example one champion per module)
  - keep shared runtime primitives and registries in separate shared modules
- Success criteria:
  - script organization is discoverable, scalable, and does not rely on monolithic files.

40. `IN_PROGRESS` Naming scheme and module sizing standards.
- Scope:
  - establish file/module naming rules that are descriptive and consistent
  - set practical module size targets and split policies for large files
  - align naming with language conventions while maximizing readability
- Success criteria:
  - new modules follow a documented naming and sizing standard; large modules are progressively split.

41. `PLANNED` Architecture revisit checkpoints.
- Scope:
  - add recurring architecture review checkpoints after each major simulation feature tranche
  - track follow-up cleanup items to prevent structural drift
- Success criteria:
  - architecture debt is surfaced and resolved incrementally rather than deferred indefinitely.

42. `DONE` Maintain explicit roadmap tracking with acceptance criteria.
- Implemented in this file.

43. `PLANNED` Cross-actor component search (champion/passive/ability/item/rune-page composition).
- Scope:
  - generalize search domain to support interchangeable champion kits and passive/ability attachments
  - preserve deterministic, script-driven behavior mapping across swapped components
  - support both maximize and minimize objective modes (best/worst compositions)
- Success criteria:
  - search pipeline can evaluate legal mixed-component compositions without champion-locked engine branching.

## Current Execution Batch
- `DONE` Item 1
- `DONE` Item 2
- `DONE` Item 3
- `DONE` Item 7 (hitbox-aware impact outcomes and melee windup interruption on stun)
- `IN_PROGRESS` Item 4 (foundational scaffold merged; full migration pending)
- `IN_PROGRESS` Item 5 (foundational scaffold merged; full migration pending)
- `IN_PROGRESS` Item 9 (slot-agnostic ability architecture for remapping and stolen abilities; controlled champion foundation landed)
- `IN_PROGRESS` Item 13 (controlled champion runtime rune effects are wired through simulation/objective; broader coverage pending)
- `DONE` Item 14 (legacy mastery system removed; rune-page legality is strict and enforced)
- `IN_PROGRESS` Item 38 (audit underway; structure migration phases being tracked)
- `IN_PROGRESS` Item 36 (champion/item coupling reduced by moving defensive item and revive decisions into generic runtime/item capability scripts)
- `IN_PROGRESS` Item 39 (phase-one scripts hierarchy landed; remaining migrations pending)
- `IN_PROGRESS` Item 40 (descriptive naming and module-splitting pass underway)

## Notes
- Large items are being delivered in iterative slices with strict compile/test/lint validation at each slice.
- Any low-confidence behavior assumptions are tracked in:
  - `Simulation/CONFIDENCE_REVIEW.md`

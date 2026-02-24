# Full Game Simulation Blueprint

Last updated: 2026-02-24

## Purpose
Define everything required, beyond JSON data coverage alone, to move this repository from a deterministic URF controlled-champion combat optimizer to a near-full League-like simulation engine that could drive a renderer with game-like behavior.

This document is implementation-focused and codebase-anchored. It is intended to be the single high-level target document for "as close as possible" simulation fidelity.

## Practical Truth
- A mathematically perfect clone of Riot server behavior is not realistic without internal server code.
- The practical target is:
  - deterministic replay correctness
  - high-fidelity behavior for modeled systems
  - explicit confidence and known-drift tracking where parity is uncertain
  - renderer-consumable state/event contracts that produce visually game-like outcomes

## Definition Of Done (Full-Game-Like)
All items below must be true:

1. Full-match simulation exists (not only isolated teamfight scenarios):
- laning, jungle, objectives, structures, recalls, death/respawn cycles, and game-end conditions.

2. World simulation fidelity is complete enough for visual playback:
- map topology, pathing, collision, vision/fog, and terrain interactions are represented in runtime state.

3. Actor model supports complete match ecology:
- controlled champion, allied champions, enemy champions, lane minions, jungle monsters, and structures.

4. Action model is unified:
- all actions (move, attack, cast, item, summoner spell, objective interactions) go through one owner-command channel model.

5. Determinism and replay guarantees are strict:
- same seed + scenario always yields byte-stable event ordering and equivalent snapshots.

6. Calibration gates exist and are enforced:
- regression suites, interaction goldens, property tests, and confidence thresholds guard behavior changes.

7. Renderer contract is stable:
- per-tick snapshots and canonical event stream are sufficient for client-side playback.

## Current Baseline (Code Review Summary)
The current runtime is robust for controlled combat optimization but still narrow versus full game simulation.

Implemented baseline highlights:
- deterministic fixed-tick event-loop combat simulation (`Simulation/src/engine.rs`, `Simulation/src/engine/event_queue/*`)
- hitbox-aware attacks/projectile travel and impact outcomes (`Simulation/src/engine/event_resolution/*`, `Simulation/src/engine/geometry/*`)
- script-driven champion behavior channel for selected champions (`Simulation/src/scripts/champions/*`)
- shared runtime stat/rune/item effect channels (`Simulation/src/scripts/runtime/*`)
- search/orchestration/reporting pipeline for optimization (`Simulation/src/search.rs`, `Simulation/src/scenario_runner.rs`, `Simulation/src/reporting.rs`)

Current non-data scope limitations:
- scenario modes are combat-centric, not full match simulation (`Simulation/src/scenario_runner/*`)
- actor ecosystem is champion-focused; no minion/jungle/structure runtime loops
- movement modes are limited (`HoldPosition`, `MaintainCombatRange`) and orbit/chase abstractions are simplified (`Simulation/src/engine/simulation_step/enemy_movement_step.rs`)
- event taxonomy is combat-centric and does not include macro-map actions (`Simulation/src/engine/event_queue/event_type_catalog.rs`)
- script event roster is narrow for enemies (`Simulation/src/scripts/champions/champion_script_event_channels.rs`)
- runtime contracts do not yet represent full-map entities and macro state (`Simulation/src/simulation_contracts/runtime_actor_contracts.rs`)

## Required Capability Areas (Non-Data)

### 1) Simulation Kernel And Deterministic Timeline
Required:
- strict event ordering contract across all new subsystems
- deterministic scheduling for concurrent/multi-actor actions
- stable clock domains for simulation, cooldown, animation/cast phases, and replay
- deterministic random-source policy across all non-test paths

Current anchors:
- `Simulation/src/engine/event_queue/event_queue_ordering.rs`
- `Simulation/src/core/build_candidate_random_helpers.rs`

Needed code additions:
- typed timeline channels for macro systems (objectives/spawns/structure states)
- deterministic tie-break identifiers for all new event families
- replay checksum support per tick window

### 2) World/Map Layer
Required:
- map representation (walkable mesh/grid/hybrid), walls, brush, objective pits, structure zones
- collision, pathfinding, and movement constraints (turning/path updates)
- position ownership for all actor classes and world entities

Current anchors:
- `Simulation/src/engine/geometry/*`
- `Simulation/src/engine/simulation_step/enemy_movement_step.rs`

Needed code additions:
- dedicated world module (recommended: `Simulation/src/world/*`)
- path planner and incremental path-following stepper
- terrain tags and blocker state channels

### 3) Vision, Fog, And Information State
Required:
- team vision model, fog-of-war, stealth/reveal rules
- ward/trinket entities and lifecycles
- visibility-gated targetability/decision constraints

Current anchors:
- item/rune hooks can express some reveal effects in notes/runtime channels, but no world vision model

Needed code additions:
- vision subsystem with team-specific visibility queries
- reveal events and stealth-state channels in actor state
- renderer-facing visible-entity projection APIs

### 4) Actor Ecosystem Expansion
Required:
- actor classes: champions, minions, monsters, structures, summoned units
- actor lifecycle policies per class
- shared command/query interfaces and per-class owner channels

Current anchors:
- `Simulation/src/simulation_contracts/runtime_actor_contracts.rs`
- `Simulation/src/engine/actor_state/*`

Needed code additions:
- class-specific actor state owners and projections
- spawn/respawn systems for non-champion actors
- hostility/aggro and target-eligibility channels by actor class

### 5) Full Ability And Combat-State Engine
Required:
- canonical cast-state machine: windup/channel/cancel/backswing/interrupt
- projectile system: spawn/travel/collision/lifetime/shape layers
- effect instance system: buffs/debuffs/auras/zones with persistence semantics
- damage pipeline with complete mitigation, penetration, shields, lifesteal/spellvamp, anti-heal, and ordering rules

Current anchors:
- `Simulation/src/core/combat_primitives_state.rs`
- `Simulation/src/engine/event_resolution/*`
- `Simulation/src/scripts/runtime/stat_resolution.rs`

Needed code additions:
- full status/effect registry keyed by effect identity and tags
- standardized combat operation pipeline (pre-hit, on-hit, post-hit phases)
- explicit per-source damage classification and modifier stacks

### 6) Target Selection And Decision Layer
Required:
- configurable target-selection policies (threat, proximity, priority, objective weight)
- behavior trees or utility/action scoring for champion AI
- lane/jungle/macro intent selection channels

Current anchors:
- range-based nearest target helpers in `Simulation/src/engine/combat_timing_and_targeting.rs`

Needed code additions:
- targeting subsystem separated from low-level combat resolution
- policy runtime interfaces for controlled champion and enemy symmetry

### 7) Summoner Spells And Active Ability Layer
Required:
- summoner spells as first-class action/cooldown entities
- cast constraints, targeting, and interaction rules
- shared slot architecture with abilities/items/summoner channels

Current anchors:
- ability slot groundwork exists in `Simulation/src/scripts/runtime/ability_slots.rs`
- roadmap item exists (`Simulation/IMPLEMENTATION_ROADMAP.md`, item 32)

Needed code additions:
- summoner spell runtime contracts and effect channels
- integration into cast/interrupt/cooldown/telemetry systems

### 8) Economy, Experience, And Progression Systems
Required:
- gold income/expenditure, last-hit rules, assist/takedown rewards
- xp curves, level-up timings, skill point assignment
- recall/shop windows and buy constraints by location/time

Current anchors:
- optimization/search objective system already exists, but not full match economy simulation

Needed code additions:
- economy subsystem with event-sourced transactions
- xp subsystem with per-actor progression channels
- shop/recall state machine and buy-eligibility checks

### 9) Objective And Structure Systems
Required:
- lane structures (turrets/inhibitors/nexus) with aggro/plate/state rules
- neutral objectives (dragon/herald/baron) with spawn/despawn/buff ownership
- objective-driven game-end logic

Current anchors:
- none as first-class runtime systems today

Needed code additions:
- structure subsystem and objective subsystem owner modules
- match-state authority module for win/loss and objective clocks

### 10) Match Flow And Macro Rules
Required:
- pre-game to end-game lifecycle
- lane wave spawning and lane pressure dynamics
- surrender/remake/time-based rule transitions (as needed for target mode)

Current anchors:
- scenario model is static-combat oriented (`Simulation/src/scenario_runner/scenario_parsing.rs`)

Needed code additions:
- match orchestrator and phase machine
- timed spawn systems for waves/objectives/events

### 11) Renderer And Replay Contracts
Required:
- compact snapshot schema with complete world/actor states
- event stream schema with stable IDs and causality links
- replay loader + deterministic replay verifier

Current anchors:
- trace/report system exists for combat runs (`Simulation/src/engine/trace_snapshot_reporting/*`, `Simulation/src/reporting/*`)

Needed code additions:
- renderer-specific projection APIs
- snapshot delta encoding and replay integrity checks

### 12) Verification, Calibration, And Quality Gates
Required:
- calibration suite versus expected interactions
- golden regression suite over canonical scenarios
- property tests for invariants (ordering, conservation, lifecycle correctness)
- performance gates and profiling workflow

Current anchors:
- strong unit/integration test coverage for current combat/search scope (`Simulation/src/tests/*`, owner tests across subsystems)
- roadmap placeholders (`Simulation/IMPLEMENTATION_ROADMAP.md`, items 20-24, 35)

Needed code additions:
- dedicated calibration harness framework
- approved baseline scenario corpus with expected envelopes/tolerances
- CI performance budgets and deterministic replay checks

## Architecture Expansion Plan (Largest Chunks First)

### Phase A: Runtime Hardening Foundation
Exit gates:
- eliminate silent fallback behavior for invalid runtime config paths
- reduce panic/expect crash surfaces in runtime/default loaders
- unified error taxonomy for parser/runtime failures

Primary targets:
- `Simulation/src/defaults.rs`
- `Simulation/src/engine/combat_timing_and_targeting.rs`
- `Simulation/src/engine/event_resolution/*`
- `Simulation/src/search.rs` and full-loadout strategy dispatch

### Phase B: Combat Engine Completion (Micro Fidelity)
Exit gates:
- generic status/cast/projectile systems complete and authoritative
- ability/item/rune effects run through unified effect channels
- deterministic combat replay parity for curated champion interactions

Primary targets:
- `Simulation/src/core/combat_primitives_state.rs`
- `Simulation/src/engine/event_resolution/*`
- `Simulation/src/scripts/runtime/*`

### Phase C: World + Actor Ecosystem
Exit gates:
- map/world module integrated
- minions/monsters/structures represented as runtime actors
- movement/pathing/collision system replaces orbit-only simplification

Primary targets:
- new `Simulation/src/world/*`
- `Simulation/src/engine/simulation_step/*`
- expanded `Simulation/src/simulation_contracts/runtime_actor_contracts.rs`

### Phase D: Macro Match Systems
Exit gates:
- lane waves, objective timers, structure states, and economy/xp loops active
- target selection and AI behavior include macro intents

Primary targets:
- new macro subsystems + scenario/match orchestrator modules
- `Simulation/src/scenario_runner/*` extension from combat-runner to match-runner

### Phase E: Renderer/Replay And Calibration
Exit gates:
- stable snapshot/event contract
- deterministic replay tooling
- calibration and golden suites enforced in CI

Primary targets:
- `Simulation/src/engine/trace_snapshot_reporting/*`
- `Simulation/src/reporting/*`
- new calibration/replay harness modules

## Progress Tracking Model For This Goal
Track completion in five weighted buckets:

1. Runtime Systems Completeness (30%)
- world, actors, combat, lifecycle, objectives, economy, AI

2. Determinism And Replay Guarantees (20%)
- seeded reproducibility, ordering proofs, replay checksums

3. Calibration And Correctness (20%)
- interaction goldens, invariant/property suites, confidence thresholds

4. Performance Envelope (15%)
- tick throughput, memory bounds, budget behavior under load

5. Renderer-Contract Readiness (15%)
- snapshot/event schema stability and playback fidelity

Status labels:
- `NOT_STARTED`
- `IN_PROGRESS`
- `VALIDATING`
- `DONE`
- `BLOCKED`

## Current Status Snapshot (2026-02-24)
Overall weighted completion estimate for this blueprint: `42%` (`IN_PROGRESS`).

Bucket status (complete / remaining):
- Runtime Systems Completeness (`30%` weight): `31% / 69%`
  - what is done: deterministic combat kernel, scripted champion channels, runtime effect hooks, world-state skeleton with deterministic encounter placement validation
  - largest remaining gap: world state is not yet integrated into step-time pathing/collision, and no non-champion actor ecology or macro match systems exist yet
- Determinism And Replay Guarantees (`20%` weight): `57% / 43%`
  - what is done: fixed-tick loop, seed controls, deterministic ordering discipline in major search/runtime paths, fail-fast controlled-script initialization, and guarded event-resolution fallback paths without non-test `expect(...)` crash points
  - largest remaining gap: no replay checksum verifier and no full-match deterministic replay contract
- Calibration And Correctness (`20%` weight): `47% / 53%`
  - what is done: strong regression coverage for current combat/search scope, fail-fast schema validation in key paths, and new world/script registration guardrails
  - largest remaining gap: no golden interaction harness/property-suite for full-system invariants
- Performance Envelope (`15%` weight): `47% / 53%`
  - what is done: broad parallelization and improved runtime diagnostics
  - largest remaining gap: coverage-stage fixed-cost latency and no enforced CI performance budgets
- Renderer-Contract Readiness (`15%` weight): `30% / 70%`
  - what is done: schema-versioned trace/report artifacts with stable structured events for current combat scope plus normalized world-state ownership scaffolding
  - largest remaining gap: no full world snapshot contract and no replay-loader validation loop

Phase-level status:
- Phase A: `IN_PROGRESS`
- Phase B: `IN_PROGRESS`
- Phase C: `IN_PROGRESS`
- Phase D: `NOT_STARTED`
- Phase E: `NOT_STARTED`

## Current Non-Data High Friction
- runtime remains optimized for combat-scenario evaluation, not full-map lifecycle orchestration
- simplified movement and event taxonomy constrain expansion into full game behavior
- runtime crash surfaces are now concentrated in defaults-loader panic paths (current scan: `0` non-test `expect(...)` and `26` non-test `panic!` callsites under `Simulation/src`)
- controlled-champion script registry is still static and low-coverage (`Vladimir`, `Sona`) relative to full roster requirements

## Immediate Next Work (Execution-Ready)
1. Introduce actor-class abstraction for non-champion entities (minion/monster/structure) and owner lifecycle channels.
2. Integrate `src/world/*` ownership into simulation-step movement/pathing channels (replace isolated enemy-position ownership paths).
3. Expand event taxonomy to include non-combat match events (spawn/objective/economy/vision).
4. Replace movement-orbit simplification with command/path model.
5. Complete generic status/cast/projectile effect-instance registries.
6. Add full target-selection policy module and AI intent channels.
7. Add economy/xp subsystem and recall/shop state machine.
8. Add objective/structure subsystem with match-end conditions.
9. Add replay contract schema versioning and deterministic playback verifier.
10. Stand up calibration harness + golden scenario suites with confidence gates.
11. Expand controlled-champion script coverage beyond `Vladimir` and `Sona` and evolve registry wiring to reduce static coupling.

## Governance
- This blueprint is the canonical full-game target document.
- Keep this file aligned with:
  - `Simulation/IMPLEMENTATION_ROADMAP.md`
  - `Simulation/IMPROVEMENT_TRACKER.md`
  - `Simulation/CURRENT_STATE.md`
  - `Simulation/COVERAGE_GAPS.md`
- Every major phase completion must update:
  - phase status
  - exit-gate evidence
  - remaining blockers/friction

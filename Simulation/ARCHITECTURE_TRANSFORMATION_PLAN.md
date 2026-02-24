# Architecture Transformation Plan

This document defines the target architecture, migration sequencing, and progress tracking for refactoring `Simulation/src/` toward smaller, explicit, ownership-scoped modules.

Standards source: `ARCHITECTURE_STANDARDS.md`

## 1) Scope And Goals

Goals:

- split oversized files into concern-specific modules
- enforce explicit ownership and mutation channels
- keep subsystem facades stable while internals move
- reduce coupling between `scenario_runner`, `search`, and `engine`
- improve readability, testability, and deterministic behavior confidence

Non-goals:

- no gameplay behavior changes as part of architectural extraction work
- no champion/item/rune logic migration from scripts into shared core
- no schema ownership shifts unless explicitly planned and documented

## 2) Baseline Snapshot (2026-02-23)

### Core file sizes

- `src/scenario_runner.rs`: 4284 lines
- `src/engine.rs`: 3579 lines
- `src/defaults.rs`: 2455 lines
- `src/search.rs`: 2244 lines
- `src/data.rs`: 2008 lines
- `src/reporting.rs`: 1075 lines
- `src/core.rs`: 933 lines
- `src/main.rs`: 679 lines

### Structure baseline

- files over 700 lines: `7`
- `mod.rs` files under `src/`: `12`
- large shared-core hotspots: `engine.rs`, `scenario_runner.rs`, `search.rs`, `defaults.rs`, `data.rs`

### Coupling snapshot

- `scenario_runner.rs` currently imports many search helpers and strategy functions directly.
- `engine.rs` has been reduced below facade budget, but `engine/event_resolution/combat_event_dispatch_resolution.rs` remains a high-complexity hotspot.
- `defaults.rs` currently mixes typed schemas, file IO, and many domain-specific loader helpers.
- `data.rs` currently mixes path resolution, parsing, validation, runtime selection shaping, and item/champion data loading.

## 3) Target Layout (Proposed)

The target keeps subsystem facades and moves implementation into explicit leaf modules.

```text
src/
  main.rs
  simulation_contracts.rs
  simulation_contracts/
    runtime_actor_contracts.rs
    search_reporting_contracts.rs
    entrypoint_cli_contracts.rs
  core.rs
  core/
    combat_primitives_state.rs
    cast_lock_state.rs
    status_effect_state.rs
    item_stat_math.rs
    objective_scoring_math.rs
    random_seed_helpers.rs

  engine.rs
  engine/
    combat_timing_and_targeting.rs
    enemy_combat_stat_modeling.rs
    trace_snapshot_reporting.rs
    geometry/
      vector_2d_math.rs
      segment_intersection_checks.rs
      hitbox_distance_checks.rs
      range_reach_checks.rs
    actor_state/
      controlled_champion_runtime_state.rs
      enemy_runtime_state.rs
    event_queue/
      event_type_catalog.rs
      queued_event_record.rs
      event_queue_ordering.rs
      event_queue_scheduler.rs
    event_resolution/
      auto_attack_event_resolution.rs
      ability_cast_event_resolution.rs
      incoming_impact_resolution.rs
      incoming_damage_resolution.rs
      survivability_item_resolution.rs
    simulation_step/
      simulation_tick_stepper.rs
      enemy_movement_step.rs
      respawn_step.rs

  search.rs
  search/
    candidate_space/
      loadout_candidate_key.rs
      candidate_generation.rs
      candidate_repair.rs
      candidate_mutation.rs
      candidate_crossover.rs
      candidate_deduplication.rs
    strategy/
      beam_search_strategy.rs
      random_search_strategy.rs
      hill_climb_strategy.rs
      genetic_search_strategy.rs
      simulated_annealing_strategy.rs
      mcts_strategy.rs
      portfolio_strategy_orchestration.rs
    scoring/
      build_objective_scoring.rs
      candidate_metric_computation.rs
      pareto_front_selection.rs
      diversity_selection.rs
    diagnostics/
      strategy_diagnostics.rs
      candidate_space_estimation.rs

  scenario_runner.rs
  scenario_runner/
    scenario_contracts.rs
    controlled_champion_scenario_runner.rs
    controlled_champion_search_runtime_support.rs
    scenario_parsing.rs
    encounter_parsing.rs
    coverage_stage_runner.rs
    search_execution_runner.rs
    fixed_loadout_runner.rs
    rune_sweep_runner.rs
    run_output_paths.rs
    strict_ranking_ordering.rs
    search_space_estimation.rs
    structured_trace_encoding.rs
    progress_reporting.rs

  defaults.rs
  defaults/
    simulator_defaults_loader.rs
    champion_simulation_defaults_loader.rs
    champion_ai_defaults_loader.rs
    game_mode_defaults_loader.rs
    item_effect_defaults_loader.rs
    schema_helpers.rs

  data.rs
  data/
    scenario_path_resolution.rs
    champion_data_loading.rs
    item_data_loading.rs
    loadout_domain_generation.rs
    rune_page_legality.rs
    enemy_preset_validation.rs

  reporting.rs
  reporting/
    markdown_report_writer.rs
    json_report_writer.rs
    formatting_helpers.rs
    diagnostics_sections.rs

  build_order.rs
  cache.rs
  respawn.rs
  status.rs
  scripts.rs
  scripts/
    ...
```

## 4) Module Boundary Contracts

These contracts must hold during and after refactor.

### 4.1 `engine` facade contract

- `engine.rs` remains the sole external facade for engine internals.
- preserve exports consumed by other modules:
  - `ControlledChampionCombatSimulation`
  - `EnemyDerivedCombatStats`
  - `derive_enemy_combat_stats(...)`
- leaf modules under `engine/*` should not be imported directly by `main.rs` or `scenario_runner.rs`.

### 4.2 `search` facade contract

- `search.rs` remains the sole external facade for search internals.
- preserve functions/types currently consumed by `scenario_runner.rs` through facades or re-exports:
  - `FullLoadoutSearchParams`
  - `build_search_ranked_full_loadout(...)`
  - `adaptive_strategy_candidates_full_loadout(...)`
  - `strategy_seed_elites_full_loadout(...)`
  - `generate_bleed_candidates_full_loadout(...)`
  - `candidate_pareto_front_keys(...)`
  - `select_diverse_top_candidates(...)`
  - `compute_build_metrics_for_candidate(...)`
  - `portfolio_strategy_list(...)`
  - `search_strategy_summary(...)`
  - `item_names(...)`
  - `choose_best_build_by_stat(...)`
- internal strategy files should not call scenario-runner-specific code.

### 4.3 `scenario_runner` facade contract

- `scenario_runner.rs` should expose mode entrypoints only:
  - `run_controlled_champion_scenario(...)`
  - `run_controlled_champion_stepper(...)`
  - `run_controlled_champion_fixed_loadout_evaluation(...)`
  - `run_controlled_champion_fixed_loadout_rune_sweep(...)`
  - `run_stat_optimization(...)`
- output formatting and path composition should move into dedicated leaf modules.

### 4.4 `defaults` and `data` contracts

- `defaults.rs` exposes typed getters and cached loader access only.
- `data.rs` exposes parse/validate/load helpers only.
- no simulation orchestration logic should live in `defaults/*` or `data/*`.

### 4.5 `reporting` contract

- reporting computes serialization/rendering only.
- reporting must not own search or engine orchestration decisions.

## 5) Ownership Channel Migration Map

| Channel | Current write sites | Target owner API | End-state rule |
|---|---|---|---|
| Event queue writes | `engine.rs` methods | `engine/event_queue/event_queue_scheduler.rs` | only queue owner mutates queue internals |
| Damage/heal writes | `engine.rs` damage helpers | `engine/event_resolution/incoming_damage_resolution.rs` | all health/mitigation writes through resolution APIs |
| Candidate mutation | `search.rs`, some `scenario_runner.rs` orchestration paths | `search/candidate_space/*` | only candidate-space owner mutates candidate keys |
| Runtime counters and unique-loadout diagnostics helpers | `scenario_runner/progress_reporting.rs` and `scenario_runner.rs` (partial migration) | `scenario_runner/progress_reporting.rs` | counters/diagnostics helpers are opaque outside owner |
| Score cache mutation | `scenario_runner.rs` through `cache.rs` | `cache.rs` | no direct map mutation outside cache owner |
| Parse normalization state | `scenario_runner.rs`, `data.rs` | `scenario_runner/scenario_parsing.rs` and `scenario_runner/encounter_parsing.rs` | all scenario normalization through parser owners |

## 6) Sequencing Rules

1. Preserve facade API first, move implementation second.
2. Extract pure helper functions before stateful code.
3. Move owner modules before consumers to avoid duplicated ownership.
4. Keep compatibility re-exports until all internal call sites are migrated.
5. One subsystem migration track at a time to minimize merge risk.
6. No `mod.rs` cleanup for a subtree until that subtree’s behavior and tests are stable.

## 7) Sequencing Risks And Mitigations

| Risk | Description | Mitigation |
|---|---|---|
| Search API breakage | `scenario_runner` currently imports many search symbols directly | keep `search.rs` facade shims until all call sites move |
| Engine state leakage | splitting damage/queue code may expose private mutable fields | introduce explicit owner methods before extraction |
| Determinism drift | ordering changes during parallel or candidate-flow extraction | add fixed-seed regression runs before/after each phase |
| Performance regression | extra allocations/indirection in extracted modules | compare diagnostics counters and runtime before merge |
| `mod.rs` churn blast radius | mass rename of script trees can produce noisy diffs and break imports | migrate one subtree per phase with path-compatibility shims |

## 8) Detailed Phase Plan

### Phase A: Governance And Guardrails

Deliverables:

- maintain architecture standards and this plan as active governance documents
- add architecture PR checklist template (`ARCHITECTURE_REFACTOR_CHECKLIST.md`)
- establish baseline metrics table (this file)

Acceptance gates:

- docs are linked from root and simulator README files
- roadmap and improvement tracker include architecture-program status

### Phase B: Split `engine.rs`

Deliverables:

- extract geometry helpers to `engine/geometry/*`
- extract event queue model/scheduling to `engine/event_queue/*`
- extract damage and impact application to `engine/event_resolution/*`
- extract actor-state structs/helpers to `engine/actor_state/*`

Acceptance gates:

- `engine.rs` reduced to `<= 2000` lines
- no behavioral diff in `src/tests/engine_tests.rs`
- no direct queue/damage mutation outside owner modules

### Phase C: Split `search.rs`

Deliverables:

- extract candidate generation/repair/mutation/crossover
- extract strategy implementations into `search/strategy/*`
- extract scoring and metric helpers into `search/scoring/*`
- keep compatibility re-exports in `search.rs`

Acceptance gates:

- `search.rs` reduced to `<= 1300` lines
- `scenario_runner` compiles without direct imports from search leaf modules
- search regression tests and deterministic tie-break behavior remain stable

### Phase D: Split `scenario_runner.rs`

Deliverables:

- extract scenario parsing and encounter parsing
- extract coverage stage and strict ranking execution flows
- extract fixed-loadout and rune-sweep mode runners
- extract output path and trace encoding helpers

Acceptance gates:

- `scenario_runner.rs` reduced to `<= 2200` lines
- scenario-runner tests pass with unchanged behavior
- no direct search-candidate mutation logic remains in facade

### Phase E: Split `defaults.rs` And `data.rs`

Deliverables:

- split defaults loaders by domain owner file
- split data loading/parsing/validation by concern
- keep top-level typed facades and cache semantics stable

Acceptance gates:

- `defaults.rs` reduced to `<= 1400` lines
- `data.rs` reduced to `<= 1400` lines
- parser error quality remains explicit with canonical paths/keys

### Phase F: Split `reporting.rs` And `core.rs`

Deliverables:

- separate markdown/json writers, formatting helpers, and diagnostics sections
- split core combat primitives vs item/stat/objective/random helpers

Acceptance gates:

- `reporting.rs` reduced to `<= 700` lines
- `core.rs` reduced to `<= 700` lines
- reporting and core tests pass with unchanged outputs/logic

### Phase G: `mod.rs` Cleanup

Deliverables:

- remove remaining unnecessary `mod.rs` usage from shared-core and script trees
- replace with explicit module files and stabilized re-exports

Acceptance gates:

- every remaining `mod.rs` has explicit justification or removal milestone
- imports compile without path ambiguity regressions

### Phase H: Final Stabilization

Deliverables:

- tighten facades to orchestration-only role
- remove expired compatibility shims
- refresh architecture metrics baseline

Acceptance gates:

- facade files (`engine.rs`, `search.rs`, `scenario_runner.rs`, `defaults.rs`, `data.rs`, `reporting.rs`, `core.rs`) are all `<= 700` lines
- architecture milestone tracker reaches `DONE` or explicit `BLOCKED` statuses with rationale

## 9) `mod.rs` Migration Detail

Current `mod.rs` inventory under `src/`:

- none (`0` files; verified `2026-02-24`)

Planned migration convention:

- subtree with children: migrate to `<module>.rs` facade + `<module>/` folder
- leaf champion modules with no children: prefer `<champion>.rs`
- subtrees with heavy churn risk: keep `mod.rs` until subtree stabilization milestone is done

## 10) Progress Metrics And Checkpoints

Track and update at each architecture milestone close:

| Metric | Baseline | Current | Target |
|---|---|---|---|
| `engine.rs` line count | 3579 | 601 | <= 700 |
| `scenario_runner.rs` line count | 4284 | 285 | <= 700 |
| `search.rs` line count | 2244 | 569 | <= 700 |
| `defaults.rs` line count | 2455 | 386 | <= 700 |
| `data.rs` line count | 2008 | 116 | <= 700 |
| `reporting.rs` line count | 1075 | 139 | <= 700 |
| `core.rs` line count | 933 | 162 | <= 700 |
| count of `mod.rs` files under `src/` | 12 | 0 | minimal/justified |
| architecture milestones complete | 1/18 | 18/18 | 18/18 |

Convenience metrics command:

- `Simulation/tools/architecture_metrics.sh` prints current line budgets and size-based progress percentages.

## 11) Milestone Tracker

Status legend:

- `DONE`
- `IN_PROGRESS`
- `PLANNED`
- `BLOCKED`

| ID | Status | Scope | Exit Criteria |
|---|---|---|---|
| ARCH-001 | DONE | Add architecture standards and transformation plan docs | docs added and linked in READMEs |
| ARCH-002 | DONE | Add architecture PR checklist template | checklist file added and linked in docs |
| ARCH-010 | DONE | Extract engine geometry modules | geometry helpers and position-update ownership moved out of facade, and `engine.rs <= 3000` |
| ARCH-011 | DONE | Extract engine event queue modules | queue types/scheduling moved and queue channel hardened |
| ARCH-012 | DONE | Extract engine damage/impact resolution modules | damage/impact/heal/revive writes routed through owner APIs with explicit owner-command naming |
| ARCH-013 | DONE | Extract engine actor-state modules | actor state helpers moved from facade |
| ARCH-014 | DONE | Split engine event-dispatch resolution hotspot | `src/engine/event_resolution/combat_event_dispatch_resolution.rs` decomposed into smaller owner slices with stable event-order behavior |
| ARCH-020 | DONE | Extract search candidate-space modules | candidate operations outside `search.rs` |
| ARCH-021 | DONE | Extract search strategy modules | strategy implementations in `search/strategy/*` |
| ARCH-022 | DONE | Extract search scoring modules | scoring/metrics/diversity outside strategy files |
| ARCH-030 | DONE | Extract scenario parsing and diagnostics modules | parsing/output/progress/strict-ranking helpers moved out of facade |
| ARCH-031 | DONE | Extract scenario execution modules | coverage/search/fixed-loadout flows moved out of facade |
| ARCH-040 | DONE | Split defaults by domain loaders | `defaults.rs` is a typed facade with domain leaf loaders |
| ARCH-041 | DONE | Split data by concern | `data.rs` is a parse/load facade with explicit leaf concern modules |
| ARCH-050 | DONE | Split reporting and formatting modules | reporting decomposition complete with stable outputs |
| ARCH-051 | DONE | Split core utility clusters | core combat primitives/math/random/build-key helpers separated into explicit `src/core/*` owner modules |
| ARCH-060 | DONE | Reduce/justify remaining `mod.rs` usage | each `mod.rs` removed or explicitly justified |
| ARCH-070 | DONE | Remove compatibility shims and stabilize | no stale shims, all milestones reconciled in docs |

Progress summary:

- Completed: `18/18` milestones (`100.0%`)
- In progress: `0/18`
- Planned: `0/18`

Recent progress log:

- 2026-02-23: `ARCH-002` completed by adding `ARCHITECTURE_REFACTOR_CHECKLIST.md`.
- 2026-02-23: `ARCH-010` started by moving `Vec2`, segment intersection checks, hitbox distance checks, and hitbox reach helpers from `engine.rs` into:
  - `src/engine/geometry.rs`
  - `src/engine/geometry/vector_2d_math.rs`
  - `src/engine/geometry/segment_intersection_checks.rs`
  - `src/engine/geometry/hitbox_distance_checks.rs`
  - `src/engine/geometry/range_reach_checks.rs`
- 2026-02-23: `ARCH-010` continued by moving spawn/projectile kinematics and adding focused geometry module tests:
  - `src/engine/geometry/spawn_positioning.rs`
  - `src/engine/geometry/projectile_kinematics.rs`
  - `src/engine/geometry/tests/geometry_module_tests.rs`
  - `engine.rs` reduced further from `3447` to `3422` lines.
- 2026-02-23: `ARCH-010` continued by extracting movement-vector math and script-point conversion helpers:
  - `src/engine/geometry/enemy_orbit_position_updates.rs`
  - `src/engine/script_point_coordinate_conversions.rs`
  - `src/engine/tests/script_point_coordinate_conversions_tests.rs`
  - `src/engine/geometry/tests/geometry_module_tests.rs` expanded with orbit-position update coverage
  - `engine.rs` reduced further from `3422` to `3397` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-011` started by extracting queue ownership into explicit event-queue modules:
  - `src/engine/event_queue.rs`
  - `src/engine/event_queue/event_type_catalog.rs`
  - `src/engine/event_queue/queued_event_record.rs`
  - `src/engine/event_queue/event_queue_ordering.rs`
  - `src/engine/event_queue/event_queue_scheduler.rs`
  - `src/engine/event_queue/tests/event_queue_scheduler_tests.rs`
  - rewired `engine.rs` to use owner APIs for enqueue/peek/pop/recurring-reschedule
  - `engine.rs` reduced further from `3397` to `3315` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-011` completed by moving queue projection/query helpers behind explicit queue owner APIs:
  - added owner-query APIs in `src/engine/event_queue/event_queue_scheduler.rs`:
    - `next_enemy_attack_ready_at`
    - `next_enemy_attack_impact_at`
    - `next_controlled_champion_attack_ready_at`
    - `next_controlled_champion_attack_impact_at`
    - `queued_projectile_impact_projections`
  - added explicit projectile-impact projection type in `src/engine/event_queue/queued_projectile_impact_projection.rs`
  - rewired `engine.rs` snapshot/reporting helpers to consume queue owner query APIs (no direct queue scanning)
  - expanded queue unit tests in `src/engine/event_queue/tests/event_queue_scheduler_tests.rs`
  - `engine.rs` reduced further from `3315` to `3277` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-012` started by extracting incoming-damage ownership into explicit event-resolution modules:
  - `src/engine/event_resolution.rs`
  - `src/engine/event_resolution/incoming_damage_resolution.rs`
  - moved controlled champion/enemy damage application and runtime-heal mutation paths out of facade into event-resolution owner module
  - `engine.rs` reduced further from `3277` to `3013` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-012` completed by hardening explicit owner-command channels for controlled champion damage/heal/revive:
  - replaced ambiguous command names with explicit event-resolution owner APIs:
    - `apply_incoming_damage_to_controlled_champion`
    - `apply_healing_to_controlled_champion`
    - `apply_revive_or_mark_controlled_champion_death`
  - rewired engine call sites to use `apply_incoming_damage_*`, `apply_healing_*`, and `apply_revive_*` channels
  - moved revive resolution ownership from `engine.rs` into `src/engine/event_resolution/incoming_damage_resolution.rs`
  - updated engine regression tests to match explicit owner-command naming
  - `engine.rs` reduced further from `3013` to `2980` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-010` completed by extracting the remaining actor-position owner loop into a dedicated simulation-step owner module:
  - `src/engine/simulation_step.rs`
  - `src/engine/simulation_step/enemy_movement_step.rs`
  - rewired `engine.rs` hot-effect tick flow to call `apply_enemy_movement_step` owner command instead of an inline facade loop
  - updated movement-step regression call sites in `src/tests/engine_tests.rs`
  - `engine.rs` reduced further from `2980` to `2940` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-013` started by extracting enemy actor-state lifecycle ownership into explicit actor-state modules:
  - `src/engine/actor_state.rs`
  - `src/engine/actor_state/enemy_runtime_state.rs`
  - moved respawn lifecycle and regeneration mutation loops behind owner commands:
    - `apply_enemy_respawn_updates`
    - `apply_enemy_regeneration_tick`
  - moved enemy active/alive query helpers behind actor-state owner module:
    - `enemy_is_alive`
    - `enemy_is_active`
  - rewired `engine.rs` hot-effect and event-loop call sites to consume these owner commands
  - `engine.rs` reduced further from `2940` to `2880` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-013` continued by moving recurring-script lifecycle checks and script-cadence writes behind actor-state owner APIs:
  - added owner commands/queries in `src/engine/actor_state/enemy_runtime_state.rs`:
    - `enemy_script_event_should_recur`
    - `enemy_ability_haste_or_urf_default`
    - `set_enemy_script_event_ready_at`
    - `apply_enemy_next_attack_bonus_physical`
  - rewired `engine.rs` to consume owner APIs for recurring-event eligibility, script-event readiness map updates, and enemy next-attack physical-bonus mutation
  - `engine.rs` reduced further from `2880` to `2867` lines.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-013` continued by moving enemy auto-attack token lifecycle and next-hit bonus consume/reset behind actor-state owner APIs:
  - added owner commands/queries in `src/engine/actor_state/enemy_runtime_state.rs`:
    - `begin_enemy_attack_sequence`
    - `enemy_attack_sequence_matches`
    - `consume_enemy_attack_damage_with_on_hit`
  - rewired `engine.rs` attack event flow to use owner APIs instead of facade-side token writes and bonus-reset mutation blocks
  - hardened attack-event index invariants with explicit fail-fast checks at owner-call boundaries (no silent drop on impossible invalid index)
  - `engine.rs` reduced further from `2867` to `2839` lines.
  - correctness review pass recorded no behavioral findings; remaining direct mutable enemy-state call sites in facade are down to two script-runtime mutation sites.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-013` continued by removing the remaining two direct mutable enemy-state facade call sites in script-action and script-execution flow:
  - added owner commands/queries in `src/engine/actor_state/enemy_runtime_state.rs`:
    - `execute_enemy_script_event_actions`
    - `enemy_aftershock_magic_damage_on_immobilize`
    - `enemy_script_epoch_matches`
    - `enemy_script_event_ready_at_or_zero`
  - rewired `engine.rs` champion-script event flow and script-action runtime mutation paths to consume owner APIs (no facade-side `&mut self.enemy_state[idx]` borrow sites remain)
  - added focused actor-state regression tests in `src/tests/engine_tests.rs`:
    - `enemy_attack_sequence_owner_methods_advance_and_invalidate_old_tokens`
    - `enemy_attack_bonus_physical_is_consumed_once_and_resets_after_hit`
    - `enemy_script_epoch_and_ready_queries_read_owner_state`
    - `enemy_script_execution_owner_method_generates_actions_for_in_range_event`
    - `enemy_aftershock_owner_method_is_zero_without_aftershock_rune`
  - `engine.rs` reduced further from `2839` to `2813` lines.
  - correctness review pass recorded no behavioral findings; direct mutable enemy-state facade borrow count is now `0`.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-013` continued by extracting high-traffic enemy read projections behind actor-state owner query APIs:
  - added owner query channels in `src/engine/actor_state/enemy_runtime_state.rs`:
    - `enemy_name`
    - `enemy_position`
    - `enemy_hitbox_radius`
    - `enemy_attack_range`
    - `enemy_attack_windup_seconds`
    - `enemy_attack_projectile_speed`
    - `enemy_attack_effect_hitbox_radius`
    - `enemy_attack_interval_seconds`
    - `enemy_target_health_snapshot_or_defaults`
    - `enemy_status_lines_at`
    - `enemy_is_stunned_at`
    - `enemy_is_invulnerable_or_untargetable_at`
  - rewired `engine.rs` hot-path call sites (enemy attacks, controlled-champion attack targeting, script damage traces, projectile-line diagnostics, and target-health snapshots) to use actor-state query APIs
  - added focused regression test `enemy_read_projection_owner_queries_return_expected_shapes` in `src/tests/engine_tests.rs`
  - `engine.rs` reduced further from `2813` to `2794` lines.
  - correctness review pass recorded no behavioral findings; direct `enemy_state[idx]`/`enemy_state.get(...)` access in `engine.rs` is now `0`.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-013` continued by extracting trace-snapshot enemy-section read composition behind actor-state owner projection APIs:
  - added owner query/projection channels in `src/engine/actor_state/enemy_runtime_state.rs`:
    - `enemy_count`
    - `enemy_trace_snapshot_at`
    - `EnemyTraceSnapshot`
  - rewired `engine.rs` state-snapshot assembly to consume actor-state projection APIs instead of facade-side `enemy_state` iteration/read composition
  - extended focused regression test `enemy_read_projection_owner_queries_return_expected_shapes` in `src/tests/engine_tests.rs` with `enemy_trace_snapshot_at` assertions
  - `engine.rs` reduced further from `2794` to `2778` lines.
  - correctness review pass recorded no behavioral findings; `engine.rs` `enemy_state` references now remain only at declaration/bootstrap sites.
  - validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-020` started by extracting the first search candidate-space owner module for full-loadout candidate operations:
  - added candidate-space module files:
    - `src/search/candidate_space.rs`
    - `src/search/candidate_space/full_loadout_candidate_operations.rs`
  - moved full-loadout candidate helper ownership out of facade-local helper cluster:
    - `candidate_order_key`
    - `random_full_candidate`
    - `candidate_loadout_variants`
    - `repair_full_candidate`
    - `mutate_full_candidate`
    - `crossover_full_candidates`
  - rewired `search.rs` facade call sites to consume candidate-space owner module helpers.
  - added focused module-private regression tests:
    - `candidate_order_key_tracks_item_and_loadout_slots`
    - `candidate_loadout_variants_deduplicates_anchor_and_base`
    - `candidate_loadout_variants_includes_anchor_and_base_when_different`
  - `search.rs` reduced further from `2244` to `2127` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-020` continued by extracting full-loadout scoring/ranking and item-only candidate-operation helper ownership into explicit candidate-space modules:
  - added candidate-space owner modules:
    - `src/search/candidate_space/full_loadout_candidate_scoring.rs`
    - `src/search/candidate_space/item_candidate_operations.rs`
  - moved ownership out of facade-local `search.rs` helper clusters:
    - full-loadout scoring/ranking:
      - `score_full_candidates`
      - `unique_ranked_full_candidates`
    - item-only candidate mutation/crossover/parent selection:
      - `tournament_parent`
      - `crossover_builds`
      - `mutate_build`
  - rewired `search.rs` and candidate-space call sites to consume owner modules (`full_loadout_candidate_scoring`, `item_candidate_operations`) instead of local helper implementations.
  - added focused candidate-space regression tests:
    - `src/search/candidate_space/tests/full_loadout_candidate_scoring_tests.rs`
    - `src/search/candidate_space/tests/item_candidate_operations_tests.rs`
  - `search.rs` reduced further from `2127` to `2009` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-020` continued by extracting item-only candidate scoring/dedupe ownership into an explicit candidate-space module:
  - added candidate-space owner module:
    - `src/search/candidate_space/item_candidate_scoring.rs`
  - moved ownership out of facade-local `search.rs` helper cluster:
    - `score_candidates`
    - `unique_ranked_from_candidates`
  - rewired `search.rs` call sites to consume `item_candidate_scoring` owner APIs instead of local helper implementations.
  - added focused candidate-space regression tests:
    - `src/search/candidate_space/tests/item_candidate_scoring_tests.rs`
    - `score_candidates_scores_each_unique_canonical_key_once`
    - `unique_ranked_from_candidates_dedupes_keys_and_filters_non_finite_scores`
    - `score_candidates_returns_empty_when_deadline_reached`
  - `search.rs` reduced further from `2009` to `1937` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-021` started by extracting item-only strategy implementations into explicit strategy owner modules:
  - added strategy owner module files:
    - `src/search/strategy.rs`
    - `src/search/strategy/item_candidate_search_strategies.rs`
  - moved item-only strategy helper ownership out of facade-local `search.rs` helper clusters:
    - `beam_search_ranked`
    - `random_search_ranked`
    - `hill_climb_search_ranked`
    - `genetic_search_ranked`
    - `simulated_annealing_search_ranked`
    - `mcts_search_ranked`
  - moved item-only strategy-local rollout/selection helpers under strategy ownership:
    - `available_actions`
    - `rollout_completion`
  - rewired `search.rs` strategy dispatch to consume strategy owner APIs from `search/strategy/*`.
  - added focused strategy-module regression tests:
    - `src/search/strategy/tests/item_candidate_search_strategies_tests.rs`
    - `available_actions_respects_boot_and_duplicate_constraints`
    - `rollout_completion_returns_legal_canonical_key_and_expected_score`
    - `mcts_search_ranked_returns_unique_legal_candidates_with_limit`
  - `search.rs` reduced further from `1937` to `1496` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-021` completed by extracting full-loadout strategy implementations into explicit strategy owner modules:
  - added strategy owner module file:
    - `src/search/strategy/full_loadout_search_strategies.rs`
  - moved full-loadout strategy helper ownership out of facade-local `search.rs` helper clusters:
    - `beam_search_ranked_full`
    - `random_search_ranked_full`
    - `hill_climb_search_ranked_full`
    - `genetic_search_ranked_full`
    - `simulated_annealing_search_ranked_full`
    - `mcts_search_ranked_full`
  - moved full-loadout strategy-local helpers under strategy ownership:
    - `tournament_parent_full`
    - `MctsFullNode`
  - rewired `search.rs` full-loadout strategy dispatch to consume strategy owner APIs from `search/strategy/*`.
  - added focused strategy-module regression tests:
    - `src/search/strategy/tests/full_loadout_search_strategies_tests.rs`
    - `random_search_ranked_full_respects_limit_and_candidate_legality`
    - `beam_search_ranked_full_returns_legal_candidates`
    - `mcts_search_ranked_full_returns_unique_legal_candidates_with_limit`
  - `search.rs` reduced further from `1496` to `1111` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-022` started by extracting explicit search scoring/diversity owner modules:
  - added scoring owner module files:
    - `src/search/scoring.rs`
    - `src/search/scoring/metric_scoring_helpers.rs`
    - `src/search/scoring/item_build_scoring_and_diversity.rs`
    - `src/search/scoring/full_loadout_scoring_and_diversity.rs`
  - moved facade-local scoring/diversity helper ownership out of `search.rs` helper clusters:
    - `select_diverse_top_builds`
    - `compute_build_metrics`
    - `pareto_front_keys`
    - `select_diverse_top_candidates`
    - `compute_build_metrics_for_candidate`
    - `candidate_pareto_front_keys`
  - rewired `search.rs` to keep facade compatibility through thin owner-wrapper functions while routing logic to `search/scoring/*`.
  - added focused scoring-module regression tests:
    - `src/search/scoring/tests/item_build_scoring_and_diversity_tests.rs`
    - `select_diverse_top_builds_applies_gap_and_diversity_filters`
    - `pareto_front_keys_excludes_dominated_entries`
    - `src/search/scoring/tests/full_loadout_scoring_and_diversity_tests.rs`
    - `select_diverse_top_candidates_applies_gap_and_diversity_filters`
    - `candidate_pareto_front_keys_excludes_dominated_entries`
  - `search.rs` reduced further from `1111` to `971` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-022` continued by extracting stat-key item-build selection into explicit scoring owner module:
  - added scoring owner module file:
    - `src/search/scoring/stat_key_build_selection.rs`
  - moved stat-key build-selection helper ownership out of facade-local `search.rs` helper cluster:
    - `choose_best_build_by_stat`
  - rewired `search.rs` stat-key build-selection facade path to thin owner wrapper over `search/scoring/stat_key_build_selection.rs`.
  - added focused scoring-module regression tests:
    - `src/search/scoring/tests/stat_key_build_selection_tests.rs`
    - `choose_best_build_by_stat_returns_empty_when_no_slots_requested`
    - `choose_best_build_by_stat_respects_single_boot_constraint`
    - `choose_best_build_by_stat_picks_highest_stat_bundle`
  - `search.rs` reduced further from `971` to `946` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-022` continued by extracting item-name list formatting into explicit scoring owner module:
  - added scoring owner module file:
    - `src/search/scoring/item_name_list_formatting.rs`
  - moved item-name list formatting helper ownership out of facade-local `search.rs` helper cluster:
    - `item_names`
  - rewired `search.rs` item-name formatting facade path to thin owner wrapper over `search/scoring/item_name_list_formatting.rs`.
  - added focused scoring-module regression tests:
    - `src/search/scoring/tests/item_name_list_formatting_tests.rs`
    - `format_item_name_list_comma_separated_returns_empty_for_empty_list`
    - `format_item_name_list_comma_separated_returns_single_name_without_separator`
    - `format_item_name_list_comma_separated_joins_multiple_names_in_order`
  - `search.rs` reduced further from `946` to `942` lines.
  - correctness review pass recorded no behavioral findings; validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-030` started by extracting scenario parse ownership into the first dedicated scenario-runner parser module:
  - added parser owner module file:
    - `src/scenario_runner/scenario_parsing.rs`
  - moved parser/validation helper ownership out of facade-local `scenario_runner.rs` helper cluster:
    - `parse_controlled_champion_config`
    - `parse_opponent_encounters`
    - `parse_scenario_search_or_default`
  - rewired `scenario_runner.rs` call sites to consume parser owner APIs from `scenario_runner/scenario_parsing.rs`.
  - added focused regression coverage:
    - `src/tests/scenario_runner_tests.rs`
    - `parse_controlled_champion_config_rejects_legacy_baseline_items_key`
  - `scenario_runner.rs` reduced further from `4284` to `4122` lines.
  - targeted parser regression tests remained green.
- 2026-02-23: `ARCH-030` continued by extracting encounter-focused parsing ownership into a dedicated owner module with typed parse output:
  - added encounter parser owner module file:
    - `src/scenario_runner/encounter_parsing.rs`
  - moved encounter parser ownership out of `src/scenario_runner/scenario_parsing.rs`:
    - `parse_opponent_encounters`
  - introduced explicit typed parse output:
    - `ParsedOpponentEncounter`
  - rewired `scenario_runner.rs` encounter parse boundaries to consume typed parser-owner APIs from `scenario_runner/encounter_parsing.rs`.
  - added focused regression coverage:
    - `src/tests/scenario_runner_tests.rs`
    - `parse_opponent_encounters_preserves_typed_encounter_fields`
  - `scenario_runner.rs` currently at `4127` lines (facade still in-progress for further extraction).
  - targeted encounter parser regression tests remained green.
- 2026-02-23: `ARCH-030` continued by extracting scenario run-output path/key ownership into an explicit owner module:
  - added run-output owner module file:
    - `src/scenario_runner/run_output_paths.rs`
  - moved run-output helper ownership out of `src/scenario_runner.rs` helper cluster:
    - `format_repo_relative_path`
    - `search_quality_profile_key`
    - `default_run_output_directory`
    - `default_fixed_loadout_output_directory`
    - `default_fixed_loadout_rune_sweep_output_directory`
  - rewired `scenario_runner.rs` output-path/reporting call sites to consume `scenario_runner/run_output_paths.rs` owner APIs.
  - added focused regression coverage:
    - `src/tests/scenario_runner_tests.rs`
    - `default_run_output_directory_compacts_popcorn_window_when_equal_to_budget`
    - `default_fixed_loadout_output_directory_normalizes_label_key`
    - `format_repo_relative_path_uses_repository_relative_simulation_paths`
  - `scenario_runner.rs` reduced further from `4127` to `4010` lines.
  - targeted output-path regression tests remained green.
- 2026-02-23: `ARCH-030` continued by extracting scenario search-progress/runtime-counter ownership into an explicit owner module:
  - added progress owner module file:
    - `src/scenario_runner/progress_reporting.rs`
  - moved progress-helper ownership out of `src/scenario_runner.rs` helper cluster:
    - `initialize_search_type_counters`
    - `increment_search_type_counter`
    - `snapshot_search_type_counters`
    - `unique_loadout_selection_count`
    - `unique_loadout_selection_count_from_ranked`
  - rewired `scenario_runner.rs` progress and diagnostics call sites to consume `scenario_runner/progress_reporting.rs` owner APIs.
  - existing focused regression coverage in `src/tests/scenario_runner_tests.rs` remained green:
    - `unique_loadout_selection_count_helpers_track_distinct_loadouts`
  - `scenario_runner.rs` reduced further from `4010` to `3947` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-030` continued by extracting strict-ranking ordering and search-space estimation helper ownership into explicit owner modules:
  - added strict-ranking owner module file:
    - `src/scenario_runner/strict_ranking_ordering.rs`
  - added search-space estimation owner module file:
    - `src/scenario_runner/search_space_estimation.rs`
  - moved strict-ranking/estimation helper ownership out of `src/scenario_runner.rs` helper cluster:
    - `heuristic_sort_remaining_candidates_for_strict_ranking`
    - `estimated_legal_item_build_count`
    - `estimated_legal_loadout_count`
    - `estimate_close_to_optimal_probability`
    - `format_percent_display`
  - rewired `scenario_runner.rs` strict-ranking and diagnostics call sites to consume owner APIs.
  - added focused regression coverage:
    - `estimated_legal_item_build_count_applies_single_boot_constraint`
    - `estimated_legal_loadout_count_matches_small_domain_combinatorics`
    - `estimate_close_to_optimal_probability_reports_unavailable_when_space_missing`
    - `format_percent_display_uses_scientific_notation_for_tiny_percent_values`
    - `strict_ranking_heuristic_ordering_sorts_by_signal_when_enabled_without_promotions`
    - `strict_ranking_heuristic_ordering_keeps_input_order_when_scores_are_flat`
  - `scenario_runner.rs` reduced further from `3947` to `3709` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-031` started by extracting fixed-loadout and rune-sweep execution entrypoint ownership into dedicated scenario-runner execution modules:
  - added execution owner module files:
    - `src/scenario_runner/fixed_loadout_runner.rs`
    - `src/scenario_runner/rune_sweep_runner.rs`
  - moved execution-entrypoint implementation ownership out of `src/scenario_runner.rs`:
    - `run_controlled_champion_fixed_loadout_evaluation`
    - `run_controlled_champion_fixed_loadout_rune_sweep`
  - rewired `scenario_runner.rs` to keep thin facade wrappers that delegate to execution-owner APIs.
  - `scenario_runner.rs` reduced further from `3709` to `2708` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-031` completed by extracting controlled-champion scenario execution entrypoint ownership into a dedicated scenario-runner module:
  - added execution owner module file:
    - `src/scenario_runner/controlled_champion_scenario_runner.rs`
  - moved controlled-champion execution-entrypoint implementation ownership out of `src/scenario_runner.rs`:
    - `run_controlled_champion_scenario`
  - rewired `scenario_runner.rs` to keep a thin facade wrapper delegating to `run_controlled_champion_scenario_impl`.
  - `scenario_runner.rs` reduced further from `2708` to `936` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-040` started by extracting champion/item simulation-default loader ownership into the first defaults domain module:
  - added defaults owner module file:
    - `src/defaults/champion_item_simulation_defaults_loader.rs`
  - moved champion/item simulation-default loader clusters out of `src/defaults.rs`:
    - Vladimir cast/offensive/pool/policy defaults loaders
    - Warwick/Vayne/Morgana/Sona/Doctor Mundo ability defaults loaders
    - Zhonya/Guardian Angel/Protoplasm item simulation-default loaders
    - related champion/item ability-effect extraction helper functions
  - rewired `defaults.rs` to keep facade-owned `OnceLock` accessors while delegating loader logic to the new owner module.
  - `defaults.rs` reduced from `2455` to `1435` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-041` started by extracting champion/item/preset data loading ownership into the first data concern module:
  - added data owner module file:
    - `src/data/champion_item_preset_data_loading.rs`
  - moved champion/item/preset loading and URF legality helper ownership out of `src/data.rs`:
    - champion base loading and lookup
    - URF mode data loading
    - item stat mapping/loading and item-pool legality helpers
    - enemy preset loading, validation, and preset->loadout conversion
  - rewired `data.rs` to keep parse/config/loadout-domain facade responsibilities while delegating champion/item/preset loader concerns to the new owner module.
  - `data.rs` reduced from `2008` to `1400` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-041` continued and completed by extracting remaining data parse/loadout concerns into explicit owner modules:
  - added data owner module files:
    - `src/data/loadout_effect_resolution.rs`
    - `src/data/simulation_search_configuration_parsing.rs`
    - `src/data/loadout_domain_modeling.rs`
  - moved parse/config/loadout-domain/loadout-resolution ownership out of `src/data.rs` and rewired `src/data.rs` into a thin facade with explicit concern re-exports.
  - `data.rs` reduced from `1400` to `116` lines (meets facade target `<= 700`).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-013` continued by extracting trace/snapshot runtime reporting ownership out of `engine.rs`:
  - added engine owner module:
    - `src/engine/trace_snapshot_reporting.rs`
  - moved trace/snapshot helper ownership out of `src/engine.rs`:
    - `trace_event`
    - `trace_cooldown_status`
    - `collect_state_snapshot_summary`
    - `emit_trace_snapshot`
    - `emit_trace_snapshots_due`
    - `enable_trace`
    - `trace_events`
    - `controlled_champion_rune_proc_telemetry`
  - `engine.rs` reduced from `1461` to `1025` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-050` completed by extracting reporting markdown/JSON writer ownership out of `reporting.rs`:
  - added reporting owner modules:
    - `src/reporting/controlled_champion_report_markdown_writer.rs`
    - `src/reporting/controlled_champion_report_json_writer.rs`
  - moved run-report markdown assembly and JSON serialization ownership out of `src/reporting.rs` while keeping `reporting.rs` as a thin helper facade/re-export layer.
  - `reporting.rs` reduced from `1075` to `140` lines (meets facade target `<= 700`).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-013` continued by extracting combat timing/targeting/scheduling and enemy stat-model derivation ownership out of `engine.rs`:
  - added engine owner modules:
    - `src/engine/combat_timing_and_targeting.rs`
    - `src/engine/enemy_combat_stat_modeling.rs`
  - moved controlled champion status/cast/attack gating, enemy range/targeting/projectile-block helpers, attack/event scheduling, and `run_until_end` into `combat_timing_and_targeting.rs`.
  - moved `derive_enemy_model` and `derive_enemy_combat_stats` into `enemy_combat_stat_modeling.rs` with `engine.rs` re-export preserved.
  - fixed post-extraction visibility scope regressions (`pub(super)` to `pub(crate)` where needed by `scenario_runner.rs`) without behavior changes.
  - `engine.rs` reduced from `1025` to `601` lines (meets facade target `<= 700`).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-020`/`ARCH-022` completed by extracting full-loadout search orchestration ownership out of `search.rs`:
  - added search owner module:
    - `src/search/full_loadout_search_orchestration.rs`
  - moved full-loadout orchestration ownership out of `src/search.rs`:
    - `FullLoadoutSearchParams`
    - `build_search_ranked_full_loadout`
    - `strategy_seed_elites_full_loadout`
    - `adaptive_strategy_candidates_full_loadout`
    - `generate_bleed_candidates_full_loadout`
  - rewired `src/search.rs` to re-export the same facade API from the new owner module.
  - `search.rs` reduced from `942` to `569` lines (meets facade target `<= 700`).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-030` completed by extracting controlled champion scenario runtime/search support helpers out of `scenario_runner.rs`:
  - added scenario-runner owner module:
    - `src/scenario_runner/controlled_champion_search_runtime_support.rs`
  - moved coverage-asset locking, partial-candidate completion, progress-state/counter helpers, trace-event shaping, and rune telemetry helper ownership out of `src/scenario_runner.rs`.
  - rewired `src/scenario_runner.rs` to import support helper ownership from the new module for fixed-loadout, rune-sweep, progress-reporting, and controlled-champion scenario flows.
  - `scenario_runner.rs` reduced from `936` to `273` lines (meets facade target `<= 700`).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-051` started by extracting core combat primitives/status/cast-lock ownership out of `core.rs`:
  - added core owner module:
    - `src/core/combat_primitives_state.rs`
  - moved `StatusEffect*`, `CastLock*`, and `CombatPrimitivesState` ownership out of `src/core.rs` while preserving `core.rs` facade re-exports.
  - `core.rs` reduced from `933` to `611` lines (meets facade target `<= 700`).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-051` completed by extracting remaining core utility clusters out of `core.rs`:
  - added core owner modules:
    - `src/core/objective_scoring_math.rs`
    - `src/core/build_candidate_random_helpers.rs`
  - moved objective-score aggregation helpers, deterministic RNG helpers, build-key utilities, and candidate/build repair helpers into explicit core owner leaves while preserving stable `core.rs` facade exports.
  - `core.rs` reduced from `611` to `162` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-060` completed by removing script-tree `mod.rs` carriers and switching to explicit module file facades:
  - migrated:
    - `src/scripts/mod.rs` -> `src/scripts.rs`
    - `src/scripts/champions/mod.rs` -> `src/scripts/champions.rs`
    - `src/scripts/items/mod.rs` -> `src/scripts/items.rs`
    - `src/scripts/registry/mod.rs` -> `src/scripts/registry.rs`
    - `src/scripts/runes/mod.rs` -> `src/scripts/runes.rs`
    - `src/scripts/runtime/mod.rs` -> `src/scripts/runtime.rs`
    - `src/scripts/champions/*/mod.rs` -> explicit champion files (`doctor_mundo.rs`, `morgana.rs`, `sona.rs`, `vayne.rs`, `vladimir.rs`, `warwick.rs`)
  - updated module-private test-path wiring to keep the test layout contract stable after file moves.
  - `mod.rs` count under `src/` is now `0`.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued `ARCH-030` second-stage decomposition inside controlled-champion runtime/search support ownership:
  - added explicit support owner leaves under:
    - `src/scenario_runner/controlled_champion_search_runtime_support/coverage_locked_asset_candidate_generation.rs`
    - `src/scenario_runner/controlled_champion_search_runtime_support/search_seed_derivation.rs`
    - `src/scenario_runner/controlled_champion_search_runtime_support/search_runtime_reporting_projections.rs`
  - rewired `src/scenario_runner/controlled_champion_search_runtime_support.rs` into a thin support facade/re-export surface with owner-channel split by concern (coverage candidate mutation, deterministic seed derivation, read-only reporting projections).
  - `src/scenario_runner/controlled_champion_search_runtime_support.rs` reduced from `682` to `165` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued second-stage `ARCH-040` defaults decomposition by extracting champion-simulation metadata/AI/profile loader ownership out of `defaults.rs`:
  - added defaults owner leaf module:
    - `src/defaults/champion_simulation_data_loading.rs`
  - moved champion simulation profile loading, champion slot-binding derivation, ability-execution default loading, champion AI profile normalization, and URF respawn-default loading into the new explicit defaults leaf module while keeping `defaults.rs` facade/caches stable.
  - `defaults.rs` reduced from `679` to `386` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: `ARCH-014` completed by splitting event-dispatch resolution into explicit event-family owner slices:
  - added event-resolution owner modules:
    - `src/engine/event_resolution/combat_event_enemy_auto_attack_resolution.rs`
    - `src/engine/event_resolution/combat_event_controlled_champion_auto_attack_resolution.rs`
    - `src/engine/event_resolution/combat_event_controlled_champion_offensive_ability_hit_resolution.rs`
    - `src/engine/event_resolution/combat_event_champion_script_dispatch_resolution.rs`
  - rewired `src/engine/event_resolution/combat_event_dispatch_resolution.rs` into a thin dispatcher + `step` lifecycle owner, delegating event-family resolution through explicit `resolve_*` owner methods.
  - reduced `src/engine/event_resolution/combat_event_dispatch_resolution.rs` from `723` to `123` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued scenario execution leaf decomposition by extracting controlled-champion candidate-search orchestration out of `controlled_champion_scenario_runner.rs`:
  - added scenario-runner owner module:
    - `src/scenario_runner/controlled_champion_candidate_search.rs`
  - moved maximum-quality coverage stage, ensemble-seed strategy orchestration, candidate merge/dedupe, and strict full-ranking loops into explicit scenario-runner search-owner helpers.
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` to delegate the search-phase orchestration to the new owner module while preserving fallback and reporting behavior.
  - reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `1783` to `1425` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued scenario execution leaf decomposition by extracting controlled-champion result reporting/trace artifact ownership out of `controlled_champion_scenario_runner.rs`:
  - added scenario-runner owner module:
    - `src/scenario_runner/controlled_champion_result_reporting.rs`
  - moved post-search console summary output, diagnostics assembly, build-order summary rendering, trace markdown/json artifact writing, and report markdown/json writing into explicit scenario result-reporting ownership.
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` to delegate post-search reporting through `emit_controlled_champion_result_reporting`.
  - reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `1425` to `855` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued scenario result-reporting decomposition by splitting analysis and artifact writing ownership out of `controlled_champion_result_reporting.rs`:
  - added scenario-runner owner modules:
    - `src/scenario_runner/controlled_champion_result_build_analysis.rs`
    - `src/scenario_runner/controlled_champion_result_artifact_writing.rs`
  - moved ranked-build analysis, diagnostics assembly, and build-order analysis into explicit analysis ownership and moved trace/report artifact writing plus final output emission into explicit artifact-writing ownership.
  - rewired `src/scenario_runner/controlled_champion_result_reporting.rs` into a thinner orchestration module that delegates to explicit analysis and artifact-writing owners.
  - reduced `src/scenario_runner/controlled_champion_result_reporting.rs` from `813` to `489` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-013` continued by extracting event-dispatch/casting/hot-effect owner modules out of `engine.rs`:
  - added event-resolution owner modules:
    - `src/engine/event_resolution/combat_event_dispatch_resolution.rs`
    - `src/engine/event_resolution/controlled_champion_casting_resolution.rs`
    - `src/engine/event_resolution/enemy_script_action_resolution.rs`
  - added simulation-step owner module:
    - `src/engine/simulation_step/hot_effects_step.rs`
  - moved contiguous event-loop/casting/tick lifecycle methods out of `src/engine.rs`:
    - `process_event`
    - `step`
    - `maybe_cast_controlled_champion_abilities_and_defensives`
    - `apply_enemy_script_actions`
    - `apply_hot_effects`
  - rewired engine module carriers:
    - `src/engine/event_resolution.rs`
    - `src/engine/simulation_step.rs`
  - `engine.rs` reduced from `2778` to `1461` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-23: `ARCH-040` continued and completed by extracting schema/helper ownership out of `defaults.rs`:
  - added defaults owner module files:
    - `src/defaults/simulator_defaults_schema_types.rs`
    - `src/defaults/defaults_path_key_and_effect_helpers.rs`
  - moved schema/type ownership and shared defaults path/key/effect helper ownership out of `src/defaults.rs` into explicit modules.
  - rewired `src/defaults.rs` to a thin typed facade and loader-access layer with explicit module imports/re-exports.
  - `defaults.rs` reduced from `1435` to `679` lines (meets facade target `<= 700`).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued second-stage `ARCH-040` defaults leaf decomposition by splitting champion/item simulation-default loader ownership into explicit leaves:
  - added defaults owner leaf modules:
    - `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs`
    - `src/defaults/champion_item_simulation_defaults_loader/item_simulation_defaults_loaders.rs`
    - `src/defaults/champion_item_simulation_defaults_loader/simulation_defaults_extraction_helpers.rs`
  - rewired `src/defaults/champion_item_simulation_defaults_loader.rs` into a thin loader facade/re-export surface.
  - reduced `src/defaults/champion_item_simulation_defaults_loader.rs` from `1065` to `16` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued scenario execution decomposition by extracting setup and enemy-build preparation ownership out of `controlled_champion_scenario_runner.rs`:
  - added scenario-runner setup owner module:
    - `src/scenario_runner/controlled_champion_scenario_setup.rs`
  - moved scenario/controlled-champion/search setup parsing and enemy-build preparation into explicit setup owner APIs:
    - `prepare_controlled_champion_scenario_search_setup`
    - `prepare_controlled_champion_enemy_build_setup`
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` into thinner orchestration over setup/candidate-search/result-reporting owners.
  - reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `855` to `725` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued scenario execution decomposition by extracting strict-ranking fallback finalization ownership out of `controlled_champion_scenario_runner.rs`:
  - added scenario-runner strict-ranking owner module:
    - `src/scenario_runner/controlled_champion_strict_ranking_finalization.rs`
  - moved strict-ranking fallback insertion, tie-break sorting, seed-best-score aggregation, and seed-hit diagnostics into explicit owner APIs:
    - `finalize_controlled_champion_strict_ranking`
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` to delegate strict-ranking finalization to the new owner module.
  - reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `725` to `656` lines (now below the `<=700` target).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Started high-impact `loadout_runtime` decomposition by extracting rune-proc telemetry ownership into an explicit runtime leaf module:
  - added runtime owner module:
    - `src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs`
  - moved rune telemetry trigger-source accounting, proc/attempt/eligibility tracking, and telemetry-entry assembly into explicit owner APIs consumed by `loadout_runtime.rs`.
  - preserved existing runtime API surface by keeping `rune_proc_telemetry(...)` as the stable `loadout_runtime` entrypoint.
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `1639` to `1347` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued high-impact `loadout_runtime` decomposition by extracting combat bonus-resolution ownership into an explicit runtime leaf module:
  - added runtime owner module:
    - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs`
  - moved on-hit and ability bonus-damage resolution, rune-trigger execution, and stack-window progression into explicit owner APIs consumed by `loadout_runtime.rs`.
  - preserved existing runtime API surface by keeping `calculate_on_hit_bonus_damage(...)` and `calculate_ability_bonus_damage(...)` as stable `loadout_runtime` entrypoints that delegate to the new owner module.
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `1347` to `777` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued second-stage `ARCH-040` defaults decomposition by splitting champion simulation-default loading into explicit champion-family leaf modules:
  - added explicit champion defaults owner leaves under:
    - `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders/`
    - `vladimir_simulation_defaults_loader.rs`
    - `warwick_simulation_defaults_loader.rs`
    - `vayne_simulation_defaults_loader.rs`
    - `morgana_simulation_defaults_loader.rs`
    - `sona_simulation_defaults_loader.rs`
    - `doctor_mundo_simulation_defaults_loader.rs`
  - rewired `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs` into a thin re-export facade (`20` lines).
  - reduced largest champion defaults leaf from `687` to `237` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued high-impact `loadout_runtime` decomposition by extracting runtime cooldown/stack reporting ownership:
  - added runtime owner module:
    - `src/scripts/runtime/loadout_runtime/runtime_state_reporting.rs`
  - moved runtime cooldown and stack description projections into explicit owner APIs consumed by `loadout_runtime.rs` (`describe_runtime_cooldowns_impl`, `describe_runtime_stacks_impl`).
  - preserved existing runtime API surface by keeping `describe_runtime_cooldowns(...)` and `describe_runtime_stacks(...)` as stable `loadout_runtime` entrypoints that delegate to the new owner module.
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `777` to `609` lines (now below the `<=700` budget target).
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued second-stage runtime decomposition by fixing the missing `combat_bonus_resolution` projection leaf and then splitting runtime read-only projection ownership:
  - added runtime owner leaves:
    - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/projection_helpers.rs`
    - `src/scripts/runtime/loadout_runtime/runtime_stat_projections.rs`
  - moved read-only runtime projection channels (attack speed, incoming multipliers, movement speed, regeneration) into explicit owner APIs while preserving stable `loadout_runtime` facade entrypoints.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Continued second-stage runtime decomposition by splitting runtime initialization/reset and runtime mutation-effect channels:
  - added runtime owner leaves:
    - `src/scripts/runtime/loadout_runtime/runtime_state_initialization.rs`
    - `src/scripts/runtime/loadout_runtime/runtime_effect_mutations.rs`
    - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/rune_proc_state_mutations.rs`
  - moved loadout-runtime flag/cooldown initialization, transient reset ownership, outgoing-heal/enemy-kill/aftershock mutation channels, and rune-proc state mutation helpers into explicit owner leaves while preserving stable facade entrypoints.
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `609` to `363` lines and reduced `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` from `613` to `357` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Completed high-impact `main.rs` orchestration-contract decomposition by moving shared root contracts into explicit owner leaves:
  - added root contract facade and owner leaves:
    - `src/simulation_contracts.rs`
    - `src/simulation_contracts/runtime_actor_contracts.rs`
    - `src/simulation_contracts/search_reporting_contracts.rs`
    - `src/simulation_contracts/entrypoint_cli_contracts.rs`
  - moved runtime/search/reporting contract types plus CLI/options contracts out of `src/main.rs` into explicit owner leaves while preserving root-level compatibility exports.
  - reduced `src/main.rs` from `679` to `149` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Completed second-stage `ARCH-041` champion/item/preset data decomposition by splitting `champion_item_preset_data_loading` ownership into explicit data leaves:
  - added explicit data owner leaves under:
    - `src/data/champion_item_preset_data_loading/champion_base_loading.rs`
    - `src/data/champion_item_preset_data_loading/item_pool_loading.rs`
    - `src/data/champion_item_preset_data_loading/urf_mode_loading.rs`
    - `src/data/champion_item_preset_data_loading/enemy_preset_loading.rs`
  - rewired `src/data/champion_item_preset_data_loading.rs` into a thin facade/re-export surface while preserving `data.rs` facade exports and call-site contracts.
  - reduced `src/data/champion_item_preset_data_loading.rs` from `620` to `17` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Completed second-stage `ARCH-041` simulation/search config parsing decomposition by splitting `simulation_search_configuration_parsing` ownership into explicit parse leaves:
  - added explicit parse owner leaves under:
    - `src/data/simulation_search_configuration_parsing/shared_parsing_primitives.rs`
    - `src/data/simulation_search_configuration_parsing/simulation_config_parsing.rs`
    - `src/data/simulation_search_configuration_parsing/enemy_config_parsing.rs`
    - `src/data/simulation_search_configuration_parsing/build_search_config_parsing.rs`
    - `src/data/simulation_search_configuration_parsing/loadout_selection_parsing.rs`
  - rewired `src/data/simulation_search_configuration_parsing.rs` into a thin facade/re-export surface while preserving `data.rs` facade exports and call-site contracts.
  - reduced `src/data/simulation_search_configuration_parsing.rs` from `599` to `15` lines.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Completed second-stage scenario/reporting ownership decomposition and finalized compatibility-shim cleanup:
  - split controlled champion execution ownership into:
    - `src/scenario_runner/controlled_champion_scenario_runner.rs` (facade)
    - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs` (execution owner leaf)
  - split reporting loadout/build section ownership into:
    - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/build_ranking_sections.rs`
    - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/enemy_profile_sections.rs`
    - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/loadout_profile_sections.rs`
  - removed root compatibility shims in `src/main.rs` (`crate::Ordering`, `crate::EnemyDerivedCombatStats`) and updated downstream modules to explicit owner imports.
  - full validation remained green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
- 2026-02-24: Marked `ARCH-013` DONE after actor-state boundary audit confirmed no remaining facade-side mutable enemy-state write channels outside owner modules.
- 2026-02-24: Marked `ARCH-070` DONE after compatibility-shim removal and doc reconciliation across roadmap/tracker/readmes.
- 2026-02-24: Completed high-friction scenario-runner ownership follow-up by decomposing controlled-champion result analysis into explicit projection leaves under:
  - `src/scenario_runner/controlled_champion_result_build_analysis/build_order_analysis.rs`
  - `src/scenario_runner/controlled_champion_result_build_analysis/candidate_metrics_projection.rs`
  - `src/scenario_runner/controlled_champion_result_build_analysis/search_diagnostics_projection.rs`
  while preserving `analyze_controlled_champion_build_results(...)` as a stable facade API in `controlled_champion_result_build_analysis.rs` and reducing that facade from `410` to `288` lines.
- 2026-02-24: Completed high-friction scenario execution ownership follow-up by decomposing controlled-champion execution internals under:
  - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/deadline_and_progress.rs`
  - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/runtime_setup.rs`
  - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/search_execution/candidate_scoring_channels.rs`
  while preserving `run_controlled_champion_scenario_impl(...)` as a stable facade API in `controlled_champion_scenario_execution.rs`, reducing:
  - `controlled_champion_scenario_execution.rs` from `406` to `289` lines
  - `controlled_champion_scenario_execution/search_execution.rs` from `353` to `264` lines.
- 2026-02-24: Full validation rerun remained green after scenario-runner follow-up slices:
  - `cargo fmt --manifest-path Simulation/Cargo.toml`
  - `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
  - `cargo test --release --manifest-path Simulation/Cargo.toml`
- 2026-02-24: Completed high-friction dense-leaf decomposition follow-up outside scenario-runner by splitting:
  - `src/search/strategy/item_candidate_search_strategies.rs` into:
    - `src/search/strategy/item_candidate_search_strategies/beam_search_strategy.rs`
    - `src/search/strategy/item_candidate_search_strategies/iterative_search_strategies.rs`
    - `src/search/strategy/item_candidate_search_strategies/mcts_search_strategy.rs`
  - `src/scripts/champions.rs` into:
    - `src/scripts/champions/champion_behavior_profile_channels.rs`
    - `src/scripts/champions/champion_script_effect_types.rs`
    - `src/scripts/champions/champion_script_event_channels.rs`
    - `src/scripts/champions/runtime_effect_channels.rs`
  - `src/engine/trace_snapshot_reporting.rs` into:
    - `src/engine/trace_snapshot_reporting/trace_lifecycle_channels.rs`
    - `src/engine/trace_snapshot_reporting/trace_status_and_projectile_projections.rs`
    - `src/engine/trace_snapshot_reporting/trace_snapshot_summary_projection.rs`
  - `src/data/loadout_domain_modeling.rs` into:
    - `src/data/loadout_domain_modeling/loadout_domain_schema.rs`
    - `src/data/loadout_domain_modeling/modeled_rune_filtering.rs`
    - `src/data/loadout_domain_modeling/rune_page_validation.rs`
    - `src/data/loadout_domain_modeling/loadout_selection_generation.rs`
  - `src/engine/actor_state/enemy_runtime_state.rs` into:
    - `src/engine/actor_state/enemy_runtime_state/enemy_trace_snapshot_projections.rs`
    - `src/engine/actor_state/enemy_runtime_state/enemy_attack_and_script_channels.rs`
    - `src/engine/actor_state/enemy_runtime_state/enemy_lifecycle_channels.rs`
  while preserving all existing facade entrypoints and reducing:
  - `src/search/strategy/item_candidate_search_strategies.rs` from `458` to `110` lines
  - `src/scripts/champions.rs` from `457` to `36` lines
  - `src/engine/trace_snapshot_reporting.rs` from `440` to `3` lines
  - `src/data/loadout_domain_modeling.rs` from `418` to `15` lines
  - `src/engine/actor_state/enemy_runtime_state.rs` from `416` to `6` lines
- 2026-02-24: Full validation rerun remained green after dense-leaf decomposition follow-up:
  - `cargo fmt --manifest-path Simulation/Cargo.toml`
  - `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
  - `cargo test --release --manifest-path Simulation/Cargo.toml`

## 12) Immediate Next Batch

1. Keep architecture milestones closed by enforcing standards in new feature work (no new oversized facades and no new cross-owner mutation channels).
2. Continue periodic architecture metrics + validation checks and serialize validation runs when concurrent data-file edits are active.
3. Target moderate-density follow-up leaves only when they block feature velocity:
  - `src/core/combat_primitives_state.rs` (`330` lines)
  - `src/engine/event_resolution/controlled_champion_casting_resolution.rs` (`328` lines)
  - `src/engine/combat_timing_and_targeting.rs` (`323` lines)
  - `src/scenario_runner/controlled_champion_search_runtime_support/coverage_locked_asset_candidate_generation.rs` (`322` lines)
  - `src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs` (`320` lines)

## 13) Observed High-Value Improvements

These were identified while executing `ARCH-010`, `ARCH-011`, `ARCH-012`, `ARCH-013`, `ARCH-020`, `ARCH-021`, `ARCH-022`, `ARCH-030`, `ARCH-031`, `ARCH-040`, `ARCH-041`, `ARCH-050`, and `ARCH-051` and should be tracked as part of subsequent phases:

1. Done: added focused geometry unit tests under `src/engine/geometry/tests/` to reduce reliance on broad engine integration tests for low-level math correctness.
2. Done: extracted projectile travel helper and enemy spawn positioning into explicit geometry/kinematics modules.
3. Done: extracted movement vector update helpers into `src/engine/geometry/enemy_orbit_position_updates.rs` and added deterministic orbit regression coverage.
4. Done: introduced explicit owner APIs for event queue scheduling/retrieval/recurring requeue in `src/engine/event_queue/event_queue_scheduler.rs`.
5. Done: moved queue projections/next-event lookups behind queue owner query APIs and removed direct queue scanning from `engine.rs` snapshot helpers.
6. Done: extracted incoming-damage and related mutation methods into `src/engine/event_resolution/incoming_damage_resolution.rs` as the initial damage owner module.
7. Done: introduced explicit owner-command naming and contracts for damage/heal/revive APIs to complete `ARCH-012` naming/channel hardening.
8. Done: extracted the actor-position owner loop into `src/engine/simulation_step/enemy_movement_step.rs` and rewired facade tick flow to owner command usage.
9. Done: completed first `ARCH-013` slice by moving enemy respawn/regeneration lifecycle mutation and alive/active queries into `src/engine/actor_state/enemy_runtime_state.rs` owner commands.
10. Done: reduced search-internal ownership leakage by routing scenario orchestration through `search.rs` facade exports while moving candidate/strategy/scoring internals into owner modules.
11. Done: completed actor-state extraction for remaining script-runtime mutation channels under `src/engine/actor_state/*`.
12. Done: replaced direct `enemy_state` recurring-script eligibility checks in event-loop scheduling with actor-state owner query APIs.
13. Done: moved enemy next-attack bonus consume/reset and attack-sequence token lifecycle writes behind actor-state owner commands.
14. Done: removed remaining mutable script-runtime enemy-state facade borrow sites from `engine.rs` by routing script action/execution mutation through actor-state owner commands.
15. Done: added focused regression coverage for new script-runtime owner channels (`execute_enemy_script_event_actions`, `enemy_aftershock_magic_damage_on_immobilize`).
16. Done: moved high-traffic enemy actor read projections behind actor-state owner query APIs and removed direct `enemy_state[idx]`/`enemy_state.get(...)` access from `engine.rs`.
17. Done: extracted trace-snapshot enemy-section iteration/read composition behind actor-state owner projection APIs (`enemy_count`, `enemy_trace_snapshot_at`) and removed remaining runtime `enemy_state` direct usage in `engine.rs`.
18. Done: started `ARCH-020` by extracting full-loadout candidate operation helpers into `src/search/candidate_space/full_loadout_candidate_operations.rs`.
19. Done: continued `ARCH-020` by extracting search full-loadout scoring/ranking and item-only candidate mutation/crossover/parent-selection helper clusters into candidate-space owner modules.
20. Done: continued `ARCH-020` by extracting item-only candidate scoring/dedupe helpers into `src/search/candidate_space/item_candidate_scoring.rs`.
21. Done: started `ARCH-021` by extracting item-only strategy implementations into `src/search/strategy/item_candidate_search_strategies.rs`.
22. Done: added focused strategy-module regression tests in `src/search/strategy/tests/item_candidate_search_strategies_tests.rs`.
23. Done: completed `ARCH-021` by extracting full-loadout strategy implementations into `src/search/strategy/full_loadout_search_strategies.rs`.
24. Done: added focused strategy-module regression tests in `src/search/strategy/tests/full_loadout_search_strategies_tests.rs`.
25. Done: started `ARCH-022` by extracting scoring/diversity owner modules into `src/search/scoring/*`.
26. Done: added focused scoring-module regression tests in `src/search/scoring/tests/*`.
27. Done: continued `ARCH-022` by extracting `choose_best_build_by_stat` into `src/search/scoring/stat_key_build_selection.rs`.
28. Done: added focused regression tests in `src/search/scoring/tests/stat_key_build_selection_tests.rs`.
29. Done: continued `ARCH-022` by extracting `item_names` into `src/search/scoring/item_name_list_formatting.rs`.
30. Done: added focused regression tests in `src/search/scoring/tests/item_name_list_formatting_tests.rs`.
31. Done: started `ARCH-030` by moving scenario parse/legacy-key validation helpers into `src/scenario_runner/scenario_parsing.rs` and wiring facade call sites to parser-owner APIs.
32. Done: continued `ARCH-030` by extracting encounter-focused parsing into `src/scenario_runner/encounter_parsing.rs` and introducing typed parse outputs (`ParsedOpponentEncounter`).
33. Done: continued `ARCH-030` by extracting run-output path/key ownership into `src/scenario_runner/run_output_paths.rs` and wiring facade call sites to output-owner APIs.
34. Done: continued `ARCH-030` by extracting search-progress/runtime-counter ownership into `src/scenario_runner/progress_reporting.rs` and wiring facade call sites to progress-owner APIs.
35. Done: continued `ARCH-030` by extracting strict-ranking ordering and search-space estimation helpers into `src/scenario_runner/strict_ranking_ordering.rs` and `src/scenario_runner/search_space_estimation.rs`.
36. Done: started `ARCH-031` by extracting fixed-loadout and rune-sweep execution entrypoints into `src/scenario_runner/fixed_loadout_runner.rs` and `src/scenario_runner/rune_sweep_runner.rs`.
37. Done: started `ARCH-040` by extracting champion/item simulation-default loader ownership into `src/defaults/champion_item_simulation_defaults_loader.rs` and reducing `defaults.rs` to `1435` lines.
38. Done: completed `ARCH-041` by extracting explicit data concern modules (`src/data/champion_item_preset_data_loading.rs`, `src/data/loadout_effect_resolution.rs`, `src/data/simulation_search_configuration_parsing.rs`, `src/data/loadout_domain_modeling.rs`) and reducing `data.rs` to `116` lines.
39. Done: continued `ARCH-013` by extracting event-loop dispatch/casting/hot-effect lifecycle methods into explicit owner modules (`src/engine/event_resolution/*`, `src/engine/simulation_step/hot_effects_step.rs`) and reducing `src/engine.rs` to `1461` lines.
40. Done: completed `ARCH-040` by extracting defaults schema/helper ownership into explicit modules (`src/defaults/simulator_defaults_schema_types.rs`, `src/defaults/defaults_path_key_and_effect_helpers.rs`) and reducing `src/defaults.rs` to `679` lines.
41. Done: continued `ARCH-013` by extracting trace/snapshot runtime reporting ownership into `src/engine/trace_snapshot_reporting.rs` and reducing `src/engine.rs` to `1025` lines.
42. Done: completed `ARCH-050` by extracting reporting writer ownership into `src/reporting/controlled_champion_report_markdown_writer.rs` and `src/reporting/controlled_champion_report_json_writer.rs`, reducing `src/reporting.rs` to `140` lines.
43. Done: continued `ARCH-013` by extracting combat timing/targeting/scheduling owner methods into `src/engine/combat_timing_and_targeting.rs` and enemy stat-model derivation into `src/engine/enemy_combat_stat_modeling.rs`, reducing `src/engine.rs` from `1025` to `601` lines.
44. Done: completed `ARCH-020`/`ARCH-022` by extracting full-loadout orchestration ownership into `src/search/full_loadout_search_orchestration.rs` and reducing `src/search.rs` from `942` to `569` lines.
45. Done: completed `ARCH-030` by extracting controlled champion scenario runtime/search support helper ownership into `src/scenario_runner/controlled_champion_search_runtime_support.rs` and reducing `src/scenario_runner.rs` from `936` to `273` lines.
46. Done: started `ARCH-051` by extracting status/cast-lock/combat-primitives ownership into `src/core/combat_primitives_state.rs` and reducing `src/core.rs` from `933` to `611` lines.
47. Done: extracted controlled-champion candidate-search orchestration into `src/scenario_runner/controlled_champion_candidate_search.rs` and reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `1783` to `1425` lines while preserving behavior.
48. Done: completed `ARCH-014` by splitting event dispatch into explicit event-family owner modules and reducing `src/engine/event_resolution/combat_event_dispatch_resolution.rs` from `723` to `123` lines.
49. Done: extracted controlled-champion post-search result reporting/trace artifact ownership into `src/scenario_runner/controlled_champion_result_reporting.rs` and reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `1425` to `855` lines.
50. Done: split scenario result-reporting analysis and artifact-writing ownership into `src/scenario_runner/controlled_champion_result_build_analysis.rs` and `src/scenario_runner/controlled_champion_result_artifact_writing.rs`, reducing `src/scenario_runner/controlled_champion_result_reporting.rs` from `813` to `489` lines.
51. Done: split defaults champion/item simulation-default loader ownership into explicit champion, item, and shared extraction-helper leaves under `src/defaults/champion_item_simulation_defaults_loader/`, reducing `src/defaults/champion_item_simulation_defaults_loader.rs` from `1065` to `16` lines.
52. Done: extracted controlled-champion setup and enemy-build preparation ownership into `src/scenario_runner/controlled_champion_scenario_setup.rs`, reducing `src/scenario_runner/controlled_champion_scenario_runner.rs` from `855` to `725` lines.
53. Done: extracted strict-ranking fallback insertion/tie-break sorting/seed-hit diagnostics ownership into `src/scenario_runner/controlled_champion_strict_ranking_finalization.rs` and reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `725` to `656` lines.
54. Done: started `src/scripts/runtime/loadout_runtime.rs` decomposition by extracting rune-proc telemetry ownership into `src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs` and reducing the parent runtime module from `1639` to `1347` lines.
55. Done: continued `src/scripts/runtime/loadout_runtime.rs` decomposition by extracting combat bonus-resolution ownership into `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` and reducing the parent runtime module from `1347` to `777` lines.
56. Done: continued `src/scripts/runtime/loadout_runtime.rs` decomposition by extracting runtime cooldown/stack reporting ownership into `src/scripts/runtime/loadout_runtime/runtime_state_reporting.rs` and reducing the parent runtime module from `777` to `609` lines (below budget).
57. Done: continued `ARCH-040` defaults decomposition by splitting champion defaults loading into explicit champion-family leaves under `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders/*`, reducing the largest champion defaults leaf from `687` to `237` lines.
58. Done: completed `ARCH-051` by extracting objective-scoring math and build/random helper ownership into `src/core/objective_scoring_math.rs` and `src/core/build_candidate_random_helpers.rs`, reducing `src/core.rs` from `611` to `162` lines.
59. Done: completed `ARCH-060` by migrating script-tree `mod.rs` files to explicit module facades/files (`src/scripts.rs`, `src/scripts/champions.rs`, champion leaf `*.rs`, and sibling script owner files), reducing `mod.rs` count under `src/` from `12` to `0`.
60. Done: continued `ARCH-030` by decomposing `src/scenario_runner/controlled_champion_search_runtime_support.rs` into explicit owner leaves (`coverage_locked_asset_candidate_generation.rs`, `search_seed_derivation.rs`, `search_runtime_reporting_projections.rs`) and reducing the support facade from `682` to `165` lines.
61. Done: continued second-stage `ARCH-040` defaults decomposition by extracting champion-simulation metadata/AI/profile loader ownership into `src/defaults/champion_simulation_data_loading.rs` and reducing `src/defaults.rs` from `679` to `386` lines.
62. Done: continued second-stage runtime decomposition by extracting read-only projection ownership into `src/scripts/runtime/loadout_runtime/runtime_stat_projections.rs` and `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/projection_helpers.rs`.
63. Done: continued second-stage runtime decomposition by extracting runtime initialization/reset ownership into `src/scripts/runtime/loadout_runtime/runtime_state_initialization.rs`.
64. Done: continued second-stage runtime decomposition by extracting runtime mutation-effect ownership into `src/scripts/runtime/loadout_runtime/runtime_effect_mutations.rs`.
65. Done: continued second-stage runtime decomposition by extracting rune-proc state mutation ownership into `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/rune_proc_state_mutations.rs`, reducing `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` from `613` to `357` lines and `src/scripts/runtime/loadout_runtime.rs` from `609` to `363` lines.
66. Done: continued second-stage reporting decomposition by extracting controlled-champion markdown report section ownership under `src/reporting/controlled_champion_report_markdown_writer/`:
    - `header_and_objective_sections.rs`
    - `search_diagnostics_section.rs`
    - `loadout_and_build_sections.rs`
   while preserving `write_controlled_champion_report_markdown(...)` as the stable facade API in `src/reporting/controlled_champion_report_markdown_writer.rs` and reducing that facade from `633` to `122` lines.
67. Done: continued second-stage scenario execution decomposition by splitting controlled-champion fixed-loadout rune-sweep ownership under `src/scenario_runner/rune_sweep_runner/`:
    - `result_aggregation.rs`
    - `report_writing.rs`
   while preserving `run_controlled_champion_fixed_loadout_rune_sweep_impl(...)` as the stable facade entrypoint in `src/scenario_runner/rune_sweep_runner.rs` and reducing that facade from `627` to `308` lines.
68. Done: continued second-stage defaults schema decomposition by splitting simulator/default schema-type ownership under `src/defaults/simulator_defaults_schema_types/`:
    - `simulation_search_and_engine_defaults_schema.rs`
    - `rune_runtime_defaults_schema.rs`
    - `champion_ai_and_execution_schema.rs`
    - `champion_behavior_and_ability_defaults_schema.rs`
    - `champion_file_defaults_schema.rs`
   while preserving `src/defaults/simulator_defaults_schema_types.rs` as the stable facade/re-export surface and reducing that facade from `637` to `22` lines.
69. Done: completed high-impact root contract decomposition by moving shared runtime/search/reporting contracts and CLI/options contracts out of `src/main.rs` into explicit owner leaves under `src/simulation_contracts/`, reducing `src/main.rs` from `679` to `149` lines while preserving root-level compatibility exports.
70. Done: completed second-stage `ARCH-041` data decomposition by splitting champion/item/preset loading + URF legality validation ownership into explicit leaves under `src/data/champion_item_preset_data_loading/*`, reducing `src/data/champion_item_preset_data_loading.rs` from `620` to `17` lines while preserving `data.rs` facade contracts.
71. Done: completed second-stage `ARCH-041` data decomposition by splitting simulation/search config parse ownership into explicit leaves under `src/data/simulation_search_configuration_parsing/*`, reducing `src/data/simulation_search_configuration_parsing.rs` from `599` to `15` lines while preserving `data.rs` facade contracts.
72. Done: completed second-stage scenario/reporting decomposition and `ARCH-070` stabilization by:
   - splitting controlled champion scenario execution ownership into:
     - `src/scenario_runner/controlled_champion_scenario_runner.rs` (thin facade)
     - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs` (execution owner leaf)
   - splitting reporting loadout/build section ownership under:
     - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/build_ranking_sections.rs`
     - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/enemy_profile_sections.rs`
     - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/loadout_profile_sections.rs`
   - removing root compatibility shims from `src/main.rs`:
     - `crate::Ordering`
     - `crate::EnemyDerivedCombatStats`
   - updating downstream imports to explicit owners and keeping full validation green.
73. Done: completed item-pool ownership follow-up by splitting `src/data/champion_item_preset_data_loading/item_pool_loading.rs` into explicit owner leaves:
   - `src/data/champion_item_preset_data_loading/item_pool_loading/item_metadata_loading.rs`
   - `src/data/champion_item_preset_data_loading/item_pool_loading/item_pool_selection_filters.rs`
   while preserving `item_pool_loading.rs` as a thin facade (`8` lines).
74. Done: completed build-search parse ownership follow-up by splitting `src/data/simulation_search_configuration_parsing/build_search_config_parsing.rs` into explicit owner leaves:
   - `src/data/simulation_search_configuration_parsing/build_search_config_parsing/build_search_config_value_mapping.rs`
   - `src/data/simulation_search_configuration_parsing/build_search_config_parsing/search_quality_profile_application.rs`
   while preserving `build_search_config_parsing.rs` as a thin facade (`7` lines).
75. Done: completed defaults schema ownership follow-up by splitting `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema.rs` into explicit owner leaves:
   - `champion_behavior_baseline_defaults_schema.rs`
   - `vladimir_ability_defaults_schema.rs`
   - `champion_specific_ability_defaults_schema.rs`
   - `item_survivability_defaults_schema.rs`
   while preserving `champion_behavior_and_ability_defaults_schema.rs` as a thin facade (`9` lines).
76. Done: completed fixed-loadout/rune-sweep shared projection extraction by introducing:
   - `src/scenario_runner/controlled_champion_enemy_scenario_projection.rs`
   and routing fixed/rune execution through explicit owner APIs (`parse_scaled_enemy_scenarios`, `build_enemy_build_projection`, `build_scenario_reference_outcomes`), reducing:
   - `src/scenario_runner/fixed_loadout_runner.rs` from `406` to `358` lines
   - `src/scenario_runner/rune_sweep_runner.rs` from `308` to `260` lines.
77. Done: completed scenario-runner high-friction follow-up by extracting result-analysis projection ownership under `src/scenario_runner/controlled_champion_result_build_analysis/*` and reducing `src/scenario_runner/controlled_champion_result_build_analysis.rs` from `410` to `288` lines.
78. Done: completed scenario-execution high-friction follow-up by extracting deadline/progress setup ownership and candidate-scoring channels under `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/*`, reducing:
   - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs` from `406` to `289` lines
   - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/search_execution.rs` from `353` to `264` lines.
79. Done: completed remaining dense-target follow-up by splitting:
   - `src/search/strategy/full_loadout_search_strategies.rs` into:
     - `src/search/strategy/full_loadout_search_strategies/beam_search_strategy.rs`
     - `src/search/strategy/full_loadout_search_strategies/iterative_search_strategies.rs`
     - `src/search/strategy/full_loadout_search_strategies/mcts_search_strategy.rs`
   - `src/search/full_loadout_search_orchestration.rs` into:
     - `src/search/full_loadout_search_orchestration/strategy_dispatch.rs`
     - `src/search/full_loadout_search_orchestration/seed_elite_generation.rs`
     - `src/search/full_loadout_search_orchestration/adaptive_candidate_generation.rs`
     - `src/search/full_loadout_search_orchestration/bleed_candidate_generation.rs`
   - `src/scripts/runtime/loadout_runtime.rs` into a thin facade with explicit state-schema ownership in `src/scripts/runtime/loadout_runtime/runtime_state_schema.rs`
   - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` into:
     - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/on_hit_bonus_damage_resolution.rs`
     - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/ability_bonus_damage_resolution.rs`
   - `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution.rs` into explicit phase-owner leaves:
     - `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution/seed_candidate_collection.rs`
     - `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution/strict_candidate_scoring.rs`
   - `src/scripts/champions/controlled_champion.rs` into:
     - `src/scripts/champions/controlled_champion/controlled_champion_script_contracts.rs`
     - `src/scripts/champions/controlled_champion/controlled_champion_script_registry.rs`
     - `src/scripts/champions/controlled_champion/controlled_champion_script_channels.rs`
   - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script.rs` into:
     - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script/vladimir_script_model.rs`
     - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script/vladimir_script_capability_channels.rs`
     - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script/vladimir_script_builder.rs`
   while preserving facade entrypoints and reducing:
   - `src/search/strategy/full_loadout_search_strategies.rs` from `409` to `98` lines
   - `src/search/full_loadout_search_orchestration.rs` from `380` to `76` lines
   - `src/scripts/runtime/loadout_runtime.rs` from `363` to `206` lines
   - `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution.rs` from `359` to `213` lines
   - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` from `357` to `66` lines
   - `src/scripts/champions/controlled_champion.rs` from `351` to `23` lines
   - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script.rs` from `350` to `5` lines
   and keeping validation green (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).

## 14) Update Rules

When milestone statuses change:

- update this file first
- mirror major movement in:
  - `IMPLEMENTATION_ROADMAP.md`
  - `IMPROVEMENT_TRACKER.md`
  - `README.md`
  - `Simulation/README.md`

## 15) Status Indicator And Friction Snapshot (2026-02-24)

Status indicator:

- Milestone completion: `18/18` (`100.0%`) complete, `0/18` in progress.
- Over-budget-gap closure (tracked-facade progress): `100.00%` complete (`0` over-budget lines remain out of `11,678` baseline over-budget lines).
- Raw line-budget helper output: `122.71%` (overshoots because multiple facades are now well below target budgets).
- Overall architecture-program completion estimate (including non-line-budget milestones): `100.0%`.

Remaining facade line-gap to target (`current - 700`):

- `core.rs`: `0` lines (`0.00%` of total remaining gap)
- `search.rs`: `0` lines (`0.00%`)
- `engine.rs`: `0` lines (`0.00%`)
- `scenario_runner.rs`: `0` lines (`0.00%`)
- `reporting.rs`: `0` lines (`0.00%`)
- `data.rs`: `0` lines (`0.00%`)
- `defaults.rs`: `0` lines (`0.00%`)

Current unresolved blockers:

- No hard technical blockers identified in this slice.
- Validation sequencing friction exists when concurrent data-file edits are running in parallel; run-order serialization is required for deterministic green test runs.

Current high-friction areas:

- Previous dense-leaf friction set is resolved:
  - `src/search/strategy/full_loadout_search_strategies.rs`
  - `src/search/full_loadout_search_orchestration.rs`
  - `src/scripts/runtime/loadout_runtime.rs`
  - `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution.rs`
  - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs`
  - `src/scripts/champions/controlled_champion.rs`
  - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script.rs`
- Remaining architecture friction is moderate and concentrated in non-facade leaves around `320-330` lines (no hard blocker):
  - `src/core/combat_primitives_state.rs` (`330` lines)
  - `src/engine/event_resolution/controlled_champion_casting_resolution.rs` (`328` lines)
  - `src/engine/combat_timing_and_targeting.rs` (`323` lines)
  - `src/scenario_runner/controlled_champion_search_runtime_support/coverage_locked_asset_candidate_generation.rs` (`322` lines)
  - `src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs` (`320` lines)
- Root-level compatibility shims remain removed (`crate::Ordering`, `crate::EnemyDerivedCombatStats`); downstream code imports explicit owner modules directly.
- Workspace concurrency risk remains high due broad parallel data-file edits outside architecture slices; architecture extractions should stay tightly scoped to avoid merge churn.

# Architecture Standards

This document defines architecture and implementation standards for `Simulation/`.
It is the source of truth for module boundaries, naming, ownership channels, and refactor quality gates.

## 1) Core Invariants

- Shared core modules must remain generic and reusable.
- Champion-specific, item-specific, rune-specific, and mastery-specific behavior belongs in script modules under `src/scripts/`.
- Shared interfaces must use role-neutral language (`actor`, `target`, `opponent`, `controlled champion`).
- Controlled champion and enemy behavior should use symmetric abstractions where practical.
- Do not add champion/item/rune special branches to shared core when scripts can express the behavior.

## 2) Explicit Naming Standard

- Prefer explicit, concern-first names over short or generic names.
- Avoid broad names that hide ownership (`events.rs`, `helpers.rs`, `utils.rs`, `misc.rs`).
- File names should include both domain and responsibility.
- Function names should indicate side effects (`apply_*`, `enqueue_*`, `resolve_*`, `load_*`, `validate_*`).
- Examples:
  - `event_queue_scheduler.rs` instead of `events.rs`
  - `candidate_generation.rs` instead of `search_helpers.rs`
  - `incoming_damage_resolution.rs` instead of `damage.rs`

## 3) Folder And Module Layout Standard

- Group files by concern using folders when that improves ownership clarity.
- Keep one concern per folder and one primary concern per file.
- Use thin subsystem facades at root:
  - `engine.rs`
  - `search.rs`
  - `scenario_runner.rs`
  - `defaults.rs`
  - `data.rs`
- Facade modules should:
  - expose public API for the subsystem
  - delegate implementation to leaf modules
  - avoid heavy business logic
- `main.rs` should orchestrate; it should not become an implementation host for engine/search internals.

## 4) Public API Surface Standard

- Public surface should be intentional and minimal (`pub(crate)` by default).
- Leaf modules should not expose types/functions unless consumed by another subsystem.
- Re-export compatibility shims are allowed during migration, but must be tracked with removal milestones.
- Breaking internal API moves require:
  - a compatibility layer in the facade, or
  - same-PR updates to all internal consumers.

## 5) State Ownership And Mutation Channels

- Every mutable state area must have a single owner module.
- Non-owner modules must not mutate owner internals directly.
- Cross-subsystem mutation must occur through owner command functions.
- Read access should flow through owner query functions or immutable snapshots.
- Direct map/list mutation across subsystem boundaries is disallowed.

### Ownership And Mutation Matrix (Current To Target)

| Mutable resource | Current primary owner (2026-02-24) | Target owner module | Allowed mutation channel |
|---|---|---|---|
| Combat event queue | `engine/event_queue/event_queue_scheduler.rs` (integrated by `engine.rs`) | `engine/event_queue/*` | `enqueue_*`, `pop_next_*`, `reschedule_*` owner APIs |
| Combat event dispatch and tick-step progression | `engine/event_resolution/combat_event_dispatch_resolution.rs` plus event-family slices under `engine/event_resolution/combat_event_*_resolution.rs` (integrated by `engine.rs`) | `engine/event_resolution/combat_event_dispatch_resolution.rs` plus event-family slices under `engine/event_resolution/combat_event_*_resolution.rs` | `process_event`, `step`, and event-family `resolve_*` owner APIs |
| Controlled champion cast/defensive orchestration | `engine/event_resolution/controlled_champion_casting_resolution.rs` (integrated by `engine.rs`) | `engine/event_resolution/controlled_champion_casting_resolution.rs` | `maybe_cast_controlled_champion_abilities_and_defensives` owner API |
| Enemy script action impact and followup scheduling | `engine/event_resolution/enemy_script_action_resolution.rs` (integrated by `engine.rs`) | `engine/event_resolution/enemy_script_action_resolution.rs` | `apply_enemy_script_actions` owner API |
| Controlled champion hot-effect tick lifecycle | `engine/simulation_step/hot_effects_step.rs` (integrated by `engine.rs`) | `engine/simulation_step/hot_effects_step.rs` | `apply_hot_effects` owner API |
| Controlled champion runtime combat gating windows, status checks, and attack/event scheduling helpers | `engine/combat_timing_and_targeting.rs` (integrated by `engine.rs`) | `engine/combat_timing_and_targeting.rs` | `is_targetable`, `can_cast`, `can_basic_attack`, `begin_cast_lock_window`, `schedule_event`, `schedule_next_*`, `run_until_end` owner APIs |
| Enemy runtime state collection | `engine/actor_state/enemy_runtime_state.rs` and `engine.rs` (partial migration) | `engine/actor_state/enemy_runtime_state.rs` | `spawn_*`, `update_*`, `mark_*` owner APIs |
| Enemy derived combat-stat modeling and runtime loadout-profile derivation | `engine/enemy_combat_stat_modeling.rs` (integrated by `engine.rs`) | `engine/enemy_combat_stat_modeling.rs` | `derive_enemy_model`, `derive_enemy_combat_stats` owner APIs |
| Enemy runtime movement position updates | `engine/simulation_step/enemy_movement_step.rs` (integrated by `engine.rs`) | `engine/simulation_step/enemy_movement_step.rs` | `apply_enemy_movement_step` owner API |
| Enemy runtime respawn/regeneration lifecycle | `engine/actor_state/enemy_runtime_state.rs` (integrated by `engine.rs`) | `engine/actor_state/enemy_runtime_state.rs` | `apply_enemy_respawn_updates`, `apply_enemy_regeneration_tick` |
| Enemy script lifecycle, cadence readiness, and script-runtime mutation | `engine/actor_state/enemy_runtime_state.rs` and `engine.rs` (partial migration) | `engine/actor_state/enemy_runtime_state.rs` | `enemy_script_event_should_recur`, `enemy_script_epoch_matches`, `enemy_script_event_ready_at_or_zero`, `set_enemy_script_event_ready_at`, `enemy_ability_haste_or_urf_default`, `execute_enemy_script_event_actions`, `enemy_aftershock_magic_damage_on_immobilize` |
| Enemy auto-attack token lifecycle and next-hit bonus consume/reset | `engine/actor_state/enemy_runtime_state.rs` and `engine.rs` (partial migration) | `engine/actor_state/enemy_runtime_state.rs` | `begin_enemy_attack_sequence`, `enemy_attack_sequence_matches`, `consume_enemy_attack_damage_with_on_hit`, `apply_enemy_next_attack_bonus_physical` |
| Enemy runtime read projections and trace-snapshot composition | `engine/actor_state/enemy_runtime_state.rs` and `engine.rs` (partial migration) | `engine/actor_state/enemy_runtime_state.rs` | `enemy_name`, `enemy_position`, `enemy_hitbox_radius`, `enemy_attack_range`, `enemy_attack_windup_seconds`, `enemy_attack_projectile_speed`, `enemy_attack_effect_hitbox_radius`, `enemy_target_health_snapshot_or_defaults`, `enemy_attack_interval_seconds`, `enemy_status_lines_at`, `enemy_is_stunned_at`, `enemy_is_invulnerable_or_untargetable_at`, `enemy_count`, `enemy_trace_snapshot_at` |
| Runtime damage/healing application | `engine/event_resolution/incoming_damage_resolution.rs` (integrated by `engine.rs`) | `engine/event_resolution/incoming_damage_resolution.rs` | `apply_incoming_damage_*`, `apply_healing_*`, `apply_revive_or_mark_*` |
| Runtime loadout stack/cooldown mutation, combat bonus resolution, and rune-proc telemetry accumulation | `scripts/runtime/loadout_runtime.rs`, `scripts/runtime/loadout_runtime/combat_bonus_resolution.rs`, `scripts/runtime/loadout_runtime/rune_proc_telemetry.rs`, and `engine.rs` call sites | `scripts/runtime/loadout_runtime.rs` + runtime owner leaves (`combat_bonus_resolution.rs`, `rune_proc_telemetry.rs`), plus engine resolution boundary adapters | Script/runtime owner APIs only (`calculate_*`, `on_*`, `trigger_*`, `loadout_*`, `rune_proc_telemetry`) |
| Search candidate mutation/dedupe/ranking primitives | `search/candidate_space/full_loadout_candidate_operations.rs`, `search/candidate_space/full_loadout_candidate_scoring.rs`, `search/candidate_space/item_candidate_operations.rs`, `search/candidate_space/item_candidate_scoring.rs`, `search.rs`, and `scenario_runner.rs` (partial migration) | `search/candidate_space/*` | `generate_*`, `repair_*`, `mutate_*`, `canonicalize_*`, `score_*`, `rank_*`, `candidate_order_key`, `candidate_loadout_variants`, `tournament_parent`, `crossover_builds`, `mutate_build`, `score_candidates`, `unique_ranked_from_candidates` |
| Full-loadout search orchestration (strategy dispatch, seed-ensemble aggregation, adaptive/bleed generation) | `search/full_loadout_search_orchestration.rs` (integrated by `search.rs`) | `search/full_loadout_search_orchestration.rs` | `build_search_ranked_full_loadout`, `strategy_seed_elites_full_loadout`, `adaptive_strategy_candidates_full_loadout`, `generate_bleed_candidates_full_loadout` |
| Search strategy execution | `search/strategy/item_candidate_search_strategies.rs`, `search/strategy/full_loadout_search_strategies.rs`, and `search.rs` facade dispatch | `search/strategy/*` | `*_search_ranked`, strategy rollout/selection owner APIs |
| Search metrics and diversity scoring | `search/scoring/item_build_scoring_and_diversity.rs`, `search/scoring/full_loadout_scoring_and_diversity.rs`, `search/scoring/stat_key_build_selection.rs`, `search/scoring/item_name_list_formatting.rs`, and `search.rs` facade dispatch | `search/scoring/*` | `select_diverse_*`, `compute_build_metrics*`, `*_pareto_front_keys`, `choose_best_build_by_stat`, `format_item_name_list_comma_separated` |
| Search score cache entries | `cache.rs` via `scenario_runner.rs` | `cache.rs` | `get_or_compute`, `record_*`, cache owner APIs |
| Search runtime counters and unique-loadout registry | `scenario_runner/progress_reporting.rs` and `scenario_runner.rs` (partial migration) | `scenario_runner/progress_reporting.rs` and `search/diagnostics/*` | Counter/registry owner APIs only |
| Scenario parsing normalization state | `scenario_runner/scenario_parsing.rs`, `scenario_runner/encounter_parsing.rs`, `scenario_runner.rs`, and `data.rs` (partial migration) | `scenario_runner/scenario_parsing.rs` + `scenario_runner/encounter_parsing.rs` | parse/validate/normalize owner APIs |
| Controlled champion scenario runtime/search support helpers (coverage-asset locking, partial-candidate completion, progress/trace telemetry helpers) | `scenario_runner/controlled_champion_search_runtime_support.rs` (integrated by `scenario_runner.rs`) | `scenario_runner/controlled_champion_search_runtime_support.rs` | `coverage_locked_assets`, `random_locked_candidate`, `mutate_locked_candidate`, `partial_candidate_completion_seed`, `complete_partial_candidate_to_full`, `structured_trace_event` owner APIs |
| Controlled champion candidate-search phase orchestration (maximum-quality coverage stage, ensemble-seed strategy orchestration, candidate merge/dedupe, strict full ranking) | `scenario_runner/controlled_champion_candidate_search.rs` (integrated by `scenario_runner/controlled_champion_scenario_runner.rs`) | `scenario_runner/controlled_champion_candidate_search.rs` | `run_maximum_quality_coverage_stage`, `run_seed_and_strict_ranking` owner APIs |
| Scenario fixed-loadout and rune-sweep execution entrypoints | `scenario_runner/fixed_loadout_runner.rs` and `scenario_runner/rune_sweep_runner.rs` (integrated by `scenario_runner.rs`) | `scenario_runner/fixed_loadout_runner.rs` + `scenario_runner/rune_sweep_runner.rs` | fixed-loadout execution owner APIs |
| Scenario controlled-champion execution entrypoint | `scenario_runner/controlled_champion_scenario_runner.rs` (integrated by `scenario_runner.rs`) | `scenario_runner/controlled_champion_scenario_runner.rs` | `run_controlled_champion_scenario_impl` owner API |
| Controlled champion scenario setup and enemy-build preparation | `scenario_runner/controlled_champion_scenario_setup.rs` (integrated by `scenario_runner/controlled_champion_scenario_runner.rs`) | `scenario_runner/controlled_champion_scenario_setup.rs` | `prepare_controlled_champion_scenario_search_setup`, `prepare_controlled_champion_enemy_build_setup` owner APIs |
| Controlled champion strict-ranking fallback finalization, tie-break sorting, and seed-hit diagnostics | `scenario_runner/controlled_champion_strict_ranking_finalization.rs` (integrated by `scenario_runner/controlled_champion_scenario_runner.rs`) | `scenario_runner/controlled_champion_strict_ranking_finalization.rs` | `finalize_controlled_champion_strict_ranking` owner API |
| Controlled champion result-reporting orchestration | `scenario_runner/controlled_champion_result_reporting.rs` (integrated by `scenario_runner/controlled_champion_scenario_runner.rs`) | `scenario_runner/controlled_champion_result_reporting.rs` | `emit_controlled_champion_result_reporting` owner API |
| Controlled champion ranked-build analysis, diagnostics assembly, and build-order analysis | `scenario_runner/controlled_champion_result_build_analysis.rs` (integrated by `scenario_runner/controlled_champion_result_reporting.rs`) | `scenario_runner/controlled_champion_result_build_analysis.rs` | `analyze_controlled_champion_build_results` owner API |
| Controlled champion trace/report artifact writing and final output emission | `scenario_runner/controlled_champion_result_artifact_writing.rs` (integrated by `scenario_runner/controlled_champion_result_reporting.rs`) | `scenario_runner/controlled_champion_result_artifact_writing.rs` | `write_controlled_champion_result_artifacts` owner API |
| Scenario run-output path/key composition | `scenario_runner/run_output_paths.rs` (integrated by `scenario_runner.rs`) | `scenario_runner/run_output_paths.rs` | `default_*_output_directory`, `format_repo_relative_path`, `search_quality_profile_key` owner APIs |
| Scenario strict-ranking heuristic ordering | `scenario_runner/strict_ranking_ordering.rs` (integrated by `scenario_runner.rs`) | `scenario_runner/strict_ranking_ordering.rs` | `heuristic_sort_remaining_candidates_for_strict_ranking` owner API |
| Scenario candidate-space estimation/probability formatting | `scenario_runner/search_space_estimation.rs` (integrated by `scenario_runner.rs`) | `scenario_runner/search_space_estimation.rs` | `estimated_legal_*`, `estimate_close_to_optimal_probability`, `format_percent_display` owner APIs |
| Defaults champion/item simulation-default loader facade/re-export surface | `defaults/champion_item_simulation_defaults_loader.rs` (integrated by `defaults.rs`) | `defaults/champion_item_simulation_defaults_loader.rs` | defaults loader facade owner APIs (`load_*_defaults` re-export surface) |
| Champion simulation-default loading from champion canonical data | `defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs` (integrated by `defaults/champion_item_simulation_defaults_loader.rs`) | `defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs` | champion simulation-default loader owner APIs (`load_*_defaults`) |
| Item simulation-default loading from item canonical data | `defaults/champion_item_simulation_defaults_loader/item_simulation_defaults_loaders.rs` (integrated by `defaults/champion_item_simulation_defaults_loader.rs`) | `defaults/champion_item_simulation_defaults_loader/item_simulation_defaults_loaders.rs` | item simulation-default loader owner APIs (`load_*_defaults`) |
| Shared champion/item defaults extraction helpers | `defaults/champion_item_simulation_defaults_loader/simulation_defaults_extraction_helpers.rs` (integrated by defaults loader leaves) | `defaults/champion_item_simulation_defaults_loader/simulation_defaults_extraction_helpers.rs` | extraction helper APIs (`champion_ability*`, `effect_*`) |
| Simulator/default schema-type ownership | `defaults/simulator_defaults_schema_types.rs` (integrated by `defaults.rs`) | `defaults/simulator_defaults_schema_types.rs` | typed schema structs only |
| Defaults path/key normalization and JSON effect helper loading | `defaults/defaults_path_key_and_effect_helpers.rs` (integrated by `defaults.rs`) | `defaults/defaults_path_key_and_effect_helpers.rs` | `normalize_*`, `read_*`, `item_effects`, `load_*_cooldown_seconds_default` owner APIs |
| Simulation/search/config parse ownership | `data/simulation_search_configuration_parsing.rs` (integrated by `data.rs`) | `data/simulation_search_configuration_parsing.rs` | `parse_*`, `apply_search_quality_profile`, `loadout_selection_key` owner APIs |
| Loadout-domain legality/modeling ownership | `data/loadout_domain_modeling.rs` (integrated by `data.rs`) | `data/loadout_domain_modeling.rs` | `build_loadout_domain`, `filter_loadout_domain_to_modeled_runes`, `validate_*`, `random_loadout_selection` owner APIs |
| Loadout stat/effect resolution ownership | `data/loadout_effect_resolution.rs` (integrated by `data.rs`) | `data/loadout_effect_resolution.rs` | `resolve_loadout`, `apply_structured_effect` owner APIs |
| Champion/item/preset data loading and URF legality validation | `data/champion_item_preset_data_loading.rs` (integrated by `data.rs`) | `data/champion_item_preset_data_loading.rs` | champion/item/preset data loader owner APIs (`load_*`, `default_item_pool`, `validate_enemy_urf_presets`) |
| Reporting markdown run-report rendering ownership | `reporting/controlled_champion_report_markdown_writer.rs` (integrated by `reporting.rs`) | `reporting/controlled_champion_report_markdown_writer.rs` | `write_controlled_champion_report_markdown` owner API |
| Reporting JSON run-report serialization ownership | `reporting/controlled_champion_report_json_writer.rs` (integrated by `reporting.rs`) | `reporting/controlled_champion_report_json_writer.rs` | `write_controlled_champion_report_json`, `report_rune_proc_telemetry_json` owner APIs |
| Core combat primitives/status/cast-lock state | `core/combat_primitives_state.rs` (integrated by `core.rs`) | `core/combat_primitives_state.rs` | `StatusEffectSet::apply/tick`, `CastLockState::begin/tick`, `CombatPrimitivesState::tick` owner APIs |
| Defaults snapshot caches (`OnceLock`) | `defaults.rs` | `defaults/*` domain loaders with facade getters | loader owner APIs only |

Rule: until a channel is migrated, do not introduce additional write sites beyond the current owner.

### Owner API Shape (Required For De-Spaghetti Refactors)

- Owner modules must keep mutable fields private and expose explicit owner commands for writes.
- Prefer explicit command/query naming:
  - commands: `enqueue_*`, `apply_*`, `set_*`, `mark_*`, `clear_*`
  - queries: `current_*`, `is_*`, `next_*`, `snapshot_*`
- Multi-field runtime state changes must be grouped behind one owner command (no scattered call-site write sequences).
- If a mutation crosses subsystem boundaries, wrap input in a typed command struct instead of passing loose scalar argument lists.
- During migration, compatibility shims may delegate to owner commands, but new direct write call sites are disallowed.
- Query projections over owner internals (for example event-queue next-impact lookups) must use owner query APIs rather than direct collection scanning from non-owner modules.
- Controlled champion/enemy health mutation paths must route through event-resolution owner modules rather than ad hoc facade-side field writes.
- Event dispatch loops and per-tick scheduling progression should route through event-resolution owner modules (`engine/event_resolution/combat_event_dispatch_resolution.rs` and event-family `engine/event_resolution/combat_event_*_resolution.rs` slices) rather than facade-local match blocks.
- Controlled champion offensive/defensive cast orchestration should route through event-resolution owner modules (`engine/event_resolution/controlled_champion_casting_resolution.rs`) rather than facade-local cast-decision blocks.
- Enemy script action impact and followup scheduling should route through event-resolution owner modules (`engine/event_resolution/enemy_script_action_resolution.rs`) rather than facade-local action-resolution loops.
- Enemy position update loops should route through simulation-step owner commands (for example `apply_enemy_movement_step`) rather than ad hoc facade loops.
- Controlled champion hot-effect tick progression should route through simulation-step owner modules (`engine/simulation_step/hot_effects_step.rs`) rather than facade-local tick mutation blocks.
- Enemy respawn and regeneration lifecycle writes should route through actor-state owner commands (`apply_enemy_respawn_updates`, `apply_enemy_regeneration_tick`) rather than facade-inline loops.
- Event-loop lifecycle eligibility checks should use actor-state owner queries (for example `enemy_script_event_should_recur`) instead of direct facade-side `enemy_state` field inspection.
- Enemy script readiness updates should use actor-state owner commands (`set_enemy_script_event_ready_at`) rather than direct facade-side map writes.
- Enemy script-event epoch checks and readiness projections should use actor-state owner queries (`enemy_script_epoch_matches`, `enemy_script_event_ready_at_or_zero`) instead of direct facade-side field/map inspection.
- Enemy script-runtime state mutation (script action execution and immobilize-triggered runtime effects) should route through actor-state owner commands (`execute_enemy_script_event_actions`, `enemy_aftershock_magic_damage_on_immobilize`) instead of facade-side mutable borrows.
- Enemy high-traffic runtime read projections should use actor-state owner query APIs (`enemy_name`, `enemy_position`, `enemy_hitbox_radius`, `enemy_attack_*`, `enemy_target_health_snapshot_or_defaults`) rather than direct facade-side field access.
- Enemy runtime status-line synthesis should use actor-state owner query/composition APIs (`enemy_status_lines_at`) rather than facade-inline status-field inspection.
- Enemy trace-snapshot enemy-section composition should use actor-state owner projection APIs (`enemy_count`, `enemy_trace_snapshot_at`) rather than facade-side `enemy_state` iteration/read composition.
- Enemy auto-attack token writes and token-matching checks should use actor-state owner APIs (`begin_enemy_attack_sequence`, `enemy_attack_sequence_matches`) rather than direct facade-side sequence-field mutation.
- Enemy next-hit bonus consume/reset (physical/magic/true) should route through a single actor-state owner command (`consume_enemy_attack_damage_with_on_hit`) instead of scattered facade-side resets.
- Controlled champion status/cast/attack gating, projectile-block maintenance, and attack/event scheduling should route through `engine/combat_timing_and_targeting.rs` rather than accumulating back into `engine.rs`.
- Enemy derived combat-stat projection and runtime loadout-profile modeling should route through `engine/enemy_combat_stat_modeling.rs` rather than facade-local stat-derivation blocks.
- Full-loadout candidate canonicalization/mutation/repair and ordering should route through candidate-space owner APIs (`candidate_order_key`, `candidate_loadout_variants`, `repair_full_candidate`, `mutate_full_candidate`, `crossover_full_candidates`) rather than ad hoc `search.rs` helper clusters.
- Full-loadout strategy dispatch, seed-ensemble aggregation, and adaptive/bleed candidate generation should route through `search/full_loadout_search_orchestration.rs` rather than expanding `search.rs` orchestration blocks.
- Full-loadout candidate scoring/ranking and item-only candidate mutation/crossover/parent-selection/scoring-dedupe should route through candidate-space owner APIs (`score_full_candidates`, `unique_ranked_full_candidates`, `tournament_parent`, `crossover_builds`, `mutate_build`, `score_candidates`, `unique_ranked_from_candidates`) rather than facade-local helper clusters.
- Item-only and full-loadout search strategy algorithm implementations should route through strategy owner modules (`search/strategy/*`) rather than accumulating in facade-local `search.rs` strategy helper clusters.
- Item-build/full-loadout metric projection, pareto-front selection, and diversity filtering should route through scoring owner modules (`search/scoring/*`) rather than facade-local `search.rs` helper clusters.
- Stat-key targeted item-build selection (`choose_best_build_by_stat`) should route through scoring owner modules (`search/scoring/*`) rather than facade-local `search.rs` helper clusters.
- Item-name list formatting (`item_names`) should route through scoring owner modules (`search/scoring/*`) rather than facade-local `search.rs` helper clusters.
- Controlled champion/search-default scenario parse channels should route through scenario parsing owner modules (`scenario_runner/scenario_parsing.rs`) rather than facade-local parser helpers.
- Opponent encounter parsing and legacy-key validation channels should route through encounter parsing owner modules (`scenario_runner/encounter_parsing.rs`) and return explicit typed parse outputs (`ParsedOpponentEncounter`) rather than tuple-based parse results in facade code.
- Fixed-loadout and fixed-loadout-rune-sweep execution entrypoint logic should route through execution owner modules (`scenario_runner/fixed_loadout_runner.rs`, `scenario_runner/rune_sweep_runner.rs`) rather than facade-local execution blocks.
- Controlled champion scenario orchestration entrypoint logic should route through execution owner modules (`scenario_runner/controlled_champion_scenario_runner.rs`) rather than facade-local execution blocks.
- Controlled champion scenario setup/search-configuration parsing and enemy-build preparation should route through scenario setup owner modules (`scenario_runner/controlled_champion_scenario_setup.rs`) rather than accumulating setup/evaluation plumbing in `controlled_champion_scenario_runner.rs`.
- Controlled champion strict-ranking fallback insertion, tie-break sorting, and seed-hit diagnostics should route through scenario owner modules (`scenario_runner/controlled_champion_strict_ranking_finalization.rs`) rather than accumulating strict-ranking finalization logic in `controlled_champion_scenario_runner.rs`.
- Controlled champion result-reporting orchestration should route through scenario owner modules (`scenario_runner/controlled_champion_result_reporting.rs`) rather than accumulating in `controlled_champion_scenario_runner.rs`.
- Controlled champion ranked-build analysis/diagnostics and build-order analysis should route through scenario owner modules (`scenario_runner/controlled_champion_result_build_analysis.rs`) rather than accumulating in `controlled_champion_result_reporting.rs`.
- Controlled champion trace/report artifact writing should route through scenario owner modules (`scenario_runner/controlled_champion_result_artifact_writing.rs`) rather than accumulating in `controlled_champion_result_reporting.rs`.
- Scenario run-output path/key formatting and repository-relative output path rendering should route through run-output owner modules (`scenario_runner/run_output_paths.rs`) rather than facade-local helper clusters.
- Search progress counters and unique-loadout count helpers should route through progress owner modules (`scenario_runner/progress_reporting.rs`) rather than facade-local helper clusters.
- Strict-ranking heuristic ordering should route through strict-ranking owner modules (`scenario_runner/strict_ranking_ordering.rs`) rather than facade-local helper clusters.
- Candidate-space estimation/probability formatting should route through estimation owner modules (`scenario_runner/search_space_estimation.rs`) rather than facade-local helper clusters.
- Controlled champion scenario runtime/search support helpers (coverage-asset locking, partial-candidate completion, progress-telemetry/tracing helpers) should route through `scenario_runner/controlled_champion_search_runtime_support.rs` rather than facade-local helper clusters.
- Controlled champion candidate-search phase orchestration (maximum-quality coverage stage, ensemble-seed strategy orchestration, candidate merge/dedupe, strict full ranking) should route through `scenario_runner/controlled_champion_candidate_search.rs` rather than accumulating back into `controlled_champion_scenario_runner.rs`.
- Rune-proc telemetry accumulation, trigger-source accounting, and runtime telemetry-entry assembly should route through `scripts/runtime/loadout_runtime/rune_proc_telemetry.rs` rather than expanding mixed telemetry logic back into `scripts/runtime/loadout_runtime.rs`.
- Runtime on-hit and ability bonus-damage resolution (including rune-trigger execution and stack-window progression) should route through `scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` rather than expanding dense combat bonus logic back into `scripts/runtime/loadout_runtime.rs`.
- Champion/item simulation-default loading and ability-effect extraction helpers should route through defaults owner modules (`defaults/champion_item_simulation_defaults_loader.rs` plus explicit leaves under `defaults/champion_item_simulation_defaults_loader/*`) rather than facade-local loader clusters.
- Simulator/default schema-type declarations should route through explicit defaults schema modules (`defaults/simulator_defaults_schema_types.rs`) rather than expanding facade struct blocks.
- Defaults path/key normalization and JSON effect helper loading should route through explicit helper modules (`defaults/defaults_path_key_and_effect_helpers.rs`) rather than facade-local helper clusters.
- Simulation/search/config parse ownership should route through explicit data config-parse modules (`data/simulation_search_configuration_parsing.rs`) rather than facade-local parser clusters.
- Loadout-domain legality/modeling ownership should route through explicit data loadout-domain modules (`data/loadout_domain_modeling.rs`) rather than facade-local loadout helper clusters.
- Loadout stat/effect resolution should route through explicit data loadout-resolution modules (`data/loadout_effect_resolution.rs`) rather than facade-local stat/effect loops.
- Champion/item/preset data loading and URF legality validation should route through data owner modules (`data/champion_item_preset_data_loading.rs`) rather than facade-local data-loader clusters.
- Reporting markdown run-report rendering should route through explicit reporting writer modules (`reporting/controlled_champion_report_markdown_writer.rs`) rather than facade-local report assembly clusters.
- Reporting JSON run-report serialization should route through explicit reporting writer modules (`reporting/controlled_champion_report_json_writer.rs`) rather than facade-local JSON assembly clusters.
- Core combat primitives/status/cast-lock state should route through `core/combat_primitives_state.rs` rather than rebuilding mutable status/cast-lock logic inside `core.rs` or other facades.
- Controlled champion health lifecycle writes should route through explicit event-resolution owner commands:
  - `apply_incoming_damage_to_controlled_champion`
  - `apply_healing_to_controlled_champion`
  - `apply_revive_or_mark_controlled_champion_death`

## 6) Dependency Direction

- Keep one-way dependencies:
  - orchestration: `main.rs`, `scenario_runner.rs`
  - domain runtime/services: `engine`, `search`, `build_order`, `cache`, `status`
  - data/default loading: `data`, `defaults`
  - scripts: `scripts/*` specialization modules
- Lower-level modules must not import higher-level orchestration modules.
- Avoid cyclic dependencies; extract shared types/contracts into neutral modules.
- `scenario_runner` may depend on `search` facades, but `search` must not depend on `scenario_runner`.

## 7) Determinism And Randomness Standard

- All non-test randomness must derive from explicit seed flow.
- Use shared deterministic RNG helpers from core paths.
- Do not introduce ambient randomness (`thread_rng` style) in simulation/search paths.
- Reports/diagnostics must include effective seeds for reproducibility.
- Any change that affects deterministic order must include a reproducibility regression test.

### Event Queue Ordering Contract

- Event scheduling order must remain deterministic and centralized in the event-queue owner.
- Ordering contract:
  - earlier `time` executes first
  - for equal time, lower numeric `priority` executes first
  - for equal time and priority, earlier enqueue sequence executes first
- Recurring-event requeue behavior must preserve the same priority and event payload while assigning a fresh enqueue sequence.

## 8) Concurrency And Synchronization Standard

- Synchronization primitives must be owned close to the data they protect.
- Do not share raw lock-protected structures across unrelated modules.
- No nested lock acquisition without documented lock ordering.
- Prefer per-resource sharding or atomics for hot-path counters.
- Parallel scoring behavior must preserve deterministic merge ordering where required.

## 9) Data And Defaults Ownership Boundaries

- Global simulator/search defaults: `Simulation/data/simulator_defaults.json`.
- Champion AI controller policy: `Simulation/data/champion_ai_profiles.json`.
- Mode defaults: `Game Mode/<mode>.json`.
- Champion canonical gameplay data and champion simulation defaults: `Characters/*.json`.
- `defaults.rs` and submodules are loaders and typed accessors, not policy authors.
- Do not scatter fallback gameplay constants through unrelated modules.

## 10) Error Handling And Validation Standard

- Parse/load failures must return actionable error messages including canonical path/key context.
- Validation should fail fast at boundaries (scenario/data/default parsing), not deep in simulation loops.
- Add schema/path names in error text when rejecting legacy or invalid fields.
- Keep normalization and validation responsibilities in parser modules, not runtime execution modules.

## 11) File And Function Size Budgets

These are working budgets to trigger refactors before files become unmaintainable.

- File length:
  - target: `150-400` lines
  - warning: `> 500` lines
  - refactor required: `> 700` lines unless explicitly waived
- Function length:
  - target: `10-35` lines
  - warning: `> 50` lines
  - refactor required: `> 80` lines unless explicitly waived
- Parameter count:
  - target: `<= 6`
  - if more, bundle into typed context/config structs
- Nesting depth:
  - target: `<= 3` levels
  - refactor when deeper
- Line width:
  - follow `rustfmt` defaults; avoid manual formatting drift

## 12) Rust Analogue To "File Per Class"

Rust does not use classes, but we adopt the same clarity goal.

- One primary type/service per file where practical.
- Keep companion helper types in the same file only when tightly bound to that primary type.
- If a file owns unrelated `struct`/`enum`/`impl` groups, split by concern.
- Prefer explicit folder + file decomposition over monolithic multi-type files.

## 13) Test Layout And Refactor Coverage

- Do not place inline test modules in production files.
- Use dedicated test files with explicit names.
- Unit tests for module-private behavior should live in nearby `tests/` folders using `#[cfg(test)]` + `#[path = "..."]`.
- Integration/contract tests should live under crate-level `tests/`.
- Each architecture refactor phase must include:
  - behavior-preserving regression tests for moved logic
  - deterministic seed/replay checks where ordering could change
  - compile-time checks that facades still satisfy current callers

## 14) `mod.rs` Policy

- `mod.rs` is not required in modern Rust.
- Default policy: avoid `mod.rs` and use explicit module file names.
- Preferred pattern:
  - `src/search.rs` as facade
  - `src/search/candidate_generation.rs`, `src/search/scoring_pipeline.rs`, etc. as leaf modules
- `mod.rs` is allowed only when:
  - preserving compatibility during staged refactors, and
  - paired with a tracked follow-up removal milestone.

### Script Tree `mod.rs` Guidelines

- Current script tree contains `mod.rs` carriers under:
  - `src/scripts/mod.rs`
  - `src/scripts/champions/mod.rs`
  - `src/scripts/champions/*/mod.rs`
  - `src/scripts/items/mod.rs`
  - `src/scripts/registry/mod.rs`
  - `src/scripts/runes/mod.rs`
  - `src/scripts/runtime/mod.rs`
- Migration convention:
  - module with child modules: use `<module>.rs` facade plus `<module>/` folder
  - leaf module without children: prefer single `<module>.rs` file
- Do not batch-rename all script `mod.rs` files at once; migrate one subtree per phase with compile-safe shims.

## 15) Deprecation And Compatibility Policy

- Compatibility shims are temporary and must be tracked in `ARCHITECTURE_TRANSFORMATION_PLAN.md`.
- Deprecated internal APIs should include:
  - migration target path
  - milestone ID for removal
- Remove shims only after all internal call sites are migrated and tests pass.

## 16) Refactor Gate Checklist

For any architecture refactor PR:

- Start from `ARCHITECTURE_REFACTOR_CHECKLIST.md` and ensure every applicable item is addressed.
- Identify current owner and target owner for each moved responsibility.
- State allowed mutation channels after the change.
- Confirm no champion/item/rune behavior moved into shared core.
- Confirm dependency direction remains acyclic.
- Confirm facade modules remain thin and primarily declarative.
- Confirm deterministic behavior is preserved (or explicitly documented and tested if changed).
- Run required validation:
  - `cargo fmt --manifest-path Simulation/Cargo.toml`
  - `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
  - `cargo test --release --manifest-path Simulation/Cargo.toml`
- Refresh architecture line-budget metrics:
  - `Simulation/tools/architecture_metrics.sh`
- Update docs:
  - `README.md`
  - `Simulation/README.md`
  - `Simulation/IMPLEMENTATION_ROADMAP.md`
  - `Simulation/IMPROVEMENT_TRACKER.md`

## 17) Implementation Correctness And Quality Review Standard

Every architecture slice must include an explicit correctness/quality review, not only structural movement.

- Run a mutation-site audit before and after extraction:
  - list direct facade writes that touch the migrating owner state
  - verify no new facade write sites were introduced
- Verify lifecycle equivalence for moved logic:
  - same event ordering contract
  - same cancellation/nullification behavior
  - same cooldown/readiness gating behavior
- Require objective validation evidence:
  - `cargo fmt --manifest-path Simulation/Cargo.toml`
  - `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
  - `cargo test --release --manifest-path Simulation/Cargo.toml`
- For each review pass, document:
  - findings (bugs/regressions) or explicit “no findings”
  - residual risk and the next highest-value extraction target
- If any behavior changes are intentional, mark them as non-architecture scope and track them in roadmap/tracker as feature/correctness work.

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
- Recent progress:
  - introduced `src/scripts/champions/controlled_champion.rs` as the engine-facing controlled champion script facade
  - removed legacy Vladimir-named compatibility aliases from shared modules (`compute_vlad_stats`, `simulate_vlad_combat`, `VladCombatSimulation`)
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
- Recent progress:
  - removed legacy `simulation.vlad_*` overrides from shared simulation parser; parser now fails fast on those keys
  - controlled champion script capabilities are now resolved from selected controlled champion identity in scenario orchestration
- Success criteria:
  - default scenario simulation block only needs scenario-owned knobs plus optional explicit overrides.

3.6 `DONE` Add actor-level level overrides and generic stack override ownership.
- Scope:
  - add `controlled_champion.level`, `opponents.default_level`, and `opponents.encounters[].actors[].level`
  - add generic stack override maps: `simulation.stack_overrides`, `controlled_champion.stack_overrides`, `opponents.stack_overrides`, and actor-level overrides
  - add `simulation.time_limit_seconds` parsing and enforce 20-minute hard cap
- Recent progress:
  - global fallback stack assumptions now load from `Simulation/data/simulator_defaults.json` (`simulation_defaults.stack_overrides`) and are overridden by scenario/actor maps.
- Success criteria:
  - scenario can independently set controlled and opponent levels, stack overrides are generic/per-actor, runtime horizon is bounded by validated time limit, and legacy keys are rejected.
  - opponent actors are minimal setup objects and do not carry proxy combat cadence fields.

4. `IN_PROGRESS` Build a generic buff/debuff/status system.
- Scope:
  - duration, stacks, refresh policy
  - persist-through-death flags
  - deterministic ticking
- Recent progress:
  - shared runtime stat-query resolver now covers cooldowns plus scalar combat metrics (incoming damage taken, healing, movement speed, outgoing bonus-ability damage)
  - key combat-time engine/runtime call sites now resolve from base metric data + runtime buff state instead of direct raw values
- Success criteria:
  - combat effects represented as status instances, not ad hoc booleans.

5. `IN_PROGRESS` Build a generic cast system.
- Scope:
  - windup, backswing, channel, cancel rules
  - cast lock and interrupt windows
- Recent progress:
  - controlled champion cast gating now enforces active cast-lock state (windup/channel/lockout), preventing same-tick multi-cast stacking in engine scheduling.
  - Vladimir offensive script decisions now emit one cast per decision tick with script-owned priority (`R` before `Q` before `E`) to align with cast-lock sequencing.
  - offensive-ultimate-before-defensive-ability-two policy is now script-owned and loaded from champion simulation data, not hardcoded in engine.
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
- Recent progress:
  - controlled champion now executes full basic-attack start/windup/hit events, so on-hit runtime effects (for example Lethal Tempo and Grasp paths) can affect outcomes in controlled simulations.
  - controlled champion spell hits now consume shared runtime ability-bonus effects (for example Luden/Liandry-style ability runtime procs) through generic runtime interfaces.
  - report diagnostics now explicitly list controlled champion runes that currently have no modeled deterministic or combat-time runtime effect.
  - shared runtime rune trigger hooks now support:
    - Press the Attack (third-hit proc + vulnerability window)
    - Fleet Footwork (combat-time heal proc)
    - Conqueror (stacking adaptive spell bonus + max-stack damage-heal conversion)
    - Aftershock (immobilize-triggered shockwave damage)
    - Electrocute (3-hit window proc damage)
    - First Strike (windowed bonus true-damage conversion)
    - Phase Rush (3-hit window movement-speed burst)
    - Arcane Comet, Summon Aery, Hail of Blades, Dark Harvest
    - Triumph, Gathering Storm, and Second Wind
  - controlled champion and enemy actors now execute rune combat effects through the same shared loadout runtime API.
  - global rune runtime tuning moved to `Simulation/data/simulator_defaults.json` under `rune_runtime_defaults` and loaded via `src/defaults.rs`.
  - Aftershock resist-window mitigation now applies during the active window for both controlled champion and enemy actors.
  - reports and trace JSON now include rune proc telemetry with per-trigger source attribution (`source_breakdown`).
  - added fixed-loadout rune sweep mode to compare keystones directly on one fixed build/loadout baseline.
  - fixed-loadout rune sweep now evaluates keystones in parallel and supports optional repeated-evaluation aggregation (`--fixed-sweep-seed-repeats`) with distinct deterministic combat seeds per repeat.
  - rune telemetry now includes proc opportunity counters/rates and damage/healing share metrics in markdown/json outputs.
  - added explicit rune level-calibration regression tests for Electrocute, Arcane Comet, First Strike, and Aftershock formulas/caps.
  - report generation now hard-fails if controlled champion rune/shard selection labels are incomplete (no degraded “none selected” output).
  - search scoring now supports explicit unmodeled-rune quality-gate policy (hard gate or per-rune penalty) with diagnostics counters.
  - added explicit unmodeled-item-effect quality-gate policy in search scoring (hard gate or per-item penalty) with diagnostics counters.
  - added script-level coverage registry helpers for modeled item effects and report surfacing of best-build unmodeled item effects.
  - added `Simulation/COVERAGE_GAPS.md` to track cross-domain fidelity/coverage gaps with concrete modeled/unmodeled inventories.
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
- Recent progress:
  - loadout parsing now fails fast on deprecated keys (`loadout.runes_reforged.rune_ids`, `loadout.season2016_masteries`) instead of silently dropping them.
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
- Recent progress:
  - enemy respawn delay now resolves with each enemy actor's level (not global controlled-champion level), preserving mixed-level encounter correctness.
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

19a. `DONE` Architecture modularization and ownership-channel hardening.
- Scope:
  - split oversized core files into explicit concern folders and small leaf modules
  - adopt explicit naming standards for module files (avoid ambiguous names like `events.rs`)
  - reduce `mod.rs` usage through explicit module file naming where practical
  - enforce owner-channel mutation flows for runtime state, queues, caches, and data transforms
  - track rollout via `ARCHITECTURE_TRANSFORMATION_PLAN.md`
- Recent progress:
  - completed remaining dense-target follow-up by splitting:
    - `src/search/strategy/full_loadout_search_strategies.rs` into explicit strategy leaves (`beam_search_strategy.rs`, `iterative_search_strategies.rs`, `mcts_search_strategy.rs`)
    - `src/search/full_loadout_search_orchestration.rs` into explicit orchestration leaves (`strategy_dispatch.rs`, `seed_elite_generation.rs`, `adaptive_candidate_generation.rs`, `bleed_candidate_generation.rs`)
    - `src/scripts/runtime/loadout_runtime.rs` into a thin facade with explicit runtime state-schema ownership (`runtime_state_schema.rs`)
    - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` into explicit owner leaves (`on_hit_bonus_damage_resolution.rs`, `ability_bonus_damage_resolution.rs`)
    - `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution.rs` into explicit phase-owner leaves (`seed_candidate_collection.rs`, `strict_candidate_scoring.rs`)
    - `src/scripts/champions/controlled_champion.rs` into explicit contract/registry/channel owner leaves
    - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script.rs` into explicit model/capability/builder owner leaves
  - reduced:
    - `src/search/strategy/full_loadout_search_strategies.rs` from `409` to `98` lines
    - `src/search/full_loadout_search_orchestration.rs` from `380` to `76` lines
    - `src/scripts/runtime/loadout_runtime.rs` from `363` to `206` lines
    - `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution.rs` from `359` to `213` lines
    - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` from `357` to `66` lines
    - `src/scripts/champions/controlled_champion.rs` from `351` to `23` lines
    - `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script.rs` from `350` to `5` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - completed high-friction scenario result-analysis decomposition by splitting:
    - `src/scenario_runner/controlled_champion_result_build_analysis/build_order_analysis.rs`
    - `src/scenario_runner/controlled_champion_result_build_analysis/candidate_metrics_projection.rs`
    - `src/scenario_runner/controlled_champion_result_build_analysis/search_diagnostics_projection.rs`
    while preserving `analyze_controlled_champion_build_results(...)` as the stable facade entrypoint
  - reduced `src/scenario_runner/controlled_champion_result_build_analysis.rs` from `410` to `288` lines
  - completed high-friction controlled-champion execution decomposition by splitting:
    - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/deadline_and_progress.rs`
    - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/runtime_setup.rs`
    - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/search_execution/candidate_scoring_channels.rs`
    while preserving `run_controlled_champion_scenario_impl(...)` as the stable execution facade entrypoint
  - reduced:
    - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs` from `406` to `289` lines
    - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/search_execution.rs` from `353` to `264` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - observed transient test failures during concurrent data-file edits (Akshan/Riven parse races); rerun after file-write stabilization was fully green
  - completed second-stage `ARCH-041` simulation/search config parsing decomposition by splitting `src/data/simulation_search_configuration_parsing.rs` ownership into explicit parse owner leaves under:
    - `src/data/simulation_search_configuration_parsing/shared_parsing_primitives.rs`
    - `src/data/simulation_search_configuration_parsing/simulation_config_parsing.rs`
    - `src/data/simulation_search_configuration_parsing/enemy_config_parsing.rs`
    - `src/data/simulation_search_configuration_parsing/build_search_config_parsing.rs`
    - `src/data/simulation_search_configuration_parsing/loadout_selection_parsing.rs`
  - rewired `src/data/simulation_search_configuration_parsing.rs` into a thin facade/re-export surface while preserving `data.rs` facade exports and reduced it from `599` to `15` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - completed second-stage `ARCH-041` champion/item/preset data decomposition by splitting `src/data/champion_item_preset_data_loading.rs` ownership into explicit owner leaves under:
    - `src/data/champion_item_preset_data_loading/champion_base_loading.rs`
    - `src/data/champion_item_preset_data_loading/item_pool_loading.rs`
    - `src/data/champion_item_preset_data_loading/urf_mode_loading.rs`
    - `src/data/champion_item_preset_data_loading/enemy_preset_loading.rs`
  - rewired `src/data/champion_item_preset_data_loading.rs` into a thin facade/re-export surface while preserving `data.rs` facade exports and reduced it from `620` to `17` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - completed high-impact `main.rs` decomposition by moving shared runtime/search/reporting contracts and CLI/options contracts into explicit owner leaves under:
    - `src/simulation_contracts/runtime_actor_contracts.rs`
    - `src/simulation_contracts/search_reporting_contracts.rs`
    - `src/simulation_contracts/entrypoint_cli_contracts.rs`
    - `src/simulation_contracts.rs` (facade/re-export surface)
  - preserved root-level compatibility exports while reducing `src/main.rs` from `679` to `149` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - continued second-stage defaults schema decomposition by splitting simulator/default schema-type ownership under:
    - `src/defaults/simulator_defaults_schema_types/simulation_search_and_engine_defaults_schema.rs`
    - `src/defaults/simulator_defaults_schema_types/rune_runtime_defaults_schema.rs`
    - `src/defaults/simulator_defaults_schema_types/champion_ai_and_execution_schema.rs`
    - `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema.rs`
    - `src/defaults/simulator_defaults_schema_types/champion_file_defaults_schema.rs`
  - rewired `src/defaults/simulator_defaults_schema_types.rs` into a thin schema facade/re-export surface while preserving the defaults API and reduced it from `637` to `22` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - continued second-stage scenario execution decomposition by splitting controlled-champion fixed-loadout rune-sweep ownership under:
    - `src/scenario_runner/rune_sweep_runner/result_aggregation.rs`
    - `src/scenario_runner/rune_sweep_runner/report_writing.rs`
  - moved rune-sweep read-only outcome/objective aggregation and markdown/json report projection into explicit owner leaves while preserving `run_controlled_champion_fixed_loadout_rune_sweep_impl(...)` as the stable facade entrypoint
  - reduced `src/scenario_runner/rune_sweep_runner.rs` from `627` to `308` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - continued second-stage reporting decomposition by splitting `src/reporting/controlled_champion_report_markdown_writer.rs` into explicit section-owner leaves under:
    - `src/reporting/controlled_champion_report_markdown_writer/header_and_objective_sections.rs`
    - `src/reporting/controlled_champion_report_markdown_writer/search_diagnostics_section.rs`
    - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections.rs`
  - preserved `write_controlled_champion_report_markdown(...)` as a stable facade entrypoint while delegating section ownership to explicit `append_*` projection helpers
  - reduced `src/reporting/controlled_champion_report_markdown_writer.rs` from `633` to `122` lines and re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - continued second-stage runtime decomposition by extracting explicit owner channels under `src/scripts/runtime/loadout_runtime/`:
    - `runtime_stat_projections.rs`
    - `runtime_state_initialization.rs`
    - `runtime_effect_mutations.rs`
    - `combat_bonus_resolution/projection_helpers.rs`
    - `combat_bonus_resolution/rune_proc_state_mutations.rs`
  - rewired `src/scripts/runtime/loadout_runtime.rs` and `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` into thinner facades with stable entrypoints and delegated owner logic
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `609` to `363` lines and reduced `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` from `613` to `357` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`)
  - completed `ARCH-051` by extracting remaining core utility ownership out of `src/core.rs` into:
    - `src/core/objective_scoring_math.rs`
    - `src/core/build_candidate_random_helpers.rs`
  - moved objective-score aggregation helpers, deterministic RNG helpers, and build-key/build-repair helper ownership into explicit core owner leaves while preserving stable `core.rs` facade exports
  - reduced `src/core.rs` from `611` to `162` lines
  - completed `ARCH-060` by removing script-tree `mod.rs` files and migrating to explicit module facades/files:
    - `src/scripts.rs`
    - `src/scripts/champions.rs`
    - `src/scripts/items.rs`
    - `src/scripts/registry.rs`
    - `src/scripts/runes.rs`
    - `src/scripts/runtime.rs`
    - `src/scripts/champions/*.rs` champion leaves
  - reduced `mod.rs` file count under `src/` from `12` to `0`
  - continued `ARCH-030` second-stage decomposition by splitting controlled-champion support ownership under:
    - `src/scenario_runner/controlled_champion_search_runtime_support/coverage_locked_asset_candidate_generation.rs`
    - `src/scenario_runner/controlled_champion_search_runtime_support/search_seed_derivation.rs`
    - `src/scenario_runner/controlled_champion_search_runtime_support/search_runtime_reporting_projections.rs`
  - rewired `src/scenario_runner/controlled_champion_search_runtime_support.rs` into a thin support facade with owner-channel split by concern and reduced it from `682` to `165` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - continued second-stage defaults decomposition by extracting champion-simulation metadata/AI/profile loader ownership out of `src/defaults.rs` into:
    - `src/defaults/champion_simulation_data_loading.rs`
  - moved champion simulation profile loading, champion slot-binding derivation, ability-execution default loading, champion AI profile normalization, and URF respawn-default loading into the new defaults owner leaf module while preserving stable defaults facade/cache entrypoints
  - reduced `src/defaults.rs` from `679` to `386` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - continued second-stage defaults decomposition by splitting champion simulation-default loading into explicit champion-family leaves under:
    - `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders/`
    - `vladimir_simulation_defaults_loader.rs`
    - `warwick_simulation_defaults_loader.rs`
    - `vayne_simulation_defaults_loader.rs`
    - `morgana_simulation_defaults_loader.rs`
    - `sona_simulation_defaults_loader.rs`
    - `doctor_mundo_simulation_defaults_loader.rs`
  - rewired `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs` into a thin defaults-loader facade/re-export surface and reduced it from `687` to `20` lines (largest champion defaults leaf now `237` lines)
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - continued high-impact `loadout_runtime` decomposition by extracting runtime cooldown/stack reporting ownership out of `src/scripts/runtime/loadout_runtime.rs` into:
    - `src/scripts/runtime/loadout_runtime/runtime_state_reporting.rs`
  - moved runtime cooldown/stack description projection into explicit runtime owner helpers while preserving `describe_runtime_cooldowns(...)` and `describe_runtime_stacks(...)` as stable runtime API entrypoints
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `777` to `609` lines (below the `<=700` target)
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - continued high-impact `loadout_runtime` decomposition by extracting combat bonus-resolution ownership out of `src/scripts/runtime/loadout_runtime.rs` into:
    - `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs`
  - moved on-hit and ability bonus-damage resolution, rune-trigger execution, and stack-window progression into explicit runtime owner helpers while preserving `calculate_on_hit_bonus_damage(...)` and `calculate_ability_bonus_damage(...)` as stable runtime API entrypoints
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `1347` to `777` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - started high-impact `loadout_runtime` decomposition by extracting rune-proc telemetry ownership out of `src/scripts/runtime/loadout_runtime.rs` into:
    - `src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs`
  - moved rune telemetry trigger-source accounting, proc/attempt/eligibility tracking, and telemetry-entry assembly into explicit runtime owner helpers while preserving `rune_proc_telemetry(...)` as the stable runtime API surface
  - reduced `src/scripts/runtime/loadout_runtime.rs` from `1639` to `1347` lines
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - continued scenario execution decomposition by extracting strict-ranking fallback/tie-break/seed-diagnostics ownership out of `src/scenario_runner/controlled_champion_scenario_runner.rs` into:
    - `src/scenario_runner/controlled_champion_strict_ranking_finalization.rs`
  - moved strict-ranking fallback insertion, strict-score tie-break sorting, seed-best-score aggregation, and seed-hit diagnostics into the explicit owner API `finalize_controlled_champion_strict_ranking`
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` to delegate strict-ranking finalization and reduced it from `725` to `656` lines (below the `<=700` target)
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - continued scenario execution decomposition by extracting setup and enemy-build preparation ownership out of `src/scenario_runner/controlled_champion_scenario_runner.rs` into:
    - `src/scenario_runner/controlled_champion_scenario_setup.rs`
  - moved scenario/controlled-champion/search setup parsing and enemy-build preparation into explicit setup owner APIs (`prepare_controlled_champion_scenario_search_setup`, `prepare_controlled_champion_enemy_build_setup`)
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` into thinner orchestration over setup/candidate-search/result-reporting owner modules and reduced it from `855` to `725` lines.
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - continued second-stage defaults decomposition by splitting `src/defaults/champion_item_simulation_defaults_loader.rs` into explicit leaves:
    - `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs`
    - `src/defaults/champion_item_simulation_defaults_loader/item_simulation_defaults_loaders.rs`
    - `src/defaults/champion_item_simulation_defaults_loader/simulation_defaults_extraction_helpers.rs`
  - rewired `src/defaults/champion_item_simulation_defaults_loader.rs` into a thin defaults-loader facade/re-export surface and reduced it from `1065` to `16` lines.
  - re-ran full validation with no findings (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`).
  - added `ARCHITECTURE_STANDARDS.md` and `ARCHITECTURE_TRANSFORMATION_PLAN.md` with baseline metrics, target tree, and milestone tracker
  - expanded standards with explicit ownership/mutation matrix, deterministic/concurrency/error-handling rules, API-surface rules, and script-tree `mod.rs` migration conventions
  - expanded transformation plan with subsystem boundary contracts, sequencing rules, risk/mitigation register, phased acceptance gates, and metrics checkpoints
  - added `ARCHITECTURE_REFACTOR_CHECKLIST.md` and marked architecture checklist milestone complete (`ARCH-002`)
  - started `ARCH-010` by extracting engine geometry into `src/engine/geometry/*` (vector math, segment intersection, hitbox distance, and reach/miss helpers)
  - continued `ARCH-010` by extracting spawn/projectile kinematics into `src/engine/geometry/spawn_positioning.rs` and `src/engine/geometry/projectile_kinematics.rs`
  - added focused geometry module tests in `src/engine/geometry/tests/geometry_module_tests.rs` to validate extracted low-level math/kinematics behavior directly
  - continued `ARCH-010` by extracting enemy orbit movement-vector math into `src/engine/geometry/enemy_orbit_position_updates.rs`
  - extracted explicit script-point conversion helpers into `src/engine/script_point_coordinate_conversions.rs` with dedicated module tests
  - started `ARCH-011` by extracting queue ownership into `src/engine/event_queue/*`:
    - `event_type_catalog.rs`
    - `queued_event_record.rs`
    - `event_queue_ordering.rs`
    - `event_queue_scheduler.rs`
  - rewired `engine.rs` queue writes/reads to explicit owner APIs (`enqueue_event`, `peek_next`, `pop_next`, `reschedule_recurring_event`)
  - added focused event queue unit tests in `src/engine/event_queue/tests/event_queue_scheduler_tests.rs`
  - completed queue projection/query ownership by moving next-attack/impact and projectile-projection lookups behind event-queue owner query APIs
  - added `src/engine/event_queue/queued_projectile_impact_projection.rs` for explicit queue projection data shape
  - rewired engine snapshot/reporting helpers to consume queue owner query APIs instead of scanning queue internals
  - started `ARCH-012` by extracting incoming-damage ownership into:
    - `src/engine/event_resolution.rs`
    - `src/engine/event_resolution/incoming_damage_resolution.rs`
  - moved controlled champion/enemy damage and runtime-heal mutation methods out of the facade into event-resolution ownership
  - completed `ARCH-012` API hardening by switching to explicit owner-command channels for controlled champion damage/heal/revive (`apply_incoming_damage_*`, `apply_healing_*`, `apply_revive_or_mark_*`)
  - moved controlled champion revive/death resolution ownership out of facade and into `src/engine/event_resolution/incoming_damage_resolution.rs`
  - completed `ARCH-010` movement-step extraction by moving the remaining actor-position owner loop into:
    - `src/engine/simulation_step.rs`
    - `src/engine/simulation_step/enemy_movement_step.rs`
  - rewired `engine.rs` to call explicit movement-step owner command (`apply_enemy_movement_step`) during hot-effect tick progression
  - started `ARCH-013` by extracting enemy actor-state lifecycle ownership into:
    - `src/engine/actor_state.rs`
    - `src/engine/actor_state/enemy_runtime_state.rs`
  - moved enemy respawn/regeneration loops and active/alive query helpers behind actor-state owner commands (`apply_enemy_respawn_updates`, `apply_enemy_regeneration_tick`, `enemy_is_alive`, `enemy_is_active`)
  - continued `ARCH-013` by moving recurring-script eligibility checks, script-cadence readiness writes, and enemy next-attack physical-bonus mutation behind actor-state owner APIs
  - continued `ARCH-013` by moving enemy auto-attack token lifecycle and next-hit bonus consume/reset behind actor-state owner APIs (`begin_enemy_attack_sequence`, `enemy_attack_sequence_matches`, `consume_enemy_attack_damage_with_on_hit`)
  - hardened attack-event owner-call boundaries with fail-fast invariant checks for invalid indices (quality guardrail; no behavior change expected)
  - continued `ARCH-013` by moving remaining script-runtime enemy-state mutation and script lifecycle projections behind actor-state owner APIs (`execute_enemy_script_event_actions`, `enemy_aftershock_magic_damage_on_immobilize`, `enemy_script_epoch_matches`, `enemy_script_event_ready_at_or_zero`)
  - added focused actor-state regression tests in `src/tests/engine_tests.rs` for attack token lifecycle, next-hit bonus consume/reset, script epoch/readiness owner queries, and new script-runtime owner channels
  - continued `ARCH-013` by moving high-traffic enemy read projections behind actor-state owner query APIs (`enemy_name`, `enemy_position`, `enemy_hitbox_radius`, `enemy_attack_*`, `enemy_target_health_snapshot_or_defaults`, `enemy_status_lines_at`, `enemy_is_*_at`)
  - removed direct `enemy_state[idx]` and `enemy_state.get(...)` usage from `src/engine.rs` runtime paths
  - added focused actor-state regression test `enemy_read_projection_owner_queries_return_expected_shapes`
  - continued `ARCH-013` by moving trace-snapshot enemy-section read composition behind actor-state owner projection APIs (`enemy_count`, `enemy_trace_snapshot_at`)
  - rewired `collect_state_snapshot_summary` to consume actor-state snapshots instead of facade-side `enemy_state` iteration/read composition
  - extended actor-state regression coverage for `enemy_trace_snapshot_at` within `enemy_read_projection_owner_queries_return_expected_shapes`
  - started `ARCH-020` by extracting first candidate-space owner module under:
    - `src/search/candidate_space.rs`
    - `src/search/candidate_space/full_loadout_candidate_operations.rs`
  - moved full-loadout candidate helper ownership out of `search.rs` helper cluster:
    - `candidate_order_key`
    - `random_full_candidate`
    - `candidate_loadout_variants`
    - `repair_full_candidate`
    - `mutate_full_candidate`
    - `crossover_full_candidates`
  - rewired `search.rs` facade call sites to candidate-space owner helpers
  - added focused candidate-space regression tests in `src/search/candidate_space/tests/full_loadout_candidate_operations_tests.rs`
  - continued `ARCH-020` by extracting additional candidate-space owner modules:
    - `src/search/candidate_space/full_loadout_candidate_scoring.rs`
    - `src/search/candidate_space/item_candidate_operations.rs`
  - moved full-loadout scoring/ranking helper ownership out of `search.rs` helper cluster:
    - `score_full_candidates`
    - `unique_ranked_full_candidates`
  - moved item-only candidate mutation/crossover/parent-selection helper ownership out of `search.rs` helper cluster:
    - `tournament_parent`
    - `crossover_builds`
    - `mutate_build`
  - continued `ARCH-020` by extracting item-only candidate scoring/dedupe owner module:
    - `src/search/candidate_space/item_candidate_scoring.rs`
  - moved item-only candidate scoring/dedupe helper ownership out of `search.rs` helper cluster:
    - `score_candidates`
    - `unique_ranked_from_candidates`
  - started `ARCH-021` by extracting item-only strategy implementations into explicit strategy owner modules:
    - `src/search/strategy.rs`
    - `src/search/strategy/item_candidate_search_strategies.rs`
  - moved item-only strategy helper ownership out of `search.rs` helper cluster:
    - `beam_search_ranked`
    - `random_search_ranked`
    - `hill_climb_search_ranked`
    - `genetic_search_ranked`
    - `simulated_annealing_search_ranked`
    - `mcts_search_ranked`
  - moved item-only strategy-local rollout/selection helpers under strategy ownership:
    - `available_actions`
    - `rollout_completion`
  - rewired `search.rs` strategy dispatch to consume strategy owner modules
  - rewired `search.rs` facade and candidate-space call sites to the new owner modules
  - added focused candidate-space regression tests:
    - `src/search/candidate_space/tests/full_loadout_candidate_scoring_tests.rs`
    - `src/search/candidate_space/tests/item_candidate_operations_tests.rs`
    - `src/search/candidate_space/tests/item_candidate_scoring_tests.rs`
  - added focused strategy-module regression tests:
    - `src/search/strategy/tests/item_candidate_search_strategies_tests.rs`
  - completed `ARCH-021` by extracting full-loadout strategy implementations into explicit strategy owner module:
    - `src/search/strategy/full_loadout_search_strategies.rs`
  - moved full-loadout strategy helper ownership out of `search.rs` helper cluster:
    - `beam_search_ranked_full`
    - `random_search_ranked_full`
    - `hill_climb_search_ranked_full`
    - `genetic_search_ranked_full`
    - `simulated_annealing_search_ranked_full`
    - `mcts_search_ranked_full`
    - `tournament_parent_full`
    - `MctsFullNode`
  - rewired `search.rs` full-loadout strategy dispatch to consume strategy owner modules
  - added focused strategy-module regression tests:
    - `src/search/strategy/tests/full_loadout_search_strategies_tests.rs`
  - started `ARCH-022` by extracting explicit scoring owner modules:
    - `src/search/scoring.rs`
    - `src/search/scoring/metric_scoring_helpers.rs`
    - `src/search/scoring/item_build_scoring_and_diversity.rs`
    - `src/search/scoring/full_loadout_scoring_and_diversity.rs`
  - continued `ARCH-022` by extracting stat-key selection owner module:
    - `src/search/scoring/stat_key_build_selection.rs`
  - continued `ARCH-022` by extracting item-name formatting owner module:
    - `src/search/scoring/item_name_list_formatting.rs`
  - moved scoring/diversity helper ownership out of `search.rs` helper clusters:
    - `select_diverse_top_builds`
    - `compute_build_metrics`
    - `pareto_front_keys`
    - `select_diverse_top_candidates`
    - `compute_build_metrics_for_candidate`
    - `candidate_pareto_front_keys`
    - `choose_best_build_by_stat`
    - `item_names`
  - rewired `search.rs` scoring/diversity facade paths to thin owner wrappers over `search/scoring/*`
  - added focused scoring-module regression tests:
    - `src/search/scoring/tests/item_build_scoring_and_diversity_tests.rs`
    - `src/search/scoring/tests/full_loadout_scoring_and_diversity_tests.rs`
    - `src/search/scoring/tests/stat_key_build_selection_tests.rs`
    - `src/search/scoring/tests/item_name_list_formatting_tests.rs`
  - started `ARCH-030` by extracting scenario parsing ownership into:
    - `src/scenario_runner/scenario_parsing.rs`
  - moved scenario parser helper ownership out of `scenario_runner.rs` helper cluster:
    - `parse_controlled_champion_config`
    - `parse_scenario_search_or_default`
  - rewired `scenario_runner.rs` parser call sites to consume `scenario_runner/scenario_parsing.rs` owner APIs
  - continued `ARCH-030` by extracting encounter parsing ownership into:
    - `src/scenario_runner/encounter_parsing.rs`
  - moved encounter parser helper ownership into typed owner APIs:
    - `parse_opponent_encounters`
    - `ParsedOpponentEncounter`
  - rewired `scenario_runner.rs` encounter parse boundaries to consume `scenario_runner/encounter_parsing.rs` owner APIs
  - continued `ARCH-030` by extracting run-output path/key ownership into:
    - `src/scenario_runner/run_output_paths.rs`
  - moved run-output helper ownership out of `scenario_runner.rs` helper cluster:
    - `format_repo_relative_path`
    - `search_quality_profile_key`
    - `default_run_output_directory`
    - `default_fixed_loadout_output_directory`
    - `default_fixed_loadout_rune_sweep_output_directory`
  - rewired `scenario_runner.rs` run-output path/reporting call sites to consume `scenario_runner/run_output_paths.rs` owner APIs
  - continued `ARCH-030` by extracting scenario search-progress/runtime-counter ownership into:
    - `src/scenario_runner/progress_reporting.rs`
  - moved progress helper ownership out of `scenario_runner.rs` helper cluster:
    - `initialize_search_type_counters`
    - `increment_search_type_counter`
    - `snapshot_search_type_counters`
    - `unique_loadout_selection_count`
    - `unique_loadout_selection_count_from_ranked`
  - rewired `scenario_runner.rs` progress and diagnostics call sites to consume `scenario_runner/progress_reporting.rs` owner APIs
  - continued `ARCH-030` by extracting strict-ranking and search-space estimation helper ownership into:
    - `src/scenario_runner/strict_ranking_ordering.rs`
    - `src/scenario_runner/search_space_estimation.rs`
  - moved strict-ranking/search-space helper ownership out of `scenario_runner.rs` helper cluster:
    - `heuristic_sort_remaining_candidates_for_strict_ranking`
    - `estimated_legal_item_build_count`
    - `estimated_legal_loadout_count`
    - `estimate_close_to_optimal_probability`
    - `format_percent_display`
  - rewired `scenario_runner.rs` strict-ranking and diagnostics call sites to consume `scenario_runner/strict_ranking_ordering.rs` and `scenario_runner/search_space_estimation.rs` owner APIs
  - started `ARCH-031` by extracting fixed-loadout execution entrypoints into dedicated runner modules:
    - `src/scenario_runner/fixed_loadout_runner.rs`
    - `src/scenario_runner/rune_sweep_runner.rs`
  - moved execution-entrypoint implementation ownership out of `scenario_runner.rs`:
    - `run_controlled_champion_fixed_loadout_evaluation`
    - `run_controlled_champion_fixed_loadout_rune_sweep`
  - rewired `scenario_runner.rs` to thin facade wrappers that delegate to fixed-loadout/rune-sweep runner owner modules
  - added architecture progress metrics helper:
    - `tools/architecture_metrics.sh`
  - metrics helper now reports tracked-facade baseline/current/target lines and size-based completion percentage in one command
  - completed `ARCH-031` by extracting controlled-champion scenario execution entrypoint ownership into:
    - `src/scenario_runner/controlled_champion_scenario_runner.rs`
  - moved controlled-champion execution-entrypoint implementation ownership out of `scenario_runner.rs`:
    - `run_controlled_champion_scenario`
  - rewired `scenario_runner.rs` to a thin facade wrapper delegating to `run_controlled_champion_scenario_impl`
  - started `ARCH-040` by extracting defaults owner module:
    - `src/defaults/champion_item_simulation_defaults_loader.rs`
  - moved champion/item simulation-default loader ownership out of `src/defaults.rs`:
    - Vladimir cast/offensive/pool/policy defaults loaders
    - Warwick/Vayne/Morgana/Sona/Doctor Mundo ability defaults loaders
    - Zhonya/Guardian Angel/Protoplasm item simulation-default loaders
    - related champion/item ability-effect extraction helpers
  - rewired `src/defaults.rs` to keep typed `OnceLock` facade accessors while delegating loader internals to the defaults owner module
  - started `ARCH-041` by extracting data owner module:
    - `src/data/champion_item_preset_data_loading.rs`
  - moved champion/item/preset data loading ownership out of `src/data.rs`:
    - champion base loading/lookup
    - URF mode data loading
    - item stat mapping/loading and item-pool legality helpers
    - enemy preset loading/validation/loadout conversion
  - rewired `src/data.rs` to keep parse/config/loadout-domain facade responsibilities while delegating champion/item/preset loader internals to the data owner module
  - completed `ARCH-041` by extracting remaining data concern owner modules:
    - `src/data/loadout_effect_resolution.rs`
    - `src/data/simulation_search_configuration_parsing.rs`
    - `src/data/loadout_domain_modeling.rs`
  - rewired `src/data.rs` into a thin facade with explicit data-concern re-exports
  - continued `ARCH-013` by extracting event-dispatch/casting/hot-effect ownership out of `src/engine.rs` into:
    - `src/engine/event_resolution/combat_event_dispatch_resolution.rs`
    - `src/engine/event_resolution/controlled_champion_casting_resolution.rs`
    - `src/engine/event_resolution/enemy_script_action_resolution.rs`
    - `src/engine/simulation_step/hot_effects_step.rs`
  - moved event/casting/tick lifecycle method ownership out of `src/engine.rs`:
    - `process_event`
    - `step`
    - `maybe_cast_controlled_champion_abilities_and_defensives`
    - `apply_enemy_script_actions`
    - `apply_hot_effects`
  - rewired module carriers in `src/engine/event_resolution.rs` and `src/engine/simulation_step.rs` to explicit owner modules
  - completed `ARCH-040` by extracting defaults schema/helper ownership out of `src/defaults.rs` into:
    - `src/defaults/simulator_defaults_schema_types.rs`
    - `src/defaults/defaults_path_key_and_effect_helpers.rs`
  - rewired `src/defaults.rs` to explicit module re-exports/imports so it remains a thin typed facade and loader-access layer
  - added focused scenario parser regression coverage in:
    - `src/tests/scenario_runner_tests.rs`
    - `parse_controlled_champion_config_rejects_legacy_baseline_items_key`
    - `parse_opponent_encounters_preserves_typed_encounter_fields`
    - `default_run_output_directory_compacts_popcorn_window_when_equal_to_budget`
    - `default_fixed_loadout_output_directory_normalizes_label_key`
    - `format_repo_relative_path_uses_repository_relative_simulation_paths`
    - `unique_loadout_selection_count_helpers_track_distinct_loadouts`
    - `search_type_counter_helpers_dedupe_keys_and_report_touched_entries_only`
    - `estimated_legal_item_build_count_applies_single_boot_constraint`
    - `estimated_legal_loadout_count_matches_small_domain_combinatorics`
    - `estimate_close_to_optimal_probability_reports_unavailable_when_space_missing`
    - `format_percent_display_uses_scientific_notation_for_tiny_percent_values`
    - `strict_ranking_heuristic_ordering_sorts_by_signal_when_enabled_without_promotions`
    - `strict_ranking_heuristic_ordering_keeps_input_order_when_scores_are_flat`
  - reduced `src/scenario_runner.rs` from `4284` lines to `936` lines while keeping parser/output/progress/strict-ranking/execution regression coverage green
  - reduced `src/defaults.rs` from `2455` lines to `679` lines while keeping defaults access behavior stable
  - reduced `src/data.rs` from `2008` lines to `116` lines while keeping data parse/load behavior stable
  - reduced `src/search.rs` from `2244` lines to `942` lines while keeping validation green
  - reduced `src/engine.rs` further from `3277` lines to `1461` lines while keeping validation green
  - continued `ARCH-013` by extracting trace/snapshot runtime reporting ownership out of `src/engine.rs` into:
    - `src/engine/trace_snapshot_reporting.rs`
  - reduced `src/engine.rs` from `1461` lines to `1025` lines while preserving behavior
  - completed `ARCH-050` by extracting reporting writer ownership out of `src/reporting.rs` into:
    - `src/reporting/controlled_champion_report_markdown_writer.rs`
    - `src/reporting/controlled_champion_report_json_writer.rs`
  - rewired `src/reporting.rs` into a thin report helper/re-export facade
  - reduced `src/reporting.rs` from `1075` lines to `140` lines while preserving behavior
  - continued `ARCH-013` by extracting combat timing/targeting/scheduling and enemy stat-model derivation ownership out of `src/engine.rs` into:
    - `src/engine/combat_timing_and_targeting.rs`
    - `src/engine/enemy_combat_stat_modeling.rs`
  - moved controlled champion status/cast/attack gating, enemy range/targeting/projectile-block helpers, attack/event scheduling, and `run_until_end` into explicit owner methods
  - moved `derive_enemy_model` and `derive_enemy_combat_stats` into explicit enemy combat-stat owner module while preserving `engine.rs` facade re-export for `derive_enemy_combat_stats`
  - reduced `src/engine.rs` from `1025` lines to `601` lines while preserving behavior (below facade target `<=700`)
  - completed `ARCH-020`/`ARCH-022` by extracting full-loadout search orchestration ownership out of `src/search.rs` into:
    - `src/search/full_loadout_search_orchestration.rs`
  - moved `FullLoadoutSearchParams` and full-loadout search orchestration helpers (`build_search_ranked_full_loadout`, seed-elite aggregation, adaptive strategy expansion, bleed candidate generation) into explicit search owner module
  - rewired `src/search.rs` into a thinner facade with explicit full-loadout orchestration re-exports
  - reduced `src/search.rs` from `942` lines to `569` lines while preserving behavior (below facade target `<=700`)
  - completed `ARCH-030` by extracting controlled champion scenario runtime/search support helper ownership out of `src/scenario_runner.rs` into:
    - `src/scenario_runner/controlled_champion_search_runtime_support.rs`
  - moved coverage-asset locking, partial-candidate completion, progress-state/counter helpers, trace-event shaping, and rune telemetry helper ownership into explicit scenario support owner module
  - rewired `src/scenario_runner.rs` to a thinner facade that imports support ownership for fixed-loadout/rune-sweep/progress-reporting/controlled-scenario flows
  - reduced `src/scenario_runner.rs` from `936` lines to `273` lines while preserving behavior (below facade target `<=700`)
  - started `ARCH-051` by extracting core combat primitives/status/cast-lock ownership out of `src/core.rs` into:
    - `src/core/combat_primitives_state.rs`
  - rewired `src/core.rs` to preserve facade re-exports while delegating status/cast-lock/combat-primitives ownership to explicit core owner module
  - reduced `src/core.rs` from `933` lines to `611` lines while preserving behavior (below facade target `<=700`)
  - continued scenario execution leaf decomposition by extracting controlled-champion candidate-search orchestration out of `src/scenario_runner/controlled_champion_scenario_runner.rs` into:
    - `src/scenario_runner/controlled_champion_candidate_search.rs`
  - moved maximum-quality coverage stage, ensemble-seed strategy orchestration, candidate merge/dedupe, and strict full-ranking loops into explicit scenario-runner search-owner helpers
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` to delegate search-phase orchestration while preserving fallback/reporting behavior
  - reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `1783` lines to `1425` lines
  - continued scenario execution leaf decomposition by extracting controlled-champion post-search result reporting/trace artifact ownership out of `src/scenario_runner/controlled_champion_scenario_runner.rs` into:
    - `src/scenario_runner/controlled_champion_result_reporting.rs`
  - moved console summary output, diagnostics assembly, build-order summary rendering, trace markdown/json writing, and report markdown/json writing into explicit scenario-runner result-reporting ownership
  - rewired `src/scenario_runner/controlled_champion_scenario_runner.rs` to delegate post-search reporting through `emit_controlled_champion_result_reporting`
  - reduced `src/scenario_runner/controlled_champion_scenario_runner.rs` from `1425` lines to `855` lines
  - continued scenario result-reporting decomposition by extracting analysis and artifact-writing ownership out of `src/scenario_runner/controlled_champion_result_reporting.rs` into:
    - `src/scenario_runner/controlled_champion_result_build_analysis.rs`
    - `src/scenario_runner/controlled_champion_result_artifact_writing.rs`
  - moved ranked-build analysis, diagnostics assembly, and build-order analysis into explicit analysis ownership and moved trace/report artifact writing plus final output emission into explicit artifact-writing ownership
  - rewired `src/scenario_runner/controlled_champion_result_reporting.rs` into a thinner orchestration module that delegates to explicit analysis/artifact owners
  - reduced `src/scenario_runner/controlled_champion_result_reporting.rs` from `813` lines to `489` lines
  - completed `ARCH-014` by splitting event dispatch into explicit event-family owner modules:
    - `src/engine/event_resolution/combat_event_enemy_auto_attack_resolution.rs`
    - `src/engine/event_resolution/combat_event_controlled_champion_auto_attack_resolution.rs`
    - `src/engine/event_resolution/combat_event_controlled_champion_offensive_ability_hit_resolution.rs`
    - `src/engine/event_resolution/combat_event_champion_script_dispatch_resolution.rs`
  - rewired `src/engine/event_resolution/combat_event_dispatch_resolution.rs` to a thin dispatcher + `step` lifecycle owner delegating through explicit `resolve_*` owner methods
  - reduced `src/engine/event_resolution/combat_event_dispatch_resolution.rs` from `723` lines to `123` lines
  - re-ran full correctness/quality validation after progress-owner extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after strict-ranking/search-space extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after fixed-loadout/rune-sweep execution extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after controlled-champion scenario execution extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after defaults owner-module extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after defaults schema/helper extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after data owner-module extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after completed data-facade decomposition (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after engine event-dispatch/casting/hot-effect extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after engine trace/snapshot extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after reporting module decomposition (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after combat timing/targeting and enemy stat-model extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after search full-loadout orchestration extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after scenario support-owner extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after core combat-primitives extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after scenario candidate-search extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after scenario result-reporting extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after scenario result-reporting analysis/artifact split (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after event-dispatch owner-slice extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after champion-family defaults-leaf decomposition (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - re-ran full correctness/quality validation after runtime-state-reporting extraction (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - completed second-stage scenario execution decomposition by splitting controlled champion execution ownership into:
    - `src/scenario_runner/controlled_champion_scenario_runner.rs` (thin facade)
    - `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs` (execution owner leaf)
  - completed second-stage reporting decomposition for loadout/build projection ownership by splitting:
    - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/build_ranking_sections.rs`
    - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/enemy_profile_sections.rs`
    - `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/loadout_profile_sections.rs`
  - removed root compatibility shims (`crate::Ordering`, `crate::EnemyDerivedCombatStats`) and updated downstream modules to explicit owner imports
  - re-ran full correctness/quality validation after scenario/reporting split + shim cleanup (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
  - completed optional follow-up split for item-pool ownership by extracting:
    - `src/data/champion_item_preset_data_loading/item_pool_loading/item_metadata_loading.rs`
    - `src/data/champion_item_preset_data_loading/item_pool_loading/item_pool_selection_filters.rs`
    while preserving `item_pool_loading.rs` as a thin facade (`8` lines)
  - completed optional follow-up split for search-config parse ownership by extracting:
    - `src/data/simulation_search_configuration_parsing/build_search_config_parsing/build_search_config_value_mapping.rs`
    - `src/data/simulation_search_configuration_parsing/build_search_config_parsing/search_quality_profile_application.rs`
    while preserving `build_search_config_parsing.rs` as a thin facade (`7` lines)
  - completed optional follow-up split for defaults schema ownership by extracting:
    - `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema/champion_behavior_baseline_defaults_schema.rs`
    - `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema/vladimir_ability_defaults_schema.rs`
    - `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema/champion_specific_ability_defaults_schema.rs`
    - `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema/item_survivability_defaults_schema.rs`
    while preserving `champion_behavior_and_ability_defaults_schema.rs` as a thin facade (`9` lines)
  - completed high-value shared projection extraction by adding:
    - `src/scenario_runner/controlled_champion_enemy_scenario_projection.rs`
    and reusing it from fixed-loadout and rune-sweep flows to reduce cross-runner duplication
  - completed high-friction dense-leaf decomposition follow-up by splitting:
    - `src/search/strategy/item_candidate_search_strategies.rs` into explicit owner leaves under `src/search/strategy/item_candidate_search_strategies/`
    - `src/scripts/champions.rs` into explicit owner leaves under `src/scripts/champions/`
    - `src/engine/trace_snapshot_reporting.rs` into explicit owner leaves under `src/engine/trace_snapshot_reporting/`
    - `src/data/loadout_domain_modeling.rs` into explicit owner leaves under `src/data/loadout_domain_modeling/`
    - `src/engine/actor_state/enemy_runtime_state.rs` into explicit owner leaves under `src/engine/actor_state/enemy_runtime_state/`
  - reduced:
    - `src/search/strategy/item_candidate_search_strategies.rs` from `458` to `110` lines
    - `src/scripts/champions.rs` from `457` to `36` lines
    - `src/engine/trace_snapshot_reporting.rs` from `440` to `3` lines
    - `src/data/loadout_domain_modeling.rs` from `418` to `15` lines
    - `src/engine/actor_state/enemy_runtime_state.rs` from `416` to `6` lines
  - re-ran full correctness/quality validation after optional follow-up slices (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test --release`) with no findings
- Follow-up options:
  - split `src/search/strategy/full_loadout_search_strategies.rs` and `src/search/full_loadout_search_orchestration.rs` to further narrow search-strategy/orchestration ownership leaves
  - split `src/scripts/runtime/loadout_runtime.rs` and `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` to reduce runtime hotspot density
  - split `src/scenario_runner/controlled_champion_candidate_search/seed_and_strict_execution.rs` and `src/scripts/champions/controlled_champion.rs` for narrower execution/script ownership leaves
  - continue focused regression coverage per future extraction slice and serialize validation runs during concurrent data-file edits
- Success criteria:
  - met: large core facades are thin, responsibilities are split by concern, and cross-module direct state mutation paths are removed.

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

23a. `DONE` Random-by-default seed policy with deterministic override.
- Scope:
  - runtime-random seed default for broader exploration
  - explicit deterministic override via CLI/config seed
  - effective seed surfaced in diagnostics/report output
- Success criteria:
  - reproducibility is explicit and opt-in, while default search remains broad.

23b. `DONE` Maximum-quality pre-budget coverage stage.
- Scope:
  - touch each legal item/rune/shard asset at least once before timed optimization
  - collect per-asset top diverse candidates and inject into main search pool
  - start time-budget accounting after coverage stage completion
- Recent progress:
  - popcorn progress-window timeout no longer interrupts coverage-stage execution; coverage remains pre-budget and breadth-guaranteed.
  - incomplete coverage now runs in explicit degraded mode (warning + output flag) instead of hard-failing the run.
- Success criteria:
  - breadth floor is guaranteed for high-quality runs and reported in diagnostics.

23c. `DONE` Remove baseline-reference reporting/evaluation workflow.
- Scope:
  - remove baseline build parsing and evaluation from scenario execution
  - emit optimized-build-only report headline/breakdowns
  - emit a single optimized-build trace timeline (no baseline/best split)
- Success criteria:
  - run outputs no longer perform or present baseline comparisons.

23d. `DONE` Improve run-output key clarity and trace JSON contract stability.
- Scope:
  - make popcorn runtime-stop keys deterministic and human-readable
  - avoid duplicate budget/window tokens when values are equal
  - emit schema-versioned structured trace JSON events for tooling consumers
- Success criteria:
  - output directories are self-describing and trace JSON parsers can rely on stable typed fields.

23e. `DONE` Parallelize top-level search orchestration paths.
- Scope:
  - parallelize ensemble seed orchestration
  - parallelize portfolio strategy execution
  - parallelize strategy-elite/adaptive generation loops
  - preserve deterministic merge ordering for reproducible seeded runs
- Success criteria:
  - multiple strategy families run concurrently and reports expose effective parallelism state.

23f. `DONE` Harden full-loadout strategy correctness and determinism.
- Scope:
  - co-optimize loadout selection during `beam` and `greedy` full-loadout item search
  - normalize adaptive/bleed strategy-key ordering before index-based seed derivation
  - recover and complete timed-out seed-stage partial candidates before strict full ranking fallback
  - keep report/build-order metrics loadout-accurate on in-run cache hits
- Success criteria:
  - fixed-seed reruns are reproducible, short-budget runs do not random-fallback when partial progress exists, and diagnostics reflect each candidate's own loadout stats.

23g. `DONE` Arm timed-search budget on first timed-phase simulation evaluation.
- Scope:
  - avoid consuming `--max-runtime-seconds` during setup/reporting
  - preserve `maximum_quality` coverage as pre-budget
  - allow in-flight simulation calls to complete naturally before wrap-up
- Success criteria:
  - short timed runs execute scored simulation work before timeout handling, while finalization is not truncated by deadline checks.

23h. `DONE` Add strict-stage heuristic ordering controls and direct fixed-loadout evaluator.
- Scope:
  - add strict full-ranking ordering controls for remaining candidates:
    - item/rune/shard signal weighting
    - random exploration promotions at the front of queue
    - zero-variance guard to avoid introducing fake ranking signal when strict seed scores are flat
  - add direct `controlled_champion_fixed_loadout` mode for one-loadout reports/traces to support controlled A/B comparisons
  - make strict-score ties deterministic with objective-side tiebreaks and stable key fallback
- Success criteria:
  - strict-stage ordering is explainable and tunable, flat-score phases avoid noisy heuristic bias, and users can run direct loadout comparisons without full search overhead.

23i. `DONE` Align build-order/report diagnostics with legal encounter/loadout semantics.
- Scope:
  - evaluate build-order stages across all configured opponent encounters using encounter weights plus worst-case blend
  - make report loadout-label validation rely on selected loadout legality and tolerate unmodeled shard labels
- Success criteria:
  - build-order recommendations are optimized against the same multi-encounter objective framing as candidate scoring, and legal loadouts no longer fail report generation due to missing unmodeled shard labels.

24. `PLANNED` Performance profiling workflow and flamegraphs.
- Scope:
  - repeatable profiling command and report artifacts
- Success criteria:
  - hotspot regressions become actionable quickly.

25. `IN_PROGRESS` Reduce synchronization overhead in caches/scoring.
- Scope:
  - minimize lock contention and shared hot-map pressure
- Recent progress:
  - replaced high-frequency search-type counter mutex updates with per-type atomics
  - replaced global unique-scored-key mutex set with sharded key-set storage
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

38. `DONE` Full codebase structure audit and abstraction-driven reorganization plan.
- Scope:
  - review the entire `Simulation/src` layout for abstraction boundaries and long-term extensibility
  - identify and prioritize high-friction files/modules for relocation or split
  - define a migration map with incremental compatibility checkpoints
- Success criteria:
  - a concrete target module layout exists with phased migration steps and acceptance criteria.

39. `DONE` Domain-oriented script hierarchy for champions, items, and runes.
- Scope:
  - replace flat script layout with domain folders (for example `scripts/champions/`, `scripts/items/`, `scripts/runes/`)
  - move per-entity script logic into dedicated files (for example one champion per module)
  - keep shared runtime primitives and registries in separate shared modules
- Success criteria:
  - script organization is discoverable, scalable, and does not rely on monolithic files.

40. `DONE` Naming scheme and module sizing standards.
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
- `DONE` Item 38 (audit completed; phased architecture migration and acceptance criteria documented)
- `IN_PROGRESS` Item 36 (champion/item coupling reduced by moving defensive item and revive decisions into generic runtime/item capability scripts)
- `DONE` Item 39 (domain-oriented script hierarchy is in place with explicit owner modules)
- `DONE` Item 40 (descriptive naming and module sizing standards are documented and applied)

## Notes
- Large items are being delivered in iterative slices with strict compile/test/lint validation at each slice.
- Any low-confidence behavior assumptions are tracked in:
  - `Simulation/CONFIDENCE_REVIEW.md`

# Champion Calculations (League of Legends Simulator)

This repository contains a data-driven combat simulator focused on URF team-fight optimization, with Vladimir and Sona as current controlled champion benchmark scenarios.

## Project Goal
- Build a generic, reusable simulation engine that can evaluate champion + item + rune loadouts with realistic combat behavior.
- Keep the engine generic and move champion/item/rune-specific mechanics into script modules and canonical data files.
- Use search algorithms to find strong full-build outcomes and strong build orders.

## Current State (Important)
- Runtime implementation is Rust (`Simulation/`).
- Search is parallelized and supports multiple algorithms (`beam`, `hill_climb`, `genetic`, `simulated_annealing`, `mcts`, `random`, `portfolio`).
- Controlled-champion and opponent simulation use shared generic abstractions (actors/champions), not enemy-only core paths.
- A shared world ownership scaffold exists under `Simulation/src/world/` and is used for deterministic encounter placement validation before run execution.
- Controlled champion script coverage currently includes `Vladimir` and `Sona`.
- Controlled-champion modes now fail fast when the selected champion has no registered controlled-champion script, preventing silent no-script degradations.
- Runtime metrics are resolved from canonical base data plus active buff state through shared stat queries:
  - cooldown metrics (ability/item/neutral)
  - scalar combat metrics (incoming damage taken, healing, movement speed, and outgoing bonus-ability damage)
- Controlled champion now runs explicit basic-attack start/windup/hit events (hitbox/projectile-aware) and uses shared runtime attack-speed/on-hit effect paths.
- Controlled champion cast-lock state now gates cast permission, preventing same-tick spell stacking.
- Engine ownership channels continue to move out of `Simulation/src/engine.rs`:
  - incoming damage, healing, and revive/death transitions now route through explicit event-resolution owner APIs in `Simulation/src/engine/event_resolution/incoming_damage_resolution.rs`
  - event queue scheduling/projection ownership remains in `Simulation/src/engine/event_queue/*`
  - combat event dispatch/tick-step progression now routes through `Simulation/src/engine/event_resolution/combat_event_dispatch_resolution.rs` with event-family owner slices in `Simulation/src/engine/event_resolution/combat_event_*_resolution.rs`
  - controlled champion cast/defensive orchestration now routes through `Simulation/src/engine/event_resolution/controlled_champion_casting_resolution.rs`
  - enemy script-action impact/followup scheduling now routes through `Simulation/src/engine/event_resolution/enemy_script_action_resolution.rs`
  - enemy movement position updates now route through `Simulation/src/engine/simulation_step/enemy_movement_step.rs` (`apply_enemy_movement_step`)
  - controlled champion hot-effect tick lifecycle now routes through `Simulation/src/engine/simulation_step/hot_effects_step.rs` (`apply_hot_effects`)
  - controlled champion status/cast/attack gating, enemy range/targeting/projectile-block helpers, and attack/event scheduling now route through `Simulation/src/engine/combat_timing_and_targeting.rs`
  - enemy derived combat-stat/loadout-runtime modeling now routes through `Simulation/src/engine/enemy_combat_stat_modeling.rs` (`derive_enemy_model`, `derive_enemy_combat_stats`)
  - enemy respawn/regeneration lifecycle and active/alive queries now route through `Simulation/src/engine/actor_state/enemy_runtime_state.rs` facade plus explicit owner leaves under `Simulation/src/engine/actor_state/enemy_runtime_state/`
  - recurring script-event eligibility and script-cadence readiness writes now route through actor-state owner APIs in `Simulation/src/engine/actor_state/enemy_runtime_state/enemy_lifecycle_channels.rs`
  - champion-script event epoch/readiness projections and script-runtime mutation now route through actor-state owner APIs in explicit owner leaves under `Simulation/src/engine/actor_state/enemy_runtime_state/`
  - high-traffic enemy read projections (name/position/hitbox/attack profile/health snapshots/status lines) now route through actor-state owner query APIs in `Simulation/src/engine/actor_state/enemy_runtime_state/enemy_trace_snapshot_projections.rs`
  - trace-snapshot enemy-section composition now routes through actor-state owner projection APIs (`enemy_count`, `enemy_trace_snapshot_at`) in `Simulation/src/engine/actor_state/enemy_runtime_state/enemy_trace_snapshot_projections.rs`
  - enemy auto-attack token lifecycle and next-hit bonus consume/reset now route through actor-state owner APIs in `Simulation/src/engine/actor_state/enemy_runtime_state/enemy_attack_and_script_channels.rs`
  - trace lifecycle/status/summary composition now routes through `Simulation/src/engine/trace_snapshot_reporting.rs` facade plus explicit owner leaves under `Simulation/src/engine/trace_snapshot_reporting/`
- Search ownership extraction is complete under `Simulation/src/search/candidate_space/*`:
  - full-loadout candidate mutation/canonicalization helper ownership now routes through `Simulation/src/search/candidate_space/full_loadout_candidate_operations.rs`
  - full-loadout candidate scoring/ranking helper ownership now routes through `Simulation/src/search/candidate_space/full_loadout_candidate_scoring.rs`
  - item-only candidate mutation/crossover/parent-selection helper ownership now routes through `Simulation/src/search/candidate_space/item_candidate_operations.rs`
  - item-only candidate scoring/dedupe helper ownership now routes through `Simulation/src/search/candidate_space/item_candidate_scoring.rs`
- Full-loadout search orchestration ownership now routes through `Simulation/src/search/full_loadout_search_orchestration.rs` facade plus explicit owner leaves under `Simulation/src/search/full_loadout_search_orchestration/` (`strategy_dispatch.rs`, `seed_elite_generation.rs`, `adaptive_candidate_generation.rs`, `bleed_candidate_generation.rs`).
- Search strategy ownership now routes through `Simulation/src/search/strategy/*`:
  - item-only strategy facade ownership routes through `Simulation/src/search/strategy/item_candidate_search_strategies.rs` with explicit owner leaves under `Simulation/src/search/strategy/item_candidate_search_strategies/`
  - full-loadout strategy helper ownership routes through `Simulation/src/search/strategy/full_loadout_search_strategies.rs` facade with explicit owner leaves under `Simulation/src/search/strategy/full_loadout_search_strategies/`
- Search scoring/diversity ownership now routes through `Simulation/src/search/scoring/*`:
  - item-build scoring/diversity helper ownership routes through `Simulation/src/search/scoring/item_build_scoring_and_diversity.rs`
  - full-loadout scoring/diversity helper ownership routes through `Simulation/src/search/scoring/full_loadout_scoring_and_diversity.rs`
  - stat-key targeted item-build selection helper ownership routes through `Simulation/src/search/scoring/stat_key_build_selection.rs`
  - item-name list formatting helper ownership routes through `Simulation/src/search/scoring/item_name_list_formatting.rs`
- Scenario ownership extraction is complete and routes through explicit owner modules:
  - controlled champion/search-default parse helpers now route through `Simulation/src/scenario_runner/scenario_parsing.rs`
  - opponent encounter parse and legacy-key validation helpers now route through `Simulation/src/scenario_runner/encounter_parsing.rs` with typed parse output (`ParsedOpponentEncounter`)
  - run-output path/key formatting helpers now route through `Simulation/src/scenario_runner/run_output_paths.rs`
  - search progress counters and unique-loadout count helpers now route through `Simulation/src/scenario_runner/progress_reporting.rs`
  - strict-ranking heuristic ordering helpers now route through `Simulation/src/scenario_runner/strict_ranking_ordering.rs`
  - legal candidate-space estimation/probability formatting helpers now route through `Simulation/src/scenario_runner/search_space_estimation.rs`
  - controlled champion runtime/search support helper ownership now routes through `Simulation/src/scenario_runner/controlled_champion_search_runtime_support.rs` (coverage-asset locking, partial-candidate completion, telemetry/trace shaping, progress-state primitives)
  - controlled champion candidate-search phase orchestration now routes through `Simulation/src/scenario_runner/controlled_champion_candidate_search.rs` with explicit phase-owner leaves (`coverage_stage_execution.rs`, `seed_and_strict_execution.rs`, `seed_and_strict_execution/*`)
  - controlled champion setup and enemy-build preparation ownership now routes through `Simulation/src/scenario_runner/controlled_champion_scenario_setup.rs`
  - controlled champion strict-ranking fallback/tie-break/seed-diagnostics finalization ownership now routes through `Simulation/src/scenario_runner/controlled_champion_strict_ranking_finalization.rs`
  - controlled champion post-search result-reporting orchestration now routes through `Simulation/src/scenario_runner/controlled_champion_result_reporting.rs`
  - controlled champion ranked-build analysis and diagnostics assembly now route through `Simulation/src/scenario_runner/controlled_champion_result_build_analysis.rs`
  - controlled champion trace/report artifact writing now routes through `Simulation/src/scenario_runner/controlled_champion_result_artifact_writing.rs`
  - fixed-loadout and fixed-loadout-rune-sweep execution entrypoint implementation now routes through `Simulation/src/scenario_runner/fixed_loadout_runner.rs` and `Simulation/src/scenario_runner/rune_sweep_runner.rs`, with rune-sweep aggregation/report projection split into explicit leaves under `Simulation/src/scenario_runner/rune_sweep_runner/`
  - controlled-champion scenario execution entrypoint now routes through `Simulation/src/scenario_runner/controlled_champion_scenario_runner.rs` facade and `Simulation/src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs` owner leaf
- Root runtime/search/reporting/CLI contract ownership now routes through `Simulation/src/simulation_contracts.rs` plus explicit owner leaves under `Simulation/src/simulation_contracts/`, and `Simulation/src/main.rs` is now a thin orchestration entrypoint.
- Core combat-primitives/status/cast-lock ownership now routes through `Simulation/src/core/combat_primitives_state.rs`.
- Defaults champion/item simulation-default loading now routes through a thin defaults-loader facade plus explicit leaf owners:
  - `Simulation/src/defaults/champion_item_simulation_defaults_loader.rs`
  - `Simulation/src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs`
  - `Simulation/src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders/*.rs` (explicit champion-family leaves)
  - `Simulation/src/defaults/champion_item_simulation_defaults_loader/item_simulation_defaults_loaders.rs`
  - `Simulation/src/defaults/champion_item_simulation_defaults_loader/simulation_defaults_extraction_helpers.rs`
- Defaults schema/type ownership now routes through `Simulation/src/defaults/simulator_defaults_schema_types.rs` facade plus explicit leaves under `Simulation/src/defaults/simulator_defaults_schema_types/`.
- Defaults champion-simulation metadata/AI/profile loader ownership now routes through `Simulation/src/defaults/champion_simulation_data_loading.rs`.
- Defaults path/key normalization and shared JSON effect helper ownership now routes through `Simulation/src/defaults/defaults_path_key_and_effect_helpers.rs`.
- Data champion/item/preset loading and URF legality ownership now routes through `Simulation/src/data/champion_item_preset_data_loading.rs` facade plus explicit leaves under `Simulation/src/data/champion_item_preset_data_loading/`.
- Data simulation/search configuration parsing ownership now routes through `Simulation/src/data/simulation_search_configuration_parsing.rs` facade plus explicit parse owner leaves under `Simulation/src/data/simulation_search_configuration_parsing/`.
- Data loadout-domain modeling/legality/sampling ownership now routes through `Simulation/src/data/loadout_domain_modeling.rs` facade plus explicit owner leaves under `Simulation/src/data/loadout_domain_modeling/`.
- Data loadout effect/stat resolution ownership now routes through `Simulation/src/data/loadout_effect_resolution.rs`.
- Runtime combat bonus-resolution ownership now routes through `Simulation/src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` facade plus explicit owner leaves under `Simulation/src/scripts/runtime/loadout_runtime/combat_bonus_resolution/`.
- Controlled champion script ownership now routes through `Simulation/src/scripts/champions/controlled_champion.rs` facade plus explicit owner leaves under `Simulation/src/scripts/champions/controlled_champion/` (contracts, registry, channels) and `Simulation/src/scripts/champions/controlled_champion/vladimir_controlled_champion_script/` (model/capability/builder).
- Runtime rune-proc state mutation ownership now routes through `Simulation/src/scripts/runtime/loadout_runtime/combat_bonus_resolution/rune_proc_state_mutations.rs`.
- Runtime rune-proc telemetry ownership now routes through `Simulation/src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs`.
- Runtime state initialization/reset ownership now routes through `Simulation/src/scripts/runtime/loadout_runtime/runtime_state_initialization.rs`.
- Runtime mutation-effect ownership now routes through `Simulation/src/scripts/runtime/loadout_runtime/runtime_effect_mutations.rs`.
- Runtime read-only projection ownership now routes through `Simulation/src/scripts/runtime/loadout_runtime/runtime_stat_projections.rs`.
- Runtime cooldown/stack reporting ownership now routes through `Simulation/src/scripts/runtime/loadout_runtime/runtime_state_reporting.rs`.
- Reporting markdown run-report rendering ownership now routes through `Simulation/src/reporting/controlled_champion_report_markdown_writer.rs` facade plus explicit section leaves under `Simulation/src/reporting/controlled_champion_report_markdown_writer/` and `Simulation/src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/`.
- Reporting JSON run-report serialization ownership now routes through `Simulation/src/reporting/controlled_champion_report_json_writer.rs`.
- Controlled champion reports and trace outputs focus on the optimized build outcome (no baseline comparison workflow).
- Reports explicitly flag controlled champion rune selections that are currently unmodeled in deterministic/runtime combat logic.
- Shared combat-time rune triggers now model runtime effects for:
  - Press the Attack, Fleet Footwork, Conqueror, Aftershock
  - Electrocute, First Strike, Phase Rush
  - Arcane Comet, Summon Aery, Hail of Blades, Dark Harvest
  - Triumph, Gathering Storm, Second Wind
- Controlled champion and enemy actors now execute rune combat logic through the same shared runtime path.
- Aftershock resist-window mitigation is now applied during the active window for both controlled champion and enemy actors.
- Search scoring now supports an explicit unmodeled-rune quality gate policy (hard gate or per-rune score penalty) to avoid rewarding placeholder loadouts.
- Search quality profiles now enforce profile-aware unmodeled-rune policy:
  - `maximum_quality` uses hard rejection for unmodeled rune candidates
  - `fast`/`balanced` keep penalty mode
- Search scoring now also supports explicit unmodeled-item-effect quality gating (hard gate or per-item penalty), with profile-aware defaults.
- Under hard-gate profiles, controlled-champion generation space is constrained up front (modeled-rune domain + modeled-runtime-item pool) so invalid candidates are not generated and then rejected later.
- Added a direct fixed-loadout evaluation mode for controlled champion A/B comparisons without search (`controlled_champion_fixed_loadout`).
- Added a fixed-loadout keystone comparison mode (`controlled_champion_fixed_loadout_rune_sweep`) for direct one-build rune sweeps.
- Reports and traces now include rune proc telemetry with trigger-source attribution and calibration metrics (opportunity counts, proc-opportunity rates, and damage/healing share).
- Optional `simulation.combat_seed` enables deterministic combat-variation runs (enemy init order + initial attack jitter); fixed-loadout rune sweep repeats now use distinct combat seeds per repeat.
- Scenarios are strict/minimal and reference canonical data from:
  - `Characters/`
  - `Items/`
  - `Game Mode/`
  - `Masteries/`
- Enemy presets for the default URF scenario are loaded from:
  - `Simulation/data/enemy_urf_presets.json`

## Search And Seed Policy
- Default search seed is runtime-random.
- Deterministic reproducibility is available via fixed seed override:
  - CLI: `--seed <u64>`
  - Scenario: `search.seed`
- Reports always include the effective seed used.
- In `maximum_quality`, a pre-budget coverage stage runs before timed optimization:
  - every legal item/rune/shard asset is explicitly touched at least once
  - top diverse seeds from that stage are injected into main search
  - runtime budget starts after coverage stage completes
  - popcorn progress-window timeout is applied after coverage (coverage itself is protected from popcorn early-stop checks)
  - if coverage is incomplete (timeout boundary or non-finite candidate gaps), search continues in explicit degraded mode and reports a coverage warning flag
- Runtime budget for timed search now arms on first timed-phase simulation evaluation (not during setup/wrap-up).
- Full-candidate objective scoring uses in-memory per-run dedupe cache only (no disk-backed cross-run score cache).
- Full-loadout `beam` and `greedy` now co-optimize loadout selection with item expansion.
- Adaptive/bleed strategy-key ordering is normalized before seed-index derivation for fixed-seed reproducibility.
- Seed-stage partial candidates are deterministically completed before strict full-ranking fallback in short-budget runs.
- Strict full-ranking can heuristic-order remaining candidates (item/rune/shard signals) with configurable random exploration promotions.

## Directory Overview
- `Simulation/`: Rust simulator, scenarios, reports, docs, and search pipeline.
- `Characters/`: champion canonical gameplay data and defaults.
- `Items/`: item data.
- `Game Mode/`: mode rules and defaults (for example URF).
- `Masteries/`: rune/stat shard data.
- `From Online/`: external-source ingestion and normalization material.

## Key Docs
- High-detail simulator docs:
  - `Simulation/README.md`
- Coverage status and tracking:
  - `Simulation/COVERAGE_GAPS.md` (coverage snapshot + complete and incomplete domain lists)
  - `Simulation/COVERAGE_STANDARDS.md` (exemplar-derived standards for adding coverage)
  - `Simulation/COVERAGE_CHECKLIST.md` (coverage done criteria and documentation update gates)
- Architecture standards and transformation tracking:
  - `Simulation/ARCHITECTURE_STANDARDS.md`
  - `Simulation/ARCHITECTURE_TRANSFORMATION_PLAN.md`
  - `Simulation/ARCHITECTURE_REFACTOR_CHECKLIST.md`
  - `Simulation/tools/architecture_metrics.sh` (line-budget/progress snapshot command)
- Full-game target blueprint (non-data + runtime systems to renderer-ready parity):
  - `Simulation/FULL_GAME_SIMULATION_BLUEPRINT.md`
- Roadmap and status:
  - `Simulation/IMPLEMENTATION_ROADMAP.md`
  - `Simulation/IMPROVEMENT_TRACKER.md`
- Current implementation snapshot:
  - `Simulation/CURRENT_STATE.md`
- Data authoring workflow:
  - `Simulation/DATA_AUTHORING_GUIDE.md`
- Contributor and agent rules:
  - `AGENTS.md` (repository-root canonical instruction file)

## Quick Run
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode controlled_champion
```
`vladimir` remains accepted as a compatibility alias for `controlled_champion`.

## License And Notices
- Code and original repository content: `AGPL-3.0-or-later` (`LICENSE`).
- Third-party notices: `THIRD_PARTY_NOTICES.md`.
- Contributor agreement: `CLA.md` and `CONTRIBUTING.md`.

## Disclaimer
- This project is not affiliated with or endorsed by Riot Games.
- League of Legends and Riot Games names, marks, and game IP belong to Riot Games, Inc.

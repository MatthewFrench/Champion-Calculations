# URF Controlled Champion Objective Simulator

This simulator targets controlled-champion URF teamfight optimization with champion-specific behavior delegated through script capabilities (current implemented controlled champion scripts: Vladimir and Sona). Controlled-champion run modes fail fast when script coverage is missing for the selected champion. For a fixed seed, it is deterministic and runs on a fixed server-tick loop (default 30 Hz) with an event queue for attacks, scripted champion actions, and survivability effects.

## Coverage Documentation Start Here
- Coverage status and complete versus incomplete sets:
  - `COVERAGE_GAPS.md`
- Exemplar-derived quality standards for adding coverage:
  - `COVERAGE_STANDARDS.md`
- Coverage completion and documentation gates:
  - `COVERAGE_CHECKLIST.md`
- Data authoring and provenance workflow:
  - `DATA_AUTHORING_GUIDE.md`
- Current implementation handoff snapshot:
  - `CURRENT_STATE.md`
- Full-game target blueprint (systems required beyond data coverage):
  - `FULL_GAME_SIMULATION_BLUEPRINT.md`
- Champion controller harness architecture (player/AI parity control contract):
  - `CHAMPION_CONTROLLER_HARNESS_ARCHITECTURE.md`
- Deterministic request/fast-forward runtime model (research-backed):
  - `DETERMINISTIC_REQUEST_AND_FAST_FORWARD_MODEL.md`

## What It Models
- Vladimir uses scripted `W`, `Q`, `E`, and `R` ability cadence.
- Controlled-champion run modes require a registered controlled-champion script and return an actionable error for unsupported champions.
- `src/world/*` owns deterministic map-bound world state registration/projections and validates encounter placement before scenario execution.
- Encounter world-state assembly now seeds baseline non-champion ecology anchors (`Structure`, `Monster`, `Minion`) with explicit allegiance ownership.
- Runtime enemy movement and enemy respawn position updates now route through world ownership upsert/clamp channels (map-bounded actor position ownership).
- Runtime world lifecycle channels now advance deterministic minion-wave spawn/despawn loops and neutral objective spawn/respawn timers through `src/world/world_actor_lifecycle_channels.rs`.
- Champion controller harness runtime ingress now routes deterministic actor-id keyed command handling through `src/champion_control_harness/*` + `src/engine/controlled_champion_controller_channels.rs` with sequence-ordered request execution, fixed tick-delay command application, command-owned movement stepping, and partial actor-symmetric opponent move/stop command channels.
- Combat runs with 2D positions (controlled champion fixed at origin; enemies maintain range with deterministic orbit/chase motion).
- Simulation intentionally ignores vertical `z` index for now; combat checks use only 2D geometry (`x`,`y`) until a verified gameplay interaction requires `z`.
- Fixed-timestep stepping via `ControlledChampionCombatSimulation.step()` at `server_tick_rate_hz`.
- URF global buffs (ability/item haste, health cost multiplier, attack speed multipliers) are applied.
- Enemy auto-attacks use start, windup, and hit phases.
- Ranged attacks/spells include projectile travel time based on distance and speed.
- Hit resolution is hitbox-aware:
  - actor hitboxes are modeled as circles
  - attack/spell/script effects carry configurable effect-hitbox radii
  - range checks include actor hitboxes plus effect hitbox reach
- Projectile block checks include projectile and barrier thickness.
- Projectile impacts now resolve with explicit outcomes:
  - blocked by projectile barriers
  - missed target hitbox at impact time
  - nullified by untargetable/stasis states
  - applied normally
- Melee auto-attacks are interrupted and cancelled if the attacker is stunned during windup (projectiles already released continue to resolve).
- Enemy auto-attacks are modeled as recurring timed events.
- Controlled champion auto-attacks are modeled as recurring start/windup/hit events with hitbox-aware impact checks.
- Enemy champion abilities and crowd control come from champion scripts and canonical champion data (no scenario combat proxies).
- Enemy units can die and respawn on URF-scaled timers (using each enemy actor's own level).
- Enemy transient stack/buff counters are cleared on death and respawn, and enemies respawn at their original spawn positions.
- Enemy scripted ability timelines are lifecycle-safe across death/respawn transitions.
- Opponent encounter parsing requires at least one positive scenario weight (all-zero-weight encounter sets are rejected).
- Guardian Angel, Zhonya's Hourglass, and Protoplasm Harness are modeled as survivability events.
- Champion/item/loadout mechanics can be extended through script hooks in `src/scripts/`.
- Shared loadout runtime scripts are applied during combat-time auto attacks, spell hits, kill events, and regeneration ticks for both controlled champion and enemies.
- Shared loadout runtime now exposes generic rune trigger hooks for:
  - on-hit events
  - ability-hit events
  - outgoing-damage healing resolution
  - immobilize-triggered effects
- Combat-time keystone coverage now includes Press the Attack, Fleet Footwork, Conqueror, and Aftershock in addition to previously modeled runtime runes.
- Combat-time keystone coverage also includes Electrocute, First Strike, and Phase Rush.
- Combat-time rune coverage now also includes Arcane Comet, Summon Aery, Hail of Blades, Dark Harvest, Triumph, Gathering Storm, and Second Wind.
- Controlled champion and enemy actors consume the same shared rune-combat runtime interfaces; controlled-champion runtime module now only owns defensive item/revive policy helpers.
- Engine event-resolution and trace owner channels now guard stale/out-of-range actor indices and skip invalid payloads instead of panicking.
- Blocking score cache lock/condvar poisoning now recovers via poisoned inner state instead of panicking.
- Required defaults channels now load through centralized strict hard-fail ownership in `src/defaults.rs`; required simulator/champion/mode defaults loaders no longer silently fall back to empty maps.
- Non-test `expect(...)` and `panic!(...)` callsites under `src/` are now zero.
- Startup now runs `preflight_required_defaults_channels()` before mode dispatch, surfacing typed contextual startup errors for required defaults ownership failures.
- Search scoring now also supports explicit unmodeled-item-effect quality gating (hard gate or per-item penalty) to reduce ranking bias from unimplemented item effects.
- When unmodeled hard gates are enabled, controlled-champion candidate generation space is constrained up front (modeled-rune loadout domain and modeled-runtime-item pool) so invalid candidates are not generated and then rejected later.
- Optional `simulation.combat_seed` applies deterministic combat variation (enemy initialization order + initial attack jitter) for robust repeated evaluation without nondeterminism.
- Full rune-proc telemetry collection is disabled for search-time scoring simulations and enabled explicitly for trace/report replay simulations.
- Rune telemetry runtime bookkeeping uses fixed-index counter arrays (no per-event hashmap lookup/allocation in hot paths).
- Rune-proc telemetry trigger/source accounting and telemetry-entry assembly now route through `src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs`.
- Runtime on-hit and ability bonus-damage resolution now routes through `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs` facade plus explicit owner leaves under `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/`.
- Runtime rune-proc trigger mutation helpers now route through `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/rune_proc_state_mutations.rs`.
- Runtime state initialization/reset ownership now routes through `src/scripts/runtime/loadout_runtime/runtime_state_initialization.rs`.
- Runtime mutation-effect ownership now routes through `src/scripts/runtime/loadout_runtime/runtime_effect_mutations.rs`.
- Runtime read-only projection ownership now routes through `src/scripts/runtime/loadout_runtime/runtime_stat_projections.rs`.
- Aftershock now models an active resist window that reduces incoming physical and magic damage while active.
- Defensive item activation and revive triggers are modeled through generic controlled champion runtime/item script capabilities (not champion-specific decision structs).
- Incoming damage/healing/revive ownership is routed through explicit event-resolution owner commands in `src/engine/event_resolution/incoming_damage_resolution.rs` (`apply_incoming_damage_*`, `apply_healing_*`, `apply_revive_or_mark_*`).
- Combat event-dispatch/step progression ownership now routes through `src/engine/event_resolution/combat_event_dispatch_resolution.rs` (`process_event`, `step`) with event-family resolution slices in `src/engine/event_resolution/combat_event_*_resolution.rs`.
- Controlled champion cast/defensive orchestration ownership now routes through `src/engine/event_resolution/controlled_champion_casting_resolution.rs`.
- Enemy script-action impact/followup scheduling ownership now routes through `src/engine/event_resolution/enemy_script_action_resolution.rs`.
- Enemy movement position updates are routed through explicit simulation-step owner command `apply_enemy_movement_step` in `src/engine/simulation_step/enemy_movement_step.rs`.
- Controlled champion hot-effect tick lifecycle ownership now routes through `src/engine/simulation_step/hot_effects_step.rs` (`apply_hot_effects`).
- World lifecycle advancement ownership now routes through `src/engine/simulation_step/world_lifecycle_step.rs` (`apply_world_lifecycle_step`).
- Controlled champion status/cast/attack gating, enemy range/targeting/projectile-block helpers, and attack/event scheduling ownership now route through `src/engine/combat_timing_and_targeting.rs`.
- Enemy derived combat-stat/loadout-runtime modeling ownership now routes through `src/engine/enemy_combat_stat_modeling.rs` (`derive_enemy_model`, `derive_enemy_combat_stats`).
- Enemy respawn/regeneration lifecycle and active/alive runtime queries are routed through actor-state owner facade `src/engine/actor_state/enemy_runtime_state.rs` plus explicit owner leaves under `src/engine/actor_state/enemy_runtime_state/`.
- Recurring script-event eligibility and script-cadence readiness updates are routed through actor-state owner APIs (`enemy_script_event_should_recur`, `set_enemy_script_event_ready_at`) in `src/engine/actor_state/enemy_runtime_state/enemy_lifecycle_channels.rs`.
- Champion-script epoch/readiness projections and script-runtime mutation are routed through actor-state owner APIs (`enemy_script_epoch_matches`, `enemy_script_event_ready_at_or_zero`, `execute_enemy_script_event_actions`, `enemy_aftershock_magic_damage_on_immobilize`) in explicit owner leaves under `src/engine/actor_state/enemy_runtime_state/`.
- High-traffic enemy read projections are routed through actor-state owner query APIs (`enemy_name`, `enemy_position`, `enemy_hitbox_radius`, `enemy_attack_*`, `enemy_target_health_snapshot_or_defaults`, `enemy_status_lines_at`, `enemy_is_*_at`) in `src/engine/actor_state/enemy_runtime_state/enemy_trace_snapshot_projections.rs`.
- Trace-snapshot enemy-section composition is routed through actor-state owner projection APIs (`enemy_count`, `enemy_trace_snapshot_at`) in `src/engine/actor_state/enemy_runtime_state/enemy_trace_snapshot_projections.rs`.
- Enemy auto-attack token lifecycle and next-hit bonus consume/reset are routed through actor-state owner APIs (`begin_enemy_attack_sequence`, `enemy_attack_sequence_matches`, `consume_enemy_attack_damage_with_on_hit`) in `src/engine/actor_state/enemy_runtime_state/enemy_attack_and_script_channels.rs`.
- Search candidate-space extraction is complete under `src/search/candidate_space/*`:
  - full-loadout candidate mutation/canonicalization helper ownership now routes through `src/search/candidate_space/full_loadout_candidate_operations.rs`
  - full-loadout candidate scoring/ranking helper ownership now routes through `src/search/candidate_space/full_loadout_candidate_scoring.rs`
  - item-only candidate mutation/crossover/parent-selection helper ownership now routes through `src/search/candidate_space/item_candidate_operations.rs`
  - item-only candidate scoring/dedupe helper ownership now routes through `src/search/candidate_space/item_candidate_scoring.rs`
- Full-loadout search orchestration ownership now routes through `src/search/full_loadout_search_orchestration.rs` facade plus explicit owner leaves under `src/search/full_loadout_search_orchestration/` (`strategy_dispatch.rs`, `seed_elite_generation.rs`, `adaptive_candidate_generation.rs`, `bleed_candidate_generation.rs`).
- Search strategy ownership now routes through `src/search/strategy/*`:
  - item-only strategy facade ownership routes through `src/search/strategy/item_candidate_search_strategies.rs` with explicit owner leaves under `src/search/strategy/item_candidate_search_strategies/`
  - full-loadout strategy helper ownership routes through `src/search/strategy/full_loadout_search_strategies.rs` facade with explicit owner leaves under `src/search/strategy/full_loadout_search_strategies/`
- Search scoring/diversity ownership now routes through `src/search/scoring/*`:
  - item-build scoring/diversity helper ownership routes through `src/search/scoring/item_build_scoring_and_diversity.rs`
  - full-loadout scoring/diversity helper ownership routes through `src/search/scoring/full_loadout_scoring_and_diversity.rs`
  - stat-key targeted item-build selection helper ownership routes through `src/search/scoring/stat_key_build_selection.rs`
  - item-name list formatting helper ownership routes through `src/search/scoring/item_name_list_formatting.rs`
- Scenario ownership extraction is complete and routes through dedicated owner modules:
  - controlled champion/search-default parse helpers now route through `src/scenario_runner/scenario_parsing.rs`
  - opponent encounter parse and legacy-key validation helpers now route through `src/scenario_runner/encounter_parsing.rs` with typed parse output (`ParsedOpponentEncounter`)
- Scenario run-output path/key ownership now routes through `src/scenario_runner/run_output_paths.rs` for runtime-stop key composition and report/trace path shaping.
- Scenario search-progress counter and unique-loadout helper ownership now routes through `src/scenario_runner/progress_reporting.rs`.
- Scenario strict-ranking heuristic ordering helper ownership now routes through `src/scenario_runner/strict_ranking_ordering.rs`.
- Scenario legal candidate-space estimation/probability formatting helper ownership now routes through `src/scenario_runner/search_space_estimation.rs`.
- Scenario controlled-champion runtime/search support helper ownership now routes through `src/scenario_runner/controlled_champion_search_runtime_support.rs` (coverage-asset locking, partial-candidate completion, telemetry/trace shaping, progress-state primitives).
- Scenario controlled-champion candidate-search phase orchestration now routes through `src/scenario_runner/controlled_champion_candidate_search.rs` with explicit phase-owner leaves (`coverage_stage_execution.rs`, `seed_and_strict_execution.rs`, `seed_and_strict_execution/*`).
- Scenario controlled-champion setup and enemy-build preparation ownership now routes through `src/scenario_runner/controlled_champion_scenario_setup.rs`.
- Scenario controlled-champion strict-ranking fallback/tie-break/seed-diagnostics finalization ownership now routes through `src/scenario_runner/controlled_champion_strict_ranking_finalization.rs`.
- Scenario controlled-champion post-search result-reporting orchestration now routes through `src/scenario_runner/controlled_champion_result_reporting.rs`.
- Scenario controlled-champion ranked-build analysis and diagnostics assembly now route through `src/scenario_runner/controlled_champion_result_build_analysis.rs` plus explicit projection leaves under `src/scenario_runner/controlled_champion_result_build_analysis/`.
- Scenario controlled-champion trace/report artifact writing now routes through `src/scenario_runner/controlled_champion_result_artifact_writing.rs`.
- Scenario fixed-loadout and fixed-loadout-rune-sweep execution entrypoint implementation now routes through `src/scenario_runner/fixed_loadout_runner.rs` and `src/scenario_runner/rune_sweep_runner.rs`, with shared enemy-scenario projection ownership in `src/scenario_runner/controlled_champion_enemy_scenario_projection.rs` and rune-sweep aggregation/report projection routed through explicit leaves under `src/scenario_runner/rune_sweep_runner/`.
- Scenario controlled-champion execution entrypoint now routes through `src/scenario_runner/controlled_champion_scenario_runner.rs` facade, `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs` owner leaf, and explicit execution sub-leaves under `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/`.
- Controlled champion script ownership now routes through `src/scripts/champions/controlled_champion.rs` facade plus explicit owner leaves under `src/scripts/champions/controlled_champion/` (contracts, registry, channels) and `src/scripts/champions/controlled_champion/vladimir_controlled_champion_script/` (model/capability/builder).
- Core combat-primitives/status/cast-lock ownership now routes through `src/core/combat_primitives_state.rs`.
- Defaults champion/item simulation-default loading now routes through a thin defaults-loader facade plus explicit leaf owners:
  - `src/defaults/champion_item_simulation_defaults_loader.rs`
  - `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs`
  - `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders/*.rs` (explicit champion-family leaves)
  - `src/defaults/champion_item_simulation_defaults_loader/item_simulation_defaults_loaders.rs`
  - `src/defaults/champion_item_simulation_defaults_loader/simulation_defaults_extraction_helpers.rs`
- Defaults schema/type ownership now routes through `src/defaults/simulator_defaults_schema_types.rs` facade plus explicit schema owner leaves under `src/defaults/simulator_defaults_schema_types/`, including explicit sub-leaves under `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema/`.
- Defaults path/key normalization and shared champion/item JSON effect helper ownership now routes through `src/defaults/defaults_path_key_and_effect_helpers.rs`.
- Data champion/item/preset loading and URF legality ownership now routes through `src/data/champion_item_preset_data_loading.rs` facade plus explicit owner leaves (including item-pool sub-leaves under `src/data/champion_item_preset_data_loading/item_pool_loading/`).
- Data simulation/search configuration parsing ownership now routes through `src/data/simulation_search_configuration_parsing.rs` facade plus explicit parse owner leaves under `src/data/simulation_search_configuration_parsing/`, including build-search parse/profile sub-leaves under `src/data/simulation_search_configuration_parsing/build_search_config_parsing/`.
- Data loadout-domain modeling/legality/sampling ownership now routes through `src/data/loadout_domain_modeling.rs` facade plus explicit owner leaves under `src/data/loadout_domain_modeling/`.
- Data loadout effect/stat resolution ownership now routes through `src/data/loadout_effect_resolution.rs`.
- Reporting markdown run-report rendering ownership now routes through `src/reporting/controlled_champion_report_markdown_writer.rs` facade plus explicit section owners under `src/reporting/controlled_champion_report_markdown_writer/` and `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/`.
- Reporting JSON run-report serialization ownership now routes through `src/reporting/controlled_champion_report_json_writer.rs`.
- Runtime cooldown/stack reporting ownership now routes through `src/scripts/runtime/loadout_runtime/runtime_state_reporting.rs`.
- Shared hook and enemy-script interfaces now use controlled champion terminology (no Vladimir-only cross-module field names).
- Runtime stat resolution is buff-aware and starts from canonical base data before applying state transforms:
  - cooldown metrics resolve through shared runtime stat queries (ability/item/neutral sources)
  - scalar combat metrics resolve through shared runtime stat queries (incoming damage taken, healing, movement speed, and outgoing bonus-ability damage)
  - modeled item cooldown passives (for example Heartsteel and Luden's Echo) load base cooldowns from canonical item effects data and then apply runtime haste/buff state
- Rune runtime tuning defaults are loaded from `data/simulator_defaults.json` under `rune_runtime_defaults` (global ownership).
- Controlled-champion combat sequencing decisions are delegated through a champion-script facade from engine (current implemented controlled champion scripts: Vladimir and Sona).
- Enemy champion script events are generated in scripts and applied by generic engine action handling.
- Foundational combat primitives are present for future fidelity work:
  - generic status effects (duration/stacks/persistence)
  - generic cast-lock windows (windup/channel/lockout)
- Controlled champion cast availability now respects active cast-lock windows, preventing same-tick multi-cast stacking from engine-side scheduling.
- Scripted enemy behavior profiles are included for:
  - Warwick
  - Vayne
  - Morgana
  - Sona
  - Doctor Mundo
- Build candidate scoring is parallelized across CPU cores (Rayon).
- Ensemble seed orchestration, portfolio strategy execution, and strategy-elite generation are also parallelized across CPU cores with deterministic merge ordering.
- Search uses simulation scoring during candidate generation (including partial candidates for strategy ranking) and strict full-simulation scoring for final candidate ranking.
- Full simulation scoring is memoized by canonical build key.
- In-flight dedupe cache avoids duplicate parallel re-simulation of the same canonical build.
- Ensemble seed runs are supported for confidence/robustness labeling.
- Cross-algorithm bleed round recombines elite candidates across strategies before final full ranking.
- Adaptive strategy allocation adds extra candidates from strategies that contribute more unique elites.
- Full-loadout `beam` and `greedy` strategies co-optimize loadout selection (runes/shards) during item expansion, not a single fixed loadout page.
- Fixed-seed reproducibility is preserved in adaptive/bleed candidate generation by sorting strategy keys before index-based seed derivation.
- Timed-out seed-stage partial candidates are deterministically completed to full candidates before strict full ranking, avoiding random fallback winners in short-budget runs.
- Strict final candidate ranking evaluates remaining candidates in parallel batches.
- Report metrics and build-order diagnostics resolve candidate loadout bonus stats from in-run simulation results and fallback recomputation as needed.
- Build-order optimization now evaluates stage outcomes across all configured opponent encounters using encounter weights and the same worst-case blend policy as objective scoring, instead of optimizing only against the first encounter.
- Build scoring uses a composite objective over:
  - time alive
  - damage dealt to enemies
  - healing done
  - enemy champions killed
  - invulnerable/untargetable seconds
  with configurable weights and per-scenario reference normalization.
  - invulnerable-seconds normalization is anchored to scenario horizon (time limit), preventing runaway objective inflation from near-permanent untargetable loops.
- Objective evaluation now supports selection-aware combat simulation so candidate loadout scoring includes combat-time runtime scripts.
- Default ownership is domain-based:
  - global simulator/search/engine defaults: `data/simulator_defaults.json`
  - champion AI controller policy defaults: `data/champion_ai_profiles.json`
  - game-mode simulation defaults: `../Game Mode/*.json`
  - champion simulation defaults: `../Characters/*.json`
  loaded through typed schema/helpers in `src/defaults.rs`.
- Ability execution geometry/routing overrides belong on canonical ability objects (`abilities.<ability_key>.execution`).
- Champion script blocks should only keep script-policy values that are not canonical gameplay ability data (for example followup priority).
- Script cast timing is controller-driven: enemies poll script abilities and cast when ready (cooldown-ready, alive, active, and not stunned), rather than using champion-file schedule constants.
- Controlled champion spell readiness now tracks by ability identity with runtime slot-to-ability mapping foundations for remap/swap support.

## Files
- `scenarios/`: Scenario catalog directory.
- `scenarios/vladimir_urf_teamfight.json`: Default URF team-fight scenario setup.
- `data/enemy_urf_presets.json`: Hardcoded URF enemy end-game presets with sources and check date.
- `data/simulator_defaults.json`: Global simulator/search/engine defaults.
  - controller ingress defaults include:
    - `engine_defaults.controlled_champion_controller_vision_radius`
    - `engine_defaults.controlled_champion_request_fixed_tick_delay`
- `data/champion_ai_profiles.json`: Champion AI controller policy (combat spacing, movement scaling, script polling, script-event priority overrides, and non-canonical cooldown overrides when canonical data is missing).
- `../Game Mode/URF.json`: URF mode data, including mode-specific simulation defaults (for example respawn tuning).
- `../Characters/<Champion>.json`: Champion canonical gameplay data, including per-ability execution fields (`abilities.<ability_key>.execution`) and ability/passive effect data used by scripts.
- `../Characters/ChampionDefaults.json`: Champion-style nested role defaults (`base_stats`, `basic_attack`, `abilities.execution_defaults`) used as fallback when champion files omit those canonical fields.
- `../AGENTS.md`: canonical repository-wide agent instructions and contributor rules.
- `CURRENT_STATE.md`: concise current-state handoff for developers and AI agents.
- `COVERAGE_GAPS.md`: tracked list of known game-fidelity and implementation-coverage gaps.
- `COVERAGE_STANDARDS.md`: exemplar-derived standards for champion/ability/item/rune/shard coverage.
- `COVERAGE_CHECKLIST.md`: contributor checklist for champion/item/rune/shard coverage work.
- `ARCHITECTURE_STANDARDS.md`: architecture standards for explicit naming, module boundaries, ownership channels, and refactor quality gates.
- `ARCHITECTURE_TRANSFORMATION_PLAN.md`: target architecture, phase plan, risks, metrics, and milestone status for architecture migration.
- `ARCHITECTURE_REFACTOR_CHECKLIST.md`: reusable checklist for architecture-focused pull requests.
- `DATA_AUTHORING_GUIDE.md`: canonical workflow for authoring champion/item/rune data and wiring runtime behavior.
- `IMPROVEMENT_TRACKER.md`: Done and pending improvements.
- `IMPLEMENTATION_ROADMAP.md`: roadmap status and planned phases.
- `Cargo.toml`: Rust package manifest.
- `src/main.rs`: Thin CLI orchestration entrypoint and mode dispatch.
- `src/simulation_contracts.rs`: Root contract facade/re-export surface for shared runtime/search/reporting/CLI contracts.
- `src/simulation_contracts/runtime_actor_contracts.rs`: Root contract owner leaf for stats, item, champion base, enemy config, simulation config, and URF buff contracts.
- `src/simulation_contracts/search_reporting_contracts.rs`: Root contract owner leaf for search diagnostics/objective/reporting contracts and candidate key/type aliases.
- `src/simulation_contracts/entrypoint_cli_contracts.rs`: Root contract owner leaf for CLI/mode/search-quality/options contracts.
- `src/core.rs`: Shared simulation math/helpers plus foundational generic combat primitives (status/cast-lock scaffolding).
- `src/core/combat_primitives_state.rs`: Core owner module for status-effect state, cast-lock state, and combat-primitives tick/update ownership.
- `src/data.rs`: Thin data facade for repository-path helpers plus explicit concern module exports.
- `src/defaults.rs`: Typed defaults facade and loader-access layer for global defaults plus domain-file champion/mode simulation defaults.
- `src/engine.rs`: Fixed-tick combat engine and event-queue simulation loop.
- `src/engine/combat_timing_and_targeting.rs`: Engine owner module for controlled champion action-gating/status windows, enemy range/targeting/projectile-block helpers, attack/event scheduling, and run-to-completion orchestration helpers.
- `src/engine/enemy_combat_stat_modeling.rs`: Engine owner module for enemy derived combat-stat and runtime-loadout modeling helpers.
- `src/engine/actor_state/enemy_runtime_state.rs`: Actor-state owner facade for enemy runtime channels.
- `src/engine/actor_state/enemy_runtime_state/enemy_trace_snapshot_projections.rs`: Actor-state owner leaf for enemy runtime query projections and trace snapshot composition.
- `src/engine/actor_state/enemy_runtime_state/enemy_attack_and_script_channels.rs`: Actor-state owner leaf for enemy auto-attack token/bonus and script-action execution channels.
- `src/engine/actor_state/enemy_runtime_state/enemy_lifecycle_channels.rs`: Actor-state owner leaf for enemy respawn/regeneration and script-cadence lifecycle channels.
- `src/engine/trace_snapshot_reporting.rs`: Engine trace/reporting facade for trace lifecycle and snapshot projection channels.
- `src/engine/trace_snapshot_reporting/trace_lifecycle_channels.rs`: Engine trace owner leaf for trace-event lifecycle (`enable_trace`, `trace_event`, `emit_trace_snapshots_due`).
- `src/engine/trace_snapshot_reporting/trace_status_and_projectile_projections.rs`: Engine trace owner leaf for status/cooldown/projectile read-only projections.
- `src/engine/trace_snapshot_reporting/trace_snapshot_summary_projection.rs`: Engine trace owner leaf for controlled champion + enemy + field snapshot summary assembly.
- `src/engine/event_queue/*`: Event queue owner modules for scheduling, ordering, and projection queries.
- `src/engine/event_resolution/combat_event_dispatch_resolution.rs`: Event-resolution owner module for combat event dispatch and tick-step progression.
- `src/engine/event_resolution/combat_event_enemy_auto_attack_resolution.rs`: Event-resolution owner module for enemy auto-attack start/windup/hit event handling.
- `src/engine/event_resolution/combat_event_controlled_champion_auto_attack_resolution.rs`: Event-resolution owner module for controlled champion auto-attack start/windup/hit event handling.
- `src/engine/event_resolution/combat_event_controlled_champion_offensive_ability_hit_resolution.rs`: Event-resolution owner module for controlled champion offensive primary/secondary/ultimate hit events.
- `src/engine/event_resolution/combat_event_champion_script_dispatch_resolution.rs`: Event-resolution owner module for champion-script event readiness/cooldown dispatch flow.
- `src/engine/event_resolution/controlled_champion_casting_resolution.rs`: Event-resolution owner module for controlled champion offensive/defensive cast orchestration.
- `src/engine/event_resolution/enemy_script_action_resolution.rs`: Event-resolution owner module for enemy script action impact/followup scheduling.
- `src/engine/event_resolution/incoming_damage_resolution.rs`: Event-resolution owner module for incoming damage, controlled champion healing, and controlled champion revive/death transitions.
- `src/engine/simulation_step/enemy_movement_step.rs`: Simulation-step owner module for enemy movement position update loops.
- `src/engine/simulation_step/hot_effects_step.rs`: Simulation-step owner module for controlled champion hot-effect tick lifecycle progression.
- `src/build_order.rs`: Build-order stage simulation and optimization.
- `src/search.rs`: Build search algorithms, portfolio/ensemble orchestration, diversity selection, and metric helpers.
- `src/search/full_loadout_search_orchestration.rs`: Search owner module for full-loadout strategy dispatch, ensemble-seed elite aggregation, adaptive strategy expansion, and bleed-candidate generation.
- `src/search/strategy/item_candidate_search_strategies.rs`: Search strategy facade for item-only strategy dispatch.
- `src/search/strategy/item_candidate_search_strategies/beam_search_strategy.rs`: Search strategy owner leaf for beam-search candidate expansion/ranking.
- `src/search/strategy/item_candidate_search_strategies/iterative_search_strategies.rs`: Search strategy owner leaf for random/hill-climb/genetic/annealing item-only strategies.
- `src/search/strategy/item_candidate_search_strategies/mcts_search_strategy.rs`: Search strategy owner leaf for item-only MCTS rollout/search channels.
- `src/search/strategy/full_loadout_search_strategies.rs`: Search strategy owner module for full-loadout strategy implementations.
- `src/reporting.rs`: Thin reporting facade with shared formatting/validation helpers and writer re-exports.
- `src/scenario_runner.rs`: Scenario mode execution orchestration (`controlled_champion`, `controlled_champion_step`, stat modes).
- `src/scenario_runner/scenario_parsing.rs`: Scenario parser-owner module for controlled champion/search-default parsing and legacy-key validation.
- `src/scenario_runner/encounter_parsing.rs`: Encounter parser-owner module for opponent encounter parsing and typed parse outputs (`ParsedOpponentEncounter`).
- `src/scenario_runner/run_output_paths.rs`: Scenario run-output owner module for output directory composition, runtime-stop key formatting, and repository-relative path rendering.
- `src/scenario_runner/progress_reporting.rs`: Scenario progress owner module for search-type runtime counters and unique-loadout diagnostics helpers.
- `src/scenario_runner/strict_ranking_ordering.rs`: Scenario strict-ranking owner module for heuristic ordering and deterministic promotion control.
- `src/scenario_runner/search_space_estimation.rs`: Scenario diagnostics owner module for legal candidate-space estimation and closeness-probability/percent formatting helpers.
- `src/scenario_runner/fixed_loadout_runner.rs`: Scenario execution owner module for fixed-loadout run/report/trace flow.
- `src/scenario_runner/rune_sweep_runner.rs`: Scenario execution owner module for fixed-loadout keystone sweep flow.
- `src/scenario_runner/controlled_champion_enemy_scenario_projection.rs`: Shared scenario owner module for scaled enemy-scenario parsing, enemy-build projection, and reference-outcome projection reused by fixed-loadout and rune-sweep flows.
- `src/scenario_runner/rune_sweep_runner/result_aggregation.rs`: Scenario execution owner leaf for read-only rune-sweep outcome/objective aggregation.
- `src/scenario_runner/rune_sweep_runner/report_writing.rs`: Scenario execution owner leaf for fixed-loadout rune-sweep markdown/json report projection and artifact writing.
- `src/scenario_runner/controlled_champion_candidate_search.rs`: Scenario execution owner module for controlled-champion candidate-search phase orchestration (coverage stage, seed ensembles, candidate merge/dedupe, strict ranking).
- `src/scenario_runner/controlled_champion_scenario_setup.rs`: Scenario execution owner module for controlled-champion setup/search-configuration parsing and enemy-build preparation.
- `src/scenario_runner/controlled_champion_strict_ranking_finalization.rs`: Scenario execution owner module for controlled-champion strict-ranking fallback insertion, tie-break sorting, and seed-hit diagnostics finalization.
- `src/scenario_runner/controlled_champion_result_reporting.rs`: Scenario execution owner module for controlled-champion post-search result-reporting orchestration.
- `src/scenario_runner/controlled_champion_result_build_analysis.rs`: Scenario execution owner facade module for controlled-champion ranked-build analysis, diagnostics assembly, and build-order analysis orchestration.
- `src/scenario_runner/controlled_champion_result_build_analysis/build_order_analysis.rs`: Scenario execution owner leaf for build-order optimization and final controlled-champion end-state projection.
- `src/scenario_runner/controlled_champion_result_build_analysis/candidate_metrics_projection.rs`: Scenario execution owner leaf for diverse-top projection, candidate metrics, pareto-front projection, and confidence/channel input selection.
- `src/scenario_runner/controlled_champion_result_build_analysis/search_diagnostics_projection.rs`: Scenario execution owner leaf for search diagnostics payload projection.
- `src/scenario_runner/controlled_champion_result_artifact_writing.rs`: Scenario execution owner module for controlled-champion trace/report artifact writing and final output emission.
- `src/scenario_runner/controlled_champion_scenario_runner.rs`: Scenario execution facade for controlled-champion coverage/search/report entrypoint routing.
- `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution.rs`: Scenario execution owner facade leaf for controlled-champion coverage/search/report orchestration flow.
- `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/deadline_and_progress.rs`: Scenario execution owner leaf for time-budget deadline and significant-improvement progress-window ownership.
- `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/runtime_setup.rs`: Scenario execution owner leaf for controlled-champion scenario search/setup and enemy-build setup ownership.
- `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/search_execution.rs`: Scenario execution owner leaf for coverage/seed/strict execution orchestration.
- `src/scenario_runner/controlled_champion_scenario_runner/controlled_champion_scenario_execution/search_execution/candidate_scoring_channels.rs`: Scenario execution owner leaf for candidate score/evaluation channels used by search execution.
- `src/scenario_runner/controlled_champion_search_runtime_support.rs`: Scenario support owner module for controlled-champion coverage-asset locking, partial-candidate completion, telemetry formatting, and structured trace-event shaping.
- `src/defaults/champion_item_simulation_defaults_loader.rs`: Thin defaults owner facade/re-export surface for champion/item simulation-default loaders.
- `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders.rs`: Defaults owner leaf module for champion simulation-default loading.
- `src/defaults/champion_item_simulation_defaults_loader/champion_simulation_defaults_loaders/*.rs`: Defaults owner leaves for explicit champion-family simulation-default loading.
- `src/defaults/champion_item_simulation_defaults_loader/item_simulation_defaults_loaders.rs`: Defaults owner leaf module for item simulation-default loading.
- `src/defaults/champion_item_simulation_defaults_loader/simulation_defaults_extraction_helpers.rs`: Defaults owner leaf module for shared champion/item effect/ability extraction helpers.
- `src/defaults/champion_simulation_data_loading.rs`: Defaults owner leaf module for champion-simulation profile loading, champion slot-binding derivation, champion AI profile normalization, and URF respawn-default loading.
- `src/defaults/simulator_defaults_schema_types.rs`: Defaults schema facade/re-export surface for simulator/champion/mode schema type declarations.
- `src/defaults/simulator_defaults_schema_types/simulation_search_and_engine_defaults_schema.rs`: Defaults schema owner leaf for simulation/search/engine defaults schema.
- `src/defaults/simulator_defaults_schema_types/rune_runtime_defaults_schema.rs`: Defaults schema owner leaf for rune runtime defaults schema.
- `src/defaults/simulator_defaults_schema_types/champion_ai_and_execution_schema.rs`: Defaults schema owner leaf for champion AI and ability execution schema.
- `src/defaults/simulator_defaults_schema_types/champion_behavior_and_ability_defaults_schema.rs`: Defaults schema owner leaf for champion behavior and champion/item ability defaults schema.
- `src/defaults/simulator_defaults_schema_types/champion_file_defaults_schema.rs`: Defaults schema owner leaf for champion defaults file and URF respawn schema.
- `src/defaults/defaults_path_key_and_effect_helpers.rs`: Defaults owner module for key normalization, repository-path resolution, and shared item/champion JSON effect helper loading.
- `src/data/champion_item_preset_data_loading.rs`: Thin data facade/re-export surface for champion/item/preset loading and URF legality validation owner leaves.
- `src/data/champion_item_preset_data_loading/champion_base_loading.rs`: Data owner leaf for champion-base loading and normalized champion lookup.
- `src/data/champion_item_preset_data_loading/item_pool_loading.rs`: Data owner leaf for item loading, stat extraction, and default URF item-pool filtering.
- `src/data/champion_item_preset_data_loading/urf_mode_loading.rs`: Data owner leaf for URF mode buff and allowed-item loading.
- `src/data/champion_item_preset_data_loading/enemy_preset_loading.rs`: Data owner leaf for enemy preset path/loading/validation and preset loadout projection.
- `src/data/simulation_search_configuration_parsing.rs`: Thin data facade/re-export surface for simulation/search configuration parsing owner leaves.
- `src/data/simulation_search_configuration_parsing/shared_parsing_primitives.rs`: Data parse owner leaf for shared config parse primitives (`as_str`, stack-override parsing).
- `src/data/simulation_search_configuration_parsing/simulation_config_parsing.rs`: Data parse owner leaf for simulation config parsing/default resolution/legacy-key validation.
- `src/data/simulation_search_configuration_parsing/enemy_config_parsing.rs`: Data parse owner leaf for opponent actor parse/placement/stack-override normalization.
- `src/data/simulation_search_configuration_parsing/build_search_config_parsing.rs`: Data parse owner leaf for build-search parse mapping and quality-profile application.
- `src/data/simulation_search_configuration_parsing/loadout_selection_parsing.rs`: Data parse owner leaf for loadout rune/shard selection parsing and deterministic key projection.
- `src/data/loadout_domain_modeling.rs`: Data owner facade for rune-page domain generation, legality checks, and randomized selection channels.
- `src/data/loadout_domain_modeling/loadout_domain_schema.rs`: Data owner leaf for rune/shard domain schema construction.
- `src/data/loadout_domain_modeling/modeled_rune_filtering.rs`: Data owner leaf for modeled-rune domain filtering.
- `src/data/loadout_domain_modeling/rune_page_validation.rs`: Data owner leaf for rune-page legality validation.
- `src/data/loadout_domain_modeling/loadout_selection_generation.rs`: Data owner leaf for deterministic/default/random loadout selection generation.
- `src/data/loadout_effect_resolution.rs`: Data owner module for structured effect application and resolved loadout stat derivation.
- `src/reporting/controlled_champion_report_markdown_writer.rs`: Reporting owner facade for controlled-champion markdown run-report assembly/writing.
- `src/reporting/controlled_champion_report_markdown_writer/header_and_objective_sections.rs`: Reporting owner leaf for header/headline/objective-breakdown and rune telemetry markdown sections.
- `src/reporting/controlled_champion_report_markdown_writer/search_diagnostics_section.rs`: Reporting owner leaf for search diagnostics markdown section projection.
- `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections.rs`: Reporting facade for loadout/build markdown section routing.
- `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/build_ranking_sections.rs`: Reporting owner leaf for diverse-top/ranking/build-order/deeper-insights markdown sections.
- `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/enemy_profile_sections.rs`: Reporting owner leaf for enemy-build and enemy-derived-profile markdown sections.
- `src/reporting/controlled_champion_report_markdown_writer/loadout_and_build_sections/loadout_profile_sections.rs`: Reporting owner leaf for base/loadout/best-build/end-stats/stack-override markdown sections.
- `src/reporting/controlled_champion_report_json_writer.rs`: Reporting owner module for controlled-champion JSON report serialization and rune telemetry encoding.
- `tools/architecture_metrics.sh`: Architecture line-budget/progress metrics helper for tracked facade files.
- `src/cache.rs`: In-memory score cache implementation (per-run only).
- `src/status.rs`: Deadline and status progress reporting helpers.
- `src/respawn.rs`: URF respawn timer model helpers.
- `src/scripts/champions.rs`: Champion script facade/re-export surface for behavior, event/action types, and runtime effect channels.
- `src/scripts/champions/champion_behavior_profile_channels.rs`: Champion behavior-profile owner leaf.
- `src/scripts/champions/champion_script_effect_types.rs`: Champion script event/action/payload type owner leaf.
- `src/scripts/champions/champion_script_event_channels.rs`: Champion script event registry/cooldown/dispatch owner leaf.
- `src/scripts/champions/runtime_effect_channels.rs`: Champion runtime effect bridge owner leaf.
- `src/scripts/champions/vladimir.rs`: Vladimir scripted formulas and combat decision APIs (offense and defensive ability decisions).
- `src/scripts/champions/<champion>.rs`: Per-champion behavior/event logic modules.
- `src/scripts/items/hooks.rs`: Item-specific simulation scripts (for example, Heartsteel stack override handling).
- `src/scripts/runes/effects.rs`: Dynamic-runtime rune classification list for loadout/runtime diagnostics.
- `src/scripts/runtime/controlled_champion_loadout.rs`: Controlled champion defensive item/revive decision helpers plus loadout hook implementation.
- `src/scripts/runtime/loadout_runtime.rs`: Shared combat-time loadout runtime state and effect helpers.
- `src/scripts/runtime/loadout_runtime/rune_proc_telemetry.rs`: Runtime owner leaf module for rune-proc telemetry counters, trigger-source tracking, and telemetry entry assembly.
- `src/scripts/runtime/loadout_runtime/combat_bonus_resolution.rs`: Runtime owner leaf module for on-hit/ability bonus-damage resolution and rune-trigger execution flow.
- `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/rune_proc_state_mutations.rs`: Runtime owner leaf module for rune-proc stack/cooldown mutation channels used by combat bonus resolution.
- `src/scripts/runtime/loadout_runtime/combat_bonus_resolution/projection_helpers.rs`: Runtime owner leaf module for read-only combat bonus projections (for example Press the Attack and Gathering Storm projections).
- `src/scripts/runtime/loadout_runtime/runtime_state_initialization.rs`: Runtime owner leaf module for runtime flag/cooldown initialization and transient reset ownership.
- `src/scripts/runtime/loadout_runtime/runtime_effect_mutations.rs`: Runtime owner leaf module for outgoing-heal/enemy-kill/immobilize mutation-effect channels.
- `src/scripts/runtime/loadout_runtime/runtime_stat_projections.rs`: Runtime owner leaf module for read-only runtime projections (attack speed, incoming multipliers, movement speed, regeneration).
- `src/scripts/runtime/loadout_runtime/runtime_state_reporting.rs`: Runtime owner leaf module for cooldown/stack status projection helpers.
- `src/scripts/runtime/stat_resolution.rs`: Shared runtime stat-query resolver for buff-aware metric transformations (cooldowns plus scalar combat metrics from base data + runtime buff state).
- `src/scripts/registry/hooks.rs`: Script hook interfaces, contexts, and dispatch registry.

## Run
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode controlled_champion
```
- `controlled_champion` mode writes a markdown report to:
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_run_report.md`
- `controlled_champion` mode writes a structured JSON report to:
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_run_report.json`
  where `<controlled_champion_key>` is the normalized champion name (for example: `vladimir`).
  where `<search_quality_profile>` is one of `fast`, `balanced`, `maximum_quality`.
  where `<runtime_budget>` is:
  - `no_hard_cap` when no runtime budget is set.
  - `<seconds>s` when only fixed budget is used (for example `300s`).
  - `<budget>__popcorn_<window>__min_improvement_<relative_percent>pct` when popcorn mode is enabled and `window != budget`.
  - `<budget>__popcorn__min_improvement_<relative_percent>pct` when popcorn mode is enabled and `window == budget`.
- All generated JSON artifacts (run reports, fixed-loadout reports, rune sweep reports, and traces) include a top-level `schema_version`.
- Report includes:
  - Human-readable generation timestamps (local and UTC)
  - Vladimir base stats at configured level (`controlled_champion.level` override, fallback `simulation.champion_level`, default `20`)
  - Vladimir end stats for best build
  - Stack override notes for stack-based items in the best build
  - Enemy derived combat profiles (HP/AD/AS/range/hit/burst stats) with similarity warnings for suspiciously close auto profiles
- Detailed search diagnostics including:
    - effective search seed used
    - effective thread count and parallelism-mode flags (seed orchestration, portfolio, strategy elites)
    - coverage-stage diagnostics for `maximum_quality` (elapsed, assets covered, seeded candidates)
    - explicit degraded-coverage warning/flag when coverage stage is incomplete
    - explicit simulation counts (new full simulations, unique scored candidates, total score requests)
    - search elapsed time and total run time (end-to-end)
    - in-memory cache hits/misses/waits
    - unique scored candidates across all search stages
    - per-search-type breakdown (requests/new simulations)
    - generated/unique/pruned candidate counts
    - strict-stage completion percentage and timeout-skipped candidate count
    - unmodeled-rune gate policy/counters (hard gate flag, per-rune penalty, rejected/penalized candidates)
    - unmodeled-item-effect gate policy/counters (hard gate flag, per-item penalty, rejected/penalized candidates)
    - estimated legal candidate-space size and coverage percentages
    - heuristic closeness-to-optimal probability estimate with explicit assumptions
  - If run with a time budget, report includes timeout and completion metadata
- Trace outputs are also champion-keyed:
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_event_trace.md`
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_event_trace.json`
	  - Trace JSON schema:
	    - `schema_version`: integer schema version for trace consumers
	    - `event_encoding`: currently `structured`
	    - `rune_proc_telemetry[]`: rune proc totals plus calibration metrics (`proc_count`, `attempt_count`, `eligible_count`, `proc_attempt_rate`, `proc_eligible_rate`, `bonus_damage`, `bonus_damage_share`, `bonus_healing`, `bonus_healing_share`) plus `source_breakdown[]` (compatibility aliases `opportunity_count`/`proc_opportunity_rate` remain mapped to eligible metrics)
	    - `events[]`: objects with `timestamp_seconds`, `event_type`, `details`, and `raw`
  - trace includes explicit impact outcome events such as `projectile_blocked`, `impact_nullified`, `attack_missed`, and `ability_missed`.
- Compatibility aliases:
  - `--mode vladimir` maps to `--mode controlled_champion`.
  - `--mode vladimir_step` maps to `--mode controlled_champion_step`.
- `controlled_champion_fixed_loadout` mode:
  - evaluates one explicit controlled champion item/rune/shard loadout directly (no candidate search/mutation).
  - required: `--fixed-item-names "<comma-separated six items>"`.
  - `search` block in scenario is optional for this mode; when omitted, search defaults are used only for objective weighting/seed defaults.
  - optional overrides: `--fixed-rune-names`, `--fixed-shard-stats`, `--fixed-eval-label`.
  - writes outputs to:
    - `Simulation/output/runs/controlled_champion/fixed_loadout/<search_quality_profile>/<fixed_eval_label_key>/vladimir_fixed_loadout_report.md`
    - `Simulation/output/runs/controlled_champion/fixed_loadout/<search_quality_profile>/<fixed_eval_label_key>/vladimir_fixed_loadout_report.json`
    - `Simulation/output/runs/controlled_champion/fixed_loadout/<search_quality_profile>/<fixed_eval_label_key>/vladimir_fixed_loadout_trace.md`
    - `Simulation/output/runs/controlled_champion/fixed_loadout/<search_quality_profile>/<fixed_eval_label_key>/vladimir_fixed_loadout_trace.json`
- `controlled_champion_fixed_loadout_rune_sweep` mode:
  - evaluates one fixed item build while sweeping all legal keystones in the same primary rune path as the baseline keystone.
  - required: `--fixed-item-names "<comma-separated six items>"`.
  - `search` block in scenario is optional for this mode; when omitted, search defaults are used only for objective weighting/seed defaults.
  - optional overrides: `--fixed-rune-names`, `--fixed-shard-stats`, `--fixed-eval-label`, `--fixed-sweep-seed-repeats`.
  - writes outputs to:
    - `Simulation/output/runs/controlled_champion/fixed_loadout_rune_sweep/<search_quality_profile>/<fixed_eval_label_key>/vladimir_fixed_loadout_rune_sweep_report.md`
    - `Simulation/output/runs/controlled_champion/fixed_loadout_rune_sweep/<search_quality_profile>/<fixed_eval_label_key>/vladimir_fixed_loadout_rune_sweep_report.json`

## Runtime Controls
- `--max-runtime-seconds N`:
  - Stops timed search after `N` seconds of simulation-budgeted search work and reports best-so-far findings.
  - Budget clock arms on the first timed-phase simulation evaluation (not during setup/report generation).
  - In `maximum_quality`, coverage stage remains pre-budget and the timer can only arm after coverage when timed-phase simulations begin.
- `--popcorn-window-seconds W`:
  - Enables progress-window stopping ("microwave popcorn mode").
  - Search continues while significant improvements keep happening; run stops when no significant improvement is observed for `W` seconds.
  - Popcorn mode always uses the `maximum_quality` search profile, regardless of `--search-quality-profile`.
  - In `maximum_quality`, popcorn early-stop checks are deferred until after the pre-budget coverage stage completes.
  - Can be combined with `--max-runtime-seconds`:
    - stop condition is whichever comes first.
- `--popcorn-min-relative-improvement-percent R`:
  - Relative objective-score delta required to count as significant progress.
  - Significant threshold is `R%` of the last best score.
- `--status-every-seconds N`:
  - Prints periodic status lines (phase, elapsed, progress, best score) while searching.
- `--search-quality-profile {fast|balanced|maximum_quality}`:
  - Applies opinionated search settings. Default is `maximum_quality`.
  - Also applies unmodeled coverage policy:
    - `maximum_quality`: hard gate (reject candidates with unmodeled runes)
    - `maximum_quality`: hard gate (reject candidates with unmodeled item effects)
    - `fast`/`balanced`: per-rune and per-item score-penalty mode
- `--seed N`:
  - Overrides runtime seed with deterministic value `N`.
  - If not provided, runtime seed is random unless the scenario explicitly sets `search.seed`.
- `simulation.combat_seed` (scenario key):
  - Optional deterministic combat-variation seed for enemy initialization order and initial auto-attack jitter.
  - `controlled_champion_fixed_loadout_rune_sweep --fixed-sweep-seed-repeats` now derives unique combat seeds per repeat from this run's effective search seed and keystone identity.

## Continuous Integration and Release
- Repository workflows are defined under:
  - `.github/workflows/continuous-integration.yml`
  - `.github/workflows/release.yml`
- Continuous integration runs on pull requests and pushes to `main`:
  - formatting check
  - clippy lint with denied warnings
  - tests
  - release build
  - smoke simulation run
  - upload of generated findings report artifacts
- Release workflow runs on version tags (`v*`):
  - builds release binary
  - generates simulation findings report
  - publishes release with:
    - binary artifact
    - markdown findings report
    - structured JSON findings report
  - release description includes extracted findings/report sections.

## Threading
- The Rust optimizer leaves one core free by default (`available_cores - 1`, minimum 1 thread).
- Override thread count with `--threads N` if needed.
- Current parallel execution includes:
  - candidate scoring batches
  - ensemble seed orchestration
  - portfolio strategy execution
  - strategy-elite/adaptive candidate generation
- You can cap threads with:
```bash
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode controlled_champion \
  --threads 8
```

## Diverse Top Builds
- `controlled_champion` mode can output top diverse builds near the best score:
  - `--top-x` number of diverse builds to keep (default `8`)
  - `--min-item-diff` minimum symmetric item difference between selected builds (default `2`)
  - `--max-relative-gap-percent` max score drop from best to still be considered (default `5.0`)
  - `--report-path` optional custom report output path
- After top builds are selected, simulator also optimizes full-item build order:
  - Uses beam plus optimistic bound pruning over order states (no partial/intermediate items).
  - Uses stage levels evenly spaced from 5 to 20 across item slots.
  - Scores each order by cumulative stage objective across stages:
    - time alive
    - damage dealt
    - healing done

## Taric (Max Attack Speed)
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode taric_as
```

## Hecarim (Max Move Speed)
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode hecarim_ms
```

## Controlled Champion Step Debug (Tick-by-Tick)
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode controlled_champion_step \
  --ticks 60
```
- Step mode uses the first entry in `opponents.encounters` and prints the selected encounter name.
- Compatibility alias: `--mode vladimir_step` is still accepted.
- `--scenario` accepts either:
  - a full/relative file path, or
  - a scenario name resolved from `Simulation/scenarios/<name>.json`.

## Extensibility
- Champion/item mechanics should be added in dedicated modules (for example under `src/scripts/`) rather than growing `main.rs`.
- Shared root contracts should be added under `src/simulation_contracts/*` rather than as inline type declarations in `main.rs`.
- Scenario JSON should stay minimal and reference canonical data from `Characters`, `Items`, and `Game Mode`.
- Opponent actors no longer accept `combat` proxy blocks; use champion scripts/data only.
- Opponent groups no longer accept `opponents.uptime_windows_enabled`; combat windows are script/runtime driven.
- For contributor workflow and completion criteria:
  - data authoring workflow: `DATA_AUTHORING_GUIDE.md`
  - coverage done criteria: `COVERAGE_CHECKLIST.md`
- Architecture direction:
  - shared simulation/core/search/reporting modules should remain champion-agnostic.
  - champion and loadout specifics should be delegated through script interfaces.
  - avoid abbreviations in new names and user-facing output.
  - ability ownership should be slot-agnostic:
    - ability behavior identity must not be hardcoded to a specific key binding (`Q`,`W`,`E`,`R`) or champion type.
    - key bindings should map to ability instances via data and runtime state.
    - runtime should support ability remapping/swapping across champions (for example stolen abilities) without changing core engine code.
    - champion-specific exceptions should be implemented in ability scripts/data, not in shared engine branches.
    - default slot bindings should be derived from canonical ability data (`abilities.<ability>.slot` / `default_keybinding`) in champion files, not from separate top-level mapping blocks or global defaults.

## Current Script Structure
The simulator now uses a domain-first script layout to keep champion/item/rune behavior organized:

```text
src/scripts/
  champions.rs
  items.rs
  registry.rs
  runes.rs
  runtime.rs
  champions/
    controlled_champion.rs
    doctor_mundo.rs
    morgana.rs
    sona.rs
    vayne.rs
    vladimir.rs
    warwick.rs
  items/
    hooks.rs
  runes/
    effects.rs
  runtime/
    controlled_champion_loadout.rs
    loadout_runtime.rs
    loadout_runtime/
      combat_bonus_resolution.rs
      combat_bonus_resolution/
        projection_helpers.rs
        rune_proc_state_mutations.rs
      rune_proc_telemetry.rs
      runtime_effect_mutations.rs
      runtime_state_initialization.rs
      runtime_state_reporting.rs
      runtime_stat_projections.rs
  registry/
    hooks.rs
```
Script-tree `mod.rs` cleanup and compatibility-shim removal are complete; architecture follow-up is now optional deeper leaf decomposition for remaining dense non-facade leaves and continued regression-coverage hardening.

## Minimal Scenario Shape
- Use champion references instead of hardcoding base stats:
  - `controlled_champion.champion`: champion name from `Characters/`.
  - `opponents.encounters[].actors[].champion`: champion name from `Characters/`.
- Required structure:
  - `controlled_champion` with at least `champion`.
  - `opponents.encounters[]`: weighted encounter list used by objective aggregation.
- Optional reference fields:
  - `controlled_champion.loadout`: optional controlled champion loadout block (not used as the optimization seed).
- Deprecated/removed:
  - `controlled_champion.baseline_items` is no longer supported.
  - `loadout.runes_reforged.rune_ids` is no longer supported (use ordered `loadout.runes_reforged.rune_names`).
  - `loadout.season2016_masteries` is no longer supported.
- Keep only scenario setup data in scenario JSON (for example actor placement, per-actor level, and stack overrides).
- Legacy flat scenario keys are removed; scenario parsing is now strict and canonical.
- Build search item pool is restricted to purchasable `LEGENDARY` items only.
- Pre-evolution items are normalized to evolved forms in simulation lookups:
  - `Manamune` -> `Muramana`
  - `Archangel's Staff` -> `Seraph's Embrace`
- Mode availability note:
  - Item JSON does not currently expose an explicit `available_in_modes` field.
  - As a practical URF-safe rule, search excludes Arena/distributed-only item patterns and focuses on normal-rift-eligible legendary items.

## Notes
- Future-proofing requirement:
  - the simulator must support champions whose available abilities can change at runtime (for example ability theft/copy mechanics).
  - this implies a separation between:
    - input slot (key binding / cast slot)
    - ability identity (script/data behavior)
    - ability owner/source champion context
  - current Vladimir-centric cast profile is an intermediate shape and should migrate toward this model.
- Champion base stats are loaded from `Characters/*.json` by champion name.
- This model now includes first-pass scripted Vladimir offensive abilities (`Q`, `E`, `R`) and enemy death/respawn handling.
- The build search supports: `beam`, `greedy`, `random`, `hill_climb`, `genetic`, `simulated_annealing`, `mcts`, and `portfolio`.
- Default scenario uses `portfolio`, which runs multiple algorithms in parallel and merges candidates.
- Useful knobs in `search`:
  - `portfolio_strategies`
  - `hill_climb_restarts`, `hill_climb_steps`, `hill_climb_neighbors`
  - `genetic_population`, `genetic_generations`, `genetic_mutation_rate`, `genetic_crossover_rate`
  - `simulated_annealing_restarts`, `simulated_annealing_iterations`, `simulated_annealing_initial_temp`, `simulated_annealing_cooling_rate`
  - `mcts_iterations`, `mcts_rollouts_per_expansion`, `mcts_exploration`
  - `ensemble_seeds`, `ensemble_seed_stride`, `ensemble_seed_top_k`
  - `objective_survival_weight`, `objective_damage_weight`, `objective_healing_weight`, `objective_enemy_kills_weight`, `objective_invulnerable_seconds_weight`
  - `robust_min_seed_hit_rate`
  - `bleed_enabled`, `bleed_budget`, `bleed_mutation_rate`
  - `multi_scenario_worst_weight` (aggregation between weighted-mean and worst-case when using multiple enemy scenarios)
  - `strict_ranking_enable_heuristic_ordering`, `strict_ranking_rune_signal_weight`, `strict_ranking_shard_signal_weight`, `strict_ranking_exploration_promotions`
  - `ranked_limit`
  - `seed` (optional; if omitted or `0`, runtime random seed is used)
- Loadout search legality:
  - Rune pages are generated from legal primary/secondary slot rules in `RunesReforged.json`.
  - Shards are generated from legal `stat_shards` slot options.
  - Runtime validation rejects illegal pages before simulation (invalid path/slot/shard structure).
  - Loadout optimization is always on for controlled champion build scoring; there is no scenario shortlist/sample knob.
- Enemy presets:
  - `controlled_champion` mode uses `data/enemy_urf_presets.json` for enemy full builds and rune pages/shards.
  - Startup validation fails fast if a preset references missing item/rune/shard data.
- Default scenario is tuned for high search quality (deeper exploration and more seed stability), so expect higher CPU time than previous presets.
- `maximum_quality` runs a pre-budget coverage stage that locks each item/rune/shard at least once, keeps top diverse candidates per locked asset, and injects those seeds into the main search.
- Stack overrides (generic, keyed by stack identifier):
  - global defaults are loaded from `Simulation/data/simulator_defaults.json` (`simulation_defaults.stack_overrides`).
  - current default baseline includes `heartsteel: 20.0` unless overridden below.
  - `simulation.stack_overrides` sets global stack overrides by stack identifier (example: `{ "heartsteel": 20.0 }`).
  - `controlled_champion.stack_overrides` overrides global stack overrides for the controlled champion.
  - `opponents.stack_overrides` sets default opponent overrides, and each actor can further override with `opponents.encounters[].actors[].stack_overrides`.
  - In build-order optimization, stack overrides are distributed by item acquisition level and current stage level (buying later yields fewer stacks by level 20).
- Level defaults:
  - `simulation.champion_level` is the fallback level.
  - `controlled_champion.level` overrides controlled champion level.
  - `opponents.default_level` overrides fallback for opponent actors, and `opponents.encounters[].actors[].level` overrides per actor.
  - If `controlled_champion.level` overrides the fallback level, Protoplasm level-scaled default values are recalculated to match the effective controlled champion level unless explicitly set in `simulation`.
- Scenario simulation knobs are now minimal by default:
  - optional overrides:
    - `simulation.time_limit_seconds` (default from `Simulation/data/simulator_defaults.json`; default value `1200` seconds)
    - hard cap: `<= 1200` seconds (20 minutes)
    - `simulation.server_tick_rate_hz`
    - `simulation.champion_level`
    - `simulation.stack_overrides`
    - `simulation.protoplasm_trigger_health_percent`
- Fallback ownership by domain:
  - URF respawn defaults load from `../Game Mode/URF.json` `respawn`.
  - Vladimir Sanguine Pool defaults load from `../Characters/Vladimir.json` `abilities.basic_ability_2` (`range`, `effects[id=damage_per_tick]`, `tick_interval_seconds`).
  - Vladimir defensive-ability-two script policy defaults load from `../Characters/Vladimir.json` `simulation.controlled_champion.defensive_ability_two`.
  - Zhonya's Hourglass defaults load from `../Items/Zhonyas Hourglass.json` (`effects_structured[id=zhonyas_time_stop]`).
  - Guardian Angel defaults load from `../Items/Guardian Angel.json` (`effects_structured[id=rebirth_resurrection_with_post_revive_health_and_mana_restore]`).
  - Protoplasm Harness defaults load from `../Items/Protoplasm Harness.json` (lifeline effects).
  - controlled champion stasis activation policy default loads from `data/champion_ai_profiles.json`.
- Protoplasm Harness lifeline cooldown:
  - fallback default is loaded from `../Items/Protoplasm Harness.json` `effects_structured[id=lifeline_gain_bonus_health_below_health_threshold].cooldown_seconds`.
- Protoplasm Harness lifeline trigger health:
  - `simulation.protoplasm_trigger_health_percent` is honored when provided; otherwise canonical item-data default is used.
- Legacy controlled champion tuning keys removed:
  - parser rejects legacy `simulation.vlad_*` knobs.
  - controlled champion offensive/defensive ability tuning must come from canonical champion data and script capabilities.
- Enemy actor policy is scenario-minimal and data-driven:
  - `opponents.encounters[].actors[]` config only actor identity, level, placement, and optional stack overrides.
  - actor `id` values may repeat across encounters only when they refer to the same champion identity; conflicting champion reuse for one `id` is rejected.
  - Champion damage/crowd-control behavior comes from canonical champion data plus champion scripts.
  - Placement/movement policy: `opponents.encounters[].actors[].placement.position` plus `placement.movement` (`hold_position` or `maintain_combat_range`).
- Report now includes:
  - Headline objective score and component outcomes (time alive, damage dealt, healing done, enemy kills)
  - Objective score breakdown for the optimized build:
    - weighted contribution and impact share (%) of survival, damage, healing, and enemy kills
    - delta vs configured weight to reveal overshadowed or underperforming objective components
  - Cap-survivor indicator for the optimized build outcome
  - Enemy derived combat profile diagnostics and similarity warnings
  - Search diagnostics (full eval counts, candidate pool, seed variance, objective weights)
  - Robust vs fragile build confidence based on ensemble seed hit rate
  - Pareto-front tagging over objective/EHP/AP/cost-timing metrics (uses the same controlled champion stack overrides as objective simulation)
  - Cache hit/miss/wait diagnostics
- Build-order optimization is focused on robust/Pareto builds first, with fallback to top builds if needed.
- Controlled champion loadout (runes/shards) is co-optimized with items in joint scoring (no loadout shortlist pre-elimination).

## Multi-Scenario Objective
- `opponents.encounters` is required and supports multiple weighted encounters.
- Each encounter entry includes:
  - `name`
  - `weight`
  - `actors` (same actor schema as primary encounter)
- Objective score is aggregated across scenarios with worst-case blending via `search.multi_scenario_worst_weight`.
- Objective normalization references are derived from scenario horizon and enemy total effective health.

## Rune Pages
- Optional scenario loadout blocks:
  - `controlled_champion.loadout`
- Controlled champion loadout optimization samples legal loadouts from the full generated domain and does not use `controlled_champion.loadout` as a search seed.
- Supported keys:
  - `runes_reforged.rune_names` (ordered array of 6 rune names:
    primary `[keystone, slot2, slot3, slot4]`, secondary `[two runes from different secondary slots in slot order]`)
  - `runes_reforged.shard_stats` (ordered array of 3 shard stats by shard slot)
- Legacy keys fail fast instead of being silently ignored:
  - `runes_reforged.rune_ids`
  - `season2016_masteries`
- Current implementation applies deterministic stat bonuses from direct passive/stat effects and reports selections/skips in output.
- Conditional or highly dynamic rune effects that cannot be represented deterministically are skipped and documented in the report.
- Runes with no modeled deterministic stat effect and no modeled combat-time runtime effect are explicitly listed in report warnings as unmodeled.
- Best-build report output also lists controlled champion items that still have unmodeled passive/active/structured runtime effects.

# Current State Snapshot (2026-02-25)

This file is a concise handoff for developers and AI agents.

## Primary Intent
- Keep the simulation engine generic and data-driven.
- Keep champion/item/rune-specific behavior in scripts and canonical data files.
- Use multi-algorithm search to find high-value item + rune loadouts and build orders.

## What Is Implemented Now
- Rust simulation engine with fixed server-tick stepping (URF default 30 Hz).
- Generic actor/champion abstractions for controlled champion and opponents.
- World subsystem skeleton is present under `src/world/` with deterministic map-bound ownership state and encounter-position validation.
- Champion script dispatch under `src/scripts/champions/`.
- Engine-facing controlled champion script facade under `src/scripts/champions/controlled_champion.rs`.
- Controlled champion script coverage now includes `Vladimir` and `Sona`.
- Controlled champion basic attacks now execute through recurring start/windup/hit events (hitbox/projectile-aware), using shared runtime attack-speed/on-hit effect paths.
- Shared loadout runtime now includes generic combat-time rune trigger hooks (on-hit, ability-hit, outgoing-damage healing, immobilize-triggered effects).
- Combat-time keystone coverage now includes Press the Attack, Fleet Footwork, Conqueror, Aftershock, Electrocute, First Strike, and Phase Rush.
- Shared runtime rune coverage also includes Arcane Comet, Summon Aery, Hail of Blades, Dark Harvest, Triumph, Gathering Storm, and Second Wind.
- Controlled champion and enemy actors now run rune combat logic through the same shared runtime interfaces.
- Search now also applies explicit unmodeled-item-effect quality gates (hard-gate or per-item penalty) alongside unmodeled-rune policy.
- CLI primary modes are `controlled_champion`, `controlled_champion_fixed_loadout`, `controlled_champion_fixed_loadout_rune_sweep`, and `controlled_champion_step` (`vladimir`/`vladimir_step` aliases still accepted).
- Controlled-champion modes now fail fast when the selected champion has no registered controlled-champion script, with an actionable supported-champion list in the error path.
- Item and runtime loadout script hooks under `src/scripts/items/` and `src/scripts/runtime/`.
- Shared runtime stat-query resolution for cooldowns and scalar combat metrics (incoming damage taken, healing, movement speed, outgoing bonus-ability damage) from base data + runtime buff state.
- Strict scenario schema and minimal scenario setup under `Simulation/scenarios/`.
- URF item allow-list restrictions and enemy URF preset loadouts.
- Parallelized search/ranking with per-run in-memory score dedupe cache.
- Top-level search orchestration is parallelized for ensemble seeds and portfolio strategies, and strategy-elite/adaptive generation is parallelized with deterministic merge ordering.
- Report and trace outputs are optimized-build only (baseline comparison path removed).
- Trace JSON output is schema-versioned and structured for downstream tooling.
- Controlled-champion and fixed-loadout trace/report artifacts now include deterministic replay signatures (final-state checksum, tick-state checksum, queue checksum, tick/event counters) for reproducibility audits.
- Controlled-champion, fixed-loadout, and rune-sweep artifact trace generation now performs strict paired replay verification and hard-fails on signature mismatches.
- Report and trace outputs now include rune proc telemetry totals plus source-attribution breakdown (`source_breakdown`), proc attempt/eligible metrics and rates, and damage/healing share metrics.
- Search-time scoring simulations now run with full rune-proc telemetry collection disabled; dedicated trace/report replay simulations explicitly enable it.
- Optional `simulation.combat_seed` now enables deterministic combat-variation simulation runs (enemy initialization ordering + initial auto-attack jitter).
- Controlled champion runtime helper module is now stateless (defensive item/revive policy only); no dedicated controlled runtime holder is stored in engine state.

## Search Behavior (Important)
- Seed behavior:
  - default is runtime-random seed (`search.seed: 0`).
  - deterministic reproducibility is explicit (`--seed <u64>` or scenario `search.seed`).
  - reports include the effective seed.
- `maximum_quality` behavior:
  - runs a pre-budget coverage stage that explicitly touches each legal item/rune/shard asset.
  - retains top diverse candidates per locked asset and injects them into main search.
  - starts runtime budget accounting only after coverage stage completes.
  - popcorn progress-window timeout does not interrupt this coverage stage.
  - if coverage cannot complete, search proceeds in explicit degraded mode and diagnostics include coverage warning flags.
- Runtime-budget behavior:
  - timed budget arms on first timed-phase simulation evaluation (not during setup/report generation).
  - for `maximum_quality`, timed budget can only arm after coverage-stage completion.
- Candidate scoring behavior:
  - generation-time strategy ranking can score partial candidates (improves greedy/beam branching quality before full builds are complete).
  - strict final ranking remains full-candidate only.
  - if time limits interrupt seed search, finite partial candidates are deterministically completed to full candidates before strict fallback selection.
  - full-loadout `beam` and `greedy` now co-optimize loadout selection (runes/shards) during item search.
  - adaptive/bleed strategy-key ordering is normalized before index-based seed math for fixed-seed reproducibility.
  - strict ranking can heuristic-order remaining candidates using strict-stage item/rune/shard signals, with configurable random exploration promotions.
  - unmodeled-rune quality gating is explicit and configurable:
    - optional hard gate (`unmodeled_rune_hard_gate`)
    - optional per-rune score penalty (`unmodeled_rune_penalty_per_rune`)
    - diagnostics include rejected/penalized candidate counts.
    - quality profiles now apply policy defaults:
      - `maximum_quality`: hard gate enabled
      - `fast`/`balanced`: penalty mode
    - when hard gate is enabled, generation domain is filtered to modeled rune choices before search (invalid rune pages are not generated and then rejected later).
  - unmodeled-item-effect quality gating is explicit and configurable:
    - optional hard gate (`unmodeled_item_effect_hard_gate`)
    - optional per-item score penalty (`unmodeled_item_effect_penalty_per_item`)
    - diagnostics include rejected/penalized candidate counts.
    - quality profiles now apply policy defaults:
      - `maximum_quality`: hard gate enabled
      - `fast`/`balanced`: penalty mode
    - when hard gate is enabled, controlled-champion item generation pool is filtered to modeled runtime-effect items before search (invalid runtime-effect items are not generated and then rejected later).
  - diagnostics now report effective thread count and parallel-mode flags for orchestration phases.

## Data/Runtime Correctness Updates
- `simulation.protoplasm_trigger_health_percent` is honored when set.
- If controlled champion level overrides simulation fallback level, Protoplasm level-scaled defaults are recalculated to the effective controlled level unless explicitly overridden.
- Legacy loadout keys now fail fast:
  - `loadout.runes_reforged.rune_ids`
  - `loadout.season2016_masteries`
- Legacy simulation tuning keys now fail fast:
  - `simulation.vlad_*`
- Deterministic loadout stat parsing no longer maps cooldown values expressed in seconds to ability haste.
- Scenario objective invulnerable-seconds normalization now references scenario horizon (instead of a fixed one-second baseline).
- Enemy respawn delay now uses each enemy actor's own level.
- Pareto/EHP/AP metric diagnostics now apply controlled champion stack overrides, matching objective simulation assumptions.
- Report metrics/build-order diagnostics re-resolve candidate loadout stats from in-run evaluation data and fallback recomputation paths.
- Opponent encounters now require at least one positive encounter weight; all-zero-weight scenario sets are rejected.
- Vladimir Sanguine Pool is modeled as per-tick area damage-over-time with range checks on each tick; trace events now include enemy-hit counts for area spells and pool ticks.
- Controlled champion cast gating now enforces cast-lock state (windup/channel/lockout), preventing same-tick spell stacking from engine scheduling.
- Controlled champion offensive-ultimate-before-defensive-ability-two policy now loads from `Characters/Vladimir.json` simulation policy data (script-owned; not engine hardcoded).
- Reports now explicitly list controlled champion runes that currently have no modeled deterministic or combat-time runtime effect.
- Shared runtime rune effects now apply combat-time behavior for Press the Attack, Fleet Footwork, Conqueror, Aftershock, Electrocute, First Strike, and Phase Rush rather than leaving them as unmodeled placeholders.
- Shared runtime rune effects now also cover Arcane Comet, Summon Aery, Hail of Blades, Dark Harvest, Triumph, Gathering Storm, and Second Wind.
- Aftershock resist-window mitigation now affects incoming physical and magic damage during the active window for both controlled champion and enemies.
- Report generation now hard-fails when controlled champion rune/shard labels are incomplete.
- Reports and report JSON now also expose controlled champion best-build items with unmodeled passive/active/structured runtime effects.
- Fixed-loadout rune sweep repeat aggregation now varies deterministic combat seeds per repeat rather than re-running identical combat realizations.
- Added calibration regressions for Electrocute, Arcane Comet, First Strike, and Aftershock level-scaling formulas plus a pool multi-target tick-hit/damage regression.
- Deterministic loadout stat parsing now supports shard stat `tenacity`.
- Controlled champion search now fails fast when strict ranking produces no valid full-build candidates, preventing invalid empty-build report/trace outputs.
- Controlled champion scenario, fixed-loadout, rune-sweep, and step modes now fail fast when the selected controlled champion has no registered controlled-champion script (instead of silently running without script abilities).
- Controlled champion script initialization now returns typed errors to runtime orchestration (no panic path) for missing champion-script defaults.
- Controlled champion scenario, fixed-loadout, rune-sweep, and step flows now validate encounter placement through shared world-state ownership checks before search/runtime execution.
- Engine event-resolution and trace/query paths now use guarded owner-channel reads instead of panic-on-missing index assumptions.
- Non-test `expect(...)` callsites are now removed from `Simulation/src`.
- Enemy champion script modules (`Morgana`, `Warwick`, `Vayne`, `Sona`) now fail soft (no action/cooldown output) when required canonical ability defaults are missing.
- Champion-specific defaults loaders now use fallible cached reads (no panic paths in those optional per-champion defaults channels).
- Required defaults channels now use centralized strict hard-fail ownership (`defaults.rs`) with no silent empty-map fallback for required simulator/champion/mode defaults channels.
- Non-test `panic!(...)` callsites are now also removed from `Simulation/src`.
- Startup now runs typed required-defaults preflight before mode dispatch and fails with contextual startup errors if required defaults ownership channels cannot load.
- World encounter-state builder now includes baseline non-champion ecology anchors (structures, monsters, minion lane spawns) through explicit class/allegiance ownership.
- Runtime enemy movement and respawn position updates now route through world ownership upsert/clamp channels, keeping enemy actor positions map-bounded each tick.
- Runtime world lifecycle ownership now advances deterministic minion wave spawn/despawn loops and neutral objective spawn/respawn timers through `src/world/world_actor_lifecycle_channels.rs` and `src/engine/simulation_step/world_lifecycle_step.rs`.
- Champion controller harness phase-2 integration now routes deterministic controlled-champion command ingress through `src/champion_control_harness/*` + `src/engine/controlled_champion_controller_channels.rs`, including per-tick request sequencing, shared action execution channels, command-owned controlled movement stepping, and data-owned fixed tick-delay command execution.
- Harness ingress now includes partial actor-symmetric control channels:
  - `queue_actor_action_request(...)` supports controlled champion and opponent actor command ingress.
  - opponent movement `MoveToPosition`, basic attack `StartBasicAttack`, and stop `StopCurrentAction` are routed through deterministic queued command channels.
  - mapped script-backed opponent `CastAbilityBySlot` channels now execute through manual command ingress with cooldown/range legality reporting.
  - mapped opponent `UseItemActive` channels now support `stasis_item` and `emergency_shield_item` with explicit availability and cooldown legality statuses.
  - enemy `emergency_shield_item` channels now include runtime shield absorption and heal-over-time ownership for manual command execution.
  - manual-control opponents now suppress autonomous script cadence so command execution flows through controller ingress.
  - enemy movement is now action-locked during stasis/stun windows (`enemy_can_take_actions` gating in movement step).
  - invalid actor IDs return explicit `RejectedControlledActorNotFound`.
- Research-backed deterministic request/fast-forward guidance is now tracked in `DETERMINISTIC_REQUEST_AND_FAST_FORWARD_MODEL.md`.

## Full-Game Transformation Status (Non-Data)
- Architecture transformation status (module ownership, explicit naming, owner-channel isolation): `100%` (`DONE`).
- Weighted completion estimate: `64%` (`IN_PROGRESS`).
- Bucket snapshot (complete / remaining):
  - Runtime Systems Completeness (`30%` weight): `61% / 39%`
  - Determinism And Replay Guarantees (`20%` weight): `88% / 12%`
  - Calibration And Correctness (`20%` weight): `69% / 31%`
  - Performance Envelope (`15%` weight): `52% / 48%`
  - Renderer-Contract Readiness (`15%` weight): `40% / 60%`
- Canonical status and gap detail:
  - `FULL_GAME_SIMULATION_BLUEPRINT.md` (`Current Status Snapshot` section)

## Recent Observed Runtime Characteristic
- Coverage stage remains a dominant fixed cost in short runs, but locked rune/shard sampling now uses direct legal construction (no high-attempt rejection loop).
- Short-budget runs can still overshoot wall-clock time because pre-budget coverage breadth is enforced before timed search starts.

## Current Known Tradeoff
- Coverage breadth floor is strong, and lock-generation overhead is reduced, but short-iteration latency is still higher than ideal because coverage breadth remains mandatory pre-budget work.
- Required defaults ownership is now strict and preflighted.
- Deterministic replay signatures are now hard-fail verified in controlled-champion, fixed-loadout, and rune-sweep trace flows; CI-wide replay-gate coverage is still pending.
- Remaining realism lift is now concentrated in command/path ownership and macro event coupling (objective/structure/economy/vision), not defaults or crash-surface channels.
- Controller ingress now includes deterministic fixed delay, data-owned vision radius, actor-symmetric opponent move/stop/basic-attack control, mapped script-cast channels, and mapped `stasis_item`/`emergency_shield_item` item-active channels, but broader opponent item-actives, non-script cast channels, and richer buffering/drop semantics are still pending.

## Highest-Value Next Work (Largest Impact First)
1. Expand partial actor-symmetric ownership from opponent move/stop/basic-attack plus mapped script-cast and mapped `stasis_item`/`emergency_shield_item` into full opponent cast/item legality + execution channels (including non-script cast families and broader item-actives).
2. Couple world lifecycle channels to combat outcomes (objective defeat, structure state transitions, and respawn ownership hooks) under `src/world/*` and engine event-resolution ownership.
3. Replace mixed movement model with terrain-aware command/path channels (pathfinding, collision, and route replanning ownership).
4. Expand event taxonomy for macro systems (spawn/objective/economy/vision events) before adding feature logic.
5. Expand controlled-champion script coverage beyond `Vladimir` and `Sona`, while reducing static registry coupling.
6. Persist and reuse coverage-stage seed corpus across runs.
7. Add explicit coverage tuning controls (enable/disable, trials-per-asset, top-per-asset).
8. Add guardrail tests for:
   - asset coverage guarantee
   - post-coverage time-budget start behavior.
9. Add CI performance budgets and profiling gates for coverage-stage and strict-ranking hot paths.
10. Promote hard-fail replay verification from current artifact flows into CI-wide replay gates and remaining runtime entrypoints (for example broader scenario/stepper channels and future full-match replay channels).

## Where To Look First
- Main orchestration:
  - `src/scenario_runner.rs`
- Search algorithms:
  - `src/search.rs`
- Data/schema parsing:
  - `src/data.rs`
- Reports:
  - `src/reporting.rs`
- Coverage tracking:
  - `COVERAGE_GAPS.md`
- Full-game target blueprint:
  - `FULL_GAME_SIMULATION_BLUEPRINT.md`
- Data authoring workflow:
  - `DATA_AUTHORING_GUIDE.md`
  - `COVERAGE_CHECKLIST.md`
- Contributor rules:
  - repository root `AGENTS.md`

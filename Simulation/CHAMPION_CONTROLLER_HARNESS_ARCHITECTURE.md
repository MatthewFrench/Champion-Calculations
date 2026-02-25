# Champion Controller Harness Architecture

## Purpose
This document defines the authoritative controller harness architecture for game-realistic champion control.

The harness must support both:
- a human-player style controller
- an artificial-intelligence controller

Both controller kinds must use the same capabilities, constraints, and visibility boundaries. No controller kind gets privileged access that bypasses gameplay limits.

## Non-Negotiable Invariants
1. Controller parity:
   human-player and artificial-intelligence controllers use the same observation and action APIs.
2. Partial observability:
   controllers only receive state that is visible from the controlled champion perspective.
3. Server-authoritative legality:
   all action requests return explicit status outcomes; illegal actions are rejected with typed reasons.
4. Deterministic simulation:
   given fixed seed and fixed inputs, controller-visible states and action outcomes are reproducible.
5. Channel ownership:
   visibility projection, action validation, and action status reporting are explicit owner channels.
6. Tick-bound request ingestion:
   controller requests are sampled and processed in deterministic sequence at server-tick boundaries.
7. Fixed command delay contract:
   accepted controller requests execute on a data-owned fixed tick delay to model authoritative ingestion cadence.

## Controlled-Champion Perspective Contract
The harness exposes `ChampionPerspectiveView` (authoritative control-surface snapshot) per tick.

The view includes:
- controlled actor identity and position
- lock states (movement lock, cast lock)
- ability slot readiness (slot mapping, remaining cooldown, cast range)
- item active readiness (remaining cooldown, cast range)
- basic attack readiness (remaining cooldown, attack range)
- visible actors only (id, class, allegiance, position, distance)

The view excludes:
- hidden enemy state outside perspective visibility
- future event queue internals
- privileged search/debug-only internals

## Action Request Contract
The harness supports explicit requests:
- move-to-position
- cast ability by slot
- start basic attack
- use item active
- stop current action

Every request returns `ChampionActionStatusReport` with one typed status:
- `AcceptedQueued`
- lockout rejections (`RejectedMovementLocked`, `RejectedCastLocked`)
- cooldown rejections (`RejectedAbilityOnCooldown`, `RejectedItemActiveOnCooldown`)
- visibility/range rejections (`RejectedTargetNotVisible`, `RejectedTargetOutOfRange`)
- ownership/keying rejections (`RejectedAbilitySlotUnbound`, `RejectedUnknownItemActive`)
- execution-channel rejections (`RejectedTargetInvalidForAction`, `RejectedControlledActorNotFound`, `RejectedUnsupportedAction`)

No illegal request may be silently accepted.

## Artificial-Intelligence Policy Layering
Controller policies sit above the harness contract and cannot bypass it.

Policy layers:
1. Generic baseline policy:
   deterministic rule-based policy that uses only `ChampionPerspectiveView`.
2. Champion-specific overlay policy:
   optional champion policy that can choose an action first, then fall back to generic policy.
3. Future advanced policy:
   utility model, planner, or learned policy still constrained to the same harness APIs.

## Deterministic Tick + Request Model
The runtime should model request handling as a server-authoritative tick loop:

1. Build controller perspective for the tick.
2. Validate requests against that perspective and return explicit status.
3. Queue only accepted requests with stable sequence IDs.
4. Execute queued requests in sequence before normal event-resolution for that tick.
5. Advance simulation using fixed tick stepping only (no large non-tick time jumps for fast-forward).

This preserves deterministic replay behavior while matching authoritative-server request semantics.
Detailed source-backed guidance is tracked in `Simulation/DETERMINISTIC_REQUEST_AND_FAST_FORWARD_MODEL.md`.

## Current Implementation (Phase-1 + Phase-2 + Phase-3 Expanded)
Controller contracts and runtime integration are implemented under:
- `src/champion_control_harness.rs`
- `src/champion_control_harness/champion_control_contracts.rs`
- `src/champion_control_harness/champion_control_observation_channels.rs`
- `src/champion_control_harness/champion_control_action_validation_channels.rs`
- `src/champion_control_harness/champion_control_decision_policy_channels.rs`
- `src/engine/controlled_champion_controller_channels.rs`
- `src/engine/event_resolution/controlled_champion_action_execution_channels.rs`
- `src/engine/simulation_step/controlled_champion_movement_step.rs`

Current scope:
- perspective-view projection from world state
- typed action validation/status contract
- generic baseline policy + layered champion-specific policy wrapper
- deterministic per-tick request queueing and sequence-ordered request execution
- data-owned controller visibility radius and fixed request-delay tuning via `Simulation/data/simulator_defaults.json` (`engine_defaults.controlled_champion_controller_vision_radius`, `engine_defaults.controlled_champion_request_fixed_tick_delay`)
- shared execution channels for controlled champion ability/item actions (script cadence and harness requests use the same execution paths)
- command-owned controlled champion movement stepping with world-bound clamping
- actor-symmetric ingress scaffold now accepts `queue_actor_action_request(...)` for controlled champion and opponent actors, with explicit `RejectedControlledActorNotFound` for invalid actor IDs
- opponent manual-control scaffold now supports deterministic `MoveToPosition`, `StartBasicAttack`, and `StopCurrentAction` command execution through the same queued ingress path
- opponent manual-control cast channels now support mapped script-backed `CastAbilityBySlot` execution for supported enemy champions, including cooldown/range legality reporting through the same harness action-status path
- manual-control opponents now suppress autonomous script cadence so controller requests are the sole command ingress
- unit tests for visibility, fairness parity, legality responses, and policy ordering

## Integration Target State
To reach full game-like behavior, integrate the harness into the runtime loop as the only control ingress:

1. Full vision model:
   fog-of-war, stealth/reveal, brush/ward rules replace distance-only visibility.
2. Command/path model:
   move commands feed pathing/collision ownership channels (not orbit simplification).
3. Full action semantics:
   cast windup, cancel rules, target loss, interruptibility, and queued command ordering.
4. Symmetric actor ownership:
   controlled champion and opponents both use shared control/action legality channels.
5. Replay contract:
   persist view snapshots + action requests + statuses for deterministic playback.

## Testing Requirements
Required coverage for harness evolution:
- visibility projection correctness under changing world state
- parity tests proving same request -> same status for human-player and artificial-intelligence views
- legality tests for cooldown/range/visibility/lock transitions
- determinism tests for fixed-seed controller loops
- replay-contract tests once playback schema exists

## Remaining High-Friction Areas
- current visibility projection is radius-only and not fog-of-war complete
- actor-symmetric ingress is still partial (opponent move/stop/basic-attack and mapped script-cast channels are wired; opponent item-active channels and non-script cast channels are still unsupported)
- command/path ownership is integrated for deterministic move targets, but pathfinding/collision/terrain routing are not yet integrated
- objective/structure/economy channels are not yet wired into perspective visibility and action legality
- only fixed delay ingestion is modeled; richer buffering/overwrite/packet-drop network semantics are still simplified versus live game

## Ownership Boundaries
- Harness contract and legality channels:
  `src/champion_control_harness/*`
- World visibility state:
  `src/world/*`
- Runtime action execution:
  `src/engine/*`
- Champion/item/rune behavior:
  `src/scripts/*`

This boundary is mandatory to prevent controller-policy logic from leaking into shared engine internals.

# Deterministic Request And Fast-Forward Model

Last updated: 2026-02-24

## Goal
Define the runtime model that best matches League-style authoritative simulation while preserving deterministic fast-forward behavior.

## Research-Backed Runtime Principles
These principles are sourced from Riot engineering publications and Riot developer statements.

1. Server-authoritative, deterministic update loop.
- Riot describes moving League toward deterministic simulation ordering where gameplay state advances from deterministic inputs and state transitions, not render timing.

2. Input ordering must be deterministic and explicit.
- Riot determinism work calls out input handling order as a core source of nondeterminism and highlights capturing player/network inputs each frame in ordered form.

3. Fixed-step simulation is required for reliable determinism.
- Riot implementation notes discuss replacing variable frame delta behavior with fixed-step updates to avoid drift.

4. Time source and frame phase boundaries must be unified.
- Riot emphasizes one unified clock and explicit phase ordering so subframe timing jitter does not alter outcomes.

5. Fast-forward should preserve authoritative loop semantics.
- Riot simulator work (Swarm) demonstrates deterministic simulation with no real-time throttling while preserving fixed-step behavior.

6. League server cadence baseline is ~30 updates per second.
- Riot developer comments publicly state League gameplay updates occur at about 30 Hz (~33 ms).

## Recommended Model For This Repository
Use this runtime contract for both human-player and artificial-intelligence controllers:

1. Tick boundary sampling.
- At each simulation tick, build controller perspective views.
- Collect controller requests for the tick (human or policy-generated).

2. Legality gate first.
- Validate every request through the champion harness.
- Return typed status immediately (accepted vs explicit rejection reason).

3. Deterministic accepted-request queue.
- Queue only accepted requests.
- Assign stable ordering keys:
  - `tick_index`
  - `phase`
  - `priority`
  - `sequence_id` (strictly monotonic)

4. Execute accepted requests before event dispatch for the tick.
- Apply command effects in queue order.
- Then run event queue dispatch and hot-effect progression.

5. No illegal side paths.
- Runtime behavior should not mutate controlled actor command state outside ingress owner channels.

6. Fast-forward via repeated fixed ticks, not coarse time jumps.
- Large time skips should be modeled as repeated deterministic ticks, preserving command/event ordering and legality outcomes.

7. Deterministic fixed-delay command execution.
- Accepted controller requests execute after a fixed tick delay configured in global engine defaults.
- Current owner keys:
  - `engine_defaults.controlled_champion_request_fixed_tick_delay`
  - `engine_defaults.controlled_champion_controller_vision_radius`
- This models server-side command ingestion cadence while preserving deterministic replay/fast-forward behavior.

## Current Repository Status
Implemented:
- deterministic controlled-champion request ingress owner channel:
  - `src/engine/controlled_champion_controller_channels.rs`
- actor-id keyed ingress scaffold (controlled champion + opponent move/stop channels):
  - `queue_actor_action_request(...)` in `src/engine/controlled_champion_controller_channels.rs`
- harness legality/status contract integration:
  - `src/champion_control_harness/*`
- sequence-ordered accepted-request execution at tick boundaries:
  - `src/engine/event_resolution/combat_event_dispatch_resolution.rs`
- fixed-tick-delay request execution and data-owned controller visibility defaults:
  - `data/simulator_defaults.json`
  - `src/defaults/simulator_defaults_schema_types/simulation_search_and_engine_defaults_schema.rs`
- shared execution channels for script and harness action paths:
  - `src/engine/event_resolution/controlled_champion_action_execution_channels.rs`
- command-owned controlled movement stepping:
  - `src/engine/simulation_step/controlled_champion_movement_step.rs`

Remaining:
- full actor-symmetric ingress for all controllable actors (opponent move/stop channels are wired; opponent cast/basic-attack/item command channels remain)
- fog/vision-aware legality
- full buffering overwrite/drop model parity beyond fixed-delay ingress
- replay contract + checksum validation

## Sources
- [Determinism in League of Legends: Implementation](https://technology.riotgames.com/news/determinism-league-legends-implementation)
- [Determinism in League of Legends: Unified Clock](https://technology.riotgames.com/news/determinism-league-legends-unified-clock)
- [Determinism in League of Legends: In Practice](https://technology.riotgames.com/news/determinism-league-legends-practice)
- [Swarm: Making an MMO Bullet-Heaven Simulator](https://technology.riotgames.com/news/swarm-making-mmo-simulator)
- [Running Online Services at Riot, Part III](https://technology.riotgames.com/news/running-online-services-riot-part-iii)
- [Riot developer statement on League server update cadence (~30 Hz)](https://www.reddit.com/r/leagueoflegends/comments/8una2o/comment/e1h5dli/)

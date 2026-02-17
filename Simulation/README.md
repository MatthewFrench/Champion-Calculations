# URF Vladimir Objective Simulator

This simulator focuses on Vladimir's pool uptime against 5 enemies in URF. For a fixed seed, it is deterministic and runs on a fixed server-tick loop (default 30 Hz) with an event queue for attacks, scripted champion actions, and survivability effects.

## What It Models
- Vladimir uses scripted `W`, `Q`, `E`, and `R` ability cadence.
- Combat runs with 2D positions (controlled champion fixed at origin; enemies maintain range with deterministic orbit/chase motion).
- Simulation intentionally ignores vertical `z` index for now; combat checks use only 2D geometry (`x`,`y`) until a verified gameplay interaction requires `z`.
- Fixed-timestep stepping via `ControlledChampionCombatSimulation.step()` at `server_tick_rate_hz` (legacy `VladCombatSimulation` alias remains available).
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
- Enemy champion abilities and crowd control come from champion scripts and canonical champion data (no scenario combat proxies).
- Enemy units can die and respawn on URF-scaled timers.
- Enemy transient stack/buff counters are cleared on death and respawn, and enemies respawn at their original spawn positions.
- Enemy scripted ability timelines are lifecycle-safe across death/respawn transitions.
- Guardian Angel, Zhonya's Hourglass, and Protoplasm Harness are modeled as survivability events.
- Champion/item/loadout mechanics can be extended through script hooks in `src/scripts/`.
- Controlled champion loadout runtime scripts are now applied during combat-time spell hits, kill events, and regeneration ticks.
- Defensive item activation and revive triggers are modeled through generic controlled champion runtime/item script capabilities (not champion-specific decision structs).
- Shared hook and enemy-script interfaces now use controlled champion terminology (no Vladimir-only cross-module field names).
- Runtime stat resolution is buff-aware and starts from canonical base data before applying state transforms:
  - cooldown metrics resolve through shared runtime stat queries (ability/item/neutral sources)
  - scalar combat metrics resolve through shared runtime stat queries (incoming damage taken, healing, movement speed, and outgoing bonus-ability damage)
  - modeled item cooldown passives (for example Heartsteel and Luden's Echo) load base cooldowns from canonical item effects data and then apply runtime haste/buff state
- Vladimir combat sequencing decisions are script-owned and delegated from engine.
- Enemy champion script events are generated in scripts and applied by generic engine action handling.
- Foundational combat primitives are present for future fidelity work:
  - generic status effects (duration/stacks/persistence)
  - generic cast-lock windows (windup/channel/lockout)
- Scripted enemy behavior profiles are included for:
  - Warwick
  - Vayne
  - Morgana
  - Sona
  - Doctor Mundo
- Build candidate scoring is parallelized across CPU cores (Rayon).
- Search uses strict full-simulation scoring for every generated candidate build.
- Full simulation scoring is memoized by canonical build key.
- Full simulation scores are persisted across runs under `Simulation/output/cache/`.
- In-flight dedupe cache avoids duplicate parallel re-simulation of the same canonical build.
- Ensemble seed runs are supported for confidence/robustness labeling.
- Cross-algorithm bleed round recombines elite candidates across strategies before final full ranking.
- Adaptive strategy allocation adds extra candidates from strategies that contribute more unique elites.
- Strict final candidate ranking evaluates remaining candidates in parallel batches.
- Build scoring uses a composite objective over:
  - time alive
  - damage dealt to enemies
  - healing done
  - enemy champions killed
  - invulnerable/untargetable seconds
  with configurable weights and per-scenario reference normalization.
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
- `data/champion_ai_profiles.json`: Champion AI controller policy (combat spacing, movement scaling, script polling, script-event priority overrides, and non-canonical cooldown overrides when canonical data is missing).
- `../Game Mode/URF.json`: URF mode data, including mode-specific simulation defaults (for example respawn tuning).
- `../Characters/<Champion>.json`: Champion canonical gameplay data, including per-ability execution fields (`abilities.<ability_key>.execution`) and ability/passive effect data used by scripts.
- `../Characters/ChampionDefaults.json`: Champion-style nested role defaults (`base_stats`, `basic_attack`, `abilities.execution_defaults`) used as fallback when champion files omit those canonical fields.
- `CURRENT_STATE.md`: concise current-state handoff for developers and AI agents.
- `IMPROVEMENT_TRACKER.md`: Done and pending improvements.
- `IMPLEMENTATION_ROADMAP.md`: roadmap status and planned phases.
- `Cargo.toml`: Rust package manifest.
- `src/main.rs`: CLI and orchestration.
- `src/core.rs`: Shared simulation math/helpers plus foundational generic combat primitives (status/cast-lock scaffolding).
- `src/data.rs`: Scenario/data loading, config parsing, loadout legality generation, and enemy preset validation.
- `src/defaults.rs`: Typed schema and loader for global defaults plus domain-file champion/mode simulation defaults.
- `src/engine.rs`: Fixed-tick combat engine and event-queue simulation loop.
- `src/build_order.rs`: Build-order stage simulation and optimization.
- `src/search.rs`: Build search algorithms, portfolio/ensemble orchestration, diversity selection, and metric helpers.
- `src/reporting.rs`: Markdown/JSON report generation.
- `src/scenario_runner.rs`: Scenario mode execution orchestration (`vladimir`, `vladimir_step`, stat modes).
- `src/cache.rs`: In-memory and persisted score cache implementations.
- `src/status.rs`: Deadline and status progress reporting helpers.
- `src/respawn.rs`: URF respawn timer model helpers.
- `src/scripts/champions/mod.rs`: Champion script dispatch, behavior profiles, runtime wrappers, and shared action/event types (including script effect hitbox descriptors).
- `src/scripts/champions/vladimir/mod.rs`: Vladimir scripted formulas and combat decision APIs (offense and defensive ability decisions).
- `src/scripts/champions/<champion>/mod.rs`: Per-champion behavior/event logic modules.
- `src/scripts/items/hooks.rs`: Item-specific simulation scripts (for example, Heartsteel stack override handling).
- `src/scripts/runes/effects.rs`: Rune runtime flag parsing and dynamic-runtime classification.
- `src/scripts/runtime/controlled_champion_loadout.rs`: Controlled champion runtime effects, defensive item/revive decision helpers, and loadout hook implementation.
- `src/scripts/runtime/loadout_runtime.rs`: Shared combat-time loadout runtime state and effect helpers.
- `src/scripts/runtime/stat_resolution.rs`: Shared runtime stat-query resolver for buff-aware metric transformations (cooldowns plus scalar combat metrics from base data + runtime buff state).
- `src/scripts/registry/hooks.rs`: Script hook interfaces, contexts, and dispatch registry.

## Run
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode vladimir
```
- `vladimir` mode writes a markdown report to:
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_run_report.md`
- `vladimir` mode writes a structured JSON report to:
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_run_report.json`
  where `<controlled_champion_key>` is the normalized champion name (for example: `vladimir`).
  where `<search_quality_profile>` is one of `fast`, `balanced`, `maximum_quality`.
  where `<runtime_budget>` is:
  - `no_hard_cap` when no runtime budget is set.
  - `<seconds>s` when only fixed budget is used (for example `300s`).
  - `<budget>__popcorn_<window>__min_improvement_<relative_percent>pct` when popcorn mode is enabled and `window != budget`.
  - `<budget>__popcorn__min_improvement_<relative_percent>pct` when popcorn mode is enabled and `window == budget`.
- Report includes:
  - Human-readable generation timestamps (local and UTC)
  - Vladimir base stats at configured level (`controlled_champion.level` override, fallback `simulation.champion_level`, default `20`)
  - Vladimir end stats for best build
  - Stack override notes for stack-based items in the best build
  - Enemy derived combat profiles (HP/AD/AS/range/hit/burst stats) with similarity warnings for suspiciously close auto profiles
  - Detailed search diagnostics including:
    - effective search seed used
    - coverage-stage diagnostics for `maximum_quality` (elapsed, assets covered, seeded candidates)
    - explicit simulation counts (new full simulations, unique scored candidates, total score requests)
    - search elapsed time and total run time (end-to-end)
    - cache hits/misses
    - unique scored candidates across all search stages
    - per-search-type breakdown (requests/new simulations/persistent cache hits)
    - generated/unique/pruned candidate counts
    - strict-stage completion percentage and timeout-skipped candidate count
    - estimated legal candidate-space size and coverage percentages
    - heuristic closeness-to-optimal probability estimate with explicit assumptions
  - If run with a time budget, report includes timeout and completion metadata
- Trace outputs are also champion-keyed:
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_event_trace.md`
  - `Simulation/output/runs/controlled_champion/<search_quality_profile>/<runtime_budget>/<controlled_champion_key>_event_trace.json`
  - Trace JSON schema:
    - `schema_version`: integer schema version for trace consumers
    - `event_encoding`: currently `structured`
    - `events[]`: objects with `timestamp_seconds`, `event_type`, `details`, and `raw`
  - trace includes explicit impact outcome events such as `projectile_blocked`, `impact_nullified`, `attack_missed`, and `ability_missed`.

## Runtime Controls
- `--max-runtime-seconds N`:
  - Stops search after `N` seconds and reports best-so-far findings.
  - In `maximum_quality`, the timer starts after the pre-budget coverage stage completes.
- `--popcorn-window-seconds W`:
  - Enables progress-window stopping ("microwave popcorn mode").
  - Search continues while significant improvements keep happening; run stops when no significant improvement is observed for `W` seconds.
  - Popcorn mode always uses the `maximum_quality` search profile, regardless of `--search-quality-profile`.
  - Can be combined with `--max-runtime-seconds`:
    - stop condition is whichever comes first.
- `--popcorn-min-relative-improvement-percent R`:
  - Relative objective-score delta required to count as significant progress.
  - Significant threshold is `R%` of the last best score.
- `--status-every-seconds N`:
  - Prints periodic status lines (phase, elapsed, progress, best score) while searching.
- `--search-quality-profile {fast|balanced|maximum_quality}`:
  - Applies opinionated search settings. Default is `maximum_quality`.
- `--seed N`:
  - Overrides runtime seed with deterministic value `N`.
  - If not provided, runtime seed is random unless the scenario explicitly sets `search.seed`.

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
- You can cap threads with:
```bash
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode vladimir \
  --threads 8
```

## Diverse Top Builds
- `vladimir` mode can output top diverse builds near the best score:
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

## Vladimir Step Debug (Tick-by-Tick)
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "Simulation/Cargo.toml" -- \
  --scenario "vladimir_urf_teamfight" \
  --mode vladimir_step \
  --ticks 60
```
- Step mode uses the first entry in `opponents.encounters` and prints the selected encounter name.
- `--scenario` accepts either:
  - a full/relative file path, or
  - a scenario name resolved from `Simulation/scenarios/<name>.json`.

## Extensibility
- Champion/item mechanics should be added in dedicated modules (for example under `src/scripts/`) rather than growing `main.rs`.
- Scenario JSON should stay minimal and reference canonical data from `Characters`, `Items`, and `Game Mode`.
- Opponent actors no longer accept `combat` proxy blocks; use champion scripts/data only.
- Opponent groups no longer accept `opponents.uptime_windows_enabled`; combat windows are script/runtime driven.
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
  champions/
    mod.rs
    vladimir/
      mod.rs
      abilities.rs
      decisions.rs
      hook.rs
    warwick/
      mod.rs
    vayne/
      mod.rs
    morgana/
      mod.rs
    sona/
      mod.rs
    doctor_mundo/
      mod.rs
  items/
    mod.rs
    hooks.rs
  runes/
    mod.rs
    effects.rs
  runtime/
    mod.rs
    controlled_champion_loadout.rs
    loadout_runtime.rs
  registry/
    mod.rs
    hooks.rs
```

This migration is active and tracked in the roadmap and improvement tracker for follow-up phases.

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
  - `ranked_limit`
  - `seed` (optional; if omitted or `0`, runtime random seed is used)
- Loadout search legality:
  - Rune pages are generated from legal primary/secondary slot rules in `RunesReforged.json`.
  - Shards are generated from legal `stat_shards` slot options.
  - Runtime validation rejects illegal pages before simulation (invalid path/slot/shard structure).
  - Loadout optimization is always on for controlled champion build scoring; there is no scenario shortlist/sample knob.
- Enemy presets:
  - `vladimir` mode uses `data/enemy_urf_presets.json` for enemy full builds and rune pages/shards.
  - Startup validation fails fast if a preset references missing item/rune/shard data.
- Default scenario is tuned for high search quality (deeper exploration and more seed stability), so expect higher CPU time than previous presets.
- `maximum_quality` runs a pre-budget coverage stage that locks each item/rune/shard at least once, keeps top diverse candidates per locked asset, and injects those seeds into the main search.
- Stack overrides (generic, keyed by stack identifier):
  - `simulation.stack_overrides` sets global stack overrides by stack identifier (example: `{ "heartsteel": 20.0 }`).
  - `controlled_champion.stack_overrides` overrides global stack overrides for the controlled champion.
  - `opponents.stack_overrides` sets default opponent overrides, and each actor can further override with `opponents.encounters[].actors[].stack_overrides`.
  - In build-order optimization, stack overrides are distributed by item acquisition level and current stage level (buying later yields fewer stacks by level 20).
- Level defaults:
  - `simulation.champion_level` is the fallback level.
  - `controlled_champion.level` overrides controlled champion level.
  - `opponents.default_level` overrides fallback for opponent actors, and `opponents.encounters[].actors[].level` overrides per actor.
- Scenario simulation knobs are now minimal by default:
  - optional overrides:
    - `simulation.time_limit_seconds` (default from `Simulation/data/simulator_defaults.json`; default value `1200` seconds)
    - hard cap: `<= 1200` seconds (20 minutes)
    - `simulation.server_tick_rate_hz`
    - `simulation.champion_level`
    - `simulation.stack_overrides`
- Fallback ownership by domain:
  - URF respawn defaults load from `../Game Mode/URF.json` `respawn`.
  - Vladimir Sanguine Pool defaults load from `../Characters/Vladimir.json` `abilities.basic_ability_2`.
  - Zhonya's Hourglass defaults load from `../Items/Zhonyas Hourglass.json` (`effects_structured[id=zhonyas_time_stop]`).
  - Guardian Angel defaults load from `../Items/Guardian Angel.json` (`effects_structured[id=rebirth_resurrection_with_post_revive_health_and_mana_restore]`).
  - Protoplasm Harness defaults load from `../Items/Protoplasm Harness.json` (lifeline effects).
  - controlled champion stasis activation policy default loads from `data/champion_ai_profiles.json`.
- Protoplasm Harness lifeline cooldown:
  - fallback default is loaded from `../Items/Protoplasm Harness.json` `effects_structured[id=lifeline_gain_bonus_health_below_health_threshold].cooldown_seconds`.
- Vladimir scripted ability knobs:
  - `simulation.vlad_q_base_damage`, `simulation.vlad_q_ap_ratio`, `simulation.vlad_q_heal_ratio_of_damage`, `simulation.vlad_q_base_cooldown_seconds`
  - `simulation.vlad_e_base_damage`, `simulation.vlad_e_ap_ratio`, `simulation.vlad_e_base_cooldown_seconds`
  - `simulation.vlad_r_base_damage`, `simulation.vlad_r_ap_ratio`, `simulation.vlad_r_base_cooldown_seconds`
  - fallback defaults are loaded from `../Characters/Vladimir.json` under `abilities.basic_ability_1`, `abilities.basic_ability_3`, and `abilities.ultimate` (effects plus cooldowns).
- Enemy actor policy is scenario-minimal and data-driven:
  - `opponents.encounters[].actors[]` config only actor identity, level, placement, and optional stack overrides.
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
  - Pareto-front tagging over objective/EHP/AP/cost-timing metrics
  - Cache hit/miss/wait diagnostics
- Build-order optimization is focused on robust/Pareto builds first, with fallback to top builds if needed.
- Vladimir loadout (runes/shards) is co-optimized with items in joint scoring (no loadout shortlist pre-elimination).

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
- Current implementation applies deterministic stat bonuses from direct passive/stat effects and reports selections/skips in output.
- Conditional or highly dynamic rune effects that cannot be represented deterministically are skipped and documented in the report.

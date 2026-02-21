# URF Controlled Champion Objective Simulator

This simulator targets controlled-champion URF teamfight optimization with champion-specific behavior delegated through script capabilities (Vladimir is the current implemented controlled champion script). For a fixed seed, it is deterministic and runs on a fixed server-tick loop (default 30 Hz) with an event queue for attacks, scripted champion actions, and survivability effects.

## What It Models
- Vladimir uses scripted `W`, `Q`, `E`, and `R` ability cadence.
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
- Combat-time rune coverage now also includes Arcane Comet, Summon Aery, Hail of Blades, Dark Harvest, Triumph, Gathering Storm, Second Wind, Scorch, Cheap Shot, Taste of Blood, Absorb Life, and Coup de Grace.
- Controlled champion and enemy actors consume the same shared rune-combat runtime interfaces; controlled-champion runtime module now only owns defensive item/revive policy helpers.
- Search scoring now also supports explicit unmodeled-item-effect quality gating (hard gate or per-item penalty) to reduce ranking bias from unimplemented item effects.
- When unmodeled hard gates are enabled, controlled-champion candidate generation space is constrained up front (modeled-rune loadout domain and modeled-runtime-item pool) so invalid candidates are not generated and then rejected later.
- Optional `simulation.combat_seed` applies deterministic combat variation (enemy initialization order + initial attack jitter) for robust repeated evaluation without nondeterminism.
- Full rune-proc telemetry collection is disabled for search-time scoring simulations and enabled explicitly for trace/report replay simulations.
- Rune telemetry runtime bookkeeping uses fixed-index counter arrays (no per-event hashmap lookup/allocation in hot paths).
- Aftershock now models an active resist window that reduces incoming physical and magic damage while active.
- Defensive item activation and revive triggers are modeled through generic controlled champion runtime/item script capabilities (not champion-specific decision structs).
- Shared hook and enemy-script interfaces now use controlled champion terminology (no Vladimir-only cross-module field names).
- Runtime stat resolution is buff-aware and starts from canonical base data before applying state transforms:
  - cooldown metrics resolve through shared runtime stat queries (ability/item/neutral sources)
  - scalar combat metrics resolve through shared runtime stat queries (incoming damage taken, healing, movement speed, and outgoing bonus-ability damage)
  - modeled item cooldown passives (for example Heartsteel and Luden's Echo) load base cooldowns from canonical item effects data and then apply runtime haste/buff state
- Rune runtime tuning defaults are loaded from `data/simulator_defaults.json` under `rune_runtime_defaults` (global ownership).
- Controlled-champion combat sequencing decisions are delegated through a champion-script facade from engine (Vladimir is currently the implemented controlled-champion script).
- Enemy champion script events are generated in scripts and applied by generic engine action handling.
- Foundational combat primitives are present for future fidelity work:
  - generic status effects (duration/stacks/persistence)
  - generic cast-lock windows (windup/channel/lockout)
- Controlled champion cast availability now respects active cast-lock windows, preventing same-tick multi-cast stacking from engine-side scheduling.
- Canonical champion data roster is now imported from `From Online/champions` into `Characters/` (baseline data coverage), while runtime script-depth remains incremental by champion.
- Scripted enemy behavior profiles are included for:
  - Warwick (Jaws of the Beast, Infinite Duress)
  - Vayne (Tumble, Condemn, Silver Bolts passive hit-tracking)
  - Morgana (Dark Binding, Tormented Shadow, Soul Shackles)
  - Sona (Hymn of Valor, Crescendo)
  - Doctor Mundo (Infected Bonesaw, Blunt Force Trauma)
  - Vladimir (Transfusion, Tides of Blood, Hemoplague first-pass)
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
- `data/champion_ai_profiles.json`: Champion AI controller policy (combat spacing, movement scaling, script polling, script-event priority overrides, and non-canonical cooldown overrides when canonical data is missing).
- `../Game Mode/URF.json`: URF mode data, including mode-specific simulation defaults (for example respawn tuning).
- `../Characters/<Champion>.json`: Champion canonical gameplay data, including per-ability execution fields (`abilities.<ability_key>.execution`) and ability/passive effect data used by scripts.
- `../Characters/ChampionDefaults.json`: Champion-style nested role defaults (`base_stats`, `basic_attack`, `abilities.execution_defaults`) used as fallback when champion files omit those canonical fields.
- `CURRENT_STATE.md`: concise current-state handoff for developers and AI agents.
- `COVERAGE_GAPS.md`: tracked list of known game-fidelity and implementation-coverage gaps.
- `COVERAGE_CHECKLIST.md`: contributor checklist for champion/item/rune/shard coverage work.
- `COVERAGE_IMPLEMENTATION_PLAYBOOK.md`: definition-of-done quality bar and batch workflow for comprehensive runtime coverage.
- `DATA_AUTHORING_GUIDE.md`: canonical workflow for authoring champion/item/rune data and wiring runtime behavior.
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
- `src/scenario_runner.rs`: Scenario mode execution orchestration (`controlled_champion`, `controlled_champion_step`, stat modes).
- `src/cache.rs`: In-memory score cache implementation (per-run only).
- `src/status.rs`: Deadline and status progress reporting helpers.
- `src/respawn.rs`: URF respawn timer model helpers.
- `src/scripts/champions/mod.rs`: Champion script dispatch, behavior profiles, runtime wrappers, and shared action/event types (including script effect hitbox descriptors).
- `src/scripts/champions/vladimir/mod.rs`: Vladimir scripted formulas and combat decision APIs (offense and defensive ability decisions).
- `src/scripts/champions/<champion>/mod.rs`: Per-champion behavior/event logic modules.
- `src/scripts/items/hooks.rs`: Item-specific simulation scripts (for example, Heartsteel stack override handling).
- `src/scripts/runes/effects.rs`: Dynamic-runtime rune classification list for loadout/runtime diagnostics.
- `src/scripts/runtime/controlled_champion_loadout.rs`: Controlled champion defensive item/revive decision helpers plus loadout hook implementation.
- `src/scripts/runtime/loadout_runtime.rs`: Shared combat-time loadout runtime state and effect helpers.
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
- Scenario JSON should stay minimal and reference canonical data from `Characters`, `Items`, and `Game Mode`.
- Opponent actors no longer accept `combat` proxy blocks; use champion scripts/data only.
- Opponent groups no longer accept `opponents.uptime_windows_enabled`; combat windows are script/runtime driven.
- For contributor workflow and completion criteria:
  - data authoring workflow: `DATA_AUTHORING_GUIDE.md`
  - comprehensive implementation quality bar: `COVERAGE_IMPLEMENTATION_PLAYBOOK.md`
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


## Execution Trackers
- `Simulation/CHAMPION_ROSTER_TRACKER.md`: roster-wide champion baseline/script-depth checklist (generated by `Simulation/scripts/generate_coverage_trackers.py`).
- `Simulation/COVERAGE_EXECUTION_TRACKER.md`: execution checklist for item/rune/mastery runtime backlog (generated by `Simulation/scripts/generate_coverage_trackers.py`).
- Regenerate both trackers with: `Simulation/scripts/generate_coverage_trackers.py`.

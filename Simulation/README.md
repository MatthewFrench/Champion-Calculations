# URF Vladimir Objective Simulator

This simulator focuses on Vladimir's pool uptime against 5 enemies in URF. It is deterministic and now runs on a fixed server-tick loop (default 30 Hz) with an event queue for attacks, ability damage ticks, and crowd control.

## What It Models
- Vladimir uses scripted `W`, `Q`, `E`, and `R` ability cadence.
- Combat now runs with 2D positions (Vladimir fixed at origin, enemies spawned in range bands and standing still).
- Fixed-timestep stepping via `VladCombatSimulation.step()` at `server_tick_rate_hz`.
- URF global buffs (ability/item haste, health cost multiplier, attack speed multipliers) are applied.
- Enemy auto-attacks use start, windup, and hit phases.
- Ranged attacks/spells include projectile travel time based on distance and speed.
- Projectile block checks use segment intersection with colinear overlap handling.
- Enemy auto-attacks and spell damage are modeled as recurring timed events.
- Stuns are modeled as recurring timed events that delay Vladimir's casting.
- Enemy units can die and respawn on URF-scaled timers.
- Enemy transient stack/buff counters are cleared on death and respawn, and enemies respawn at their original spawn positions.
- Enemy scripted ability timelines are lifecycle-safe across death/respawn and uptime-window transitions.
- Guardian Angel, Zhonya's Hourglass, and Protoplasm Harness are modeled as survivability events.
- Champion/item/loadout mechanics can be extended through script hooks in `src/scripts/`.
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
  with configurable weights and per-scenario baseline normalization.

## Files
- `scenario_vlad_urf.json`: Scenario setup (champion references, behavior knobs, tick rate, build search settings).
- `data/enemy_urf_presets.json`: Hardcoded URF enemy end-game presets with sources and check date.
- `IMPROVEMENT_TRACKER.md`: Done and pending improvements.
- `Cargo.toml`: Rust package manifest.
- `src/main.rs`: CLI and orchestration.
- `src/core.rs`: Shared simulation math/helpers (objective scoring, stat transforms, build/random utilities).
- `src/data.rs`: Scenario/data loading, config parsing, loadout legality generation, and enemy preset validation.
- `src/engine.rs`: Fixed-tick combat engine and event-queue simulation loop.
- `src/build_order.rs`: Build-order stage simulation and optimization.
- `src/search.rs`: Build search algorithms, portfolio/ensemble orchestration, diversity selection, and metric helpers.
- `src/reporting.rs`: Markdown/JSON report generation.
- `src/scenario_runner.rs`: Scenario mode execution orchestration (`vladimir`, `vladimir_step`, stat modes).
- `src/cache.rs`: In-memory and persisted score cache implementations.
- `src/status.rs`: Deadline and status progress reporting helpers.
- `src/respawn.rs`: URF respawn timer model helpers.
- `src/scripts/vladimir.rs`: Vladimir scripted ability formulas/cooldowns and champion-specific hooks.
- `src/scripts/enemies.rs`: Enemy champion behavior profiles and attack on-hit scripting.
- `src/scripts/hooks.rs`: Script hook interfaces and dispatch points.
- `src/scripts/item_hooks.rs`: Item-specific simulation scripts (for example, Heartsteel stack assumptions).
- `src/scripts/loadout_hooks.rs`: Loadout/rune/mastery script notes and extension point.

## Run
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vladimir
```
- `vladimir` mode now also writes a markdown report to `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/output/vladimir_run_report.md`.
- `vladimir` mode also writes a structured JSON report to `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/output/vladimir_run_report.json`.
- Report includes:
  - Vladimir base stats at configured level (`simulation.champion_level`, default `20`)
  - Vladimir end stats for best build
  - Stack assumptions/notes for stack-based items in the best build
  - Enemy derived combat profiles (HP/AD/AS/range/hit/burst stats) with similarity warnings for suspiciously close auto profiles
  - If run with a time budget, report includes timeout/progress metadata

## Runtime Controls
- `--max-runtime-seconds N`:
  - Stops search after `N` seconds and reports best-so-far findings.
- `--status-every-seconds N`:
  - Prints periodic status lines (phase, elapsed, progress, best score) while searching.
- `--search-quality-profile {fast|balanced|maximum_quality}`:
  - Applies opinionated search settings. Default is `maximum_quality`.

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
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
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
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode taric_as
```

## Hecarim (Max Move Speed)
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode hecarim_ms
```

## Vladimir Step Debug (Tick-by-Tick)
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vladimir_step \
  --ticks 60
```

## Extensibility
- Champion/item mechanics should be added in dedicated modules (for example under `src/scripts/`) rather than growing `main.rs`.
- Scenario JSON should stay minimal and reference canonical data from `Characters`, `Items`, and `Game Mode`.

## Minimal Scenario Shape
- Use champion references instead of hardcoding base stats:
  - `vladimir_champion`: champion name from `Characters/`.
  - `enemies[].champion`: champion name from `Characters/`.
- Keep only scenario-specific behavior in scenario JSON (example: simplified `ability_dps_*`, stun cadence).
- Build search item pool is restricted to purchasable `LEGENDARY` items only.
- Pre-evolution items are normalized to evolved forms in simulation lookups:
  - `Manamune` -> `Muramana`
  - `Archangel's Staff` -> `Seraph's Embrace`
- Mode availability note:
  - Item JSON does not currently expose an explicit `available_in_modes` field.
  - As a practical URF-safe rule, search excludes Arena/distributed-only item patterns and focuses on normal-rift-eligible legendary items.

## Notes
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
  - `objective_survival_weight`, `objective_damage_weight`, `objective_healing_weight`
  - `robust_min_seed_hit_rate`
  - `bleed_enabled`, `bleed_budget`, `bleed_mutation_rate`
  - `multi_scenario_worst_weight` (aggregation between weighted-mean and worst-case when using multiple enemy scenarios)
  - `ranked_limit`
- Loadout search legality:
  - Rune pages are generated from legal primary/secondary slot rules in `RunesReforged.json`.
  - Shards are generated from legal `stat_shards` slot options.
  - Mastery pages are generated from legal Season 2016 tree/tier/point constraints in `Season2016.json`.
  - Loadout optimization is always on for Vladimir build scoring; there is no scenario shortlist/sample knob for runes/shards/masteries.
- Enemy presets:
  - `vladimir` mode uses `data/enemy_urf_presets.json` for enemy full builds and rune/mastery pages.
  - Startup validation fails fast if a preset references missing item/rune/shard/mastery data.
- Default scenario is tuned for high search quality (deeper exploration and more seed stability), so expect higher CPU time than previous presets.
- Heartsteel assumptions:
  - `simulation.heartsteel_assumed_stacks_at_8m` controls expected proc count by 8 minutes (default `20`).
  - Simulator converts that proc count into an estimated permanent bonus health and applies it as effective bonus health.
  - In build-order optimization, Heartsteel stacks are distributed by item acquisition level and current stage level (so buying it later yields fewer stacks by level 20).
- Level assumption:
  - `simulation.champion_level` sets champion level used for base stat scaling in simulation and report (default `20`).
- Respawn model knobs:
  - `simulation.urf_respawn_flat_reduction_seconds` (default `3.0`)
  - `simulation.urf_respawn_extrapolation_per_level` (default `2.5`)
- Vladimir scripted ability knobs:
  - `simulation.vlad_q_base_damage`, `simulation.vlad_q_ap_ratio`, `simulation.vlad_q_heal_ratio_of_damage`, `simulation.vlad_q_base_cooldown_seconds`
  - `simulation.vlad_e_base_damage`, `simulation.vlad_e_ap_ratio`, `simulation.vlad_e_base_cooldown_seconds`
  - `simulation.vlad_r_base_damage`, `simulation.vlad_r_ap_ratio`, `simulation.vlad_r_base_cooldown_seconds`
- Enemy script hooks (scenario enemy fields):
  - Burst windows: `burst_interval_seconds`, `burst_start_offset_seconds`, `burst_magic_flat`, `burst_physical_flat`, `burst_true_flat`, `burst_ad_ratio`, `burst_ap_ratio`
  - Optional uptime model: enable with `simulation.enemy_uptime_model_enabled`, then per enemy use `uptime_cycle_seconds`, `uptime_active_seconds`, `uptime_phase_seconds`
- Report now includes:
  - Headline objective score and component outcomes (time alive, damage dealt, healing done, enemy kills)
  - Cap-survivor indicators for baseline and best build outcomes
  - Enemy derived combat profile diagnostics and similarity warnings
  - Search diagnostics (full eval counts, candidate pool, seed variance, objective weights)
  - Robust vs fragile build confidence based on ensemble seed hit rate
  - Pareto-front tagging over objective/EHP/AP/cost-timing metrics
  - Cache hit/miss/wait diagnostics
- Build-order optimization is focused on robust/Pareto builds first, with fallback to top builds if needed.
- Vladimir loadout (runes/masteries/shards) can be co-optimized with items in joint scoring (no loadout shortlist pre-elimination).

## Multi-Scenario Objective
- Optional `enemy_scenarios` array is supported:
  - each entry can include `name`, `weight`, and `enemies` (same schema as top-level `enemies`)
- If `enemy_scenarios` is omitted, simulator uses top-level `enemies` as a single scenario.
- Objective score is aggregated across scenarios with worst-case blending via `search.multi_scenario_worst_weight`.

## Runes/Masteries
- Optional scenario loadout blocks:
  - `vladimir_loadout`
  - `enemy_loadout` (`vladimir_step` mode only)
- Supported keys:
  - `runes_reforged.rune_ids` (array of rune IDs)
  - `runes_reforged.rune_names` (array of rune names)
  - `runes_reforged.shard_stats` (slot-ordered shard stat keys, e.g. `ability_haste`, `health`, `attack_speed`)
  - `season2016_masteries` (array of mastery names, or objects `{ \"name\": \"...\", \"rank\": N }`)
- Current implementation applies deterministic stat bonuses from direct passive/stat effects and reports selections/skips in output.
- Conditional or highly dynamic rune/mastery effects that cannot be represented deterministically are skipped and documented in the report.

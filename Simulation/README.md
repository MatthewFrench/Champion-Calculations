# URF Vladimir Survival Simulator

This simulator focuses on Vladimir's pool uptime and survival time against 5 enemies in URF. It is deterministic and now runs on a fixed server-tick loop (default 30 Hz) with an event queue for attacks, ability damage ticks, and crowd control.

## What It Models
- Vladimir only casts W (Sanguine Pool) on cooldown.
- Fixed-timestep stepping via `VladCombatSimulation.step()` at `server_tick_rate_hz`.
- URF global buffs (ability/item haste, health cost multiplier, attack speed multipliers) are applied.
- Enemy auto-attacks and spell damage are modeled as recurring timed events.
- Stuns are modeled as recurring timed events that delay Vladimir's casting.
- Guardian Angel, Zhonya's Hourglass, and Protoplasm Harness are modeled as survivability events.
- Champion/item mechanics can be extended in compiled Rust code paths.
- Build candidate scoring is parallelized across CPU cores (Rayon).
- Search now supports two-stage scoring:
  - coarse proxy filter for broad exploration
  - full event simulation for finalist ranking
- Full simulation scoring is memoized by canonical build key.
- In-flight dedupe cache avoids duplicate parallel re-simulation of the same canonical build.
- Ensemble seed runs are supported for confidence/robustness labeling.
- Cross-algorithm bleed round recombines elite candidates across strategies before final full ranking.
- Adaptive strategy allocation adds extra candidates from strategies that contribute more unique elites.
- Full ranking now uses capped prechecks to prune clearly non-competitive candidates before exact simulation.

## Files
- `scenario_vlad_urf.json`: Scenario setup (champion references, behavior knobs, tick rate, build search settings).
- `Cargo.toml`: Rust package manifest.
- `src/main.rs`: Simulator and optimizer.

## Run
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vlad
```
- `vlad` mode now also writes a markdown report to `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/output/vlad_run_report.md`.
- Report includes:
  - Vladimir base stats at configured level (`simulation.champion_level`, default `20`)
  - Vladimir end stats for best build
  - Stack assumptions/notes for stack-based items in the best build

## Threading
- The Rust optimizer leaves one core free by default (`available_cores - 1`, minimum 1 thread).
- Override thread count with `--threads N` if needed.
- You can cap threads with:
```bash
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vlad \
  --threads 8
```

## Diverse Top Builds
- `vlad` mode can output top diverse builds near the best score:
  - `--top-x` number of diverse builds to keep (default `8`)
  - `--min-item-diff` minimum symmetric item difference between selected builds (default `2`)
  - `--max-relative-gap-percent` max score drop from best to still be considered (default `5.0`)
  - `--report-path` optional custom report output path
- After top builds are selected, simulator also optimizes full-item build order:
  - Uses full-item permutations only (no partial/intermediate items).
  - Uses stage levels evenly spaced from 5 to 20 across item slots.
  - Scores each order by cumulative survival across stages.

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

## Vlad Step Debug (Tick-by-Tick)
```bash
source "$HOME/.cargo/env"
cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vlad_step \
  --ticks 60
```

## Extensibility
- Champion/item mechanics should be added as compiled Rust logic in `src/main.rs` (or split into modules as the codebase grows).
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
- This is still a survival-first model; spell DPS is now eventized but full per-spell champion kits still need script/data integration.
- The build search supports: `beam`, `greedy`, `random`, `hill_climb`, `genetic`, `simulated_annealing`, `mcts`, and `portfolio`.
- Default scenario uses `portfolio`, which runs multiple algorithms in parallel and merges candidates.
- Useful knobs in `search`:
  - `portfolio_strategies`
  - `hill_climb_restarts`, `hill_climb_steps`, `hill_climb_neighbors`
  - `genetic_population`, `genetic_generations`, `genetic_mutation_rate`, `genetic_crossover_rate`
  - `simulated_annealing_restarts`, `simulated_annealing_iterations`, `simulated_annealing_initial_temp`, `simulated_annealing_cooling_rate`
  - `mcts_iterations`, `mcts_rollouts_per_expansion`, `mcts_exploration`
  - `ensemble_seeds`, `ensemble_seed_stride`, `ensemble_seed_top_k`
  - `coarse_pool_limit`, `robust_min_seed_hit_rate`
  - `bleed_enabled`, `bleed_budget`, `bleed_mutation_rate`
  - `multi_scenario_worst_weight` (aggregation between weighted-mean and worst-case when using multiple enemy scenarios)
  - `ranked_limit`
- Default scenario is tuned for high search quality (deeper exploration and more seed stability), so expect higher CPU time than previous presets.
- Heartsteel assumptions:
  - `simulation.heartsteel_assumed_stacks_at_8m` controls expected proc count by 8 minutes (default `20`).
  - Simulator converts that proc count into an estimated permanent bonus health and applies it as effective bonus health.
  - In build-order optimization, Heartsteel stacks are distributed by item acquisition level and current stage level (so buying it later yields fewer stacks by level 20).
- Level assumption:
  - `simulation.champion_level` sets champion level used for base stat scaling in simulation and report (default `20`).
- Enemy script hooks (scenario enemy fields):
  - Burst windows: `burst_interval_seconds`, `burst_start_offset_seconds`, `burst_magic_flat`, `burst_physical_flat`, `burst_true_flat`, `burst_ad_ratio`, `burst_ap_ratio`
  - Optional uptime model: enable with `simulation.enemy_uptime_model_enabled`, then per enemy use `uptime_cycle_seconds`, `uptime_active_seconds`, `uptime_phase_seconds`
- Report now includes:
  - Search diagnostics (coarse/full eval counts, candidate pool, seed variance)
  - Robust vs fragile build confidence based on ensemble seed hit rate
  - Pareto-front tagging over survival/EHP/AP/cost-timing metrics
  - Cache hit/miss/wait diagnostics and capped precheck counts
- Build-order optimization is focused on robust/Pareto builds first, with fallback to top builds if needed.

## Multi-Scenario Objective
- Optional `enemy_scenarios` array is supported:
  - each entry can include `name`, `weight`, and `enemies` (same schema as top-level `enemies`)
- If `enemy_scenarios` is omitted, simulator uses top-level `enemies` as a single scenario.
- Objective score is aggregated across scenarios with worst-case blending via `search.multi_scenario_worst_weight`.

## Runes/Masteries
- Optional scenario loadout blocks:
  - `vladimir_loadout`
  - `enemy_loadout` (applied to all enemies)
- Supported keys:
  - `runes_reforged.rune_ids` (array of rune IDs)
  - `runes_reforged.rune_names` (array of rune names)
  - `runes_reforged.shard_stats` (slot-ordered shard stat keys, e.g. `ability_haste`, `health`, `attack_speed`)
  - `season2016_masteries` (array of mastery names, or objects `{ \"name\": \"...\", \"rank\": N }`)
- Current implementation applies deterministic stat bonuses from direct passive/stat effects and reports selections/skips in output.
- Conditional or highly dynamic rune/mastery effects that cannot be represented deterministically are skipped and documented in the report.

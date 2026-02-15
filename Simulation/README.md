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
- The Rust optimizer uses all available CPU cores by default.
- You can cap threads with:
```bash
RAYON_NUM_THREADS=8 cargo run --release --manifest-path "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/Cargo.toml" -- \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vlad
```

## Diverse Top Builds
- `vlad` mode can output top diverse builds near the best score:
  - `--top-x` number of diverse builds to keep (default `8`)
  - `--min-item-diff` minimum symmetric item difference between selected builds (default `2`)
  - `--max-relative-gap-percent` max score drop from best to still be considered (default `5.0`)
  - `--report-path` optional custom report output path

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
- The build search uses a beam search by default. You can switch to greedy or random in the scenario.
- Heartsteel assumptions:
  - `simulation.heartsteel_assumed_stacks_at_8m` controls expected proc count by 8 minutes (default `20`).
  - Simulator converts that proc count into an estimated permanent bonus health and applies it as effective bonus health.
- Level assumption:
  - `simulation.champion_level` sets champion level used for base stat scaling in simulation and report (default `20`).

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

# URF Vladimir Survival Simulator

This simulator focuses on Vladimir's pool uptime and survival time against 5 enemies in URF. It is deterministic and now runs on a fixed server-tick loop (default 30 Hz) with an event queue for attacks, ability damage ticks, and crowd control.

## What It Models
- Vladimir only casts W (Sanguine Pool) on cooldown.
- Fixed-timestep stepping via `VladCombatSimulation.step()` at `server_tick_rate_hz`.
- URF global buffs (ability/item haste, health cost multiplier, attack speed multipliers) are applied.
- Enemy auto-attacks and spell damage are modeled as recurring timed events.
- Stuns are modeled as recurring timed events that delay Vladimir's casting.
- Guardian Angel, Zhonya's Hourglass, and Protoplasm Harness are modeled as survivability events.
- Optional Python scripts can register hooks for champion/item logic that is not generically modeled.

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

## Notes
- Champion base stats are loaded from `Characters/*.json` by champion name.
- This is still a survival-first model; spell DPS is now eventized but full per-spell champion kits still need script/data integration.
- The build search uses a beam search by default. You can switch to greedy or random in the scenario.

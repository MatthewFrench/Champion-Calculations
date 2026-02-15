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
- `scenario_vlad_urf.json`: Scenario, base stats, enemy list, tick rate, and build search settings.
- `urf_sim.py`: Simulator and optimizer.

## Run
```bash
python "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/urf_sim.py" \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vlad
```

## Taric (Max Attack Speed)
```bash
python "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/urf_sim.py" \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode taric_as
```

## Hecarim (Max Move Speed)
```bash
python "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/urf_sim.py" \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode hecarim_ms
```

## Vlad Step Debug (Tick-by-Tick)
```bash
python "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/urf_sim.py" \
  --scenario "/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json" \
  --mode vlad_step \
  --ticks 60
```

## Script Hooks
- Scenario-level scripts: `simulation.scripts` (list of Python file paths).
- Enemy-level scripts: `enemies[i].scripts` (list of Python file paths).
- Relative script paths are resolved from the scenario file directory.
- Each script must define:
```python
def register(sim):
    # sim.register_hook("on_pre_tick", callback)
    # sim.schedule_event(...)
    pass
```
- Available hook names:
  - `on_init`
  - `on_pre_tick`
  - `on_post_tick`
  - `on_event_pre`
  - `on_event_post`
  - `on_damage`
  - `on_cast_pool`
  - `on_cast_zhonya`
  - `on_trigger_protoplasm`
  - `on_ga_revive`

## Notes
- The base stats for champions are placeholders; adjust them in `scenario_vlad_urf.json` as needed.
- This is still a survival-first model; spell DPS is now eventized but full per-spell champion kits still need script/data integration.
- The build search uses a beam search by default. You can switch to greedy or random in the scenario.

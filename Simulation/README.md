# URF Vladimir Survival Simulator

This simulator focuses on Vladimir's pool uptime and survival time against 5 enemies in URF. It is deterministic and uses a simplified combat model so we can iterate on builds quickly.

## What It Models
- Vladimir only casts W (Sanguine Pool) on cooldown.
- URF global buffs (ability/item haste, health cost multiplier, attack speed multipliers) are applied.
- Enemy damage is modeled as auto-attack DPS + a simple spell DPS term.
- Stuns are modeled as fixed intervals that delay Vladimir's casting.
- Guardian Angel, Zhonya's Hourglass, and Protoplasm Harness are modeled as survivability events.

## Files
- `scenario_vlad_urf.json`: Scenario, base stats, enemy list, and build search settings.
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

## Notes
- The base stats for champions are placeholders; adjust them in `scenario_vlad_urf.json` as needed.
- This is a survival-first model and does not yet simulate full ability rotations.
- The build search uses a beam search by default. You can switch to greedy or random in the scenario.

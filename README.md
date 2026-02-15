# Vladimir URF Build Goal

Goal: In URF, maximize how long Vladimir survives and how much damage he deals to enemies around him while surviving. The strategy centers on Sanguine Pool uptime (W). While in pool, Vladimir is untargetable, so the build should prioritize durability, sustained damage, and pool uptime; other abilities are secondary.

This project can be extended to other champions and scenarios, but the current focus is Vladimir versus 5 enemies.

**Intent**
Create a local, reproducible dataset and deterministic simulator to search for survivability‑optimized URF builds without relying on external meta sites.

**Folder Structure**
- `/Users/matthewfrench/Documents/League of Legends/Vladimir/Characters` Champion data used by the simulator (abilities, effects).
- `/Users/matthewfrench/Documents/League of Legends/Vladimir/Game Mode` Mode rules and URF global buffs.
- `/Users/matthewfrench/Documents/League of Legends/Vladimir/Items` Item stats, passives, and actives in normalized JSON.
- `/Users/matthewfrench/Documents/League of Legends/Vladimir/Masteries` Rune and mastery data.
- `/Users/matthewfrench/Documents/League of Legends/Vladimir/From Online` Raw and normalized imports, schemas, and item pipeline utilities.
- `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation` Deterministic simulator and scenarios.

**Data Notes**
- Items are local JSON files with stats, passives, actives, and parsed effects.
- URF mode data includes global buffs (haste, health cost multiplier, attack speed modifiers) and notes about patch variability.
- Champion base stats and attack type are loaded from `Characters` by champion reference in scenario files; scenarios should only contain scenario-specific behavior knobs.

**Simulator**
- Entry point: `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/urf_sim.py`
- Scenario config: `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json`
- Focus: Vlad survival time while chaining W in a fight against 5 enemies.
- Deterministic: Same inputs produce the same results.
- Models: pool uptime, health costs, basic healing from pool, GA revive, Zhonya stasis, Protoplasm lifeline, and enemy attacks/spell damage/stuns as timed events on a fixed tick loop (default 30 Hz).
- Extensibility: optional Python hook scripts can add champion/item behaviors that are not generically modeled.

**Current Drawbacks (Important)**
- The current simulator is a simplified model. It does not yet run full champion kits from `Characters` data.
- Enemy output is approximated with auto-attack DPS + simplified spell DPS terms from scenario config.
- `Masteries`/runes are not yet modeled in the combat loop.
- This means outputs are useful for fast iteration and ranking directionally, but not final high-fidelity truth.

**Incremental Plan (In Progress)**
- Wire champion base stats, growth, and ability data from `Characters` into the simulation engine.
- Replace simplified enemy DPS terms with ability/event timelines derived from champion data.
- Add rune/mastery effects from `Masteries` where they materially affect survivability and DPS.
- Expand item passive/active coverage and timing interactions.
- Keep deterministic mode as default while improving realism step by step.

**Research Goal**
Find a “best URF tier” survivability build for Vladimir and compare against fixed baselines (including a specific baseline item list). Expand later to attack speed (Taric) and move speed (Hecarim) optimization.

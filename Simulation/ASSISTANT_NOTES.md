# Assistant Notes (Historical Decision Log)

## Archive Notice
- This file contains early-phase historical notes and includes outdated assumptions.
- Use `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/CURRENT_STATE.md` for current architecture/search behavior.
- Use `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/README.md` for current run semantics and schema details.

Date: 2026-02-10

## Why This Exists
This note log explains the decisions and assumptions behind the URF Vladimir simulator so another session can quickly align on the intent, modeling choices, and known gaps.

## High-Level Intent
- Build a deterministic simulator focused on Vladimir’s survival time while chaining Sanguine Pool (W) in URF.
- Avoid reliance on external meta sites by using local structured data (items, champions, URF mode rules).
- Enable a fast search for “best URF tier” builds without exhaustive combinations.

## What I Built
- `src/main.rs`: a deterministic Rust simulator and build optimizer.
- `scenario_vlad_urf.json`: baseline scenario with champion references, 5 enemies, stuns, and search parameters.
- `README.md`: usage and purpose of the simulator.

## Why These Decisions
1. Deterministic simulation over Monte Carlo
   - Required by user and easier to debug.
   - Enables reproducible comparisons across builds and changes.

2. Simplified combat model
   - Uses auto-attack DPS + a flat/scaled “ability DPS” term for enemies.
   - Keeps the model stable and fast while still sensitive to item stats.

3. Priority on Vlad W (Sanguine Pool)
   - The user emphasized pool uptime and survival vs. 5 enemies.
   - W cooldown, health cost, healing, and untargetable window are explicitly modeled.

4. Handling of GA / Zhonya / Protoplasm
   - These items directly affect “time alive.”
   - GA modeled as a revive window with base-health heal.
   - Zhonya modeled as a timed stasis triggered at low HP.
   - Protoplasm modeled as a threshold lifeline (bonus health + HoT).

5. Beam search instead of exhaustive search
   - Exhaustive search is expensive with hundreds of items.
   - Beam search is deterministic, tunable, and provides a good balance between coverage and speed.

## Key Assumptions (Simplifications)
- Enemy champions are modeled with base stats + item stats only.
- Enemy damage = auto-attack DPS + ability DPS with simple AD/AP scaling.
- Stuns are modeled as periodic events that can delay W casts.
- No explicit targeting, movement, or spacing logic beyond stun timing.
- No full ability rotations (other than Vlad W), no summoner spells.
- URF buffs applied only for ability/item haste, health cost multiplier, and attack speed multipliers.

## How Items Are Used
- Loads from `Items/` JSON.
- Filters out `CONSUMABLE` and `TRINKET` ranks.
- Prevents multiple boots.
- Treats item stats as additive; most passives/actives are not yet fully modeled.

## Scenario Configuration
File: `scenario_vlad_urf.json`
- References Vlad and enemies by champion name, loading base stats from `Characters/`.
- Includes ability DPS and stun intervals for enemies.
- Includes Vlad baseline fixed item list for comparison.
- Search configuration uses beam search.

## Current Output Example (From Initial Run)
- Baseline fixed build time alive: ~11.45s
- Best survival build found: ~14.95s

Note: These numbers are sensitive to the placeholder base stats and DPS values. Adjust in `scenario_vlad_urf.json` to match your desired model.

## Known Gaps / Next Steps
- Expand active/passive modeling (e.g., Rocketbelt, Randuin active, additional lifelines).
- Add ability haste and item haste impacts to more actives.
- Incorporate CC reduction/tenacity effects and partial stun mitigation.
- Introduce optional Monte Carlo mode for robustness (while keeping deterministic default).
- Add a small local search (swap/replace) on top of beam search.
- Add “enemy build constraints” (force boots, ban Golden Spatula, etc.).

## If You Pick This Up
Start by validating:
- Base stats for Vladimir and enemy champions.
- Enemy DPS parameters and stun timings.
- Vlad W healing ratio and URF health cost modifier.
- The baseline item list and any items to exclude from search.

Once validated, tighten the item pool and improve passives/actives to get more realistic rankings.

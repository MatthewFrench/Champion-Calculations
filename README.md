# Vladimir URF Build Goal

Goal: In URF, maximize how long Vladimir survives and how much damage he deals to enemies around him while surviving. The strategy centers on Sanguine Pool uptime (W). While in pool, Vladimir is untargetable, so the build should prioritize durability, sustained damage, and pool uptime; other abilities are secondary.

This project can be extended to other champions and scenarios, but the current focus is Vladimir versus 5 enemies.

**Architecture Standard**
- The long-term target is a champion-agnostic simulation core with script-driven specialization.
- Core simulation systems should remain generic and reusable.
- Champion-specific and item/rune/mastery-specific mechanics should live in script modules with shared interfaces.
- New code and documentation should avoid abbreviations for champion names and major domain terms.

**Intent**
Create a local, reproducible dataset and deterministic simulator to search for survivability‑optimized URF builds without relying on external meta sites.

**Folder Structure**
- `Characters/` Champion data used by the simulator (abilities, effects).
- `Game Mode/` Mode rules and URF global buffs.
- `Items/` Item stats, passives, and actives in normalized JSON.
- `Masteries/` Rune and mastery data.
- `From Online/` Raw and normalized imports, schemas, and item pipeline utilities.
- `Simulation/` Deterministic simulator and scenarios.

**Data Notes**
- Items are local JSON files with stats, passives, actives, and parsed effects.
- URF mode data includes global buffs (haste, health cost multiplier, attack speed modifiers) and notes about patch variability.
- Champion base stats and attack type are loaded from `Characters` by champion reference in scenario files; scenarios should only contain scenario-specific behavior knobs.

**Simulator**
- Entry point: `Simulation/src/main.rs`
- Scenario config catalog: `Simulation/scenarios/`
- Focus: Vlad survival time while chaining W in a fight against 5 enemies.
- Deterministic: Same inputs produce the same results.
- Models: pool uptime, health costs, basic healing from pool, GA revive, Zhonya stasis, Protoplasm lifeline, and enemy attacks/spell damage/stuns as timed events on a fixed tick loop (default 30 Hz).
- Extensibility: Rust engine can be extended with additional champion/item mechanics as compiled code.
- Search scope: build search operates on purchasable `LEGENDARY` items only (no intermediate components).
- Level assumption: simulation currently uses configurable champion level (default level 20 for URF team-fight modeling).

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

**License**
- Code and original repository content are licensed under `AGPL-3.0-or-later`. See `LICENSE`.
- Some files include third-party content and references that are not relicensed as your own. See `THIRD_PARTY_NOTICES.md`.

**Contributions**
- Contributions are welcome.
- Contributors must agree to `CLA.md` (details in `CONTRIBUTING.md`) to keep future licensing options available.

**Commercial Licensing (Future)**
- No separate commercial license is currently offered.
- The maintainer may offer one in the future. See `COMMERCIAL_LICENSE.md`.

**Name and Branding**
- Project name is currently "Champion Calculations" (descriptive, no registered trademark claim at this time).
- See `TRADEMARKS.md` for branding and anti-confusion guidance.

**Riot / Third-Party Disclaimer**
- This project is not affiliated with or endorsed by Riot Games.
- League of Legends and Riot Games names, marks, and game IP belong to Riot Games, Inc.

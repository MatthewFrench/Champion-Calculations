# Confidence And Review Questions

## High Confidence
- Core simulation/search pipeline compiles cleanly and passes strict linting and tests.
- Enemy death and respawn loops are active, with URF-scaled respawn delays.
- Vladimir combat sequencing decisions are now delegated through script APIs.
- Enemy champion script-event behavior is now generated in scripts and applied by generic engine action handlers.
- Enemy temporary combat stacks now clear on death and on respawn:
  - Lethal Tempo stacks
  - Guinsoo stacks
  - Fervor stacks
  - Thunderlord stack counter
  - periodic on-hit counters (for example Vayne-style third-hit tracking)
- Enemy respawn now also resets position to original spawn position.
- Reports now include per-enemy derived combat profiles and similarity warnings for suspiciously close auto-attack profiles.
- Default ownership is now domain-based and loaded via `Simulation/src/defaults.rs`:
  - global simulator/search/engine defaults from `Simulation/data/simulator_defaults.json`
  - mode defaults from `Game Mode/URF.json`
  - champion defaults from `Characters/*.json`
- Controlled champion spell readiness now tracks by ability identity through runtime slot mapping primitives.

## Medium Confidence (Likely Correct But Approximate)
- Scripted enemy ability timing and damage constants are intentionally first-pass approximations.
- Projectile blocking currently uses line-segment intersection with active block zones; this is deterministic but simplified versus full engine geometry.
- Movement/kiting model is deterministic and 2D, but still simplified relative to full pathing/collision/turn-rate behavior.
- Using only 2D (`x`,`y`) geometry and ignoring vertical `z` index is likely acceptable for current simulator scope, but not yet validated against every niche interaction.

## Low Confidence / Needs Validation
- Full kit fidelity for champions is still incomplete:
  - Vladimir offensive script is first pass and does not yet include every empowered/conditional nuance.
  - Enemy script coverage is incomplete beyond current scripted champions/events.
- Rune and mastery runtime effects are only partially modeled in combat-time behavior.
  - Some dynamic effects are still represented as notes or simplified assumptions.
- Respawn timing currently uses level-scaling and URF flat reduction, but does not yet include full game-time increase-factor modeling from live rules.
- Projectile interaction is not yet full collision/hitbox/path-block fidelity.
- Ability identity is still partially represented by champion-specific cast fields; slot mapping foundations exist, but full actor-wide slot-agnostic runtime remapping support (for stolen/swapped abilities) is not yet implemented.

## Questions To Review
1. Do we want to include game-time as an explicit simulation input so death timers can apply full time-based scaling (not just level-based scaling)?
2. Should we prioritize full Vladimir kit fidelity next, or broader enemy-champion script coverage first?
3. Should dynamic rune/mastery runtime effects be elevated into dedicated script modules per rune/mastery family (for example keystones first)?
4. Do we want a stricter verification mode that compares scripted values against sourced tables and fails on unknown/unsourced constants?
5. Should we treat key bindings as pure actor input slots mapped to runtime ability instances so stolen abilities and remaps are first-class?
6. For ability theft behavior, should stolen abilities inherit source-champion scaling rules exactly, or should they resolve through recipient-champion overrides when documented?

## Research Notes (2026-02-16)
- Cooldowns during death:
  - Community-maintained League wiki states cooldowns continue while dead.
  - Source: [Death (League Wiki)](https://wiki.leagueoflegends.com/en-us/Death)
- Current base death-timer direction:
  - Official patch notes show recent Summoner's Rift death-timer rule changes (example: 26.1 adjusted by-level values and time-scaling window).
  - Source: [Patch 26.1 Notes](https://www.leagueoflegends.com/en-us/news/game-updates/patch-26-1-notes/)
- ARURF/URF details:
  - Official recent ARURF patch notes expose many mode-specific knobs but do not clearly publish a full respawn formula.
  - Source: [Patch 25.04 Notes](https://www.leagueoflegends.com/en-us/news/game-updates/patch-25-04-notes/)
  - Implication: URF respawn math should stay configurable via data defaults until we can verify formula details from authoritative sources.
- Ability theft baseline semantics:
  - League wiki documents Sylas Hijack as on-target cooldown-gated steal with hijacked cast held temporarily and cast as recast behavior.
  - Source: [Sylas (League Wiki)](https://wiki.leagueoflegends.com/en-us/Sylas)

## Script-Extraction Backlog (From Audit)
- Vladimir defensive/offensive decisions are script-owned, but the engine still executes some Vladimir effect applications directly after script decisions.
- Enemy script-event behavior generation is script-owned, but effect application remains in engine as generic actions (intended architectural boundary).
- Next extraction opportunity: move more effect execution semantics behind script/config interfaces while keeping engine generic.

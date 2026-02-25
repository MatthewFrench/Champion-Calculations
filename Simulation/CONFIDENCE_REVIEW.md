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
- Champion data uncertainty follow-up:
  - `Zyra` `Garden of Thorns` one-vs-two seed spawn distribution weighting remains unresolved from current source notes; lifecycle/state sequencing is modeled, but exact probability weighting needs an authoritative verification source before confidence can be raised further.
  - Uncertainty 1 (deferred code follow-up): `Renekton` `Slice and Dice` uses formula-based dash velocity (`760 + 100% movement speed`), but runtime execution metadata currently consumes only base dash speed.
  - Uncertainty 2 (deferred code follow-up): `Sylas` slot-E behavior (`Abscond` -> `Abduct`) is a multi-stage same-slot flow; runtime still lacks first-class stage identity and recast-window event modeling.
  - Additional deferred code follow-up: `Vex` `Looming Darkness` uses cast-distance-scaled explosion radius (`200 : 300`), but runtime currently treats execution hitbox radius as a static value.
  - Additional deferred code follow-up: champion data now carries execution-semantic keys for attack-cadence-coupled behavior (for example `resolution_timing`, `target_required`, `resets_basic_attack_timer_on_cast`, `empowered_attack_window_seconds`), but runtime loaders/scripts do not consume these keys yet.
  - Semantic-key authoring baseline has expanded to `18/682` active abilities (including `Nasus` `Siphoning Strike`, `Garen` `Decisive Strike`, `Jax` `Empower`, `Renekton` `Ruthless Predator`, `MonkeyKing` `Crushing Blow`, `Blitzcrank` `Power Fist`, `Leona` `Shield of Daybreak`, `Vayne` `Tumble`, `Camille` `Precision Protocol`, `Ekko` `Phase Dive`, `Rengar` `Savagery`, `XinZhao` `Three Talon Strike`, `Yorick` `Last Rites`, `Trundle` `Chomp`, `Volibear` `Frenzied Maul`, and `Vi` `Relentless Force`); runtime consumption remains deferred code follow-up.
  - Data-quality no-regression update: truncation audits now include effect-level `effects[*].context_notes`; wave 120 repaired previously discovered fragments on `XinZhao` `Wind Becomes Lightning` and `Fiora` `Riposte`, leaving current effect-level queue at `0/0`.
- Item data uncertainty follow-up:
  - No open item-structure uncertainty remains in the latest wave for `Dragonheart` or `Twilight's Edge`; keep periodic source-drift checks because page/module text can diverge on distributed Arena items.
  - Active cooldown metadata was backfilled for `Stridebreaker` (`15s`), `Hextech Rocketbelt` (`40s`), and `Redemption` (`90s`); runtime behavior remains deferred where item actives are not yet modeled.
  - Active cooldown metadata was additionally backfilled for `Ravenous Hydra` (`10s`), and the remaining no-fixed-cooldown `on_activate` queue now has explicit reusable `activation_cadence` schema coverage (`11/11` effects across `10` files).
- Rune data quality follow-up:
  - Broad decimal-spacing cleanup is now complete for the previously tracked queue (`0` remaining `x. y` artifacts in `effects_structured.raw`).
  - Rune narrative decimal-spacing cleanup is now complete for touched flat/split rune files (`88` `x. y` artifacts in rune narrative fields -> `0`).
  - Deferred runtime/code follow-up remains: add loader/lint enforcement for raw-to-metadata consistency checks (including guardrails against scripted placeholder artifacts like `\1.\2`) so this stays no-regression by construction.
  - Multi-branch rune `per_rank` effects are now explicitly decomposed via `semantic_components` (`14/14`), but runtime currently does not consume those components directly; this is tracked as deferred code follow-up for maintainability and future deterministic parsing paths.

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

## Data Research Notes (2026-02-24)
- Dragonheart immediate backfill:
  - Current page notes publish explicit acquisition-round mapping for immediate backfill (`3-4 => 1`, `5-6 => 2`, `7-8 => 3`, `9+ => 4`) via the round-versus-souls tooltip table.
  - Source: [Dragonheart](https://wiki.leagueoflegends.com/en-us/Dragonheart)
  - Source: [Dragonheart Patch History](https://wiki.leagueoflegends.com/en-us/Dragonheart/Patch_history)
- Twilight's Edge mode branches:
  - Current page infobox + patch-history section provide level-based AS/AH tooltip tables and show `25%` AD/AP world modifiers; older module-derived description lines still show `20%`, so canonical data now follows page current-state values and records the reconciliation note.
  - Source: [Twilight's Edge](https://wiki.leagueoflegends.com/en-us/Twilight%27s_Edge)
- Gambler's Blade cap reconciliation:
  - Current canonical effect text shows max stored value 240; patch history records a V14.12 historical increase to 245.
  - Source: [Gambler's Blade](https://wiki.leagueoflegends.com/en-us/Gambler%27s_Blade)
- Attack-cadence-coupled champion ability fidelity wave:
  - Re-verified `Jax` `Empower`, `Renekton` `Ruthless Predator`, `Rengar` `Savagery`, and `Wukong` `Crushing Blow` with explicit cast-gate versus empowered-hit resolution timing notes and page-level template citations.
  - Source: [Template:Data_Jax/Empower](https://wiki.leagueoflegends.com/en-us/Template:Data_Jax/Empower)
  - Source: [Template:Data_Renekton/Ruthless_Predator](https://wiki.leagueoflegends.com/en-us/Template:Data_Renekton/Ruthless_Predator)
  - Source: [Template:Data_Rengar/Savagery](https://wiki.leagueoflegends.com/en-us/Template:Data_Rengar/Savagery)
  - Source: [Template:Data_Wukong/Crushing_Blow](https://wiki.leagueoflegends.com/en-us/Template:Data_Wukong/Crushing_Blow)
- Champion truncation-defect correction:
  - Corrected `Braum` `Glacial Fissure` first-target knockup note from truncated text to source-aligned minimum `0.6s` plus travel-distance-scaled maximum duration semantics.
  - Source: [Template:Data_Braum/Glacial_Fissure](https://wiki.leagueoflegends.com/en-us/Template:Data_Braum/Glacial_Fissure)
- Rune cadence text normalization:
  - Corrected `Lethal Tempo` stack-decay interval text artifacts (`0. 5` -> `0.5`) and synchronized parsed numeric extraction (`0.5`) across flat and split rune structures.
- Rune decimal/value normalization wave:
  - Normalized decimal spacing and corrected parsed numeric/value metadata for `Electrocute`, `Dark Harvest`, `Press the Attack`, and `Lethal Tempo` in both flat and split rune files.
- Runes decimal normalization completion waves:
  - Completed four dedicated normalization waves across all rune trees (`Domination`, `Precision`, `Resolve`, `Sorcery`, `Inspiration`) and cleared the broader decimal-spacing backlog (`28` entries across `27` runes -> `0`).
  - Corrected dependent numeric metadata for affected entries (`numbers_extracted`, and where impacted `value_range` / `scaling` / `formula`) in both flat and split files.
  - Verified no literal backreference placeholder artifacts remain in rune raw text after bulk updates.
- Runes semantic-explicitness decomposition waves:
  - Added explicit `semantic_components` on all multi-branch `per_rank` rune effects (`14/14`) to label branch semantics (melee/ranged, AD/AP, thresholds, cadence windows) instead of relying only on positional value arrays.
  - Synchronized semantic-component additions across flat and split rune files with no parity drift.
- Champion cast-timing fidelity wave:
  - Re-verified `Jinx` (`W`), `Kai'Sa` (`E`), `Yone` (`Q`,`W`), and `Zeri` (`W`) for attack-speed-scaled cast gating, player-visible resolution timing, and branch behavior (terrain/delayed or transform branches where applicable).
- Champion execution-semantics wave (103-106):
  - Re-verified `Nasus` `Spirit Fire` delayed-impact + periodic-tick cadence, `Renekton` `Slice and Dice` recast-gating + move-speed-scaled dash semantics, `Sylas` `Chain Lash` delayed-detonation timing plus `Abscond` stage-window flow, and `Vex` `Looming Darkness` cast-distance-scaled radius behavior.
  - Source: [Template:Data_Nasus/Spirit_Fire](https://wiki.leagueoflegends.com/en-us/Template:Data_Nasus/Spirit_Fire)
  - Source: [Template:Data_Renekton/Slice_and_Dice](https://wiki.leagueoflegends.com/en-us/Template:Data_Renekton/Slice_and_Dice)
  - Source: [Template:Data_Sylas/Chain_Lash](https://wiki.leagueoflegends.com/en-us/Template:Data_Sylas/Chain_Lash)
  - Source: [Template:Data_Sylas/Abscond](https://wiki.leagueoflegends.com/en-us/Template:Data_Sylas/Abscond)
  - Source: [Template:Data_Vex/Looming_Darkness](https://wiki.leagueoflegends.com/en-us/Template:Data_Vex/Looming_Darkness)
- Champion execution-semantics wave (107-108):
  - Re-verified `Olaf` `Reckless Swing` and `Fiora` `Bladework` and encoded explicit execution-semantic fields in champion ability `execution` objects for future runtime consumption.
  - Source: [Template:Data_Olaf/Reckless_Swing](https://wiki.leagueoflegends.com/en-us/Template:Data_Olaf/Reckless_Swing)
  - Source: [Template:Data_Fiora/Bladework](https://wiki.leagueoflegends.com/en-us/Template:Data_Fiora/Bladework)
- Champion execution-semantics waves (111-114):
  - Re-verified and encoded explicit execution-semantic keys for attack-cadence-coupled abilities on `Nasus` (`Siphoning Strike`), `Garen` (`Decisive Strike`), `Jax` (`Empower`), `Renekton` (`Ruthless Predator`), `MonkeyKing` (`Crushing Blow`), `Blitzcrank` (`Power Fist`), `Leona` (`Shield of Daybreak`), and `Vayne` (`Tumble`).
  - Source: [Template:Data_Nasus/Siphoning_Strike](https://wiki.leagueoflegends.com/en-us/Template:Data_Nasus/Siphoning_Strike)
  - Source: [Template:Data_Garen/Decisive_Strike](https://wiki.leagueoflegends.com/en-us/Template:Data_Garen/Decisive_Strike)
  - Source: [Template:Data_Jax/Empower](https://wiki.leagueoflegends.com/en-us/Template:Data_Jax/Empower)
  - Source: [Template:Data_Renekton/Ruthless_Predator](https://wiki.leagueoflegends.com/en-us/Template:Data_Renekton/Ruthless_Predator)
  - Source: [Template:Data_Wukong/Crushing_Blow](https://wiki.leagueoflegends.com/en-us/Template:Data_Wukong/Crushing_Blow)
  - Source: [Template:Data_Blitzcrank/Power_Fist](https://wiki.leagueoflegends.com/en-us/Template:Data_Blitzcrank/Power_Fist)
  - Source: [Template:Data_Leona/Shield_of_Daybreak](https://wiki.leagueoflegends.com/en-us/Template:Data_Leona/Shield_of_Daybreak)
  - Source: [Template:Data_Vayne/Tumble](https://wiki.leagueoflegends.com/en-us/Template:Data_Vayne/Tumble)
- Champion execution-semantics waves (116-119):
  - Re-verified and encoded explicit execution-semantic keys for attack-cadence-coupled abilities on `Camille` (`Precision Protocol`), `Ekko` (`Phase Dive`), `Rengar` (`Savagery`), `XinZhao` (`Three Talon Strike`), `Yorick` (`Last Rites`), `Trundle` (`Chomp`), `Volibear` (`Frenzied Maul`), and `Vi` (`Relentless Force`).
  - Source: [Template:Data_Camille/Precision_Protocol](https://wiki.leagueoflegends.com/en-us/Template:Data_Camille/Precision_Protocol)
  - Source: [Template:Data_Ekko/Phase_Dive](https://wiki.leagueoflegends.com/en-us/Template:Data_Ekko/Phase_Dive)
  - Source: [Template:Data_Rengar/Savagery](https://wiki.leagueoflegends.com/en-us/Template:Data_Rengar/Savagery)
  - Source: [Template:Data_Xin_Zhao/Three_Talon_Strike](https://wiki.leagueoflegends.com/en-us/Template:Data_Xin_Zhao/Three_Talon_Strike)
  - Source: [Template:Data_Yorick/Last_Rites](https://wiki.leagueoflegends.com/en-us/Template:Data_Yorick/Last_Rites)
  - Source: [Template:Data_Trundle/Chomp](https://wiki.leagueoflegends.com/en-us/Template:Data_Trundle/Chomp)
  - Source: [Template:Data_Volibear/Frenzied_Maul](https://wiki.leagueoflegends.com/en-us/Template:Data_Volibear/Frenzied_Maul)
  - Source: [Template:Data_Vi/Relentless_Force](https://wiki.leagueoflegends.com/en-us/Template:Data_Vi/Relentless_Force)
- Item active cooldown completeness wave (109):
  - Re-verified and encoded explicit active cooldown metadata on `Stridebreaker` (`15s`), `Hextech Rocketbelt` (`40s`), and `Redemption` (`90s`) across all active effect branches in structured data.
  - Source: [Stridebreaker](https://wiki.leagueoflegends.com/en-us/Stridebreaker)
  - Source: [Hextech Rocketbelt](https://wiki.leagueoflegends.com/en-us/Hextech_Rocketbelt)
  - Source: [Redemption](https://wiki.leagueoflegends.com/en-us/Redemption)
- Item active cooldown completeness wave (115):
  - Re-verified and encoded explicit active cooldown metadata on `Ravenous Hydra` (`10s`) for `ravenous_crescent_active_physical_damage`.
  - Source: [Ravenous Hydra](https://wiki.leagueoflegends.com/en-us/Ravenous_Hydra)
- Item activation-cadence schema wave (120):
  - Added explicit reusable `activation_cadence` models on all currently tracked no-fixed-cooldown `on_activate` effects (`11/11` across `10` files), separating charge/consumable/single-use/round-limited cadence from fixed-cooldown semantics.
- Champion effect-level truncation normalization wave (120):
  - Re-verified and repaired effect-level timing-note truncation on `XinZhao` `Wind Becomes Lightning` and `Fiora` `Riposte`, and expanded no-regression audits to include `effects[*].context_notes`.
  - Source: [Template:Data_Xin_Zhao/Wind_Becomes_Lightning](https://wiki.leagueoflegends.com/en-us/Template:Data_Xin_Zhao/Wind_Becomes_Lightning)
  - Source: [Template:Data_Fiora/Riposte](https://wiki.leagueoflegends.com/en-us/Template:Data_Fiora/Riposte)
- Rune narrative decimal-normalization wave (110):
  - Normalized decimal-spacing artifacts in rune narrative fields (`wiki_descriptions` and touched `long_desc`) across flat and split structures while preserving structured-value parity.

## Script-Extraction Backlog (From Audit)
- Vladimir defensive/offensive decisions are script-owned, but the engine still executes some Vladimir effect applications directly after script decisions.
- Enemy script-event behavior generation is script-owned, but effect application remains in engine as generic actions (intended architectural boundary).
- Next extraction opportunity: move more effect execution semantics behind script/config interfaces while keeping engine generic.

# Vladimir URF Run Report

- Generated (local): `2026-02-17 05:16:35 -06:00`
- Generated (UTC): `2026-02-17T11:16:35.273804+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **16.4613**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **454.87s / 110,489.1 / 32,548.2 / 31 / 448.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `16.4613`
- Worst-case scenario score: `16.4613`
- Worst-case blend weight: `0.35`
- Final blended objective score: `16.4613`
- survival: weight `0.26` | normalized `0.3791` | contribution `0.0972` | impact `0.59%` | delta vs weight `-25.05pp`
- damage: weight `0.13` | normalized `5.9801` | contribution `0.7667` | impact `4.66%` | delta vs weight `-8.16pp`
- healing: weight `0.08` | normalized `12.0997` | contribution `0.9307` | impact `5.65%` | delta vs weight `-2.04pp`
- enemy_kills: weight `0.51` | normalized `6.2000` | contribution `3.1795` | impact `19.31%` | delta vs weight `-31.97pp`
- invulnerable_seconds: weight `0.03` | normalized `448.0000` | contribution `11.4872` | impact `69.78%` | delta vs weight `+67.22pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `280`
- Unique scored candidates (all search stages): `280`
- Total score requests (all search stages): `280`
- Full evaluations cache hits/misses/waits: `0/280/0`
- Full persistent cache hits/entries: `0/280`
- Candidate keys generated / duplicate-pruned / unique: `1,458/15/1,443`
- Strict candidates seed-scored / remaining / processed: `0/1,443/0`
- Strict non-finite / timeout-skipped: `0/1,443`
- Strict completion: `0.0%`
- Bleed candidates injected: `1,443`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `1.42s`
- Total run time (end-to-end): `110.32s`

- Effective seed: `11699002614220878288`
- Coverage stage (pre-budget): `1.15s`; assets covered `5/181`; seeded candidates unique/raw `15/15`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/1,443` (0.0%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `1.0s`; significant threshold `1.00% of last best score`; significant events `11`; seconds since last significant improvement `1.3`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `2.813e-11%`
- Estimated legal-space coverage (persistent cache): `2.813e-11%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 280 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - coverage_stage: requests `280`, new simulations `280`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Unsealed Spellbook
  - Rune: Hextech Flashtraption
  - Rune: Biscuit Delivery
  - Rune: Jack Of All Trades
  - Rune: Presence of Mind
  - Rune: Legend: Haste
  - Shard 1: attack_speed
  - Shard 2: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Unsealed Spellbook.
  - Vladimir: Applied rune stat effect from Unsealed Spellbook.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied rune stat effect from Presence of Mind.
  - Vladimir: Applied rune stat effect from Legend: Haste.
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Shard 'tenacity' in slot 3 not applicable in current stat model.

## Best Build
- Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff

## Vladimir End Stats (Best Build)
- HP: 4,321.7, Armor: 109.5, MR: 54.7, AP: 377.8, AD: 0.0, Ability Haste: 5,741.0, Move Speed (flat bonus): 6.0, Move Speed (% bonus): 0.0

## Stack Overrides
- Bloodletter's Curse has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.

## Enemy Builds (URF Presets)
- Warwick: Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail
  - Source: https://www.metasrc.com/lol/urf/build/warwick (last checked 2026-02-15)
  - Runes: Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking
  - Shards: attack_speed, movement_speed, tenacity
- Vayne: Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Phantom Dancer, Blade of the Ruined King, Infinity Edge
  - Source: https://www.metasrc.com/lol/urf/build/vayne (last checked 2026-02-15)
  - Runes: Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth
  - Shards: attack_speed, movement_speed, health
- Morgana: Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo
  - Source: https://www.metasrc.com/lol/urf/build/morgana (last checked 2026-02-15)
  - Runes: Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter
  - Shards: ability_haste, movement_speed, health
- Sona: Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap
  - Source: https://www.metasrc.com/lol/urf/build/sona (last checked 2026-02-15)
  - Runes: Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize
  - Shards: ability_haste, movement_speed, health
- Dr. Mundo: Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra
  - Source: https://www.metasrc.com/lol/urf/build/drmundo (last checked 2026-02-15)
  - Runes: Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight
  - Shards: ability_haste, health, tenacity

## Enemy Derived Combat Profiles
- Warwick: HP 3501.0, Armor 191.6, MR 70.9, AD 237.5, AS 2.681 (interval 0.373s), range 125, projectile speed 0, move speed 422.4, desired combat range 130, hit physical 237.5, hit ability 0.0, burst phys/magic/true 0.0/0.0/0.0
- Vayne: HP 2641.5, Armor 110.4, MR 54.7, AD 294.6, AS 6.274 (interval 0.159s), range 550, projectile speed 2000, move speed 396.8, desired combat range 520, hit physical 294.6, hit ability 0.0, burst phys/magic/true 0.0/0.0/0.0
- Morgana: HP 3440.5, Armor 154.8, MR 54.7, AD 122.5, AS 1.613 (interval 0.620s), range 450, projectile speed 1600, move speed 387.6, desired combat range 450, hit physical 122.5, hit ability 0.0, burst phys/magic/true 0.0/0.0/0.0
- Sona: HP 2413.5, Armor 105.8, MR 54.7, AD 106.0, AS 1.851 (interval 0.540s), range 550, projectile speed 1500, move speed 387.6, desired combat range 520, hit physical 106.0, hit ability 0.0, burst phys/magic/true 0.0/0.0/0.0
- Dr. Mundo: HP 6479.8, Armor 192.5, MR 72.7, AD 148.5, AS 1.635 (interval 0.612s), range 125, projectile speed 0, move speed 400.0, desired combat range 140, hit physical 148.5, hit ability 0.0, burst phys/magic/true 0.0/0.0/0.0

## Diverse Top Builds
1. `score 16.4613` (+0.0000 vs top): Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~7,869.8, AP~377.8, timing score -7631.67, total cost 14,550

## Build Order Optimization
1. Cumulative score: `8.09` | Order: Bloodletter's Curse, Celestial Opposition, Void Staff, Actualizer, Stormsurge, Abyssal Mask
   - Stage 1 (level 5): objective `0.993`, time alive `129.57s`, damage `50,594.3`, healing `9,466.3`
   - Stage 2 (level 8): objective `2.562`, time alive `1200.00s`, damage `345,897.1`, healing `90,356.1`
   - Stage 3 (level 11): objective `0.992`, time alive `1200.00s`, damage `307,408.1`, healing `90,382.9`
   - Stage 4 (level 14): objective `0.991`, time alive `1200.00s`, damage `292,070.8`, healing `86,655.8`
   - Stage 5 (level 17): objective `1.548`, time alive `954.20s`, damage `226,732.8`, healing `67,262.3`
   - Stage 6 (level 20): objective `1.000`, time alive `454.87s`, damage `110,489.1`, healing `32,548.2`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Actualizer, Bloodletter's Curse, Celestial Opposition, Stormsurge, Void Staff.
- Most frequent items in selected top set: Abyssal Mask (1/1), Actualizer (1/1), Bloodletter's Curse (1/1), Celestial Opposition (1/1), Stormsurge (1/1), Void Staff (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

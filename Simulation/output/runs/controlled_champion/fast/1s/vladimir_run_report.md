# Vladimir URF Run Report

- Generated (local): `2026-02-18 03:10:51 -06:00`
- Generated (UTC): `2026-02-18T09:10:51.326609+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.6832**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **16.46s / 13,349.5 / 2,146.2 / 2 / 12.50s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.6832`
- Worst-case scenario score: `0.6832`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.6832`
- survival: weight `0.26` | normalized `0.0137` | contribution `0.0035` | impact `0.51%` | delta vs weight `-25.13pp`
- damage: weight `0.13` | normalized `0.7225` | contribution `0.0926` | impact `13.56%` | delta vs weight `+0.74pp`
- healing: weight `0.08` | normalized `0.7979` | contribution `0.0614` | impact `8.98%` | delta vs weight `+1.29pp`
- enemy_kills: weight `0.51` | normalized `0.4000` | contribution `0.2051` | impact `30.03%` | delta vs weight `-21.26pp`
- invulnerable_seconds: weight `0.03` | normalized `12.5000` | contribution `0.3205` | impact `46.92%` | delta vs weight `+44.35pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `1`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `388`
- Unique scored candidates (all search stages): `388`
- Total score requests (all search stages): `8,740`
- Full evaluations cache hits/misses/waits: `0/8,740/0`
- Full persistent cache hits/entries: `0/388`
- Candidate keys generated / duplicate-pruned / unique: `128/0/128`
- Strict candidates seed-scored / remaining / processed: `0/128/0`
- Strict non-finite / timeout-skipped: `0/128`
- Strict completion: `0.0%`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `1.05s`
- Total run time (end-to-end): `2.07s`

- Effective seed: `9790906431087312789`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/128` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `3.898e-11%`
- Estimated legal-space coverage (persistent cache): `3.898e-11%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 388 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `8,740`, new simulations `8,740`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir: none selected.
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.


## Best Build
- Abyssal Mask, Bloodletter's Curse, Guardian Angel, Heartsteel, Warmog's Armor, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 5,996.8, Armor: 204.5, MR: 54.7, AP: 270.1, AD: 55.0, Ability Haste: 30.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

## Stack Overrides
- Bloodletter's Curse has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.
- Heartsteel estimated stacks by level 20: 12.0 (acquired at level 11, reference full-at-20 stack target 20, estimated permanent bonus health: +331.6).

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
1. `score 0.6832` (+0.0000 vs top): Abyssal Mask, Bloodletter's Curse, Guardian Angel, Heartsteel, Warmog's Armor, Zhonya's Hourglass | seed hits: 1/1 (100%) robust | Pareto-front
   - metrics: EHP~14,386.6, AP~279.0, timing score -8888.33, total cost 18,100

## Build Order Optimization
1. Cumulative score: `12.90` | Order: Guardian Angel, Zhonya's Hourglass, Heartsteel, Warmog's Armor, Bloodletter's Curse, Abyssal Mask
   - Stage 1 (level 5): objective `2.069`, time alive `9.04s`, damage `5,017.3`, healing `464.1`
   - Stage 2 (level 8): objective `4.008`, time alive `13.80s`, damage `7,766.5`, healing `899.1`
   - Stage 3 (level 11): objective `1.770`, time alive `16.10s`, damage `9,193.8`, healing `1,242.8`
   - Stage 4 (level 14): objective `1.769`, time alive `16.10s`, damage `10,780.8`, healing `1,627.9`
   - Stage 5 (level 17): objective `1.756`, time alive `16.10s`, damage `12,224.9`, healing `1,866.2`
   - Stage 6 (level 20): objective `1.525`, time alive `16.46s`, damage `13,071.0`, healing `2,073.2`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Bloodletter's Curse, Guardian Angel, Heartsteel, Warmog's Armor, Zhonya's Hourglass.
- Most frequent items in selected top set: Abyssal Mask (1/1), Bloodletter's Curse (1/1), Guardian Angel (1/1), Heartsteel (1/1), Warmog's Armor (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

# Vladimir URF Run Report

- Generated (local): `2026-02-16 16:09:30 -06:00`
- Generated (UTC): `2026-02-16T22:09:30.014048+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Baseline objective score: **0.0822**
- Best objective score: **0.1685**
- Improvement: **+104.98%**
- Baseline time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **3.66s / 3,321.5 / 248.5 / 0 / 2.00s**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **6.60s / 6,078.9 / 782.6 / 0 / 4.00s**
- Baseline cap survivor: **false**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Baseline Build
- Weighted-mean score: `0.0822`
- Worst-case scenario score: `0.0822`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0822`
- survival: weight `0.26` | normalized `0.0031` | contribution `0.0008` | impact `0.95%` | delta vs weight `-24.69pp`
- damage: weight `0.13` | normalized `0.1798` | contribution `0.0230` | impact `28.03%` | delta vs weight `+15.21pp`
- healing: weight `0.08` | normalized `0.0924` | contribution `0.0071` | impact `8.64%` | delta vs weight `+0.95pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `2.0000` | contribution `0.0513` | impact `62.37%` | delta vs weight `+59.81pp`

### Best Build
- Weighted-mean score: `0.1685`
- Worst-case scenario score: `0.1685`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.1685`
- survival: weight `0.26` | normalized `0.0055` | contribution `0.0014` | impact `0.84%` | delta vs weight `-24.80pp`
- damage: weight `0.13` | normalized `0.3290` | contribution `0.0422` | impact `25.03%` | delta vs weight `+12.21pp`
- healing: weight `0.08` | normalized `0.2909` | contribution `0.0224` | impact `13.28%` | delta vs weight `+5.59pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `4.0000` | contribution `0.1026` | impact `60.86%` | delta vs weight `+58.29pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `3,561`
- Unique scored candidates (all search stages): `4,147`
- Total score requests (all search stages): `4,147`
- Full evaluations cache hits/misses/waits: `0/3,561/0`
- Full persistent cache hits/entries: `586/9,360`
- Candidate keys generated / duplicate-pruned / unique: `1,600/0/1,600`
- Strict candidates seed-scored / remaining / processed: `0/1,600/0`
- Strict non-finite / timeout-skipped: `0/1,600`
- Strict completion: `0.0%`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `1.11s`
- Total run time (end-to-end): `1.65s`

- Time budget: `1.0s`; timed_out: `true`; progress: `0/1,600` (0.0%)

- Estimated total legal candidate space: `566,359,639,971,840`
- Estimated legal-space coverage (this run): `7.322e-10%`
- Estimated legal-space coverage (persistent cache): `1.653e-9%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 5663596 candidates in the legal space) and n = 4147 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `4,147`, new simulations `3,561`, persistent cache hits `586`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir: none selected.
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.


## Baseline Build
- none provided

## Best Build
- Abyssal Mask, Ardent Censer, Celestial Opposition, Zaz'Zak's Realmspike, Zeke's Convergence, Zephyr

## Vladimir End Stats (Best Build)
- HP: 3,812.0, Armor: 134.5, MR: 54.7, AP: 79.7, AD: 0.0, Ability Haste: 55.0, Move Speed (flat bonus): 14.0, Move Speed (% bonus): 0.0

## Stack Overrides
- Zephyr has stack-based passive text in item data; currently treated as baseline/implicit unless explicitly modeled.

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
- Warwick: HP 3501.0, Armor 191.6, MR 70.9, AD 237.5, AS 2.681 (interval 0.373s), range 125, projectile speed 0, move speed 422.4, desired combat range 130, hit physical 237.5, hit ability 26.9, burst phys/magic/true 0.0/0.0/0.0
- Vayne: HP 2641.5, Armor 110.4, MR 54.7, AD 294.6, AS 6.274 (interval 0.159s), range 550, projectile speed 2000, move speed 396.8, desired combat range 520, hit physical 294.6, hit ability 27.7, burst phys/magic/true 70.0/0.0/0.0
- Morgana: HP 3440.5, Armor 154.8, MR 54.7, AD 122.5, AS 1.613 (interval 0.620s), range 450, projectile speed 1600, move speed 387.6, desired combat range 450, hit physical 122.5, hit ability 53.2, burst phys/magic/true 0.0/140.0/0.0
- Sona: HP 2413.5, Armor 105.8, MR 54.7, AD 106.0, AS 1.851 (interval 0.540s), range 550, projectile speed 1500, move speed 387.6, desired combat range 520, hit physical 106.0, hit ability 44.3, burst phys/magic/true 0.0/100.0/0.0
- Dr. Mundo: HP 6479.8, Armor 192.5, MR 72.7, AD 148.5, AS 1.635 (interval 0.612s), range 125, projectile speed 0, move speed 400.0, desired combat range 140, hit physical 148.5, hit ability 18.4, burst phys/magic/true 90.0/0.0/0.0

## Diverse Top Builds
1. `score 0.1685` (+0.0000 vs top): Abyssal Mask, Ardent Censer, Celestial Opposition, Zaz'Zak's Realmspike, Zeke's Convergence, Zephyr | seed hits: 1/8 (12%) fragile | Pareto-front
   - metrics: EHP~7,418.2, AP~79.7, timing score -5875.00, total cost 10,350

## Build Order Optimization
1. Cumulative score: `3.71` | Order: Zeke's Convergence, Abyssal Mask, Ardent Censer, Celestial Opposition, Zaz'Zak's Realmspike, Zephyr
   - Stage 1 (level 5): objective `0.490`, time alive `2.16s`, damage `1,261.2`, healing `100.5`
   - Stage 2 (level 8): objective `0.491`, time alive `2.67s`, damage `3,505.0`, healing `181.0`
   - Stage 3 (level 11): objective `0.877`, time alive `6.35s`, damage `6,633.6`, healing `665.1`
   - Stage 4 (level 14): objective `0.875`, time alive `6.40s`, damage `6,687.7`, healing `719.8`
   - Stage 5 (level 17): objective `0.487`, time alive `6.27s`, damage `6,739.0`, healing `773.7`
   - Stage 6 (level 20): objective `0.487`, time alive `6.60s`, damage `6,078.9`, healing `782.6`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Ardent Censer, Celestial Opposition, Zaz'Zak's Realmspike, Zeke's Convergence, Zephyr.
- Most frequent items in selected top set: Abyssal Mask (1/1), Ardent Censer (1/1), Celestial Opposition (1/1), Zaz'Zak's Realmspike (1/1), Zeke's Convergence (1/1), Zephyr (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

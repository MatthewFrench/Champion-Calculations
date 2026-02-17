# Vladimir URF Run Report

- Generated (local): `2026-02-16 20:00:48 -06:00`
- Generated (UTC): `2026-02-17T02:00:48.993097+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Baseline objective score: **0.1531**
- Best objective score: **0.4984**
- Improvement: **+225.49%**
- Baseline time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **6.73s / 4,904.6 / 527.8 / 0 / 4.00s**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **10.00s / 13,633.6 / 1,493.1 / 2 / 6.00s**
- Baseline cap survivor: **false**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Baseline Build
- Weighted-mean score: `0.1531`
- Worst-case scenario score: `0.1531`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.1531`
- survival: weight `0.26` | normalized `0.0056` | contribution `0.0014` | impact `0.94%` | delta vs weight `-24.70pp`
- damage: weight `0.13` | normalized `0.2655` | contribution `0.0340` | impact `22.22%` | delta vs weight `+9.40pp`
- healing: weight `0.08` | normalized `0.1962` | contribution `0.0151` | impact `9.86%` | delta vs weight `+2.16pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `4.0000` | contribution `0.1026` | impact `66.98%` | delta vs weight `+64.42pp`

### Best Build
- Weighted-mean score: `0.4984`
- Worst-case scenario score: `0.4984`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.4984`
- survival: weight `0.26` | normalized `0.0083` | contribution `0.0021` | impact `0.43%` | delta vs weight `-25.21pp`
- damage: weight `0.13` | normalized `0.7379` | contribution `0.0946` | impact `18.98%` | delta vs weight `+6.16pp`
- healing: weight `0.08` | normalized `0.5551` | contribution `0.0427` | impact `8.57%` | delta vs weight `+0.87pp`
- enemy_kills: weight `0.51` | normalized `0.4000` | contribution `0.2051` | impact `41.16%` | delta vs weight `-10.13pp`
- invulnerable_seconds: weight `0.03` | normalized `6.0000` | contribution `0.1538` | impact `30.87%` | delta vs weight `+28.30pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `42,926`
- Unique scored candidates (all search stages): `46,365`
- Total score requests (all search stages): `61,185`
- Full evaluations cache hits/misses/waits: `0/42,926/0`
- Full persistent cache hits/entries: `18,259/46,365`
- Candidate keys generated / duplicate-pruned / unique: `1,600/0/1,600`
- Strict candidates seed-scored / remaining / processed: `0/1,600/0`
- Strict non-finite / timeout-skipped: `0/1,600`
- Strict completion: `0.0%`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `120.42s`
- Total run time (end-to-end): `121.71s`

- Time budget: `120.0s`; timed_out: `true`; progress: `0/1,600` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `4.658e-9%`
- Estimated legal-space coverage (persistent cache): `4.658e-9%`
- Estimated closeness probability (top 0.000001% heuristic): `0.05%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 46365 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `61,185`, new simulations `42,926`, persistent cache hits `18,259`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir: none selected.
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.


## Baseline Build
- none provided

## Best Build
- Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Void Staff

## Vladimir End Stats (Best Build)
- HP: 4,240.0, Armor: 159.5, MR: 54.7, AP: 287.9, AD: 60.0, Ability Haste: 70.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

## Stack Overrides
- Bloodletter's Curse has stack-based passive text in item data; currently treated as baseline/implicit unless explicitly modeled.

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
1. `score 0.4984` (+0.0000 vs top): Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Void Staff | seed hits: 1/8 (12%) fragile | Pareto-front
   - metrics: EHP~8,781.0, AP~287.9, timing score -8471.67, total cost 16,950

## Build Order Optimization
1. Cumulative score: `11.88` | Order: Bloodletter's Curse, Unending Despair, Abyssal Mask, Void Staff, Actualizer, Umbral Glaive
   - Stage 1 (level 5): objective `0.859`, time alive `2.73s`, damage `3,887.1`, healing `152.7`
   - Stage 2 (level 8): objective `2.137`, time alive `6.33s`, damage `7,443.3`, healing `741.8`
   - Stage 3 (level 11): objective `2.301`, time alive `40.99s`, damage `14,250.0`, healing `1,713.6`
   - Stage 4 (level 14): objective `2.507`, time alive `48.51s`, damage `15,874.8`, healing `1,803.4`
   - Stage 5 (level 17): objective `3.077`, time alive `53.97s`, damage `16,945.2`, healing `1,915.3`
   - Stage 6 (level 20): objective `1.000`, time alive `10.00s`, damage `13,633.6`, healing `1,493.1`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Void Staff.
- Most frequent items in selected top set: Abyssal Mask (1/1), Actualizer (1/1), Bloodletter's Curse (1/1), Umbral Glaive (1/1), Unending Despair (1/1), Void Staff (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

# Vladimir URF Run Report

- Generated (local): `2026-02-18 19:41:07 -06:00`
- Generated (UTC): `2026-02-19T01:41:07.153514+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.4078**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **16.10s / 13,472.4 / 3,691.0 / 2 / 10.50s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.4078`
- Worst-case scenario score: `0.4078`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.4078`
- survival: weight `0.26` | normalized `0.0134` | contribution `0.0034` | impact `0.84%` | delta vs weight `-24.80pp`
- damage: weight `0.13` | normalized `0.7292` | contribution `0.0935` | impact `22.92%` | delta vs weight `+10.10pp`
- healing: weight `0.08` | normalized `1.3721` | contribution `0.1055` | impact `25.88%` | delta vs weight `+18.19pp`
- enemy_kills: weight `0.51` | normalized `0.4000` | contribution `0.2051` | impact `50.30%` | delta vs weight `-0.98pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0088` | contribution `0.0002` | impact `0.06%` | delta vs weight `-2.51pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `true` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `3,490,998`
- Unique scored candidates (all search stages): `3,490,998`
- Total score requests (all search stages): `6,932,179`
- Full evaluations cache hits/misses/waits: `125,678/4,427,789/1`
- Full persistent cache hits/entries: `2,378,712/3,501,111`
- Candidate keys generated / duplicate-pruned / unique: `17,729/417/17,312`
- Strict candidates seed-scored / remaining / processed: `12,589/4,723/12,589`
- Strict non-finite / timeout-skipped: `0/4,723`
- Strict completion: `72.7%`
- Strict ordering heuristic (enabled / rune_weight / shard_weight / exploration_promotions): `true` / `0.30` / `0.20` / `1`
- Bleed candidates injected: `1,816`
- Adaptive candidates injected: `2,570`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `648.88s`
- Total run time (end-to-end): `655.26s`

- Effective seed: `17153168798231823016`
- Coverage stage (pre-budget): `6.84s`; assets covered `181/181`; seeded candidates unique/raw `543/543`
- Time budget: `600.0s`; timed_out: `true`; progress: `12,589/17,312` (72.7%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `600.0s`; significant threshold `1.00% of last best score`; significant events `24`; seconds since last significant improvement `522.3`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `3.507e-7%`
- Estimated legal-space coverage (persistent cache): `3.517e-7%`
- Estimated closeness probability (top 0.000001% heuristic): `3.43%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 3490998 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `2,882,041`, new simulations `1,756,784`, persistent cache hits `1,089,256`
  - strategy_elites: requests `2,736,779`, new simulations `1,740,187`, persistent cache hits `950,939`
  - adaptive_search: requests `1,303,247`, new simulations `920,706`, persistent cache hits `338,517`
  - coverage_stage: requests `10,112`, new simulations `10,112`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Aftershock
  - Rune: Demolish
  - Rune: Second Wind
  - Rune: Revitalize
  - Rune: Triumph
  - Rune: Coup de Grace
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Aftershock' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Demolish' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Revitalize' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Coup de Grace' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Second Wind' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
  - Vladimir: Rune 'Triumph' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
- Controlled champion runes with no modeled deterministic/runtime combat effect:
  - Aftershock
  - Demolish
  - Revitalize
  - Coup de Grace

## Best Build
- Bloodletter's Curse, Guardian Angel, Heartsteel, Liandry's Torment, Warmog's Armor, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 6,317.3, Armor: 204.5, MR: 54.7, AP: 337.6, AD: 55.0, Ability Haste: 23.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 2.0

## Stack Overrides
- Bloodletter's Curse has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.
- Heartsteel estimated stacks by level 20: 16.0 (acquired at level 8, reference full-at-20 stack target 20, estimated permanent bonus health: +442.5).
- Liandry's Torment has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.

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
1. `score 0.4078` (+0.0000 vs top): Bloodletter's Curse, Guardian Angel, Heartsteel, Liandry's Torment, Warmog's Armor, Zhonya's Hourglass | seed hits: 1/8 (12%) fragile | Pareto-front
   - metrics: EHP~14,820.5, AP~342.1, timing score -9256.67, total cost 18,450
2. `score 0.4078` (+0.0000 vs top): Guardian Angel, Heartsteel, Liandry's Torment, Rylai's Crystal Scepter, Warmog's Armor, Zhonya's Hourglass | seed hits: 1/8 (12%) fragile
   - metrics: EHP~14,820.5, AP~342.1, timing score -9326.67, total cost 18,150
3. `score 0.4072` (-0.0006 vs top): Guardian Angel, Heartsteel, Liandry's Torment, Morellonomicon, Warmog's Armor, Zhonya's Hourglass | seed hits: 1/8 (12%) fragile | Pareto-front
   - metrics: EHP~14,730.9, AP~350.3, timing score -9414.17, total cost 18,400
4. `score 0.4068` (-0.0010 vs top): Cosmic Drive, Guardian Angel, Heartsteel, Liandry's Torment, Warmog's Armor, Zhonya's Hourglass | seed hits: 1/8 (12%) fragile | Pareto-front
   - metrics: EHP~14,712.6, AP~345.3, timing score -9366.67, total cost 18,550
5. `score 0.3962` (-0.0117 vs top): Dusk and Dawn, Guardian Angel, Heartsteel, Liandry's Torment, Warmog's Armor, Zhonya's Hourglass | seed hits: 1/8 (12%) fragile
   - metrics: EHP~14,712.6, AP~345.3, timing score -9476.67, total cost 18,650

## Build Order Optimization
1. Cumulative score: `634.93` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Liandry's Torment, Bloodletter's Curse
   - Stage 1 (level 5): objective `629.817`, time alive `6.58s`, damage `3,248.4`, healing `380.2`
   - Stage 2 (level 8): objective `0.500`, time alive `7.00s`, damage `3,049.3`, healing `512.7`
   - Stage 3 (level 11): objective `0.874`, time alive `13.32s`, damage `7,863.5`, healing `1,283.0`
   - Stage 4 (level 14): objective `1.365`, time alive `13.55s`, damage `9,555.3`, healing `1,751.4`
   - Stage 5 (level 17): objective `1.368`, time alive `13.56s`, damage `11,949.3`, healing `1,981.5`
   - Stage 6 (level 20): objective `1.004`, time alive `14.06s`, damage `13,305.4`, healing `2,225.2`
2. Cumulative score: `6.19` | Order: Heartsteel, Guardian Angel, Zhonya's Hourglass, Warmog's Armor, Liandry's Torment, Morellonomicon
   - Stage 1 (level 5): objective `0.077`, time alive `0.59s`, damage `1,366.9`, healing `0.0`
   - Stage 2 (level 8): objective `0.491`, time alive `7.00s`, damage `3,092.4`, healing `523.9`
   - Stage 3 (level 11): objective `0.843`, time alive `13.32s`, damage `8,003.5`, healing `1,318.9`
   - Stage 4 (level 14): objective `1.367`, time alive `13.55s`, damage `9,702.8`, healing `1,798.9`
   - Stage 5 (level 17): objective `1.882`, time alive `13.56s`, damage `12,083.1`, healing `2,034.1`
   - Stage 6 (level 20): objective `1.531`, time alive `14.84s`, damage `13,480.4`, healing `2,268.7`
3. Cumulative score: `634.95` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Liandry's Torment, Cosmic Drive
   - Stage 1 (level 5): objective `629.817`, time alive `6.58s`, damage `3,248.4`, healing `380.2`
   - Stage 2 (level 8): objective `0.502`, time alive `7.00s`, damage `3,049.3`, healing `512.7`
   - Stage 3 (level 11): objective `0.889`, time alive `13.32s`, damage `7,863.5`, healing `1,283.0`
   - Stage 4 (level 14): objective `1.367`, time alive `13.55s`, damage `9,555.3`, healing `1,751.4`
   - Stage 5 (level 17): objective `1.370`, time alive `13.56s`, damage `11,949.3`, healing `1,981.5`
   - Stage 6 (level 20): objective `1.004`, time alive `14.06s`, damage `13,275.0`, healing `2,206.5`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Heartsteel, Liandry's Torment, Warmog's Armor, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (5/5), Heartsteel (5/5), Liandry's Torment (5/5), Warmog's Armor (5/5), Zhonya's Hourglass (5/5), Bloodletter's Curse (1/5), Cosmic Drive (1/5), Dusk and Dawn (1/5).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

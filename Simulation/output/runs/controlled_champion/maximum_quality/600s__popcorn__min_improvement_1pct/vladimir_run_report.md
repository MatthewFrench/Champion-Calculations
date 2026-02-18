# Vladimir URF Run Report

- Generated (local): `2026-02-18 04:01:27 -06:00`
- Generated (UTC): `2026-02-18T10:01:27.199633+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.6968**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **18.40s / 14,470.8 / 2,405.5 / 3 / 12.50s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.4811`
- Worst-case scenario score: `0.4811`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.4811`
- survival: weight `0.26` | normalized `0.0153` | contribution `0.0039` | impact `0.82%` | delta vs weight `-24.82pp`
- damage: weight `0.13` | normalized `0.7832` | contribution `0.1004` | impact `20.87%` | delta vs weight `+8.05pp`
- healing: weight `0.08` | normalized `0.8942` | contribution `0.0688` | impact `14.30%` | delta vs weight `+6.61pp`
- enemy_kills: weight `0.51` | normalized `0.6000` | contribution `0.3077` | impact `63.96%` | delta vs weight `+12.68pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0104` | contribution `0.0003` | impact `0.06%` | delta vs weight `-2.51pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `2,260,001`
- Unique scored candidates (all search stages): `2,264,157`
- Total score requests (all search stages): `4,188,070`
- Full evaluations cache hits/misses/waits: `264,930/2,282,143/0`
- Full persistent cache hits/entries: `1,640,997/2,533,657`
- Candidate keys generated / duplicate-pruned / unique: `15,209/11,454/3,755`
- Strict candidates seed-scored / remaining / processed: `1,600/2,155/1,600`
- Strict non-finite / timeout-skipped: `0/2,155`
- Strict completion: `42.6%`
- Bleed candidates injected: `1,866`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `611.88s`
- Total run time (end-to-end): `620.48s`

- Effective seed: `17544274344834918393`
- Coverage stage (pre-budget): `3.52s`; assets covered `181/181`; seeded candidates unique/raw `543/543`
- Time budget: `600.0s`; timed_out: `true`; progress: `1,600/3,755` (42.6%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `600.0s`; significant threshold `1.00% of last best score`; significant events `9`; seconds since last significant improvement `600.8`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `2.275e-7%`
- Estimated legal-space coverage (persistent cache): `2.545e-7%`
- Estimated closeness probability (top 0.000001% heuristic): `2.24%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 2264157 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `2,525,170`, new simulations `1,335,595`, persistent cache hits `1,065,941`
  - strategy_elites: requests `1,652,793`, new simulations `936,441`, persistent cache hits `575,056`
  - coverage_stage: requests `10,107`, new simulations `10,107`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir: none selected.
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.


## Best Build
- Guardian Angel, Heartsteel, Protoplasm Harness, Rylai's Crystal Scepter, Warmog's Armor, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 6,400.0, Armor: 204.5, MR: 54.7, AP: 283.5, AD: 55.0, Ability Haste: 20.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

## Stack Overrides
- Heartsteel estimated stacks by level 20: 16.0 (acquired at level 8, reference full-at-20 stack target 20, estimated permanent bonus health: +466.4).

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
1. `score 0.6968` (+0.0000 vs top): Guardian Angel, Heartsteel, Protoplasm Harness, Rylai's Crystal Scepter, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust
   - metrics: EHP~15,018.3, AP~288.1, timing score -9110.00, total cost 17,650
2. `score 0.6968` (+0.0000 vs top): Bloodletter's Curse, Guardian Angel, Heartsteel, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust | Pareto-front
   - metrics: EHP~15,018.3, AP~288.1, timing score -9081.67, total cost 17,950
3. `score 0.6966` (-0.0002 vs top): Guardian Angel, Heartsteel, Morellonomicon, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust | Pareto-front
   - metrics: EHP~14,928.7, AP~296.3, timing score -9226.67, total cost 17,900
4. `score 0.6963` (-0.0005 vs top): Cosmic Drive, Guardian Angel, Heartsteel, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust | Pareto-front
   - metrics: EHP~14,910.4, AP~291.3, timing score -9191.67, total cost 18,050
5. `score 0.6963` (-0.0005 vs top): Guardian Angel, Heartsteel, Protoplasm Harness, Riftmaker, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust
   - metrics: EHP~14,910.4, AP~291.3, timing score -9285.00, total cost 18,150
6. `score 0.6963` (-0.0005 vs top): Dusk and Dawn, Guardian Angel, Heartsteel, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust
   - metrics: EHP~14,910.4, AP~291.3, timing score -9301.67, total cost 18,150
7. `score 0.6959` (-0.0008 vs top): Guardian Angel, Heartsteel, Protoplasm Harness, Titanic Hydra, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust | Pareto-front
   - metrics: EHP~15,284.9, AP~230.4, timing score -9355.00, total cost 18,350
8. `score 0.6955` (-0.0013 vs top): Guardian Angel, Heartsteel, Hextech Rocketbelt, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass | seed hits: 8/8 (100%) robust | Pareto-front
   - metrics: EHP~14,784.0, AP~289.5, timing score -9140.00, total cost 17,700

## Build Order Optimization
1. Cumulative score: `9.63` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Rylai's Crystal Scepter, Protoplasm Harness
   - Stage 1 (level 5): objective `1.000`, time alive `9.20s`, damage `5,730.3`, healing `464.1`
   - Stage 2 (level 8): objective `1.000`, time alive `9.20s`, damage `6,890.8`, healing `691.6`
   - Stage 3 (level 11): objective `2.196`, time alive `16.10s`, damage `10,015.1`, healing `1,144.5`
   - Stage 4 (level 14): objective `2.228`, time alive `17.00s`, damage `11,605.6`, healing `1,514.2`
   - Stage 5 (level 17): objective `2.208`, time alive `17.00s`, damage `13,104.0`, healing `1,744.7`
   - Stage 6 (level 20): objective `1.000`, time alive `18.40s`, damage `14,370.7`, healing `2,384.5`
2. Cumulative score: `11.73` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Bloodletter's Curse, Protoplasm Harness
   - Stage 1 (level 5): objective `2.770`, time alive `9.20s`, damage `5,730.3`, healing `464.1`
   - Stage 2 (level 8): objective `1.013`, time alive `9.20s`, damage `6,890.8`, healing `691.6`
   - Stage 3 (level 11): objective `2.225`, time alive `16.10s`, damage `10,015.1`, healing `1,144.5`
   - Stage 4 (level 14): objective `2.232`, time alive `17.00s`, damage `11,605.6`, healing `1,514.2`
   - Stage 5 (level 17): objective `2.212`, time alive `17.00s`, damage `13,104.0`, healing `1,744.7`
   - Stage 6 (level 20): objective `1.279`, time alive `18.40s`, damage `14,370.7`, healing `2,384.5`
3. Cumulative score: `9.65` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Morellonomicon, Protoplasm Harness, Warmog's Armor
   - Stage 1 (level 5): objective `1.000`, time alive `9.20s`, damage `5,730.3`, healing `464.1`
   - Stage 2 (level 8): objective `1.000`, time alive `9.20s`, damage `6,890.8`, healing `691.6`
   - Stage 3 (level 11): objective `2.220`, time alive `16.10s`, damage `10,015.1`, healing `1,144.5`
   - Stage 4 (level 14): objective `2.197`, time alive `16.10s`, damage `11,547.5`, healing `1,357.8`
   - Stage 5 (level 17): objective `2.238`, time alive `18.00s`, damage `12,813.2`, healing `1,996.5`
   - Stage 6 (level 20): objective `1.000`, time alive `18.40s`, damage `14,389.0`, healing `2,370.0`
4. Cumulative score: `11.74` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Cosmic Drive, Protoplasm Harness
   - Stage 1 (level 5): objective `2.779`, time alive `9.20s`, damage `5,730.3`, healing `464.1`
   - Stage 2 (level 8): objective `1.015`, time alive `9.20s`, damage `6,890.8`, healing `691.6`
   - Stage 3 (level 11): objective `2.226`, time alive `16.10s`, damage `10,015.1`, healing `1,144.5`
   - Stage 4 (level 14): objective `2.233`, time alive `17.00s`, damage `11,605.6`, healing `1,514.2`
   - Stage 5 (level 17): objective `2.212`, time alive `17.00s`, damage `13,094.4`, healing `1,733.3`
   - Stage 6 (level 20): objective `1.279`, time alive `18.40s`, damage `14,360.7`, healing `2,373.2`
5. Cumulative score: `9.63` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Riftmaker, Protoplasm Harness
   - Stage 1 (level 5): objective `1.000`, time alive `9.20s`, damage `5,730.3`, healing `464.1`
   - Stage 2 (level 8): objective `1.000`, time alive `9.20s`, damage `6,890.8`, healing `691.6`
   - Stage 3 (level 11): objective `2.196`, time alive `16.10s`, damage `10,015.1`, healing `1,144.5`
   - Stage 4 (level 14): objective `2.229`, time alive `17.00s`, damage `11,605.6`, healing `1,514.2`
   - Stage 5 (level 17): objective `2.208`, time alive `17.00s`, damage `13,094.4`, healing `1,733.3`
   - Stage 6 (level 20): objective `1.000`, time alive `18.40s`, damage `14,360.7`, healing `2,373.2`
6. Cumulative score: `11.74` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Dusk and Dawn, Protoplasm Harness
   - Stage 1 (level 5): objective `2.779`, time alive `9.20s`, damage `5,730.3`, healing `464.1`
   - Stage 2 (level 8): objective `1.015`, time alive `9.20s`, damage `6,890.8`, healing `691.6`
   - Stage 3 (level 11): objective `2.226`, time alive `16.10s`, damage `10,015.1`, healing `1,144.5`
   - Stage 4 (level 14): objective `2.233`, time alive `17.00s`, damage `11,605.6`, healing `1,514.2`
   - Stage 5 (level 17): objective `2.212`, time alive `17.00s`, damage `13,094.4`, healing `1,733.3`
   - Stage 6 (level 20): objective `1.279`, time alive `18.40s`, damage `14,360.7`, healing `2,373.2`
7. Cumulative score: `9.35` | Order: Heartsteel, Zhonya's Hourglass, Guardian Angel, Warmog's Armor, Protoplasm Harness, Titanic Hydra
   - Stage 1 (level 5): objective `0.174`, time alive `2.10s`, damage `3,817.5`, healing `145.5`
   - Stage 2 (level 8): objective `1.471`, time alive `7.00s`, damage `7,938.8`, healing `738.8`
   - Stage 3 (level 11): objective `2.198`, time alive `16.10s`, damage `10,073.4`, healing `1,157.8`
   - Stage 4 (level 14): objective `2.241`, time alive `17.00s`, damage `11,683.3`, healing `1,531.9`
   - Stage 5 (level 17): objective `2.266`, time alive `18.40s`, damage `12,969.4`, healing `2,170.0`
   - Stage 6 (level 20): objective `1.002`, time alive `17.00s`, damage `14,177.9`, healing `2,480.6`
8. Cumulative score: `9.66` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Warmog's Armor, Hextech Rocketbelt, Protoplasm Harness
   - Stage 1 (level 5): objective `1.000`, time alive `9.20s`, damage `5,730.3`, healing `464.1`
   - Stage 2 (level 8): objective `1.000`, time alive `9.20s`, damage `6,890.8`, healing `691.6`
   - Stage 3 (level 11): objective `2.224`, time alive `16.10s`, damage `10,015.1`, healing `1,144.5`
   - Stage 4 (level 14): objective `2.231`, time alive `17.00s`, damage `11,605.6`, healing `1,514.2`
   - Stage 5 (level 17): objective `2.209`, time alive `17.00s`, damage `13,055.6`, healing `1,725.2`
   - Stage 6 (level 20): objective `1.000`, time alive `18.40s`, damage `14,322.5`, healing `2,365.2`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Heartsteel, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (8/8), Heartsteel (8/8), Protoplasm Harness (8/8), Warmog's Armor (8/8), Zhonya's Hourglass (8/8), Bloodletter's Curse (1/8), Cosmic Drive (1/8), Dusk and Dawn (1/8).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

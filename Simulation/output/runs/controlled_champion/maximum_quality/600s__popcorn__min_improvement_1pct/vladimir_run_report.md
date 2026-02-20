# Vladimir URF Run Report

- Generated (local): `2026-02-19 15:15:59 -06:00`
- Generated (UTC): `2026-02-19T21:15:59.639615+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.1604**
- Best outcome:
  - Time alive: **14.06s**
  - Damage dealt: **13,025.9**
  - Healing done: **2,336.5**
  - Enemy kills: **0**
  - Invulnerable seconds: **10.50s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.1604`
- Worst-case scenario score: `0.1604`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.1604`
- survival: weight `0.26` | normalized `0.0117` | contribution `0.0030` | impact `1.87%` | delta vs weight `-23.77pp`
- damage: weight `0.13` | normalized `0.7050` | contribution `0.0904` | impact `56.34%` | delta vs weight `+43.52pp`
- healing: weight `0.08` | normalized `0.8686` | contribution `0.0668` | impact `41.65%` | delta vs weight `+33.95pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0088` | contribution `0.0002` | impact `0.14%` | delta vs weight `-2.42pp`

## Rune Proc Telemetry (Best Trace)
- Summon Aery:
  - Procs: `3`
  - Attempts: `17`
  - Eligible: `3`
  - Proc rate (vs attempts): `17.6%`
  - Proc rate (vs eligible): `100.0%`
  - Bonus damage: `278.57` (2.14% share)
  - Bonus healing: `0.00` (0.00% share)
  - Sources:
    - ability:
      - Procs: `3`
      - Attempts: `17`
      - Eligible: `3`
      - Proc rate (vs attempts): `17.6%`
      - Proc rate (vs eligible): `100.0%`
      - Bonus damage: `278.57`
      - Bonus healing: `0.00`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout:
  - Candidates: `1`
  - Finalists: `1`
- Ensemble seeds: `8`
- Parallelism:
  - Threads: `9`
  - Seed orchestration parallel: `true`
  - Portfolio parallel: `true`
  - Strategy-elites parallel: `true`
- Objective weights:
  - survival: `0.26`
  - damage: `0.13`
  - healing: `0.08`
  - enemy_kills: `0.51`
  - invulnerable_seconds: `0.03`
- Simulations executed (new full combat runs): `2,827`
- Unique scored candidates (all search stages): `2,827`
- Total score requests (all search stages): `4,575,695`
- In-memory full-evaluation cache:
  - Hits: `4,571,776`
  - Misses: `3,919`
  - Waits: `142`
- Candidate key generation:
  - Generated: `15,010`
  - Duplicate-pruned: `12,831`
  - Unique: `2,179`
- Strict candidate progression:
  - Seed-scored: `1,603`
  - Remaining: `576`
  - Processed: `2,179`
- Strict stage:
  - Non-finite: `0`
  - Timeout-skipped: `0`
  - Completion: `100.0%`
- Strict ordering heuristic:
  - Enabled: `true`
  - Rune signal weight: `0.30`
  - Shard signal weight: `0.20`
  - Exploration promotions: `1`
- Bleed candidates injected: `872`
- Adaptive candidates injected: `1,282`
- Seed-best stats:
  - Mean: `0.16`
  - Stddev: `0.000`
- Search elapsed time: `8.16s`
- Total run time (end-to-end): `9.95s`

- Effective seed: `7733817158969335486`
- Unmodeled rune gate:
  - Hard gate: `true`
  - Penalty per rune: `0.0000`
  - Rejected: `0`
  - Penalized: `0`
- Unmodeled item-effect gate:
  - Hard gate: `true`
  - Penalty per item: `0.0000`
  - Rejected: `0`
  - Penalized: `0`
- Coverage stage (pre-budget):
  - Elapsed: `2.02s`
  - Assets covered: `22/28`
  - Seeded candidates (unique/raw): `56/66`
- Coverage warning: Coverage incomplete: touched 22/28 assets; coverage stage could not produce finite candidates for at least one locked asset. Continuing search in degraded coverage mode.
- Time budget:
  - Budget: `600.0s`
  - Timed out: `false`
  - Progress: `2,179/2,179` (100.0%) (budget starts after pre-budget coverage stage)

- Popcorn mode:
  - Window: `600.0s`
  - Significant threshold: `1.00% of last best score`
  - Significant events: `5`
  - Seconds since last significant improvement: `8.0`

- Estimated total legal candidate space: `33,264`
- Estimated legal-space coverage (this run): `8.498677%`
- Estimated closeness probability (top 0.000001% heuristic): `8.15%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.003006253% (about top 1 candidates in the legal space) and n = 2827 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `1,151,850`, new simulations `3,021`
  - coverage_stage: requests `1,170`, new simulations `855`
  - adaptive_search: requests `2,286,243`, new simulations `35`
  - strict_full_ranking: requests `576`, new simulations `6`
  - strategy_elites: requests `1,135,856`, new simulations `2`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Summon Aery
  - Rune: Nimbus Cloak
  - Rune: Celerity
  - Rune: Gathering Storm
  - Rune: Magical Footwear
  - Rune: Jack Of All Trades
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Nimbus Cloak.
  - Vladimir: Applied rune stat effect from Celerity.
  - Vladimir: Applied rune stat effect from Celerity.
  - Vladimir: Applied rune stat effect from Magical Footwear.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Summon Aery' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
  - Vladimir: Rune 'Gathering Storm' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.

## Best Build
- Guardian Angel, Heartsteel, Liandry's Torment, Luden's Echo, Protoplasm Harness, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 5,512.8, Armor: 204.5, MR: 54.7, AP: 346.1, AD: 55.0, Ability Haste: 38.0, Move Speed (flat bonus): 10.0, Move Speed (% bonus): 25.0

## Stack Overrides
- Heartsteel estimated stacks by level 20: 16.0 (acquired at level 8, reference full-at-20 stack target 20, estimated permanent bonus health: +378.8).
- Liandry's Torment has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.
- Luden's Echo has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.

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
1. `score 0.1604` (+0.0000 vs top): Guardian Angel, Heartsteel, Liandry's Torment, Luden's Echo, Protoplasm Harness, Zhonya's Hourglass | seed hits: 8/8 (100%) robust | Pareto-front
   - metrics: EHP~12,935.0, AP~350.0, timing score -9199.17, total cost 17,700
2. `score 0.1572` (-0.0032 vs top): Guardian Angel, Heartsteel, Liandry's Torment, Protoplasm Harness, Void Staff, Zhonya's Hourglass | seed hits: 8/8 (100%) robust
   - metrics: EHP~12,916.7, AP~345.0, timing score -9261.67, total cost 17,950

## Build Order Optimization
1. Cumulative score: `4.17` | Order: Guardian Angel, Heartsteel, Protoplasm Harness, Zhonya's Hourglass, Liandry's Torment, Luden's Echo
   - Stage 1 (level 5): objective `0.487`, time alive `6.58s`, damage `3,262.0`, healing `380.9`
   - Stage 2 (level 8): objective `0.487`, time alive `7.00s`, damage `3,056.7`, healing `513.5`
   - Stage 3 (level 11): objective `0.521`, time alive `8.20s`, damage `4,143.2`, healing `934.9`
   - Stage 4 (level 14): objective `0.862`, time alive `13.80s`, damage `8,886.3`, healing `1,896.4`
   - Stage 5 (level 17): objective `1.322`, time alive `13.80s`, damage `11,390.8`, healing `2,128.7`
   - Stage 6 (level 20): objective `0.487`, time alive `14.06s`, damage `12,661.7`, healing `2,257.8`
2. Cumulative score: `4.11` | Order: Heartsteel, Guardian Angel, Liandry's Torment, Zhonya's Hourglass, Protoplasm Harness, Void Staff
   - Stage 1 (level 5): objective `0.000`, time alive `0.00s`, damage `0.0`, healing `0.0`
   - Stage 2 (level 8): objective `0.491`, time alive `7.00s`, damage `3,099.8`, healing `524.7`
   - Stage 3 (level 11): objective `0.490`, time alive `7.77s`, damage `5,186.2`, healing `606.6`
   - Stage 4 (level 14): objective `1.280`, time alive `13.55s`, damage `10,399.8`, healing `1,556.4`
   - Stage 5 (level 17): objective `1.357`, time alive `13.80s`, damage `11,542.7`, healing `2,177.7`
   - Stage 6 (level 20): objective `0.491`, time alive `14.06s`, damage `12,575.8`, healing `2,267.8`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Heartsteel, Liandry's Torment, Protoplasm Harness, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (2/2), Heartsteel (2/2), Liandry's Torment (2/2), Protoplasm Harness (2/2), Zhonya's Hourglass (2/2), Luden's Echo (1/2), Void Staff (1/2).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

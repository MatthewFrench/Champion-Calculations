# Vladimir URF Run Report

- Generated (local): `2026-02-19 13:54:21 -06:00`
- Generated (UTC): `2026-02-19T19:54:21.097416+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.1572**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **14.06s / 12,738.6 / 2,294.9 / 0 / 10.50s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.1572`
- Worst-case scenario score: `0.1572`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.1572`
- survival: weight `0.26` | normalized `0.0117` | contribution `0.0030` | impact `1.91%` | delta vs weight `-23.73pp`
- damage: weight `0.13` | normalized `0.6895` | contribution `0.0884` | impact `56.21%` | delta vs weight `+43.39pp`
- healing: weight `0.08` | normalized `0.8531` | contribution `0.0656` | impact `41.73%` | delta vs weight `+34.04pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0088` | contribution `0.0002` | impact `0.14%` | delta vs weight `-2.42pp`

## Rune Proc Telemetry (Best Trace)
- Summon Aery: procs `3` / attempts `17` / eligible `3` (proc/attempt 17.6%, proc/eligible 100.0%), bonus damage `277.07` (2.18% share), bonus healing `0.00` (0.00% share)
  - sources: ability (procs 3, attempts 17, eligible 3, proc/attempt 17.6%, proc/eligible 100.0%, damage 277.07, healing 0.00)

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `true` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `0`
- Unique scored candidates (all search stages): `3,420,746`
- Total score requests (all search stages): `7,043,109`
- Full evaluations cache hits/misses/waits: `2,638,674/4,404,275/8`
- Full persistent cache hits/entries: `160/100`
- Candidate keys generated / duplicate-pruned / unique: `17,776/231/17,545`
- Strict candidates seed-scored / remaining / processed: `0/17,545/17,545`
- Strict non-finite / timeout-skipped: `17,485/0`
- Strict completion: `100.0%`
- Strict ordering heuristic (enabled / rune_weight / shard_weight / exploration_promotions): `true` / `0.30` / `0.20` / `1`
- Bleed candidates injected: `1,751`
- Adaptive candidates injected: `3,225`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `108.06s`
- Total run time (end-to-end): `108.91s`

- Effective seed: `17153168798231823016`
- Unmodeled rune gate (hard_gate / penalty_per_rune / rejected / penalized): `true` / `0.0000` / `4,387,419` / `0`
- Unmodeled item-effect gate (hard_gate / penalty_per_item / rejected / penalized): `true` / `0.0000` / `16,556` / `0`
- Coverage stage (pre-budget): `4.98s`; assets covered `0/181`; seeded candidates unique/raw `0/0`
- Coverage warning: Coverage incomplete: touched 0/181 assets; coverage stage could not produce finite candidates for at least one locked asset. Continuing search in degraded coverage mode.
- Time budget: `600.0s`; timed_out: `false`; progress: `17,545/17,545` (100.0%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `600.0s`; significant threshold `1.00% of last best score`; significant events `7`; seconds since last significant improvement `45.6`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `3.436e-7%`
- Estimated legal-space coverage (persistent cache): `1.005e-11%`
- Estimated closeness probability (top 0.000001% heuristic): `3.36%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 3420746 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - strategy_elites: requests `2,218,396`, new simulations `300`, persistent cache hits `100`
  - adaptive_search: requests `2,473,969`, new simulations `0`, persistent cache hits `0`
  - seed_search:portfolio: requests `2,323,087`, new simulations `0`, persistent cache hits `0`
  - strict_full_ranking: requests `17,545`, new simulations `0`, persistent cache hits `60`
  - coverage_stage: requests `10,112`, new simulations `0`, persistent cache hits `0`

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
  - Shard 1: attack_speed
  - Shard 2: health
  - Shard 3: tenacity
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Nimbus Cloak.
  - Vladimir: Applied rune stat effect from Celerity.
  - Vladimir: Applied rune stat effect from Celerity.
  - Vladimir: Applied rune stat effect from Magical Footwear.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
  - Vladimir: Applied shard 'tenacity' in slot 3.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Summon Aery' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
  - Vladimir: Rune 'Gathering Storm' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.

## Best Build
- Guardian Angel, Heartsteel, Liandry's Torment, Protoplasm Harness, Void Staff, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 5,504.8, Armor: 204.5, MR: 54.7, AP: 341.1, AD: 55.0, Ability Haste: 20.0, Move Speed (flat bonus): 10.0, Move Speed (% bonus): 23.0

## Stack Overrides
- Heartsteel estimated stacks by level 20: 16.0 (acquired at level 8, reference full-at-20 stack target 20, estimated permanent bonus health: +378.8).
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
1. `score 0.1572` (+0.0000 vs top): Guardian Angel, Heartsteel, Liandry's Torment, Protoplasm Harness, Void Staff, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~12,916.7, AP~345.0, timing score -9261.67, total cost 17,950

## Build Order Optimization
1. Cumulative score: `4.58` | Order: Guardian Angel, Heartsteel, Liandry's Torment, Zhonya's Hourglass, Protoplasm Harness, Void Staff
   - Stage 1 (level 5): objective `0.487`, time alive `6.58s`, damage `3,262.0`, healing `380.9`
   - Stage 2 (level 8): objective `0.487`, time alive `7.00s`, damage `3,056.7`, healing `513.5`
   - Stage 3 (level 11): objective `0.487`, time alive `7.77s`, damage `5,128.8`, healing `593.6`
   - Stage 4 (level 14): objective `1.285`, time alive `13.80s`, damage `10,440.8`, healing `1,515.2`
   - Stage 5 (level 17): objective `1.347`, time alive `13.80s`, damage `11,578.0`, healing `2,128.7`
   - Stage 6 (level 20): objective `0.487`, time alive `14.06s`, damage `12,375.7`, healing `2,216.5`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Heartsteel, Liandry's Torment, Protoplasm Harness, Void Staff, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (1/1), Heartsteel (1/1), Liandry's Torment (1/1), Protoplasm Harness (1/1), Void Staff (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

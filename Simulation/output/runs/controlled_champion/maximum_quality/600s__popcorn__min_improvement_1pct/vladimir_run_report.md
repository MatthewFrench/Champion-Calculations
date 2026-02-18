# Vladimir URF Run Report

- Generated (local): `2026-02-18 04:43:18 -06:00`
- Generated (UTC): `2026-02-18T10:43:18.047297+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **44.9850**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **17.00s / 13,374.5 / 2,729.8 / 2 / 12.50s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.3799`
- Worst-case scenario score: `0.3799`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.3799`
- survival: weight `0.26` | normalized `0.0142` | contribution `0.0036` | impact `0.96%` | delta vs weight `-24.68pp`
- damage: weight `0.13` | normalized `0.7239` | contribution `0.0928` | impact `24.43%` | delta vs weight `+11.61pp`
- healing: weight `0.08` | normalized `1.0148` | contribution `0.0781` | impact `20.55%` | delta vs weight `+12.86pp`
- enemy_kills: weight `0.51` | normalized `0.4000` | contribution `0.2051` | impact `54.00%` | delta vs weight `+2.71pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0104` | contribution `0.0003` | impact `0.07%` | delta vs weight `-2.49pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `true` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `4,458,913`
- Unique scored candidates (all search stages): `4,461,684`
- Total score requests (all search stages): `8,140,627`
- Full evaluations cache hits/misses/waits: `317,946/4,486,655/161`
- Full persistent cache hits/entries: `3,336,026/7,012,786`
- Candidate keys generated / duplicate-pruned / unique: `18,477/601/17,876`
- Strict candidates seed-scored / remaining / processed: `12,514/5,362/17,876`
- Strict non-finite / timeout-skipped: `0/0`
- Strict completion: `100.0%`
- Bleed candidates injected: `1,849`
- Adaptive candidates injected: `3,285`
- Seed-best mean/stddev: `0.53` / `0.006`
- Search elapsed time: `544.09s`
- Total run time (end-to-end): `545.29s`

- Effective seed: `15540744735400828093`
- Coverage stage (pre-budget): `4.48s`; assets covered `181/181`; seeded candidates unique/raw `543/543`
- Time budget: `600.0s`; timed_out: `false`; progress: `17,876/17,876` (100.0%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `600.0s`; significant threshold `1.00% of last best score`; significant events `15`; seconds since last significant improvement `216.0`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `4.482e-7%`
- Estimated legal-space coverage (persistent cache): `7.045e-7%`
- Estimated closeness probability (top 0.000001% heuristic): `4.36%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 4461684 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - adaptive_search: requests `3,148,468`, new simulations `1,804,282`, persistent cache hits `1,291,200`
  - seed_search:portfolio: requests `2,534,336`, new simulations `1,347,461`, persistent cache hits `1,063,211`
  - strategy_elites: requests `2,442,358`, new simulations `1,323,400`, persistent cache hits `977,662`
  - coverage_stage: requests `10,103`, new simulations `10,103`, persistent cache hits `0`
  - strict_full_ranking: requests `5,362`, new simulations `1,409`, persistent cache hits `3,953`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Unsealed Spellbook
  - Rune: Magical Footwear
  - Rune: Biscuit Delivery
  - Rune: Approach Velocity
  - Rune: Triumph
  - Rune: Legend: Bloodline
  - Shard 1: attack_speed
  - Shard 2: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Magical Footwear.
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Shard 'tenacity' in slot 3 not applicable in current stat model.
  - Vladimir: Rune 'Triumph' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.

## Best Build
- Bloodletter's Curse, Guardian Angel, Kaenic Rookern, Overlord's Bloodmail, Warmog's Armor, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 5,446.5, Armor: 204.5, MR: 54.7, AP: 252.0, AD: 85.0, Ability Haste: 15.0, Move Speed (flat bonus): 10.0, Move Speed (% bonus): 0.0

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
1. `score 44.9850` (+0.0000 vs top): Bloodletter's Curse, Guardian Angel, Kaenic Rookern, Overlord's Bloodmail, Warmog's Armor, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~12,196.4, AP~247.6, timing score -9318.33, total cost 18,650

## Build Order Optimization
1. Cumulative score: `11.68` | Order: Guardian Angel, Zhonya's Hourglass, Bloodletter's Curse, Overlord's Bloodmail, Kaenic Rookern, Warmog's Armor
   - Stage 1 (level 5): objective `2.765`, time alive `9.20s`, damage `5,853.8`, healing `497.2`
   - Stage 2 (level 8): objective `2.225`, time alive `13.80s`, damage `8,606.5`, healing `831.0`
   - Stage 3 (level 11): objective `2.238`, time alive `16.10s`, damage `10,078.4`, healing `1,036.1`
   - Stage 4 (level 14): objective `1.734`, time alive `16.10s`, damage `11,170.0`, healing `1,311.7`
   - Stage 5 (level 17): objective `1.720`, time alive `16.10s`, damage `11,950.4`, healing `1,516.7`
   - Stage 6 (level 20): objective `1.000`, time alive `16.10s`, damage `13,374.5`, healing `1,858.3`

## Deeper Insights
- Common core across all selected top builds: Bloodletter's Curse, Guardian Angel, Kaenic Rookern, Overlord's Bloodmail, Warmog's Armor, Zhonya's Hourglass.
- Most frequent items in selected top set: Bloodletter's Curse (1/1), Guardian Angel (1/1), Kaenic Rookern (1/1), Overlord's Bloodmail (1/1), Warmog's Armor (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

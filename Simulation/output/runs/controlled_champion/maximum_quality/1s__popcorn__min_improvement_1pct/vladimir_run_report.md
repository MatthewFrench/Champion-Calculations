# Vladimir URF Run Report

- Generated (local): `2026-02-18 19:30:02 -06:00`
- Generated (UTC): `2026-02-19T01:30:02.750178+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.0134**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **1.55s / 1,880.6 / 0.0 / 0 / 0.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0134`
- Worst-case scenario score: `0.0134`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0134`
- survival: weight `0.26` | normalized `0.0013` | contribution `0.0003` | impact `2.47%` | delta vs weight `-23.17pp`
- damage: weight `0.13` | normalized `0.1018` | contribution `0.0130` | impact `97.53%` | delta vs weight `+84.71pp`
- healing: weight `0.08` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-7.69pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-2.56pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `true` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `10,113`
- Unique scored candidates (all search stages): `10,113`
- Total score requests (all search stages): `10,113`
- Full evaluations cache hits/misses/waits: `0/10,113/0`
- Full persistent cache hits/entries: `0/10,113`
- Candidate keys generated / duplicate-pruned / unique: `2,408/421/1,987`
- Strict candidates seed-scored / remaining / processed: `0/1,987/0`
- Strict non-finite / timeout-skipped: `0/1,987`
- Strict completion: `0.0%`
- Strict ordering heuristic (enabled / rune_weight / shard_weight / exploration_promotions): `true` / `0.30` / `0.20` / `1`
- Bleed candidates injected: `1,865`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `11.03s`
- Total run time (end-to-end): `11.41s`

- Effective seed: `5795014477735195376`
- Coverage stage (pre-budget): `10.55s`; assets covered `181/181`; seeded candidates unique/raw `543/543`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/1,987` (0.0%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `1.0s`; significant threshold `1.00% of last best score`; significant events `7`; seconds since last significant improvement `7.2`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `1.016e-9%`
- Estimated legal-space coverage (persistent cache): `1.016e-9%`
- Estimated closeness probability (top 0.000001% heuristic): `0.01%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 10113 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - coverage_stage: requests `10,113`, new simulations `10,113`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: First Strike
  - Rune: Hextech Flashtraption
  - Rune: Triple Tonic
  - Rune: Approach Velocity
  - Rune: Triumph
  - Rune: Legend: Bloodline
  - Shard 1: attack_speed
  - Shard 2: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'First Strike' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Hextech Flashtraption' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Triple Tonic' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Approach Velocity' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Legend: Bloodline' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Shard 'tenacity' in slot 3 not applicable in current stat model.
  - Vladimir: Rune 'Triumph' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
- Controlled champion runes with no modeled deterministic/runtime combat effect:
  - First Strike
  - Hextech Flashtraption
  - Triple Tonic
  - Approach Velocity
  - Legend: Bloodline

## Best Build
- Abyssal Mask, Actualizer, Bloodletter's Curse, Morellonomicon, Rapid Firecannon, Void Staff

## Vladimir End Stats (Best Build)
- HP: 4,444.5, Armor: 109.5, MR: 54.7, AP: 365.7, AD: 0.0, Ability Haste: 55.0, Move Speed (flat bonus): 4.0, Move Speed (% bonus): 0.0

## Stack Overrides
- Bloodletter's Curse has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.
- Rapid Firecannon has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.

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
1. `score 0.0134` (+0.0000 vs top): Abyssal Mask, Actualizer, Bloodletter's Curse, Morellonomicon, Rapid Firecannon, Void Staff | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~8,093.4, AP~365.7, timing score -8444.17, total cost 16,850

## Build Order Optimization
1. Cumulative score: `1.99` | Order: Morellonomicon, Void Staff, Bloodletter's Curse, Actualizer, Abyssal Mask, Rapid Firecannon
   - Stage 1 (level 5): objective `0.000`, time alive `0.00s`, damage `0.0`, healing `0.0`
   - Stage 2 (level 8): objective `0.402`, time alive `0.58s`, damage `1,694.7`, healing `0.0`
   - Stage 3 (level 11): objective `0.406`, time alive `1.03s`, damage `1,824.9`, healing `0.0`
   - Stage 4 (level 14): objective `0.398`, time alive `1.28s`, damage `1,974.3`, healing `0.0`
   - Stage 5 (level 17): objective `0.400`, time alive `1.58s`, damage `1,938.8`, healing `0.0`
   - Stage 6 (level 20): objective `0.385`, time alive `1.55s`, damage `1,880.6`, healing `0.0`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Actualizer, Bloodletter's Curse, Morellonomicon, Rapid Firecannon, Void Staff.
- Most frequent items in selected top set: Abyssal Mask (1/1), Actualizer (1/1), Bloodletter's Curse (1/1), Morellonomicon (1/1), Rapid Firecannon (1/1), Void Staff (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

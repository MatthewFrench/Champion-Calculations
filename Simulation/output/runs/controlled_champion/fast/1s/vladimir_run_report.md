# Vladimir URF Run Report

- Generated (local): `2026-02-18 19:29:54 -06:00`
- Generated (UTC): `2026-02-19T01:29:54.011931+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.0124**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **1.55s / 1,743.0 / 0.0 / 0 / 0.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0124`
- Worst-case scenario score: `0.0124`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0124`
- survival: weight `0.26` | normalized `0.0013` | contribution `0.0003` | impact `2.66%` | delta vs weight `-22.98pp`
- damage: weight `0.13` | normalized `0.0943` | contribution `0.0121` | impact `97.34%` | delta vs weight `+84.52pp`
- healing: weight `0.08` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-7.69pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-2.56pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `1`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `false` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `902`
- Unique scored candidates (all search stages): `902`
- Total score requests (all search stages): `31,957`
- Full evaluations cache hits/misses/waits: `37/1,276/0`
- Full persistent cache hits/entries: `308/902`
- Candidate keys generated / duplicate-pruned / unique: `128/0/128`
- Strict candidates seed-scored / remaining / processed: `0/128/0`
- Strict non-finite / timeout-skipped: `0/128`
- Strict completion: `0.0%`
- Strict ordering heuristic (enabled / rune_weight / shard_weight / exploration_promotions): `true` / `0.30` / `0.20` / `1`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `2.36s`
- Total run time (end-to-end): `2.67s`

- Effective seed: `5537833927140705212`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/128` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `9.061e-11%`
- Estimated legal-space coverage (persistent cache): `9.061e-11%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 902 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `31,957`, new simulations `1,276`, persistent cache hits `308`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Aftershock
  - Rune: Demolish
  - Rune: Bone Plating
  - Rune: Overgrowth
  - Rune: Triumph
  - Rune: Legend: Haste
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Legend: Haste.
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Aftershock' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Demolish' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Bone Plating' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Overgrowth' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Triumph' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
- Controlled champion runes with no modeled deterministic/runtime combat effect:
  - Aftershock
  - Demolish
  - Bone Plating
  - Overgrowth

## Best Build
- Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 4,390.5, Armor: 209.5, MR: 54.7, AP: 302.4, AD: 60.0, Ability Haste: 79.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 2.0

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
1. `score 0.0124` (+0.0000 vs top): Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Zhonya's Hourglass | seed hits: 1/1 (100%) robust | Pareto-front
   - metrics: EHP~10,190.3, AP~302.4, timing score -8538.33, total cost 17,200

## Build Order Optimization
1. Cumulative score: `2.17` | Order: Bloodletter's Curse, Unending Despair, Zhonya's Hourglass, Actualizer, Abyssal Mask, Umbral Glaive
   - Stage 1 (level 5): objective `0.000`, time alive `0.00s`, damage `0.0`, healing `0.0`
   - Stage 2 (level 8): objective `0.434`, time alive `0.69s`, damage `1,472.0`, healing `0.0`
   - Stage 3 (level 11): objective `0.416`, time alive `1.11s`, damage `1,673.7`, healing `0.0`
   - Stage 4 (level 14): objective `0.529`, time alive `1.61s`, damage `1,827.9`, healing `0.0`
   - Stage 5 (level 17): objective `0.404`, time alive `1.58s`, damage `1,797.0`, healing `0.0`
   - Stage 6 (level 20): objective `0.385`, time alive `1.55s`, damage `1,743.0`, healing `0.0`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Zhonya's Hourglass.
- Most frequent items in selected top set: Abyssal Mask (1/1), Actualizer (1/1), Bloodletter's Curse (1/1), Umbral Glaive (1/1), Unending Despair (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

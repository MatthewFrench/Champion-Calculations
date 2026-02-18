# Vladimir URF Run Report

- Generated (local): `2026-02-18 04:30:00 -06:00`
- Generated (UTC): `2026-02-18T10:30:00.403669+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.0461**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **3.66s / 5,093.7 / 347.8 / 0 / 2.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0461`
- Worst-case scenario score: `0.0461`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0461`
- survival: weight `0.26` | normalized `0.0031` | contribution `0.0008` | impact `1.70%` | delta vs weight `-23.94pp`
- damage: weight `0.13` | normalized `0.2757` | contribution `0.0353` | impact `76.64%` | delta vs weight `+63.82pp`
- healing: weight `0.08` | normalized `0.1293` | contribution `0.0099` | impact `21.57%` | delta vs weight `+13.87pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0017` | contribution `0.0000` | impact `0.09%` | delta vs weight `-2.47pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `1`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `false` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `632`
- Unique scored candidates (all search stages): `1,869`
- Total score requests (all search stages): `7,676`
- Full evaluations cache hits/misses/waits: `0/6,138/0`
- Full persistent cache hits/entries: `1,538/12,152`
- Candidate keys generated / duplicate-pruned / unique: `92/0/92`
- Strict candidates seed-scored / remaining / processed: `0/92/0`
- Strict non-finite / timeout-skipped: `0/92`
- Strict completion: `0.0%`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `1.15s`
- Total run time (end-to-end): `1.53s`

- Effective seed: `42`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/92` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `1.878e-10%`
- Estimated legal-space coverage (persistent cache): `1.221e-9%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 1869 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `7,676`, new simulations `6,138`, persistent cache hits `1,538`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Arcane Comet
  - Rune: Nimbus Cloak
  - Rune: Transcendence
  - Rune: Gathering Storm
  - Rune: Shield Bash
  - Rune: Overgrowth
  - Shard 1: attack_speed
  - Shard 2: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Nimbus Cloak.
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Shard 'tenacity' in slot 3 not applicable in current stat model.
  - Vladimir: Rune 'Arcane Comet' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
  - Vladimir: Rune 'Gathering Storm' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.

## Best Build
- Abyssal Mask, Actualizer, Endless Hunger, Kaenic Rookern, Spear of Shojin, Youmuu's Ghostblade

## Vladimir End Stats (Best Build)
- HP: 4,168.5, Armor: 109.5, MR: 54.7, AP: 134.0, AD: 165.0, Ability Haste: 25.0, Move Speed (flat bonus): 4.0, Move Speed (% bonus): 15.0

## Stack Overrides
- Spear of Shojin has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.

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
1. `score 0.0461` (+0.0000 vs top): Abyssal Mask, Actualizer, Endless Hunger, Kaenic Rookern, Spear of Shojin, Youmuu's Ghostblade | seed hits: 1/1 (100%) robust | Pareto-front
   - metrics: EHP~7,590.8, AP~134.0, timing score -8630.00, total cost 17,350

## Build Order Optimization
1. Cumulative score: `3.09` | Order: Spear of Shojin, Kaenic Rookern, Actualizer, Abyssal Mask, Endless Hunger, Youmuu's Ghostblade
   - Stage 1 (level 5): objective `0.495`, time alive `2.06s`, damage `3,583.0`, healing `121.8`
   - Stage 2 (level 8): objective `0.548`, time alive `2.30s`, damage `3,867.7`, healing `207.2`
   - Stage 3 (level 11): objective `0.556`, time alive `3.79s`, damage `5,290.5`, healing `250.4`
   - Stage 4 (level 14): objective `0.513`, time alive `3.78s`, damage `5,388.1`, healing `301.0`
   - Stage 5 (level 17): objective `0.487`, time alive `3.72s`, damage `5,218.9`, healing `324.4`
   - Stage 6 (level 20): objective `0.487`, time alive `3.66s`, damage `5,060.5`, healing `347.8`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Actualizer, Endless Hunger, Kaenic Rookern, Spear of Shojin, Youmuu's Ghostblade.
- Most frequent items in selected top set: Abyssal Mask (1/1), Actualizer (1/1), Endless Hunger (1/1), Kaenic Rookern (1/1), Spear of Shojin (1/1), Youmuu's Ghostblade (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

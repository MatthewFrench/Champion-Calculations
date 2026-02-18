# Vladimir URF Run Report

- Generated (local): `2026-02-18 04:33:48 -06:00`
- Generated (UTC): `2026-02-18T10:33:48.357362+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.0344**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **3.66s / 3,778.3 / 256.4 / 0 / 2.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0344`
- Worst-case scenario score: `0.0344`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0344`
- survival: weight `0.26` | normalized `0.0031` | contribution `0.0008` | impact `2.28%` | delta vs weight `-23.36pp`
- damage: weight `0.13` | normalized `0.2045` | contribution `0.0262` | impact `76.27%` | delta vs weight `+63.45pp`
- healing: weight `0.08` | normalized `0.0953` | contribution `0.0073` | impact `21.33%` | delta vs weight `+13.64pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0017` | contribution `0.0000` | impact `0.12%` | delta vs weight `-2.44pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `1`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `false` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `0`
- Unique scored candidates (all search stages): `0`
- Total score requests (all search stages): `0`
- Full evaluations cache hits/misses/waits: `0/0/0`
- Full persistent cache hits/entries: `0/2,543,763`
- Candidate keys generated / duplicate-pruned / unique: `0/0/1`
- Strict candidates seed-scored / remaining / processed: `0/1/0`
- Strict non-finite / timeout-skipped: `0/1`
- Strict completion: `0.0%`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `4.39s`
- Total run time (end-to-end): `4.75s`

- Effective seed: `1738881412323637786`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/1` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `0.000000%`
- Estimated legal-space coverage (persistent cache): `2.555e-7%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: 0.0%: no unique candidates were scored in this run.

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Dark Harvest
  - Rune: Taste of Blood
  - Rune: Sixth Sense
  - Rune: Relentless Hunter
  - Rune: Bone Plating
  - Rune: Unflinching
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.

## Best Build
- Axiom Arc, Bloodthirster, Hubris, Infinity Edge, Moonstone Renewer, Umbral Glaive

## Vladimir End Stats (Best Build)
- HP: 3,064.5, Armor: 109.5, MR: 54.7, AP: 36.0, AD: 330.0, Ability Haste: 73.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 2.0

## Stack Overrides
- Hubris has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.

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
1. `score 0.0344` (+0.0000 vs top): Axiom Arc, Bloodthirster, Hubris, Infinity Edge, Moonstone Renewer, Umbral Glaive | seed hits: 0/1 (0%) fragile | Pareto-front
   - metrics: EHP~5,580.4, AP~36.0, timing score -8996.67, total cost 17,650

## Build Order Optimization
1. Cumulative score: `3.04` | Order: Moonstone Renewer, Axiom Arc, Bloodthirster, Hubris, Infinity Edge, Umbral Glaive
   - Stage 1 (level 5): objective `0.515`, time alive `2.06s`, damage `3,566.8`, healing `106.1`
   - Stage 2 (level 8): objective `0.512`, time alive `2.01s`, damage `3,442.4`, healing `130.8`
   - Stage 3 (level 11): objective `0.527`, time alive `3.19s`, damage `4,158.3`, healing `185.4`
   - Stage 4 (level 14): objective `0.509`, time alive `3.17s`, damage `4,023.0`, healing `209.0`
   - Stage 5 (level 17): objective `0.487`, time alive `3.58s`, damage `3,896.6`, healing `232.6`
   - Stage 6 (level 20): objective `0.487`, time alive `3.66s`, damage `3,778.3`, healing `256.4`

## Deeper Insights
- Common core across all selected top builds: Axiom Arc, Bloodthirster, Hubris, Infinity Edge, Moonstone Renewer, Umbral Glaive.
- Most frequent items in selected top set: Axiom Arc (1/1), Bloodthirster (1/1), Hubris (1/1), Infinity Edge (1/1), Moonstone Renewer (1/1), Umbral Glaive (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

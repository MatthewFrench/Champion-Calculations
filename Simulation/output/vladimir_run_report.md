# Vladimir URF Run Report

- Generated (unix): `1771196044`
- Scenario: `scenario_vlad_urf.json`

## Headline
- Baseline objective score: **1.0000**
- Best objective score: **1.0000**
- Improvement: **+0.00%**
- Baseline time alive / damage dealt / healing done / enemy kills: **87.66s / 28017.5 / 5596.5 / 6**
- Best time alive / damage dealt / healing done / enemy kills: **87.66s / 28017.5 / 5596.5 / 6**
- Baseline cap survivor: **false**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `192/1`
- Ensemble seeds: `1`
- Objective weights (survival/damage/healing): `0.55/0.30/0.15`
- Full evaluations: `18` (cache hits/misses/waits: `0/18/0`)
- Full persistent cache hits/entries: `20/54`
- Unique candidate builds: `1`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`

- Time budget: `1.0s`; elapsed: `1.0s`; timed_out: `true`; progress: `0/1`

## Vladimir Base Stats At Level
- HP: 2690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Runes/Masteries
- Vladimir:
  - Rune: Grasp of the Undying
  - Rune: Conditioning
  - Rune: Overgrowth
  - Rune: Transcendence
  - Rune: Gathering Storm
  - Shard 1: ability_haste
  - Shard 2: health
  - Shard 3: health
- Enemies: none selected.

- Applied deterministic loadout effects:
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.

## Baseline Build
- Liandry's Torment, Boots of Swiftness, Zhonya's Hourglass, Guardian Angel, Protoplasm Harness, Morellonomicon

## Best Build
- Guardian Angel, Liandry's Torment, Morellonomicon, Protoplasm Harness, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 4593.0, Armor: 204.5, MR: 54.7, AP: 290.1, AD: 55.0, Ability Haste: 43.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

## Stack Assumptions
- Liandry's Torment has stack-based passive text in item data; currently treated as baseline/implicit unless explicitly modeled.

## Enemy Builds (URF Presets)
- Warwick: Stridebreaker, Mercury's Treads, Blade of the Ruined King, Kraken Slayer, Spirit Visage, Thornmail
  - Source: https://www.metasrc.com/lol/urf/build/warwick (last checked 2026-02-15)
  - Runes: Lethal Tempo, Triumph, Legend: Alacrity, Last Stand, Celerity, Waterwalking
  - Masteries: Fervor of Battle (1), Legendary Guardian (1)
- Vayne: Berserker's Greaves, Kraken Slayer, Guinsoo's Rageblade, Fiendhunter Bolts, Blade of the Ruined King, Infinity Edge
  - Source: https://www.metasrc.com/lol/urf/build/vayne (last checked 2026-02-15)
  - Runes: Lethal Tempo, Triumph, Legend: Alacrity, Coup de Grace, Conditioning, Overgrowth
  - Masteries: Fervor of Battle (1), Battering Blows (1)
- Morgana: Sorcerer's Shoes, Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo
  - Source: https://www.metasrc.com/lol/urf/build/morgana (last checked 2026-02-15)
  - Runes: Arcane Comet, Manaflow Band, Transcendence, Gathering Storm, Cheap Shot, Ultimate Hunter
  - Masteries: Thunderlord's Decree (1), Piercing Thoughts (1)
- Sona: Sorcerer's Shoes, Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap
  - Source: https://www.metasrc.com/lol/urf/build/sona (last checked 2026-02-15)
  - Runes: Summon Aery, Manaflow Band, Transcendence, Gathering Storm, Conditioning, Revitalize
  - Masteries: Windspeaker's Blessing (1), Intelligence (1)
- Dr. Mundo: Mercury's Treads, Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra
  - Source: https://www.metasrc.com/lol/urf/build/drmundo (last checked 2026-02-15)
  - Runes: Grasp of the Undying, Demolish, Conditioning, Overgrowth, Magical Footwear, Cosmic Insight
  - Masteries: Grasp of the Undying (1), Perseverance (1)

## Diverse Top Builds
1. `score 1.0000` (+0.0000 vs top): Guardian Angel, Liandry's Torment, Morellonomicon, Protoplasm Harness, Zhonya's Hourglass | seed hits: 0/1 (0%) fragile | Pareto-front
   - metrics: EHP~10545.4, AP~290.1, timing score -8405.00, total cost 14800

## Build Order Optimization
1. Cumulative score: `261.24` | Order: Guardian Angel, Zhonya's Hourglass, Protoplasm Harness, Morellonomicon, Liandry's Torment
   - Stage 1 (level 5): `9.84s`
   - Stage 2 (level 9): `41.18s`
   - Stage 3 (level 13): `55.03s`
   - Stage 4 (level 16): `67.54s`
   - Stage 5 (level 20): `87.66s`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Liandry's Torment, Morellonomicon, Protoplasm Harness, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (1/1), Liandry's Torment (1/1), Morellonomicon (1/1), Protoplasm Harness (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

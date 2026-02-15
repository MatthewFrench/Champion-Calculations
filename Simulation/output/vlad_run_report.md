# Vladimir URF Run Report

- Generated (unix): `1771146526`
- Scenario: `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json`

## Headline
- Baseline time alive: **14.85s**
- Best time alive: **19.91s**
- Improvement: **+34.02%**

- Champion level assumption: **20**

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Ensemble seeds: `8`
- Coarse evaluations: `922429` (cache hits/misses/waits: `2193886/922429/6481`)
- Full evaluations: `5382` (cache hits/misses/waits: `480/5382/0`)
- Unique candidate builds: `5382` (coarse pool limit `1200`)
- Bleed candidates injected: `1473`
- Seed-best mean/stddev: `19.91` / `0.000`

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
- Enemies (applied to all):
  - Shard 1: attack_speed
  - Shard 2: health
  - Shard 3: health

- Applied deterministic loadout effects:
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.
  - Enemies: Applied shard 'attack_speed' in slot 1.
  - Enemies: Applied shard 'health' in slot 2.
  - Enemies: Applied shard 'health' in slot 3.

## Baseline Build
- Liandry's Torment, Boots of Swiftness, Zhonya's Hourglass, Guardian Angel, Protoplasm Harness, Morellonomicon

## Best Build
- Guardian Angel, Heartsteel, Protoplasm Harness, Randuin's Omen, Unending Despair, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 5754.1, Armor: 329.5, MR: 54.7, AP: 200.6, AD: 55.0, Ability Haste: 43.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

## Stack Assumptions
- Heartsteel estimated stacks by level 20: 12.0 (acquired at level 11, reference full-at-20 stack target 20, estimated permanent bonus health: +307.9).

## Enemy Builds (DPS-Optimized)
- Warwick: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Vayne: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Morgana: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Sona: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Dr. Mundo: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows

## Diverse Top Builds
1. `19.91s` (+0.00s vs top): Guardian Angel, Heartsteel, Protoplasm Harness, Randuin's Omen, Unending Despair, Zhonya's Hourglass | seed hits: 8/8 (100%) robust | Pareto-front
   - metrics: EHP~17577.9, AP~209.3, timing score -9055.00, total cost 17450
2. `19.61s` (-0.29s vs top): Dead Man's Plate, Guardian Angel, Heartsteel, Iceborn Gauntlet, Protoplasm Harness, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile
   - metrics: EHP~16665.6, AP~205.6, timing score -9041.67, total cost 17750
3. `19.61s` (-0.29s vs top): Frozen Heart, Guardian Angel, Heartsteel, Protoplasm Harness, Sunfire Aegis, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile
   - metrics: EHP~16292.0, AP~194.7, timing score -8521.67, total cost 17150

## Build Order Optimization
1. Cumulative score: `86.70` | Order: Guardian Angel, Zhonya's Hourglass, Heartsteel, Randuin's Omen, Unending Despair, Protoplasm Harness
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.76s`
   - Stage 3 (level 11): `14.10s`
   - Stage 4 (level 14): `15.00s`
   - Stage 5 (level 17): `15.65s`
   - Stage 6 (level 20): `19.61s`
2. Cumulative score: `86.23` | Order: Guardian Angel, Zhonya's Hourglass, Dead Man's Plate, Heartsteel, Iceborn Gauntlet, Protoplasm Harness
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.76s`
   - Stage 3 (level 11): `14.23s`
   - Stage 4 (level 14): `14.84s`
   - Stage 5 (level 17): `15.38s`
   - Stage 6 (level 20): `19.45s`
3. Cumulative score: `86.20` | Order: Guardian Angel, Zhonya's Hourglass, Sunfire Aegis, Heartsteel, Protoplasm Harness, Frozen Heart
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.76s`
   - Stage 3 (level 11): `14.23s`
   - Stage 4 (level 14): `14.82s`
   - Stage 5 (level 17): `15.37s`
   - Stage 6 (level 20): `19.45s`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Heartsteel, Protoplasm Harness, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (3/3), Heartsteel (3/3), Protoplasm Harness (3/3), Zhonya's Hourglass (3/3), Dead Man's Plate (1/3), Frozen Heart (1/3), Iceborn Gauntlet (1/3), Randuin's Omen (1/3).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

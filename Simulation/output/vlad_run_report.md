# Vladimir URF Run Report

- Generated (unix): `1771146975`
- Scenario: `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json`

## Headline
- Baseline time alive: **14.85s**
- Best time alive: **19.91s**
- Improvement: **+34.02%**

- Champion level assumption: **20**

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Enemy scenarios: `1`
- Ensemble seeds: `8`
- Coarse evaluations: `1255831` (cache hits/misses/waits: `3939830/1255831/472`)
- Full evaluations: `2751` (cache hits/misses/waits: `480/2751/0`)
- Full capped prechecks: `5848`
- Unique candidate builds: `7448` (coarse pool limit `1200`)
- Bleed candidates injected: `1457`
- Adaptive candidates injected: `2292`
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
2. `19.71s` (-0.19s vs top): Dead Man's Plate, Guardian Angel, Heartsteel, Randuin's Omen, Warmog's Armor, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~18863.0, AP~222.0, timing score -9151.67, total cost 18150
3. `19.66s` (-0.24s vs top): Frozen Heart, Guardian Angel, Heartsteel, Iceborn Gauntlet, Randuin's Omen, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile
   - metrics: EHP~17295.4, AP~183.9, timing score -8661.67, total cost 17550
4. `19.61s` (-0.29s vs top): Guardian Angel, Heartsteel, Protoplasm Harness, Sunfire Aegis, Thornmail, Zhonya's Hourglass | seed hits: 8/8 (100%) robust
   - metrics: EHP~16774.2, AP~200.2, timing score -8950.00, total cost 17100
5. `19.61s` (-0.29s vs top): Dead Man's Plate, Guardian Angel, Heartsteel, Iceborn Gauntlet, Protoplasm Harness, Zhonya's Hourglass | seed hits: 8/8 (100%) robust
   - metrics: EHP~16665.6, AP~205.6, timing score -9041.67, total cost 17750

## Build Order Optimization
1. Cumulative score: `86.70` | Order: Guardian Angel, Zhonya's Hourglass, Heartsteel, Randuin's Omen, Unending Despair, Protoplasm Harness
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.76s`
   - Stage 3 (level 11): `14.10s`
   - Stage 4 (level 14): `15.00s`
   - Stage 5 (level 17): `15.65s`
   - Stage 6 (level 20): `19.61s`
2. Cumulative score: `86.69` | Order: Guardian Angel, Zhonya's Hourglass, Randuin's Omen, Heartsteel, Warmog's Armor, Dead Man's Plate
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.76s`
   - Stage 3 (level 11): `14.28s`
   - Stage 4 (level 14): `14.99s`
   - Stage 5 (level 17): `15.59s`
   - Stage 6 (level 20): `19.50s`
3. Cumulative score: `86.40` | Order: Guardian Angel, Zhonya's Hourglass, Heartsteel, Thornmail, Sunfire Aegis, Protoplasm Harness
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.76s`
   - Stage 3 (level 11): `14.10s`
   - Stage 4 (level 14): `14.99s`
   - Stage 5 (level 17): `15.49s`
   - Stage 6 (level 20): `19.49s`
4. Cumulative score: `86.23` | Order: Guardian Angel, Zhonya's Hourglass, Dead Man's Plate, Heartsteel, Iceborn Gauntlet, Protoplasm Harness
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.76s`
   - Stage 3 (level 11): `14.23s`
   - Stage 4 (level 14): `14.84s`
   - Stage 5 (level 17): `15.38s`
   - Stage 6 (level 20): `19.45s`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Heartsteel, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (5/5), Heartsteel (5/5), Zhonya's Hourglass (5/5), Protoplasm Harness (3/5), Randuin's Omen (3/5), Dead Man's Plate (2/5), Iceborn Gauntlet (2/5), Frozen Heart (1/5).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

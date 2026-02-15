# Vladimir URF Run Report

- Generated (unix): `1771135269`
- Scenario: `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenario_vlad_urf.json`

## Headline
- Baseline time alive: **14.84s**
- Best time alive: **19.05s**
- Improvement: **+28.41%**

- Champion level assumption: **20**

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
- Dead Man's Plate, Guardian Angel, Heartsteel, Randuin's Omen, Thornmail, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 5224.5, Armor: 409.5, MR: 54.7, AP: 183.1, AD: 55.0, Ability Haste: 8.0, Move Speed (flat bonus): 4.0, Move Speed (% bonus): 0.0

## Stack Assumptions
- Dead Man's Plate has stack-based passive text in item data; currently treated as baseline/implicit unless explicitly modeled.
- Heartsteel estimated stacks by level 20: 12.0 (acquired at level 11, reference full-at-20 stack target 20, estimated permanent bonus health: +278.4).

## Enemy Builds (DPS-Optimized)
- Warwick: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Vayne: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Morgana: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Sona: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows
- Dr. Mundo: Bloodthirster, Kraken Slayer, Phantom Dancer, Statikk Shiv, Stormrazor, Yun Tal Wildarrows

## Diverse Top Builds
1. `19.05s` (+0.00s vs top): Dead Man's Plate, Guardian Angel, Heartsteel, Randuin's Omen, Thornmail, Zhonya's Hourglass

## Build Order Optimization
1. Cumulative score: `85.87` | Order: Guardian Angel, Zhonya's Hourglass, Heartsteel, Randuin's Omen, Thornmail, Dead Man's Plate
   - Stage 1 (level 5): `8.57s`
   - Stage 2 (level 8): `13.56s`
   - Stage 3 (level 11): `14.23s`
   - Stage 4 (level 14): `14.99s`
   - Stage 5 (level 17): `15.65s`
   - Stage 6 (level 20): `18.87s`

## Deeper Insights
- Common core across all selected top builds: Dead Man's Plate, Guardian Angel, Heartsteel, Randuin's Omen, Thornmail, Zhonya's Hourglass.
- Most frequent items in selected top set: Dead Man's Plate (1/1), Guardian Angel (1/1), Heartsteel (1/1), Randuin's Omen (1/1), Thornmail (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

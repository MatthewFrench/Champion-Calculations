# Vladimir URF Run Report

- Generated (unix): `1771151341`
- Scenario: `scenario_vlad_urf.json`

## Headline
- Baseline time alive: **180.00s**
- Best time alive: **180.00s**
- Improvement: **+0.00%**

- Champion level assumption: **20**

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1200/1`
- Ensemble seeds: `8`
- Coarse evaluations: `1255437` (cache hits/misses/waits: `3935313/1255437/1`)
- Full evaluations: `65` (cache hits/misses/waits: `417/65/0`)
- Full capped prechecks: `0`
- Unique candidate builds: `7447` (coarse pool limit `1200`)
- Bleed candidates injected: `1474`
- Adaptive candidates injected: `2285`
- Seed-best mean/stddev: `180.00` / `0.000`

- Time budget: `5.0s`; elapsed: `126.6s`; timed_out: `true`; progress: `0/7447`

## Vladimir Base Stats At Level
- HP: 2690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Runes/Masteries
- Vladimir:
  - Rune: Lethal Tempo
  - Rune: Presence of Mind
  - Rune: Legend: Bloodline
  - Rune: Coup de Grace
  - Rune: Bone Plating
  - Rune: Shield Bash
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
  - Mastery: Fury (3/5)
  - Mastery: Sorcery (2/5)
  - Mastery: Feast (1/1)
  - Mastery: Natural Talent (3/5)
  - Mastery: Vampirism (2/5)
  - Mastery: Oppressor (1/1)
  - Mastery: Battering Blows (3/5)
  - Mastery: Piercing Thoughts (2/5)
  - Mastery: Deathfire Touch (1/1)
  - Mastery: Recovery (3/5)
  - Mastery: Unyielding (2/5)
  - Mastery: Tough Skin (1/1)
  - Mastery: Runic Armor (3/5)
  - Mastery: Veteran's Scars (2/5)
  - Mastery: Perseverance (1/1)
- Enemies: none selected.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Presence of Mind.
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.
  - Vladimir: Applied mastery stat effect from Fury.
  - Vladimir: Applied mastery stat effect from Battering Blows.
  - Vladimir: Applied mastery stat effect from Unyielding.

## Baseline Build
- Liandry's Torment, Boots of Swiftness, Zhonya's Hourglass, Guardian Angel, Protoplasm Harness, Morellonomicon

## Best Build
- Axiom Arc, Guardian Angel, Protoplasm Harness, Randuin's Omen, Unending Despair, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 4342.5, Armor: 333.5, MR: 54.7, AP: 154.0, AD: 110.0, Ability Haste: 1963.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 2.0

## Stack Assumptions
- No explicit stack assumptions triggered for selected best build items.

## Enemy Builds (DPS-Optimized)
- Warwick: Stridebreaker, Blade of the Ruined King, Spirit Visage, Thornmail, Death's Dance
- Vayne: Kraken Slayer, Guinsoo's Rageblade, Fiendhunter Bolts, Blade of the Ruined King, Infinity Edge
- Morgana: Liandry's Torment, Blackfire Torch, Rylai's Crystal Scepter, Zhonya's Hourglass, Luden's Echo
- Sona: Luden's Echo, Lich Bane, Stormsurge, Shadowflame, Rabadon's Deathcap
- Dr. Mundo: Heartsteel, Warmog's Armor, Spirit Visage, Thornmail, Titanic Hydra

## Diverse Top Builds
1. `180.00s` (+0.00s vs top): Axiom Arc, Guardian Angel, Protoplasm Harness, Randuin's Omen, Unending Despair, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~12771.2, AP~154.0, timing score -8680.00, total cost 17200

## Build Order Optimization
1. Cumulative score: `1080.00` | Order: Guardian Angel, Axiom Arc, Protoplasm Harness, Randuin's Omen, Unending Despair, Zhonya's Hourglass
   - Stage 1 (level 5): `180.00s`
   - Stage 2 (level 8): `180.00s`
   - Stage 3 (level 11): `180.00s`
   - Stage 4 (level 14): `180.00s`
   - Stage 5 (level 17): `180.00s`
   - Stage 6 (level 20): `180.00s`

## Deeper Insights
- Common core across all selected top builds: Axiom Arc, Guardian Angel, Protoplasm Harness, Randuin's Omen, Unending Despair, Zhonya's Hourglass.
- Most frequent items in selected top set: Axiom Arc (1/1), Guardian Angel (1/1), Protoplasm Harness (1/1), Randuin's Omen (1/1), Unending Despair (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

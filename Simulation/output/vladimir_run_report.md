# Vladimir URF Run Report

- Generated (unix): `1771155252`
- Scenario: `scenario_vlad_urf.json`

## Headline
- Baseline time alive: **27.71s**
- Best time alive: **180.00s**
- Improvement: **+549.48%**

- Champion level assumption: **20**

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `192/1`
- Ensemble seeds: `1`
- Coarse evaluations: `17329` (cache hits/misses/waits: `21163/17329/0`)
- Full evaluations: `29` (cache hits/misses/waits: `0/29/0`)
- Full persistent cache hits/entries: `71/101`
- Full capped prechecks: `0`
- Unique candidate builds: `332` (coarse pool limit `160`)
- Bleed candidates injected: `134`
- Adaptive candidates injected: `60`
- Seed-best mean/stddev: `0.00` / `0.000`

- Time budget: `2.0s`; elapsed: `2.0s`; timed_out: `true`; progress: `144/332`

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
- Guardian Angel, Heartsteel, Protoplasm Harness, Randuin's Omen, Warmog's Armor, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 6532.4, Armor: 279.5, MR: 54.7, AP: 226.3, AD: 55.0, Ability Haste: 28.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

## Stack Assumptions
- Heartsteel estimated stacks by level 20: 16.0 (acquired at level 8, reference full-at-20 stack target 20, estimated permanent bonus health: +462.4).

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
1. `180.00s` (+0.00s vs top): Guardian Angel, Heartsteel, Protoplasm Harness, Randuin's Omen, Warmog's Armor, Zhonya's Hourglass | seed hits: 1/1 (100%) robust | Pareto-front
   - metrics: EHP~17837.1, AP~231.1, timing score -9145.00, total cost 17750

## Build Order Optimization
1. Cumulative score: `179.58` | Order: Guardian Angel, Heartsteel, Zhonya's Hourglass, Randuin's Omen, Warmog's Armor, Protoplasm Harness
   - Stage 1 (level 5): `9.84s`
   - Stage 2 (level 8): `13.79s`
   - Stage 3 (level 11): `20.51s`
   - Stage 4 (level 14): `28.05s`
   - Stage 5 (level 17): `34.00s`
   - Stage 6 (level 20): `73.39s`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Heartsteel, Protoplasm Harness, Randuin's Omen, Warmog's Armor, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (1/1), Heartsteel (1/1), Protoplasm Harness (1/1), Randuin's Omen (1/1), Warmog's Armor (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

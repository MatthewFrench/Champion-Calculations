# Vladimir URF Run Report

- Generated (local): `2026-02-15 21:34:24 -06:00`
- Generated (UTC): `2026-02-16T03:34:24.411015+00:00`
- Generated (unix): `1,771,212,864`
- Scenario: `Simulation/scenario_vlad_urf.json`

## Headline
- Baseline objective score: **1.0000**
- Best objective score: **1.0000**
- Improvement: **+0.00%**
- Baseline time alive / damage dealt / healing done / enemy kills: **16.56s / 10,226.9 / 1,807.8 / 0**
- Best time alive / damage dealt / healing done / enemy kills: **16.56s / 10,226.9 / 1,807.8 / 0**
- Baseline cap survivor: **false**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1,200/1`
- Ensemble seeds: `8`
- Objective weights (survival/damage/healing): `0.55/0.30/0.15`
- Full evaluations: `9` (cache hits/misses/waits: `0/9/0`)
- Full persistent cache hits/entries: `24/36`
- Candidate keys generated / duplicate-pruned / unique: `0/0/1`
- Strict candidates seed-scored / remaining / processed: `0/1/0`
- Strict non-finite / timeout-skipped: `0/1`
- Strict completion: `0.0%`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`

- Time budget: `2.0s`; elapsed: `2.0s`; timed_out: `true`; progress: `0/1` (0.0%)

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

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
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Grasp of the Undying' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
  - Vladimir: Rune 'Gathering Storm' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.

## Baseline Build
- Liandry's Torment, Boots of Swiftness, Zhonya's Hourglass, Guardian Angel, Protoplasm Harness, Morellonomicon

## Best Build
- Guardian Angel, Liandry's Torment, Morellonomicon, Protoplasm Harness, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 4,593.0, Armor: 204.5, MR: 54.7, AP: 290.1, AD: 55.0, Ability Haste: 43.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

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

## Enemy Derived Combat Profiles
- Warwick: HP 3501.0, Armor 191.6, MR 70.9, AD 237.5, AS 2.681 (interval 0.373s), range 175, projectile speed 0, move speed 422.4, desired combat range 130, hit physical 237.5, hit ability 26.9, burst phys/magic/true 0.0/0.0/0.0
- Vayne: HP 2641.5, Armor 111.4, MR 54.7, AD 294.6, AS 5.845 (interval 0.171s), range 550, projectile speed 2500, move speed 390.7, desired combat range 520, hit physical 294.6, hit ability 27.7, burst phys/magic/true 70.0/0.0/0.0
- Morgana: HP 3440.5, Armor 154.8, MR 54.7, AD 122.5, AS 1.613 (interval 0.620s), range 450, projectile speed 1800, move speed 387.6, desired combat range 500, hit physical 122.5, hit ability 53.2, burst phys/magic/true 0.0/140.0/0.0
- Sona: HP 2413.5, Armor 105.8, MR 54.7, AD 106.0, AS 1.851 (interval 0.540s), range 550, projectile speed 1900, move speed 387.6, desired combat range 520, hit physical 106.0, hit ability 44.3, burst phys/magic/true 0.0/100.0/0.0
- Dr. Mundo: HP 6479.8, Armor 192.5, MR 72.7, AD 148.5, AS 1.635 (interval 0.612s), range 175, projectile speed 0, move speed 400.0, desired combat range 140, hit physical 148.5, hit ability 18.4, burst phys/magic/true 90.0/0.0/0.0

## Diverse Top Builds
1. `score 1.0000` (+0.0000 vs top): Guardian Angel, Liandry's Torment, Morellonomicon, Protoplasm Harness, Zhonya's Hourglass | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~10,545.4, AP~290.1, timing score -8405.00, total cost 14,800

## Build Order Optimization
1. Cumulative score: `5.56` | Order: Guardian Angel, Zhonya's Hourglass, Protoplasm Harness, Morellonomicon, Liandry's Torment
   - Stage 1 (level 5): objective `1.000`, time alive `9.56s`, damage `5,078.0`, healing `494.0`
   - Stage 2 (level 9): objective `1.233`, time alive `14.50s`, damage `6,631.8`, healing `607.5`
   - Stage 3 (level 13): objective `1.322`, time alive `16.01s`, damage `7,973.3`, healing `1,272.0`
   - Stage 4 (level 16): objective `1.002`, time alive `16.02s`, damage `9,767.1`, healing `1,668.7`
   - Stage 5 (level 20): objective `1.000`, time alive `16.56s`, damage `10,226.9`, healing `1,807.8`

## Deeper Insights
- Common core across all selected top builds: Guardian Angel, Liandry's Torment, Morellonomicon, Protoplasm Harness, Zhonya's Hourglass.
- Most frequent items in selected top set: Guardian Angel (1/1), Liandry's Torment (1/1), Morellonomicon (1/1), Protoplasm Harness (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

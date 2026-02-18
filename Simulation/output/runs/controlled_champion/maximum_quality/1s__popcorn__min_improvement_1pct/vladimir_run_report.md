# Vladimir URF Run Report

- Generated (local): `2026-02-18 04:34:05 -06:00`
- Generated (UTC): `2026-02-18T10:34:05.215870+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.0615**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **3.66s / 7,134.1 / 391.9 / 0 / 2.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0615`
- Worst-case scenario score: `0.0615`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0615`
- survival: weight `0.26` | normalized `0.0031` | contribution `0.0008` | impact `1.27%` | delta vs weight `-24.37pp`
- damage: weight `0.13` | normalized `0.3861` | contribution `0.0495` | impact `80.45%` | delta vs weight `+67.62pp`
- healing: weight `0.08` | normalized `0.1457` | contribution `0.0112` | impact `18.21%` | delta vs weight `+10.52pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0017` | contribution `0.0000` | impact `0.07%` | delta vs weight `-2.49pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `true` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `10,110`
- Unique scored candidates (all search stages): `10,110`
- Total score requests (all search stages): `10,110`
- Full evaluations cache hits/misses/waits: `0/10,110/0`
- Full persistent cache hits/entries: `0/2,553,873`
- Candidate keys generated / duplicate-pruned / unique: `2,409/420/1,989`
- Strict candidates seed-scored / remaining / processed: `0/1,989/0`
- Strict non-finite / timeout-skipped: `0/1,989`
- Strict completion: `0.0%`
- Bleed candidates injected: `1,866`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `9.90s`
- Total run time (end-to-end): `10.30s`

- Effective seed: `15776479977196226112`
- Coverage stage (pre-budget): `4.87s`; assets covered `181/181`; seeded candidates unique/raw `543/543`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/1,989` (0.0%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `1.0s`; significant threshold `1.00% of last best score`; significant events `10`; seconds since last significant improvement `1.7`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `1.016e-9%`
- Estimated legal-space coverage (persistent cache): `2.566e-7%`
- Estimated closeness probability (top 0.000001% heuristic): `0.01%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 10110 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - coverage_stage: requests `10,110`, new simulations `10,110`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: First Strike
  - Rune: Magical Footwear
  - Rune: Time Warp Tonic
  - Rune: Cosmic Insight
  - Rune: Nimbus Cloak
  - Rune: Waterwalking
  - Shard 1: attack_speed
  - Shard 2: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Magical Footwear.
  - Vladimir: Applied rune stat effect from Nimbus Cloak.
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Shard 'tenacity' in slot 3 not applicable in current stat model.

## Best Build
- Abyssal Mask, Actualizer, Bloodletter's Curse, Bloodthirster, Morellonomicon, Shadowflame

## Vladimir End Stats (Best Build)
- HP: 4,468.5, Armor: 109.5, MR: 54.7, AP: 380.7, AD: 80.0, Ability Haste: 55.0, Move Speed (flat bonus): 10.0, Move Speed (% bonus): 15.0

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
1. `score 0.0615` (+0.0000 vs top): Abyssal Mask, Actualizer, Bloodletter's Curse, Bloodthirster, Morellonomicon, Shadowflame | seed hits: 0/8 (0%) fragile | Pareto-front
   - metrics: EHP~8,137.1, AP~380.7, timing score -8750.00, total cost 17,800

## Build Order Optimization
1. Cumulative score: `3.16` | Order: Bloodletter's Curse, Morellonomicon, Shadowflame, Actualizer, Abyssal Mask, Bloodthirster
   - Stage 1 (level 5): objective `0.512`, time alive `2.06s`, damage `3,987.1`, healing `125.9`
   - Stage 2 (level 8): objective `0.589`, time alive `2.30s`, damage `4,769.1`, healing `230.6`
   - Stage 3 (level 11): objective `0.513`, time alive `3.79s`, damage `6,723.1`, healing `277.5`
   - Stage 4 (level 14): objective `0.544`, time alive `3.78s`, damage `7,327.0`, healing `319.5`
   - Stage 5 (level 17): objective `0.512`, time alive `3.72s`, damage `7,357.9`, healing `369.3`
   - Stage 6 (level 20): objective `0.487`, time alive `3.66s`, damage `7,134.1`, healing `391.9`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Actualizer, Bloodletter's Curse, Bloodthirster, Morellonomicon, Shadowflame.
- Most frequent items in selected top set: Abyssal Mask (1/1), Actualizer (1/1), Bloodletter's Curse (1/1), Bloodthirster (1/1), Morellonomicon (1/1), Shadowflame (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

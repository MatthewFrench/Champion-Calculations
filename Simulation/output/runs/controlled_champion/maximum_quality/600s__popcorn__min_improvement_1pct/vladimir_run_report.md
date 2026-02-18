# Vladimir URF Run Report

- Generated (local): `2026-02-18 03:28:11 -06:00`
- Generated (UTC): `2026-02-18T09:28:11.713820+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **46.7478**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **1200.00s / 324,668.1 / 155,368.0 / 90 / 1,192.03s**
- Best cap survivor: **true**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `46.7478`
- Worst-case scenario score: `46.7478`
- Worst-case blend weight: `0.35`
- Final blended objective score: `46.7478`
- survival: weight `0.26` | normalized `1.0000` | contribution `0.2564` | impact `0.55%` | delta vs weight `-25.09pp`
- damage: weight `0.13` | normalized `17.5722` | contribution `2.2528` | impact `4.82%` | delta vs weight `-8.00pp`
- healing: weight `0.08` | normalized `57.7576` | contribution `4.4429` | impact `9.50%` | delta vs weight `+1.81pp`
- enemy_kills: weight `0.51` | normalized `18.0000` | contribution `9.2308` | impact `19.75%` | delta vs weight `-31.54pp`
- invulnerable_seconds: weight `0.03` | normalized `1192.0310` | contribution `30.5649` | impact `65.38%` | delta vs weight `+62.82pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `242,936`
- Unique scored candidates (all search stages): `243,324`
- Total score requests (all search stages): `405,904`
- Full evaluations cache hits/misses/waits: `17,536/261,032/0`
- Full persistent cache hits/entries: `127,336/253,432`
- Candidate keys generated / duplicate-pruned / unique: `5,583/420/5,163`
- Strict candidates seed-scored / remaining / processed: `1,600/3,563/1,600`
- Strict non-finite / timeout-skipped: `0/3,563`
- Strict completion: `31.0%`
- Bleed candidates injected: `1,840`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `611.45s`
- Total run time (end-to-end): `680.39s`

- Effective seed: `13845328595312065070`
- Coverage stage (pre-budget): `9.49s`; assets covered `181/181`; seeded candidates unique/raw `543/543`
- Time budget: `600.0s`; timed_out: `true`; progress: `1,600/5,163` (31.0%) (budget starts after pre-budget coverage stage)

- Popcorn mode: window `600.0s`; significant threshold `1.00% of last best score`; significant events `15`; seconds since last significant improvement `576.4`

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `2.444e-8%`
- Estimated legal-space coverage (persistent cache): `2.546e-8%`
- Estimated closeness probability (top 0.000001% heuristic): `0.24%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 243324 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `395,794`, new simulations `250,922`, persistent cache hits `127,336`
  - coverage_stage: requests `10,110`, new simulations `10,110`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Unsealed Spellbook
  - Rune: Hextech Flashtraption
  - Rune: Triple Tonic
  - Rune: Jack Of All Trades
  - Rune: Triumph
  - Rune: Cut Down
  - Shard 1: attack_speed
  - Shard 2: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Unsealed Spellbook.
  - Vladimir: Applied rune stat effect from Unsealed Spellbook.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Shard 'tenacity' in slot 3 not applicable in current stat model.
  - Vladimir: Rune 'Triumph' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.

## Best Build
- Heartsteel, Morellonomicon, Protoplasm Harness, Rylai's Crystal Scepter, Warmog's Armor, Winter's Approach

## Vladimir End Stats (Best Build)
- HP: 6,851.7, Armor: 109.5, MR: 54.7, AP: 271.8, AD: 0.0, Ability Haste: 3,850.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 0.0

## Stack Overrides
- Heartsteel estimated stacks by level 20: 0.0 (acquired at level 20, reference full-at-20 stack target 20, estimated permanent bonus health: +0.0).

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
1. `score 46.7478` (+0.0000 vs top): Heartsteel, Morellonomicon, Protoplasm Harness, Rylai's Crystal Scepter, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile | Pareto-front
   - metrics: EHP~13,902.9, AP~297.7, timing score -8573.33, total cost 16,450
2. `score 46.7461` (-0.0017 vs top): Cosmic Drive, Heartsteel, Protoplasm Harness, Rylai's Crystal Scepter, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile
   - metrics: EHP~13,888.3, AP~292.7, timing score -8663.33, total cost 16,600
3. `score 46.7461` (-0.0017 vs top): Heartsteel, Protoplasm Harness, Riftmaker, Rylai's Crystal Scepter, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile
   - metrics: EHP~13,888.3, AP~292.7, timing score -8623.33, total cost 16,700
4. `score 46.7461` (-0.0017 vs top): Dusk and Dawn, Heartsteel, Protoplasm Harness, Rylai's Crystal Scepter, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile
   - metrics: EHP~13,888.3, AP~292.7, timing score -8773.33, total cost 16,700
5. `score 46.7379` (-0.0099 vs top): Dusk and Dawn, Heartsteel, Morellonomicon, Protoplasm Harness, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile
   - metrics: EHP~13,817.2, AP~300.9, timing score -8890.00, total cost 16,950
6. `score 46.7379` (-0.0099 vs top): Cosmic Drive, Heartsteel, Morellonomicon, Protoplasm Harness, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile
   - metrics: EHP~13,817.2, AP~300.9, timing score -8780.00, total cost 16,850
7. `score 46.7379` (-0.0099 vs top): Heartsteel, Morellonomicon, Protoplasm Harness, Riftmaker, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile | Pareto-front
   - metrics: EHP~13,817.2, AP~300.9, timing score -8748.33, total cost 16,950
8. `score 46.7363` (-0.0116 vs top): Cosmic Drive, Dusk and Dawn, Heartsteel, Protoplasm Harness, Warmog's Armor, Winter's Approach | seed hits: 1/8 (12%) fragile
   - metrics: EHP~13,802.7, AP~295.9, timing score -8905.00, total cost 17,100

## Build Order Optimization
1. Cumulative score: `25.28` | Order: Protoplasm Harness, Rylai's Crystal Scepter, Winter's Approach, Morellonomicon, Warmog's Armor, Heartsteel
   - Stage 1 (level 5): objective `17.606`, time alive `1200.00s`, damage `177,804.5`, healing `54,225.0`
   - Stage 2 (level 8): objective `2.885`, time alive `1200.00s`, damage `204,585.3`, healing `66,006.7`
   - Stage 3 (level 11): objective `1.354`, time alive `1200.00s`, damage `231,740.3`, healing `73,896.6`
   - Stage 4 (level 14): objective `1.452`, time alive `1200.00s`, damage `255,239.9`, healing `81,823.8`
   - Stage 5 (level 17): objective `0.997`, time alive `1200.00s`, damage `291,067.4`, healing `94,754.3`
   - Stage 6 (level 20): objective `0.990`, time alive `1200.00s`, damage `318,188.3`, healing `105,703.0`
2. Cumulative score: `23.46` | Order: Protoplasm Harness, Heartsteel, Warmog's Armor, Winter's Approach, Morellonomicon, Riftmaker
   - Stage 1 (level 5): objective `17.606`, time alive `1200.00s`, damage `177,804.5`, healing `54,225.0`
   - Stage 2 (level 8): objective `1.008`, time alive `410.20s`, damage `73,288.7`, healing `22,752.8`
   - Stage 3 (level 11): objective `1.373`, time alive `1200.00s`, damage `237,762.1`, healing `79,249.5`
   - Stage 4 (level 14): objective `1.485`, time alive `1200.00s`, damage `264,973.0`, healing `88,818.5`
   - Stage 5 (level 17): objective `1.000`, time alive `1200.00s`, damage `292,716.3`, healing `97,268.6`
   - Stage 6 (level 20): objective `0.992`, time alive `1200.00s`, damage `320,528.5`, healing `106,838.2`

## Deeper Insights
- Common core across all selected top builds: Heartsteel, Protoplasm Harness, Warmog's Armor, Winter's Approach.
- Most frequent items in selected top set: Heartsteel (8/8), Protoplasm Harness (8/8), Warmog's Armor (8/8), Winter's Approach (8/8), Morellonomicon (4/8), Rylai's Crystal Scepter (4/8), Cosmic Drive (3/8), Dusk and Dawn (3/8).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

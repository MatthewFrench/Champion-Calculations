# Vladimir URF Run Report

- Generated (local): `2026-02-19 15:15:41 -06:00`
- Generated (UTC): `2026-02-19T21:15:41.340948+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **-0.1329**
- Best outcome:
  - Time alive: **8.79s**
  - Damage dealt: **4,418.8**
  - Healing done: **505.5**
  - Enemy kills: **0**
  - Invulnerable seconds: **6.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0471`
- Worst-case scenario score: `0.0471`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0471`
- survival: weight `0.26` | normalized `0.0073` | contribution `0.0019` | impact `3.99%` | delta vs weight `-21.66pp`
- damage: weight `0.13` | normalized `0.2392` | contribution `0.0307` | impact `65.07%` | delta vs weight `+52.25pp`
- healing: weight `0.08` | normalized `0.1879` | contribution `0.0145` | impact `30.68%` | delta vs weight `+22.98pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0050` | contribution `0.0001` | impact `0.27%` | delta vs weight `-2.29pp`

## Rune Proc Telemetry (Best Trace)
- Arcane Comet:
  - Procs: `1`
  - Attempts: `11`
  - Eligible: `1`
  - Proc rate (vs attempts): `9.1%`
  - Proc rate (vs eligible): `100.0%`
  - Bonus damage: `153.04` (3.46% share)
  - Bonus healing: `0.00` (0.00% share)
  - Sources:
    - ability:
      - Procs: `1`
      - Attempts: `11`
      - Eligible: `1`
      - Proc rate (vs attempts): `9.1%`
      - Proc rate (vs eligible): `100.0%`
      - Bonus damage: `153.04`
      - Bonus healing: `0.00`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout:
  - Candidates: `1`
  - Finalists: `1`
- Ensemble seeds: `1`
- Parallelism:
  - Threads: `9`
  - Seed orchestration parallel: `false`
  - Portfolio parallel: `true`
  - Strategy-elites parallel: `true`
- Objective weights:
  - survival: `0.26`
  - damage: `0.13`
  - healing: `0.08`
  - enemy_kills: `0.51`
  - invulnerable_seconds: `0.03`
- Simulations executed (new full combat runs): `857`
- Unique scored candidates (all search stages): `857`
- Total score requests (all search stages): `35,422`
- In-memory full-evaluation cache:
  - Hits: `88`
  - Misses: `4,965`
  - Waits: `0`
- Candidate key generation:
  - Generated: `128`
  - Duplicate-pruned: `0`
  - Unique: `128`
- Strict candidate progression:
  - Seed-scored: `0`
  - Remaining: `128`
  - Processed: `0`
- Strict stage:
  - Non-finite: `0`
  - Timeout-skipped: `128`
  - Completion: `0.0%`
- Strict ordering heuristic:
  - Enabled: `true`
  - Rune signal weight: `0.30`
  - Shard signal weight: `0.20`
  - Exploration promotions: `1`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best stats:
  - Mean: `0.00`
  - Stddev: `0.000`
- Search elapsed time: `1.59s`
- Total run time (end-to-end): `1.59s`

- Effective seed: `8672920028738828121`
- Unmodeled rune gate:
  - Hard gate: `false`
  - Penalty per rune: `0.0200`
  - Rejected: `0`
  - Penalized: `4,964`
- Unmodeled item-effect gate:
  - Hard gate: `false`
  - Penalty per item: `0.0200`
  - Rejected: `0`
  - Penalized: `4,803`
- Time budget:
  - Budget: `1.0s`
  - Timed out: `true`
  - Progress: `0/128` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `8.609e-11%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 857 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `35,422`, new simulations `4,965`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Arcane Comet
  - Rune: Manaflow Band
  - Rune: Absolute Focus
  - Rune: Gathering Storm
  - Rune: Cheap Shot
  - Rune: Relentless Hunter
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Manaflow Band' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Absolute Focus' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Cheap Shot' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Relentless Hunter' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Arcane Comet' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
  - Vladimir: Rune 'Gathering Storm' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
- Controlled champion runes with no modeled deterministic/runtime combat effect:
  - Manaflow Band
  - Absolute Focus
  - Cheap Shot
  - Relentless Hunter
- Controlled champion items with unmodeled passive/active/structured runtime effects:
  - Abyssal Mask
  - Actualizer
  - Bloodletter's Curse
  - Bloodsong
  - Bloodthirster

## Best Build
- Abyssal Mask, Actualizer, Bloodletter's Curse, Bloodsong, Bloodthirster, Guardian Angel

## Vladimir End Stats (Best Build)
- HP: 4,022.5, Armor: 154.5, MR: 54.7, AP: 190.8, AD: 135.0, Ability Haste: 48.0, Move Speed (flat bonus): 0.0, Move Speed (% bonus): 2.0

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
- No diverse builds found under current thresholds.

## Build Order Optimization
- No build-order optimization results available.

## Deeper Insights
- Broaden thresholds (`--max-relative-gap-percent`) or lower diversity constraint (`--min-item-diff`) to surface more alternatives.

# Vladimir URF Run Report

- Generated (local): `2026-02-19 14:20:55 -06:00`
- Generated (UTC): `2026-02-19T20:20:55.503621+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.0955**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **13.10s / 7,593.7 / 1,392.7 / 0 / 10.50s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0955`
- Worst-case scenario score: `0.0955`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0955`
- survival: weight `0.26` | normalized `0.0109` | contribution `0.0028` | impact `2.93%` | delta vs weight `-22.71pp`
- damage: weight `0.13` | normalized `0.4110` | contribution `0.0527` | impact `55.15%` | delta vs weight `+42.33pp`
- healing: weight `0.08` | normalized `0.5177` | contribution `0.0398` | impact `41.68%` | delta vs weight `+33.99pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0088` | contribution `0.0002` | impact `0.23%` | delta vs weight `-2.33pp`

## Rune Proc Telemetry (Best Trace)
- Arcane Comet: procs `2` / attempts `12` / eligible `2` (proc/attempt 16.7%, proc/eligible 100.0%), bonus damage `317.74` (4.18% share), bonus healing `0.00` (0.00% share)
  - sources: ability (procs 2, attempts 12, eligible 2, proc/attempt 16.7%, proc/eligible 100.0%, damage 317.74, healing 0.00)

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `maximum_quality`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `8`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `true` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `2,821`
- Unique scored candidates (all search stages): `2,821`
- Total score requests (all search stages): `728,299`
- Full evaluations cache hits/misses/waits: `4,765/3,909/120`
- Full persistent cache hits/entries: `119,849/2,821`
- Candidate keys generated / duplicate-pruned / unique: `9,148/7,061/2,087`
- Strict candidates seed-scored / remaining / processed: `0/2,087/0`
- Strict non-finite / timeout-skipped: `0/2,087`
- Strict completion: `0.0%`
- Strict ordering heuristic (enabled / rune_weight / shard_weight / exploration_promotions): `true` / `0.30` / `0.20` / `1`
- Bleed candidates injected: `721`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `6.06s`
- Total run time (end-to-end): `6.92s`

- Effective seed: `17481730509162292401`
- Unmodeled rune gate (hard_gate / penalty_per_rune / rejected / penalized): `true` / `0.0000` / `0` / `0`
- Unmodeled item-effect gate (hard_gate / penalty_per_item / rejected / penalized): `true` / `0.0000` / `0` / `0`
- Coverage stage (pre-budget): `4.18s`; assets covered `22/33`; seeded candidates unique/raw `57/66`
- Coverage warning: Coverage incomplete: touched 22/33 assets; coverage stage could not produce finite candidates for at least one locked asset. Continuing search in degraded coverage mode.
- Time budget: `1.0s`; timed_out: `true`; progress: `0/2,087` (0.0%) (budget starts after pre-budget coverage stage)

- Estimated total legal candidate space: `33,264`
- Estimated legal-space coverage (this run): `8.480640%`
- Estimated legal-space coverage (persistent cache): `8.480640%`
- Estimated closeness probability (top 0.000001% heuristic): `8.13%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.003006253% (about top 1 candidates in the legal space) and n = 2821 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `727,123`, new simulations `3,069`, persistent cache hits `119,513`
  - coverage_stage: requests `1,176`, new simulations `840`, persistent cache hits `336`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Arcane Comet
  - Rune: Nimbus Cloak
  - Rune: Celerity
  - Rune: Gathering Storm
  - Rune: Magical Footwear
  - Rune: Jack Of All Trades
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Nimbus Cloak.
  - Vladimir: Applied rune stat effect from Celerity.
  - Vladimir: Applied rune stat effect from Celerity.
  - Vladimir: Applied rune stat effect from Magical Footwear.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied rune stat effect from Jack Of All Trades.
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Arcane Comet' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
  - Vladimir: Rune 'Gathering Storm' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.

## Best Build
- Blade of the Ruined King, Guardian Angel, Guinsoo's Rageblade, Heartsteel, Infinity Edge, Zhonya's Hourglass

## Vladimir End Stats (Best Build)
- HP: 4,134.7, Armor: 204.5, MR: 54.7, AP: 177.4, AD: 200.0, Ability Haste: 8.0, Move Speed (flat bonus): 10.0, Move Speed (% bonus): 25.0

## Stack Overrides
- Blade of the Ruined King has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.
- Guinsoo's Rageblade has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.
- Heartsteel estimated stacks by level 20: 8.0 (acquired at level 14, reference full-at-20 stack target 20, estimated permanent bonus health: +150.6).

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
1. `score 0.0955` (+0.0000 vs top): Blade of the Ruined King, Guardian Angel, Guinsoo's Rageblade, Heartsteel, Infinity Edge, Zhonya's Hourglass | seed hits: 5/8 (62%) robust | Pareto-front
   - metrics: EHP~10,183.4, AP~187.4, timing score -9706.67, total cost 19,150

## Build Order Optimization
1. Cumulative score: `635.36` | Order: Guardian Angel, Zhonya's Hourglass, Blade of the Ruined King, Heartsteel, Guinsoo's Rageblade, Infinity Edge
   - Stage 1 (level 5): objective `631.614`, time alive `6.58s`, damage `3,262.0`, healing `380.9`
   - Stage 2 (level 8): objective `0.515`, time alive `6.40s`, damage `2,886.1`, healing `404.9`
   - Stage 3 (level 11): objective `0.955`, time alive `11.50s`, damage `5,192.1`, healing `829.2`
   - Stage 4 (level 14): objective `0.946`, time alive `13.36s`, damage `7,695.8`, healing `1,208.9`
   - Stage 5 (level 17): objective `0.840`, time alive `13.31s`, damage `6,703.3`, healing `1,235.7`
   - Stage 6 (level 20): objective `0.487`, time alive `13.10s`, damage `6,930.9`, healing `1,241.8`

## Deeper Insights
- Common core across all selected top builds: Blade of the Ruined King, Guardian Angel, Guinsoo's Rageblade, Heartsteel, Infinity Edge, Zhonya's Hourglass.
- Most frequent items in selected top set: Blade of the Ruined King (1/1), Guardian Angel (1/1), Guinsoo's Rageblade (1/1), Heartsteel (1/1), Infinity Edge (1/1), Zhonya's Hourglass (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

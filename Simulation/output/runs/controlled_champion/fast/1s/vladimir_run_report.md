# Vladimir URF Run Report

- Generated (local): `2026-02-19 14:42:18 -06:00`
- Generated (UTC): `2026-02-19T20:42:18.813626+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **-0.1677**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **1.55s / 1,721.3 / 0.0 / 0 / 0.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0123`
- Worst-case scenario score: `0.0123`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0123`
- survival: weight `0.26` | normalized `0.0013` | contribution `0.0003` | impact `2.70%` | delta vs weight `-22.95pp`
- damage: weight `0.13` | normalized `0.0932` | contribution `0.0119` | impact `97.30%` | delta vs weight `+84.48pp`
- healing: weight `0.08` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-7.69pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-2.56pp`

## Rune Proc Telemetry (Best Trace)
- No rune procs were recorded during the best-trace replay.

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `1`
- Parallelism (threads / seed-orchestration / portfolio / strategy-elites): `9` / `false` / `true` / `true`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `1,752`
- Unique scored candidates (all search stages): `1,752`
- Total score requests (all search stages): `34,219`
- In-memory full-evaluation cache hits/misses/waits: `468/2,400/0`
- Candidate keys generated / duplicate-pruned / unique: `128/0/128`
- Strict candidates seed-scored / remaining / processed: `0/128/0`
- Strict non-finite / timeout-skipped: `0/128`
- Strict completion: `0.0%`
- Strict ordering heuristic (enabled / rune_weight / shard_weight / exploration_promotions): `true` / `0.30` / `0.20` / `1`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `1.61s`
- Total run time (end-to-end): `1.61s`

- Effective seed: `330752026848107508`
- Unmodeled rune gate (hard_gate / penalty_per_rune / rejected / penalized): `false` / `0.0200` / `0` / `2,398`
- Unmodeled item-effect gate (hard_gate / penalty_per_item / rejected / penalized): `false` / `0.0200` / `0` / `2,338`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/128` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `1.760e-10%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: Estimated as P(hit top 0.000001% candidate set) = 1 - (1 - q)^n, with q = 0.000001000% (about top 9954248 candidates in the legal space) and n = 1752 unique scored candidates. This is a conservative random-sampling approximation, not a guarantee.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `34,219`, new simulations `2,400`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Aftershock
  - Rune: Demolish
  - Rune: Bone Plating
  - Rune: Revitalize
  - Rune: Magical Footwear
  - Rune: Time Warp Tonic
  - Shard 1: attack_speed
  - Shard 2: health
  - Shard 3: tenacity
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied rune stat effect from Magical Footwear.
  - Vladimir: Applied shard 'attack_speed' in slot 1.
  - Vladimir: Applied shard 'health' in slot 2.
  - Vladimir: Applied shard 'tenacity' in slot 3.
- Skipped unsupported/non-deterministic effects:
  - Vladimir: Rune 'Demolish' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Bone Plating' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Revitalize' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Time Warp Tonic' currently has no implemented deterministic stat effect or combat-time runtime effect in controlled champion simulation.
  - Vladimir: Rune 'Aftershock' has a combat-time script effect and is not fully represented as static pre-fight stats at level 20.
- Controlled champion runes with no modeled deterministic/runtime combat effect:
  - Demolish
  - Bone Plating
  - Revitalize
  - Time Warp Tonic
- Controlled champion items with unmodeled passive/active/structured runtime effects:
  - Abyssal Mask
  - Actualizer
  - Bloodletter's Curse
  - Umbral Glaive
  - Unending Despair

## Best Build
- Abyssal Mask, Actualizer, Bloodletter's Curse, Umbral Glaive, Unending Despair, Void Staff

## Vladimir End Stats (Best Build)
- HP: 4,374.5, Armor: 159.5, MR: 54.7, AP: 292.4, AD: 60.0, Ability Haste: 70.0, Move Speed (flat bonus): 10.0, Move Speed (% bonus): 0.0

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

# Vladimir URF Run Report

- Generated (local): `2026-02-18 03:50:46 -06:00`
- Generated (UTC): `2026-02-18T09:50:46.420743+00:00`
- Scenario: `Simulation/scenarios/vladimir_urf_teamfight.json`

## Headline
- Best objective score: **0.0382**
- Best time alive / damage dealt / healing done / enemy kills / invulnerable seconds: **3.66s / 4,106.8 / 309.6 / 0 / 2.00s**
- Best cap survivor: **false**

- Champion level assumption: **20**

## Objective Score Breakdown
### Best Build
- Weighted-mean score: `0.0382`
- Worst-case scenario score: `0.0382`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.0382`
- survival: weight `0.26` | normalized `0.0031` | contribution `0.0008` | impact `2.05%` | delta vs weight `-23.59pp`
- damage: weight `0.13` | normalized `0.2223` | contribution `0.0285` | impact `74.65%` | delta vs weight `+61.83pp`
- healing: weight `0.08` | normalized `0.1151` | contribution `0.0089` | impact `23.19%` | delta vs weight `+15.50pp`
- enemy_kills: weight `0.51` | normalized `0.0000` | contribution `0.0000` | impact `0.00%` | delta vs weight `-51.28pp`
- invulnerable_seconds: weight `0.03` | normalized `0.0017` | contribution `0.0000` | impact `0.11%` | delta vs weight `-2.45pp`

## Search Diagnostics
- Strategy: `portfolio(beam, hill_climb, genetic, simulated_annealing, mcts, random, greedy)`
- Search quality profile: `fast`
- Enemy scenarios: `1`
- Loadout candidates/finalists: `1/1`
- Ensemble seeds: `1`
- Objective weights (survival/damage/healing/enemy_kills/invulnerable_seconds): `0.26/0.13/0.08/0.51/0.03`
- Simulations executed (new full combat runs): `0`
- Unique scored candidates (all search stages): `0`
- Total score requests (all search stages): `5,739`
- Full evaluations cache hits/misses/waits: `0/5,739/0`
- Full persistent cache hits/entries: `0/263,543`
- Candidate keys generated / duplicate-pruned / unique: `0/0/1`
- Strict candidates seed-scored / remaining / processed: `0/1/0`
- Strict non-finite / timeout-skipped: `0/1`
- Strict completion: `0.0%`
- Bleed candidates injected: `0`
- Adaptive candidates injected: `0`
- Seed-best mean/stddev: `0.00` / `0.000`
- Search elapsed time: `1.03s`
- Total run time (end-to-end): `1.38s`

- Effective seed: `18130075023443300966`
- Time budget: `1.0s`; timed_out: `true`; progress: `0/1` (0.0%)

- Estimated total legal candidate space: `995,424,835,958,784`
- Estimated legal-space coverage (this run): `0.000000%`
- Estimated legal-space coverage (persistent cache): `2.648e-8%`
- Estimated closeness probability (top 0.000001% heuristic): `0.00%`
- Closeness probability note: 0.0%: no unique candidates were scored in this run.
- Search-type simulation breakdown:
  - seed_search:portfolio: requests `5,739`, new simulations `5,739`, persistent cache hits `0`

## Vladimir Base Stats At Level
- HP: 2,690.0, Armor: 109.5, MR: 54.7, AD: 112.0, AS: 0.908, MS: 330.0

## Selected Rune Page And Shards
- Vladimir:
  - Rune: Electrocute
  - Rune: Taste of Blood
  - Rune: Grisly Mementos
  - Rune: Treasure Hunter
  - Rune: Shield Bash
  - Rune: Bone Plating
  - Shard 1: ability_haste
  - Shard 2: movement_speed
  - Shard 3: health
- Opponents: champion-specific preset rune pages are listed in Enemy Builds.

- Applied deterministic loadout effects:
  - Vladimir: Applied shard 'ability_haste' in slot 1.
  - Vladimir: Applied shard 'movement_speed' in slot 2.
  - Vladimir: Applied shard 'health' in slot 3.

## Best Build
- Abyssal Mask, Axiom Arc, Black Cleaver, Bloodthirster, Navori Flickerblade, Zaz'Zak's Realmspike

## Vladimir End Stats (Best Build)
- HP: 3,774.5, Armor: 109.5, MR: 54.7, AP: 35.8, AD: 175.0, Ability Haste: 63.0, Move Speed (flat bonus): 4.0, Move Speed (% bonus): 2.0

## Stack Overrides
- Black Cleaver has stack-based passive text in item data; currently treated as default/implicit unless explicitly modeled.

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
1. `score 0.0382` (+0.0000 vs top): Abyssal Mask, Axiom Arc, Black Cleaver, Bloodthirster, Navori Flickerblade, Zaz'Zak's Realmspike | seed hits: 0/1 (0%) fragile | Pareto-front
   - metrics: EHP~6,873.3, AP~35.8, timing score -7956.67, total cost 14,850

## Build Order Optimization
1. Cumulative score: `3.02` | Order: Black Cleaver, Abyssal Mask, Zaz'Zak's Realmspike, Axiom Arc, Bloodthirster, Navori Flickerblade
   - Stage 1 (level 5): objective `0.491`, time alive `2.06s`, damage `3,545.8`, healing `118.1`
   - Stage 2 (level 8): objective `0.515`, time alive `2.02s`, damage `3,673.3`, healing `169.1`
   - Stage 3 (level 11): objective `0.531`, time alive `3.65s`, damage `4,519.4`, healing `238.6`
   - Stage 4 (level 14): objective `0.497`, time alive `3.68s`, damage `4,372.5`, healing `262.2`
   - Stage 5 (level 17): objective `0.496`, time alive `3.72s`, damage `4,235.3`, healing `285.9`
   - Stage 6 (level 20): objective `0.487`, time alive `3.66s`, damage `4,106.8`, healing `309.6`

## Deeper Insights
- Common core across all selected top builds: Abyssal Mask, Axiom Arc, Black Cleaver, Bloodthirster, Navori Flickerblade, Zaz'Zak's Realmspike.
- Most frequent items in selected top set: Abyssal Mask (1/1), Axiom Arc (1/1), Black Cleaver (1/1), Bloodthirster (1/1), Navori Flickerblade (1/1), Zaz'Zak's Realmspike (1/1).
- Interpretation: these recurring items are your current high-confidence survivability spine; swaps around them represent viable style variants.

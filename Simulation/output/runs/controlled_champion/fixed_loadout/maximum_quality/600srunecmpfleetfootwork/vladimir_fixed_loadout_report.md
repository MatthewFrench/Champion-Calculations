# Controlled Champion Fixed Loadout Evaluation

- Scenario: `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenarios/vladimir_urf_teamfight.json`
- Search quality profile: `maximum_quality`
- Controlled champion: `Vladimir`
- Build items: `Bloodletter's Curse, Guardian Angel, Heartsteel, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass`
- Runes: `Fleet Footwork, Triumph, Legend: Alacrity, Last Stand, Second Wind, Unflinching`
- Shards: `ability_haste, movement_speed, health`

## Headline
- Objective score: **0.2913**
- Outcome (time_alive / damage / healing / enemy_kills / invulnerable_seconds): **14.54s / 12101.7 / 3546.8 / 1 / 10.50s**

## Objective Score Breakdown
- Weighted-mean score: `0.2913`
- Worst-case scenario score: `0.2913`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.2913`
- survival: weight `0.26` | normalized `0.0121` | contribution `0.0031` | impact `1.07%`
- damage: weight `0.13` | normalized `0.6550` | contribution `0.0840` | impact `28.83%`
- healing: weight `0.08` | normalized `1.3185` | contribution `0.1014` | impact `34.82%`
- enemy_kills: weight `0.51` | normalized `0.2000` | contribution `0.1026` | impact `35.21%`
- invulnerable_seconds: weight `0.03` | normalized `0.0088` | contribution `0.0002` | impact `0.08%`

## Notes
- This mode evaluates one fixed build and loadout directly; no candidate search or mutation is performed.
- Trace markdown: `Simulation/output/runs/controlled_champion/fixed_loadout/maximum_quality/600srunecmpfleetfootwork/vladimir_fixed_loadout_trace.md`
- Trace json: `Simulation/output/runs/controlled_champion/fixed_loadout/maximum_quality/600srunecmpfleetfootwork/vladimir_fixed_loadout_trace.json`

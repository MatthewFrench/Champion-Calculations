# Controlled Champion Fixed Loadout Evaluation

- Scenario: `/Users/matthewfrench/Documents/League of Legends/Vladimir/Simulation/scenarios/vladimir_urf_teamfight.json`
- Search quality profile: `maximum_quality`
- Controlled champion: `Vladimir`
- Build items: `Bloodletter's Curse, Guardian Angel, Heartsteel, Protoplasm Harness, Warmog's Armor, Zhonya's Hourglass`
- Runes: `Conqueror, Triumph, Legend: Alacrity, Last Stand, Second Wind, Unflinching`
- Shards: `ability_haste, movement_speed, health`

## Headline
- Objective score: **0.2990**
- Outcome (time_alive / damage / healing / enemy_kills / invulnerable_seconds): **14.54s / 12325.6 / 3762.0 / 1 / 10.50s**

## Objective Score Breakdown
- Weighted-mean score: `0.2990`
- Worst-case scenario score: `0.2990`
- Worst-case blend weight: `0.35`
- Final blended objective score: `0.2990`
- survival: weight `0.26` | normalized `0.0121` | contribution `0.0031` | impact `1.04%`
- damage: weight `0.13` | normalized `0.6671` | contribution `0.0855` | impact `28.60%`
- healing: weight `0.08` | normalized `1.3985` | contribution `0.1076` | impact `35.98%`
- enemy_kills: weight `0.51` | normalized `0.2000` | contribution `0.1026` | impact `34.30%`
- invulnerable_seconds: weight `0.03` | normalized `0.0088` | contribution `0.0002` | impact `0.08%`

## Notes
- This mode evaluates one fixed build and loadout directly; no candidate search or mutation is performed.
- Trace markdown: `Simulation/output/runs/controlled_champion/fixed_loadout/maximum_quality/600srunecmpconqueror/vladimir_fixed_loadout_trace.md`
- Trace json: `Simulation/output/runs/controlled_champion/fixed_loadout/maximum_quality/600srunecmpconqueror/vladimir_fixed_loadout_trace.json`

## Rune Proc Telemetry
- Conqueror: procs `19` / attempts `21` / eligible `19` (proc/attempt 90.5%, proc/eligible 100.0%), bonus damage `425.15` (3.45% share), bonus healing `145.08` (3.86% share)
  - sources: ability (procs 16, attempts 16, eligible 16, proc/attempt 100.0%, proc/eligible 100.0%, damage 425.15, healing 0.00); on_hit (procs 3, attempts 5, eligible 3, proc/attempt 60.0%, proc/eligible 100.0%, damage 0.00, healing 145.08)
- Triumph: procs `1` / attempts `1` / eligible `1` (proc/attempt 100.0%, proc/eligible 100.0%), bonus damage `0.00` (0.00% share), bonus healing `535.13` (14.22% share)
  - sources: enemy_kill (procs 1, attempts 1, eligible 1, proc/attempt 100.0%, proc/eligible 100.0%, damage 0.00, healing 535.13)


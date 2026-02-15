# Improvement Tracker

## Done
- Enforced runtime budget checks across all major phases and search loops.
- Added phase-aware periodic status updates from initialization through finalization.
- Moved enemy URF presets into a data file:
  - `Simulation/data/enemy_urf_presets.json`
  - Includes source links and last checked date.
- Added strict startup validation for enemy preset items, runes, shards, and masteries.
- Added structured run output JSON:
  - default path mirrors markdown report with `.json` extension.
- Added search quality profiles:
  - `fast`
  - `balanced`
  - `maximum_quality`
- Replaced full permutation build order search with beam plus optimistic bound pruning.
- Added regression tests for legality and key rules.
- Added persistent full-score cache across runs under:
  - `Simulation/output/cache/`

## Not Done
- Champion script realism expansion:
  - Champion-specific skill behavior models for enemies beyond generic timed damage and crowd control.
  - Planned as the next deeper follow-up item.

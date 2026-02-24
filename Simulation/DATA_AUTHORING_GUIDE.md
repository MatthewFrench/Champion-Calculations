# Data Authoring Guide

This guide defines how to add or update champion, item, rune, and shard data so it stays aligned with the simulator architecture.

Start here before implementation:
- `Simulation/COVERAGE_STANDARDS.md` (exemplar-derived quality bar per category)
- `Simulation/COVERAGE_CHECKLIST.md` (completion gate)

## Current Data Reality
- We already have broad source data in:
  - `From Online/champions/`
  - `From Online/items/`
  - `Masteries/RunesReforged.json`
  - `Masteries/RunesReforged/` (split tree/stat-shard structure)
- Champion corpus file parity is complete:
  - `From Online/champions`: `172` champion files
  - `Characters/`: `172` canonical champion files (+ `ChampionDefaults.json`)
  - parity gap: `0` champions
  - parity tracker: `Simulation/champion_data_coverage_inventory.json`
- Manual champion behavior verification progress:
  - `172/172` champions marked manual verified in `Simulation/champion_behavior_verification_tracker.json` (`100%`)
  - `0` champions remain source-extracted-only
  - strict-fragment context-note queue currently `0` entries across `0` champion files (cleared and held in waves 63-75)
  - secondary cadence-fragment queue (`every 0.` class) currently `0` entries across `0` champion files (cleared in post-wave audit)
  - tertiary article-token queue (`during the 0.` class) currently `0` entries across `0` champion files (cleared in post-wave audit)
  - champion ability `description_source` completeness currently `860/860` (`0` missing across `0` champions)
- Current champion data priority is no-regression fidelity maintenance and unresolved-confidence follow-up (execution semantics depth, context-note completeness, and ambiguity tracking), not file-creation parity.
- Structured-item parse-confidence completeness is currently fully normalized:
  - `243/243` structured item files have numeric `effects_structured[].parse_confidence` entries on all structured effects (`0` missing/null entries)
- Rune cadence-text spacing artifact queue is currently clear (`0` remaining `0. 5`-style interval strings in flat/split canonical files).
- Broader rune decimal-spacing cleanup is now clear (`0` remaining `x. y` spacing artifacts in canonical flat/split rune files).
- Rune multi-branch semantic decomposition is now explicit for all current `per_rank` effects:
  - `14/14` `formula.type = per_rank` rune effects include `semantic_components`
  - deferred runtime/code follow-up remains to consume/enforce these components automatically
- Deferred code follow-up remains for maintainability: add loader/lint enforcement so raw-text numeric normalization and dependent metadata consistency are validated automatically.
- We do **not** yet have full runtime-mechanic coverage for all assets.
- The canonical gap tracker is `Simulation/COVERAGE_GAPS.md`.

## Canonical Ownership Map
- Champion canonical gameplay data:
  - `Characters/<Champion>.json`
- Champion AI policy (spacing/movement/script polling/priority overrides):
  - `Simulation/data/champion_ai_profiles.json`
- Item canonical data:
  - `Items/<Item>.json`
- Rune and shard canonical data:
  - `Masteries/RunesReforged.json` (flat compatibility file)
  - `Masteries/RunesReforged/RunesReforged.json` (split index)
  - `Masteries/RunesReforged/Trees/*/primary_runes.json`
  - `Masteries/RunesReforged/Trees/*/secondary_runes.json`
  - `Masteries/RunesReforged/StatShards/stat_shards.json`
- Champion corpus parity tracking:
  - `Simulation/champion_data_coverage_inventory.json`
- Champion manual behavior-fidelity tracking:
  - `Simulation/champion_behavior_verification_tracker.json`
- Mode defaults (for example URF respawn):
  - `Game Mode/<Mode>.json`
- Global simulator/search/engine defaults:
  - `Simulation/data/simulator_defaults.json`

Do not move data across these ownership boundaries.

## Best Complete Examples
Detailed acceptance criteria for these examples live in:
- `Simulation/COVERAGE_STANDARDS.md`

- Champion canonical + script integration:
  - `Characters/Vladimir.json`
  - `Simulation/src/scripts/champions/vladimir/`
  - `Simulation/src/scripts/champions/controlled_champion.rs`
- Modeled survivability item effects:
  - `Items/Zhonyas Hourglass.json`
  - `Items/Guardian Angel.json`
  - `Items/Protoplasm Harness.json`
  - `Simulation/src/scripts/items/hooks.rs`
  - `Simulation/src/scripts/runtime/controlled_champion_loadout.rs`
- Modeled combat-time rune behavior:
  - `Masteries/RunesReforged.json` (runtime compatibility)
  - `Masteries/RunesReforged/RunesReforged.json` (split authoring index)
  - `Simulation/src/scripts/runes/effects.rs`
  - `Simulation/src/scripts/runtime/loadout_runtime.rs`
- Coverage registry and tests:
  - `Simulation/src/scripts/coverage.rs`
  - `Simulation/src/scripts/tests/coverage_tests.rs`
  - `Simulation/src/scripts/runtime/tests/loadout_runtime_tests.rs`

## Data Provenance And Research Standards
- Source hierarchy:
  - Tier 1: Riot/Data Dragon/official patch notes
  - Tier 2: CommunityDragon and League Wiki data templates/pages for formula clarification
  - Tier 3: Meta/build sites only for scenario presets, never canonical mechanics
- For champion/item/rune data updates:
  - Add or update `sources` entries on the owning JSON.
  - Record URL/path, `accessed` date, and `used_for`.
  - Verify cited source URLs resolve (prefer HTTP 200 at authoring time) and avoid stale endpoint variants.
  - Keep patch `data_version` aligned to the researched patch.
  - If a mechanic is ambiguous, record it in `Simulation/CONFIDENCE_REVIEW.md`.
  - Web research is acceptable and encouraged for behavior verification and formula clarification before finalizing structured effects.
  - For low-confidence formula interpretations, include at least one page-level verification source (for example item page or patch notes), not only dataset-level citations.
  - If an item-page citation resolves through a redirect (pseudo-item/turret-item/minion-item/champion-upgrade identities), cite both the redirect URL and the canonical parent gameplay/champion page used for semantic verification.
  - For manual confidence increases, perform an entity-intent review first (what the item/rune is for in gameplay and whether structured effects capture that role).
  - Keep `schema_notes.effects_structured_reviewed` normalized as `YYYY-MM-DD`.
  - For CommunityDragon item-dataset citations, use `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/items.json`.
  - For item `stats` keys, use loader-canonical key names (for example `magicResist`, `critChance`, not legacy variants like `magicResistance` or `criticalStrikeChance`) to avoid silent stat drops during ingestion.
  - If an item is retained only as legacy/reference data, add explicit `lifecycle` metadata with `status`, `exclude_from_simulation = true`, `reason`, `replacement_item`, and `replacement_id`.
  - If research uncovers known bug-specific behavior, document it, but keep intended behavior as canonical for simulation data and do not model bug behavior in canonical data.
  - For mode-variant item behavior, keep root fields as Tier-1 baseline semantics and encode only mode-specific divergence under `mode_overrides.<mode_key>` (for example `mode_overrides.URF`, `mode_overrides.ARENA`).
  - If `mode_overrides` branches are added before runtime mode-overlay consumption exists, add explicit mode-page citations and log deferred runtime follow-up in `Simulation/COVERAGE_GAPS.md` in the same change.

## In-Game Behavior Fidelity Review (Required)
Command audits are necessary but not sufficient. For non-trivial data edits, perform manual behavior review before and after editing.

For each edited champion ability, item effect, or rune effect where behavior timing matters:
- Activation model:
  - What player input triggers it (cast, attack, proc condition)?
  - Does it require a target, and does it fail outside range or eligibility constraints?
- Timing model:
  - Is there windup/cast time, projectile travel, or delayed followup?
  - Does it interact with basic-attack cadence (for example attack resets or empowered-hit timing)?
  - Is the effect instantaneous, on-hit, on-cast, or on-resolution?
  - For multi-phase actives (for example mark, timeout return, recast return), are all branches and cooldown-start timing explicitly captured?
  - For single-use transform actives, is the post-use state transition (for example shattered/no-reactivation state) explicitly captured?
- Outcome model:
  - Who is affected (self, ally, enemy, structures, champions-only)?
  - What is player-visible timing (when damage/heal/shield/control actually appears)?
- Exclusion and edge-trigger model:
  - Are source-defined exclusions/edge rules captured (for example dodged/blocked/blinded misses, spell-shield bypass handling, proc-damage classification, zero-damage proc eligibility, below-threshold retrigger behavior)?

Example fidelity pattern:
- Treat targeted empowered-hit casts as timed, gated actions rather than instant tooltip math; encode both cast eligibility and hit-resolution timing.

Documentation and confidence rules for behavior fidelity:
- Capture verified execution semantics in `schema_notes.context_notes` when semantics or confidence changed.
- Keep `parse_confidence` conservative until activation and timing semantics are source-verified.
- If execution semantics remain ambiguous after research, log the ambiguity in `Simulation/CONFIDENCE_REVIEW.md` and track follow-up in `Simulation/COVERAGE_GAPS.md`.
- If semantics are clear in data but current runtime cannot express them yet, add explicit deferred runtime follow-up scope in `Simulation/COVERAGE_GAPS.md` in the same change.
- Prefer explicit notes over silent assumptions when behavior is not obvious from tooltip formula text alone.
- For known bug-vs-intended discrepancies, encode intended behavior in structured data by default and document bug-divergence observations as notes only.
- Project policy lock: intended non-bug behavior is the default canonical simulation model; known bug behavior is not modeled in canonical data.

## Data-First Priority Queue (Current)
This queue is ordered by impact and tracking value.

Priority A. Champion corpus fidelity normalization (highest impact):
- Keep canonical champion file parity at `172/172` via `Simulation/champion_data_coverage_inventory.json`.
- Run targeted manual behavior-fidelity re-verification waves on touched/high-impact champions so non-trivial abilities keep execution-semantics notes aligned to in-game behavior.
- Track manual verification coverage in `Simulation/champion_behavior_verification_tracker.json` and keep full-corpus no-regression (`172/172` manual verified; `0` source-extracted-only).
- Keep tracker integrity auditable each wave: `manual_behavior_verified_champion_keys` length must match `totals.manual_behavior_verified_champions`, and `source_extracted_only_champions` must equal corpus-total minus verified-count.
- For each manually verified champion in a wave, record at least one page-level champion ability citation in the champion `sources` list and in the wave entry of `Simulation/champion_behavior_verification_tracker.json`.
- Maintain full-corpus active-ability execution no-regression (`682/682`) and full ability context-note no-regression (`860/860`).
- Maintain champion ability `description_source` no-regression at `860/860` and treat new missing entries as blocking data-quality regressions.
- Run targeted provenance-hardening passes so fallback-derived `description_source` values are replaced with direct source-corpus strings on high-impact champions over time.
- Run dedicated cleanup passes for generated/truncated context-note fragments (for example `for 0.` / `within 4.`) and pair each correction wave with tracker updates and provenance notes.
- Source-extracted-only strict-fragment queue is `0` champions (cleared and held in waves 63-75); keep this as a no-regression guardrail while maintaining full manual-verification coverage (`172/172`).
- Use strict fragment detection in truncation audits (integer-dot token patterns) so valid decimal timings (for example `0.25 seconds`) are not tracked as truncation defects.
- Run a secondary cadence-fragment sweep (for example `every 0.` patterns) so periodic-tick truncation fragments are not missed by the strict primary pattern.
- Run a tertiary article-token sweep (for example `during the 0.` patterns) so article-interposed truncation fragments are not missed.
- Run a cast-time fragment sweep (for example `have a 0.` patterns) so cast-windup truncation defects are not missed.
- Normalize `context_notes` shape before truncation audits (support both string and array forms) so fragment queues are complete and reproducible.

Priority B. Runes split-structure synchronization:
- Keep split rune structure (`Masteries/RunesReforged/`) synchronized with flat compatibility file (`Masteries/RunesReforged.json`) on every rune/mastery edit.
- Keep per-tree primary and secondary files complete for all five trees (`Domination`, `Inspiration`, `Precision`, `Resolve`, `Sorcery`) plus stat shard parity file.
- Track any split-vs-flat drift as blocking data-quality regression.

Priority C. Champion inventory and planning hygiene:
- Refresh `Simulation/champion_data_coverage_inventory.json` after every champion denominator/parity change (new champion keys).
- Keep champion fidelity wave scope explicit (recommended 10-20 champions per pass) and close each wave with realistic confidence/provenance notes.

1. Modeled runtime confidence floor maintenance:
- Current modeled runtime set has numeric confidence coverage with minimum `parse_confidence >= 0.70`.
- Keep this as a no-regression guardrail while refining complex per-target/proc semantics.

2. Low-confidence structured effect cleanup:
- Raise low-confidence item/rune structured effects with manual normalization.
- Track unresolved low-confidence entries in `Simulation/COVERAGE_GAPS.md`.
- Current low-confidence item queue (`0` files):
  - None currently (`parse_confidence < 0.65` queue cleared).
- Current structured-item missing/null parse-confidence queue (`0` files, `0` effect entries):
  - None currently; keep numeric `parse_confidence` coverage as a no-regression guardrail on touched structured effects.

3. Page-level citation depth for ambiguous formulas:
- Keep Tier-2 dataset citations in place and add page-level references where formulas remain low-confidence.
- High-impact legal URF unmodeled queue is now cleared (`0/102` without page-level citations); keep this as a no-regression guardrail while expanding broader page-depth coverage.

4. Runtime-modeled item condition-token compatibility audit:
- Keep runtime-modeled item condition vocabulary aligned with parser/loader expectations.
- Treat token changes that can alter loader parsing as compatibility-sensitive data updates and track them in coverage docs.
- Include active-cast metadata completeness checks (cooldown + cast range) on targeted active effects to avoid under-specified execution data.

5. Source endpoint and sell-value no-regression:
- Keep CommunityDragon item dataset citations on the current endpoint (`global/default/v1/items.json`) and verify URL health in audits.
- Keep `shop.prices.sell` reconciliation against Tier-1 dataset values as a recurring check on item-economy edits, with explicit exception-register tracking for intentional page-level overrides.

6. Legal URF page-level follow-up (runtime-filtered denominator):
- Runtime-filtered legal URF pool currently tracks `113` legendary items (`111` with effect payload, `102` unmodeled effect items).
- Current no-page-citation queue is `0/102` after wave 31 (cleared).
- Maintain this queue at `0` and treat regressions as blocking.
- Broader structured no-page queue is now cleared (`0/243`); keep this as a no-regression guardrail and maintain page-level depth on future complex edits.
- Current higher-priority no-page subset (ranked `EPIC`/`LEGENDARY`/`BOOTS`/`STARTER`) is cleared (`0` items).
- Keep intentional No Sell support-quest sell-value overrides (`World Atlas`, `Runic Compass`, `Bounty of Worlds`) and shared support-income rule-table precision as maintenance follow-up.

7. Champion provenance cleanup:
- Keep champion source lists deduplicated and concise during ongoing edits.

8. Structured-source metadata normalization:
- Structured-item `sources[].accessed` metadata normalization is complete (`0/243` missing).
- Keep this as a no-regression guardrail on future item data edits.

9. Item stat-key canonicalization no-regression:
- Keep item stat keys aligned with loader-supported names (`magicResist`, `critChance` canonical).
- Treat reintroduction of legacy stat keys (for example `magicResistance`, `criticalStrikeChance`) as a blocking data-quality regression.
- Current migration status: legacy `criticalStrikeChance` usage is `0` files; keep canonical `critChance` usage as a no-regression guardrail.

10. Legacy ID/version drift tracking:
- Maintain explicit exception tracking when current Tier-1 item IDs map to different names than legacy files.
- Current tracked case: `Zephyr` file shares ID `3172` with current Tier-1 `Gunmetal Greaves`.

11. Distributed-item economy representation:
- Keep explicit `shop.prices` representation on distributed/prismatic items when Tier-1 datasets provide economy values.
- Current coverage is `57/57` distributed items with explicit `shop.prices`; keep this as a no-regression guardrail while preserving acquisition-scope notes and documenting page-vs-dataset economy presentation drift.

12. Mode-scoped effect semantics:
- For shared-ID effects that differ by mode timing/values (for example Sudden Death effects), encode current-mode scope explicitly in structured conditions and `schema_notes.context_notes`.
- Track unresolved canonical mode-policy decisions in `Simulation/COVERAGE_GAPS.md` before broad confidence increases.

13. Legacy/replaced item lifecycle policy:
- When current Tier-1 datasets no longer map an identity to the local file's gameplay meaning, keep the file only with explicit lifecycle exclusion metadata (`exclude_from_simulation = true`) and replacement/reason notes.
- Track lifecycle-policy exceptions in `Simulation/COVERAGE_GAPS.md` so legacy data does not silently re-enter active simulation pools.

14. Dragonheart acquisition-round fidelity:
- Canonical structured effects include immediate backfill branch with explicit page-published round-threshold brackets (`3-4 => 1`, `5-6 => 2`, `7-8 => 3`, `9+ => 4`).
- Keep this table as a no-regression guardrail and re-verify only when source tables change.

15. Champion-ability upgrade pseudo-item policy:
- Define canonical expectations for pseudo-item upgrade entries tied to champion abilities (for example `Fire at Will`), including source provenance shape and simulation-use eligibility semantics.

16. Trigger-exclusion and proc-edge normalization plan (documented):
- Phase 1: inventory on-hit/proc items that still lack explicit exclusion/edge semantics (shield interactions, miss/parry/blind exclusions, spell-shield handling, zero-damage trigger behavior).
- Phase 2: normalize to shared condition/modifier vocabulary and add page-level execution notes.
- Phase 3: run no-regression audits for trigger-token consistency/confidence floors and update queue metrics.

17. Distributed/prismatic economy rollout plan (documented):
- Phase 1 (completed): backfill `shop.prices.total`/`shop.prices.sell` on all distributed/prismatic files when Tier-1 provides explicit values (`57/57` explicit).
- Phase 2 (in progress): document acquisition scope and page-vs-dataset economy drift on each touched file.
- Phase 3 (pending): classify each discrepancy as accepted canonical, intentional override, or unresolved follow-up in coverage docs with explicit counts.

18. Policy-resolution plan (documentation-first):
- Resolve distributed availability/map-overlay policy (dataset map flags vs page-level mode scope) using a canonical `mode_overrides.<mode>` shape that stores only divergent fields and keeps Tier-1 baseline at root.
- Resolve legacy ID/name drift handling for deterministic Tier-1 reconciliation by standardizing `lifecycle` metadata requirements (`status`, `exclude_from_simulation`, `reason`, `replacement_item`, `replacement_id`) for legacy-reference files.
- Sync policy outcomes across standards/checklist/gaps in the same change when decisions land.

19. Champion source metadata no-regression:
- Maintain champion provenance completeness at `0/173` unsourced files and `0/173` files with missing `sources[].accessed`.
- Keep `Characters/ChampionDefaults.json` sourced and update provenance when champion-default envelopes are revised.

20. Mastery source metadata no-regression:
- Maintain mastery provenance completeness at `0/2` unsourced files and `0/2` files with missing `sources[].accessed`.
- Keep `Masteries/Season2016.json` and `Masteries/RunesReforged.json` source metadata normalized on touched entries.

21. Champion active-ability execution metadata no-regression:
- Maintain `682/682` champion active abilities with non-empty `execution` objects.
- Treat missing active execution objects as a blocking data-quality gap unless source data is unavailable and explicitly documented.

22. Rune decimal-spacing normalization no-regression:
- Keep rune `effects_structured.raw` decimal formatting normalized (avoid `x. y` artifacts) while preserving source meaning.
- When correcting raw decimal spacing, update dependent numeric fields (`numbers_extracted`, and when affected `value_range` / `scaling` / `formula`) and verify flat/split parity remains exact.

23. Rune multi-branch semantic-explicitness no-regression:
- Keep `semantic_components` populated for touched rune effects that use multi-branch `formula.type = per_rank` values (for example melee/ranged, AD/AP, threshold progression).
- Keep flat/split parity exact when adding or adjusting semantic-component branch labels and values.
- If runtime does not yet consume a new semantic-component branch, document deferred code follow-up in `Simulation/COVERAGE_GAPS.md` in the same change.

24. Champion execution-metadata expressiveness follow-up:
- For movement abilities with formula-based velocity (for example `base + movement speed`), preserve formula semantics in notes and track runtime follow-up when `execution` stores only base speed.
- For multi-stage same-slot abilities, document stage windows/recast gating and track runtime follow-up if stage identity is not first-class.
- For cast-distance-scaled area-size abilities, document min/max radius behavior and track runtime follow-up if current runtime cannot interpolate dynamic radius.

### Progress Snapshot (2026-02-24)
- Note: historical bullets below retain per-wave baseline counts captured when each wave landed; use top-of-file "Current Data Reality" and `Simulation/COVERAGE_GAPS.md` for current totals.
- Completed in current data-first lane:
  - Completed champion execution-semantics wave 103 (`Nasus` `Spirit Fire`):
    - manually re-verified delayed first-impact timing, periodic zone-tick cadence, and armor-reduction linger semantics
    - expanded context-note execution semantics and schema notes with page-level template provenance
  - Completed champion execution-semantics wave 104 (`Renekton` `Slice and Dice`):
    - manually re-verified two-stage recast gating, traversal-hit timing, and move-speed-scaled dash-velocity semantics
    - documented deferred runtime follow-up for formula-based dash-speed consumption (`760 + 100% movement speed`)
  - Completed champion execution-semantics wave 105 (`Sylas` `Chain Lash`, `Abscond`):
    - repaired delayed-detonation truncation defects on `Chain Lash` effect notes (`After a 0.` -> full `0.6-second` semantics)
    - re-verified `Abscond` stage-one/stage-two timing-window semantics and logged deferred runtime follow-up for explicit multi-stage ability identity handling
  - Completed champion execution-semantics wave 106 (`Vex` `Looming Darkness`):
    - manually re-verified projectile-to-explosion timing, cast-distance-scaled radius behavior (`200 : 300`), and Doom flee-origin semantics
    - documented deferred runtime follow-up for dynamic radius interpolation so runtime does not flatten this to a static hitbox
  - Completed champion cast-timing fidelity wave 91:
    - manually re-verified attack-speed-scaled cast gating and player-visible hit resolution on `Jinx` (`W`), `Kai'Sa` (`E`), `Yone` (`Q`,`W`), and `Zeri` (`W`)
    - added page-level ability template citations for each touched champion and recorded wave scope in `Simulation/champion_behavior_verification_tracker.json`
    - corrected discovered truncation defect in `Yone` `Fate Sealed` context notes (`Yone will blink after a 0.` -> full follow-up timing semantics)
  - Completed item source-reconciliation wave 92 (`Twilight's Edge`):
    - aligned canonical Material/Spirit AD/AP modifiers to current page-verified `25%` values and documented infobox/module description divergence
    - encoded level-1-to-18 tooltip tables for Material attack-speed (`50-150%`) and Spirit ability-haste (`30-120`) scaling branches
  - Completed item confidence-resolution wave 93 (`Dragonheart`):
    - replaced inferred immediate-backfill thresholds with explicit page-published acquisition-round table values (`3-4 => 1`, `5-6 => 2`, `7-8 => 3`, `9+ => 4`)
    - raised immediate-backfill confidence and removed inference-only language from canonical context notes
  - Completed runes decimal/value normalization wave 94:
    - normalized decimal spacing and corrected structured numeric/value metadata on `Electrocute`, `Dark Harvest`, `Press the Attack`, and `Lethal Tempo`
    - applied equivalent corrections in flat and split rune files to preserve parity
  - Completed runes decimal/value normalization wave 95 (`Domination` + `Precision` tranche):
    - normalized decimal-spacing artifacts and synchronized dependent numeric metadata for high-impact non-keystone entries (`Cheap Shot`, `Taste of Blood`, `Triumph`, `Presence of Mind`, `Conqueror`, `Fleet Footwork`, `Legend` variants)
    - preserved flat/split parity during the batch update
  - Completed runes decimal/value normalization wave 96 (`Resolve` tranche):
    - normalized decimal artifacts and dependent metadata on `Grasp of the Undying`, `Aftershock`, `Guardian`, `Font of Life`, `Shield Bash`, `Bone Plating`, and `Overgrowth`
    - corrected range/min-max metadata where decimal spacing had previously split values
  - Completed runes decimal/value normalization wave 97 (`Sorcery` tranche):
    - normalized decimal artifacts and dependent metadata on `Summon Aery`, `Arcane Comet`, `Phase Rush`, `Absolute Focus`, `Scorch`, `Waterwalking`, and `Gathering Storm`
    - reconciled by-level min/max metadata for affected range-formula entries
  - Completed runes decimal/value normalization wave 98 (cross-tree no-regression repair/audit tranche):
    - cleared remaining broad decimal-spacing queue (`28` entries across `27` runes -> `0`)
    - validated no literal scripted-placeholder artifacts remain in rune raw text and confirmed flat/split synchronization after batch normalization
    - documented deferred runtime code follow-up for loader/lint enforcement of raw-to-metadata consistency checks
  - Completed rune semantic-explicitness wave 99 (`Inspiration` tranche):
    - added explicit `semantic_components` decomposition for multi-branch `per_rank` effects on `First Strike` and `Jack of All Trades`
    - documented trigger-window/cadence/threshold semantics with named branches instead of positional-only value interpretation
  - Completed rune semantic-explicitness wave 100 (`Precision` tranche):
    - added explicit `semantic_components` decomposition for `Lethal Tempo`, `Fleet Footwork`, `Conqueror`, and `Presence of Mind` multi-branch `per_rank` effects
    - encoded melee/ranged, stack-cadence, and AD/AP branch semantics explicitly
  - Completed rune semantic-explicitness wave 101 (`Resolve` tranche):
    - added explicit `semantic_components` decomposition for `Grasp of the Undying`, `Demolish`, and `Font of Life` multi-branch `per_rank` effects
    - encoded stack windows, turret branch values, and ally-heal radius/progression semantics explicitly
  - Completed rune semantic-explicitness wave 102 (`Sorcery` tranche):
    - added explicit `semantic_components` decomposition for `Phase Rush` and `Gathering Storm` multi-branch `per_rank` effects
    - completed full `per_rank` semantic decomposition baseline (`14/14`), with deferred runtime consumption/enforcement logged for code follow-up
  - Completed item parse-confidence completeness wave 87:
    - backfilled missing/null `effects_structured[].parse_confidence` values across `12` remaining effect entries in `5` item files (`Trinity Force`, `Twilight's Edge`, `Warden's Eye`, `Wooglet's Witchcap`, `Zeke's Convergence`)
    - cleared structured-item missing/null parse-confidence queue to `0` entries (`0/243` files with gaps)
  - Completed champion attack-cadence fidelity wave 88:
    - manually re-verified cast-gating and hit-resolution timing semantics on `Jax` (`W`), `Renekton` (`W`), `Rengar` (`Q`), and `MonkeyKing` (`Q`)
    - added page-level ability template citations for each touched champion and recorded wave scope in `Simulation/champion_behavior_verification_tracker.json`
    - corrected `Renekton` `Ruthless Predator` truncation defects and aligned empowered branch stun-duration semantics to source-verified values
  - Completed champion truncation-correction wave 89:
    - corrected `Braum` `Glacial Fissure` first-target knockup truncation (`at least 0.` -> `at least 0.6 seconds`) and clarified travel-distance-scaled maximum duration semantics
    - recorded page-level citation provenance and wave scope in `Simulation/champion_behavior_verification_tracker.json`
  - Completed runes cadence-text normalization wave 90:
    - corrected `Lethal Tempo` interval spacing artifacts (`0. 5` -> `0.5`) in flat and split rune structures
    - synchronized parsed numeric extraction (`numbers_extracted: [0.5]`) to maintain flat/split semantic parity
  - Completed champion provenance-hardening wave 81:
    - re-verified and hardened source-corpus `description_source` phrasing for `Mordekaiser` (`R`), `Pyke` (`Q`), `Sion` (`Q`), and `Urgot` (`R`)
    - added page-level champion ability citation provenance for all touched champions and recorded scope in `Simulation/champion_behavior_verification_tracker.json`
  - Completed champion provenance-hardening wave 80:
    - re-verified and hardened source-corpus `description_source` phrasing for `Braum` (`R`), `Lux` (`R`), `Pantheon` (`Q`), and `Poppy` (`R`)
    - repaired a discovered truncated context-note fragment in `Poppy` `Keeper's Verdict` while hardening provenance
    - added page-level champion ability citation provenance for all touched champions and recorded scope in `Simulation/champion_behavior_verification_tracker.json`
  - Completed item confidence-reconciliation wave 82:
    - (historical wave-82 baseline) encoded `Dragonheart` acquisition-round immediate backfill threshold brackets as inferred structured modifiers (`3-4 => 1`, `5-6 => 2`, `7-8 => 3`, `9+ => 4`) based on published cadence/cap constraints
    - reconciled `Gambler's Blade` historical V14.12 `245` cap note against current canonical `240` cap and documented historical-only context metadata
  - Completed data-quality audit wave 83:
    - re-ran split-vs-flat rune parity checks (`61/61` rune IDs, `3/3` stat shard slots)
    - re-ran champion no-regression audits (tracker integrity, `description_source` completeness, truncation queues) and confirmed clean state
  - Completed full-corpus champion `description_source` backfill pass:
    - backfilled remaining missing champion ability `description_source` entries from canonical ability `description` text when source strings were absent
    - raised champion ability `description_source` completeness from `784/860` to `860/860` (`0` missing across `0` champions)
    - reclassified champion `description_source` tracking from backlog to no-regression maintenance
  - Completed champion fidelity normalization wave 75:
    - manually reviewed and normalized execution-semantics notes across `3` champions (`Teemo`, `Viego`, `Zyra`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `169/172` to `172/172` (full manual-verified corpus)
    - backfilled `description_source` for touched missing entries (`Viego` `Spectral Maw`)
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed champion fidelity normalization wave 74:
    - manually reviewed and normalized execution-semantics notes across `2` champions (`Ivern`, `Malphite`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `167/172` to `169/172`
    - backfilled `description_source` for touched missing entries (`Ivern` `Friend of the Forest`)
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed champion fidelity normalization wave 73:
    - manually reviewed and normalized execution-semantics notes across `2` champions (`Hwei`, `Illaoi`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `165/172` to `167/172`
    - backfilled `description_source` for touched missing entries (`Illaoi` `Tentacle Smash`)
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed champion fidelity normalization wave 72:
    - manually reviewed and normalized execution-semantics notes across `2` champions (`Chogath`, `Heimerdinger`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `163/172` to `165/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed champion fidelity normalization wave 71:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Rakan`, `Rammus`, `RekSai`, `Renata`, `TwistedFate`, `Zilean`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `157/172` to `163/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed champion fidelity normalization wave 70:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Elise`, `Evelynn`, `Kalista`, `Khazix`, `Kindred`, `Mordekaiser`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `151/172` to `157/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed champion fidelity normalization wave 69:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Zeri`, `Tristana`, `Twitch`, `Syndra`, `Veigar`, `Senna`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `145/172` to `151/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed champion fidelity normalization wave 68:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Ambessa`, `Annie`, `Ashe`, `Ezreal`, `Lucian`, `Yasuo`) with page-level champion ability citation provenance
    - normalized `3` non-primary truncation defects discovered during manual review (`Ambessa`, `Evelynn`, `Senna`) before closing the wave
    - raised manual verification tracker coverage from `139/172` to `145/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - Completed post-wave article-token truncation cleanup sweep:
    - normalized `1` additional article-interposed timing fragment (`during the 0.` class) on `TahmKench` (`Abyssal Dive`) to complete source-backed delay semantics
    - recorded tertiary article-token sweep requirement in standards/checklist so this class is audited on future waves
    - kept strict primary, secondary cadence, and tertiary article-token truncation queues at `0/0` after post-edit audits
  - Completed touched-wave `description_source` backfill and backlog tracking:
    - backfilled missing `description_source` on touched abilities (`Mordekaiser` `Realm of Death`, `Rammus` `Powerball`) from source corpus
    - quantified remaining champion-ability `description_source` backlog (`79/860` missing across `58` champions) at wave-71 baseline and documented as active follow-up in `Simulation/COVERAGE_GAPS.md`
  - Completed secondary cadence-fragment cleanup sweep:
    - normalized `5` additional periodic-tick truncation fragments (`every 0.` class) across `2` champions (`Nasus`, `Fiora`) using source-backed full cadence semantics
    - added page-level champion ability citation provenance for touched cadence entries (`Fury of the Sands`, `Grand Challenge`)
    - recorded cleanup scope in `Simulation/champion_behavior_verification_tracker.json` as manual verification wave 67 (`Nasus`, `Fiora`)
    - kept both strict primary and secondary cadence truncation queues at `0/0` after post-edit audits
  - Completed champion fidelity normalization wave 66:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Camille`, `Fiddlesticks`, `Jax`, `Rumble`, `Yorick`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - manual verification tracker coverage remained `139/172` (already-manual-verified champion set touched)
    - reduced string+array-aware strict-fragment queue from `6/5` to `0/0` (queue cleared)
  - Completed champion fidelity normalization wave 65:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Nautilus`, `Olaf`, `Swain`, `XinZhao`, `Nasus`) with page-level champion ability citation provenance
    - normalized `13` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - manual verification tracker coverage remained `139/172` (already-manual-verified champion set touched)
    - reduced string+array-aware strict-fragment queue from `19/10` to `6/5`
  - Completed champion fidelity normalization wave 64:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Zaahen`, `Jhin`, `Yone`, `Darius`, `Ekko`) with page-level champion ability citation provenance
    - normalized `15` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `136/172` to `139/172`
    - reduced string+array-aware strict-fragment queue from `34/15` to `19/10`
  - Completed champion fidelity normalization wave 63:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Caitlyn`, `Poppy`, `Quinn`, `Shen`, `Tryndamere`) with page-level champion ability citation provenance
    - normalized `5` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `131/172` to `136/172`
    - reduced string+array-aware strict-fragment queue from `39/20` to `34/15`
  - Completed champion fidelity normalization wave 62:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Ryze`, `Sett`, `Sivir`, `TahmKench`, `Thresh`, `Xayah`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `125/172` to `131/172`
    - reduced string+array-aware strict-fragment queue from `46/26` to `40/20`
  - Completed champion fidelity normalization wave 61:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Graves`, `Hecarim`, `JarvanIV`, `Kassadin`, `Kennen`, `Pyke`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `119/172` to `125/172`
    - reduced string+array-aware strict-fragment queue from `52/32` to `46/26`
  - Completed champion fidelity normalization wave 60:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`LeeSin`, `Leona`, `Lulu`, `Brand`, `Braum`, `Diana`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `113/172` to `119/172`
    - reduced string+array-aware strict-fragment queue from `58/38` to `52/32`
  - Completed champion fidelity normalization wave 59:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Akali`, `Alistar`, `Amumu`, `Yunara`, `Kaisa`, `Kayn`) with page-level champion ability citation provenance
    - normalized `18` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `107/172` to `113/172`
    - reduced string+array-aware strict-fragment queue from `68/44` to `58/38`
  - Completed champion fidelity normalization wave 58:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Jayce`, `Janna`, `Gragas`, `Gangplank`, `Blitzcrank`, `Azir`) with page-level champion ability citation provenance
    - normalized `12` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `101/172` to `107/172`
    - reduced string+array-aware strict-fragment queue from `80/50` to `68/44`
  - Completed champion fidelity normalization wave 57:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Pantheon`, `Ornn`, `Nidalee`, `Nami`, `MissFortune`, `Karma`) with page-level champion ability citation provenance
    - normalized `12` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `95/172` to `101/172`
    - reduced string+array-aware strict-fragment queue from `92/56` to `80/50`
  - Completed champion fidelity normalization wave 56:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Vex`, `Taric`, `Sylas`, `Rengar`, `Renekton`, `Qiyana`) with page-level champion ability citation provenance
    - normalized `12` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `89/172` to `95/172`
    - reduced string+array-aware strict-fragment queue from `104/62` to `92/56`
  - Completed champion fidelity normalization wave 55:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Zac`, `Viktor`, `Skarner`, `Rell`, `Orianna`, `Zoe`) with page-level champion ability citation provenance
    - normalized `17` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `83/172` to `89/172`
    - reduced string+array-aware strict-fragment queue from `121/68` to `104/62`
  - Completed champion fidelity normalization wave 50:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Talon`, `Smolder`, `Sejuani`, `Nunu`, `Nilah`, `Maokai`) with page-level champion ability citation provenance
    - normalized `30` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `53/172` to `59/172`
    - reduced string+array-aware strict-fragment queue from `249/100` to `219/94`
  - Completed champion fidelity normalization wave 49:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Mel`, `Katarina`, `Corki`, `Velkoz`, `Singed`, `Ziggs`) with page-level champion ability citation provenance
    - normalized `33` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - corrected tracker-key integrity drift by adding missing `Lissandra` to `manual_behavior_verified_champion_keys` and re-reconciling totals
    - raised manual verification tracker coverage from `47/172` to `53/172`
    - reduced string+array-aware strict-fragment queue from `282/106` to `249/100`
  - Completed champion fidelity normalization wave 48:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Udyr`, `Sion`, `Lissandra`, `AurelionSol`, `Naafiri`) with page-level champion ability citation provenance
    - normalized `29` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `42/172` to `47/172`
    - reduced string+array-aware strict-fragment queue from `309/108` to `280/107`
  - Completed champion fidelity normalization wave 47:
    - manually reviewed and normalized timing-fragment execution semantics across `18` champions (`Akshan`, `Aurora`, `Briar`, `Gnar`, `Irelia`, `Leblanc`, `Lillia`, `Lux`, `MonkeyKing`, `Nocturne`, `Riven`, `Shaco`, `Taliyah`, `Urgot`, `Varus`, `Xerath`, `Yuumi`, `Zed`)
    - normalized `51` strict fragment candidates in touched champion ability/effect `context_notes` to complete source-backed timing semantics
    - raised manual verification tracker coverage from `25/172` to `42/172`
    - corrected truncation-audit implementation to support both string and array `context_notes`, then re-baselined corrected-method queue from `360/123` to `309/108` after wave cleanup
  - Completed champion fidelity normalization wave 46:
    - manually reviewed execution-semantics behavior on `Fiddlesticks` (`Q`, `W`, `E`), `Rumble` (`Q`, `R`), `Swain` (`Q`, `R`), and `Nautilus` (`E`, `R`)
    - normalized candidate truncated timing fragments on all touched abilities and added page-level ability verification sources for each touched champion
    - raised manual verification tracker coverage from `21/172` to `25/172`
    - tightened truncation-audit criteria to strict fragment detection (excluding valid decimals like `0.25`); superseded by string+array-aware audit baseline in wave 47
  - Completed champion fidelity normalization wave 45:
    - manually reviewed attack-cadence-coupled execution semantics for `Camille` (`Q`), `Ekko` (`E`), `Trundle` (`Q`), `Volibear` (`W`), `Vi` (`E`), and `Yorick` (`Q`)
    - added page-level ability verification sources to each touched champion and raised manual verification tracker coverage from `15/172` to `21/172`
    - fixed targeted truncated context-note fragments in touched abilities while preserving canonical execution metadata
    - logged broader generated-context truncation backlog (`225` ability entries across `132` champion files) as an explicit tracked data-quality follow-up (superseded by strict-fragment rebaseline in wave 46)
  - Completed champion fidelity normalization wave 44:
    - closed full-corpus active-ability execution gaps (`682/682` with non-empty `execution`)
    - closed full-corpus champion ability context-note gaps (`860/860` with non-empty `context_notes`)
    - refined attack-cadence-coupled execution semantics for `Olaf` `Reckless Swing` and normalized missing context/execution metadata on `Jax`, `Leblanc`, `Aphelios`, `Vladimir`, `Sona`, `DrMundo`, `Morgana`, `Vayne`, and `Warwick`
    - revalidated champion corpus/data provenance guardrails (`172/172` parity and `0/173` source/accessed gaps in `Characters/*.json`)
  - `9` modeled runtime item files now have explicit `sources`.
  - `8` preset-critical item files now have explicit `sources` (including `Stridebreaker`).
  - `23` additional item files had both source provenance backfilled and legacy non-ISO review metadata normalized.
  - `30` additional legal-URF, low-confidence unmodeled item files had source provenance backfilled.
  - `51` remaining legal-URF unmodeled effect items with missing `sources` were backfilled.
  - Item files with `effects_structured` and `sources: null` reduced from `226` to `0`.
  - Unmodeled legal URF items with `effects_structured` and missing `sources` reduced from `81` to `0`.
  - Tier-2 citation coverage (`CommunityDragon` and/or League Wiki) increased to `243/243` for structured item files.
  - Modeled runtime item files with minimum `parse_confidence <= 0.60` were reduced from `5` to `0` after manual behavior-aligned normalization.
  - Item low-confidence backlog (`parse_confidence < 0.65`) reduced from `95` to `85` files.
  - Non-ISO/missing item review metadata reduced from `23` non-ISO + `0` missing to `0` total.
  - `Bloodthirster` review metadata gap was filled.
  - `Characters/Vladimir.json` duplicate source entry was removed.
  - Added provenance sources to `Characters/ChampionDefaults.json` (project champion corpus + generated champion datasets) to close champion-defaults sourcing gap.
  - Backfilled champion-source `sources[].accessed` metadata on generated-source entries for `DrMundo`, `Morgana`, `Sona`, `Vayne`, `Vladimir`, and `Warwick`, reducing champion missing-accessed backlog to `0/7`.
  - Backfilled `Masteries/Season2016.json` source `accessed` metadata (`6` entries), reducing mastery missing-accessed backlog to `0/2`.
  - Completed champion active-ability execution metadata coverage (`23/23`) by filling missing `execution` objects across `DrMundo` (`W`, `E`), `Morgana` (`W`, `E`), `Sona` (`W`, `E`), `Vayne` (`E`), `Vladimir` (`W`), and `Warwick` (`W`, `E`).
  - Corrected `Vayne` `Silver Bolts` ability type from `Active` to source-aligned `Passive`.
  - Added champion parity inventory file `Simulation/champion_data_coverage_inventory.json` (current snapshot: `6/172` canonical champion files, `166` missing from `Characters/` vs `From Online/champions` denominator).
  - Created split runes structure under `Masteries/RunesReforged/` with top-level index, stat shard file, and per-tree `primary_runes.json` + `secondary_runes.json` files for all five trees.
  - Kept runtime compatibility by retaining flat file `Masteries/RunesReforged.json` while documenting split structure as authoring baseline.
  - Rune entries using `stat_modifier` with null/empty `stat` were reduced from `13` to `0` by converting narrative-only entries to `condition_note`.
  - Restored parser-compatible Protoplasm Lifeline threshold condition token (`health_below_30_percent`) after identifying a data-only compatibility regression.
  - Refined high-impact preset item data (`Stridebreaker`, `Warmog's Armor`, `Titanic Hydra`, `Rabadon's Deathcap`, `Phantom Dancer`) with entity-intent notes and page-level citations.
  - Increased page-level item citation coverage from `17/243` to `22/243`.
  - Refined modeled runtime item semantics for `Heartsteel`, `Kraken Slayer`, and `Liandry's Torment` with higher-confidence formula and trigger metadata after entity-intent review.
  - Refined `Stormsurge` structured effects (including death-trigger discharge semantics) and raised its minimum confidence from `0.60` to `0.70`.
  - Removed modeled-runtime borderline (`0.65`) confidence entries (`3` -> `0`).
  - Normalized modeled `Zhonya's Hourglass` confidence from missing/null to explicit numeric confidence.
  - Refined preset-borderline item data for `Lich Bane`, `Stridebreaker`, and `Titanic Hydra`, raising all three to `>= 0.70` minimum confidence.
  - Refined low-confidence legal URF item data for `Eclipse` and `Rod of Ages`, reducing backlog from `87` to `85`.
  - Increased page-level item citation coverage from `22/243` to `24/243` by adding page-level citations for `Eclipse` and `Rod of Ages`.
  - Completed legal URF low-confidence citation/semantics wave 1 (`Malignance`, `Terminus`, `Sundered Sky`, `Statikk Shiv`, `Fiendhunter Bolts`) and raised each to `>= 0.65` minimum confidence.
  - Completed legal URF low-confidence citation/semantics wave 2 (`Essence Reaver`, `Iceborn Gauntlet`, `Jak'Sho, The Protean`, `Runaan's Hurricane`, `Sunfire Aegis`) and raised each to `>= 0.65` minimum confidence.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `85` to `75`.
  - Increased page-level item citation coverage from `24/243` to `34/243`.
  - Reduced legal URF unmodeled low-confidence/no-page-citation queue from `28` to `18`.
  - Completed legal URF low-confidence citation/semantics wave 3 (`Solstice Sleigh`, `Bloodletter's Curse`, `Dawncore`, `Echoes of Helia`, `Force of Nature`), raising four of five to `>= 0.65` minimum confidence.
  - Added page-level citations and intent/context notes for all five wave-3 items while preserving conservative confidence on unresolved support-income diminishing-gold semantics (`Solstice Sleigh`).
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `75` to `71`.
  - Increased page-level item citation coverage from `34/243` to `39/243`.
  - Reduced legal URF unmodeled low-confidence/no-page-citation queue from `18` to `13`.
  - Completed legal URF low-confidence citation/semantics wave 4 (`Bloodsong`, `Dream Maker`, `Hexoptics C44`, `Hextech Rocketbelt`, `Hollow Radiance`), raising three of five to `>= 0.65` minimum confidence.
  - Added page-level citations and intent/context notes for all five wave-4 items while preserving conservative confidence on support-income diminishing-gold semantics (`Bloodsong`, `Dream Maker`) pending shared-rule-table data.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `71` to `68`.
  - Increased page-level item citation coverage from `39/243` to `44/243`.
  - Reduced legal URF unmodeled low-confidence/no-page-citation queue from `13` to `8`.
  - Completed legal URF low-confidence citation/semantics wave 5 (`Horizon Focus`, `Hullbreaker`, `Knight's Vow`, `Mejai's Soulstealer`, `Opportunity`), raising four of five to `>= 0.68` minimum confidence and one (`Hullbreaker`) to `>= 0.65`.
  - Added page-level citations and intent/context notes for all five wave-5 items.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `68` to `63`.
  - Increased page-level item citation coverage from `44/243` to `49/243`.
  - Reduced legal URF unmodeled low-confidence/no-page-citation queue from `8` to `3`.
  - Completed legal URF low-confidence citation/semantics wave 6 (`Overlord's Bloodmail`, `Ravenous Hydra`, `Redemption`), raising all three to `>= 0.65` minimum confidence.
  - Added page-level citations and intent/context notes for all three wave-6 items.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `63` to `60`.
  - Increased page-level item citation coverage from `49/243` to `52/243`.
  - Reduced legal URF unmodeled low-confidence/no-page-citation queue from `3` to `0` (cleared).
  - Completed legal URF low-confidence support-income precision wave 7 (`Solstice Sleigh`, `Bloodsong`, `Dream Maker`) with page-verified pre/post-5-minute diminishing-gold formulas and explicit shared-rule-table dependency notes.
  - Raised all three wave-7 support-income entries from low confidence (`0.50/0.60/0.60`) to `0.68`.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `60` to `57`.
  - Logged sibling support-quest rule consistency as the next data-quality follow-up (`World Atlas`, `Runic Compass`, `Bounty of Worlds`, `Celestial Opposition`, `Zaz'Zak's Realmspike`).
  - Completed support-quest sibling harmonization wave 8 (`World Atlas`, `Runic Compass`, `Bounty of Worlds`, `Celestial Opposition`, `Zaz'Zak's Realmspike`) for diminishing-gold rule schema, confidence, and page-level provenance.
  - Increased page-level item citation coverage from `52/243` to `57/243`.
  - Closed sibling support-quest diminishing-gold schema/confidence backlog.
  - Logged new follow-up: support-quest starter/epic sell-state representation discrepancy (`World Atlas`, `Runic Compass`).
  - Completed consumable-and-manaflow fidelity wave 9 (`Health Potion`, `Refillable Potion`, `Tear of the Goddess`, `Manamune`) with explicit activation/timing semantics and page-level verification sources.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `57` to `53`.
  - Increased page-level item citation coverage from `57/243` to `61/243`.
  - Corrected Tear-line sibling sell values (`Archangel's Staff`, `Muramana`, `Winter's Approach`, `Fimbulwinter`) to Tier-1 canonical values.
  - Completed source-and-economy reconciliation wave 10:
    - migrated legacy CommunityDragon citation endpoint usage from `235/243` to `0/243`
    - reconciled sell-value mismatches vs Data Dragon from `209/238` to `0/238`
    - manually spot-checked representative starter, legendary, support-quest, and map-specific item outputs after normalization
  - Completed support sell-state policy resolution wave 11:
    - updated quest-stage support items (`World Atlas`, `Runic Compass`, `Bounty of Worlds`) to page-verified No Sell representation (`shop.prices.sell = 0`)
    - recorded explicit Tier-1 discrepancy notes on each affected item (`Data Dragon`/`CommunityDragon` still list `sell = 160`)
    - reclassified sell-value mismatch tracking from unresolved backlog to intentional exception register (`3/238` intentional overrides)
  - Completed rune low-confidence note cleanup wave 11:
    - normalized low-confidence narrative effects across `17` runes into condition-note taxonomy with explicit trigger/owner semantics
    - reduced rune low-confidence backlog (`parse_confidence <= 0.60`) from `17` to `0`
  - Refined `Runic Compass` Shared Riches range/charge/timing semantics and raised its minimum confidence to `0.68`, reducing item low-confidence backlog (`parse_confidence < 0.65`) from `53` to `52`.
  - Completed item execution-semantics and citation wave 12 (`Control Ward`, `Doran's Shield`, `Everfrost`, `Fimbulwinter`, `Demonic Embrace`) with page-level verification and behavior-aligned context notes.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `52` to `47`.
  - Increased page-level item citation coverage from `61/243` to `66/243`.
  - Completed item execution-semantics and provenance wave 13 (`Galeforce`, `Gustwalker Hatchling`, `Mosstomper Seedling`, `Scorchclaw Pup`, `Talisman of Ascension`) with manual behavior review and page-level verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `47` to `42`.
  - Increased page-level item citation coverage from `66/243` to `71/243`.
  - Normalized structured-item source metadata completeness (`sources[].accessed`) from `203/243` missing to `0/243`.
  - Completed item execution-semantics and alignment wave 14 (`Fated Ashes`, `Hellfire Hatchet`, `Hamstringer`, `Sanguine Gift`, `Spectral Cutlass`) with manual behavior review and page-level verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `42` to `37`.
  - Increased page-level item citation coverage from `71/243` to `77/243`.
  - Corrected Tier-1 drift on `Spectral Cutlass` and reconciled `Redemption` total price to Data Dragon `16.3.1`.
  - Normalized legacy item stat key usage from `30` files using `stats.magicResistance` to `0` by converting all to `stats.magicResist`.
  - Reconciled `The Golden Spatula` base stat block and shop pricing to Tier-1 `16.3.1` values with page-level patch-history context.
  - Identified and tracked one cross-version identity exception: `Zephyr` and `Gunmetal Greaves` share ID `3172`.
  - Completed item execution-semantics and citation wave 15 (`Eleisa's Miracle`, `Chainlaced Crushers`, `Cloak of Starry Night`, `Lightning Rod`, `Reverberation`, `Runecarver`) with manual behavior review and page-level verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `37` to `31`.
  - Increased page-level item citation coverage from `77/243` to `83/243`.
  - Reduced low-confidence/no-page-citation queue from `36` to `30`.
  - Corrected `Reverberation` crowd-control trigger fidelity by including grounding in Rumble stack-trigger semantics.
  - Completed item execution-semantics and citation wave 16 (`Crystalline Overgrowth`, `Overcharged`, `Kinkou Jitte`, `Puppeteer`, `Jarvan I's`) with manual behavior review and page-level/official verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `31` to `26`.
  - Increased page-level item citation coverage from `83/243` to `88/243`.
  - Reduced low-confidence/no-page-citation queue from `30` to `25`.
  - Documented mode-variant timing/value differences for `Overcharged` (Clash vs Swiftplay Sudden Death behavior) as a follow-up policy gap.
  - Completed item execution-semantics and citation wave 17 (`Demon King's Crown`, `Detonation Orb`, `Diamond-Tipped Spear`, `Reaper's Toll`, `Sword of the Divine`) with manual behavior review and official/page-level verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `26` to `21`.
  - Increased page-level item citation coverage from `88/243` to `93/243`.
  - Reduced low-confidence/no-page-citation queue from `25` to `20`.
  - Identified secondary page-level depth queue for legal URF unmodeled items (`58/103` currently without page-level citations).
  - Logged a lifecycle-marker policy follow-up: standardize retired/replaced item exclusion metadata beyond the current `Zephyr` case.
  - Completed item execution-semantics and citation wave 18 (`Pyromancer's Cloak`, `Crimson Lucidity`, `Regicide`, `Rite of Ruin`, `Dragonheart`, `Fire at Will`) with manual behavior review and page-level verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `21` to `15`.
  - Increased page-level item citation coverage from `93/243` to `99/243`.
  - Reduced low-confidence/no-page-citation queue from `20` to `14`.
  - Logged Dragonheart acquisition-round soul-backfill behavior as a tracked follow-up fidelity gap (documented in context notes, not yet modeled in structured effects).
  - Completed item execution-semantics and citation wave 19 (`Diadem of Songs`, `Sword of Blossoming Dawn`, `Lifeline`, `Hexbolt Companion`, `Fulmination`) with manual behavior review and page-level verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `15` to `10`.
  - Increased page-level item citation coverage from `99/243` to `104/243`.
  - Reduced low-confidence/no-page-citation queue from `14` to `9`.
  - Logged Lifeline displacement/channel/crowd-control special-interaction semantics as a tracked follow-up fidelity gap.
  - Completed item execution-semantics and citation wave 20 (`Flesheater`, `Force of Entropy`, `Gambler's Blade`, `Guardian's Dirk`, `Gusto`, `Hemomancer's Helm`, `Innervating Locket`, `Reality Fracture`, `Scarecrow Effigy`) with manual behavior review and page-level verification.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `10` to `1`.
  - Increased page-level item citation coverage from `104/243` to `113/243`.
  - Reduced low-confidence/no-page-citation queue from `9` to `0`.
  - Concentrated remaining low-confidence queue on one multi-effect item (`The Golden Spatula`) pending deeper semantic normalization.
  - Completed item execution-semantics and citation wave 21 (`The Golden Spatula`, `Abyssal Mask`, `Ardent Censer`, `Black Cleaver`, `Death's Dance`, `Morellonomicon`) with page-level/module-level verification where relevant.
  - Reduced item low-confidence backlog (`parse_confidence < 0.65`) from `1` to `0`.
  - Increased page-level item citation coverage from `113/243` to `118/243`.
  - Reduced legal URF unmodeled no-page-citation queue from `58/103` to `53/103`.
  - Recorded Golden Spatula mode-scope/source-drift reconciliation as a follow-up fidelity area while clearing the confidence-floor backlog.
  - Completed item execution-semantics and citation wave 22 (`Trinity Force`, `Muramana`, `Sterak's Gage`, `The Collector`, `Wit's End`, `Zeke's Convergence`) with page-level verification and explicit execution-model context notes.
  - Corrected `Zeke's Convergence` Frostfire Tempest tick semantics from placeholder 30-per-tick to verified 7.5-per-0.25s cadence (150 total over 5 seconds), with cooldown-start and champion-only target scope explicitly encoded.
  - Expanded `Muramana` Shock to include champion-ability damage branch (4% melee / 3% ranged max mana) plus cast-instance/per-target limiter and proc-damage exclusion conditions.
  - Increased page-level item citation coverage from `118/243` to `124/243`.
  - Reduced legal URF unmodeled no-page-citation queue from `53/103` to `47/103`.
  - Completed item execution-semantics and citation wave 23 (`Chempunk Chainsword`, `Cosmic Drive`, `Dead Man's Plate`, `Locket of the Iron Solari`, `Maw of Malmortius`, `Rapid Firecannon`) with page-level verification and behavior-model context notes.
  - Corrected `Maw of Malmortius` passive omnivamp metadata from incorrect 30% to page-verified 10% and strengthened Lifeline trigger semantics.
  - Added active cooldown-start semantics for `Locket of the Iron Solari` Devotion and energized stack-generation cadence details for `Rapid Firecannon`.
  - Increased page-level item citation coverage from `124/243` to `130/243`.
  - Reduced legal URF unmodeled no-page-citation queue from `47/103` to `41/103`.
  - Completed item execution-semantics and citation wave 24 (`Cryptbloom`, `Dusk and Dawn`, `Edge of Night`, `Hubris`, `Mikael's Blessing`, `Serpent's Fang`, `Spear of Shojin`) with manual behavior review and page-level verification.
  - Corrected `Mikael's Blessing` active execution metadata by adding page-verified 120-second cooldown and 650 cast-range semantics across cleanse/heal branches.
  - Added `Spear of Shojin` Focused Will per-cast-instance stack-throttle metadata (one stack per second) and refreshed execution-model notes across all seven wave-24 items.
  - Increased page-level item citation coverage from `130/243` to `137/243`.
  - Reduced legal URF unmodeled no-page-citation queue from `41/103` to `34/103`.
  - Logged active-cast cooldown/range metadata completeness as a recurring audit follow-up for targeted item actives.
  - Recalibrated legal-URF denominator tracking to runtime-filter parity (`default_item_pool` + runtime effect-payload detection): `113` legal pool items, `111` with effect payload, `102` unmodeled.
  - Completed item execution-semantics and citation wave 25 (`Bastionbreaker`, `Endless Hunger`, `Experimental Hexplate`, `Frozen Heart`, `Hextech Gunblade`, `Immortal Shieldbow`) with manual behavior review and page-level verification.
  - Corrected `Hextech Gunblade` active metadata completeness with page-verified cooldown (`60s`) and cast range (`700`) semantics.
  - Split `Experimental Hexplate` Overdrive into explicit attack-speed and movement-speed branches with cooldown-start-on-cast timing semantics.
  - Increased page-level item citation coverage from `137/243` to `143/243`.
  - Reduced runtime-filtered legal URF unmodeled no-page-citation queue from `33/102` to `27/102`.
  - Completed item execution-semantics and citation wave 26 (`Axiom Arc`, `Banshee's Veil`, `Imperial Mandate`, `Randuin's Omen`, `Shurelya's Battlesong`, `Youmuu's Ghostblade`) with manual behavior review and page-level verification.
  - Corrected trigger-gating fidelity for `Axiom Arc` and `Imperial Mandate`, and active cooldown/range-or-radius metadata for `Randuin's Omen`, `Shurelya's Battlesong`, and `Youmuu's Ghostblade`.
  - Increased page-level item citation coverage from `143/243` to `149/243`.
  - Reduced runtime-filtered legal URF unmodeled no-page-citation queue from `27/102` to `21/102`.
  - Completed item execution-semantics and citation wave 27 (`Mercurial Scimitar`, `Navori Flickerblade`, `Profane Hydra`, `Riftmaker`, `Unending Despair`, `Voltaic Cyclosword`) with manual behavior review and page-level verification.
  - Corrected active cooldown completeness for `Mercurial Scimitar` (`90s`) and `Profane Hydra` (`10s`) and refreshed execution-timing context notes across all six items.
  - Normalized `Navori Flickerblade` crit stat key to loader-canonical `stats.critChance` and logged remaining `stats.criticalStrikeChance` migration backlog (`23` files).
  - Increased page-level item citation coverage from `149/243` to `155/243`.
  - Reduced runtime-filtered legal URF unmodeled no-page-citation queue from `21/102` to `15/102`.
  - Completed dedicated crit-stat key migration pass by normalizing remaining legacy `stats.criticalStrikeChance` keys to loader-canonical `stats.critChance` (legacy crit-key usage now `0/322`).
  - Completed item execution-semantics and citation wave 28 (`Kaenic Rookern`, `Lord Dominik's Regards`, `Moonstone Renewer`, `Mortal Reminder`, `Nashor's Tooth`) with manual behavior review and page-level verification.
  - Increased page-level item citation coverage from `155/243` to `160/243`.
  - Reduced runtime-filtered legal URF unmodeled no-page-citation queue from `15/102` to `10/102`.
  - Completed item execution-semantics and citation wave 29 (`Actualizer`, `Archangel's Staff`, `Bandlepipes`, `Serylda's Grudge`, `Staff of Flowing Water`, `Trailblazer`, `Umbral Glaive`, `Whispering Circlet`, `Winter's Approach`, `Yun Tal Wildarrows`) with manual behavior review and page-level verification.
  - Increased page-level item citation coverage from `160/243` to `170/243`.
  - Reduced runtime-filtered legal URF unmodeled no-page-citation queue from `10/102` to `0/102` (queue cleared).
  - Logged deferred runtime follow-up scope for newly-refined data semantics: mode-aware resource-cost branches, visibility-gated prep windows, movement-stack trail logic, owner attack-type split durations/values, and on-attack cooldown-reduction timing.
  - Completed item execution-semantics and citation wave 30 (`Wordless Promise`, `Anathema's Chains`, `Seraph's Embrace`, `Stormrazor`, `Perplexity`, `Atma's Reckoning`) with manual behavior review and page-level verification.
  - Corrected `Anathema's Chains` active semantics by separating active cooldown (`90s`) from in-combat cast-lockout (`15s`) and documenting global-target/no-cast-time behavior.
  - Increased page-level item citation coverage from `170/243` to `176/243`.
  - Reduced broader structured no-page citation queue from `73` to `67` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Logged rotating-mode stat/cost package drift notes for `Perplexity` and `Atma's Reckoning` as policy follow-up.
  - Completed item execution-semantics and citation wave 31 (`Doran's Ring`, `Divine Sunderer`, `Goredrinker`, `Prowler's Claw`, `Gargoyle Stoneplate`, `Duskblade of Draktharr`) with manual behavior review and page-level verification.
  - Added missing active cooldown/timing metadata from page-level sources (`Divine Sunderer` Spellblade ICD start timing, `Goredrinker` 15s active cooldown + attack-windup cast semantics, `Prowler's Claw` 25s cooldown/500 range/0.15s cast time, `Gargoyle Stoneplate` 30s cooldown/no-cast-time behavior).
  - Added explicit mode-availability context notes for distributed Arena-scoped items and logged canonical-policy follow-up for source/map availability drift.
  - Increased page-level item citation coverage from `176/243` to `182/243`.
  - Reduced broader structured no-page citation queue from `67` to `61` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and economy-reconciliation wave 32 (`Dark Seal`, `Tiamat`, `Night Harvester`, `Radiant Virtue`, `Moonflair Spellblade`, `Everfrost` + distributed economy updates on wave-31 Arena-scoped items).
  - Added page-level League Wiki citations and manual execution-semantics notes for `Dark Seal`, `Tiamat`, `Night Harvester`, `Radiant Virtue`, and `Moonflair Spellblade`.
  - Added distributed-item economy fields (`shop.prices.total` / `shop.prices.sell`) for `Divine Sunderer`, `Goredrinker`, `Prowler's Claw`, `Gargoyle Stoneplate`, `Duskblade of Draktharr`, `Everfrost`, `Night Harvester`, `Radiant Virtue`, and `Moonflair Spellblade`; documented acquisition-scope and dataset-vs-page drift notes.
  - Increased page-level item citation coverage from `182/243` to `187/243`.
  - Reduced broader structured no-page citation queue from `61/243` to `56/243` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and citation wave 33 (`Bami's Cinder`, `Bramble Vest`, `Catalyst of Aeons`, `Hexdrinker`, `Hextech Alternator`) with manual behavior review and page-level verification.
  - Added explicit trigger-exclusion and interaction-edge semantics where source notes provided them (for example dodged/blocked/blinded miss exclusions, spell-shield handling, proc-damage class, zero-damage trigger behavior, below-threshold retrigger notes).
  - Increased page-level item citation coverage from `187/243` to `192/243`.
  - Reduced broader structured no-page citation queue from `56/243` to `51/243` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and citation wave 34 (`Lost Chapter`, `Haunting Guise`, `Executioner's Calling`, `Seeker's Armguard`, `Warden's Mail`) with manual behavior review and page-level verification.
  - Added edge-semantics clarifications for shield-interaction, stasis single-use transform, and basic-damage reduction source/cap behavior where page notes provided additional nuance.
  - Increased page-level item citation coverage from `192/243` to `197/243`.
  - Reduced broader structured no-page citation queue from `51/243` to `46/243` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and citation wave 35 (`Oblivion Orb`, `Phage`, `Sheen`, `Quicksilver Sash`, `Verdant Barrier`) with manual behavior review and page-level verification.
  - Expanded execution-edge semantics for Spellblade trigger exclusions, Quicksilver activation constraints/cleanse scope, and Annul shield lifecycle notes (including death/cooldown restart interactions).
  - Increased page-level item citation coverage from `197/243` to `202/243`.
  - Reduced broader structured no-page citation queue from `46/243` to `41/243` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and citation wave 36 (`Boots of Swiftness`, `Ionian Boots of Lucidity`, `Plated Steelcaps`, `Recurve Bow`, `Cull`) with manual behavior review and page-level verification.
  - Added full Reap branch coverage for `Cull` (on-hit sustain, minion-gold progression cap, and completion payout/disable semantics) after gameplay-intent review.
  - Added `Overcharged` `mode_overrides` overlays for Swiftplay and URF Sudden Death timing/value divergence while preserving Clash baseline semantics at root.
  - Aligned Quicksilver-family edge semantics on `Mercurial Scimitar` with `Quicksilver Sash` (airborne lockout, suppression/nearsight scope, no-cast-time/stealth behavior, cooldown-transfer note).
  - Reconciled `Wordless Promise` active-cooldown fidelity from tooltip-ambiguous baseline to page-verified `10s` Promise cooldown with team/target gating notes.
  - Increased page-level item citation coverage from `202/243` to `207/243`.
  - Reduced broader structured no-page citation queue from `41/243` to `36/243` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and citation wave 37 (`Armored Advance`, `Black Spear`, `Ghostcrawlers`, `Guardian's Amulet`, `Guardian's Horn`, `Guardian's Orb`, `Scout's Slingshot`, `Swiftmarch`, `Zephyr`) with manual behavior review and page-level verification.
  - Added mode-variant Arena overlays (`mode_overrides.ARENA`) for `Guardian's Horn` and `Guardian's Orb`; documented Zephyr/Gunmetal-Greaves shared-ID drift and Black Spear bind-window/runtime follow-up scope.
  - Increased page-level item citation coverage from `207/243` to `216/243`.
  - Reduced broader structured no-page citation queue from `36/243` to `27/243` and cleared higher-priority no-page subset (`9` -> `0`) while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and citation wave 38 (`Arcane Sweeper (Trinket)`, `Farsight Alteration`, `Oracle Lens`, `Stealth Ward`, `Slightly Magical Boots`, `Crown of the Shattered Queen`, `Turbo Chemtank`) with manual behavior review and page-level verification.
  - Added trinket execution-semantics detail for activation lockouts, charge/recharge scaling, ward limits, detection ranges, and player-visible reveal windows on wave-38 vision utility items.
  - Expanded distributed-item explicit economy representation from `10/57` to `13/57` by adding `shop.prices` on `Crown of the Shattered Queen`, `Turbo Chemtank`, and `Slightly Magical Boots` with acquisition-scope reconciliation notes.
  - Increased page-level item citation coverage from `216/243` to `223/243`.
  - Reduced broader structured no-page citation queue from `27/243` to `20/243` while keeping legal URF unmodeled no-page queue at `0/102`.
  - Completed data-first item execution-semantics and citation wave 39 (`Anti-Tower Socks`, `Base Turret Reinforced Armor (Turret Item)`, `Black Hole Gauntlet`, `Cruelty`, `Darksteel Talons`, `Death's Daughter`, `Decapitator`, `Empyrean Promise`, `Mirage Blade`, `Ohmwrecker (Turret Item)`, `Phreakish Gusto`, `Raise Morale`, `Reinforced Armor (Turret Item)`, `Shield of Molten Stone`, `Super Mech Armor`, `Super Mech Power Field`, `Twilight's Edge`, `Twin Mask`, `Warden's Eye`, `Wooglet's Witchcap`) with manual behavior review and page-level verification.
  - Cleared broader structured no-page citation queue from `20/243` to `0/243` by adding page-level sources for all remaining queued items and dual-citation fallback on redirect-backed pseudo-items.
  - Expanded distributed-item explicit economy representation from `13/57` to `23/57` by adding `shop.prices` on ten distributed Arena identities.
  - Corrected page-verified structured-value mismatches on `Darksteel Talons` (melee armor ratio), `Twin Mask` (15%/30% teammate transfer), `Twilight's Edge` (20% AD/AP world modifiers), `Empyrean Promise` (20s active cooldown + dash speed), `Warden's Eye` (trap reveal + line-of-sight note), `Wooglet's Witchcap` (120s stasis cooldown), and `Black Hole Gauntlet` (max-health scaling branch).
  - Completed distributed/prismatic economy rollout wave 40 across the remaining `34` distributed files by backfilling explicit `shop.prices` from Tier-1 item-ID reconciliation (`23/57` -> `57/57`).
  - Backfilled source provenance for previously unsourced distributed utility/legacy entries (`Lucky Dice`, `Enhanced Lucky Dice`, `Poro-Snax`, `Total Biscuit of Everlasting Will`, `Your Cut`) and normalized sparse legacy active-shape inconsistencies (`active: {}` -> `active: []` where applicable).
  - Reduced non-structured item provenance backlog from `79` to `74` files with null/empty `sources`.
  - Completed non-structured provenance completion wave 41 by backfilling sources for the remaining `72` canonical non-structured unsourced item files, clearing canonical non-structured provenance to `0/77` unsourced (the `2` non-item report artifacts under `Items/` are now sourced and tracked separately from canonical item coverage counts).
  - Logged a data-reconciliation follow-up on `Gambler's Blade`: structured range currently uses `30` to `240` per Tier-1/local tooltip data, while wiki patch history references a historical `245` cap increase (since resolved as historical-context-only tracking).
  - Canonical non-structured provenance backlog is now cleared (`0/77` unsourced); keep this as a no-regression guardrail.
- Remaining risks and improvements:
- Tracked broader page-level formula citation queue is cleared (`243/243`); keep this as a no-regression guardrail on future complex/non-trivial edits.
  - Item low-confidence backlog is cleared (`0` files with `parse_confidence < 0.65`); rune low-confidence backlog remains `0` at the `<= 0.60` threshold.
  - Low-confidence page-verification queue is cleared (`0/0` low-confidence item files without page-level citations).
  - Legal URF unmodeled page-level citation depth is now cleared (`0/102` without page-level citations); maintain this as a no-regression guardrail.
- Broader structured no-page citation queue is now cleared (`0/243`); maintain queue-clear state as a no-regression requirement.
  - Legal URF unmodeled low-confidence citation-depth gap remains cleared (`0/0`).
  - Source endpoint correctness and sell-value reconciliation are now no-regression requirements; periodic audits are still required with explicit handling for the `3` intentional support-quest sell overrides.
  - Structured-item `sources[].accessed` metadata is now fully normalized; keep it guarded as a no-regression requirement.
  - Keep item stat-key canonicalization guarded (`stats.magicResist`, `stats.critChance`); legacy key usage (`magicResistance`, `criticalStrikeChance`) should remain at `0` files.
  - Resolve the tracked cross-version ID/name drift exception (`Zephyr` vs current Tier-1 `Gunmetal Greaves` on ID `3172`) with explicit legacy-ID policy.
  - Resolve canonical lifecycle/availability policy for rotating-mode-only item identities (`Wordless Promise`, `Perplexity`, `Atma's Reckoning`) so simulation pools stay intentionally scoped.
  - Standardize lifecycle-marker coverage policy for any additional retired/replaced item identities (currently explicit on `Zephyr` only).
  - Maintain Dragonheart acquisition-round table no-regression (`3-4 => 1`, `5-6 => 2`, `7-8 => 3`, `9+ => 4`) and re-verify if the page-published threshold table changes.
  - Resolve Lifeline special-case snapback interactions (displacement/channel/crowd-control edge cases) if simulation accuracy should include these interaction branches.
  - Continue Golden Spatula fidelity follow-up on mode-scoped stat/economy drift and non-modeled runtime handling assumptions (confidence floor work is complete).
  - Muramana Shock proc-damage exclusion is now documented in data, but runtime proc-damage classification for champion-ability branches is still a deferred code-layer follow-up.
  - Maintain intended-behavior-first policy for known bug notes; do not implement bug behavior in canonical data.
  - Run a broader targeted-active audit to confirm cooldown and cast-range metadata are consistently encoded when page/tooltips publish those values.
  - Maintain `Gambler's Blade` canonical-240 plus historical-245 reconciliation policy as a no-regression guardrail on future edits.
  - Define canonical policy for champion-ability upgrade pseudo-items so upgrade entries are represented consistently across data and coverage tracking.
  - Maintain distributed/prismatic economy no-regression (`57/57` explicit) while finalizing canonical policy for non-shop acquisition vs Tier-1 gold fields, including unresolved page-vs-dataset economy drift (`Cost 0 / Sell 2000` page display vs Tier-1 `1000 / 400` values on multiple Arena distributed identities).
  - Keep redirect-backed pseudo-item provenance policy enforced (redirect item URL + canonical parent gameplay/champion page citation) so minion/turret/champion-upgrade semantics remain auditable.
  - Define canonical distributed-item availability/map-overlay policy when Tier-1 map flags and page-level mode scope diverge (for example Arena-scoped distributed identities).
  - Extend canonical mode-scoping policy beyond `Overcharged` and add runtime mode-overlay consumption so `mode_overrides` branches are selected automatically per active mode (`Guardian's Horn` / `Guardian's Orb` Arena overlays included).
  - Non-structured item provenance baseline is now complete for canonical items (`0/77` unsourced); keep this as a no-regression guardrail and maintain folder-level provenance/accessed completeness across the two non-item `Items/` report artifacts.
  - Support-income diminishing-gold formulas are now explicit across the support-quest family, but shared runtime-table encoding is still pending for exact per-minute behavior.
- Remaining work should be tracked from `Simulation/COVERAGE_GAPS.md` Data Quality Gap Snapshot counts.

## Quick Audit Commands (Data Coverage Hygiene)
Use these from repository root.

Items with structured effects and null sources:
```bash
for f in Items/*.json; do
  if [ "$(jq '(.effects_structured // []) | length' "$f")" -gt 0 ] && [ "$(jq '(.sources == null)' "$f")" = "true" ]; then
    echo "$(basename "$f")"
  fi
done
```

Items with structured effects and no Tier-2 citation (`CommunityDragon` or League Wiki):
```bash
for f in Items/*.json; do
  if [ "$(jq '(.effects_structured // []) | length' "$f")" -eq 0 ]; then
    continue
  fi
  has_tier2=$(jq -r '
    [(.sources // [])[]?
      | select(((.url // "") | test("raw\\.communitydragon\\.org")) or ((.url // "") | test("wiki\\.leagueoflegends\\.com")))
    ] | length
  ' "$f")
  if [ "$has_tier2" -eq 0 ]; then
    echo "$(basename "$f")"
  fi
done
```

Items with structured effects and no page-level citation (League Wiki item pages):
```bash
for f in Items/*.json; do
  if [ "$(jq '(.effects_structured // []) | length' "$f")" -eq 0 ]; then
    continue
  fi
  has_page_level=$(jq -r '
    [(.sources // [])[]?
      | select((.url // "") | test("wiki\\.leagueoflegends\\.com"))
    ] | length
  ' "$f")
  if [ "$has_page_level" -eq 0 ]; then
    echo "$(basename "$f")"
  fi
done
```

Legal URF unmodeled low-confidence items with no page-level citation:
```bash
python3 - <<'PY'
import json,glob,os,re
root='.'
text=open('Simulation/COVERAGE_GAPS.md').read().splitlines()
start=text.index('### Unmodeled Runtime Item Effects In Legal URF Pool (`102`)')+1
names=[]
for line in text[start:]:
    if line.startswith('### ') or line.startswith('## '):
        break
    m=re.match(r'- `(.+)`', line)
    if m:
        names.append(m.group(1))
def norm(s):
    return ''.join(ch.lower() for ch in s if ch.isalnum())
files={norm(os.path.splitext(os.path.basename(p))[0]):p for p in glob.glob('Items/*.json')}
for name in names:
    path=files.get(norm(name))
    if not path:
        continue
    item=json.load(open(path))
    effects=item.get('effects_structured') or []
    if not any(isinstance(e.get('parse_confidence'), (int,float)) and e['parse_confidence'] < 0.65 for e in effects):
        continue
    has_wiki=any('wiki.leagueoflegends.com' in ((s.get('url') or '')) for s in (item.get('sources') or []))
    if has_wiki:
        continue
    minimum=min((e.get('parse_confidence') for e in effects if isinstance(e.get('parse_confidence'), (int,float))), default=None)
    print(f\"{item.get('name')}\tmin_parse_confidence={minimum}\")
PY
```

Items with structured effects and missing/null `parse_confidence` entries:
```bash
for f in Items/*.json; do
  if [ "$(jq '(.effects_structured // []) | length' "$f")" -eq 0 ]; then
    continue
  fi
  missing=$(jq '[.effects_structured[]? | select((has("parse_confidence") | not) or .parse_confidence == null)] | length' "$f")
  if [ "$missing" -gt 0 ]; then
    echo "$(basename "$f") : missing_or_null_parse_confidence_entries=$missing"
  fi
done
```

Items with non-ISO or missing review metadata:
```bash
for f in Items/*.json; do
  if [ "$(jq '(.effects_structured // []) | length' "$f")" -eq 0 ]; then
    continue
  fi
  reviewed=$(jq -r '.schema_notes.effects_structured_reviewed // "<missing>"' "$f")
  if [ "$reviewed" = "<missing>" ] || ! printf '%s' "$reviewed" | rg -q '^[0-9]{4}-[0-9]{2}-[0-9]{2}$'; then
    echo "$(basename "$f") : $reviewed"
  fi
done
```

Structured item files with missing `sources[].accessed` metadata:
```bash
for f in Items/*.json; do
  if [ "$(jq '(.effects_structured // []) | length' "$f")" -eq 0 ]; then
    continue
  fi
  missing_accessed=$(jq -r '
    [(.sources // [])[]?
      | select(((.url != null) or (.path != null)) and ((.accessed // "") == ""))
    ] | length
  ' "$f")
  if [ "$missing_accessed" -gt 0 ]; then
    echo "$(basename "$f") : missing_accessed_entries=$missing_accessed"
  fi
done
```

Item stat-key canonicalization audit (`magicResist` vs legacy `magicResistance`):
```bash
for f in Items/*.json; do
  legacy=$(jq '((.stats // {}) | has("magicResistance"))' "$f")
  if [ "$legacy" = "true" ]; then
    echo "$(basename "$f")"
  fi
done
```

Cross-version ID/name drift audit (file name vs current Data Dragon item identity):
```bash
python3 - <<'PY'
import glob, json, os, urllib.request
with urllib.request.urlopen('https://ddragon.leagueoflegends.com/cdn/16.3.1/data/en_US/item.json', timeout=30) as r:
    dd=json.load(r)['data']
for path in glob.glob('Items/*.json'):
    item=json.load(open(path))
    iid=str(item.get('id'))
    if iid not in dd:
        continue
    local_name=(item.get('name') or '').strip()
    dd_name=(dd[iid].get('name') or '').strip()
    if local_name and dd_name and local_name != dd_name:
        print(f"{os.path.basename(path)}\tid={iid}\tlocal={local_name}\tddragon={dd_name}")
PY
```

Runtime-modeled item condition-token inventory (compatibility audit):
```bash
for f in \
  "Items/Blade of the Ruined King.json" \
  "Items/Guardian Angel.json" \
  "Items/Guinsoos Rageblade.json" \
  "Items/Heartsteel.json" \
  "Items/Kraken Slayer.json" \
  "Items/Liandrys Torment.json" \
  "Items/Ludens Echo.json" \
  "Items/Protoplasm Harness.json" \
  "Items/Zhonyas Hourglass.json"; do
  jq -r '.effects_structured[]? | (.conditions // [])[]?' "$f"
done | sort -u
```

Modeled runtime item confidence-floor audit:
```bash
for f in \
  "Items/Blade of the Ruined King.json" \
  "Items/Guardian Angel.json" \
  "Items/Guinsoos Rageblade.json" \
  "Items/Heartsteel.json" \
  "Items/Kraken Slayer.json" \
  "Items/Liandrys Torment.json" \
  "Items/Ludens Echo.json" \
  "Items/Protoplasm Harness.json" \
  "Items/Zhonyas Hourglass.json"; do
  min=$(jq '[.effects_structured[]? | .parse_confidence? | numbers] | min // null' "$f")
  printf "%s\tmin_parse_confidence=%s\n" "$(basename "$f")" "$min"
done
```

Runes with null `stat` in `stat_modifier` entries (flat compatibility file):
```bash
jq -r '
  .paths[].slots[].runes[]
  | select([(.effects_structured // [])[] | select(.effect_type=="stat_modifier" and (.stat==null or .stat==""))] | length > 0)
  | .name
' Masteries/RunesReforged.json | sort -u
```

Runes with null `stat` in split primary-tree files:
```bash
jq -r '
  .slots[].runes[]
  | select([(.effects_structured // [])[] | select(.effect_type=="stat_modifier" and (.stat==null or .stat==""))] | length > 0)
  | .name
' Masteries/RunesReforged/Trees/*/primary_runes.json | sort -u
```

## How We Got The Complete Examples There
### Step 1: Start from source data
- Champion/item raw source is kept under `From Online/`.
- Item normalization pipeline:
  - `From Online/run_item_pipeline.py`
  - `From Online/normalize_items.py`
  - `From Online/make_sample_set.py`

### Step 2: Canonicalize into domain-owned files
- Champion gameplay values go in `Characters/<Champion>.json` (`base_stats`, `abilities`, `basic_attack`).
- Item gameplay values go in `Items/<Item>.json` with structured effects (`effects_structured`) and stable effect IDs.
- Runes/shards stay synchronized across:
  - `Masteries/RunesReforged.json` (flat compatibility file)
  - `Masteries/RunesReforged/RunesReforged.json` (split index)
  - `Masteries/RunesReforged/Trees/*/primary_runes.json`
  - `Masteries/RunesReforged/Trees/*/secondary_runes.json`
  - `Masteries/RunesReforged/StatShards/stat_shards.json`

### Step 3: Load defaults from canonical data, not hardcoded literals
- Add or reuse loader paths in `Simulation/src/defaults.rs`.

### Step 4: Implement runtime behavior in scripts/runtime modules
- Champion-specific behavior in `Simulation/src/scripts/champions/<champion>/`.
- Shared loadout/item/rune behavior in `Simulation/src/scripts/runtime/` and `Simulation/src/scripts/items/`.

### Step 5: Register modeled coverage explicitly
- Add modeled asset keys in `Simulation/src/scripts/coverage.rs` and `Simulation/src/scripts/runes/effects.rs` as needed.

### Step 6: Add tests for behavior and regression
- Add unit tests near module `tests/` folders.

### Step 7: Update tracking docs
- Update `Simulation/COVERAGE_GAPS.md`.
- Update `Simulation/IMPROVEMENT_TRACKER.md`.
- Update `Simulation/champion_data_coverage_inventory.json` whenever champion corpus parity counts change.
- Update `Simulation/champion_behavior_verification_tracker.json` whenever manual behavior-verification scope changes.
- Update docs if architecture/ownership changed.

## Champion Authoring Playbook
### Step 0: Confirm champion corpus parity target
- Use `Simulation/champion_data_coverage_inventory.json` as a no-regression denominator check (`From Online/champions/*.json` vs `Characters/*.json`).
- When parity is already complete, select the next fidelity wave (low-confidence mechanics, shallow context notes, or behavior-ambiguity champions) instead of file-creation waves.
- Record the selected fidelity wave and its reviewed champion/ability scope in `Simulation/champion_behavior_verification_tracker.json` before marking coverage progress complete.

### Step 1: Author canonical champion file
- Required base shape:
  - `name`, `data_version`, `base_stats`, `basic_attack`, `abilities`.
- Put gameplay numbers in `abilities` and ability `effects`.
- Put ability timing/geometry in `abilities.<ability_key>.execution`.
- Keep `simulation` policy minimal and only for simulator-specific knobs that do not belong in canonical gameplay fields.

### Step 2: Keep AI controller policy separate
- Put movement/cadence/priority policy in `Simulation/data/champion_ai_profiles.json`.

### Step 3: Wire script behavior
- Controlled champion capability: `Simulation/src/scripts/champions/controlled_champion.rs`.
- Champion-specific behavior module: `Simulation/src/scripts/champions/<champion>/`.

### Step 4: Add tests
- Champion script tests in `Simulation/src/scripts/champions/tests/` or `Simulation/src/scripts/champions/<champion>/tests/`.

## Item Authoring Playbook
### Step 1: Start from source item JSON
- Use `From Online/items/` (and normalized outputs when useful).

### Step 2: Ensure canonical item data is complete
- accurate `stats`
- passive/active raw text
- `effects_structured` entries with stable `id` and useful trigger/cooldown/duration/scaling fields
- `schema_notes.effects_structured_reviewed` date update

### Step 3: Implement runtime behavior when effect impacts combat
- For stat assumptions and notes: `Simulation/src/scripts/items/hooks.rs`.
- For combat-time behavior: `Simulation/src/scripts/runtime/loadout_runtime.rs` and related runtime hooks.

### Step 4: Register modeled status
- Add item key in `Simulation/src/scripts/coverage.rs` when runtime modeled.

## Rune And Shard Authoring Playbook
### Step 1: Use canonical rune/shard source data
- Authoring source of truth:
  - split index: `Masteries/RunesReforged/RunesReforged.json`
  - per-tree files: `Masteries/RunesReforged/Trees/*/primary_runes.json` and `Masteries/RunesReforged/Trees/*/secondary_runes.json`
  - stat shards: `Masteries/RunesReforged/StatShards/stat_shards.json`
- Runtime compatibility source (must stay synchronized until loader migration): `Masteries/RunesReforged.json`.

### Step 2: Wire deterministic stat effects
- Ensure effect shape is parseable by `Simulation/src/data.rs` (`apply_structured_effect`, `apply_stat_bonus`).
- Add support for new stat keys in `apply_stat_bonus` when needed.

### Step 3: Wire dynamic combat-time rune effects
- Add rune key to `Simulation/src/scripts/runes/effects.rs`.
- Implement runtime behavior in `Simulation/src/scripts/runtime/loadout_runtime.rs`.
- Add tuning defaults in `Simulation/data/simulator_defaults.json` and typed loader fields in `Simulation/src/defaults.rs`.

### Step 4: Legacy mastery note
- Legacy `Season2016` masteries are intentionally retired from runtime support.
- Runes Reforged split files are data-authoring baseline; runtime still reads flat compatibility file until deferred code migration.

## Data Authoring Rules That Must Hold
- Do not hardcode champion/item/rune behavior in shared core modules:
  - `Simulation/src/engine.rs`
  - `Simulation/src/core.rs`
  - `Simulation/src/search.rs`
  - `Simulation/src/reporting.rs`
- Use explicit domain naming (for example `Vladimir`, not `Vlad`).
- Avoid direct raw metric mutation when generic runtime apply/resolve paths exist.
- Keep champion ability `description_source` populated on touched abilities; backfill missing values from source-corpus/authoritative ability text during the same edit.
- If mechanics are uncertain, record ambiguity in `Simulation/CONFIDENCE_REVIEW.md`.

## Definition Of Done
Before calling data work complete, run the checklist in:
- `Simulation/COVERAGE_CHECKLIST.md`

and required validation:
- `cargo fmt --manifest-path Simulation/Cargo.toml`
- `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
- `cargo test --release --manifest-path Simulation/Cargo.toml`

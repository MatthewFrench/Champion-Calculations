# Coverage Standards (Exemplar-Derived)

This document defines the quality bar for adding or expanding coverage in:
- champions
- abilities
- items
- rune and mastery systems (Runes Reforged + stat shards)

Use this together with:
- `Simulation/COVERAGE_CHECKLIST.md` (completion gate)
- `Simulation/COVERAGE_GAPS.md` (tracked incomplete coverage)
- `Simulation/DATA_AUTHORING_GUIDE.md` (authoring workflow)

## Active Priority Mode (Data-First)
Current working priority is data coverage quality before new runtime code expansion.

Data-first means:
- improve canonical data correctness and provenance first
- improve data consistency and review metadata first
- document deferred runtime/code expansion work clearly for later execution

Runtime expansion items are still tracked in `Simulation/COVERAGE_GAPS.md`, but they are not the primary focus of this phase.

## Exemplar Reference Set (Current Best-In-Repo)
These examples are the baseline quality bar for new coverage.

| Category | Best Current Example(s) | Why This Is The Bar |
| --- | --- | --- |
| Champion data + controlled runtime | `Characters/Vladimir.json`, `Simulation/src/scripts/champions/vladimir/`, `Simulation/src/scripts/champions/controlled_champion.rs`, `Simulation/src/scripts/champions/vladimir/tests/vladimir_tests.rs` | Canonical champion data drives script defaults, slot bindings, runtime decisions, and regression tests. |
| Ability event coverage (enemy scripted events) | `Simulation/src/scripts/champions/morgana/mod.rs`, `Simulation/src/scripts/champions/tests/champions_tests.rs` | Includes range gating, damage formulas, and a scheduled followup event (`Soul Shackles Detonate`) with tested behavior. |
| Item runtime coverage | Data: `Items/Zhonyas Hourglass.json`, `Items/Guardian Angel.json`, `Items/Protoplasm Harness.json`, `Items/Heartsteel.json`, `Items/Ludens Echo.json`; Code: `Simulation/src/defaults.rs`, `Simulation/src/scripts/items/hooks.rs`, `Simulation/src/scripts/runtime/loadout_runtime.rs`; Tests: `Simulation/src/scripts/runtime/tests/loadout_runtime_tests.rs`, `Simulation/src/scripts/runtime/tests/controlled_champion_loadout_tests.rs` | Shows end-to-end flow from structured item effects -> typed loader defaults -> runtime behavior -> regression tests. |
| Rune and shard runtime coverage | Data: `Masteries/RunesReforged.json`, `Simulation/data/simulator_defaults.json`; Code: `Simulation/src/scripts/runes/effects.rs`, `Simulation/src/scripts/runtime/loadout_runtime.rs`, `Simulation/src/data.rs`, `Simulation/src/defaults.rs`; Tests: `Simulation/src/scripts/runtime/tests/loadout_runtime_tests.rs`, `Simulation/src/scripts/runtime/tests/controlled_champion_loadout_tests.rs` | Dynamic rune behavior, level-scaled formulas, telemetry, deterministic parsing, and coverage-key guard tests are all represented. |

## Source Hierarchy And Research Standard
When adding or changing data coverage, use this source order:
1. Riot authoritative data and notes (Data Dragon, official patch notes, official mode notes).
2. CommunityDragon or equivalent canonical static data mirrors.
3. League of Legends Wiki data templates/pages for formula disambiguation.
4. Build/meta sites only for scenario presets or popularity context, never as canonical mechanics truth.

Research requirements for data updates:
- Record exact URLs in the owning JSON `sources` array.
- Include `accessed` date and `used_for` field per source.
- Keep `data_version` aligned to the patch version being modeled.
- If mechanics remain ambiguous, add a note in `Simulation/CONFIDENCE_REVIEW.md`.
- Web research is acceptable and encouraged to fully understand data behavior, fill information gaps, and verify uncertain mechanics before finalizing structured data.
- For non-trivial or low-confidence structured effects, include at least one Tier-2 citation source (CommunityDragon or League Wiki) in the item `sources` list when available.
- For low-confidence formula interpretation, do not rely only on dataset-level sources; add at least one page-level citation (for example item page or patch notes) in `sources`.
- Validate that source URLs used in `sources` resolve at authoring time (prefer HTTP 200) so provenance remains auditable.
- For CommunityDragon item-dataset citations, use the current endpoint `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/items.json`.
- Perform an entity-intent review before raising confidence: confirm the entity's gameplay role, intended combat pattern, and whether the structured effects reflect that behavior.
- Perform an in-game execution-semantics review for non-trivial effects before raising confidence: confirm activation requirements, target/range gating, and timing behavior.
- Execution-semantics review must cover player-visible behavior where relevant: when the effect starts, when it resolves, and what opponents/allies can observe.
- If behavior mixes spell casting with basic-attack cadence (for example reset or empowered-hit patterns), document both cast and hit timing semantics explicitly.
- If an effect has separate basic-attack-hit and ability-damage branches, encode each branch as separate structured effects with explicit trigger and gating semantics.
- If source notes include trigger exclusions or interaction edge rules (for example dodged/blocked/blinded misses, spell-shield handling, proc-damage classification, zero-damage proc eligibility, below-threshold retrigger clauses), encode them explicitly in structured `conditions`/`modifiers` and summarize the execution impact in `schema_notes.context_notes`.
- If sources present both per-tick and total values for one timed effect, ensure structured values are cadence-consistent (per-tick value matches interval) and capture total-over-duration for auditability.
- If sources describe known in-game bugs, capture them as notes for follow-up, but model intended behavior by default unless an explicit bug-emulation policy is approved.

Data metadata requirements:
- `schema_notes.effects_structured_reviewed` should use ISO date format `YYYY-MM-DD`.
- If a legacy free-text review note is replaced with an ISO review date, preserve the note in `schema_notes.context_notes`.
- If any effect keeps `parse_confidence < 0.65`, add a concrete follow-up note in `Simulation/COVERAGE_GAPS.md`.
- Runtime-modeled item effects should use numeric `parse_confidence` values (avoid null/missing confidence on modeled effect entries).
- Do not leave runtime-modeled item files with `sources: null`.
- When editing an item file, backfill missing `sources[].accessed` values on existing source entries in that file so provenance metadata stays complete.
- When editing existing non-trivial champion/item/rune data, explicitly sanity-check the entity's gameplay purpose and behavior pattern, then record that intent in `schema_notes.context_notes` when confidence or semantics changed.
- When semantics are updated, record the verified in-game execution model in `schema_notes.context_notes` (what the player does, when the effect resolves, and who is affected).
- When data semantics exceed current runtime capability (for example visibility-state windows, on-attack trigger classes, charge-state transforms, mode-gated resource branches), document the deferred code follow-up explicitly in `Simulation/COVERAGE_GAPS.md` in the same change.
- For control-triggered effects, encode the full trigger set from source text (for example include both immobilize and ground triggers when both are listed).
- If an item effect depends on a shared cross-item system rule (for example support-income diminishing-gold logic), preserve conservative confidence until that shared rule is encoded explicitly and track the dependency in `Simulation/COVERAGE_GAPS.md`.
- If editing a shared-rule effect on one member of an item family (for example the support-quest upgrade line), review sibling items for rule-schema consistency and track deferred sibling harmonization in `Simulation/COVERAGE_GAPS.md`.
- If an effect's timing or values differ by mode (for example Clash vs Swiftplay sudden-death behavior), encode the mode scope explicitly in `conditions`/`context_notes` and track unresolved canonical mode-policy decisions in `Simulation/COVERAGE_GAPS.md`.
- For mode-variant item behavior, keep Tier-1 baseline semantics at the root item object and encode only divergent mode semantics in `mode_overrides.<mode_key>` (for example `mode_overrides.URF` or `mode_overrides.ARENA`) using sparse override fields.
- For distributed/prismatic or other mode-exclusive items, record acquisition/availability scope explicitly in `schema_notes.context_notes` (for example Arena-only anvil acquisition) and track dataset-vs-page availability drift in `Simulation/COVERAGE_GAPS.md`.
- For distributed/prismatic mode-scoped items where Tier-1 datasets expose economy fields, keep explicit `shop.prices.total` / `shop.prices.sell` for reconciliation and document that acquisition is non-shop (for example Prismatic Item Anvils) in `schema_notes.context_notes`.
- For round/phase progression effects, verify whether acquisition timing backfills prior progression states; if not encoded, document the gap explicitly in `schema_notes.context_notes` and track it in `Simulation/COVERAGE_GAPS.md`.
- If Tier-1 dataset fields and page-level gameplay behavior disagree (for example sell-state restrictions), document the discrepancy in `schema_notes.context_notes` and track canonical-resolution follow-up in `Simulation/COVERAGE_GAPS.md`.
- When item economy fields are edited (for example `shop.prices.sell`), validate against Tier-1 dataset values and document any intentional override policy in coverage docs.
- Intentional economy overrides (for example page-verified No Sell behavior where Tier-1 datasets still provide numeric sell values) are acceptable only when the owning item file and `Simulation/COVERAGE_GAPS.md` both explicitly record the exception.
- Item stat blocks must use loader-canonical key vocabulary (for example `magicResist` and `critChance` under `stats`); legacy aliases that bypass loader mapping (for example `magicResistance`, `criticalStrikeChance`) are not acceptable.
- When Tier-1 reconciliation uses item IDs, verify ID-to-name alignment against current datasets and track any legacy-ID/name drift exception in `Simulation/COVERAGE_GAPS.md`.
- If an item is retained only as legacy/reference data (retired or replaced in current Tier-1 datasets), add explicit `lifecycle` metadata with `exclude_from_simulation = true` and a concrete replacement/reason note.
- Lifecycle metadata on legacy/reference items should include at least `status`, `exclude_from_simulation`, `reason`, `replacement_item`, and `replacement_id` so ID/name drift remains auditable.
- When a known bug diverges from intended gameplay behavior, keep intended behavior as the canonical simulation target and track bug-emulation requests as deferred runtime follow-up work.

## Standards By Coverage Category
### 1) Champion Coverage Standard (Data + Runtime)
Data requirements:
- Champion file includes canonical `base_stats`, `basic_attack`, and `abilities`.
- Ability identities are stable and slot bindings are data-owned (no slot hardcoding in engine paths).
- Ability geometry and cast data are in `abilities.<ability_key>.execution`.
- Any simulator-only policy remains minimal and under `simulation`.
- `sources` and `schema_notes` are present and current.

Code requirements:
- Champion behavior is implemented under `Simulation/src/scripts/champions/<champion>/`.
- Controlled champion wiring goes through `Simulation/src/scripts/champions/controlled_champion.rs`.
- Enemy scripted-event wiring goes through `Simulation/src/scripts/champions/mod.rs`.
- Shared engine/core/search/reporting modules remain champion-agnostic.

Test requirements:
- Decision tests for cast gating, priority, and cooldown scheduling.
- Damage or effect tests for at least one offensive and one defensive/survivability path where applicable.
- Slot-binding compatibility tests when slot identity participates in behavior.

Documentation requirements:
- Update `Simulation/COVERAGE_GAPS.md` counts and lists.
- Mark the champion status as data-complete, runtime-complete, or partial (explicitly).

### 2) Ability Coverage Standard (Scripted Ability/Event Behavior)
Data requirements:
- Ability formulas and cooldown/range are sourced from canonical champion data, not inline constants.
- If followup timing exists, keep the delay in canonical ability effect data when possible.
- Ability execution semantics are documented where relevant: cast type, target/range requirements, windup/cast time, projectile or hit timing, and basic-attack-cadence coupling.

Code requirements:
- Ability execution returns generic script actions (`ApplyDamage`, `ScheduleFollowup`, etc.).
- Range checks and projectile/hitbox execution come from data defaults/loaders.
- Cooldown lookup path resolves through defaults helpers, not hardcoded per-ability constants.

Test requirements:
- In-range and out-of-range behavior tests.
- Followup scheduling tests when multi-stage abilities exist.
- Formula breakpoint tests (for example level or scaling-term transitions) when applicable.
- Non-instant execution timing tests (windup/cast/hit timing and attack-cadence coupling) when the ability depends on those semantics.

Documentation requirements:
- Update scripted-ability coverage summary in `Simulation/COVERAGE_GAPS.md`.

### 3) Item Coverage Standard (Data + Runtime)
Data requirements:
- Item file has accurate `stats`, effect text fields, and `effects_structured`.
- Item `stats` keys are loader-compatible (canonical stat-key vocabulary used consistently across files).
- Every runtime-modeled effect has a stable `effects_structured[].id`.
- Structured effects include trigger/cooldown/duration/scaling metadata required by runtime loaders.
- Active cast effects should explicitly encode cooldown and cast-range metadata when those values are available in source text.
- Missing active cooldown/cast-range metadata (when source text provides those values) is a blocking data-quality failure.
- Non-trivial active or combat-triggered effects include execution-semantics notes for activation gating, timing, and player-visible resolution behavior.
- Distributed/prismatic mode-exclusive items explicitly document both economy representation (`shop.prices`) and acquisition scope (for example Arena anvil-only) when those differ from standard shop purchasing.
- Multi-phase active effects (for example mark + timeout branch + recast branch) encode each branch explicitly, including cooldown-start semantics.
- Single-use transform actives (for example one-time stasis that converts to a shattered variant) should encode both activation behavior and post-use state transition semantics in structured conditions/modifiers and context notes.
- For actives with both cooldown and temporary cast-lockout constraints, encode cooldown and lockout as separate fields/conditions (do not collapse lockout text into base cooldown).
- Multi-branch passives that mix attack-hit and ability-damage triggers should encode branch-specific triggers and shared limiter semantics explicitly.
- Triggered passives should encode explicit trigger exclusions and interaction edges from source notes (for example miss/parry/blind exclusions, spell-shield bypass, proc-damage class, zero-damage proc eligibility).
- Cleanse-style actives should encode activation constraints and cleanse scope nuances when source notes provide them (for example airborne activation lockout, suppression removal scope, and no-cast-time behavior).
- `schema_notes.effects_structured_reviewed` is updated.
- Modeled runtime items should include a populated `sources` array (do not leave `sources: null`).
- Bug-history notes should not silently override intended behavior semantics in structured data; intended behavior remains canonical unless explicitly documented otherwise.
- Legacy/reference-only items should include explicit lifecycle exclusion metadata so they do not enter active simulation pools accidentally.
- Review metadata should be normalized to `YYYY-MM-DD` for tracking consistency.
- Low-confidence parse entries (`parse_confidence < 0.65`) should carry follow-up tracking in `Simulation/COVERAGE_GAPS.md`.
- Runtime-modeled effect entries should keep numeric `parse_confidence` values so precision tracking remains auditable.
- Runtime-modeled items must keep parser-compatible condition token vocabulary (for example health-threshold trigger keys) unless runtime loader support is updated in the same change.

Code requirements:
- Runtime constants are loaded from item data through `Simulation/src/defaults.rs`.
- Combat or defensive behavior lives in `Simulation/src/scripts/items/` and/or `Simulation/src/scripts/runtime/`.
- Modeled runtime keys are registered in `Simulation/src/scripts/coverage.rs`.

Test requirements:
- Coverage-registry membership tests.
- Cooldown and scaling tests at runtime for modeled procs/actives.
- Decision tests for defensive activations and lifecycle effects (stasis/revive/shield) where modeled.

Documentation requirements:
- Update item modeled/unmodeled lists in `Simulation/COVERAGE_GAPS.md`.
- Keep compatibility aliases documented when keys differ from current item filenames.

### 4) Rune, Mastery, And Shard Coverage Standard
Data requirements:
- Canonical source remains `Masteries/RunesReforged.json`.
- Rune path slot ordering and shard slot options stay valid.
- Deterministic stat effects are encoded in parseable structured-effect forms.
- Avoid `effect_type = stat_modifier` entries with null/empty `stat` unless explicitly documented as narrative-only.
- Prefer `effect_type = condition_note` for narrative constraints that are not direct stat modifiers.
- Legacy `Season2016` masteries remain unsupported unless explicit project direction changes.

Code requirements:
- Dynamic combat-time rune keys are defined in `Simulation/src/scripts/runes/effects.rs`.
- Runtime behavior is implemented in `Simulation/src/scripts/runtime/loadout_runtime.rs`.
- Runtime tuning constants are owned by `Simulation/data/simulator_defaults.json` and loaded via `Simulation/src/defaults.rs`.
- Deterministic static rune and shard parsing stays in `Simulation/src/data.rs` (`apply_structured_effect`, `apply_stat_bonus`).

Test requirements:
- Observable runtime-effect assertions for each dynamic rune key.
- Formula breakpoint tests for level-scaled or cooldown-scaled rune behavior.
- Telemetry integrity tests for modeled proc runes.
- Deterministic parsing tests for static runes and shards that claim deterministic coverage.

Documentation requirements:
- Keep dynamic and deterministic modeled sets separated in `Simulation/COVERAGE_GAPS.md`.
- Explicitly document shard stats that parse but are not fully modeled at combat-time.

## Improvement Backlog For Existing Modeled Coverage
These are quality improvements to already-covered assets.

### Data Quality Improvements
- Maintain full provenance coverage for item files with `effects_structured` (all currently sourced) and keep this as a no-regression guardrail.
- Enforce `sources` de-duplication on champion/item files during future edits.
- Raise precision of low-confidence modeled item parses (`parse_confidence` around `0.55` to `0.65`) with manual formula normalization notes.
- Add/maintain a data audit for runtime-modeled item condition token compatibility to prevent loader/parser regressions.

### Champion And Ability Improvements
- Expand controlled champion script coverage from `Vladimir` to remaining champions with the same capability interface.
- Add enemy-script tests for every scripted event path (cooldown lookup, range gates, and followup priorities), not only representative cases.

### Item Runtime Improvements
- Add explicit regression tests for Heartsteel assumption paths in `Simulation/src/scripts/items/hooks.rs` (stack override mapping and note generation).
- Consolidate compatibility handling for `Luden's Echo` and alias keys into one documented alias source to reduce drift risk.

### Rune And Shard Improvements
- Reduce key-table duplication between dynamic rune registry and runtime telemetry tables by deriving from one canonical mapping.
- Add deterministic regression tests for currently modeled static runes (`Celerity`, `Jack Of All Trades`, `Legend: Alacrity`, `Legend: Haste`, `Magical Footwear`, `Nimbus Cloak`).
- Complete runtime tenacity application for shard-supported tenacity stats.

## Deferred Runtime Expansion Backlog (Documented For Later)
These are intentionally deferred while data-first work is active:
- Expand controlled champion script coverage beyond `Vladimir`.
- Add full enemy-script event-path tests (cooldown, range gating, followup behavior).
- Unify rune runtime key mapping and telemetry key mapping to a single canonical table.
- Complete runtime tenacity behavior application for shard-supported tenacity stats.
- Add explicit opt-in bug-emulation runtime pathways for known bug-divergence scenarios while keeping intended behavior as the default simulation target.

## Standard Adoption Rule
When adding new coverage:
1. Pick the matching exemplar from this file.
2. Meet or exceed that exemplar in data completeness, loader wiring, runtime behavior, and tests.
3. Update `Simulation/COVERAGE_GAPS.md` and `Simulation/COVERAGE_CHECKLIST.md` state in the same change.

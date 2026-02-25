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
| Rune and shard runtime coverage | Data: `Masteries/RunesReforged.json`, `Masteries/RunesReforged/RunesReforged.json`, `Simulation/data/simulator_defaults.json`; Code: `Simulation/src/scripts/runes/effects.rs`, `Simulation/src/scripts/runtime/loadout_runtime.rs`, `Simulation/src/data.rs`, `Simulation/src/defaults.rs`; Tests: `Simulation/src/scripts/runtime/tests/loadout_runtime_tests.rs`, `Simulation/src/scripts/runtime/tests/controlled_champion_loadout_tests.rs` | Dynamic rune behavior, level-scaled formulas, telemetry, deterministic parsing, and coverage-key guard tests are all represented. |

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
- When a League Wiki item page is a redirect (common for pseudo-item, turret-item, minion-item, or champion-upgrade identities), cite both the redirect URL and the canonical parent gameplay/champion page used to verify behavior semantics.
- Validate that source URLs used in `sources` resolve at authoring time (prefer HTTP 200) so provenance remains auditable.
- If League Wiki blocks scripted URL checks (for example HTTP 403 on automated HEAD/GET), validate reachability via manual browser open and keep normalized page URLs in `sources`.
- For CommunityDragon item-dataset citations, use the current endpoint `https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/items.json`.
- Perform an entity-intent review before raising confidence: confirm the entity's gameplay role, intended combat pattern, and whether the structured effects reflect that behavior.
- Perform an in-game execution-semantics review for non-trivial effects before raising confidence: confirm activation requirements, target/range gating, and timing behavior.
- Execution-semantics review must cover player-visible behavior where relevant: when the effect starts, when it resolves, and what opponents/allies can observe.
- For manual champion verification waves, include at least one page-level champion ability source in each touched champion file (`sources`) and track that citation in `Simulation/champion_behavior_verification_tracker.json`.
- If behavior mixes spell casting with basic-attack cadence (for example reset or empowered-hit patterns), document both cast and hit timing semantics explicitly.
- If an effect has separate basic-attack-hit and ability-damage branches, encode each branch as separate structured effects with explicit trigger and gating semantics.
- If source notes include trigger exclusions or interaction edge rules (for example dodged/blocked/blinded misses, spell-shield handling, proc-damage classification, zero-damage proc eligibility, below-threshold retrigger clauses), encode them explicitly in structured `conditions`/`modifiers` and summarize the execution impact in `schema_notes.context_notes`.
- If sources present both per-tick and total values for one timed effect, ensure structured values are cadence-consistent (per-tick value matches interval) and capture total-over-duration for auditability.
- If current canonical effect text and patch-history values diverge, model current canonical behavior by default and record historical values as context notes/modifiers only.
- If threshold tables are not explicitly published but can be derived from published cadence/cap constraints, mark the encoded table as inferred and track remaining uncertainty in `Simulation/CONFIDENCE_REVIEW.md`.
- If sources describe known in-game bugs, capture them as notes for follow-up, but keep intended behavior as canonical in data.
- Project policy lock: data coverage must default to intended non-bug behavior; bug behavior is documented only as follow-up notes and is not part of canonical simulation data.

Data metadata requirements:
- `schema_notes.effects_structured_reviewed` should use ISO date format `YYYY-MM-DD`.
- If a legacy free-text review note is replaced with an ISO review date, preserve the note in `schema_notes.context_notes`.
- If any effect keeps `parse_confidence < 0.65`, add a concrete follow-up note in `Simulation/COVERAGE_GAPS.md`.
- Touched structured item effects should keep numeric `parse_confidence` values even when not runtime-modeled (avoid null/missing confidence on edited `effects_structured` entries).
- Runtime-modeled item effects should use numeric `parse_confidence` values (avoid null/missing confidence on modeled effect entries).
- Do not leave runtime-modeled item files with `sources: null`.
- Do not leave canonical item files unsourced, including non-structured placeholder/stat-only items (`effects_structured` empty is allowed, `sources` missing is not).
- When editing an item file, backfill missing `sources[].accessed` values on existing source entries in that file so provenance metadata stays complete.
- When editing champion or mastery files, backfill missing `sources[].accessed` values on existing source entries in the touched file so provenance metadata stays complete across domains.
- When editing Runes Reforged data, keep flat and split datasets synchronized (`Masteries/RunesReforged.json` plus `Masteries/RunesReforged/` tree/stat-shard files) and treat split-vs-flat drift as blocking.
- When correcting generated decimal-spacing artifacts in rune `effects_structured.raw` text, also normalize dependent numeric metadata (`numbers_extracted`, and when affected `value_range` / `scaling` / `formula`) so structured values remain semantically consistent.
- When running scripted/bulk rune decimal normalization, run a post-edit scan for literal backreference placeholders (for example `\1.\2`) in `effects_structured.raw`; treat any hit as a blocking data defect.
- When a rune `formula.type = per_rank` encodes multiple branches (for example melee/ranged, AD/AP, stack-threshold branches), add `semantic_components` that map each branch to explicit named fields.
- If decimal/metadata normalization quality currently depends on manual scripts only, log deferred code follow-up for loader/lint enforcement in `Simulation/COVERAGE_GAPS.md` so future waves are automatically guarded.
- When editing existing non-trivial champion/item/rune data, explicitly sanity-check the entity's gameplay purpose and behavior pattern, then record that intent in `schema_notes.context_notes` when confidence or semantics changed.
- When semantics are updated, record the verified in-game execution model in `schema_notes.context_notes` (what the player does, when the effect resolves, and who is affected).
- Keep champion ability `description_source` populated on touched abilities; if missing, backfill from authoritative/source-corpus ability text in the same change.
- If a temporary fallback-derived `description_source` value is used to close completeness, track a provenance-hardening follow-up in `Simulation/COVERAGE_GAPS.md` to replace it with direct source-corpus text.
- Do not keep fragmentary/truncated context-note strings in touched data (for example `for 0.` or `within 4.`); normalize to complete timing/unit semantics before marking the entry reviewed.
- Use strict fragment detection for truncation audits (integer-dot tokens like `for 0.`), and do not classify valid decimals (for example `for 0.25 seconds`) as truncation defects.
- Run a secondary cadence-fragment sweep for phrases like `every 0.` so truncated periodic-tick notes are not missed by the strict primary pattern.
- Run a tertiary article-token sweep for phrases like `during the 0.` so article-interposed truncation fragments are also detected.
- Run a cast-time fragment sweep for phrases like `have a 0.` so truncated cast-windup timing notes are also detected.
- Truncation audits must normalize `context_notes` shape before matching (support both string and array forms on ability/effect notes) so queue counts are complete and reproducible.
- If truncation cleanup uses scripted/bulk edits, scope replacements to `context_notes` fields only and run a post-edit audit so canonical `description`/`description_source` text is unchanged unless intentionally edited.
- When a note contains an integer quantity that is intentional (for example stack count), avoid terminal integer-dot phrasing (for example `up to 3.`) and include units/entity labels (for example `up to 3 stacks`) so truncation audits remain reliable.
- When data semantics exceed current runtime capability (for example visibility-state windows, on-attack trigger classes, charge-state transforms, mode-gated resource branches), document the deferred code follow-up explicitly in `Simulation/COVERAGE_GAPS.md` in the same change.
- For movement abilities with formula-based velocity (for example `base_speed + movement_speed`), preserve the published formula in notes and track deferred runtime follow-up if execution schema currently stores only base speed.
- For effects with distance-scaled area size (for example `min_radius : max_radius based on cast distance`), document interpolation semantics and track deferred runtime follow-up if runtime cannot currently resolve dynamic radius.
- For multi-stage same-slot abilities (for example stage-1 dash plus stage-2 recast skillshot), document stage windows and gating semantics explicitly and track deferred runtime follow-up when stage identity is not first-class in runtime execution.
- For attack-cadence-coupled abilities, model execution semantics explicitly in `abilities.<ability_key>.execution` using stable keys when source-verified (for example `resolution_timing`, `resets_basic_attack_timer_on_cast`, `empowered_attack_window_seconds`, `max_empowered_attacks`, `target_required`) so future runtime consumption is deterministic.
- For attack-cadence-coupled abilities with two-phase behavior (for example cast-time self-buff then next-hit resolution), use semantically explicit `resolution_timing` values that describe both phases (for example `on_cast_for_self_buff_then_on_empowered_basic_attack_hit`) instead of flattening to generic `on_cast`.
- For non-trivial projectile and dash contact abilities, prefer semantically explicit `resolution_timing` values (`on_projectile_hit`, `on_dash_contact_or_completion`) instead of flattening to generic cast-complete timing.
- `execution.resolution_timing` must use the enum-like canonical vocabulary in `Simulation/data/execution_semantics_vocabulary.json` (do not introduce ad-hoc timing strings without updating that vocabulary file in the same change).
- If `target_required` or pre-resolution cancellation behavior is ambiguous after source review, keep the chosen fallback explicit in ability `context_notes` and track uncertainty follow-up in `Simulation/CONFIDENCE_REVIEW.md` and `Simulation/COVERAGE_GAPS.md`.
- For duration-limited variable-hit empowered states where a fixed attack-count cap is not source-published, temporary `max_empowered_attacks = 0` sentinel usage is acceptable only with explicit ability `context_notes` and same-change follow-up tracking in `Simulation/COVERAGE_GAPS.md` for a future explicit unbounded-within-window schema.
- For control-triggered effects, encode the full trigger set from source text (for example include both immobilize and ground triggers when both are listed).
- If an item effect depends on a shared cross-item system rule (for example support-income diminishing-gold logic), preserve conservative confidence until that shared rule is encoded explicitly and track the dependency in `Simulation/COVERAGE_GAPS.md`.
- If editing a shared-rule effect on one member of an item family (for example the support-quest upgrade line), review sibling items for rule-schema consistency and track deferred sibling harmonization in `Simulation/COVERAGE_GAPS.md`.
- If an effect's timing or values differ by mode (for example Clash vs Swiftplay sudden-death behavior), encode the mode scope explicitly in `conditions`/`context_notes` and track unresolved canonical mode-policy decisions in `Simulation/COVERAGE_GAPS.md`.
- For mode-variant item behavior, keep Tier-1 baseline semantics at the root item object and encode only divergent mode semantics in `mode_overrides.<mode_key>` (for example `mode_overrides.URF` or `mode_overrides.ARENA`) using sparse override fields.
- When adding `mode_overrides.<mode_key>` branches before runtime overlay support exists, cite the mode-page source in the item `sources` list and log deferred runtime-consumption follow-up in `Simulation/COVERAGE_GAPS.md`.
- For distributed/prismatic or other mode-exclusive items, record acquisition/availability scope explicitly in `schema_notes.context_notes` (for example Arena-only anvil acquisition) and track dataset-vs-page availability drift in `Simulation/COVERAGE_GAPS.md`.
- For distributed/prismatic mode-scoped items where Tier-1 datasets expose economy fields, keep explicit `shop.prices.total` / `shop.prices.sell` for reconciliation and document that acquisition is non-shop (for example Prismatic Item Anvils) in `schema_notes.context_notes`.
- Treat removal of already-established distributed/prismatic `shop.prices` fields as a blocking regression unless an explicit policy change is documented in `Simulation/COVERAGE_GAPS.md`.
- For round/phase progression effects, verify whether acquisition timing backfills prior progression states; if not encoded, document the gap explicitly in `schema_notes.context_notes` and track it in `Simulation/COVERAGE_GAPS.md`.
- If Tier-1 dataset fields and page-level gameplay behavior disagree (for example sell-state restrictions), document the discrepancy in `schema_notes.context_notes` and track canonical-resolution follow-up in `Simulation/COVERAGE_GAPS.md`.
- If page-level source sections disagree with each other (for example infobox/tooltip tables vs module-derived description lines), reconcile using highest-confidence current-state evidence (infobox values, tooltip tables, and patch-history context), then document the reconciliation decision explicitly in `schema_notes.context_notes`.
- When item economy fields are edited (for example `shop.prices.sell`), validate against Tier-1 dataset values and document any intentional override policy in coverage docs.
- Intentional economy overrides (for example page-verified No Sell behavior where Tier-1 datasets still provide numeric sell values) are acceptable only when the owning item file and `Simulation/COVERAGE_GAPS.md` both explicitly record the exception.
- Item stat blocks must use loader-canonical key vocabulary (for example `magicResist` and `critChance` under `stats`); legacy aliases that bypass loader mapping (for example `magicResistance`, `criticalStrikeChance`) are not acceptable.
- When Tier-1 reconciliation uses item IDs, verify ID-to-name alignment against current datasets and track any legacy-ID/name drift exception in `Simulation/COVERAGE_GAPS.md`.
- If Tier-1 and page-level sources disagree because one item ID is reused for different mode-scoped identities, do not silently collapse to one source; document reconciliation strategy in `schema_notes.context_notes` and track mode-aware identity follow-up in `Simulation/COVERAGE_GAPS.md`.
- If an item is retained only as legacy/reference data (retired or replaced in current Tier-1 datasets), add explicit `lifecycle` metadata with `exclude_from_simulation = true` and a concrete replacement/reason note.
- Lifecycle metadata on legacy/reference items should include at least `status`, `exclude_from_simulation`, `reason`, `replacement_item`, and `replacement_id` so ID/name drift remains auditable.
- When a known bug diverges from intended gameplay behavior, keep intended behavior as the canonical simulation target and document the divergence for awareness only.

## Standards By Coverage Category
### 1) Champion Coverage Standard (Data + Runtime)
Data requirements:
- Champion corpus parity is mandatory: every champion key in `From Online/champions/*.json` must have one canonical `Characters/<Champion>.json` file with matching identity.
- Champion file includes canonical `base_stats`, `basic_attack`, and `abilities`.
- Ability identities are stable and slot bindings are data-owned (no slot hardcoding in engine paths).
- Ability geometry and cast data are in `abilities.<ability_key>.execution`.
- Active abilities should have non-empty `execution` metadata; treat missing active-execution objects as a blocking coverage gap unless source data is unavailable and explicitly documented.
- For non-trivial active abilities where cast and hit resolution differ, `execution` should encode explicit semantic timing keys (for example cast-complete vs empowered-basic-attack-hit resolution) when source behavior is verified.
- Champion behavior-fidelity progress must be tracked in `Simulation/champion_behavior_verification_tracker.json` (manual verified vs source-extracted-only status).
- Tracker totals must reconcile with tracked keys after each wave: `manual_behavior_verified_champion_keys` length equals `totals.manual_behavior_verified_champions`, and `source_extracted_only_champions` equals corpus-total minus verified-count.
- Manual champion verification waves must include page-level champion ability citations for each touched champion (both in champion `sources` and in tracker wave metadata).
- Touched champion ability/effect `context_notes` entries must not contain truncated timing fragments; resolve fragmentary notes to complete behavior descriptions in the same change.
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
- Update `Simulation/champion_data_coverage_inventory.json` in the same change when champion corpus parity counts move.
- Mark the champion status as data-complete, runtime-complete, or partial (explicitly).

### 2) Ability Coverage Standard (Scripted Ability/Event Behavior)
Data requirements:
- Ability formulas and cooldown/range are sourced from canonical champion data, not inline constants.
- If followup timing exists, keep the delay in canonical ability effect data when possible.
- Ability execution semantics are documented where relevant: cast type, target/range requirements, windup/cast time, projectile or hit timing, and basic-attack-cadence coupling.
- Ability execution semantics include dynamic geometry/speed branches when relevant (for example movement-speed-scaled dash velocity, cast-distance-scaled radius, and stage-gated recast windows).

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
- Fixed-cooldown `on_activate` effects should include explicit execution fields (`cast_windup_seconds`, `cast_range`, and/or `effect_hitbox_radius`) on each active branch where source semantics support them.
- If an `on_activate` effect intentionally has no fixed cooldown (for example charge/consume/single-use/round-limited behavior), represent that explicitly in `conditions`/`schema_notes.context_notes` so it is tracked as intentional semantics rather than missing cooldown metadata.
- If an `on_activate` effect intentionally has no fixed cooldown, also encode `effects_structured[].activation_cadence` with a canonical `model` value from `Simulation/data/execution_semantics_vocabulary.json` (`charge_consumption`, `consumable_single_use`, `single_use_transform`, or `round_limited_uses`).
- If one active ability is represented by multiple `effects_structured` branches, shared active cooldown metadata should be encoded consistently across those branches.
- Missing active cooldown/cast-range metadata (when source text provides those values) is a blocking data-quality failure.
- Redirect-backed pseudo-items should include dual provenance in `sources` (redirect page + canonical parent gameplay/champion page) so behavior interpretation remains auditable.
- Trinket/ward utility items should explicitly encode charge count, recharge scaling, placement limits, level requirements, and reveal/detection timing windows when source text or notes provide those values.
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
- Canonical rune authoring includes both:
  - compatibility flat file: `Masteries/RunesReforged.json`
  - split structure: `Masteries/RunesReforged/RunesReforged.json`, `Masteries/RunesReforged/Trees/*/primary_runes.json`, `Masteries/RunesReforged/Trees/*/secondary_runes.json`, and `Masteries/RunesReforged/StatShards/stat_shards.json`
- Split structure and flat file must remain synchronized on every rune/mastery edit until runtime loader migration lands.
- Rune path slot ordering and shard slot options stay valid.
- Deterministic stat effects are encoded in parseable structured-effect forms.
- Multi-branch `per_rank` rune effects include `semantic_components` with explicit branch identities (for example melee/ranged or AD/AP) instead of relying only on positional arrays.
- Rune narrative text fields (`wiki_descriptions`, touched `long_desc`) should keep decimal formatting normalized (`x.y`, not `x. y`) so authoring audits and reviewer interpretation remain reliable.
- Mastery JSON files keep explicit `sources` provenance with complete `sources[].accessed` metadata on touched entries.
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
- Maintain champion corpus parity inventory (`Simulation/champion_data_coverage_inventory.json`) as a no-regression guardrail (`172/172` current file parity) and prioritize fidelity-normalization waves over generated champion data.
- Maintain manual behavior-verification tracker full-corpus coverage (`Simulation/champion_behavior_verification_tracker.json`, currently `172/172`) and run targeted re-verification waves when champion semantics change.
- Maintain full provenance coverage for item files with `effects_structured` (all currently sourced) and keep this as a no-regression guardrail.
- Maintain full parse-confidence completeness for structured item effects (`null`/missing queue at `0` after wave 87) and keep this as a no-regression guardrail.
- Expand champion execution semantic-key coverage on attack-cadence-coupled abilities (`execution.resolution_timing` and related keys) as a data-first precursor to runtime consumption.
- Continue active-item cooldown metadata completion on branch-structured actives until remaining no-structured-cooldown queue is cleared.
- Keep rune narrative decimal-format queue (`x. y` artifacts) at `0` across flat + split rune structures.
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
- Add ally-state stat-link runtime support for teammate-transfer effects (for example `Twin Mask`) when Arena runtime modeling is expanded.
- Add runtime loader migration for split rune authoring files (`Masteries/RunesReforged/RunesReforged.json` and tree/stat-shard files) before removing flat-file compatibility.

## Standard Adoption Rule
When adding new coverage:
1. Pick the matching exemplar from this file.
2. Meet or exceed that exemplar in data completeness, loader wiring, runtime behavior, and tests.
3. Update `Simulation/COVERAGE_GAPS.md` and `Simulation/COVERAGE_CHECKLIST.md` state in the same change.

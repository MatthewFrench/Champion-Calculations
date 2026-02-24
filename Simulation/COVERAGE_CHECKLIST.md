# Coverage Checklist

Use this checklist whenever you add or modify champion, item, rune, or shard coverage in data or runtime code.

Read first:
- `Simulation/COVERAGE_STANDARDS.md`

## 0) Scope And Intent
- [ ] You identified whether the change is data-only coverage, runtime coverage, or both.
- [ ] You identified the affected domains (champion, item, rune, shard/stat).
- [ ] You identified whether this update adds modeled behavior, updates gap tracking, or both.
- [ ] You selected the matching exemplar category in `Simulation/COVERAGE_STANDARDS.md` and used it as the implementation baseline.
- [ ] You explicitly marked whether this task is in the current data-first priority lane or deferred runtime-expansion lane.

## 1) Ownership And Schema Placement
- [ ] The field is stored in its canonical domain owner.
- [ ] No champion, item, rune, or mastery specific logic was added to shared core modules.
- [ ] Champion files keep canonical gameplay data; artificial intelligence policy is in `Simulation/data/champion_ai_profiles.json`.
- [ ] No deprecated schema keys were reintroduced.

## 2) Champion Coverage
- [ ] Champion key parity was checked against `From Online/champions/*.json`, and every added/edited canonical champion file preserves 1:1 key identity with source corpus naming.
- [ ] `Characters/<Champion>.json` has complete canonical sections (`base_stats`, `basic_attack`, `abilities`).
- [ ] Ability execution geometry and timing are on `abilities.<ability_key>.execution`.
- [ ] Champion active abilities keep non-empty `execution` objects (no active-ability execution metadata regressions).
- [ ] Non-trivial ability data was manually reviewed for in-game execution semantics (activation requirements, target/range gating, timing/windup, and player-visible resolution behavior).
- [ ] Full-corpus champion quality audit remains clean after edits (no regressions for active `execution` completeness or non-passive `context_notes` completeness).
- [ ] For attack-cadence-coupled casts (empowered-hit/reset/timed-hit patterns), both cast gating and hit-resolution timing semantics are explicitly documented in ability notes.
- [ ] Touched champion abilities have non-empty `description_source` values; missing values were backfilled from authoritative/source-corpus ability text.
- [ ] If temporary fallback-derived `description_source` values were used to close completeness, provenance-hardening follow-up was explicitly tracked in `Simulation/COVERAGE_GAPS.md`.
- [ ] Touched champion/effect `context_notes` strings were checked for truncation artifacts (for example `for 0.` / `within 4.`) and normalized to complete timing/unit semantics.
- [ ] Truncation audits used strict fragment criteria (integer-dot tokens) so valid decimal timing values (for example `0.25`) were not misclassified.
- [ ] Truncation audits included a secondary cadence-fragment sweep (for example `every 0.` patterns) so truncated periodic-tick notes were not missed.
- [ ] Truncation audits included a tertiary article-token sweep (for example `during the 0.` patterns) so article-interposed timing fragments were not missed.
- [ ] Truncation audits included a cast-time fragment sweep (for example `have a 0.` patterns) so cast-windup truncation defects were not missed.
- [ ] Truncation audits handled both `context_notes` string and `context_notes` array shapes so queue counts were not underreported.
- [ ] If scripted/bulk truncation cleanup was used, replacements were scoped to `context_notes` fields and a post-edit audit confirmed `description`/`description_source` text was unchanged unless intentionally updated.
- [ ] Intentional integer quantities in touched notes avoid terminal integer-dot phrasing (for example use `3 stacks` instead of `3.`) so truncation audits do not flag false positives.
- [ ] `Simulation/champion_behavior_verification_tracker.json` was updated to reflect manual verified champion/ability scope for this wave (or explicitly marked unchanged with reason).
- [ ] Tracker integrity check passed: `manual_behavior_verified_champion_keys` count equals `totals.manual_behavior_verified_champions`, and `source_extracted_only_champions` equals corpus-total minus verified-count.
- [ ] When tracker baseline is full-corpus manual coverage (`172/172`), touched champions remain in `manual_behavior_verified_champion_keys` unless a documented downgrade reason is recorded.
- [ ] Each manually verified champion in this wave includes at least one page-level champion ability source in `sources`, and the wave entry in `Simulation/champion_behavior_verification_tracker.json` records that page-level citation.
- [ ] `sources` and `schema_notes` are updated.
- [ ] Champion `sources` entries include complete provenance metadata (`url`/`path`, `accessed`, `used_for`), and missing `sources[].accessed` values were backfilled on touched files.
- [ ] If champion behavior changed at runtime, script logic is implemented in `Simulation/src/scripts/champions/<champion>/`.
- [ ] If controlled champion behavior was added, `Simulation/src/scripts/champions/controlled_champion.rs` registry wiring is updated.
- [ ] If enemy scripted events changed, `Simulation/src/scripts/champions/mod.rs` event registry and labels are updated.
- [ ] Champion runtime behavior has tests.

## 3) Item Coverage
- [ ] `Items/<Item>.json` includes accurate `stats`, passive and active text, and `effects_structured`.
- [ ] Item `stats` keys use loader-canonical names (for example `magicResist`, `critChance`, not legacy aliases like `magicResistance` or `criticalStrikeChance`).
- [ ] Structured effects use stable effect identifiers and include trigger, cooldown, duration, and scaling where applicable.
- [ ] Active cast effects include explicit cooldown and cast-range metadata when these values are present in source text (blocking gate when applicable).
- [ ] Trinket/ward utility items encode source-verified charge count, recharge scaling, placement limits, level requirements, and reveal/detection timing windows when available.
- [ ] Non-trivial active/on-hit/combat-triggered item effects were manually reviewed for execution semantics (activation gating, target/range requirements, timing/windup, and resolution timing).
- [ ] `schema_notes.effects_structured_reviewed` is updated.
- [ ] `schema_notes.effects_structured_reviewed` uses ISO date format `YYYY-MM-DD`.
- [ ] If a non-ISO legacy review note was replaced, the original note is preserved in `schema_notes.context_notes`.
- [ ] Canonical item files touched in this change remain sourced even when `effects_structured` is empty (non-structured/stat-only/placeholder entries are not exempt from provenance).
- [ ] Runtime-modeled items include explicit `sources` entries (do not leave modeled items with `sources: null`).
- [ ] `sources` entries include `url` or `path`, `accessed`, and `used_for` fields.
- [ ] If the edited item file had legacy source entries missing `accessed`, they were backfilled in the same change.
- [ ] Cited source URLs were verified to resolve at authoring time, and CommunityDragon item dataset citations use the current `global/default/v1/items.json` endpoint.
- [ ] Non-trivial or low-confidence item effects include at least one Tier-2 citation source (CommunityDragon or League Wiki) when available.
- [ ] Low-confidence formula interpretations include at least one page-level verification source (for example item page or patch notes), not only dataset-level citations.
- [ ] If a cited item page is a redirect (pseudo-item/turret-item/minion-item/champion-upgrade identities), `sources` includes both the redirect URL and the canonical parent gameplay/champion page used for behavior verification.
- [ ] For non-trivial edits to existing item data, entity-purpose and gameplay-pattern sanity check was completed and captured in `schema_notes.context_notes` when semantics/confidence changed.
- [ ] For semantics updates, `schema_notes.context_notes` includes a concise execution-model note (what the player does, when effect applies, and who receives it).
- [ ] If updated data semantics cannot be represented by current runtime capabilities, deferred code follow-up scope is explicitly recorded in `Simulation/COVERAGE_GAPS.md` in the same change.
- [ ] For ally-state transfer effects (for example teammate stat-link mechanics), dynamic update and alive/ownership gating semantics are documented in data, and deferred runtime follow-up is tracked when unmodeled.
- [ ] If an effect has separate basic-attack-hit and ability-damage branches, each branch is represented as its own structured effect with explicit trigger and limiter semantics.
- [ ] For multi-phase active effects (for example mark/recast/timeout), each branch and cooldown-start event is explicitly represented in structured data and/or context notes.
- [ ] For single-use transform actives, post-use state transitions (for example transform/shatter behavior and reactivation constraints) are explicitly represented in structured data and/or context notes.
- [ ] For actives with both cooldown and cast-lockout text, cooldown and lockout are represented separately (lockout is not encoded as base cooldown).
- [ ] If source text provides both per-tick and total-over-duration values, structured values are cadence-consistent with the interval and retain total-duration auditability.
- [ ] For control-triggered effects, all listed trigger classes are represented (for example immobilize plus ground when both are specified).
- [ ] If source notes define trigger exclusions or edge interactions (for example dodged/blocked/blinded misses, spell-shield handling, proc-damage class, zero-damage proc eligibility, or below-threshold retrigger behavior), those rules are encoded in structured `conditions`/`modifiers` and reflected in `schema_notes.context_notes`.
- [ ] If the effect depends on a shared cross-item system rule (for example support-income diminishing-gold), conservative confidence and explicit dependency tracking were kept in `Simulation/COVERAGE_GAPS.md`.
- [ ] If a shared-rule effect was edited on one item in a progression/family set, sibling items were reviewed for rule-schema/confidence consistency (or deferred sibling work was explicitly tracked in `Simulation/COVERAGE_GAPS.md`).
- [ ] If effect timing/values differ by mode, mode scope is explicit in `conditions`/`schema_notes.context_notes`, and unresolved cross-mode policy decisions are tracked in `Simulation/COVERAGE_GAPS.md`.
- [ ] For mode-variant item behavior, root data remains Tier-1 baseline and only divergent semantics are encoded under `mode_overrides.<mode_key>` (or explicitly tracked as deferred policy work in `Simulation/COVERAGE_GAPS.md`).
- [ ] If `mode_overrides.<mode_key>` branches were added or changed, each branch has explicit mode-page citation provenance and runtime overlay-consumption follow-up is tracked when current runtime cannot consume overlays yet.
- [ ] If the item is distributed/prismatic or mode-exclusive, acquisition/availability scope is explicit in `schema_notes.context_notes`, and dataset-vs-page availability drift is tracked in `Simulation/COVERAGE_GAPS.md` when discovered.
- [ ] If the item is distributed/prismatic and Tier-1 datasets expose economy values, `shop.prices.total`/`shop.prices.sell` are explicitly populated and context notes clarify non-shop acquisition semantics (for example Prismatic Anvils).
- [ ] Existing distributed/prismatic `shop.prices` fields were preserved during edits (or any intentional removal/policy shift is explicitly documented in `Simulation/COVERAGE_GAPS.md`).
- [ ] For round/phase progression effects, acquisition-timing backfill behavior was verified; if not encoded, the gap is explicitly documented in `schema_notes.context_notes` and tracked in `Simulation/COVERAGE_GAPS.md`.
- [ ] Any dataset-vs-page-level discrepancy discovered during review (for example sell-state or restriction behavior) is documented in `schema_notes.context_notes` and tracked in `Simulation/COVERAGE_GAPS.md`.
- [ ] If current canonical effect text and patch-history values diverge, canonical current behavior is modeled while historical values are kept as context-only metadata.
- [ ] If threshold brackets are inferred from published cadence/cap constraints (not explicitly tabulated in source text), inference status is documented in `modifiers`/`schema_notes.context_notes` and tracked in `Simulation/CONFIDENCE_REVIEW.md`.
- [ ] If sources include known bug-specific behavior, structured data still models intended behavior by default and bug divergence is documented as a note only.
- [ ] Intended non-bug behavior remains the default canonical data model for all touched entities (bug behavior is never promoted to canonical data).
- [ ] Cleanse-style actives include source-verified activation constraints and scope nuances when available (for example airborne lockout, suppression/nearsight handling, cast-time behavior).
- [ ] If item economy fields were edited (for example `shop.prices.sell`), values were cross-checked against Tier-1 dataset values and intentional overrides were documented.
- [ ] If item economy fields were edited (for example `shop.prices.total` / `shop.prices.sell`), Tier-1 ID/name drift was checked (legacy file identity vs current dataset identity), and any exception was tracked in `Simulation/COVERAGE_GAPS.md`.
- [ ] If Tier-1 and page-level sources diverge because one item ID maps to different mode-scoped identities, reconciliation notes were added in `schema_notes.context_notes` and mode-aware identity follow-up was tracked in `Simulation/COVERAGE_GAPS.md`.
- [ ] If the item is retired/replaced but retained for reference, `lifecycle` metadata explicitly marks simulation exclusion (`exclude_from_simulation = true`) with replacement/reason notes.
- [ ] Legacy/reference lifecycle metadata includes required fields (`status`, `exclude_from_simulation`, `reason`, `replacement_item`, `replacement_id`) so ID/name drift handling is explicit.
- [ ] Any intentional sell/economy override is listed as an explicit exception (item file context notes + `Simulation/COVERAGE_GAPS.md` snapshot/priority notes).
- [ ] Any `parse_confidence < 0.65` entry is tracked with explicit follow-up in `Simulation/COVERAGE_GAPS.md`.
- [ ] Touched structured item effects keep numeric `parse_confidence` values (no null/missing confidence on edited `effects_structured` entries).
- [ ] Runtime-modeled effect entries use numeric `parse_confidence` values (no null/missing confidence on modeled effects).
- [ ] Runtime-modeled item condition tokens remain parser-compatible with existing loaders (or loader updates are shipped in the same change).
- [ ] If the item affects combat or runtime behavior, behavior is implemented in scripts or runtime modules (not core engine modules).
- [ ] If runtime-modeled, the item key is added to `Simulation/src/scripts/coverage.rs`.
- [ ] Item behavior has regression tests.

## 4) Rune And Shard Coverage
- [ ] Rune canonical data is synchronized between flat compatibility file (`Masteries/RunesReforged.json`) and split structure (`Masteries/RunesReforged/RunesReforged.json` + `Trees/*/primary_runes.json` + `Trees/*/secondary_runes.json` + `StatShards/stat_shards.json`).
- [ ] Rune decimal text normalization was checked for generated spacing artifacts (for example `0. 5` -> `0.5`) in both flat and split files when touched.
- [ ] Split rune-tree structure exists and is complete for all trees (`Domination`, `Inspiration`, `Precision`, `Resolve`, `Sorcery`) with both primary and secondary files.
- [ ] Mastery files touched in this change include explicit `sources` entries with complete `accessed` metadata (backfill missing `sources[].accessed` values in touched files).
- [ ] Deterministic stat effects are parseable by `Simulation/src/data.rs`.
- [ ] `effect_type = stat_modifier` entries do not use null/empty `stat` unless explicitly documented as narrative-only.
- [ ] New deterministic stat keys are handled in `apply_stat_bonus` when needed.
- [ ] Dynamic rune behavior is added to `Simulation/src/scripts/runes/effects.rs` and runtime logic is implemented.
- [ ] Runtime rune tuning defaults are added to `Simulation/data/simulator_defaults.json` and `Simulation/src/defaults.rs`.
- [ ] Rune runtime behavior has tests.
- [ ] Shard stat keys are supported or explicitly documented as unmodeled.

## 5) Coverage Documentation Tracking (Required)
- [ ] `Simulation/COVERAGE_GAPS.md` snapshot counts are updated when modeled sets or legal pools change.
- [ ] `Simulation/COVERAGE_GAPS.md` lists reflect the current champion, item, rune, and shard runtime state.
- [ ] `Simulation/champion_data_coverage_inventory.json` is refreshed when champion corpus parity counts change.
- [ ] `Simulation/champion_behavior_verification_tracker.json` is refreshed when manual champion verification scope changes.
- [ ] Data coverage and runtime coverage are distinguished explicitly (do not mark data-only completion as runtime completion).
- [ ] Compatibility aliases are documented when they affect coverage interpretation.
- [ ] Intentional unchanged gaps are explicitly recorded as unchanged.
- [ ] Deferred runtime-expansion items remain listed when data-first execution is active.

## 6) Quality Gates And Project Tracking
- [ ] Unmodeled rune and unmodeled item behavior is handled intentionally (modeled, gated, or tracked as a gap).
- [ ] `Simulation/IMPROVEMENT_TRACKER.md` is updated with landed work.
- [ ] `Simulation/README.md` and root `README.md` are updated when contributor workflow or documentation navigation changed.

## 7) Confidence And Research
- [ ] Source hierarchy was followed (Riot/Data Dragon first, secondary validation second, meta sites only for presets/context).
- [ ] Web research was used where needed to verify behavior details and close data gaps.
- [ ] Entity-intent review was completed for edited assets (role, gameplay purpose, and modeled behavior alignment).
- [ ] Manual in-game behavior review was completed for edited non-trivial effects (activation, timing, and player-visible outcomes), not only schema-level checks.
- [ ] Data provenance is recorded in owning files (`sources` with URL, accessed date, and usage notes).
- [ ] Uncertain mechanics were verified from authoritative sources when possible.
- [ ] Remaining ambiguity and assumptions are recorded in `Simulation/CONFIDENCE_REVIEW.md`.

## 8) Validation Gates (Required)
- [ ] `cargo fmt --manifest-path Simulation/Cargo.toml`
- [ ] `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
- [ ] `cargo test --release --manifest-path Simulation/Cargo.toml`

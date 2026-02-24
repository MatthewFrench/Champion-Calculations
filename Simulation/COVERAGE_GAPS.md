# Coverage Gaps Tracker (2026-02-24)

This file tracks what is complete and not complete for coverage across data and runtime code.

## How To Use This File
1. Read **Coverage At A Glance** for the current status.
2. Use the domain sections to find exactly what is missing.
3. Update this file whenever modeled sets, counts, or fidelity assumptions change.
4. Keep this file aligned with:
   - `Simulation/CURRENT_STATE.md`
   - `Simulation/COVERAGE_CHECKLIST.md`
   - `Simulation/COVERAGE_STANDARDS.md`

## Coverage Quality Standards
- Use `Simulation/COVERAGE_STANDARDS.md` to determine the expected data/code/test quality bar for each category before adding or claiming coverage.
- Use this file (`COVERAGE_GAPS.md`) to track what is complete vs incomplete after those standards are applied.

## Active Focus (Data-First)
- Current priority is data coverage quality and provenance improvements.
- Runtime/code expansion items are documented below as deferred backlog for later execution.

## Coverage At A Glance
| Domain | Data Coverage | Runtime Coverage | Current Gap |
| --- | --- | --- | --- |
| Champion files (From Online parity) | `172/172` champions from `From Online/champions/*.json` have canonical `Characters/<Champion>.json` files | N/A | File-parity gap is closed; maintain no-regression fidelity and confidence follow-ups on future champion edits |
| Champion active-ability execution metadata (full corpus) | `682/682` active abilities across canonical champions include non-empty `execution` objects | N/A | No active-execution gap; keep as no-regression guardrail |
| Champion ability context-note coverage | `860/860` champion abilities include `context_notes` execution semantics | N/A | No context-note completeness gap; keep as no-regression guardrail |
| Champion manual behavior verification coverage | `172/172` champions are marked manual verified in `Simulation/champion_behavior_verification_tracker.json` | N/A | Full-corpus manual behavior verification is complete; maintain no-regression on future champion edits |
| Controlled champion scripts (full corpus) | `172/172` canonical champions have ability data | `1/172` (`Vladimir`) | `171` champions remain data-complete but unscripted at runtime |
| Enemy scripted-event champions (full corpus) | `172/172` canonical champions have ability data | `5/172` (`Doctor Mundo`, `Morgana`, `Sona`, `Vayne`, `Warwick`) | Enemy scripted-event runtime coverage remains intentionally partial |
| Item files | `320` canonical item JSON files (`295` loaded by runtime after excluded ranks), plus `2` non-canonical report artifacts under `Items/` | Runtime effect coverage is a subset of legal URF pool | Effect behavior not modeled for many items |
| Legal URF legendary pool | `113` items | `9` items with modeled runtime effects (of `111` with effect payload) | `102` legal items still unmodeled at runtime |
| RunesReforged runes | `61` runes with split-tree authoring files (`5/5` trees + stat shards split file) | `22` modeled (`16` dynamic + `6` deterministic static) | `39` runes unmodeled; runtime still consumes flat compatibility file pending loader migration |
| Stat shards | `3` slots (`2` options each) | Deterministic parse covers shard stats | Tenacity runtime effect incomplete |

## Deferred Runtime Expansion Backlog (Expand Later)
- Resolve current controlled-champion script channel visibility/export regressions so validation gates pass again (`Simulation/src/scripts/champions/controlled_champion.rs` and `Simulation/src/scripts/champions/controlled_champion/*`; unresolved symbol/re-export errors in `engine.rs`, `scenario_runner.rs`, and related tests).
- Expand controlled champion script coverage beyond `Vladimir` using the existing capability interface.
- Add full enemy-script event-path tests (cooldown, range, and followup behavior per event).
- Unify rune runtime key mapping and telemetry key mapping to one canonical table.
- Complete runtime tenacity behavior application for shard-supported tenacity stats.
- Add ally-target chain/fallback runtime resolution support for `Moonstone Renewer` Starlit Grace (nearest-other-ally within 800 units, else same-target fallback).
- Add runtime support for charge-based Manaflow progression and transform thresholds (`Archangel's Staff`, `Whispering Circlet`, `Winter's Approach`) so stack/cap/transform behavior can be simulated directly.
- Add runtime support for visibility-gated prep buffs and champion-only consume triggers (`Umbral Glaive` Nightstalker unseen/seen windows and empowered-hit consume branch).
- Add runtime support for movement-state stack engines with max-stack trail aura windows and melee consume-on-next-hit branching (`Trailblazer` Momentum).
- Add runtime support for owner attack-type split duration/value branches and on-CC ally-aura activation (`Bandlepipes` Fanfare melee/ranged variance).
- Add runtime support for explicit on-attack trigger class and cooldown-reduction-on-hit/crit rules (`Yun Tal Wildarrows` Practice/Flurry timing model).
- Add mode-aware gating support for resource-cost multiplier branches when mode rules set ability costs to zero (`Actualizer` in URF).
- Add runtime support for ally-link ownership constraints and dead-state passive suspension (`Wordless Promise` Promise tether and stat-sharing behavior).
- Add runtime support for Quicksilver-group cooldown isolation and expanded cleanse subtype handling (`Quicksilver Sash` and `Mercurial Scimitar` airborne activation lockout plus suppression/nearsight scope).
- Add runtime mode-overlay consumption for `mode_overrides` branches so mode-specific item semantics (for example `Overcharged` in Swiftplay/URF) are selected automatically at load/runtime.
- Add runtime loader support for split rune authoring structure (`Masteries/RunesReforged/RunesReforged.json` + tree/stat-shard files) so flat compatibility file can be deprecated safely.
- Keep runtime compile-integrity no-regression across ongoing schema/decomposition refactor slices so required validation gates (`cargo fmt`, `cargo clippy -D warnings`, `cargo test --release`) remain clean after data-only waves.

## Data Quality Gap Snapshot (2026-02-24 Audit, Updated)
| Data Area | Current State | Gap |
| --- | --- | --- |
| Item provenance (`Items/*.json` with `effects_structured`) | `0/243` have `sources: null` | Provenance backlog is cleared; keep this as a no-regression guardrail |
| Item source metadata completeness (`sources[].accessed`) | `0/243` structured item files have source entries missing `accessed` date metadata | Structured-item source metadata normalization is complete; keep this as a no-regression guardrail |
| Item review metadata (`effects_structured_reviewed`) | `0` missing, `0` non-ISO format | Legacy non-ISO values normalized; enforce ISO format on new edits |
| Item parse confidence | `0` item files have `parse_confidence < 0.65` | Low-confidence backlog is cleared; keep this as a no-regression guardrail |
| Modeled runtime item data precision | `0/9` modeled runtime item files have minimum numeric `parse_confidence <= 0.60`; `0/9` remain at `0.65`; `0/9` have missing numeric confidence | Modeled runtime confidence floor is now >= `0.70`; maintain this as a no-regression guardrail |
| Legal URF unmodeled item provenance | `0/102` legal unmodeled effect items still have `sources: null` | High-impact legal URF provenance backlog is cleared; maintain this as a guardrail |
| Secondary citation depth | `243/243` item files with `effects_structured` include Tier-2 citations (`CommunityDragon` and/or League Wiki) | Tier-2 dataset coverage is complete; maintain for new updates |
| Page-level formula citation depth | `243/243` item files include page-level League Wiki item citations | Queue is cleared; keep this as a no-regression guardrail while maintaining page-level depth on future edits |
| Higher-priority no-page citation queue (`EPIC`/`LEGENDARY`/`BOOTS`/`STARTER`) | `0` files remain without page-level citation | Queue is cleared; keep this as a no-regression guardrail while broadening lower-priority page-depth coverage |
| Low-confidence page-verification queue | `0/0` low-confidence item files (`parse_confidence < 0.65`) lack page-level citation | Queue is cleared; keep this as a no-regression guardrail |
| Legal URF unmodeled page-level citation depth | `0/102` legal-URF unmodeled effect items still have no page-level citation | Queue is cleared; keep this as a no-regression guardrail while expanding broader page-depth coverage |
| CommunityDragon source URL accessibility | `0/243` structured item files still cite legacy `global/en_us/v1/items.json` source URLs | Endpoint migration is complete; keep URL-health checks as a no-regression guardrail |
| League Wiki URL-health automation reliability | Automated URL checks against League Wiki currently return HTTP `403` for valid page URLs in this environment | Keep normalized League Wiki URLs in `sources`; treat manual browser verification as acceptable when scripted URL checks are blocked |
| Canonical sell-price alignment | `3/300` item files with numeric sell values differ from Data Dragon `16.3.1` sell values (`World Atlas`, `Runic Compass`, `Bounty of Worlds`) | All current mismatches are intentional page-level No Sell overrides for support-quest staging; keep this explicit exception register |
| Item stat-key canonicalization | `0/322` item files use legacy `stats.magicResistance` key (`30` files normalized to `stats.magicResist`) | Canonical key normalization is complete; keep this as a no-regression guardrail to avoid silent magic-resist stat drops in loader parsing |
| Crit-stat key canonicalization | `0/322` item files use legacy `stats.criticalStrikeChance` keys (`22` files normalized in dedicated wave plus opportunistic early migration on `Navori Flickerblade`) | Canonical crit stat-key normalization is complete; keep this as a no-regression guardrail |
| Canonical total-cost alignment | `1/300` item files with numeric total price differ from Data Dragon `16.3.1` due ID/version drift (`Zephyr` file uses Arena-scoped legacy identity on ID `3172`, while Tier-1 baseline maps to `Gunmetal Greaves`) | Track explicit legacy-ID exception policy and resolve canonicalization strategy for mixed-epoch/mode-shared item identities |
| Legacy/retired item exclusion markers | `1` file (`Zephyr`) currently uses explicit `lifecycle.exclude_from_simulation = true` metadata | Standardize lifecycle-marker policy for any additional retired/replaced identities so legacy data does not enter live simulation pools |
| Rotating-mode availability lifecycle policy | `Wordless Promise`, `Perplexity`, and `Atma's Reckoning` now include page-level notes indicating rotating-mode/patch-history availability variance | Define canonical lifecycle/availability policy for rotating-mode-only item identities so simulation pools remain intentionally scoped |
| Distributed-item economy semantics | `57/57` distributed/prismatic item files now include explicit `shop.prices` with Tier-1 reconciliation | Queue is cleared; keep this as a no-regression guardrail while finalizing canonical policy for page-vs-dataset economy presentation drift (`Cost 0 / Sell 2000` page display vs Tier-1 reconciliation values on several Arena distributed identities) |
| Redirect-backed pseudo-item provenance policy | `10/10` redirect-backed pseudo-item identities in the latest queue now include both redirect page citations and canonical parent gameplay/champion-page citations | Keep redirect + parent-page dual-citation policy as a no-regression requirement for pseudo-item updates |
| Distributed-item mode-availability drift semantics | `Divine Sunderer`, `Goredrinker`, `Prowler's Claw`, `Gargoyle Stoneplate`, `Duskblade of Draktharr`, `Everfrost`, `Night Harvester`, `Radiant Virtue`, `Moonflair Spellblade`, `Crown of the Shattered Queen`, and `Turbo Chemtank` now include page-verified Arena-scoped availability/context notes | Finalize canonical policy for dataset-vs-page availability/map-flag drift and mode overlays for distributed/prismatic identities (including `Prowler's Claw` map-flag divergence) |
| Rotating-mode stat-package drift semantics | `Atma's Reckoning` and `Perplexity` now include page-level patch-history context notes showing alternate rotating-mode stat/cost packages versus Tier-1 baseline snapshots | Define canonical policy for representing mode-variant stat/cost packages when Tier-1 baseline and rotating-mode pages diverge |
| Mode-scoped sudden-death timing semantics | `Overcharged` now keeps Clash timing at root and encodes Swiftplay/URF timing-value divergence under `mode_overrides` | Extend canonical mode-overlay policy beyond `Overcharged` and keep mode-key naming/shape consistent across future mode-variant items |
| Mode-overrides runtime interpretation follow-up | `Overcharged` (`SWIFTPLAY`/`URF`) plus `Guardian's Horn` and `Guardian's Orb` (`ARENA`) now carry data-level `mode_overrides` branches for mode-variant semantics | Add runtime mode-overlay consumption so simulations can select mode branches automatically instead of relying only on root-baseline fields |
| Trinket vision/charge runtime follow-up | `Arcane Sweeper`, `Farsight Alteration`, `Oracle Lens`, and `Stealth Ward` now encode page-verified cast windows, charge/recharge scaling, ward limits, and reveal/detection branches in structured data | Runtime currently lacks trinket vision-entity simulation (charge state, ward placement limits, reveal radius/scouting entities), so behavior remains data-documented but not executable in simulation loops |
| Dragonheart late-acquisition stack semantics | `Dragonheart` now documents that acquisition-round immediate soul-grant behavior exists in patch-history notes but is not encoded | Decide whether to model acquisition-round soul backfill/phase timing as canonical structured data or keep as documented non-modeled behavior |
| Champion-ability upgrade pseudo-item semantics | `Fire at Will` now documents Gangplank-ultimate-upgrade coupling and redirect-based source provenance | Define canonical policy for pseudo-item upgrade entries (source requirements, rank/tag conventions, and simulation eligibility expectations) |
| Moonstone ally-chain runtime follow-up | `Moonstone Renewer` now documents nearest-other-ally chain targeting, same-target fallback branch, and 800-unit scope in structured/context notes | Add runtime ally-target selection + fallback chain-resolution support (deferred code work) so Starlit Grace behavior is simulation-faithful |
| Manaflow charge/transform runtime follow-up | `Archangel's Staff`, `Whispering Circlet`, and `Winter's Approach` now encode page-verified charge cadence, consume triggers, and transform thresholds with clearer execution notes | Add runtime charge-state and transform-threshold support so stack/cap progression is simulation-faithful when these items are modeled |
| Promise tether runtime follow-up | `Wordless Promise` now documents dead-state passive suspension, tether-gated stat sharing, and active link constraints (including `10s` Promise cooldown and team/target limits) with page-level verification | Add runtime ally-link ownership, dead-state suspension, and dynamic stat-link recalculation support for Promise behavior |
| Arena teammate-stat-link runtime follow-up | `Twin Mask` now documents page-verified dynamic teammate-stat transfer (live-stat updates + alive/dual-item gated amplification) | Add runtime ally-state stat-link support so teammate-sharing effects can be simulated when Arena item runtime coverage is expanded |
| Visibility/movement/on-attack runtime follow-up | `Actualizer`, `Bandlepipes`, `Trailblazer`, `Umbral Glaive`, and `Yun Tal Wildarrows` now include page-verified mode gates, owner-attack-type splits, visibility windows, movement-stack timing, and on-attack cooldown-reduction semantics | Add runtime support for mode-aware resource branches, attack-type variance, visibility-state windows, movement-stack trail engines, and explicit on-attack trigger classes |
| Lifeline special-interaction fidelity | `Lifeline` now documents anchor-window timing, recast-window timing, and cooldown-start-after-blink behavior | Special-case interactions with displacement/channel/crowd-control states remain partially ambiguous and are not fully encoded |
| Muramana Shock proc-classification fidelity | `Muramana` now encodes both attack-hit and champion-ability Shock branches, including cast-instance/target throttling and proc-damage exclusion notes | Runtime proc-damage classification for champion-ability branches is still a deferred code-layer follow-up |
| Golden Spatula multi-effect fidelity | `The Golden Spatula` now has module/page-level cross-verification, expanded execution-semantics notes, and minimum parse confidence `0.65` | Keep follow-up on cross-mode stat-package drift (Arena vs Mayhem) and non-modeled runtime handling for death-persistence edge behavior |
| Gambler's Blade stored-gold cap reconciliation | Current Tier-1/local tooltip data and structured effects use `30` to `240` stored-gold range | Verify wiki patch-history `245` cap note against current authoritative data and clarify whether this is a historical-only value or an unresolved data discrepancy |
| Runtime-modeled condition-token compatibility | Protoplasm Lifeline threshold token restored to loader-compatible form (`health_below_30_percent`) | Add repeatable audits for compatibility-sensitive condition-token edits on modeled runtime items |
| Known-bug behavior simulation policy | Coverage standards/checklist require intended-behavior modeling by default, with bug-specific observations captured as notes only | Keep bug behavior out of canonical simulation data; document divergences for awareness and follow-up discussion only |
| Active-cast execution metadata completeness | Targeted active metadata normalization now covers `Mikael's Blessing`, `Hextech Gunblade`, `Mercurial Scimitar`, `Profane Hydra`, `Randuin's Omen`, `Shurelya's Battlesong`, `Youmuu's Ghostblade`, and `Wordless Promise` with explicit cooldown/cast-range-or-radius semantics where source text provides values | Continue broader audit so targeted item actives consistently encode cooldown and cast-range metadata when sources publish them |
| Quicksilver cleanse edge-semantics runtime follow-up | `Quicksilver Sash` and `Mercurial Scimitar` now encode page-verified cleanse/activation nuances in data (`90s` cooldown, airborne activation lockout, suppression/nearsight removal, no-cast-time/stealth-preserving activation, and item-group cooldown-transfer note) | Add runtime support for Quicksilver-group cooldown-isolation and extended cleanse subtype handling beyond generic cleanse semantics |
| Black Spear bind-window runtime follow-up | `Black Spear` now captures page-verified cast/channel/incapacitation timing and champion-scope semantics in structured data/context notes | Add runtime handling for bind-window edge behavior (rebind/no-effect time windows and invalid-target cooldown branch behavior) while keeping intended behavior as default |
| Trigger-exclusion and proc-edge semantics normalization | `Bami's Cinder`, `Bramble Vest`, `Catalyst of Aeons`, `Hexdrinker`, `Hextech Alternator`, `Executioner's Calling`, `Oblivion Orb`, `Sheen`, `Seeker's Armguard`, and `Warden's Mail` now include page-verified trigger exclusions and interaction-edge semantics (miss/parry/blind exclusions, shield/spell-shield interactions, proc-damage notes, zero-damage proc behavior, threshold retrigger clauses, and source/cap nuances) | Continue normalizing edge-semantics coverage across remaining on-hit/proc items so simulation behavior does not rely on implicit assumptions |
| Legacy-expectation behavior drift clarity | `Seeker's Armguard` now has page-verified notes that current data-version behavior is the single-use Time Stop variant that transforms into `Shattered Armguard` | Continue documenting identity-vs-behavior drift where long-lived item names keep legacy expectations but current gameplay behavior differs materially |
| Legal URF denominator parity | Coverage snapshot now reflects runtime-filter parity from `default_item_pool` + runtime effect-payload detection (`113` pool / `111` payload / `102` unmodeled) | Keep coverage metrics derived from runtime-filter-equivalent audits to avoid denominator drift |
| Legal URF low-confidence unmodeled citation depth | `0/0` legal URF unmodeled low-confidence item files have no page-level League Wiki citation | Citation-depth queue is cleared; keep this as a no-regression guardrail |
| Shared support-income rule precision | Support-quest family (`World Atlas`, `Runic Compass`, `Bounty of Worlds`, `Bloodsong`, `Celestial Opposition`, `Dream Maker`, `Solstice Sleigh`, `Zaz'Zak's Realmspike`) now encodes page-verified pre/post-5-minute threshold and reduction formulas | Exact per-minute windowing/team-item dynamics still rely on shared support-income rule-table encoding |
| Support-quest sell-state representation | Quest-stage support items (`World Atlas`, `Runic Compass`, `Bounty of Worlds`) now encode page-verified No Sell behavior with `shop.prices.sell = 0` and explicit Tier-1 discrepancy notes | Keep intentional override policy documented and re-verify if official datasets expose sell-restriction flags |
| Rune structured stat semantics | `0` runes include `stat_modifier` entries with null/empty `stat` | Cleared from baseline `21`; keep using `condition_note` for narrative-only entries |
| Rune parse confidence | `0` runes include `parse_confidence <= 0.60` | Low-confidence rune note cleanup wave completed; maintain this as a no-regression guardrail |
| Preset item provenance (`enemy_urf_presets` items with `effects_structured`) | `0` currently unsourced | Preset data now has provenance coverage; maintain for new preset updates |
| Non-structured item provenance (`Items/*.json` without `effects_structured`) | `0/77` canonical non-structured item files have null/empty `sources` | Queue is cleared for canonical item files; keep this as a no-regression guardrail |
| Items-folder provenance completeness (`Items/*.json`, canonical + report artifacts) | `0/322` item JSON files have null/empty `sources`; `0/322` have source entries missing `accessed` | Folder-level provenance/accessed normalization is complete; keep this as a no-regression guardrail |
| Champion corpus parity (`From Online/champions` -> `Characters`) | `172/172` champion keys have canonical `Characters/<Champion>.json` parity (`0` missing) | File-parity backlog is cleared; keep inventory as a no-regression guardrail |
| Champion manual verification tracker coverage | `172/172` champions currently marked `manual_behavior_verified` (`100%`) in `Simulation/champion_behavior_verification_tracker.json` | Full-corpus manual verification is complete; keep this as a no-regression guardrail and re-open targeted waves when semantics change |
| Champion verification tracker integrity | `manual_behavior_verified_champion_keys` count and tracker totals are reconciled (`172` verified keys, `0` source-extracted-only, corpus `172`) | Keep per-wave reconciliation check as a no-regression guardrail (`verified_count == key_count`; `source_extracted_only == corpus_total - verified_count`) |
| Champion manual-verification page-level citation depth | `3/3` champions in the latest fidelity wave include explicit page-level champion ability citations in both touched champion `sources` and tracker wave scope | Enforce this as a no-regression requirement for all future manual verification waves and backfill older-wave citation annotations where missing |
| Champion context-note truncation audit | `0` ability entries across `0` champion files are currently flagged by strict regex audit as candidate truncated timing fragments in `context_notes`; secondary cadence-fragment sweep (`every 0.` class) and tertiary article-token sweep (`during the 0.` class) are also `0/0` | Queue is cleared; keep string+array-aware strict auditing plus cadence/article-token sweeps as no-regression guardrails |
| Source-extracted-only strict-fragment queue | `0` source-extracted-only champions currently have strict-fragment notes | Queue is cleared; keep this as a no-regression guardrail while maintaining full-corpus manual verification coverage |
| Champion source metadata completeness (`Characters/*.json`) | `0/173` champion JSON files have null/empty `sources`; `0/173` have source entries missing `accessed` | Champion provenance/accessed normalization is complete; keep this as a no-regression guardrail |
| Champion active-ability execution metadata completeness | `682/682` champion active abilities include non-empty `execution` objects | Active-ability execution metadata backlog is cleared; keep this as a no-regression guardrail |
| Champion context-note coverage completeness | `860/860` champion abilities include `context_notes` | Context-note completeness backlog is cleared; keep this as a no-regression guardrail |
| Champion ability `description_source` completeness | `860/860` champion abilities currently include non-empty `description_source` (`0` missing across `0` champions) | Backlog is cleared; keep this as a no-regression guardrail on all champion edits |
| Champion `description_source` provenance depth | Missing entries were backfilled to close completeness (`860/860`) using canonical ability `description` fallback text when source strings were absent | Run targeted provenance-hardening pass to replace fallback-derived `description_source` values with direct source-corpus strings on high-impact champions |
| Champion stochastic-spawn weighting confidence | `Zyra` `Garden of Thorns` lifecycle semantics are documented and manually verified, but one-vs-two seed spawn distribution weighting remains unresolved in available source notes | Keep lifecycle behavior canonical and track weighting follow-up in `Simulation/CONFIDENCE_REVIEW.md` until an authoritative distribution source is confirmed |
| Mastery source metadata completeness (`Masteries/*.json`) | `0/2` mastery JSON files have null/empty `sources`; `0/2` have source entries missing `accessed` | Mastery provenance/accessed normalization is complete; keep this as a no-regression guardrail |
| Runes Reforged split-structure coverage | `5/5` trees now have split `primary_runes.json` + `secondary_runes.json` files, plus `RunesReforged/RunesReforged.json` index and `StatShards/stat_shards.json` | Keep split tree structure as authoring baseline while maintaining flat-file compatibility (`Masteries/RunesReforged.json`) until runtime loader migration |

## Data Coverage Progress (2026-02-24)
- Note: historical bullets below retain per-wave baseline counts captured at the time each wave landed; use "Coverage At A Glance" and "Champion Corpus Parity Snapshot" for current totals.
- Completed:
  - added `Simulation/champion_behavior_verification_tracker.json` to track deeper manual champion behavior verification beyond source extraction (current baseline: `172/172` manual verified, policy defaulting to intended non-bug behavior)
  - completed champion fidelity normalization wave 75:
    - manually reviewed and normalized execution-semantics notes across `3` champions (`Teemo`, `Viego`, `Zyra`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `169/172` to `172/172` (full manual-verified corpus)
    - backfilled `description_source` on touched missing entries (`Viego` `Spectral Maw`)
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed champion fidelity normalization wave 74:
    - manually reviewed and normalized execution-semantics notes across `2` champions (`Ivern`, `Malphite`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `167/172` to `169/172`
    - backfilled `description_source` on touched missing entries (`Ivern` `Friend of the Forest`)
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed champion fidelity normalization wave 73:
    - manually reviewed and normalized execution-semantics notes across `2` champions (`Hwei`, `Illaoi`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `165/172` to `167/172`
    - backfilled `description_source` on touched missing entries (`Illaoi` `Tentacle Smash`)
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed champion fidelity normalization wave 72:
    - manually reviewed and normalized execution-semantics notes across `2` champions (`Chogath`, `Heimerdinger`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `163/172` to `165/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed full-corpus champion `description_source` backfill pass:
    - backfilled all remaining missing champion ability `description_source` entries from canonical ability `description` text where source strings were absent
    - raised champion ability `description_source` completeness from `784/860` to `860/860` (`0` missing across `0` champions)
    - moved `description_source` tracking from active backlog to no-regression guardrail
  - completed champion fidelity normalization wave 71:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Rakan`, `Rammus`, `RekSai`, `Renata`, `TwistedFate`, `Zilean`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `157/172` to `163/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed champion fidelity normalization wave 70:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Elise`, `Evelynn`, `Kalista`, `Khazix`, `Kindred`, `Mordekaiser`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `151/172` to `157/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed champion fidelity normalization wave 69:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Zeri`, `Tristana`, `Twitch`, `Syndra`, `Veigar`, `Senna`) with page-level champion ability citation provenance
    - raised manual verification tracker coverage from `145/172` to `151/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed champion fidelity normalization wave 68:
    - manually reviewed and normalized execution-semantics notes across `6` champions (`Ambessa`, `Annie`, `Ashe`, `Ezreal`, `Lucian`, `Yasuo`) with page-level champion ability citation provenance
    - normalized `3` non-primary truncation defects discovered during manual review (`Ambessa`, `Evelynn`, `Senna`) before closing the wave
    - raised manual verification tracker coverage from `139/172` to `145/172`
    - strict, cadence, and article-token truncation queues remained `0/0` after wave audit
  - completed post-wave article-token truncation cleanup sweep:
    - normalized `1` additional article-interposed timing fragment (`during the 0.` class) on `TahmKench` (`Abyssal Dive`) to complete source-backed delay semantics
    - documented tertiary article-token sweep requirements in standards/checklist and validated queue clear state (`0/0`)
  - backfilled `description_source` on touched manual-review abilities where missing (`Mordekaiser` `Realm of Death`, `Rammus` `Powerball`) and documented broader champion `description_source` backlog (`79/860` missing across `58` champions) for follow-up waves (wave-71 baseline)
  - completed secondary cadence-fragment cleanup sweep outside the strict primary regex queue:
    - normalized `5` additional periodic-tick truncation fragments (`every 0.` class) across `2` champions (`Nasus`, `Fiora`) using source-backed full cadence semantics
    - added page-level champion ability citation provenance for touched cadence entries (`Fury of the Sands`, `Grand Challenge`)
    - recorded cleanup scope in `Simulation/champion_behavior_verification_tracker.json` as manual verification wave 67 (`Nasus`, `Fiora`)
    - kept both strict primary and secondary cadence truncation queues at `0/0` after post-edit audits
  - completed champion fidelity normalization wave 66:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Camille`, `Fiddlesticks`, `Jax`, `Rumble`, `Yorick`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - manual verification tracker coverage remained `139/172` (already-manual-verified champion set touched)
    - reduced string+array-aware strict fragment queue from `6/5` to `0/0` (queue cleared)
  - completed champion fidelity normalization wave 65:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Nautilus`, `Olaf`, `Swain`, `XinZhao`, `Nasus`) with page-level champion ability citation provenance
    - normalized `13` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - manual verification tracker coverage remained `139/172` (already-manual-verified champion set touched)
    - reduced string+array-aware strict fragment queue from `19/10` to `6/5`
  - completed champion fidelity normalization wave 64:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Zaahen`, `Jhin`, `Yone`, `Darius`, `Ekko`) with page-level champion ability citation provenance
    - normalized `15` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `136/172` to `139/172`
    - reduced string+array-aware strict fragment queue from `34/15` to `19/10`
  - completed champion fidelity normalization wave 63:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Caitlyn`, `Poppy`, `Quinn`, `Shen`, `Tryndamere`) with page-level champion ability citation provenance
    - normalized `5` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `131/172` to `136/172`
    - reduced string+array-aware strict fragment queue from `39/20` to `34/15`
  - standardized strict-fragment queue auditing to explicit `context_notes` string+array paths before wave 55 tracking, producing a reconciled wave-55 baseline of `121/68` for follow-on cleanup deltas
  - completed champion fidelity normalization wave 62:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Ryze`, `Sett`, `Sivir`, `TahmKench`, `Thresh`, `Xayah`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `125/172` to `131/172`
    - reduced string+array-aware strict fragment queue from `46/26` to `40/20`
  - completed champion fidelity normalization wave 61:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Graves`, `Hecarim`, `JarvanIV`, `Kassadin`, `Kennen`, `Pyke`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `119/172` to `125/172`
    - reduced string+array-aware strict fragment queue from `52/32` to `46/26`
  - completed champion fidelity normalization wave 60:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`LeeSin`, `Leona`, `Lulu`, `Brand`, `Braum`, `Diana`) with page-level champion ability citation provenance
    - normalized `6` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `113/172` to `119/172`
    - reduced string+array-aware strict fragment queue from `58/38` to `52/32`
  - completed champion fidelity normalization wave 59:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Akali`, `Alistar`, `Amumu`, `Yunara`, `Kaisa`, `Kayn`) with page-level champion ability citation provenance
    - normalized `18` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `107/172` to `113/172`
    - reduced string+array-aware strict fragment queue from `68/44` to `58/38`
  - completed champion fidelity normalization wave 58:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Jayce`, `Janna`, `Gragas`, `Gangplank`, `Blitzcrank`, `Azir`) with page-level champion ability citation provenance
    - normalized `12` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `101/172` to `107/172`
    - reduced string+array-aware strict fragment queue from `80/50` to `68/44`
  - completed champion fidelity normalization wave 57:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Pantheon`, `Ornn`, `Nidalee`, `Nami`, `MissFortune`, `Karma`) with page-level champion ability citation provenance
    - normalized `12` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `95/172` to `101/172`
    - reduced string+array-aware strict fragment queue from `92/56` to `80/50`
  - completed champion fidelity normalization wave 56:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Vex`, `Taric`, `Sylas`, `Rengar`, `Renekton`, `Qiyana`) with page-level champion ability citation provenance
    - normalized `12` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `89/172` to `95/172`
    - reduced string+array-aware strict fragment queue from `104/62` to `92/56`
  - completed champion fidelity normalization wave 55:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Zac`, `Viktor`, `Skarner`, `Rell`, `Orianna`, `Zoe`) with page-level champion ability citation provenance
    - normalized `17` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `83/172` to `89/172`
    - reduced string+array-aware strict fragment queue from `121/68` to `104/62`
  - completed champion fidelity normalization wave 54:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Belveth`, `Cassiopeia`, `Draven`, `Gwen`, `Kayle`, `KogMaw`) with page-level champion ability citation provenance
    - normalized `18` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `77/172` to `83/172`
    - reduced string+array-aware strict fragment queue from `147/76` to `129/70`
  - completed champion fidelity normalization wave 53:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Jinx`, `Anivia`, `Aatrox`, `Fizz`, `Ahri`, `Bard`) with page-level champion ability citation provenance
    - normalized `22` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `71/172` to `77/172`
    - reduced string+array-aware strict fragment queue from `169/82` to `147/76`
  - completed champion fidelity normalization wave 52:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Neeko`, `Milio`, `MasterYi`, `Malzahar`, `Karthus`, `KSante`) with page-level champion ability citation provenance
    - normalized `24` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `65/172` to `71/172`
    - reduced string+array-aware strict fragment queue from `193/88` to `169/82`
  - completed champion fidelity normalization wave 51:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Galio`, `Kled`, `Soraka`, `Shyvana`, `Seraphine`, `Samira`) with page-level champion ability citation provenance
    - normalized `26` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `59/172` to `65/172`
    - reduced string+array-aware strict fragment queue from `219/94` to `193/88`
  - completed champion fidelity normalization wave 50:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Talon`, `Smolder`, `Sejuani`, `Nunu`, `Nilah`, `Maokai`) with page-level champion ability citation provenance
    - normalized `30` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `53/172` to `59/172`
    - reduced string+array-aware strict fragment queue from `249/100` to `219/94`
  - completed champion fidelity normalization wave 49:
    - manually reviewed and normalized timing-semantics notes across `6` champions (`Mel`, `Katarina`, `Corki`, `Velkoz`, `Singed`, `Ziggs`) with page-level champion ability citation provenance
    - normalized `33` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - corrected tracker-key integrity drift by adding missing `Lissandra` to `manual_behavior_verified_champion_keys` so key-list totals reconcile with tracker totals
    - raised manual verification tracker coverage from `47/172` to `53/172`
    - reduced string+array-aware strict fragment queue from `282/106` to `249/100`
  - completed champion fidelity normalization wave 48:
    - manually reviewed and normalized timing-semantics notes across `5` champions (`Udyr`, `Sion`, `Lissandra`, `AurelionSol`, `Naafiri`) with page-level champion ability citation provenance
    - normalized `29` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `42/172` to `47/172`
    - reduced string+array-aware strict fragment queue from `309/108` to `280/107`
  - completed champion fidelity normalization wave 47:
    - manually reviewed and normalized timing-fragment execution notes across `18` champions (`Akshan`, `Aurora`, `Briar`, `Gnar`, `Irelia`, `Leblanc`, `Lillia`, `Lux`, `MonkeyKing`, `Nocturne`, `Riven`, `Shaco`, `Taliyah`, `Urgot`, `Varus`, `Xerath`, `Yuumi`, `Zed`)
    - normalized `51` strict fragment candidates in touched champion ability/effect `context_notes` to source-backed complete timing semantics
    - raised manual verification tracker coverage from `25/172` to `42/172`
    - corrected truncation-audit implementation to handle both string and array `context_notes`, then re-baselined the corrected-method queue from `360/123` to `309/108` after wave cleanup
  - completed champion fidelity normalization wave 44:
    - fixed remaining active-ability execution gaps (`4` -> `0`) by adding source-backed execution metadata for `Jax` (`Q`), `Leblanc` (`R`), and `Aphelios` (`Q`, `E`)
    - added missing champion ability context notes for `Vladimir` (`Q`, `E`, `R`), `Sona` (`Q`), `DrMundo` (`Q`, `R`), and execution-semantics refinement for `Olaf` (`E`) with attack-cadence-coupled timing notes
    - closed passive context-note backlog on canonical authored champions (`DrMundo`, `Morgana`, `Sona`, `Vayne`, `Vladimir`, `Warwick`) and reached full-corpus context-note coverage (`860/860`)
    - validated full-corpus champion quality no-regression metrics: parity `172/172`, active execution `682/682`, champion sources/accessed completeness `0/173` gaps
  - completed champion fidelity normalization wave 45:
    - manually reviewed and refined attack-cadence-coupled execution semantics on `Camille` (`Q`), `Ekko` (`E`), `Trundle` (`Q`), `Volibear` (`W`), `Vi` (`E`), and `Yorick` (`Q`)
    - fixed targeted truncated context-note fragments in touched abilities (for example `Trundle` `Q` and `Ekko` `E`) while preserving canonical execution metadata
    - added page-level ability verification provenance on all six touched champions and raised manual verification tracker coverage from `15/172` to `21/172`
  - completed champion fidelity normalization wave 46:
    - manually reviewed and refined execution-semantics notes for `Fiddlesticks` (`Q`, `W`, `E`), `Rumble` (`Q`, `R`), `Swain` (`Q`, `R`), and `Nautilus` (`E`, `R`)
    - normalized candidate truncated timing fragments on all touched abilities and added page-level ability provenance for each touched champion
    - raised manual verification tracker coverage from `21/172` to `25/172`
  - tightened truncation-audit criteria to strict fragment detection (requires trailing integer-dot tokens, avoids valid-decimal false positives); this baseline was later superseded by string+array-aware auditing in wave 47
  - backfilled `sources` and normalized review date to `2026-02-23` for all `9` modeled runtime item files
  - backfilled `sources` for `8` high-priority preset items (`Thornmail`, `Spirit Visage`, `Blackfire Torch`, `Stormsurge`, `Shadowflame`, `Rylai's Crystal Scepter`, `Lich Bane`, `Stridebreaker`)
  - backfilled `sources` for `23` additional A/B/C item files with `effects_structured`
  - backfilled `sources` for `30` additional legal-URF low-confidence unmodeled items (`Solstice Sleigh` through `Essence Reaver` batch)
  - backfilled `sources` for the remaining `51` legal-URF unmodeled effect items that were still unsourced
  - reduced item files with `effects_structured` and `sources: null` from `226` to `0`
  - reduced legal URF unmodeled effect items missing sources from `81` to `0`
  - standardized Tier-2 citation coverage across all structured item files (`243/243`)
  - refined structured behavior semantics for modeled runtime items (`Guinsoo's Rageblade`, `Heartsteel`, `Kraken Slayer`, `Luden's Echo`, `Protoplasm Harness`) after entity-intent review
  - reduced modeled runtime files with minimum `parse_confidence <= 0.60` from `5` to `0`
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `95` to `85`
  - normalized all remaining non-ISO item review metadata to ISO format (`23` -> `0`)
  - filled missing review metadata for `Bloodthirster`
  - deduplicated `Characters/Vladimir.json` source list
  - reduced runes with null/empty `stat` in `stat_modifier` effects from `21` to `0`
  - kept preset-rune null-stat set at `0` (none of current preset runes use null `stat_modifier`)
  - restored parser-compatible Protoplasm Lifeline threshold condition token (`health_below_30_percent`) after detecting a data-only compatibility regression
  - refined high-impact preset item data for `Stridebreaker`, `Warmog's Armor`, `Titanic Hydra`, `Rabadon's Deathcap`, and `Phantom Dancer` with entity-intent notes and page-level citations
  - increased page-level item citation depth from `17/243` to `22/243`
  - refined modeled runtime item data for `Heartsteel`, `Kraken Slayer`, and `Liandry's Torment`, removing modeled-borderline confidence entries (`3` -> `0`)
  - refined `Stormsurge` data semantics and raised its minimum confidence from `0.60` to `0.70`
  - normalized `Zhonya's Hourglass` modeled confidence from missing/null to explicit numeric confidence
  - refined preset-borderline item data for `Lich Bane`, `Stridebreaker`, and `Titanic Hydra`, raising all three from `0.65` minimum confidence to `>=0.70`
  - refined low-confidence legal URF item data for `Eclipse` and `Rod of Ages`, reducing backlog from `87` to `85`
  - increased page-level item citation depth from `22/243` to `24/243` with new citations for `Eclipse` and `Rod of Ages`
  - completed legal URF low-confidence citation/semantics wave 1 (`Malignance`, `Terminus`, `Sundered Sky`, `Statikk Shiv`, `Fiendhunter Bolts`) and raised each to `>= 0.65` minimum confidence
  - completed legal URF low-confidence citation/semantics wave 2 (`Essence Reaver`, `Iceborn Gauntlet`, `Jak'Sho, The Protean`, `Runaan's Hurricane`, `Sunfire Aegis`) and raised each to `>= 0.65` minimum confidence
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `85` to `75`
  - increased page-level item citation depth from `24/243` to `34/243`
  - reduced legal URF unmodeled low-confidence/no-page-citation queue from `28` to `18`
  - completed legal URF low-confidence citation/semantics wave 3 (`Solstice Sleigh`, `Bloodletter's Curse`, `Dawncore`, `Echoes of Helia`, `Force of Nature`), raising four of five to `>= 0.65` minimum confidence
  - added page-level citations and entity-intent context notes for all wave-3 items
  - kept conservative confidence on `Solstice Sleigh` support-income diminishing-gold semantics and documented shared-rule dependency for later data expansion
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `75` to `71`
  - increased page-level item citation depth from `34/243` to `39/243`
  - reduced legal URF unmodeled low-confidence/no-page-citation queue from `18` to `13`
  - completed legal URF low-confidence citation/semantics wave 4 (`Bloodsong`, `Dream Maker`, `Hexoptics C44`, `Hextech Rocketbelt`, `Hollow Radiance`), raising three of five to `>= 0.65` minimum confidence
  - added page-level citations and entity-intent context notes for all wave-4 items
  - kept conservative confidence on support-income diminishing-gold effects for `Bloodsong` and `Dream Maker` and tracked shared-rule dependency with `Solstice Sleigh`
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `71` to `68`
  - increased page-level item citation depth from `39/243` to `44/243`
  - reduced legal URF unmodeled low-confidence/no-page-citation queue from `13` to `8`
  - completed legal URF low-confidence citation/semantics wave 5 (`Horizon Focus`, `Hullbreaker`, `Knight's Vow`, `Mejai's Soulstealer`, `Opportunity`), raising four of five to `>= 0.68` minimum confidence and one (`Hullbreaker`) to `>= 0.65`
  - added page-level citations and entity-intent context notes for all wave-5 items
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `68` to `63`
  - increased page-level item citation depth from `44/243` to `49/243`
  - reduced legal URF unmodeled low-confidence/no-page-citation queue from `8` to `3`
  - completed legal URF low-confidence citation/semantics wave 6 (`Overlord's Bloodmail`, `Ravenous Hydra`, `Redemption`), raising all three to `>= 0.65` minimum confidence
  - added page-level citations and entity-intent context notes for all wave-6 items
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `63` to `60`
  - increased page-level item citation depth from `49/243` to `52/243`
  - reduced legal URF unmodeled low-confidence/no-page-citation queue from `3` to `0` (cleared)
  - completed legal URF low-confidence support-income precision wave 7 (`Solstice Sleigh`, `Bloodsong`, `Dream Maker`) with page-verified pre/post-5-minute diminishing-gold formulas and explicit shared-rule-table dependency notes
  - raised all three wave-7 support-income entries from conservative low confidence (`0.50/0.60/0.60`) to `0.68` after manual formula verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `60` to `57`
  - documented a new follow-up quality gap: support-quest sibling item consistency for diminishing-gold rule schema/confidence representation
  - completed support-quest sibling harmonization wave 8 (`World Atlas`, `Runic Compass`, `Bounty of Worlds`, `Celestial Opposition`, `Zaz'Zak's Realmspike`) for diminishing-gold rule schema, confidence, and page-level provenance
  - raised page-level item citation depth from `52/243` to `57/243`
  - closed sibling support-income schema/confidence harmonization backlog for the support-quest family
  - documented a new follow-up quality gap: support-quest sell-state representation discrepancy (`World Atlas` and `Runic Compass`)
  - cleared preset item confidence-borderline set (`minimum parse_confidence <= 0.65`) from `3` to `0`
  - completed consumable-and-manaflow fidelity wave 9 (`Health Potion`, `Refillable Potion`, `Tear of the Goddess`, `Manamune`) with page-verified activation/timing semantics, improved structured precision, and updated page-level citations
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `57` to `53`
  - increased page-level item citation depth from `57/243` to `61/243`
  - completed Tear-line sibling sell-value corrections (`Archangel's Staff`, `Muramana`, `Winter's Approach`, `Fimbulwinter`) aligned to Data Dragon `16.3.1`
  - completed source-and-economy reconciliation wave 10 across item data:
    - migrated legacy CommunityDragon item dataset citations to the current endpoint (`global/default/v1/items.json`) for all structured item files (`235/243` -> `0/243`)
    - reconciled item sell values to Data Dragon `16.3.1` for all checked items (`209/238` mismatches -> `0/238`)
    - manually spot-checked representative starter, legendary, support-quest, and map-specific items after bulk normalization
  - documented a lower-priority provenance expansion opportunity at the time: `79` non-structured item files with null/empty `sources`
  - completed support sell-state policy resolution wave 11:
    - set quest-stage support item sell values to page-verified No Sell behavior (`World Atlas`, `Runic Compass`, `Bounty of Worlds`: `sell = 0`)
    - documented intentional Tier-1 dataset discrepancy notes on each affected file (`Data Dragon`/`CommunityDragon` still report `sell = 160`)
    - reclassified sell-value mismatch tracking as explicit intentional overrides (`3/238`), with no unresolved accidental sell mismatches
  - completed rune low-confidence note normalization wave 11:
    - normalized low-confidence narrative effects across `17` runes to condition-note taxonomy with clearer trigger/owner semantics
    - reduced rune low-confidence backlog (`parse_confidence <= 0.60`) from `17` to `0`
  - refined `Runic Compass` Shared Riches execution semantics with higher-confidence target/range/timing-gate representation, reducing item low-confidence backlog (`parse_confidence < 0.65`) from `53` to `52`
  - completed item execution-semantics and citation wave 12 (`Control Ward`, `Doran's Shield`, `Everfrost`, `Fimbulwinter`, `Demonic Embrace`) with manual behavior review and page-level sources
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `52` to `47`
  - increased page-level item citation depth from `61/243` to `66/243`
  - identified a new provenance metadata gap: `203/243` structured item files currently have at least one source entry missing `accessed` date metadata
  - completed item execution-semantics and provenance wave 13 (`Galeforce`, `Gustwalker Hatchling`, `Mosstomper Seedling`, `Scorchclaw Pup`, `Talisman of Ascension`) with page-level behavior verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `47` to `42`
  - increased page-level item citation depth from `66/243` to `71/243`
  - normalized structured-item source metadata completeness (`sources[].accessed`) from `203/243` files missing entries to `0/243`
  - completed item execution-semantics and alignment wave 14 (`Fated Ashes`, `Hellfire Hatchet`, `Hamstringer`, `Sanguine Gift`, `Spectral Cutlass`) with manual behavior review and page-level verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `42` to `37`
  - increased page-level item citation depth from `71/243` to `77/243`
  - corrected Tier-1 drift on `Spectral Cutlass` (stats, active timing/cooldown semantics, and total price) and reconciled `Redemption` total price to Data Dragon `16.3.1`
  - normalized legacy item stat key usage from `30` files using `stats.magicResistance` to `0` by converting all to loader-canonical `stats.magicResist`
  - reconciled `The Golden Spatula` base stat block and shop pricing to Tier-1 `16.3.1` values and added patch-history-aware page citation
  - identified a remaining cross-version identity exception: `Zephyr` and `Gunmetal Greaves` currently share ID `3172`, creating one intentional Tier-1 total-price mismatch to be resolved by explicit legacy-ID policy
  - completed item execution-semantics and citation wave 15 (`Eleisa's Miracle`, `Chainlaced Crushers`, `Cloak of Starry Night`, `Lightning Rod`, `Reverberation`, `Runecarver`) with manual behavior review and page-level verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `37` to `31`
  - increased page-level item citation depth from `77/243` to `83/243`
  - reduced low-confidence/no-page-citation queue from `36` to `30`
  - corrected crowd-control trigger fidelity in `Reverberation` structured effects to include grounding in the Rumble trigger set
  - completed item execution-semantics and citation wave 16 (`Crystalline Overgrowth`, `Overcharged`, `Kinkou Jitte`, `Puppeteer`, `Jarvan I's`) with manual behavior review and page-level/official verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `31` to `26`
  - increased page-level item citation depth from `83/243` to `88/243`
  - reduced low-confidence/no-page-citation queue from `30` to `25`
  - documented mode-variance semantics on `Overcharged` (Clash vs Swiftplay Sudden Death timing/value differences) as a policy follow-up
  - completed item execution-semantics and citation wave 17 (`Demon King's Crown`, `Detonation Orb`, `Diamond-Tipped Spear`, `Reaper's Toll`, `Sword of the Divine`) with manual behavior review and official/page-level verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `26` to `21`
  - increased page-level item citation depth from `88/243` to `93/243`
  - reduced low-confidence/no-page-citation queue from `25` to `20`
  - documented a new lifecycle-marker follow-up: standardize retired/replaced item exclusion metadata policy beyond current `Zephyr` usage
  - completed item execution-semantics and citation wave 18 (`Pyromancer's Cloak`, `Crimson Lucidity`, `Regicide`, `Rite of Ruin`, `Dragonheart`, `Fire at Will`) with manual behavior review and page-level verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `21` to `15`
  - increased page-level item citation depth from `93/243` to `99/243`
  - reduced low-confidence/no-page-citation queue from `20` to `14`
  - documented Dragonheart acquisition-round soul-grant timing as a follow-up fidelity gap (tracked but not yet encoded in structured effects)
  - completed item execution-semantics and citation wave 19 (`Diadem of Songs`, `Sword of Blossoming Dawn`, `Lifeline`, `Hexbolt Companion`, `Fulmination`) with manual behavior review and page-level verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `15` to `10`
  - increased page-level item citation depth from `99/243` to `104/243`
  - reduced low-confidence/no-page-citation queue from `14` to `9`
  - documented Lifeline displacement/channel/crowd-control special-interaction semantics as a follow-up fidelity gap
  - completed item execution-semantics and citation wave 20 (`Flesheater`, `Force of Entropy`, `Gambler's Blade`, `Guardian's Dirk`, `Gusto`, `Hemomancer's Helm`, `Innervating Locket`, `Reality Fracture`, `Scarecrow Effigy`) with manual behavior review and page-level verification
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `10` to `1`
  - increased page-level item citation depth from `104/243` to `113/243`
  - reduced low-confidence/no-page-citation queue from `9` to `0`
  - documented remaining low-confidence concentration on `The Golden Spatula` as a dedicated semantic-normalization follow-up
  - completed item execution-semantics and citation wave 21 (`The Golden Spatula`, `Abyssal Mask`, `Ardent Censer`, `Black Cleaver`, `Death's Dance`, `Morellonomicon`) with page-level/module-level verification where relevant
  - reduced item low-confidence backlog (`parse_confidence < 0.65`) from `1` to `0`
  - increased page-level item citation depth from `113/243` to `118/243`
  - reduced legal URF unmodeled no-page-citation queue from `58/103` to `53/103`
  - documented Golden Spatula mode-scope/source-drift semantics as an explicit follow-up fidelity area (confidence floor now cleared)
  - completed item execution-semantics and citation wave 22 (`Trinity Force`, `Muramana`, `Sterak's Gage`, `The Collector`, `Wit's End`, `Zeke's Convergence`) with manual behavior review and page-level verification
  - corrected `Zeke's Convergence` Frostfire Tempest tick-value semantics from incorrect 30-per-tick placeholder math to verified 7.5-per-0.25s cadence (150 total over storm duration) and champion-only target scope
  - expanded `Muramana` Shock structured behavior to include champion-ability damage branch (4% melee / 3% ranged max mana) with cast-instance/per-target limiter and proc-damage exclusion semantics
  - increased page-level item citation depth from `118/243` to `124/243`
  - reduced legal URF unmodeled no-page-citation queue from `53/103` to `47/103`
  - completed item execution-semantics and citation wave 23 (`Chempunk Chainsword`, `Cosmic Drive`, `Dead Man's Plate`, `Locket of the Iron Solari`, `Maw of Malmortius`, `Rapid Firecannon`) with manual behavior review and page-level verification
  - corrected `Maw of Malmortius` passive omnivamp metadata from 30% to page-verified 10% and tightened Lifeline threshold semantics notes
  - added `Locket of the Iron Solari` Devotion cooldown-start semantics and refreshed `Rapid Firecannon` Energize generation/structure-interaction behavior notes
  - increased page-level item citation depth from `124/243` to `130/243`
  - reduced legal URF unmodeled no-page-citation queue from `47/103` to `41/103`
  - documented intended-behavior-first policy for known bug notes, with bug-divergence behavior kept out of canonical data
  - completed item execution-semantics and citation wave 24 (`Cryptbloom`, `Dusk and Dawn`, `Edge of Night`, `Hubris`, `Mikael's Blessing`, `Serpent's Fang`, `Spear of Shojin`) with manual behavior review and page-level verification
  - corrected `Mikael's Blessing` active structured metadata with page-verified cooldown (`120s`) and cast range (`650`) for both cleanse/heal branches
  - added `Spear of Shojin` Focused Will per-cast-instance stack-throttle metadata (one stack per second) and refreshed execution-model notes across all seven wave-24 items
  - increased page-level item citation depth from `130/243` to `137/243`
  - reduced legal URF unmodeled no-page-citation queue from `41/103` to `34/103`
  - identified active-cast cooldown/range completeness as a recurring data-audit follow-up
  - recalibrated legal-URF denominator tracking to runtime-filter parity (`default_item_pool` + runtime effect-payload detection): `113` pool items, `111` with effect payload, `102` unmodeled
  - completed item execution-semantics and citation wave 25 (`Bastionbreaker`, `Endless Hunger`, `Experimental Hexplate`, `Frozen Heart`, `Hextech Gunblade`, `Immortal Shieldbow`) with manual behavior review and page-level verification
  - corrected active metadata completeness for `Hextech Gunblade` by adding page-verified cooldown (`60s`) and cast range (`700`) semantics
  - split `Experimental Hexplate` Overdrive into explicit attack-speed and movement-speed branches with cooldown-start-on-cast timing semantics
  - increased page-level item citation depth from `137/243` to `143/243`
  - reduced runtime-filtered legal URF unmodeled no-page-citation queue from `33/102` to `27/102`
  - completed item execution-semantics and citation wave 26 (`Axiom Arc`, `Banshee's Veil`, `Imperial Mandate`, `Randuin's Omen`, `Shurelya's Battlesong`, `Youmuu's Ghostblade`) with manual behavior review and page-level verification
  - corrected trigger-gating fidelity for `Axiom Arc` (takedown-driven ultimate cooldown refund) and `Imperial Mandate` (mark application/consume shared-cooldown semantics)
  - corrected targeted active metadata completeness for `Randuin's Omen` (`90s`, `500 radius`), `Shurelya's Battlesong` (`75s`, `1000 range`), and `Youmuu's Ghostblade` (`45s`)
  - increased page-level item citation depth from `143/243` to `149/243`
  - reduced runtime-filtered legal URF unmodeled no-page-citation queue from `27/102` to `21/102`
  - completed item execution-semantics and citation wave 27 (`Mercurial Scimitar`, `Navori Flickerblade`, `Profane Hydra`, `Riftmaker`, `Unending Despair`, `Voltaic Cyclosword`) with manual behavior review and page-level verification
  - corrected active cooldown completeness for `Mercurial Scimitar` (`90s`) and `Profane Hydra` (`10s`) plus execution-timing context notes across all six items
  - normalized `Navori Flickerblade` crit stat key to loader-canonical `stats.critChance` and logged remaining `stats.criticalStrikeChance` backlog (`23` files) for follow-up
  - increased page-level item citation depth from `149/243` to `155/243`
  - reduced runtime-filtered legal URF unmodeled no-page-citation queue from `21/102` to `15/102`
  - completed dedicated crit-stat key migration pass: normalized remaining `22` files from legacy `stats.criticalStrikeChance` to loader-canonical `stats.critChance` (with earlier opportunistic migration on `Navori Flickerblade`) and cleared crit-key backlog (`0/322` remaining)
  - completed item execution-semantics and citation wave 28 (`Kaenic Rookern`, `Lord Dominik's Regards`, `Moonstone Renewer`, `Mortal Reminder`, `Nashor's Tooth`) with manual behavior review and page-level verification
  - increased page-level item citation depth from `155/243` to `160/243`
  - reduced runtime-filtered legal URF unmodeled no-page-citation queue from `15/102` to `10/102`
  - documented explicit runtime code follow-up need for `Moonstone Renewer` ally-chain/fallback target-resolution behavior to preserve simulation fidelity
  - completed item execution-semantics and citation wave 29 (`Actualizer`, `Archangel's Staff`, `Bandlepipes`, `Serylda's Grudge`, `Staff of Flowing Water`, `Trailblazer`, `Umbral Glaive`, `Whispering Circlet`, `Winter's Approach`, `Yun Tal Wildarrows`) with manual behavior review and page-level verification
  - increased page-level item citation depth from `160/243` to `170/243`
  - reduced runtime-filtered legal URF unmodeled no-page-citation queue from `10/102` to `0/102` (queue cleared)
  - documented explicit deferred runtime follow-up scope for mode-aware resource branches, visibility-gated windows, movement-stack trail engines, attack-type split durations/values, and on-attack cooldown-reduction semantics
  - completed item execution-semantics and citation wave 30 (`Wordless Promise`, `Anathema's Chains`, `Seraph's Embrace`, `Stormrazor`, `Perplexity`, `Atma's Reckoning`) with manual behavior review and page-level verification
  - corrected `Anathema's Chains` active semantics by separating true active cooldown (`90s`) from in-combat cast-lockout gating (`15s`) and documented global-targeting/no-cast-time behavior notes
  - finalized prior uncertainty handling for `Bandlepipes`, `Umbral Glaive`, and `Trailblazer` using intended-behavior assumptions in data notes, while keeping explicit runtime follow-up rows for capability gaps
  - increased page-level item citation depth from `170/243` to `176/243`
  - reduced broader structured no-page citation queue from `73` to `67` items while maintaining legal URF unmodeled no-page queue at `0/102`
  - documented rotating-mode stat/cost package drift notes on `Perplexity` and `Atma's Reckoning` and tracked policy follow-up
  - completed item execution-semantics and citation wave 31 (`Doran's Ring`, `Divine Sunderer`, `Goredrinker`, `Prowler's Claw`, `Gargoyle Stoneplate`, `Duskblade of Draktharr`) with manual behavior review and page-level verification
  - corrected active metadata completeness across wave-31 actives and Spellblade timing (`Divine Sunderer` Spellblade ICD timing, `Goredrinker` 15s cooldown + attack-windup cast semantics, `Prowler's Claw` 25s cooldown/500 range/0.15s cast time, `Gargoyle Stoneplate` 30s cooldown/no-cast-time behavior)
  - added explicit mode-availability context notes for distributed Arena-scoped identities and tracked dataset-vs-page availability/map-flag policy follow-up
  - increased page-level item citation depth from `176/243` to `182/243`
  - reduced broader structured no-page citation queue from `67` to `61` items while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and economy-reconciliation wave 32 (`Dark Seal`, `Tiamat`, `Night Harvester`, `Radiant Virtue`, `Moonflair Spellblade`, plus distributed economy updates on `Divine Sunderer`, `Goredrinker`, `Prowler's Claw`, `Gargoyle Stoneplate`, `Duskblade of Draktharr`, and `Everfrost`)
  - added page-level League Wiki citations and execution-semantics context notes for `Dark Seal`, `Tiamat`, `Night Harvester`, `Radiant Virtue`, and `Moonflair Spellblade`
  - expanded distributed/prismatic economy representation (`shop.prices`) from `1/57` to `10/57` files and documented non-shop acquisition semantics on each touched item
  - increased page-level item citation depth from `182/243` to `187/243`
  - reduced broader structured no-page citation queue from `61/243` to `56/243` while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and citation wave 33 (`Bami's Cinder`, `Bramble Vest`, `Catalyst of Aeons`, `Hexdrinker`, `Hextech Alternator`) with manual behavior review and page-level verification
  - added explicit trigger-exclusion and interaction-edge semantics where source notes published them (for example landed-hit exclusions, spell-shield/proc-damage behavior, zero-damage proc eligibility, and threshold retrigger notes)
  - increased page-level item citation depth from `187/243` to `192/243`
  - reduced broader structured no-page citation queue from `56/243` to `51/243` while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and citation wave 34 (`Lost Chapter`, `Haunting Guise`, `Executioner's Calling`, `Seeker's Armguard`, `Warden's Mail`) with manual behavior review and page-level verification
  - corrected and expanded execution metadata for edge semantics and special-case behavior notes (for example shield-interaction on Grievous Wounds application, Time Stop single-use transform behavior, and Rock Solid source/cap nuances)
  - increased page-level item citation depth from `192/243` to `197/243`
  - reduced broader structured no-page citation queue from `51/243` to `46/243` while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and citation wave 35 (`Oblivion Orb`, `Phage`, `Sheen`, `Quicksilver Sash`, `Verdant Barrier`) with manual behavior review and page-level verification
  - normalized execution-edge semantics for Spellblade structure/plant trigger behavior, Quicksilver activation/cleanse constraints, and Annul shield lifecycle notes (including death/cooldown-restart interactions)
  - increased page-level item citation depth from `197/243` to `202/243`
  - reduced broader structured no-page citation queue from `46/243` to `41/243` while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and citation wave 36 (`Boots of Swiftness`, `Ionian Boots of Lucidity`, `Plated Steelcaps`, `Recurve Bow`, `Cull`) with manual behavior review and page-level verification
  - corrected `Cull` Reap completeness by adding on-hit sustain, minion-gold progression, and one-time completion payout semantics (including permanent disable branch notes)
  - aligned Quicksilver-family data fidelity by normalizing `Mercurial Scimitar` cleanse edge semantics with `Quicksilver Sash` (activation lockout, suppression/nearsight scope, no-cast-time/stealth behavior, cooldown-transfer notes)
  - added explicit `mode_overrides` branches on `Overcharged` for Swiftplay and URF Sudden Death timing/value divergence while keeping Clash baseline semantics at root
  - reconciled `Wordless Promise` active cooldown from tooltip-ambiguous baseline to page-verified `10s` Promise cooldown with team/target gating notes
  - increased page-level item citation depth from `202/243` to `207/243`
  - reduced broader structured no-page citation queue from `41/243` to `36/243` while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and citation wave 37 (`Armored Advance`, `Black Spear`, `Ghostcrawlers`, `Guardian's Amulet`, `Guardian's Horn`, `Guardian's Orb`, `Scout's Slingshot`, `Swiftmarch`, `Zephyr`) with manual behavior review and page-level verification
  - added mode-variant Arena overlays (`mode_overrides.ARENA`) for `Guardian's Horn` and `Guardian's Orb`, plus source-level reconciliation notes for cross-mode ID drift (`Zephyr`/`Gunmetal Greaves`) and economy divergence (`Ghostcrawlers` sell value)
  - increased page-level item citation depth from `207/243` to `216/243`
  - cleared higher-priority no-page citation queue (`9` -> `0`) and reduced broader structured no-page queue from `36/243` to `27/243` while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and citation wave 38 (`Arcane Sweeper (Trinket)`, `Farsight Alteration`, `Oracle Lens`, `Stealth Ward`, `Slightly Magical Boots`, `Crown of the Shattered Queen`, `Turbo Chemtank`) with manual behavior review and page-level verification
  - encoded trinket-specific execution semantics across wave-38 utility items (activation lockouts, charge/recharge scaling, level requirements, shared ward limits, reveal/detection windows, and player-visible ward reveal behavior)
  - expanded distributed/prismatic economy representation from `10/57` to `13/57` by adding explicit `shop.prices` on `Crown of the Shattered Queen`, `Turbo Chemtank`, and `Slightly Magical Boots` (with acquisition-scope reconciliation notes)
  - increased page-level item citation depth from `216/243` to `223/243` and reduced broader structured no-page queue from `27/243` to `20/243` while maintaining legal URF unmodeled no-page queue at `0/102`
  - completed data-first item execution-semantics and citation wave 39 (`Anti-Tower Socks`, `Base Turret Reinforced Armor (Turret Item)`, `Black Hole Gauntlet`, `Cruelty`, `Darksteel Talons`, `Death's Daughter`, `Decapitator`, `Empyrean Promise`, `Mirage Blade`, `Ohmwrecker (Turret Item)`, `Phreakish Gusto`, `Raise Morale`, `Reinforced Armor (Turret Item)`, `Shield of Molten Stone`, `Super Mech Armor`, `Super Mech Power Field`, `Twilight's Edge`, `Twin Mask`, `Warden's Eye`, `Wooglet's Witchcap`) with manual behavior review and page-level verification
  - cleared the broader structured no-page citation queue from `20/243` to `0/243` by adding page-level sources for all remaining queued items and adding parent-page fallback citations for redirect-backed pseudo-items
  - expanded distributed/prismatic economy representation from `13/57` to `23/57` by adding explicit `shop.prices` on ten distributed Arena items (including `Black Hole Gauntlet`, `Cruelty`, `Darksteel Talons`, `Decapitator`, `Empyrean Promise`, `Mirage Blade`, `Shield of Molten Stone`, `Twilight's Edge`, `Twin Mask`, and `Wooglet's Witchcap`)
  - corrected page-verified structured-value mismatches in existing covered data (`Darksteel Talons` melee armor ratio, `Twin Mask` transfer ratios, `Twilight's Edge` world AD/AP multipliers, `Empyrean Promise` active cooldown/dash speed, `Warden's Eye` reveal scope, `Wooglet's Witchcap` stasis cooldown, and `Black Hole Gauntlet` scaling branch)
  - completed distributed/prismatic economy rollout wave 40 across the remaining `34` distributed files by backfilling explicit `shop.prices` from Tier-1 item-ID reconciliation (`23/57` -> `57/57`)
  - backfilled source provenance for previously unsourced distributed utility/legacy entries (`Lucky Dice`, `Enhanced Lucky Dice`, `Poro-Snax`, `Total Biscuit of Everlasting Will`, `Your Cut`) and normalized sparse active-shape inconsistencies (`active: {}` -> `active: []` on the two legacy consumable/reward entries)
  - reduced non-structured item files with null/empty `sources` from `79` to `74` while maintaining structured-item provenance/page-depth guardrails at `243/243`
  - completed non-structured provenance completion wave 41 by backfilling sources for the remaining `72` canonical non-structured unsourced item files (`74` queue included `2` report artifacts), clearing canonical non-structured provenance to `0/77` unsourced
  - normalized empty-active shape inconsistencies across canonical queue outliers (`active: {}` -> `active: []` on `Eye of the Herald`, `Tunneler`, `Turret Plating`, `Vampiric Scepter`, `Void Staff`, `Winged Moonplate`, and `Zeal`)
  - validated queue coverage inputs against authoritative and local channels for all touched files (Data Dragon, CommunityDragon, local ingestion snapshot, and League Wiki page presence)
  - completed champion/mastery provenance and champion execution-normalization wave 42:
    - backfilled missing `sources[].accessed` metadata on generated champion source entries for `DrMundo`, `Morgana`, `Sona`, `Vayne`, `Vladimir`, and `Warwick`
    - added explicit provenance sources to `Characters/ChampionDefaults.json` and normalized mastery source metadata completeness on `Masteries/Season2016.json` (`sources[].accessed` backfill)
    - completed active-ability execution metadata coverage (`23/23`) by adding source-backed execution objects for `DrMundo` (`W`, `E`), `Morgana` (`W`, `E`), `Sona` (`W`, `E`), `Vayne` (`E`), `Vladimir` (`W`), and `Warwick` (`W`, `E`)
    - corrected `Vayne` `Silver Bolts` ability type from `Active` to `Passive` based on source targeting metadata
    - closed champion/mastery source-accessed backlog to `0` files in both domains (`Characters`: `0/7` missing, `Masteries`: `0/2` missing)
  - completed champion-parity planning + runes split-structure wave 43:
    - generated `Simulation/champion_data_coverage_inventory.json` with current parity snapshot against `From Online/champions/*.json` (`6/172` covered, `166` missing)
    - created split runes authoring structure under `Masteries/RunesReforged/`:
      - index file: `Masteries/RunesReforged/RunesReforged.json`
      - stat shards: `Masteries/RunesReforged/StatShards/stat_shards.json`
      - per-tree folders with both `primary_runes.json` and `secondary_runes.json` plus `tree.json` for `Domination`, `Inspiration`, `Precision`, `Resolve`, and `Sorcery`
    - preserved runtime compatibility by keeping legacy flat file `Masteries/RunesReforged.json` in place while split structure becomes explicit data-authoring baseline
- Deferred:
  - runtime/code expansion backlog remains deferred by design (data-first execution mode)

## Data-First Coverage Priorities (Next)
1. Execute targeted champion behavior-fidelity re-verification waves across the full canonical corpus (`172/172` manual verified baseline):
   - keep `Simulation/champion_data_coverage_inventory.json` as file-parity guardrail
   - keep `Simulation/champion_behavior_verification_tracker.json` current and reconciled (`manual_behavior_verified_champion_keys` vs tracker totals)
   - prioritize manually reviewed execution semantics for high-impact/touched champions and abilities
   - track unresolved behavior ambiguity in `Simulation/CONFIDENCE_REVIEW.md` and `Simulation/COVERAGE_GAPS.md`
2. Keep champion-file quality bar strict during fidelity waves:
   - canonical sections required (`base_stats`, `basic_attack`, `abilities`)
   - active ability `execution` required on every active ability
   - full provenance (`sources` + `sources[].accessed`) and in-game execution semantics notes for non-trivial abilities
   - maintain full ability context-note coverage at `860/860` and keep execution-semantics notes current on non-trivial ability edits
3. Maintain runes split-structure parity as no-regression baseline:
   - keep `Masteries/RunesReforged/RunesReforged.json` index + all `Trees/*/primary_runes.json` and `Trees/*/secondary_runes.json` synchronized with flat `Masteries/RunesReforged.json`
   - preserve `Masteries/RunesReforged/StatShards/stat_shards.json` parity with flat-file stat shard data
4. Maintain modeled runtime confidence floor (`>= 0.70`) and continue second-pass semantic refinement on complex proc/per-target effects.
5. Maintain item low-confidence backlog at `0` files (`parse_confidence < 0.65`) as a no-regression requirement.
6. Maintain page-level verification citation depth for complex item semantics as a no-regression guardrail (current League Wiki page coverage: `243/243`, remaining no-page queue: `0/243`).
7. Maintain legal URF unmodeled page-level citation depth at `0/102` as a no-regression guardrail.
8. Maintain rune low-confidence floor (`0` runes with `parse_confidence <= 0.60`) and continue condition-note taxonomy cleanup for remaining medium-confidence narrative effects.
9. Maintain support-quest sell-state exception policy and remaining shared-rule precision details:
   - keep intentional No Sell sell-value overrides documented for `World Atlas`, `Runic Compass`, and `Bounty of Worlds`
   - shared support-income runtime rule-table encoding for exact per-minute windowing/team-item dynamics
10. Maintain provenance and Tier-2 citation no-regression (structured item baseline: `243/243` sourced and Tier-2 cited).
11. Keep source URL-health and sell-value reconciliation as recurring no-regression audits on future item data edits.
12. Maintain canonical item stat-key normalization (`stats.magicResist`, `stats.critChance`) and block reintroduction of legacy stat keys (`stats.magicResistance`, `stats.criticalStrikeChance`).
13. Resolve cross-version ID/name drift policy for legacy item files (`Zephyr`/`Gunmetal Greaves` shared ID `3172`) so Tier-1 reconciliation audits can stay deterministic.
14. Maintain structured-item source metadata completeness (`sources[].accessed`) as a no-regression guardrail.
15. Maintain distributed/prismatic economy no-regression (`57/57` explicit `shop.prices`) while finalizing canonical policy for non-shop acquisition vs Tier-1 economy fields.
16. Define canonical distributed-item mode-overlay policy for dataset-vs-page availability drift (for example Arena map-flag divergence on distributed/prismatic IDs) using the documented `mode_overrides.<mode>` schema pattern.
17. Standardize narrative effect taxonomy and maintain `condition_note` usage for non-stat behavioral constraints.
18. Add recurring audits for modeled-item condition-token compatibility and numeric confidence completeness to prevent data-only parser/metrics regressions.
19. Maintain non-structured provenance no-regression on canonical item files (`0/77` unsourced) and maintain items-folder provenance/accessed completeness at `0/322` gaps (canonical files plus report artifacts).
20. Extend canonical mode-scoping policy for shared-ID item effects beyond `Overcharged` now that `mode_overrides` overlays are in use for Clash/Swiftplay/URF plus Arena map-difference divergences (`Guardian's Horn`, `Guardian's Orb`).
21. Standardize lifecycle-marker policy for retired/replaced item identities (required fields: `status`, `exclude_from_simulation`, `reason`, `replacement_item`, `replacement_id`) and enforce exclusion behavior guardrails.
22. Resolve Dragonheart acquisition-round stack semantics (immediate soul backfill table and phase timing) as explicit structured behavior if simulation fidelity requires purchase-round sensitivity.
23. Define canonical policy for champion-ability upgrade pseudo-items (for example `Fire at Will`) including source provenance shape and simulation-use expectations.
24. Keep intended-behavior-first handling for known bug notes; keep bug behavior out of canonical simulation data and document divergences as notes only.
25. Maintain champion provenance/accessed no-regression (`Characters`: `0/173` unsourced and `0/173` with missing `sources[].accessed`).
26. Maintain mastery provenance/accessed no-regression (`Masteries`: `0/2` unsourced and `0/2` with missing `sources[].accessed`).
27. Maintain champion active-ability execution metadata no-regression (`682/682` active abilities with non-empty `execution` objects).
28. Maintain champion manual behavior-verification tracker at full-corpus coverage (`172/172` current baseline) and run targeted re-verification waves when semantics change.
29. Keep champion context-note truncation queues at `0/0` (strict primary regex plus secondary cadence-fragment sweep plus tertiary article-token sweep) on every champion wave so truncation regressions are caught immediately.
30. Maintain champion ability `description_source` no-regression (`860/860` currently populated) and treat missing entries as blocking regressions.

### Remaining Broader No-Page Queue (`0`)
- None (queue cleared in wave 39; keep as a no-regression guardrail).

### Planned Data Passes (Documented Plan)
1. Trigger-Exclusion and Proc-Edge Normalization Plan:
   - phase 1: inventory remaining on-hit/proc items with missing explicit exclusion/edge semantics (for example shield interactions, miss/parry/blind exclusions, spell-shield handling, zero-damage trigger behavior).
   - phase 2: normalize those items to shared condition/modifier vocabulary and add page-level notes + `schema_notes.context_notes` execution impact.
   - phase 3: run no-regression audits on trigger-token consistency and confidence floors, then update coverage metrics and remaining queue.
2. Distributed/Prismatic Economy Rollout Plan:
   - phase 1 (completed): filled `shop.prices.total` and `shop.prices.sell` on all distributed/prismatic files where Tier-1 exposes explicit economy fields (`57/57` explicit).
   - phase 2 (in progress): keep acquisition-scope context notes and flag page-vs-dataset economy drift per item (Arena/anvil/non-shop path semantics).
   - phase 3 (pending): consolidate discrepancy classes into canonical policy outcomes in coverage docs (accepted Tier-1 canonical, intentional override, or unresolved follow-up) with explicit queue counts.
3. Policy-Resolution Plan (Data Documentation First):
   - resolve distributed availability/map-overlay policy (dataset map flags vs page-level mode scope) with canonical root-baseline plus `mode_overrides.<mode>` divergence encoding.
   - resolve legacy ID/name drift handling for deterministic Tier-1 reconciliation (for example `Zephyr`/`Gunmetal Greaves`) with standardized `lifecycle` metadata requirements.
   - keep policy decisions documented in `Simulation/COVERAGE_STANDARDS.md`, `Simulation/COVERAGE_CHECKLIST.md`, and this tracker in the same change when resolved.
4. Champion Fidelity Plan (Data-Only):
   - phase 1 (completed): close file-parity gap to `172/172` canonical champion files and keep `Simulation/champion_data_coverage_inventory.json` current as no-regression guardrail.
   - phase 2 (completed): ran manual execution-semantics review waves over generated champion files, prioritizing high-impact kits and attack-cadence-coupled abilities.
   - phase 3 (completed): standardized context-note depth for champion passives and closed passive-note backlog (`172/172` documented).
   - phase 4 (continuous): keep source denominator synchronized when new `From Online/champions` entries appear, then open targeted parity/fidelity deltas for new champions only.
   - phase 5 (completed): raised manual verification coverage in `Simulation/champion_behavior_verification_tracker.json` to full corpus (`172/172`) with wave-level reviewed champion/ability scope.
5. Runes Reforged Split-Structure Plan:
   - phase 1 (completed): create split index/stat-shards/tree files (`Masteries/RunesReforged/RunesReforged.json`, `StatShards/stat_shards.json`, `Trees/*/{tree,primary_runes,secondary_runes}.json`).
   - phase 2 (in progress): keep split files and flat compatibility file synchronized on every rune/mastery edit, with explicit parity audits in authoring checks.
   - phase 3 (pending runtime follow-up): migrate runtime/data loaders to consume split structure directly, then deprecate flat-file compatibility path in a controlled code phase.
6. Champion Context-Note Truncation Cleanup Plan:
   - phase 1 (completed): inventory strict-regex-detected truncated timing fragments with string+array-aware context-note normalization (post-wave-75 strict queue now `0` entries across `0` champion files; corrected-method pre-cleanup baseline `360/123`) and track as explicit gap closure.
   - phase 2 (completed): prioritized high-impact combat kits/attack-cadence abilities and normalized tracked truncated notes to full timing/unit semantics with page-level verification in waves 63-75, including secondary cadence (`every 0.`) and tertiary article-token (`during the 0.`) cleanup classes.
   - phase 3 (in progress): run full-corpus no-regression audits to keep strict primary, cadence, and article-token truncation queues at `0` on future champion edits.
7. Champion Ability `description_source` Backfill Plan:
   - phase 1 (completed): quantified backlog and backfilled touched-wave missing entries where encountered.
   - phase 2 (completed): ran dedicated full-corpus backfill to close champion ability `description_source` backlog (`860/860`, `0` missing).
   - phase 3 (in progress): run full-corpus no-regression audits to keep champion ability `description_source` completeness at `860/860` on future edits.

### Active Low-Confidence Item Queue (`0`)
- None currently (`parse_confidence < 0.65` queue cleared).

## Source Of Truth For Coverage Checks
- Champion script registries:
  - `Simulation/src/scripts/champions/controlled_champion.rs`
  - `Simulation/src/scripts/champions/mod.rs`
- Champion corpus parity inventory:
  - `Simulation/champion_data_coverage_inventory.json`
- Champion manual behavior-verification tracker:
  - `Simulation/champion_behavior_verification_tracker.json`
- Champion source denominator:
  - `From Online/champions/*.json`
- Runtime item/rune modeled sets:
  - `Simulation/src/scripts/coverage.rs`
  - `Simulation/src/scripts/runes/effects.rs`
  - `Simulation/src/scripts/runtime/loadout_runtime.rs`
- Runes Reforged canonical/split data:
  - `Masteries/RunesReforged.json` (runtime compatibility flat file)
  - `Masteries/RunesReforged/RunesReforged.json` (split index)
  - `Masteries/RunesReforged/Trees/*/primary_runes.json`
  - `Masteries/RunesReforged/Trees/*/secondary_runes.json`
  - `Masteries/RunesReforged/StatShards/stat_shards.json`
- Data loaders and legal pool filters:
  - `Simulation/src/data.rs`
- Quality-gate wiring:
  - `Simulation/src/scenario_runner.rs`

## Champion And Ability Coverage
### Champion Corpus Parity Snapshot
- Source denominator (`From Online/champions/*.json`): `172` champions
- Canonical `Characters/<Champion>.json` files: `172`
- Missing canonical champion files: `0`
- Coverage tracker: `Simulation/champion_data_coverage_inventory.json`

### Runtime-Scripted Champion Subset Snapshot
| Champion | Canonical Data Present | Controlled Script | Enemy Scripted Events | Scripted Ability Coverage Summary |
| --- | --- | --- | --- | --- |
| Doctor Mundo | Yes | No | Yes | `Infected Bonesaw` |
| Morgana | Yes | No | Yes | `Dark Binding`, `Soul Shackles`, `Soul Shackles Detonate` |
| Sona | Yes | No | Yes | `Crescendo` |
| Vayne | Yes | No | Yes | `Tumble` empower + periodic true-hit (`Silver Bolts` style) |
| Vladimir | Yes | Yes | No | Controlled champion `Q`, `W`, `E`, `R` loop + Vladimir passive stat hook |
| Warwick | Yes | No | Yes | `Infinite Duress` + passive on-hit profile |

### Champion Gaps
- Champion file parity is complete (`172/172`), manual-verification corpus coverage is complete (`172/172`), and champion ability `description_source` coverage is complete (`860/860`); highest-priority champion data follow-up is maintaining no-regression fidelity, hardening `description_source` provenance depth, and resolving documented confidence ambiguities.
- Controlled champion generic script support:
  - implemented only for `Vladimir` in current runtime
  - `171` data-complete champions remain unscripted for controlled runtime behavior
- Enemy script depth is intentionally partial for all scripted enemies.
- `Vladimir` has no enemy-scripted spell events yet.
- Slot/remap architecture foundations exist, but full actor-wide slot-agnostic remap/steal behavior is still incomplete.

## Item Effect Coverage
### Item Snapshot
- Item JSON files in repository: `322` (`320` canonical item definitions + `2` non-canonical report artifacts)
- Items loaded by runtime (`load_items`, excluding consumable/trinket ranks): `295`
- Legal URF legendary search pool: `113`
- Legal URF pool items with effect payload (passive text, active effect, or `effects_structured`): `111`
- Legal URF pool items with modeled runtime effect behavior: `9`
- Legal URF pool items with unmodeled runtime effect payload: `102`

### Modeled Runtime Item Effects In Legal URF Pool (`9`)
- `Blade of the Ruined King`
- `Guardian Angel`
- `Guinsoo's Rageblade`
- `Heartsteel`
- `Kraken Slayer`
- `Liandry's Torment`
- `Luden's Echo`
- `Protoplasm Harness`
- `Zhonya's Hourglass`

### Runtime Item Effect Registry Keys (`10`)
The runtime modeled registry contains `10` normalized keys:
- `bladeoftheruinedking`
- `guardianangel`
- `guinsoosrageblade`
- `heartsteel`
- `krakenslayer`
- `liandrystorment`
- `ludensecho`
- `ludenscompanion` (compatibility key; no corresponding current item file in this repository)
- `protoplasmharness`
- `zhonyashourglass`

### Unmodeled Runtime Item Effects In Legal URF Pool (`102`)
- `Abyssal Mask`
- `Actualizer`
- `Ardent Censer`
- `Axiom Arc`
- `Bandlepipes`
- `Banshee's Veil`
- `Bastionbreaker`
- `Black Cleaver`
- `Blackfire Torch`
- `Bloodletter's Curse`
- `Bloodsong`
- `Bloodthirster`
- `Celestial Opposition`
- `Chempunk Chainsword`
- `Cosmic Drive`
- `Cryptbloom`
- `Dawncore`
- `Dead Man's Plate`
- `Death's Dance`
- `Dream Maker`
- `Dusk and Dawn`
- `Echoes of Helia`
- `Eclipse`
- `Edge of Night`
- `Endless Hunger`
- `Essence Reaver`
- `Experimental Hexplate`
- `Fiendhunter Bolts`
- `Force of Nature`
- `Frozen Heart`
- `Hexoptics C44`
- `Hextech Gunblade`
- `Hextech Rocketbelt`
- `Hollow Radiance`
- `Horizon Focus`
- `Hubris`
- `Hullbreaker`
- `Iceborn Gauntlet`
- `Immortal Shieldbow`
- `Imperial Mandate`
- `Jak'Sho, The Protean`
- `Kaenic Rookern`
- `Knight's Vow`
- `Lich Bane`
- `Locket of the Iron Solari`
- `Lord Dominik's Regards`
- `Malignance`
- `Maw of Malmortius`
- `Mejai's Soulstealer`
- `Mercurial Scimitar`
- `Mikael's Blessing`
- `Moonstone Renewer`
- `Morellonomicon`
- `Mortal Reminder`
- `Muramana`
- `Nashor's Tooth`
- `Navori Flickerblade`
- `Opportunity`
- `Overlord's Bloodmail`
- `Phantom Dancer`
- `Profane Hydra`
- `Rabadon's Deathcap`
- `Randuin's Omen`
- `Rapid Firecannon`
- `Ravenous Hydra`
- `Redemption`
- `Riftmaker`
- `Rod of Ages`
- `Runaan's Hurricane`
- `Rylai's Crystal Scepter`
- `Seraph's Embrace`
- `Serpent's Fang`
- `Serylda's Grudge`
- `Shadowflame`
- `Shurelya's Battlesong`
- `Solstice Sleigh`
- `Spear of Shojin`
- `Spirit Visage`
- `Staff of Flowing Water`
- `Statikk Shiv`
- `Sterak's Gage`
- `Stormsurge`
- `Stridebreaker`
- `Sundered Sky`
- `Sunfire Aegis`
- `Terminus`
- `The Collector`
- `Thornmail`
- `Titanic Hydra`
- `Trailblazer`
- `Trinity Force`
- `Umbral Glaive`
- `Unending Despair`
- `Voltaic Cyclosword`
- `Warmog's Armor`
- `Whispering Circlet`
- `Winter's Approach`
- `Wit's End`
- `Youmuu's Ghostblade`
- `Yun Tal Wildarrows`
- `Zaz'Zak's Realmspike`
- `Zeke's Convergence`

### Deterministic Item Stat-Model Gaps
- Structured item passives are not globally applied in deterministic stat resolution; only selected paths are modeled.
- Ratio and scaling passives (for example global amplification effects) are not represented unless explicitly scripted.

## Rune, Mastery, And Shard Coverage
### Rune Snapshot
- Total runes in compatibility flat file `Masteries/RunesReforged.json`: `61`
- Total runes in split index `Masteries/RunesReforged/RunesReforged.json`: `61`
- Split tree coverage: `5/5` trees with both `primary_runes.json` and `secondary_runes.json`
- Split stat shard coverage: `1/1` stat shard file (`Masteries/RunesReforged/StatShards/stat_shards.json`)
- Dynamic combat-time runes modeled: `16`
- Deterministic static runes modeled: `6`
- Modeled union (`dynamic OR deterministic static`): `22`
- Runes unmodeled in both paths: `39`

### Dynamic Combat-Time Runes Modeled (`16`)
- `Aftershock`
- `Arcane Comet`
- `Conqueror`
- `Dark Harvest`
- `Electrocute`
- `First Strike`
- `Fleet Footwork`
- `Gathering Storm`
- `Grasp of the Undying`
- `Hail of Blades`
- `Lethal Tempo`
- `Phase Rush`
- `Press the Attack`
- `Second Wind`
- `Summon Aery`
- `Triumph`

### Deterministic Static Runes Modeled (`6`)
- `Celerity`
- `Jack Of All Trades`
- `Legend: Alacrity`
- `Legend: Haste`
- `Magical Footwear`
- `Nimbus Cloak`

### Runes Unmodeled (`39`)
- `Absolute Focus`
- `Absorb Life`
- `Approach Velocity`
- `Axiom Arcanist`
- `Biscuit Delivery`
- `Bone Plating`
- `Cash Back`
- `Cheap Shot`
- `Conditioning`
- `Cosmic Insight`
- `Coup de Grace`
- `Cut Down`
- `Deep Ward`
- `Demolish`
- `Font of Life`
- `Glacial Augment`
- `Grisly Mementos`
- `Guardian`
- `Hextech Flashtraption`
- `Last Stand`
- `Legend: Bloodline`
- `Manaflow Band`
- `Overgrowth`
- `Presence of Mind`
- `Relentless Hunter`
- `Revitalize`
- `Scorch`
- `Shield Bash`
- `Sixth Sense`
- `Sudden Impact`
- `Taste of Blood`
- `Time Warp Tonic`
- `Transcendence`
- `Treasure Hunter`
- `Triple Tonic`
- `Ultimate Hunter`
- `Unflinching`
- `Unsealed Spellbook`
- `Waterwalking`

### Mastery System Coverage
- Legacy `Season2016` masteries are intentionally retired and unsupported by runtime.
- Loadout parsing fails fast for `loadout.season2016_masteries`.
- Runes Reforged authoring now has split tree files plus a compatibility flat file; runtime still consumes the flat file until deferred loader migration.

### Shard And Stat Gaps
- Tenacity shard stat parses into deterministic stats, but runtime crowd-control duration reduction from tenacity is not modeled.
- `crit_chance_percent` is loaded into stats, but combat-time critical-strike behavior is not modeled.

## Engine, Combat, And Scenario Fidelity Gaps
### Engine And Physics
- 2D-only combat geometry (`x`, `y`); no validated `z` interaction model.
- Movement and pathing are deterministic simplifications versus live behavior.
- Projectile blocking and collision remain simplified versus live rules.
- Timing is discrete-tick simulation, not frame-accurate engine emulation.

### Combat System
- Resource systems (mana, energy, other resources) are not first-class runtime constraints.
- Many champion-specific conditional states and interactions are not represented.
- Item and rune interactions outside modeled sets are absent from combat outcomes.

### Artificial Intelligence And Scenario Scope
- Enemy and controlled champion behavior is policy-driven but narrow versus full gameplay decisions.
- Scope is teamfight-centric; lane, objective, and map-state systems are not modeled.

## Tracking Notes
- Search quality gates exist for both unmodeled runes and unmodeled item effects.
- Reports expose gate policy and rejected and penalized candidate counters.
- Update this file when any of the following changes:
  - modeled champion, item, or rune sets
  - legal URF pool composition
  - shard/stat runtime support
  - engine or fidelity assumptions

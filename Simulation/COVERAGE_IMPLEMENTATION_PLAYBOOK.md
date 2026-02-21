# Coverage Implementation Playbook

This playbook defines the **implementation quality bar** for adding comprehensive champion, item, rune, and mastery/shard coverage.

Use this with:
- `Simulation/DATA_AUTHORING_GUIDE.md`
- `Simulation/COVERAGE_CHECKLIST.md`
- `Simulation/COVERAGE_EXECUTION_TRACKER.md`
- `Simulation/CHAMPION_ROSTER_TRACKER.md`

---

## 1) Definition Of Done (Global)
A coverage unit (champion script, item runtime effect, rune runtime effect) is considered done only when all are true:

1. **Canonical data is correct and owned by the right file domain** (no policy leakage into wrong schema owner).
2. **Runtime behavior is implemented in scripts/runtime modules**, not shared engine/core/search/reporting special-cases.
3. **Tests cover formula correctness + runtime trigger behavior + edge conditions**.
4. **Coverage registries and trackers reflect the new modeled status**.
5. **Docs/tracker updates explain what was modeled, what remains unmodeled, and known assumptions.**

---

## 2) Champion Coverage Standard

### 2.1 Data requirements (`Characters/<Champion>.json`)
- Canonical sections present and populated:
  - `name`, `base_stats`, `basic_attack`, `abilities`
- Every castable ability includes canonical slot identity (`slot` or `default_keybinding`) and runtime execution geometry as applicable.
- Champion-owned simulation policy is minimal and keyed by stable mechanic identity.

### 2.2 Runtime requirements
- Controlled champion behavior integration (if supported) via champion script facade.
- Enemy scripted-event behavior integration for key combat abilities.
- No champion-specific branches in shared core modules when script extension can express the behavior.

### 2.3 Minimum tests
- Ability damage/heal/scaling correctness against canonical data.
- Range/targeting gating behavior.
- Cooldown/trigger cadence behavior.
- At least one negative-path test (out of range, unavailable state, target invalid, etc.).

---

## 3) Item Coverage Standard

### 3.1 Data requirements (`Items/<Item>.json`)
- Structured effects are explicit and stable (`effects_structured[].id` is durable).
- Trigger/cooldown/duration/scaling metadata is present when relevant.
- `schema_notes` clarifies assumptions/derivation where values are ambiguous.

### 3.2 Runtime requirements
- Runtime mechanics implemented in shared loadout/item runtime hooks.
- Behavior expresses actor/target-neutral interfaces and supports both controlled champion and enemies where applicable.
- Modeled item key added to `Simulation/src/scripts/coverage.rs` only after runtime behavior is truly represented.

### 3.3 Minimum tests
- Proc/trigger timing and cooldown enforcement.
- Damage/healing/stat bonus formula checks.
- Interaction safety (cannot trigger while invalid, no duplicate-proc bugs, etc.).

---

## 4) Rune And Shard Coverage Standard

### 4.1 Data requirements (`Masteries/RunesReforged.json`)
- Slot ordering and path legality remain canonical.
- Deterministic stat-affecting runes are parseable in stat resolution.
- Dynamic runes have runtime IDs/keys that map cleanly to runtime behavior.

### 4.2 Runtime requirements
- Dynamic effects implemented in shared runtime code paths used by both sides.
- Runtime tuning ownership stays in `Simulation/data/simulator_defaults.json`.
- Modeled dynamic keys listed in `Simulation/src/scripts/runes/effects.rs` only after behavior exists.

### 4.3 Minimum tests
- Trigger conditions and cooldown windows.
- Level/stack scaling and caps.
- Source attribution/telemetry correctness if telemetry is exposed.

---

## 5) Mastery Policy Standard
- Legacy `Season2016` masteries are currently retired.
- If re-enabled in future, require:
  - explicit parser/runtime policy ownership,
  - deterministic legality validation,
  - runtime + report parity tests,
  - updated docs and coverage trackers.

---

## 6) Batch Delivery Workflow (Recommended)
Use predictable batches to land comprehensive progress safely.

1. Pick one lane:
   - Champion scripts batch (for example 5–10 champions), or
   - Item runtime batch (for example 10 items), or
   - Rune runtime batch (for example 5–10 runes).
2. Implement canonical data fixes first (if needed).
3. Implement runtime scripting.
4. Add/extend tests.
5. Update coverage registries.
6. Regenerate trackers:
   - `Simulation/scripts/generate_coverage_trackers.py`
7. Update docs:
   - `Simulation/IMPROVEMENT_TRACKER.md`
   - `Simulation/COVERAGE_GAPS.md`
   - `Simulation/IMPLEMENTATION_ROADMAP.md` status if milestone changed.
8. Run full validation gates before commit.

---

## 7) PR Acceptance Template
Every coverage PR should include:

1. **Modeled now**
   - explicit list of champions/items/runes covered.
2. **Behavior implemented**
   - trigger rules, cooldown rules, scaling formulas.
3. **Still unmodeled**
   - precise remaining gaps and why they were deferred.
4. **Confidence notes**
   - authoritative sources used, assumptions, and uncertainty notes.
5. **Validation**
   - `cargo fmt`
   - `cargo clippy -D warnings`
   - `cargo test --release`

This keeps quality aligned across contributors and prevents partial “checkbox” coverage claims that lack runtime fidelity.

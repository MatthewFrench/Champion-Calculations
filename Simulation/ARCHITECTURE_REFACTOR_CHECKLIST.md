# Architecture Refactor Checklist

Use this checklist for architecture-oriented changes under `Simulation/src/`.

Standards:

- `ARCHITECTURE_STANDARDS.md`
- `ARCHITECTURE_TRANSFORMATION_PLAN.md`

## 1) Scope And Ownership

- [ ] Refactor scope is mapped to one or more `ARCH-*` milestone IDs.
- [ ] Current owner and target owner are identified for every moved responsibility.
- [ ] No new cross-module direct mutation paths were introduced.

## 2) API And Compatibility

- [ ] Public facade API impact is explicitly listed (`engine.rs`, `search.rs`, `scenario_runner.rs`, `defaults.rs`, `data.rs`, `reporting.rs`).
- [ ] Compatibility re-exports/shims were added where needed.
- [ ] Shim removal milestone IDs are recorded for temporary compatibility layers.

## 3) Behavior And Determinism

- [ ] Change preserves gameplay/runtime behavior (or behavior differences are explicitly documented and approved).
- [ ] Seeded determinism is preserved and validated for affected flows.
- [ ] Ordering-sensitive paths (parallel search merges, queue ordering, tie-breaks) were validated.

## 4) Performance And Safety

- [ ] No hot-path regressions were introduced by unnecessary allocation/copying.
- [ ] Concurrency changes preserve lock ownership and avoid undocumented nested lock ordering.
- [ ] Error paths include actionable key/path context for parse/load failures.

## 5) Testing And Validation

- [ ] Added/updated tests for moved logic (unit/integration as appropriate).
- [ ] Required validation commands passed:
  - [ ] `cargo fmt --manifest-path Simulation/Cargo.toml`
  - [ ] `cargo clippy --all-targets --all-features --manifest-path Simulation/Cargo.toml -- -D warnings`
  - [ ] `cargo test --release --manifest-path Simulation/Cargo.toml`

## 6) Documentation And Tracking

- [ ] Updated `ARCHITECTURE_TRANSFORMATION_PLAN.md` milestone status and metrics.
- [ ] Updated `IMPLEMENTATION_ROADMAP.md` architecture progress notes.
- [ ] Updated `IMPROVEMENT_TRACKER.md` with landed improvements.
- [ ] Updated `README.md` and `Simulation/README.md` when user-facing architecture guidance changed.

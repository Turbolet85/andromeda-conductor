# Report — 2026-06-19-emission-gap-resume

**Chunk:** Emission gap/resume — deterministic exact-length silence window (>20s) + resume marker, the P-015 restart-detection lever (conductor-faults, P-015)
**Date:** 2026-06-19T22:46:20Z
**Commits:** none yet — the chunk is uncommitted; wrap P7 creates the commit. (Prior commit `ae9e697` = the already-wrapped port-occupier chunk.)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-faults/src/gap.rs` (new) · `crates/conductor-faults/src/error.rs` (mod) · `crates/conductor-faults/src/lib.rs` (mod)
- **Symbols / APIs:** new public `EmissionGap` struct with `new(Duration) -> Result<Self, FaultError>`, `gap() -> Duration`, `gap_ms() -> u64`; new public consts `MIN_GAP`/`MAX_GAP` (`Duration`); two new `FaultError` variants `GapTooShort { gap, min }` / `GapTooLong { gap, max }` (enum is `#[non_exhaustive]`). All re-exported from `conductor-faults` `lib.rs`. No IPC methods, endpoints, ports/sockets, or env vars.
- **Crates / modules:** new module `conductor-faults::gap`; no crate added/removed.
- **Dependencies:** none added/bumped (uses `std::time::Duration`); `Cargo.lock`/`Cargo.toml` un-drifted.
- **Schema / config:** none (runtime helper type, not scenario config — no garde/serde, no TOML keys).
- **Coverage of new surfaces:**
  - `EmissionGap` (backend fault-descriptor type — not an external surface / hot-path / UI element) → validation **constructor-validated ✓** (>20s floor + 1h ceiling → typed `FaultError`, never panic; garde **n/a** — runtime helper, not config) · instrumentation **n/a** (no span/log; obs `fault.silence` span deferred to the driven-under-timeline epoch) · PII **n/a** (no payload) · tests **unit ✓ (8) + doctest ✓** · a11y **n/a** (no UI) · tokens **n/a** (no UI).

## Deviations from intent
- **Included `gap_ms()`** (plan marked *optional*) — serves the documented Epoch-7 timeline/`PhaseSpec` `u64`-ms boundary; tested; the `u128→u64` narrowing is bounded by `MAX_GAP` and carries a justifying comment (consistent with `rate.rs`'s uncommented-cast style; no clippy flag at the project's lint level).
- **Included `GapTooLong` + `MAX_GAP` ceiling** (plan marked *recommended*) — parity with core's `MAX_GAP_MS` sanity bound (phase_spec.rs:15); both variants tested.
- **Exported `MIN_GAP`/`MAX_GAP`** (plan: "if public") — the contract thresholds, available to consumers/tests.
- **Did NOT add a `MalformedGapResume` ordering error** — deliberate plan-adherence: resume is intrinsic to the single-`Duration` shape, so there is nothing to order (the P2 design/security extracts had speculated such an error; the plan explicitly rejected it).
- **Micro-reorder**: wrote `error.rs` (variants) before `gap.rs` (which references them) — compile-dependency order; no scope/behavior change.
- **scope.md determinism framing refined at P5** (val-1 intent-incomplete): "deterministic under seed" → "deterministic by construction (exact)" — an exact gap is seed-independent by nature; recorded in `scope.md` + `plan.md`. Same intent ("exact gap lengths"), no scope change.

## Decisions & corrections
- **Validation idiom** — chose `Result<Self, FaultError>` (extend the `#[non_exhaustive]` `FaultError` that is *designed* to extend, per error.rs:6 + port_occupier precedent) over `rate.rs`'s `Option<Self>`. Rationale: a named sub-floor error is clearer than an anonymous `None` for the P-015-critical 20s threshold. Establishes the split: `conductor-faults` helpers that can fail a *named domain constraint* return `Result<_, FaultError>`; `rate.rs`'s pure-value bounds (`windows>0`) use `Option`.
- **Seed-independence** — chose exact/seedless (no `seed` param): "exact gap lengths" means the value is fixed, so a seed would be the "computed-but-never-applied" anti-pattern. Determinism is by construction; the both-directions test becomes exact-value-preservation + distinct-inputs-diverge (not `different_seeds_diverge`). Mirrors the exception-fingerprint primitive's seed-independent derived value (2026-06-18 learning).
- **P2 design distiller over-reach** — the design domain-extractor manufactured downstream verdict-rendering relevance for a no-UI backend chunk and carried cross-domain `nextest` content (failed in-domain validation); a single retry naming the check yielded the correct `## No domain coverage`. (Process note: domain extractors can over-reach on backend-only chunks; the retry-with-check-name path corrected it.)
- No explicit user corrections this session — the user drove via the andromeda pipeline and approved the P5 plan by launching `/andromeda-implement`.

## Outcome
- **Acceptance criteria: all met** — exact gap validated against the >20s floor (single named `MIN_GAP`) + sanity ceiling; out-of-range ⇒ typed `FaultError`, no panic (verdict/error wall); exact deterministic gap (preserved verbatim); re-exported from `lib.rs`; emits no OTLP/span/log.
- **Gates green:** `cargo nextest run -p conductor-faults` (15/15) · `cargo test --doc -p conductor-faults` (1/1) · `cargo clippy --workspace --all-targets -- -D warnings` (clean) · `cargo nextest run --workspace --profile ci` (161/161, +8) · `cargo llvm-cov nextest --workspace --fail-under-lines 60` (96.80% total; gap.rs 97.37%, error.rs 100%).
- **Smoke:** skipped — no boot-path change (pure library primitive; cleanup/timeline wiring is Epoch 7/8).

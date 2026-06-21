# Report — 2026-06-20-bursty-train-pattern

**Chunk:** Bursty-train pattern — repeating active/quiet duty cycle, the P-013 activity-floor false-positive guard (conductor-faults, P-013)
**Date:** 2026-06-21T07:35:23Z
**Commits:** (uncommitted at report time — this wrap authors the single `feat(2026-06-20-bursty-train-pattern)` commit)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-faults/src/train.rs`
  - MOD `crates/conductor-faults/src/error.rs`
  - MOD `crates/conductor-faults/src/lib.rs`
  - (process artifacts: `.andromeda/master-route.md`, `conductor-0.1.0/working-route.md`, the chunk folder, run-dirs — not code surfaces)
- **Symbols / APIs (new public, all in `conductor-faults`):**
  - `BurstyTrain` (`#[derive(Debug, Clone, Copy, PartialEq, Eq)]` value) + `impl Default`
  - `BurstyTrain::new(active: Duration, quiet: Duration) -> Result<Self, FaultError>` (validated)
  - `BurstyTrain::canonical() -> Self` (infallible — statically-valid 5min/10min)
  - `BurstyTrain::active()/quiet()/period() -> Duration` · `active_ms()/quiet_ms()/period_ms() -> u64`
  - `BurstyTrain::is_active_at(offset: Duration) -> bool` (within-cycle phase query; half-open `[0,active)`)
  - consts `MAX_WINDOW` (1h), `CANONICAL_ACTIVE` (5min), `CANONICAL_QUIET` (10min)
  - `FaultError` +3 variants: `ActiveZero`, `QuietZero`, `WindowTooLong { window: Duration, max: Duration }` (enum stays `#[non_exhaustive]`)
- **Crates / modules:** new module `train` in `conductor-faults`; no new crate; no crate-edge change (leaf crate, zero dependents per code-graph).
- **Dependencies:** none added/bumped; `Cargo.lock` un-drifted (`std::time` + existing `thiserror` only).
- **Schema / config:** none — no scenario-config struct, no `runs.db` schema, no violation schema, no env var, no port/socket.
- **Coverage of new surfaces:**
  - `BurstyTrain` constructor (input-validation surface) → validation **Result/`FaultError`✓** (garde **n/a** — this is a fault-helper constructor, not a serde scenario-config struct; garde wiring for the fault helpers is the tracked Epoch-7 scenario-config follow-up, same posture as `EmissionGap`/`AbruptSilence`) · instrumentation **n/a — deferred** (`fault.*` span is route-sequenced to Epoch 7/8 timeline wiring; the prior chunk's playbook rule classes this as routine non-drift) · PII **n/a** (no payload) · tests **unit✓ (8 `train` + 3 `error` Display) + doctest✓** · a11y **n/a** (no UI) · tokens **n/a** (no UI).

## Deviations from intent
- Added "bursty-train" to the `lib.rs` crate-doc header parenthetical — plan marked this *optional*; taken for accuracy (the helper is now shipped). Immaterial doc edit.
- Folded the `Default` assertion into `canonical_is_5min_10min_and_is_the_default` instead of a separate test — `Default` only delegates to `canonical()`, so one test covers both; avoids the tautological-`Default`-test clippy issue (`default_constructed_unit_structs`) the `AbruptSilence` chunk hit.
- `duty_cycle()` not implemented — the plan deliberately omitted it (keeps the surface deterministic-test-friendly, no float epsilon); deferred to Epoch 7 if a scenario needs the ratio.
- No other deviations; all plan acceptance criteria met.

## Decisions & corrections
- **P4 "Helper shape" (AskUserQuestion) → Parameterized + validated (`Result`).** `BurstyTrain` joins `EmissionGap`/`PortOccupier` on the `Result`/`FaultError` branch of the fault-constructor idiom; `AbruptSilence` remains the infallible branch; the `Option` (anonymous-bound) branch is still unused. The idiom now spans all 4 Epoch-4 helpers.
- **`MAX_WINDOW` = 1h, inclusive, applied per window** — deliberately sized so the *same* helper expresses the Epoch-7 "lunch" duty cycle (60min quiet) the activity-floor scenarios (P-013..P-016) need; a fixed marker would have forced a second helper. Parity with `EmissionGap`'s `MAX_GAP`.
- **`is_active_at` half-open convention** — active is `[0, active)` of each cycle, quiet `[active, period)`; `is_active_at(active)` is `false`, `is_active_at(period)` is `true` (wraps). This is the testable "repeating" contract distinguishing P-013 from P-014 (never active again) / P-015 (one gap).
- **Activity-floor family complete** — `BurstyTrain` (P-013, recurring/healthy) is the false-positive guard alongside `AbruptSilence` (P-014, death) + `EmissionGap` (P-015, restart). Epoch 4 (Fault helpers) reaches 4/4.

## Outcome
- **All acceptance criteria met.** Gates green:
  - `cargo nextest run -p conductor-faults` → 29/29
  - `cargo clippy -p conductor-faults --all-targets -- -D warnings` → clean
  - `cargo test -p conductor-faults --doc` → 3/3 (incl. new `BurstyTrain::new`)
  - `cargo nextest run --workspace --profile ci` → 175/175 (was 164, +11)
  - `cargo llvm-cov nextest -p conductor-faults --fail-under-lines 60` → 98.22% lines (train.rs 100% lines / 100% fns)
  - `Cargo.lock` un-drifted · `cargo audit` exit 0 · `cargo deny check` ok
- **Smoke:** skipped — pure library primitive, no boot-path changed (timeline/cleanup wiring is Epoch 7/8), matching the `AbruptSilence` chunk.
- **No spec↔reality gaps surfaced** during implement.

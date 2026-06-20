# Report — 2026-06-19-abrupt-silence-fault

**Chunk:** Abrupt-silence fault — permanent emission stop (no resume), the P-014 activity-floor death lever (conductor-faults, P-014)
**Date:** 2026-06-20T11:34:28Z
**Commits:** (none yet — implement does not commit; this wrap creates the chunk commit)

## Changes (structured — detectors read this)
- **Files:** `crates/conductor-faults/src/silence.rs` (new) · `crates/conductor-faults/src/lib.rs` (modified: `mod silence;` + `pub use silence::AbruptSilence;` + crate-doc "Shipped so far" line). `error.rs` deliberately UNTOUCHED.
- **Symbols / APIs:** new public type `AbruptSilence` (unit marker struct, `#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]`) with `AbruptSilence::new() -> Self` (**infallible** — no `Result`) and `AbruptSilence::resumes(&self) -> bool` (always `false`); re-exported at crate root. No IPC methods · no endpoints · no ports/sockets · no env vars · no `FaultError` variant added.
- **Crates / modules:** new module `silence` inside the existing `conductor-faults` crate. No new workspace crate. Zero inbound crate edges to `conductor-faults` (code-graph confirmed) → purely additive `pub` surface.
- **Dependencies:** none added; none bumped. `Cargo.toml` untouched; `Cargo.lock` un-drifted (helper uses no external crates — only `Self`).
- **Schema / config:** none (no scenario-config field, no `runs.db` column, no violation schema, no garde descriptor).
- **Coverage of new surfaces:**
  - `AbruptSilence` (library fault primitive — NOT an external/network surface, hot-path op, or UI element) → validation {n/a — infallible marker, no config input to bounds-check} · instrumentation {✗ deferred — the obs `fault.silence` span is route-sequenced to Epoch 7/8 timeline wiring, not this chunk} · PII {n/a} · tests {unit✓ — 3 unit + 1 doctest, `silence.rs` 100% line coverage} · a11y {n/a — no UI} · tokens {n/a — no UI}

## Deviations from intent
- **Dropped the planned `default_equals_new` unit test** (plan listed `Default == new()` under value-semantics acceptance). **Justification:** the clippy lint `default_constructed_unit_structs` (default-level, fired under `-D warnings`) correctly flags `AbruptSilence::default()` as redundant for a *unit* struct, making the assertion tautological. The `Default` **derive is retained** (load-bearing for `clippy::new_without_default` + marker hygiene); substantive value semantics stay covered by `is_a_copy_value` (Copy + `PartialEq`) and `construction_is_reproducible` (determinism). Net: 3 unit tests + 1 doctest, `silence.rs` at 100% coverage. Every other plan element shipped exactly as specified.

## Decisions & corrections
- **Design fork resolved by P4 AskUserQuestion → "Infallible marker":** `AbruptSilence` is a parameterless, infallible marker (`new() -> Self`), the crate's **first infallible fault helper** (`FaultError` untouched). Rationale: unlike `EmissionGap` (validates a 20s floor) and `PortOccupier` (acquires a socket), a permanent stop has no bound to validate and no resource to acquire — nothing to fail on. This is the same principle the gap chunk used (a *named* error only for a *real* threshold) taken to its conclusion: no threshold ⇒ no error. The user rejected both the "fallible-for-symmetry" shape and the "carry a ~30s cue const" shape.
- **Permanence is positively testable, not just absent:** `resumes(&self) -> bool { false }` encodes the no-resume contract that structurally distinguishes P-014 from `EmissionGap`/P-015 (which conceptually resumes).
- **Module named `silence.rs`** (concept-named like `gap.rs`→`EmissionGap`, and matches the obs `fault.silence` span name) housing the descriptive type `AbruptSilence`.
- **Clippy `default_constructed_unit_structs` is a live hazard for unit-struct + derived-Default + a `default()`-assertion** (see Deviations) — surfaced during the fix-loop, fixed by dropping the tautological test.
- **Carried/deferred (NOT actioned this chunk):** the obs `fault.silence` span lists a `fault_duration_ms` attribute (obs-plan §4) that has no value for a *permanent* stop; the null/sentinel-duration handling is an Epoch-7/8 timeline-wiring concern. Flagged so it is not lost.

## Outcome
- **Acceptance criteria: all met.** Lives in `conductor-faults` + re-exported from `lib.rs`; infallible panic-free `new()`; `FaultError`/`error.rs` untouched (verdict/error wall holds vacuously); deterministic by construction (no seed, no different-seeds test); `resumes()==false` exercised; no new deps; coverage ≥60%.
- **Gates green:** `cargo nextest run -p conductor-faults` 19/19 (+4 → +3 after the test drop) · `cargo test --doc -p conductor-faults` 2/2 · `cargo nextest run --workspace --profile ci` **164/164** (was 161; +3) · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo llvm-cov nextest --fail-under-lines 60` TOTAL **96.79%**, `silence.rs` **100.00%**, exit 0.
- **Smoke:** skipped — `no boot-path change` (library-only touchpoints: a new module + a re-export; no binary/daemon/entry-point; plan lists no `agent-run.sh`). Consistent with the port-occupier and emission-gap chunks.
- **Fix-loop:** green in 2 iterations (iter 1 caught `clippy::default_constructed_unit_structs`; iter 2 fully green).

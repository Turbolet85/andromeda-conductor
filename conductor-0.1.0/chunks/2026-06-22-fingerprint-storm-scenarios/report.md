# Report — 2026-06-22-fingerprint-storm-scenarios

**Chunk:** Fingerprint-storm scenarios (P-017, P-018) — fingerprint-identity triple (identical/path/line → one fp; type/frame → distinct) + storm-cue thresholds (6→Suggested/12→Autonomous) catalog TOMLs; all-Hard, zero model change (conductor-core scenarios)
**Date:** 2026-06-22T19-08-21Z
**Commits:** (uncommitted at report time — this wrap commits the chunk; HEAD carries the prior `2026-06-22-activity-floor-restart-suppression-scenarios` feat + an unrelated `#17` code-graph-cookbook chore)

## Changes (structured — detectors read this)
- **Files:** `scenarios/fingerprint-storm.toml` (NEW), `scenarios/fingerprint-distinct.toml` (NEW), `crates/conductor-core/src/scenario.rs` (MOD — tests only).
- **Symbols / APIs:** none — no new public fn / IPC method / endpoint / export / port / socket / env var. The change is 2 declarative scenario config files + 4 new in-crate `#[cfg(test)]` fns (+6 test instances) in `conductor-core`.
- **Crates / modules:** none added / removed / changed (no production code in `scenario.rs`; the model types `Scenario`/`ExpectedCheck`/`ComparisonKind`/`ClaimClass` are reused unchanged — code-graph: 33/18/57/69 refs, none modified).
- **Dependencies:** none added / bumped.
- **Schema / config:** 2 new `scenarios/*.toml` keyed to P-017/P-018, reusing the existing serde+garde `Scenario` schema (no schema change). New read-back token declared in config only: `RetryStorm` (inferred — spec uses prose; `Contains` in fingerprint-storm, `Absent` in fingerprint-distinct). All `[[expected]]` checks `class="Hard"`; the 6→Suggested/12→Autonomous severity escalation is declare-only (model-side, P-020).
- **Coverage of new surfaces:**
  - `scenarios/fingerprint-storm.toml` (config — P-017+P-018) → validation **garde✓** (`Scenario::from_toml_str`) · instrumentation **n/a** (declarative config; the `scenario.run`/`verify.readback*` spans are the Epoch-8 driver's, per obs §4) · PII **n/a** · tests **unit✓** (loader + all-Hard + `Contains "RetryStorm"` rstest) · a11y **n/a** · tokens **n/a**
  - `scenarios/fingerprint-distinct.toml` (config — P-017) → validation **garde✓** · instrumentation **n/a** · PII **n/a** · tests **unit✓** (loader + all-Hard + `Absent "RetryStorm"` rstest) · a11y **n/a** · tokens **n/a**

## Deviations from intent
- **None material.** Phase counts match the plan's recipe (storm = two phases 6×/12×; distinct = three sub-floor phases base/type/frame).
- **One minor, justified strengthening:** the Contains/Absent presence tests additionally assert `c.expected == "RetryStorm"` (the sibling guards check only the `kind`). Justification: it pins the **inferred** token explicitly — the chunk's headline Epoch-8 calibration point — so an accidental token rename is caught at unit level.
- **scope.md acceptance #3 amended at phase P5** (validation-1 intent-incomplete): from "6→Suggested/12→Autonomous both asserted" to "storm detection asserted via `Contains "RetryStorm"` (Hard); severity escalation declare-only, deferred to P-020/P-021" — the precise form of the all-Hard boundary the P4 decision fixed.

## Decisions & corrections
- **P4 AskUserQuestion — defer severity to P-020 (all-Hard):** §P-018's 6→Suggested/12→Autonomous is model-determined ("final severity per P-020; the model retains discretion"), and arch §Probabilistic-Assertion Policy classes severity choice as model-interpretive. So — matching every prior catalog chunk (error-baseline-spike / latency-regression / activity-floor / restart-suppression all assert the deterministic detector token Hard and defer model-side severity to P-020) — this chunk asserts only the deterministic `RetryStorm` **detection** token (Hard) and records the severity escalation **declare-only** in TOML comments. Chunk stays all-Hard, zero CalibrationRegion.
- **Two-file outcome-coherence split** (`fingerprint-storm.toml` `Contains "RetryStorm"` / `fingerprint-distinct.toml` `Absent "RetryStorm"`): forced by the single-read-back rule — `Contains`+`Absent` of one token contradict and cannot co-exist in a file (the activity-floor P-013/P-014 precedent + `.claude/rules/testing.md`).
- **P-017 has no direct user signal** ("identifier infrastructure for P-018") → verified THROUGH P-018: the identical/path/line triple shares ONE fingerprint (so a mixed-variant storm counts toward one threshold → `Contains`); type/frame variants are DISTINCT, kept sub-floor (<5×) so they never aggregate → `Absent`. Reuses the shipped `conductor-emit` `FingerprintVariant` primitive; `EmissionSpec` carries no count field, so the Epoch-8 driver realizes per-phase counts from phase names.
- **`RetryStorm` is an inferred token** (the `LatencyRegression` precedent — spec prose, `Contains` substring-tolerant); flagged in both TOMLs as an Epoch-8 calibration point alongside the `<20s` tier (no explicit storm-detection p99 budget) + the 30s-window emission cadence + the model-mediated incident surfacing.

## Outcome
- **Acceptance criteria met:** both TOMLs parse + garde-validate, keyed to P-IDs (P-017+P-018 / P-017); P-017 asserted both ways (aggregate→`Contains`, distinct→`Absent`) in outcome-coherent separate files; every `[[expected]]` `class="Hard"`; no new dependency / model change.
- **Gates green (0 fix iterations):** `cargo nextest run -p conductor-core` **129/129** (123→129) · `cargo nextest run --workspace --profile ci` **334/334** (328→334) · `cargo clippy --workspace --all-targets -- -D warnings` **clean** · `cargo test --doc -p conductor-core` **0** · determinism insta goldens **UNCHANGED** (seeds 4317017/4317018 feed no golden; each loader case is an explicit `#[case]` stem).
- **Smoke:** `bash scripts/agent-run.sh run` **exit 0** (full nextest + doctests through the harness, exercising both new TOMLs).

# Report — 2026-06-21-hard-signals-scenarios

**Chunk:** Hard-signals scenarios (P-005..P-008) — ERROR-status / exception-event / severity-17-boundary / root-vs-deep catalog TOMLs; Hard checks + P-008 CalibrationRegion per v2.1 amendment (conductor-core scenarios)
**Date:** 2026-06-21T23:36:16Z
**Commits:** (none yet — chunk committed in this wrap's P7; prior `last_wrap` HEAD = 0a9efb9 connection-lifecycle-scenarios)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `scenarios/span-status-error-detection.toml` (P-005)
  - NEW `scenarios/exception-event-capture.toml` (P-006)
  - NEW `scenarios/high-severity-log-capture.toml` (P-007)
  - NEW `scenarios/root-span-error-scope.toml` (P-008)
  - MOD `crates/conductor-core/src/scenario.rs` (test module only)
  - (process artifacts, not impl: `.andromeda/master-route.md`, `conductor-0.1.0/working-route.md`, the chunk folder, the phase/wrap run-dirs)
- **Symbols / APIs:** **none public.** Added 3 `#[cfg(test)]` fns to `scenario.rs`: `hard_signal_fixtures_load_and_validate` (rstest, 4 cases), `p008_root_span_error_scope_checks_are_calibration_region`, `p007_high_severity_log_capture_asserts_both_sides_of_the_boundary`. No new public fn / IPC method / endpoint / export / port / socket / env var.
- **Crates / modules:** none added/removed. `conductor-core` test module extended only.
- **Dependencies:** **none added / none bumped** — `Cargo.lock` un-drifted (verified `git diff --name-only -- Cargo.lock` empty).
- **Schema / config:** 4 new declarative scenario config files (serde+garde `Scenario` TOML). **No schema migration, no Rust model change** — reuses the existing `Scenario` / `ExpectedCheck` / `ComparisonKind` (`Contains` + `Absent`) / `ClaimClass` (`Hard` + `CalibrationRegion`) / `SloTier` (`<5s`) / `Signal` (`Logs` for P-007). P-008's check is `class = "CalibrationRegion"` (the v2.1 amendment).
- **Coverage of new surfaces:**
  - `scenarios/*.toml` (4 hard-signal config files — external-input surface) → validation **garde✓** (via `Scenario::from_toml_str` + `#[garde(dive)]` on `expected`; fixture rstest proves load+validate) · instrumentation **n/a** (no runtime operation added this chunk) · PII **n/a** (synthetic markers only) · tests **unit✓** (round-trip rstest + 2 guard tests) · a11y **n/a** (no UI) · tokens **n/a** (no UI)

## Deviations from intent
1. **Smoke command.** Plan's Test Commands listed `bash scripts/agent-run.sh status` (expecting exit 0). In reality `status` requires a `<run_id>` (exit 2 without one) and reads a *specific* run's journal — and no live run exists yet (live scenario execution is the Epoch-8 CLI driver). Ran **`bash scripts/agent-run.sh run`** instead — the agent-driven release-gate entrypoint (workspace nextest + doctest + clippy through the harness, exit 0) — the correct source-of-truth smoke for a no-boot-path-change chunk. **Justification:** `status`/`logs` need a prior run; `boot` needs a live Pulse / rmcp stub; neither is available pre-Epoch-8. No code impact. (See Decisions: future Epoch-7 catalog chunks should use `run` as the smoke.)

Everything else matched the plan exactly: 4 files one-per-P-ID, the Hard/CalibrationRegion split, P-007's two-sided boundary via the existing `ComparisonKind::Absent` + `signal="logs"`, no new emit primitive / inbound bind / dependency.

## Decisions & corrections
- **Catalog shape (P4 AskUserQuestion):** 4 files, one per P-ID (mirrors ch1's "distinct verification recipe ⇒ distinct file"), descriptive kebab names; P-007 two-sided via `Contains`+`Absent`; P-008 `CalibrationRegion`.
- **Zero Rust model change (research finding):** `ComparisonKind::Absent` expresses P-007's "WARN <17 does NOT contribute"; `ClaimClass::CalibrationRegion` expresses P-008's model-side tendency; the `conductor-verify` evaluator (`slo.rs:79` `evaluate_check`, test `exact_contains_absent_compare_as_expected` slo.rs:99) already handles all four `ComparisonKind`s — so this chunk is config + tests only.
- **P-007 emission signal:** set `emission = { signal = "logs" }` on its phases (logs vs traces is semantically load-bearing for "High-Severity Log Capture"); P-005/006/008 use the trace default (ch1 convention — phase names carry intent, emission realization deferred to the Epoch-8 driver).
- **Smoke-command learning (correction):** for Epoch-7 scenario-catalog chunks (no boot-path change), the smoke is `agent-run.sh run`, not `status` — `status` requires a `<run_id>` and there are no live runs until the Epoch-8 CLI driver. Corrects the prior connection-lifecycle handoff note that implied `status` exits 0 read-only.

## Outcome
- **Acceptance criteria met:** all 4 TOMLs deserialize + garde-validate + build a valid `PhaseTimeline` (rstest green); P-008 `CalibrationRegion` asserted; P-007 two-sided (`Contains`+`Absent`) asserted; no new dependency; no model change; determinism preserved (seeded, `from_toml_str` path).
- **Gates green:** `cargo nextest run -p conductor-core` 110/110 · `cargo nextest run --workspace --profile ci` **315/315** · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo test --doc` 0.
- **Smoke:** `bash scripts/agent-run.sh run` (release-gate path) exit 0 — no boot-path change this chunk; `status`/`boot` require a live run / Pulse unavailable pre-Epoch-8.

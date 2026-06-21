# Codebase Research — 2026-06-21-verdict-assertion-policy-split

## Scope
- **Depth:** moderate · **Reads:** 7 (verdict.rs, report_state.rs, run_record.rs, verify/lib.rs, verify/error.rs, verify/preflight.rs, verify/Cargo.toml) · **Globs/Greps:** 3 + 2 code-graph queries

## Files inspected
- `crates/conductor-core/src/verdict.rs` (full) — **`Verdict { Pass, Fail, CalibrationRegion }` already exists** (derive `Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize`; serde PascalCase). Carries `label()` (`CalibrationRegion → "HOLD"`) + `status_prefix()` (`[PASS]/[FAIL]/[HOLD]`). This chunk does NOT define the enum — it builds the logic that *produces* one. Doc-comment already cites arch §Probabilistic-Assertion Policy + the verdict/error wall.
- `crates/conductor-core/src/report_state.rs` (full) — `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`; the Verdict→ReportState mapping is NOT this chunk (Epoch 6). `Blocked` already produced by preflight.
- `crates/conductor-core/src/run_record.rs` (full) — the 11-field `RunRecord` envelope; `verdict: Option<Verdict>`, `state: ReportState`. The classification output ultimately lands in `verdict`. Schema/golden owned by test-plan §3 — do not touch the envelope here.
- `crates/conductor-verify/src/preflight.rs` (full) — **the pattern to mirror.** `ReadyState` is a serializable outcome *value*; `run_preflight()` returns `Result<ReadyState, VerifyError>` = `Ok(value)` for every outcome (incl. Blocked), `Err` only for harness faults; `ReadyState::report_state()` maps value→`ReportState`. `#[tracing::instrument(name="verify.readback.preflight")]`; uses `conductor_core::{now_rfc3339, redact_value}`. Auxiliary outcome enums (`ToolPresence`, `CanaryOutcome`) are `Serialize` with `#[serde(rename_all="lowercase")]`.
- `crates/conductor-verify/src/lib.rs` (full) — modules `client, error, manifest, preflight, spawn`; **no verdict/classify module yet.** lib.rs doc literally says "verdict classification build[s] on this handle in later Epoch-5 chunks" — this is that chunk.
- `crates/conductor-verify/src/error.rs` (full) — `VerifyError` is `#[non_exhaustive]`, `thiserror`; doc says "later Epoch-5 verification chunks extend the surface." A classification needs NO new variant (classification always returns `Ok(Verdict)`); only add one if a harness fault is genuinely introduced (unlikely).
- `crates/conductor-verify/Cargo.toml` (full) — deps: `conductor-core` (only workspace dep), rmcp, tokio, thiserror, tracing, serde, serde_json, toml. No new dep needed (classification is pure logic over existing types).

## Graph impact (from the code-graph query)
- **`conductor-core::Verdict`** (def `verdict.rs:13`) — referenced only inside `conductor-core` so far (`error.rs:41-42` CoreError, `lib.rs:31` re-export, `run_record.rs:4,32,85` envelope). **Zero `conductor-verify` references today** → this chunk is the first consumer of `Verdict` in the verify seam. No existing caller to break.
- **crate_edges:** `conductor-verify → conductor-core` is the only edge (star topology preserved; the chunk adds no new edge).

## Patterns detected
- **Outcome-as-value** (`preflight.rs:51-75`): a `#[derive(Serialize)]` struct holds the outcome + its human-facing fields, with a `report_state()`/`verdict()` accessor mapping to the core enum; the producing fn returns `Ok(outcome)` for all verification results. The verdict classifier should follow this exact shape.
- **Verdict/error wall** (`error.rs:1-12`, `preflight.rs:82-84`): MCP/transport failures are caught into a `Blocked`/typed value; `Result::Err` is harness-only. Classification has no transport, so it returns `Ok(Verdict)` infallibly.
- **Tracing span naming** (`preflight.rs:85`): `verify.readback.*` bounded names (obs rule). A verdict-classification span, if any, fits the bounded `verify.readback*` family — do NOT invent a high-cardinality per-claim name (obs rule §Spans).
- **Canonical-name golden** (`verdict.rs:50-59`, `run_record.rs:94-101`): enum wire spellings are locked via exact-string `assert_eq!` at unit level (testing rule + test-plan §4 amendment 2026-06-16).

## Conventions to follow
- **Determinism**: no wall-clock / RNG inside classification (`testing.md` + security extract); same inputs ⇒ same `Verdict`. (`std::time` only ever at journal stamps, which are not in this chunk.)
- **Module style**: new `verdict.rs` (or `classify.rs`) module in `conductor-verify/src/`, registered in `lib.rs:19-23`, public types re-exported in `lib.rs:25-33` next to the preflight exports.
- **Tests**: per-seam `cargo nextest run -p conductor-verify`; `#[rstest]` `#[case]` table rows; exact-string golden for any new serializable outcome enum. Both-directions where a seed is involved (n/a here — no seed in classification).
- **Sanitization**: if any observed/expected string is captured for the calibration-region delta, run it through `conductor_core::redact_value` before it could reach a log/artifact (security/obs: no host-path/struct-name leak).

## New files to create
- `crates/conductor-verify/src/verdict.rs` — the verdict-classification module: the assertion **policy-class** enum (Hard vs CalibrationRegion), the hard-path deterministic evaluator (observed vs expected → `Pass`/`Fail`), the calibration-region path (capture delta → `CalibrationRegion`, never `Fail`), returning a `Verdict` (and, recommended, a small outcome struct carrying the delta for the human-facing report).
- `crates/conductor-verify/tests/verdict.rs` — integration tests over the public classification surface (hard Pass/Fail, calibration-region-never-fails, determinism, canonical spellings of any new enum).

## Files to modify
- `crates/conductor-verify/src/lib.rs` — add `mod verdict;` + re-export the new public types alongside the preflight exports; update the crate doc-comment (the "verdict classification build[s] on this handle in later chunks" line is now fulfilled).

## Open questions
- **Assertion-model altitude (for P4):** does this chunk build only the minimal policy-split classifier (policy-class enum + observed/expected comparison → Verdict + delta capture), DEFERRING all concrete per-scenario `expected` blocks + SLO tolerance to the next chunk ("Expected-outcome + SLO timing model")? Or does it pre-build a richer typed-comparison kind set (equality / membership / ordering)? Recommended: minimal/generic now, concrete next — surface at P4 review.

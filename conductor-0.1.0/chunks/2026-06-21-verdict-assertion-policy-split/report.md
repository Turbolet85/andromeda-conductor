# Report — 2026-06-21-verdict-assertion-policy-split

**Chunk:** Verdict + assertion-policy split — deterministic hard Pass/Fail vs model-interpretive CalibrationRegion classification (conductor-verify)
**Date:** 2026-06-21T15:32:31Z
**Commits:** (uncommitted at report time — wrap commits in P7)

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-verify/src/verdict.rs`
  - NEW `crates/conductor-verify/tests/verdict.rs`
  - MOD `crates/conductor-verify/src/lib.rs`
- **Symbols / APIs:** new public, re-exported from the `conductor-verify` crate root:
  - `ClaimClass { Hard, CalibrationRegion }` — enum (the assertion policy class)
  - `Assessment { verdict, observed, expected, delta }` — struct + `verdict()` accessor (the outcome value)
  - `classify(class: ClaimClass, matched: bool, observed: impl Into<String>, expected: impl Into<String>) -> Assessment` — fn
  - No new ports / sockets / env vars / IPC methods / endpoints.
- **Crates / modules:** added module `verdict` to `conductor-verify`; no crate added/removed/renamed. Crate-dep graph unchanged (`conductor-verify → conductor-core` stays the only edge; `verdict.rs` imports only `conductor_core` + `serde`).
- **Dependencies:** none added, none bumped (uses existing `conductor-core` + `serde`; `rstest` already a dev-dep).
- **Schema / config:** none. `Assessment` is an internal `Serialize` type, NOT the `RunRecord` run-report envelope — the 11-field envelope (`run_record.rs`) is untouched.
- **Coverage of new surfaces:**
  - `classify()` verdict-classification logic → validation **n/a** (in-process value from the caller's comparison, not an external-input boundary — no deserialize/env/path/MCP-stdout) · instrumentation **✗ deliberate** (pure logic, un-instrumented per plan + the egress-probe precedent; the must-trace op is the read-back verification flow, which carries `verify.readback` spans) · PII **redacted✓** (`observed`/`expected` pass through `conductor_core::redact_value` at capture) · tests **unit+integ (11)** · a11y **n/a** (no UI) · tokens **n/a** (no UI)

## Deviations from intent
1. **lib.rs doc reword dropped the "OTLP egress liveness check" forward-reference.** The old sentence lumped egress + verdict as "build on this handle in later chunks," but the egress liveness check shipped in `conductor-emit` (per master-route), not `conductor-verify` — so that clause was never accurate for this crate. Scoped the sentence to the verdict classifier that actually lives here. Doc-accuracy fix within the touchpoint (lib.rs).
2. **`Verdict`/`CalibrationRegion` written as plain backticks (not `[`…`]` intra-doc links) in lib.rs.** `Verdict` isn't re-exported at the verify crate root, so an intra-doc link wouldn't resolve. (Inside `verdict.rs`, where `Verdict` is imported, the links resolve normally.)
3. **No `tracing` instrumentation in `classify`.** Kept pure per the plan's "keep classification light / probe-un-instrumented precedent." The delta is carried as the `Assessment.delta` field (the data contract); emitting it is the Epoch-6 report seam's job.

(The Option-A scope — minimal split + delta, deferring concrete comparison kinds + SLO timing tolerance to the next chunk — is the user-confirmed P4 decision, not a deviation.)

## Decisions & corrections
- **P4 scope decision (Option A, user-selected via AskUserQuestion):** this chunk builds the minimal policy-split classifier (`ClaimClass` + `classify` → `Assessment` with delta capture); concrete per-scenario `expected` blocks, typed comparison kinds, and SLO timing tolerance are deferred to the next chunk ("Expected-outcome + SLO timing model").
- `classify` is **infallible** — returns a value, never `Result::Err`; `VerifyError` gained no variant (the verdict/error wall: `Err` is harness-only).
- `delta` is `Some` only for `CalibrationRegion`; hard `Pass`/`Fail` carry `delta: None` (the verdict is itself the hard signal).
- `observed`/`expected` are redacted at capture (defense-in-depth; `classify` is not itself an external-input boundary, but its captured strings flow toward the run-report artifact).

## Outcome
- **Acceptance criteria met:** `classify` infallible (value, never `Err`) ✓ · `CalibrationRegion` never `Fail` (test-proven both `matched`/`!matched`) ✓ · deterministic, no wall-clock/RNG ✓ · `observed`/`expected` redacted via `redact_value` ✓ · `ClaimClass` canonical serde spelling locked by exact-string golden ✓ · no new crate edge (star topology preserved) ✓.
- **Gates green:** `cargo nextest run -p conductor-verify` **35/35** · `cargo clippy --workspace --all-targets -- -D warnings` **clean** · `cargo test -p conductor-verify --doc` **0 (ok)** · `cargo nextest run --workspace --profile ci` **213/213** (+11). Green in 1 iteration, no fixes.
- **Smoke:** skipped — no boot-path change (pure library logic; the `agent-run.sh` boot/run leg depends on the Epoch-8 `conductor` CLI, not yet built; `agent-run.sh status` requires a `<run_id>` with no runs to query, exit 2 usage).

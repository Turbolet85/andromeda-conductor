# Report — 2026-06-21-expected-outcome-slo-timing-model

**Chunk:** Expected-outcome + SLO timing model — per-scenario expected blocks + concrete comparison kinds + tier-scaled journal-relative SLO tolerance (<5s/<20s/<90s) feeding classify→Assessment (conductor-verify)
**Date:** 2026-06-21
**Commits:** (pending — wrap commits this chunk)

## Changes (structured — detectors read this)
- **Files:**
  - NEW: `crates/conductor-core/src/expected.rs` · `crates/conductor-verify/src/slo.rs` · `crates/conductor-verify/tests/expected_slo.rs`
  - MOD: `crates/conductor-core/src/lib.rs` · `crates/conductor-core/src/scenario.rs` · `crates/conductor-verify/src/lib.rs` · `crates/conductor-verify/src/verdict.rs`
- **Symbols / APIs:**
  - NEW (core, public): `ClaimClass {Hard, CalibrationRegion}` — **MOVED here from `conductor-verify`**, now also derives `Deserialize` (it is config); `ComparisonKind {Exact, Contains, Absent, CountAtLeast}` (serde + Deserialize, canonical PascalCase wire names); `ExpectedCheck {kind, class, expected: String}` (serde + `garde::Validate`).
  - NEW (core, public method): `SloTier::deadline_ms(self) -> i64` → 5_000 / 20_000 / 90_000 (additive impl; the `SloTier` enum and the `Scenario` struct are **unchanged**).
  - NEW (verify, public): `SloOutcome {latency_ms: i64, within_tolerance: bool}`, `CheckOutcome {assessment, slo, slo_tier}`, `evaluate_slo(tier, journal_emitted_at_ms: i64, read_back_observed_at_ms: i64) -> SloOutcome`, `compare(&ExpectedCheck, &str) -> bool`, `evaluate_check(check, observed, tier, emitted_ms, observed_ms) -> CheckOutcome`.
  - CHANGED (verify): `conductor_verify::ClaimClass` now **re-exports** `conductor_core::ClaimClass` (source-compatible); `classify`'s `class: ClaimClass` param resolves to the core type — `classify` signature byte-unchanged.
  - REMOVED (verify): the local `ClaimClass` enum definition + its `claim_class_serializes_to_canonical_names` test (relocated to core's `expected.rs`).
- **Crates / modules:** core `+mod expected`; verify `+mod slo`. No crate added/removed. **Star topology preserved** — `slo.rs` imports only `conductor_core` + crate-internal `verdict`; verify → core only.
- **Dependencies:** **none added/bumped.** core already had `garde`+`serde`; verify did **NOT** add `garde` (the model lives in core — the P4 "Core model" decision).
- **Schema / config:** new `ExpectedCheck` config type (serde + garde, validated at load → `CoreError::Validation`). **Not yet wired into `Scenario` / `scenarios/*.toml`** (deferred to Epoch 7). No `runs.db` / run-report-envelope change — `RunRecord` untouched (`latency_ms: Option<i64>` + `slo_tier` already existed; this chunk computes them).
- **Coverage of new surfaces:**
  - `ExpectedCheck` (declarative config type) → validation garde✓ (`length(min=1)` on `expected`; rejected-empty test) · instrumentation n/a (pure data) · PII n/a · tests unit (core, 5) · a11y n/a · tokens n/a
  - `evaluate_check` / `compare` / `evaluate_slo` (verify evaluator) → validation n/a (consumes the already-validated config) · instrumentation ✗ **deliberate** (pure logic — the must-trace op is the wrapping read-back flow, the egress-probe/`classify` precedent, obs-plan §4) · PII redacted✓ (`observed`/`expected` redacted inside `classify` via `redact_value` — host-path-redaction test) · tests unit (verify, 7) + integ (18) · a11y n/a · tokens n/a

## Deviations from intent
All four are sanctioned finalizations under the plan's explicit "/implement finalizes the exact representation + which kinds make the cut" latitude — none is a scope divergence or spec↔reality gap:
1. **Comparison set finalized to 4** (`Exact/Contains/Absent/CountAtLeast`). "Present" (a plan candidate) collapsed into `Contains` (presence of a token *is* Contains — no redundant variant); "ordering (p50≤p95≤p99)" stays a load-time garde concern (per research), not a runtime `ComparisonKind`.
2. **`ExpectedCheck.expected` is a flat `String` + garde `length(min=1)`** (not a richer typed target). Matches the existing flat-config style (`Scenario`/`PhaseSpec`) and sidesteps garde-0.22.1's lack of container-level cross-field `custom`. The `CountAtLeast` numeric floor is enforced **safely at compare-time** (non-numeric → unmet → `CalibrationRegion`, never a false `Pass`).
3. **No separate slack constant** — `deadline_ms()` is the tier bound; `within ⇔ latency_ms ≤ deadline_ms` (the tier IS the band, per the plan's own note). Runtime hardware-profile scaling stays out of scope (flagged at P5 review).
4. **`evaluate_slo` takes epoch-millis `i64`** (not `SystemTime`). Matches the envelope's integer-ms journal-offset representation (arch §Data model conventions); the `std::time` stamping is upstream, the evaluator only subtracts — the "never the virtual clock" invariant holds.

## Decisions & corrections
- **P4 scope decision (user-selected, AskUserQuestion):** Option 1 **"Core model, defer TOML"** — `ExpectedCheck`+`ComparisonKind` in core, `ClaimClass` moved to core (re-exported from verify), evaluator in verify, `Scenario`/TOML untouched (Epoch 7 wires per-P-ID blocks). Honors config-in-core + claim-class-declared-up-front; smallest blast radius.
- **`ClaimClass` relocation:** gained `Deserialize` (config now); verify re-exports `conductor_core::ClaimClass` for source-compat. Code-graph confirmed zero external callers, so the move was low-risk; the full-workspace gate caught no straggler.
- **Evaluator infallible** — no new `VerifyError` variant (verdict/error wall); the only `Err` path is the model's garde load-time validation (core's `CoreError::Validation`).
- **`compare`/`evaluate_slo`/`evaluate_check` left un-instrumented** (pure logic — the egress-probe/`classify` precedent).
- **Sample-count floor safety:** an unmet `CountAtLeast` forces `CalibrationRegion` regardless of the declared class (arch §Timing-Tolerance Model — sample floors never hard-fail).

## Outcome
- **All 8 plan acceptance criteria met.**
- **Gates green:** `cargo nextest run -p conductor-core -p conductor-verify --profile ci` → 127✓ · `cargo clippy -p conductor-core -p conductor-verify --all-targets -- -D warnings` → clean · `cargo test -p conductor-core -p conductor-verify --doc` → 0 · `cargo nextest run --workspace --profile ci` → **243✓ / 0 skipped (+30 from the 213 baseline)**.
- **Smoke:** skipped — pure library, no boot-path (the Epoch-8 CLI is not built).

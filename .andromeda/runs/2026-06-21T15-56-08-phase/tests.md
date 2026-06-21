# tests extract

## Relevance
partial — the chunk implements comparison/timing evaluation logic that feeds the existing `classify` function; core determinism and comparison patterns apply, but most critical paths and security vectors are orthogonal to outcome evaluation itself.

## Constraints
- Per §2 Test Strategy: determinism is enforced upstream (tokio `start_paused` virtual clock, seeded `conductor-timeline`, proptest `proptest-regressions/` persistence); same scenario + seed ⇒ same stream shape and same comparison outcome (§7 property-test trigger on determinism-replay).
- Per §3 Test Harness Contract, §Log format: latency measured **journal-relative** only — `read_back_observed_at − journal_emitted_at`, never wall-clock-from-test-start; stamps from `std::time::SystemTime`/`Instant` (NOT tokio virtual clock — security anti-pattern §11 Test Data bans virtual-clock leak into ground-truth artifacts).
- Per §5 Integration Test Strategy: expected-outcome block must be serde + garde-validated at load (matching the established config discipline), guard against out-of-range values (e.g., p50>p95>p99 ordering violations, error-rate band bounds) — integrates with §6 E2E Critical Path 1 end-to-end proof.
- Per §6 E2E Test Strategy, Critical Paths 1/2/3/4: the comparison result + timing outcome resolve into envelope `verdict`/`state`/`latency_ms` fields (Run-report envelope §3); mismatches ⇒ hard `Fail` (deterministic) or `CalibrationRegion` (sample-floor / model-interpretive).
- Per §10 Quality Gates, build-failure condition: the determinism invariant breach (same seed ⇒ different comparison outcome) is caught by golden-test insta redaction failures and surfaces as a test failure via §4 Unit Test Strategy canonicalization pattern (exact-string `assert_eq!` on serialization at unit level).

## Patterns to follow
- Per §4 Unit Test Strategy: exact-string `assert_eq!` unit goldens (matching `verdict.rs`/`report_state.rs`/`scenario.rs` serialization-golden pattern); insta reserved for E2E journal-golden mechanism with `run_id`/timestamp redaction §6/§7.
- Per §5 Integration Test Strategy: rstest `#[case]` table-driven over valid/invalid expected-outcome config matrix (parallel to the garde config-validation security-vector-coverage trigger §1 Scope Summary coverage trigger type #2).
- Per §7 Test Data & Fixtures: expected-outcome fixtures as checked-in declarative serde+garde config, not developer-created inline test data; scenario configs one-per-P-ID under `scenarios/` are the fixture library.
- Per §6 E2E Critical Paths 1–4: assertion on latency-relative timing (within slo_tier bound) feeds the exit-code/envelope verdict signal; the deterministic-replay property test (same seed ⇒ identical comparison outcome) is covered by insta golden on the JSONL journal redacting timestamps.

## Anti-patterns to avoid
- Per §11 Test Anti-Patterns § Unit: NEVER assert on a real wall-clock duration in a test — drive `tokio::time::advance` under `start_paused = true` and assert scheduled ordering/shape, not real elapsed time; journal-relative latency only (§3 Log format binds).
- Per §11 Test Anti-Patterns § Test Data: NEVER stamp the emission journal/report from tokio's virtual clock — use `std::time::SystemTime`/`Instant`; a paused-clock leak into the ground-truth artifact corrupts SLO checks (security anti-pattern).
- Per §11 Test Anti-Patterns § E2E: NEVER treat a `CalibrationRegion` state (model-interpretive, sample-floor miss) as a non-zero process exit — it is a reported envelope state, not a hard `Fail` exit (Creator Brief assertion-policy split, §1 Scope Summary critical path point 2).

## Contract bindings
- **obs ↔ tests harness** — Log format (§3) defines the per-run JSONL journal schema; obs derives its log-format binding from §3 (not the reverse). The emission journal is "the left side of every SLO check" (Creator Brief); the journal-relative latency (`read_back_observed_at − journal_emitted_at`) is computed here and stamped into the envelope; obs reads the result.
- **verdict-assertion-policy-split (prior chunk) ↔ classify** — the comparison result + timing outcome resolve into `matched: bool` + `ClaimClass` that the existing `classify(class, matched, observed, expected) → Assessment` consumes; wiring to `classify` is the chunk's final gate (scope §4 Wiring to classify).
- **5-command discipline ↔ status endpoint** — the Run-report envelope (with computed `latency_ms`, `slo_tier`, `verdict`, `state`) is read by the `status` command via `runs.db` row or JSONL journal (§3 5-command implementation, status bullet; no HTTP/IPC endpoint exists).

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-verify` passes for new comparison + SLO timing evaluation unit tests (fixture-driven, exact-string `assert_eq!` serialization goldens per §4 pattern).
- (tests) Property-test: same scenario + seed ⇒ identical comparison outcome + identical SLO tier outcome, asserted via insta golden redaction on the JSONL journal (§7 property-test trigger on determinism-replay, §6 E2E / golden snapshot pattern).
- (tests) SLO timing fixture matrix: valid bounds (tier-scaled tolerances <5s / <20s / <90s, journal-relative latency) pass; out-of-range (negative durations, inverted tier assignments) rejected at load via garde validation (§5 Integration trigger type §1 coverage trigger #2, security Vector 2).
- (tests) Comparison kind matrix: exact equality, membership, presence/absence, numeric ordering, count-floor all covered by table-driven `#[rstest]` cases; each kind maps correctly to `matched: bool` + `verdict`/`state`/`CalibrationRegion` outcome (§6 E2E Critical Paths 1–4 envelope assertions).

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling** (§4 Unit Test Strategy) — external-CLI tools (cargo-nextest, cargo-llvm-cov) are reference floors (any green install satisfies the gate); crate dev-deps caret-resolved with `Cargo.lock` authoritative. **Relevance:** this chunk's fixtures (rstest, proptest, insta) follow the same policy — exact-version resolved in `Cargo.lock`, floor logic applies to any external CLI tools the expected-outcome evaluator may invoke.
- **2026-06-16-emission-journal-writer** (§4 Unit Test Strategy, conductor-report bullet) — unit serialization goldens use exact-string `assert_eq!`; insta stays the E2E mechanism. **Relevance:** this chunk's unit comparison/timing logic follows the same golden pattern (exact-string at unit level for outcome serialization; insta for E2E journal redaction), ensuring determinism across re-runs.

# tests extract

## Relevance
Partial — the chunk's test scope overlaps with unit test patterns and fixture coverage, but does NOT trigger E2E/critical-path coverage.

## Constraints
- Per §4 Unit Test Strategy, all scenario validation and fixture round-trip tests use rstest table-driven `#[case]` rows over valid/invalid config matrices (test-plan §4: "rstest 0.26.1 … `#[rstest]` + `#[case]` for table-driven P-ID and garde-config matrices").
- Per §1 Coverage triggers: security-vector-coverage (negative-test) for scenario config parsing — garde must reject out-of-range config at load (error fraction ∈ [0,1], non-negative durations, p50≤p95≤p99 ordering violations, severity-mix sum violations) (test-plan §1 Trigger: "security-vector-coverage / property-test").
- Per scope.md §Definition of done: all checks `class="Hard"` with valid/invalid garde `#[case]` rows wherever the model is touched; the suppression *absence* leg decision (Q1) is documented for the Epoch-8 evaluator.
- Per §3 Test Harness Contract / Status endpoint shape: the Run-report envelope (verdict/state/latency_ms/slo_tier) MUST NOT leak absolute host paths or internal struct names — enforced by insta golden snapshot + tracing field-allowlist redaction (test-plan §3).
- Per §2 Test Strategy / agent-runnable invariants: determinism-under-seed is hard-enforced — same scenario+seed ⇒ identical emission-journal stream shape, via `#[tokio::test(flavor="current_thread", start_paused=true)]` + `tokio::time::advance` (test-plan §2).
- Per scope.md §Definition of done: determinism goldens re-baselined iff either TOML feeds the seed-named replay goldens (`crates/conductor-timeline/tests/snapshots/replay__fixture_seed_<N>.snap`) — grep new seed values, not just scenario names.

## Patterns to follow
- Scenario deserialization + garde-validation unit tests: mirror ch1–ch3 pattern of fixture round-trip tests proving each TOML deserializes via `Scenario::from_toml_str`, garde-validates, and produces valid `PhaseTimeline` through the existing scheduler (scope.md §Definition of done; test-plan §4 "what unit tests cover" / conductor-report golden pattern).
- Config-class validation: all P-013..P-016/P-057 checks declared `class="Hard"` per the Probabilistic-Assertion Policy (test-plan §1: "suppression/bypass logic" + activity-floor/lifecycle timing → hard pass/fail) — asserted in the golden as exact enum serialization (test-plan §4: "`verdict.rs`/`report_state.rs`/`scenario.rs` serialization-golden pattern").
- Determinism under seed (the amplitude constraint from scope.md §Long-timeline realism): declare spec-faithful durations here (P-013 ~90-min including 60-min cold-start + bursty training + 30-min lunch), proven via property-test golden snapshot under `start_paused` (no real-time elapsed); Epoch-8 owns runtime compression for live runs.

## Anti-patterns to avoid
- **Do NOT** express absence checks (P-013 no false silence, P-016(a) 15s burst suppressed, P-057(a) 8×/3% suppressed) without a decision in the scope notes — the ch1 `ComparisonKind` set is presence-only (`Contains`/`CountAtLeast`); the scope's Open Q1 decision (minimal `NotContains` vs. declare-only-positive deferred to Epoch-8) must be recorded in chunk notes so the evaluator knows whether the TOML asserts or only declares absence intent (test-plan §1 Coverage triggers: "compare via pinned manifest").
- **Do NOT** serialize the Run-report envelope with absolute host paths or internal seam-crate struct names (test-plan §3: "Artifacts MUST NOT leak…enforced by insta golden snapshot + tracing field-allowlist redaction").
- **Do NOT** let dev-dependency versions drift untracked — external-CLI tools (cargo-nextest, cargo-llvm-cov) are reference floors (test-plan §4 Tool-version policy); crate dev-deps are caret-resolved with `Cargo.lock` authoritative; any gate running green satisfies it (test-plan-amendments 2026-06-16).

## Contract bindings
- **tests ↔ obs §3 (harness):** the Run-report envelope (JSONL journal shape, per-run `<run_id>.jsonl` under `runs/`) is defined in test-plan §3 Log format and flows downstream to obs; the distinction between self-obs stream (service-identity + `run_id`, per obs-plan §3) and the per-run emission journal (verdict/state envelope, SLO ground truth) must not be conflated (test-plan-amendments 2026-06-15).
- **tests ↔ architecture (Probabilistic-Assertion Policy):** suppression/bypass logic + activity-floor/lifecycle timing are hard pass/fail (not calibration-region); severity-choice is CalibrationRegion (report-for-human, not hard-failed) — the TOML declares Hard checks; the model-interpretive severity (P-020/P-021) is later Epoch-7 (test-plan §1 Critical paths; scope.md §Requirement source of truth).

## Acceptance criteria contributions
- "(tests) `cargo nextest run -p conductor-core` passes all scenario deserialization + garde-validation fixtures for activity-floor + restart-suppression TOMLs (P-013..P-016/P-057)."
- "(tests) All `class=\"Hard\"` checks in activity-floor.toml + restart-suppression.toml assert via insta golden snapshot (verdict/state enum serialization, matching `scenario.rs` canonical pattern)."
- "(tests) Determinism golden snapshots for new seed values (if activity-floor.toml or restart-suppression.toml seeds feed `crates/conductor-timeline/tests/snapshots/replay__fixture_seed_<N>.snap`) re-baselined under `#[tokio::test(flavor=\"current_thread\", start_paused=true)]`."
- "(tests) Suppression absence-leg assertion method (minimal `NotContains` ComparisonKind vs. declare-only-positive deferred to Epoch-8) is decided + recorded in chunk notes for D-tests-coverage / Epoch-8 evaluator."

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling** (test-plan §4): external-CLI tool versions (cargo-nextest, cargo-llvm-cov) reframed as reference floors (not exact pins); dev-deps caret-resolved with `Cargo.lock` authoritative — relevant to fixture test selection and gate thresholds.
- **2026-06-16-emission-journal-writer** (test-plan §4): conductor-report golden pattern uses exact-string `assert_eq!` at unit level (verdict/state/scenario serialization), with insta reserved as E2E mechanism (run_id/timestamp redaction) — directly constrains how activity-floor + restart-suppression scenario goldens are locked.
- **2026-06-15-structured-logging-stack** (test-plan §3 Log format): self-obs stream (service-identity fields) is SEPARATE from per-run emission journal (verdict/state envelope) — no conflation in activity-floor/restart-suppression journal assertions.
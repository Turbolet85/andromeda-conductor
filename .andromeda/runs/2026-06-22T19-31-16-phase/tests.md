# tests extract

## Relevance
Relevant — the chunk adds scenarios spanning multiple P-IDs with mixed test classes (Hard + CalibrationRegion), requiring unit fixture validation, determinism assertions, and envelope checks.

## Constraints
- Per test-plan §1, scenario-config validation is testable via garde `range` + custom invariant fixtures (hard constraint on all P-019..P-023, P-059, P-060 TOML deserialize paths).
- Per test-plan §2, all test layers must be agent-runnable with machine-parseable output; no human visual-confirmation gates except operator-checklist items for P-019 visual UX (declare-only, not auto-tested).
- Per test-plan §4, unit-level golden lock on envelope serialization uses exact-string `assert_eq!`, not insta (conductor-report pattern); insta reserved for E2E journal golden with `run_id`/timestamp redaction.
- Per test-plan §3, the Run-report envelope must carry mixed `class`: `Hard` for lifecycle timing (P-022 120 s, P-023 cool-down, P-060 tier→SLO routing) and `CalibrationRegion` for model-interpretive legs (P-019/P-020 severity choice, P-059 interpretation continuity).
- Per test-plan §5, determinism property-test: same scenario+seed ⇒ same stream shape (fixture via `conductor-timeline` seeded generator, `#[tokio::test(flavor="current_thread", start_paused=true)]`).
- Per test-plan §6 critical path 4, severity-lifecycle full pass observes Pulse auto-resolve + resolution summary via MCP read-back; lifecycle timing asserted hard, severity *choice* asserted CalibrationRegion (report-for-human).

## Patterns to follow
- Fixture round-trip tests proving each `.toml` deserializes + garde-validates + builds a valid `PhaseTimeline` (mirrors ch1–ch5; scope Q7 zero-model-change).
- Mixed-class expected checks: `Hard` for P-022/P-023 timing + P-060 routing; `CalibrationRegion` for P-019/P-020 choice + P-059 continuity (first family carrying BOTH classes; scope Q5).
- Substring-tolerant token assertion via `Contains`/`Absent`/`CountAtLeast` (inferred from spec prose; LatencyRegression/RetryStorm precedent ch1–ch5).
- If any new TOML seed feeds the seed-named replay goldens (`crates/conductor-timeline/tests/snapshots/replay__fixture_seed_<N>.snap`), re-baseline only if seeds differ (expected UNCHANGED; goldens currently load only `error-baseline-spike.toml`).

## Anti-patterns to avoid
- Deserializing scenario config without garde validation at load (test-plan §1); all fixtures must validate range/ordering/sum invariants at parse time.
- Hard-failing on severity *choice* (model-interpretive, P-019/P-020) — only hard-fail on lifecycle timing (P-022 120 s, P-023 cool-down logic, P-060 routing); severity selection routes to `ManualCheck`/CalibrationRegion (arch Probabilistic-Assertion Policy).
- Human review gates in CI (no insta interactive accept, no visual regression with human sign-off); operator-checklist items (P-019 halo/counter/dropdown) are declare-only comments, not auto-tested.

## Contract bindings
- **tests ↔ obs** (test-plan §3) — per-run emission journal (`runs/<run_id>.jsonl`) JSONL with wall-clock stamps from `std::time::SystemTime`; self-obs stream is a separate artifact with service-identity fields per obs-plan §3.
- **tests ↔ arch** (test-plan §6 critical path 4) — MCP read-back (`query_incident_list`/`retrieve_report`/`mark_incident_resolved`) maps verdict/state per arch Standard Contracts (auto-resolve within 120 s, resolution summary, new-not-reopen); `KnownResidual` maps to `degraded_mode`.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes for new severity-lifecycle fixtures; round-trip asserts deserialize + garde-validate + build valid `PhaseTimeline` (per test-plan §3).
- (tests) Mixed-class expected-checks: both `Hard` and `CalibrationRegion` present in the suite; ≥1 invalid-fixture `#[case]` where the model is touched (per test-plan §1/§4).
- (tests) Determinism: severity-lifecycle scenarios with fixed seed reproduce identical stream shape (property-test via seeded `conductor-timeline` fixture under `start_paused`) (per test-plan §5).
- (tests) Envelope unit assertion: per-P-ID `verdict`/`state`/`slo_tier` correct via exact-string `assert_eq!` where applicable (per test-plan §4).

## Relevant amendment history
- 2026-06-15-structured-logging-stack (§3) — self-obs stream is separate from the emission journal; clarifies the binding, no envelope field change.
- 2026-06-16-emission-journal-writer (§4) — conductor-report unit goldens use exact-string `assert_eq!`, insta reserved for E2E journal golden with redaction. Applies to envelope assertions.

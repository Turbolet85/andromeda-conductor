# tests extract

## Relevance
Relevant — chunk delivers 4 scenario TOML configs (P-005..P-008) that extend the unit test & fixture layer per existing seam primitives; no new emit primitives or harness changes; full determinism + round-trip fixture coverage required.

## Constraints
- Per test-plan §4 Unit Test Strategy: scenario-config validation surface must use garde `#[case]` valid/invalid matrices + fixture round-trip proof that each TOML deserializes + validates + builds a valid `PhaseTimeline` per test-plan §7 (rstest `#[fixture]` + `#[case]`).
- Per test-plan §1 Coverage scope: every scenario carries P-ID keying; no scenario without a P-ID.
- Per test-plan §3 Test Harness Contract / Test data bootstrap: deterministic seeded synthetic generation via `conductor-timeline` (same scenario+seed ⇒ same stream shape); self-bootstrapping — no developer-seeded data.
- Per test-plan §1 Hard-signals definition: P-005/P-006/P-007 carry `class="Hard"`; P-008 carries `class="CalibrationRegion"` per the v2.1 amendment (routes to `ManualCheck`, never hard-failed).
- Per test-plan §4 Scenario-config validation: serde + garde `#[derive(Validate)]` range + cross-field invariants unit-tested with valid/invalid fixtures.
- Per test-plan §2 Agent-runnable invariants: machine-parseable output; gates run under `#[tokio::test(flavor="current_thread", start_paused=true)]` (no real wall-clock).

## Patterns to follow
- Per test-plan §4 Fixture pattern: rstest `#[rstest]` + `#[case]` table-driven rows over the catalog + garde valid/invalid matrices; `#[fixture]` for the seeded `conductor-timeline` generator.
- Per test-plan §7 Self-bootstrapping: declarative scenario config (serde + garde) one-per-P-ID under `scenarios/`; fixture files checked in as committed artifacts.
- Per test-plan §11 Anti-Patterns: assert on the Run-report envelope contract (verdict/state output), not implementation details or fixture-internal values; assert scheduled ordering/shape under `start_paused`, not real durations.
- Per test-plan §4 scope: scenario validation is testable via unit + fixture round-trip (no external service); P-005..P-008 reference only existing Epoch-3 emit primitives.

## Anti-patterns to avoid
- Per test-plan §11 Test Data: NEVER use non-deterministic generators without seed control; NEVER stamp the journal from tokio's virtual clock (`std::time` only); NEVER use production data.
- Per test-plan §11 Unit: NEVER assert on real wall-clock duration in a timeline test; NEVER test private fns/internal fields; NEVER write brittle assertions against fixture-internal values.
- Per test-plan §11 Universal: NEVER add a scenario without a Pulse P-ID.

## Contract bindings
- **tests ↔ obs (Log format)**: the per-run JSONL journal is DEFINED by test-plan §3 §Log format (JSONL via `tracing-subscriber` `format::Json`, fields `journal_emitted_at` from `std::time::SystemTime` + envelope identity `run_id`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`) — obs derives its envelope FROM this subsection.
- **tests ↔ core/emit**: P-005..P-008 reference existing Epoch-3 emit primitives (error-spans, exception events, severity logs, multi-service `parent_span_id` linking); no extension of `Scenario`/`ExpectedCheck` unless a comparison kind / claim class is genuinely missing.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core` passes for scenario-catalog fixture round-trip (P-005..P-008 TOML deserialize + garde validate + `PhaseTimeline` build).
- (tests) P-005/P-006/P-007 expected checks carry `class="Hard"`; P-008 carries `class="CalibrationRegion"` per the v2.1 amendment, routed to `ManualCheck` (not a hard-fail exit).
- (tests) Coverage: scenario-config validation with valid/invalid garde `#[case]` matrices per test-plan §7; line coverage ≥ 60% (test-plan §10 minimal threshold).
- (tests) Determinism: same scenario+seed ⇒ same stream shape, proven via `#[tokio::test(flavor="current_thread", start_paused=true)]` + `tokio::time::advance` (no real wall-clock assertion).

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling (§4):** external-CLI tool versions reframed as reference floors, not exact pins; crate dev-deps (rstest, proptest, insta, assert_cmd, assert_fs, predicates) caret-resolved with `Cargo.lock` authoritative. Applies: fixture tests use rstest floors + resolved `Cargo.lock` versions.

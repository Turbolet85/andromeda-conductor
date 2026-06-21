# tests extract

## Relevance
Relevant — the chunk delivers the verdict-classification logic that all subsequent verification checks consume, directly binding to unit/integration tests on the `conductor-verify` seam.

## Constraints
1. Verdict classification is a pure deterministic function (test-plan.md §2 agent-runnable invariants: "Deterministic — same inputs ⇒ same Verdict; no wall-clock read, no RNG inside classification itself").
2. Hard-path (Pass/Fail) assertions must use exact deterministic comparison against expected values (test-plan.md §1 coverage triggers: property-test on determinism-replay + golden tests asserting identical stream shape).
3. Calibration-region assertions must route to `Verdict::CalibrationRegion` on delta capture, never emit `Fail` on exact-value mismatch of model-interpretive claims (test-plan.md §1: "model-interpretive claims are 'calibration-region checks + report-for-human, never hard-failed'").
4. Verdict classification returns `Ok(Verdict)`; `Result::Err` is reserved for harness faults only — no classification error escapes as a verdict (test-plan.md §Surfaces: "Verdict/error wall: classification returns `Ok(Verdict)`").
5. Unit tests on verdict logic use rmcp in-process stub MCP server for state mapping assertions (test-plan.md §4: "`conductor-verify`: verdict logic + preflight state mapping (`ready:false ⇒ blocked`, `degraded_mode ⇒ KnownResidual`, empty canary ⇒ `blocked`) against an rmcp in-process stub").
6. Golden tests lock the canonical verdict serialization via exact-string `assert_eq!` at unit level, matching the `verdict.rs`/`report_state.rs` pattern (test-plan.md §4 amendment 2026-06-16: "canonical line shape is locked via exact-string `assert_eq!` at unit level").

## Patterns to follow
1. Unit-level verdict-classification tests use rstest `#[rstest]` with `#[case]` rows over the P-ID catalog (test-plan.md §4: "table-driven `#[rstest]` `#[case]` rows over the P-001..P-060 catalog"); seeded `conductor-timeline` fixture for deterministic input.
2. Cross-surface parity test asserts identical `verdict`/`state` envelope fields when the same scenario runs via CLI and Tauri paths (test-plan.md §6 critical path 7: "Tauri-launched run and the headless `conductor run` produce identical envelope verdict/state for the same scenario+seed").
3. State-mapping assertions (preflight gate + MCP read-back) use rmcp stub server to exercise `ready:false ⇒ blocked`, `degraded_mode ⇒ KnownResidual` transitions (test-plan.md §5: "verdict logic + preflight state mapping").

## Anti-patterns to avoid
1. Do NOT hard-fail a model-interpretive claim on exact-value mismatch — only hard-path (deterministic) assertions produce Pass/Fail; model claims route to CalibrationRegion (test-plan.md §11 E2E: "NEVER treat a `blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` state as a non-zero process exit").
2. Do NOT include wall-clock reads or RNG inside the verdict-classification logic itself — determinism is enforced via seeded inputs (test-plan.md §2: "Deterministic — same inputs ⇒ same Verdict; no wall-clock read, no RNG inside classification itself").
3. Do NOT fake Pulse's reaction as a CI verdict in classification tests — rmcp stub canary proves wiring only; live Pulse behavior is local-gate-only (test-plan.md §11 Test Strategy: "NEVER fake Pulse's *reaction* as a CI verdict").

## Contract bindings
- **Verdict envelope ↔ report**: `verdict` field (Pass/Fail/CalibrationRegion) serializes into the Run-report envelope (`runs.db` row + JSONL journal + Markdown) — shared contract per test-plan.md §3 Status endpoint shape.
- **Verdict classification ↔ state mapping**: the verdict-classification module consumes the preflight readiness signal (`ready:true/false`) and MCP `degraded_mode` reads to map state transitions (`ready:false ⇒ Blocked`, `degraded_mode ⇒ KnownResidual`), binding to the `conductor-verify` preflight gate (test-plan.md §5 coverage patterns).

## Acceptance criteria contributions
1. (tests) `cargo nextest run -p conductor-verify` passes all verdict-classification unit tests, including state-mapping assertions for the hard-path (Pass/Fail) and calibration-region transitions.
2. (tests) Hard-path (Pass/Fail) assertions on deterministic claims (timing, suppression logic, lifecycle ordering) produce exact-string golden-lock via `assert_eq!` (test-plan.md §4 amendment 2026-06-16).
3. (tests) Model-interpretive assertions (severity choice, hypothesis quality) route to `Verdict::CalibrationRegion` and are asserted as present + delta-captured in the envelope, never as `Fail` exits (Creator Brief assertion-policy split per test-plan.md §1).
4. (tests) Cross-surface parity test asserts `verdict` field identity when the same scenario+seed runs via CLI and Tauri mock-runtime, both reading from the same `assert_fs::TempDir` file `runs.db` (test-plan.md §6 critical path 7).

## Relevant amendment history
- **2026-06-16-emission-journal-writer**: Clarified that unit serialization goldens use exact-string `assert_eq!` for canonical shapes (verdict.rs / report_state.rs pattern), with insta reserved for E2E journal-golden mechanism (run_id/timestamp redaction). Routine per playbook rule #5 (spec-illustration → sound-impl alignment). No envelope change.

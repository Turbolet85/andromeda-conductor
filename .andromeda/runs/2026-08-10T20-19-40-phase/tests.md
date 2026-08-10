# tests extract

## Relevance
Relevant — the chunk adds a `contracts/` manifest loader (unit-tier bounds-check surface) and a new named preflight `Blocked` precondition (integration-tier stub-MCP surface), both squarely inside the CI-runnable test tiers.

## Constraints
- Test tier is **Minimal (0)**, augmented by triggers — not a Comprehensive escalation; breadth here is per-seam unit tests plus the rmcp-stub integration leg, not new E2E paths (per test-plan §1 Test Scope Summary).
- The live-Pulse leg is **operator/local gate only, never a CI gate**; every assertion this chunk adds must be CI-runnable against the rmcp stub server (in-process duplex / `TokioChildProcess`), with the real sidecar leg gated on `ANDROMEDA_PULSE_MCP_ENABLED` (per test-plan §9 Live-Pulse scenarios + §6 driver table).
- `ready:false` must produce `state="Blocked"` with a **named precondition string**, `Ok` not `Err`, and never a silent downgrade to pass/fail/manual-check; only a true harness fault is `Err` (per test-plan §3 `boot`).
- Artifacts and precondition strings must not leak **absolute host paths or internal seam-crate struct names** — asserted by a dedicated negative test, matching the existing loader negative tests (per test-plan §3 Status endpoint shape + §11 Test Data).
- New contract manifest is a unit-tier surface: **load + bounds-check + missing/malformed = harness fault, never a silent default**, tested with valid/invalid fixtures, in-crate `#[cfg(test)] mod tests` under `conductor-core` (per test-plan §4 What unit tests cover / Scenario-config validation surface).
- `Blocked` is a **reported envelope state, not a non-zero process exit** — no test may assert a failing exit code for an unmet contract term (per test-plan §11 E2E, stack-specific ban).
- Coverage gate binds: workspace line coverage ≥ 60% via `cargo llvm-cov nextest --fail-under-lines 60`, zero-retry flakiness budget, `cargo audit --deny warnings` + `cargo deny check` green (per test-plan §10 Quality Gates).

## Patterns to follow
- **Committed-contract loader test set** — `crates/conductor-core/src/load_envelope.rs` `mod tests` is the direct precedent for a new `contracts/` manifest: `loads_and_bounds_checks_the_committed_envelope`, `missing_file_is_a_harness_fault`, `load_failure_message_never_contains_the_path`, plus per-field rejection cases (per test-plan §4 Scenario-config validation surface).
- **Host-path-free message assertion** — `envelope_message_names_identity_without_host_paths_or_type_names` in the same file is the shape for asserting a new `blocked_precondition` string carries identity but no path/type name (per test-plan §3 Status endpoint shape).
- **rmcp stub over stdio for the preflight leg** — in-process duplex for verdict/state mapping, `TokioChildProcess` test-binary for the spawn/`.env()` path; existing legs live in `crates/conductor-verify/tests/preflight.rs` and `preflight_spawn.rs` (per test-plan §5 Boundary types + §8 Mocking table).
- **Exact-set-equality both-directions gate** — `check_scenario_backing` in `crates/conductor-core/src/drift.rs` (tests at lines ~305-372) is the existing mechanic if `P-073` leaves `UNBACKED_AUTO`; the pin edit and the scenario must land together or one direction of the gate fails (per test-plan §6 Coverage-matrix completeness gate, scenario-backing leg).
- **Fixture discipline** — rstest `#[fixture]`/`#[case]` table rows for the valid/invalid contract-term matrix; committed declarative config as the fixture, no developer-seeded DB (per test-plan §7 Seed strategies).

## Anti-patterns to avoid
- NEVER fake Pulse's *reaction* as a CI verdict — the stub canary proves MCP wiring only; a contract term Conductor cannot measure must surface as a named precondition, not a stubbed "measurement" (per test-plan §11 Test Strategy, stack-specific).
- NEVER treat `Blocked` as a non-zero process exit, and never let a failed preflight silently downgrade (per test-plan §11 E2E + §3 `boot`).
- NEVER interpolate operator values (data-dir, contract terms) into sidecar argv/shell — `.env(...)` only, fixed hard-coded sidecar path; negative test asserts argv-injection rejection (per test-plan §11 Mocking, stack-specific).

## Contract bindings
- **tests ↔ obs (log format):** any new `Blocked` precondition surfaced in the journal must stay one-line JSONL with the envelope identity fields and no host paths; test-plan §3 Log format is the schema owner and obs derives from it — the redaction/field-allowlist half is obs's, asserted here by a negative test.
- **tests ↔ arch/verify (preflight contract):** the named-precondition set grows from four to five; `ReadyState`/`blocked_precondition` shape and `Ok(Blocked)` not `Err` are fixed by test-plan §3 `boot` + arch §Standard Contracts.
- **tests ↔ core (envelope):** `ReportState` stays five and `LAMP_META` stays six — no new state/lamp may be introduced to carry an unmet term (per test-plan §1 cli Signal + §3 Status endpoint shape).
- **tests ↔ security (supply chain):** the folded-in `cargo audit` re-check is the same CI gate test-plan §9/§10 name — record the result, verify `cargo deny check` green as the overlapping signal.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-core -p conductor-verify` passes; the new contract loader has load / missing-file-is-harness-fault / bounds-rejection cases and preflight has a case per new unmet term (per test-plan §4 What unit tests cover, §5 MCP initialize preflight readiness gate).
- (tests) A negative test asserts the new `Blocked` precondition string and any load-failure message contain no absolute host path and no internal struct/type name, and that the unmet term yields `Ok(ReadyState{ready:false, state:"Blocked"})` with exit code 0 — never `Err`, never a hard `Fail` exit (per test-plan §3 boot / Status endpoint shape, §11 E2E).
- (tests) The preflight assertion runs green in CI against the rmcp stub with no live-Pulse dependency; any live-sidecar leg stays `ANDROMEDA_PULSE_MCP_ENABLED`-gated (per test-plan §9 Live-Pulse scenarios).
- (tests) Workspace gates green with no new retries: `cargo llvm-cov nextest --fail-under-lines 60`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo audit --deny warnings` + `cargo deny check`, committed un-drifted `Cargo.lock` (per test-plan §10 Quality Gates, §11 CI).

## Relevant amendment history
- **2026-08-09-interpretation-correctness-posture** (§5 GUI deferral · §6 Coverage-matrix completeness gate) — added the scenario-backing leg: exact-set equality in both directions over `UNBACKED_AUTO`, `Auto` only, unit-tier and CI-runnable. Directly governs this chunk's CARRY 2 (`P-073` retiring from the pin must land in the same commit as its backing scenario).
- **2026-08-09-in-lane-sut-scenarios** (§6 Selector strategy) — de-hardcoded the P-ID range to "the manifest's accepted set"; precedent that any new assertion touching P-073 must name the set, not a literal range or count.
- **2026-08-09-sut-load-envelope** (§1 cli Signal · §6 E2E driver row + Selector strategy) — recorded `[ENVIRONMENT-SUSPECT]` as a run-level non-lamp qualifier alongside the closed six-label lamp set; relevant because Term E keeps this chunk's prescribed storm inside `contracts/pulse-load-envelope.toml`, whose breach prints that qualifier.
- **2026-08-08-sut-capability-manifest** (§1 Scenario catalog · §4 · §6 · §7) — catalog/coverage assertions re-sourced from the SUT capability manifest instead of a hard-coded 60; the de-hardcode precedent a new `contracts/` manifest loader should follow.
- **2026-06-16-emission-journal-writer** (§4 conductor-report bullet) — unit serialization goldens use exact-string `assert_eq!`; insta stays the E2E mechanism. Applies if this chunk golden-locks any new contract or precondition serialization at unit level.

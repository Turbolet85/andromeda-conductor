# tests extract

## Relevance
Relevant — this chunk (`2026-06-23-conductor-run-suite-report-verbs`) builds the CLI verbs that are the release gate and execute scenarios covered by tests.

## Constraints
- Per test-plan §3, the 5-command `run` verb must invoke `cargo nextest run --workspace --profile ci` + `cargo test --workspace --doc` + `cargo clippy` at the gate level, and scenario invocation via `conductor run <scenario|P-ID> --seed <s>` must exit 0 on Pass, non-zero only on hard Fail (no non-zero exit for Blocked/ManualCheck/KnownResidual/CalibrationRegion — assertion-policy split per test-plan §1).
- Per test-plan §2, `conductor run` and `conductor suite` produce Run-report envelope JSONL/`runs.db` rows that are assertion-parseable by agents reading exit codes + structured envelope fields (verdict/state/latency_ms/slo_tier).
- Per test-plan §3, the 5-command `run` verb must wire the harness contract: bootstrap with `conductor_core::init_observability("conductor", …)` FIRST, capture JSONL journal + Markdown report + `runs.db` rows under `runs/<run_id>.*`, assert `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` ASCII status labels (test-plan §3 Surfaces cli note: "survive NO_COLOR/piping").
- Per test-plan §4 Unit Test Strategy, error handling surface must test the sanitization invariant (security: "Exposing internal error details ... rejected") on the anyhow edge + report artifacts (no host paths, no internal struct names per test-plan §3 Status endpoint shape).
- Per test-plan §5, cross-surface parity path asserts the Tauri-launched run and headless `conductor run` produce identical envelope verdict/state/seed for the same scenario+seed — both surfaces must resolve scenarios from `scenarios/` and apply the same clock+scheduling model.
- Per test-plan §6 E2E cli scenario (headless deterministic run), `assert_cmd` covers `conductor run <P-ID> --seed <s>` + exit code + label assertions + `runs.db` row existence; same seed re-run must yield identical stream shape (insta golden on the journal, redacting run_id/timestamps).

## Patterns to follow
- Use cargo-nextest 0.9.137 as the test runner with JUnit XML output per test-plan §4; the CLI binary is asserted via `assert_cmd` 2.1.2 `Command::cargo_bin("conductor")` with rstest/proptest fixtures for scenario+seed variants per test-plan §6.
- The Run-report envelope (verdict/state/latency_ms/slo_tier/fingerprints fields per test-plan §3) is the canonical contract serialized identically into JSONL journal + `runs.db` row + Markdown report; unit serialization goldens use exact-string `assert_eq!` (test-plan §4 amendment: conductor-report bullet), with insta reserved as the E2E journal-golden mechanism (redacting run_id/timestamps, test-plan §7).
- Scenario config resolution uses serde+garde (test-plan §1 entity: scenario-config validation surface); invalid configs are unit-tested to assert garde rejects out-of-range values per test-plan §5 security-vector-coverage trigger 2.
- Environment-handle canonicalization (`CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SEED`) via `std::fs::canonicalize` + bounds-check at the CLI edge before any read/write per scope.md §Surfaces / test-plan §5 Vector 1 negative-test.

## Anti-patterns to avoid
- NEVER test implementation details (test-plan §11 Unit ban); assert on the public envelope contract + observable behavior (exit code/labels/`runs.db` state).
- NEVER assert real wall-clock duration in a timeline-driven scenario (test-plan §11 Unit ban); drive `tokio::time::advance` under `start_paused = true` and assert scheduled ordering.
- NEVER treat blocked/ManualCheck/KnownResidual/CalibrationRegion as non-zero process exits (test-plan §11 E2E ban) — those are reported envelope states; only hard Fail is a non-zero exit per test-plan §1 exit-code semantics.
- NEVER inject `ANDROMEDA_PULSE_DATA_DIR` into sidecar argv/shell (test-plan §11 Mocking ban / Vector 4 negative-test); pass only via `.env()` after rejecting metacharacters; hard-code the sidecar path.

## Contract bindings
- **Harness ↔ tests:** test-plan §3 5-command discipline binds to the 5-command harness (boot/run/status/cleanup/logs); the `run` verb wires the test gate and scenario invocation; `status`/`logs` read the envelope/JSONL artifacts produced by `run`.
- **Obs ↔ tests:** test-plan §3 Log format (JSONL + required fields per Run-report envelope) is THE SOURCE OF TRUTH; obs-plan derives its product-side envelope FROM this subsection; the binding is tested via negative-test asserting no host paths/struct names leak into the journal.
- **CLI ↔ core seams:** the chunk wires already-built engine seams (timeline/emit/verify/report) per scope.md; no new engine logic added — only CLI-crate-local clap structs + handlers + bootstrap.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-cli` passes for new CLI unit tests; `assert_cmd` E2E in `tests/cli_smoke.rs` covers help/exit-codes/run/suite/report verbs with rmcp stub for read-back leg.
- (tests) Coverage: CLI binary paths (clap parsing, scenario resolution, envelope production) maintain ≥60% line coverage (Minimal tier test-plan §10).
- (tests) E2E path P1 (headless deterministic run with MCP read-back) passes: `conductor run error-baseline-spike --seed <s>` exits 0, `runs.db` row has `verdict=Pass`/`state=Pass`, JSONL journal exists + parses to envelope, same-seed re-run yields identical stream shape per insta golden.
- (tests) Envelope production is sanitized: no host paths, no internal struct names in JSONL journal + Markdown report + `runs.db` artifacts (asserted by golden test per test-plan §5 artifact-sanitization trigger).

## Relevant amendment history
- **2026-06-16-test-framework-fixtures-coverage-tooling** (test-plan §4): external-CLI tool versions (cargo-nextest, cargo-llvm-cov) reframed as reference floors, not exact pins — any green-running install satisfies the gate per cargo-audit/deny precedent; crate dev-deps caret-resolved with `Cargo.lock` authoritative. Applies: CLI chunk resolves its own nextest/clippy/doctest versions; if they pass green, they satisfy the floor.
- **2026-06-16-emission-journal-writer** (test-plan §4): unit serialization goldens use exact-string `assert_eq!` (matching established `verdict.rs`/`report_state.rs`/`scenario.rs` pattern), with insta reserved as E2E mechanism (run_id/timestamp redaction, §6/§7). Applies: the CLI chunk must adopt this pattern for envelope serialization unit tests in `conductor-report` integration; E2E scenarios use insta golden on the JSONL journal.
- **2026-06-17-raw-otlp-message-scaffold** (test-plan §2 Integration row): OTLP-egress loopback gRPC `TraceService` stub (tokio-stream + tonic on ephemeral `127.0.0.1:0`) registered as integration mechanism. Applies: the chunk's E2E smoke may exercise egress via this stub (CI-runnable without live Pulse); actual `:4317` egress is gated on live Pulse leg.
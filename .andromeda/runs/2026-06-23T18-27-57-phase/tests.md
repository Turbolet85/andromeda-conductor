# tests extract

## Relevance
Relevant — chunk delivers the 5-command harness discipline, the central test-plan §3 contract that gates all downstream test execution and agent-driven CI.

## Constraints
- Per test-plan §3 Test Harness Contract: implement exactly 5 commands (`boot` = preflight gate, `run` = cargo-nextest + scenario invocation, `status` = disk-artifact read, `cleanup` = idempotent teardown, `logs` = emission journal surface) with both `.sh` and `.ps1` parity (test-plan §3 / §9 CI Integration).
- Per test-plan §3 / scope.md: the harness is orchestration + artifact-read only — no re-implementation of engine logic, no Pulse process management, no new inbound listener binding beyond the `:4317` port-occupier fault.
- Per test-plan §3 / 5-command implementation: `boot` preflight gate must assert MCP `initialize` handshake (protocol `2024-11-05`, required-tool presence, canary round-trip); must surface `ready:true` or `Blocked` with named precondition — never silently downgrade a failed preflight (security Vector 4 / anti-pattern §Mocking).
- Per test-plan §3 / exit-code contract: `run` exits 0 on all checks Pass, non-zero on hard Fail; `blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are reported envelope states, NOT process-failure exits (Creator Brief assertion-policy split, test-plan §6 E2E anti-pattern).
- Per test-plan §3 / Log format: structured JSONL — one JSON object per line, emitted by `tracing-subscriber` JSON; required fields include envelope identity (`run_id`, `seed`, `scenario`, `p_ids`, `verdict`, `state`, `latency_ms`, `slo_tier`, `fingerprints`) — NO absolute host paths, NO internal struct names (security anti-pattern, asserted by negative test).
- Per test-plan §9 CI Integration: the `run` command must accept mutually-exclusive `--unit` / `--integration` / `--e2e` flags to split pipeline into parallel stages (nextest-only, stub-legs, webview-legs respectively); with no flag, full aggregate runs.

## Patterns to follow
- Per test-plan §2 Agent-runnable invariants: all test output must be machine-parseable — cargo-nextest `ci`-profile JUnit XML + `NextestExitCode` enum (exit codes 100/101 for test failure, 4 for no-tests); envelope/journal asserted via `serde_json::from_str` or `jq -e` (shell-level smoke gate); deterministic — no real network beyond loopback stubs, no real time without tokio `start_paused` injection.
- Per test-plan §3 / Status endpoint shape: Run-report envelope JSON serializes identically into Markdown report, `runs.db` row, and each JSONL journal line; agent reads by `jq -e` predicate on JSONL or bound-parameter SQL on `runs.db`, never via HTTP endpoint (no listener exists).
- Per test-plan §3 / cleanup: must be idempotent — re-running against a clean state is a no-op (file-absence is not an error, `DELETE` of zero rows succeeds); the `:4317` port-occupier must release its bind on cleanup (verified by `TcpListener::bind("127.0.0.1:4317")` succeeding post-cleanup).

## Anti-patterns to avoid
- Per test-plan §11 CI: NEVER run live-Pulse scenarios as a CI gate — operator/local `workflow_dispatch` only (the rmcp stub-leg CI is fully runnable; live Pulse leg is architecture-documented as local-gate-only per arch CI/CD note).
- Per test-plan §11 Mocking: NEVER interpolate `ANDROMEDA_PULSE_DATA_DIR` (or any operator value) into sidecar argv/shell — pass strictly via `.env(...)` after rejecting injection metacharacters; spawn `andromeda-pulse-mcp` as a fixed hard-coded path (rmcp STDIO injection CVE-2026-30623 / security Vector 4); negative test asserts this boundary (test-plan §1 coverage-triggers).
- Per test-plan §11 Universal (agent-driven specific): NEVER include a manual smoke step or human visual review of artifacts; all verification must be agent-drivable (exit codes, artifact shape, parsed JSON, DB rows).

## Contract bindings
- **5-command discipline ↔ obs §3**: `boot` returns JSON `{"ready": true}` (or `false`); `status` reads Run-report envelope (shared by Markdown + `runs.db` + JSONL); `logs` surfaces per-run JSONL journal. Both the harness 5-command structure and the status-endpoint shape are defined here (test-plan §3) and flow downstream to obs for envelope greps and field-allowlist redaction.
- **Log format ↔ obs §Log format binding**: the `tracing-subscriber` JSON log format (§3 Log format) is the source of truth and obs derives its product-side log envelope FROM this subsection; the field-allowlist / host-path redaction layer is owned by obs downstream; test-plan asserts the boundary via a negative test.
- **OTLP egress liveness check ↔ conductor-emit integration**: `boot` preflight includes a transport-connectability probe (optional turmoil deterministic partition/refusal injection or a loopback gRPC stub); a refused transport surfaces as `Result::Err`, NOT a verdict (security Vector 5).

## Acceptance criteria contributions
- (tests) `scripts/agent-run.sh boot` and `scripts/agent-run.ps1 boot` both report `{"ready": true}` or `{"ready": false}` + named precondition without emitting scenarios.
- (tests) `scripts/agent-run.sh run --unit` / `--integration` / `--e2e` flags split the pipeline stages as per §9 table; with no flag, full aggregate runs; exit 0 on all checks Pass, non-zero only on hard Fail.
- (tests) `scripts/agent-run.sh status` reads `runs.db` row via bound-parameter SQL; `scripts/agent-run.sh logs` tails per-run JSONL journal; both run without a live Pulse.
- (tests) `scripts/agent-run.sh cleanup` is idempotent (repeat-safe, file-absence check + zero-row `DELETE`, `:4317` port-occupier release verified); both `.sh` and `.ps1` carry identical semantics per scope.md val-1 anchor.

## Relevant amendment history
- **2026-06-15 (structured-logging-stack)**: clarified that the `tracing` self-observation stream (stderr / `logs/agent-latest.jsonl`) is a SEPARATE artifact from the per-run emission journal (`runs/<run_id>.jsonl`, the SLO ground truth + Run-report envelope §3 owns). This amendment confirms the §3 ↔ obs-plan §3 binding is unchanged — the Log format subsection owns the envelope shape; obs derives its schema FROM test-plan §3, not the reverse.
- **2026-06-16 (test-framework-fixtures-coverage-tooling)**: reframed external-CLI tool versions (`cargo-nextest`, `cargo-llvm-cov`) as reference floors (outside `Cargo.lock`; any green-running install satisfies the gate, per cargo-audit precedent) rather than exact pins. Clarifies that the harness implementation may resolve tool versions dynamically so long as CI gates pass green.

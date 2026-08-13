# tests extract

## Relevance
Partial-relevant — the chunk drives the §3 `boot` preflight command on its live/operator leg; tests owns the harness contract, the live-vs-CI gate boundary, and the tier/mechanism for pinning any key-diff finding. It adds no CI test surface.

## Constraints
- The live-Pulse leg is an operator/local gate reached via `workflow_dispatch` or `scripts/agent-run.sh` only — never a CI gate, and no existing gate may be made to depend on a running Pulse (per test-plan.md §9 CI Integration → Live-Pulse scenarios; §11 CI).
- `boot` contract is fixed: `conductor preflight --json`, readiness signal stdout `{"ready": true}`; `ready:false` ⇒ every dependent scenario reports `state="Blocked"` carrying the named precondition string, never silently downgraded; only handshake/transport failure (a harness fault) is a non-zero exit (per test-plan.md §3 5-command implementation → `boot`).
- The live poll budget is `CONDUCTOR_PREFLIGHT_TIMEOUT` (default 30s) **raised** to `contracts/pulse-run-contract.toml` `[incident_formation].min_canary_poll_seconds` — it raises, never lowers; the stub leg stays <1s on `CanaryPoll::immediate()` (per test-plan.md §3 `boot` → Timeout).
- An empty or malformed canary / read-back result must become `Blocked`, never a false pass-as-empty — this is exactly the silent failure mode the key-diff pre-check exists to catch (per test-plan.md §5 Cross-module patterns → query_incident_list / retrieve_telemetry_slice; §1 coverage trigger Vector 4(c)).
- Pulse's *reaction* may not be faked as a CI verdict: a live observation cannot be folded back into the stub as an assertion of Pulse behavior — the stub canary proves MCP wiring only (per test-plan.md §8 What NOT to mock; §11 Test Strategy).
- Every recorded artifact (readiness envelope, journal, evidence file) must carry no absolute host path and no internal seam-crate struct name; `data_dir` stays redacted — asserted by a negative test, not by convention (per test-plan.md §3 Status endpoint shape + Log format).
- The standing gates stay binding through this chunk: `cargo llvm-cov --fail-under-lines 60`, zero-retry flakiness budget (no nextest `retries`), and `cargo audit --deny warnings` + `cargo deny check` green against this chunk's own lockfile delta (per test-plan.md §10 Build failure conditions; §9 Supply-chain audit row).

## Patterns to follow
- Reuse the existing hand-rolled JSON-RPC duplex stub harness at `D:\dev\projects\conductor\crates\conductor-verify\tests\common\mod.rs` (`StubConfig` / `serve_stub`) for any CI-runnable assertion derived from the live findings — one live shape becomes a stub case, not a new live dependency (per test-plan.md §5 Driver(s); §8 Mocking libraries).
- Add per-precondition integration tests in the established style of `D:\dev\projects\conductor\crates\conductor-verify\tests\preflight.rs` — `#[tokio::test(flavor = "current_thread")]`, one snake_case test per named precondition (the file already holds `an_empty_canary_names_the_workspace_key_precondition` and `a_blocked_precondition_carries_no_absolute_host_path`) (per test-plan.md §4 Conventions; §5).
- Pin a key-shape agreement at the tier the plan already uses: exact-string `assert_eq!` for canonical serialization shapes at unit tier, with insta reserved as the E2E/journal golden mechanism (committed, CI fail-don't-write, never `cargo insta review`) (per test-plan.md §7 Golden artifacts; §4 conductor-report bullet).
- Read evidence through the existing `status` / `logs` mechanisms — `runs/<run_id>.jsonl` parsed by `serde_json`/`jq -e` plus a bound-parameter `SELECT` on `runs.db` — rather than a new evidence reader; `runs.db` SQL is never `format!`-built (per test-plan.md §3 `status` / `logs`; §11 Integration).
- Sandbox any CLI-driving test with `assert_fs::TempDir` + `.env("CONDUCTOR_RUNS_DIR", …)` and keep `cleanup` idempotent (per test-plan.md §3 Test data bootstrap → Per-test isolation; §3 `cleanup`).

## Anti-patterns to avoid
- Never turn a reported state into a process failure: a live blocked preflight is `Ok(Blocked)` with its named precondition — `Blocked`/`ManualCheck`/`KnownResidual`/`CalibrationRegion` are envelope states, not non-zero exits or panics (per test-plan.md §11 E2E, stack-specific).
- Never introduce a real-network / live-Pulse dependency into any CI-run test or required workflow step (per test-plan.md §11 CI + §11 Universal).
- Never interpolate `ANDROMEDA_PULSE_DATA_DIR` (or any operator value) into the sidecar argv/shell — pass strictly via `.env(...)` with the program name fixed/hard-coded (per test-plan.md §11 Mocking, stack-specific).

## Contract bindings
- **tests §3 Log format ↔ obs-plan §3** — test-plan.md §3 is the source of truth for the JSONL emission-journal schema; obs owns the field-allowlist/redaction layer (`conductor-core::redact`) that keeps host paths and struct names out. Live evidence assertions cross that boundary and must not conflate the self-obs stream (`logs/agent-latest.jsonl`, `logs/conductor-tauri.jsonl`) with the per-run emission journal `runs/<run_id>.jsonl`.
- **tests §9 Supply-chain audit row ↔ security-plan §Dependency Security** — the chunk's thirteenth `cargo audit` re-run + `cargo deny check` is a tests-owned CI/quality gate; tool versions are floors, not pins (test-plan.md §4 Tool-version policy).
- **tests §3 `boot` ↔ arch Standard Contracts / `contracts/pulse-run-contract.toml`** — the negotiated protocol `2024-11-05`, the pinned required-tool manifest, and the `min_canary_poll_seconds` floor are contract-owned inputs the harness asserts, not test-local literals.

## Acceptance criteria contributions
- (tests) `cargo nextest run -p conductor-verify` and the workspace `--profile ci` leg pass, and no new or modified test requires a live Pulse to run green in CI (per test-plan.md §9 CI Integration → Live-Pulse scenarios).
- (tests) The live leg is invoked through `scripts/agent-run.{sh,ps1} boot` with identical semantics on both shells; a `ready:false` result prints its named precondition and yields `Ok(Blocked)` — no panic, no non-zero exit for a blocked precondition (per test-plan.md §3 5-command implementation → `boot`).
- (tests) The key-diff outcome is pinned in code: agreement frozen by a committed exact-assert/golden case reusing the stub harness, divergence landed as a finding plus a test — never a silent adjustment of `extract.rs` readers, and never `cargo insta review` (per test-plan.md §7 Test Data & Fixtures → Golden artifacts).
- (tests) A negative test asserts every recorded readiness/journal artifact carries no absolute host path and no internal struct name (`data_dir` redacted), and the standing gates stay green: `--fail-under-lines 60`, zero retries, `cargo audit`/`cargo deny` (per test-plan.md §10 Quality Gates & Coverage Targets).

## Relevant amendment history
- **2026-08-10-pulse-run-contract** (§3 → `boot`): the flat 30s budget became `CONDUCTOR_PREFLIGHT_TIMEOUT` raised to the run contract's `[incident_formation].min_canary_poll_seconds` floor, because the chunk made that floor real in `canary_poll()` and a spec stating 30s described a value the live leg would not honour. This is precisely the mechanism this chunk measures live for the first time.
- **2026-08-13-dispatcher-determinism-goldens** (§7 Golden artifacts): golden inventory restated as families × seed with the never-`cargo insta review` rule made explicit — governs how a key-diff agreement may be frozen.
- **2026-06-16-emission-journal-writer** (§4 conductor-report bullet): unit-tier serialization goldens use exact-string `assert_eq!`; insta stays the E2E journal-golden mechanism with `run_id`/timestamp redaction — sets the tier for a key-shape pin.
- **2026-06-15-structured-logging-stack** + **2026-08-10-scenario-run-root-span-tree** (§3 Log format): the self-obs stream is a distinct artifact from the emission journal, and now carries two line variants (event line, span-lifecycle line with `span`/`span_event`/`parent`) gated by the same redaction allowlist — relevant to reading live-run evidence without conflating the two schemas.

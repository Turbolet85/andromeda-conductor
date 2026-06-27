# tests extract

## Relevance
Relevant — E2E integration of live Pulse with MCP read-back verification; covers cli + ipc-internal + persistent-storage surfaces via 5 existing scenario families and 4 critical paths.

## Constraints
1. Test tier Minimal (0) augmented with coverage triggers (property-test for determinism, contract-test for MCP, cross-surface-coordination for parity, chaos-test for faults) per §1 Scope Summary.
2. Agent-runnable invariants apply: machine-parseable output (NextestExitCode, JSONL/jq), no human-in-loop, deterministic execution, self-bootstrapping fixtures per §2 Test Strategy.
3. 5-command discipline (boot = preflight MCP initialize + canary round-trip; run = scenario invocation; status = runs.db row/JSONL read; cleanup = rm artifacts + DELETE rows; logs = JSONL journal emit) per §3 Test Harness Contract.
4. Integration boundaries: MCP read-back via rmcp stub (CI) / live Pulse (local gate), runs.db via rusqlite bound-parameter SQL, cli via assert_cmd subprocess per §5 Integration Test Strategy.
5. E2E critical paths 1–5 via cli + ipc-internal surfaces (headless deterministic run + fingerprint-storm + restart-suppression + severity-lifecycle + known-residual) per §6 E2E Test Strategy.
6. Log format binding: emission journal (runs/<run_id>.jsonl) JSONL with required fields distinct from self-obs stream (logs/agent-latest.jsonl + logs/conductor-tauri.jsonl) per §3 Test Harness Contract + amendments 2026-06-15-structured-logging-stack.
7. Live-Pulse scenarios as explicit **operator/local gate via `workflow_dispatch`, NOT CI** (CI uses rmcp stub) per §9 CI Integration.

## Patterns to follow
1. rmcp 1.7.0 stub server over stdio (in-process duplex for verdict logic; TokioChildProcess test-binary for spawn/`.env()` injection-rejection path) per §5 Module ↔ external (MCP sidecar).
2. assert_cmd 2.1.2 + predicates 3 + assert_fs 1 for cli subprocess driver (exit code + NO_COLOR-stable `[PASS]`/`[FAIL]`/`[BLOCKED]` labels + runs.db row check) per §6 cli surface driver.
3. assert_fs::TempDir for cross-process file DB isolation per run per §7 Test Data & Fixtures (per-cross-process-test lifecycle).
4. insta 1.46.1 golden snapshots with `run_id`/`journal_emitted_at`/`read_back_observed_at` redaction for Run-report envelope + JSONL journal per §7 Golden artifacts.
5. rstest 0.26.1 `#[fixture]` + seeded `conductor-timeline` generator + table-driven `#[case]` over scenario catalog per §7 Fixture library.

## Anti-patterns to avoid
1. NEVER escalate to load/saturation testing — bounded "typical/high" profiles (P-060) only; 50k+ spans/sec is Pulse's domain per §11 Test Strategy anti-pattern.
2. NEVER fake Pulse's *reaction* as CI verdict — rmcp stub canary proves MCP wiring only; Pulse behavior is live/local-gate assertion per §11 Test Strategy anti-pattern.
3. NEVER build runs.db SQL via `format!` — use rusqlite bound parameters even for synthetic data per §11 Integration anti-pattern; a negative test asserts this.

## Contract bindings
obs §3: self-obs stream (logs/agent-latest.jsonl + logs/conductor-tauri.jsonl) carries per-line base fields (timestamp_ms, level, target, service.name, run_id); distinct from the per-run emission journal (runs/<run_id>.jsonl, the SLO ground truth + Run-report envelope). Status endpoint binding: Run-report envelope (run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, journal_emitted_at, read_back_observed_at, fingerprints) shared by runs.db row + JSONL journal + Markdown report per §3 Status endpoint shape.

## Acceptance criteria contributions
1. (tests) `conductor preflight` against live Pulse → `ready: true` with passing canary round-trip (MCP initialize + fingerprint-fidelity via retrieve_telemetry_slice); prior `Blocked` state extinguished.
2. (tests) Each of the 5 scenario families (error-baseline-spike, fingerprint-storm, restart-suppression, pii-scrub, connection-lifecycle) runs against live Pulse and lands a non-`Blocked` verdict-first report state (Pass / Fail / ManualCheck / KnownResidual per claim class).
3. (tests) Evidence artifacts (JSONL + runs.db + Markdown report) written per run with journal-relative latency recorded and within tier-scaled tolerance; no host-path or internal-struct-name leakage.
4. (tests) Critical Path 1 (headless deterministic scenario run + MCP read-back) reproducible: same seed ⇒ identical emission-stream shape on re-run (insta golden journal comparison, redacting run_id/timestamps).
5. (tests) Coverage ≥60% line coverage on new test seams (cargo-llvm-cov 0.8.7 `--fail-under-lines 60`); zero flakiness (no nextest retries); determinism and verdict/error-wall invariants intact (no unlogged panics).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3 Log format): self-obs stream (stderr / logs/agent-latest.jsonl) noted as SEPARATE from the per-run emission journal (runs/<run_id>.jsonl, the SLO ground truth). This chunk owns the emission-journal schema; the self-obs stream is owned by obs-plan §3.
- **2026-06-26-live-counter-channel-stream** (§5 Integration Test Strategy): deferred Tauri `tauri::test` mock-runtime command/Channel assertion + GUI leg of cross-surface parity to the Epoch-9 `tauri-driver` GUI harness chunk. This chunk focuses on CLI E2E + MCP read-back (headless leg of Critical Path 7: `conductor-cli` CLI-smoke parity E2E); the webview leg defers.

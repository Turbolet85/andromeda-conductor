# security extract

## Relevance
relevant

## Constraints
1. `ANDROMEDA_PULSE_DATA_DIR` MUST be validated for argument-injection metacharacters and passed strictly via the `.env(...)` builder to `TokioChildProcess`, never interpolated into argv or shell (per security-plan §Threat Model "Vector: MCP read-back" + §Input Validation table row "ANDROMEDA_PULSE_DATA_DIR" + §Anti-Patterns §Input).
2. Canary round-trip (emit known incident → Pulse ingests → `query_incident_list` read-back → content-fidelity assertion by run_id/seed/timestamp fingerprint) MUST complete successfully for preflight to reach `ready: true`; empty or non-matching canary MUST surface as `blocked` ReportState with named precondition, never silent pass (per security-plan §Threat Model "preflight readiness gate — mismatch/empty ⇒ **blocked**" + §Input Validation "MCP read-back child stdout").
3. `andromeda-pulse-mcp` sidecar MUST be spawned as a fixed hard-coded program path only, never from operator-supplied command, shell string, or dynamically constructed argv (per security-plan §Anti-Patterns §Input + §Threat Model "spawn `andromeda-pulse-mcp` as a fixed...program path only").
4. All `tonic::Status` codes and MCP tool responses (protobuf-decoded via rmcp 1.7.0) MUST enforce bounded recursion; decode errors MUST NOT panic — they route through the verdict/error wall as `Ok(ReportState::Blocked)` or `Ok(Verdict::Fail)`, never `panic!` or unhandled `Result::Err` (per security-plan §Input Validation "bound prost/protobuf decoding" + §Error Handling + §Threat Model "trusted-child boundary").
5. Run-report artifacts (JSONL journal `<run_id>.jsonl`, `runs.db` rows, Markdown report `<run_id>.md`) MUST NOT leak absolute filesystem paths (canonicalized `CONDUCTOR_RUNS_DIR`, `ANDROMEDA_PULSE_DATA_DIR`) or internal seam-crate struct/field names — record only verdict/state/identity fields (`run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`, `latency_ms`, `fingerprints`) + journal-relative SLO latency (per security-plan §Error Handling "Run-report artifact sanitization" + §Anti-Patterns §Logging).
6. Preflight failures (protocol version mismatch / missing required tool / empty canary / keychain read-while-write fault) MUST surface as distinct `blocked` ReportState with named precondition string in the `runs.db` record, never downgraded to Pass/Fail/ManualCheck (per security-plan §Threat Model "mismatch/empty ⇒ **blocked**, never silent downgrade" + §Anti-Patterns §Universal + Scope "Verdict/error wall").

## Patterns to follow
1. Typed verdict/error wall: all MCP transport/canary/verification failures route through the `Verdict` / `ReportState` enum (`Blocked`, `Fail`, `Pass`, `ManualCheck`, `KnownResidual`), never panics or type-erased `anyhow::Error` at the read-back boundary (per security-plan §Error Handling + §Threat Model).
2. Canary round-trip pattern: emit one known/identifiable canary incident with a unique fingerprint (run_id + seed + timestamp), let Pulse ingest it, retrieve it via `query_incident_list`, assert content fidelity (the read-back incident matches the emitted one), proving data-dir workspace wiring end-to-end — a failed canary becomes `blocked`, not a false pass (per Scope "canary bridge" + security-plan §Threat Model).
3. Run-artifact write with bound parameters: `runs.db` writes use rusqlite 0.38.0 parameterized statements for `run_id` / `seed` / fingerprint-JSON1 (never string-formatted SQL) and explicitly NULL measurement fields for `Blocked` rows (per security-plan §Input Validation "runs.db writes (rusqlite raw SQL...use **bound parameters**)").

## Anti-patterns to avoid
1. NEVER interpolate `ANDROMEDA_PULSE_DATA_DIR` or any operator-supplied value into the `andromeda-pulse-mcp` argv or shell — pass strictly via `.env(...)` builder and reject argument-injection metacharacters before spawn (CVE-2026-30623 / OX advisory, per security-plan §Anti-Patterns §Input §Code Patterns).
2. NEVER expose absolute filesystem paths or internal struct/field names in run-report artifacts (JSONL journals, `runs.db` rows, Markdown reports) — they are agent-parseable ground truth shared across hosts and must remain sanitized (per security-plan §Anti-Patterns §Logging).
3. NEVER silently downgrade a failed preflight (version mismatch / missing tool / empty canary / keychain fault) to Pass/Fail/ManualCheck — surface it as the distinct `blocked` state with named precondition string (per security-plan §Anti-Patterns §Universal).

## Contract bindings
(none)

## Acceptance criteria contributions
1. (security) Preflight canary round-trip completes with content-fidelity assertion against live Pulse; `conductor preflight` exits `ready: true` (was `Blocked` before this chunk) (per security-plan §Threat Model preflight gate).
2. (security) Preflight failures (version / tool / canary / keychain) route to distinct `blocked` ReportState with named precondition in `runs.db`; no panic or false pass (per security-plan §Anti-Patterns §Universal).
3. (security) Run artifacts (JSONL/`runs.db`/Markdown) contain no absolute filesystem paths or internal struct names — only verdict/state/identity fields + journal-relative SLO latency (per security-plan §Error Handling artifact sanitization).
4. (security) `ANDROMEDA_PULSE_DATA_DIR` validated for argument-injection metacharacters in `conductor-verify` before `TokioChildProcess` spawn; sidecar spawned as fixed hard-coded path (per security-plan §Anti-Patterns §Input).

## Relevant amendment history
- **2026-06-24-sanitized-stderr-agent-mode-logging** — clarified env-handle taxonomy: path handles (`CONDUCTOR_*` / `ANDROMEDA_PULSE_DATA_DIR` requiring `std::fs::canonicalize` + bounds-check) distinct from non-path labels (`CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` / `CONDUCTOR_AGENT_MODE`, no validation). Relevant because live-pulse validates `ANDROMEDA_PULSE_DATA_DIR` at the conductor-verify edge before propagating to sidecar via `.env(...)` (per security-plan §Input Validation).
- **2026-06-21-runs-db-index** — confirmed bundled SQLite 3.50.4 (via `libsqlite3-sys 0.36.0`); `Cargo.lock` committed. Relevant because live-pulse writes `runs.db` rows with bound parameters + NULL-enforcement for `Blocked` rows (per security-plan §Input Validation "runs.db writes").

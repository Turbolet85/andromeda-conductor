# obs extract

## Relevance
Relevant — conductor-verify MCP read-back client is boundary-only instrumentation; chunk adapts call mechanism to Pulse's raw JSON shape and fixes error wall distinction.

## Constraints
1. Per §1: conductor-verify is "Boundary-only" — MCP read-back calls are seams; only boundary instrumentation applies (no auto-instrumentation).
2. Per §4: Must-trace spans `verify.readback*` with attributes `mcp_method` (query_incident_list / retrieve_report / retrieve_telemetry_slice / mark_incident_resolved), `latency_ms`.
3. Per §6: Boundary-call wrappers must log method name + latency_ms + error (if any) + canary-check result for every read-back call.
4. Per §10: Zero-unlogged-panics SLO — error wall must distinguish real call/transport error from successful-but-empty result; no panic masking.
5. Per §11 Error Reporting: Malformed MCP read-back (protobuf decode, transport failure) ⇒ typed `blocked`, never panic or unlogged exception.
6. Per §3: Use `tracing` 0.1.x + `tracing-subscriber` JSON formatter; no OTel SDK for self-observation.

## Patterns to follow
1. Per §4: Manual `#[tracing::instrument]` span wrapping each of the four tool-call paths; emit `mcp_method` + `latency_ms` as span attributes.
2. Per §6: Log boundary calls via `tracing::{info,warn,error}!()` with method name, latency_ms, error status, and canary result; structured JSON per line.
3. Per §3: Self-obs lines must carry the base-line schema (`timestamp_ms`, `level`, `target`, `service.name`, `service.version`, `deployment.environment`, `run_id`) — NOT the run-report envelope.
4. Per §11 Error Reporting: Convert transport/protocol errors to typed `blocked` verdict at the verify boundary; log as `tracing::error!(...)` with sanitized context.

## Anti-patterns to avoid
1. Per §11 Error Reporting: NEVER leave an MCP read-back panic uncaptured or let malformed child/transport input crash the run — bounded prost recursion + typed result.
2. Per §11 Logs: NEVER leak absolute host paths or internal struct names (drive-letter, `/home`, `/Users`, `%APPDATA%`, backtrace file paths) — apply redaction layer on JSON emit.
3. Per §11 Spans: NEVER use high-cardinality span names — the bounded set (verify.readback, verify.readback_*, etc.) is low-cardinality; conform to it.

## Contract bindings
Tests harness §3: Seam tests must exercise the real wire format (Pulse's raw `tools/call` result shape); stub must emit raw shapes; regression test pins the contract.

## Acceptance criteria contributions
1. (obs) Boundary-call logs: every MCP read-back call emits a `tracing::info!()` event with `mcp_method`, `latency_ms`, success/error status.
2. (obs) Error wall distinction: real call/transport errors are logged and converted to typed `blocked` (not masked by canary's misleading "incident not found"); error reasons preserved in logs.
3. (obs) Panic safety: malformed MCP responses (protobuf failures, transport errors) produce typed `blocked` result + `tracing::error!(...)` log; never panic.
4. (obs) Zero-unlogged-panics gate: CI passes when all call-site errors are structured-logged and painted to verdict/error wall.

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§3): Self-obs base-line schema (`timestamp_ms`, `level`, `target`, service-identity, `run_id`) must appear on every JSONL line; run-report envelope (`verdict`, `state`, `latency_ms`, etc.) appears only on scenario-result events — the chunk's call-site logs are base-line lines, not envelope records.
- **2026-06-15-log-error-boundary-redaction** (§6 + §11): Redaction model: absolute host-file paths → `<redacted>` (drive-letter, `/home`, `/Users`, `%APPDATA%`, backtrace file paths); struct names kept out by field-name allowlist + `Display`-not-`Debug`; `target` module path preserved — the chunk's error logging must conform (no raw backtraces or host paths).
- **2026-06-24-sanitized-stderr-agent-mode-logging** (§3): Agent mode is a read-only trigger (`agent_mode = flag || env-set`) that Conductor reads, never writes — relevant for understanding when JSON-only output is enforced.

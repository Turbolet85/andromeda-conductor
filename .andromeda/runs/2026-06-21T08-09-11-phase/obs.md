# obs extract

## Relevance
Partial — MCP read-back client is the transport layer for boundary-only observation; observability binds to spawn hardening, error capture, and bounded-scope instrumentation only.

## Constraints
1. **No OTel SDK for self-observation** (per obs-plan §3 OTel SDK init): the rmcp client is a seam; any instrumentation must use `tracing` spans only, never initialize opentelemetry-sdk. Transport errors fan into typed `VerifyError` (verdict/error wall per obs-plan §11 Error Reporting: "malformed child/transport input ⇒ blocked, never panic").
2. **Bounded span name set** (per obs-plan §11 Spans / Traces): if instrumentation is added, span names must stay within `{module}.{operation}` pattern — `verify.readback` + `verify.readback_*` variants are pre-authorized for read-back paths (obs-plan §4 Critical Paths must-trace scenarios).
3. **Spawn hardening blocks injection** (per obs-plan §11 Project-specific bans): fixed program path + `.env(ANDROMEDA_PULSE_DATA_DIR, ...)` only; metacharacter rejection before spawn (CVE-2026-30623). This is security-adjacent obs (attestable via audit log + error logs on rejection).
4. **No W3C trace context** (per obs-plan §11 Spans / Traces): correlation is `run_id` field on every log line; this chunk's IPC boundary carries no `traceparent` (both-surface parity is `runs.db` envelope comparison — same seed ⇒ same verdict/state).
5. **Error capture required** (per obs-plan §10 Always-required SLO invariant): if rmcp spawn fails or transport errors occur, emit structured `tracing::error!` with context; convert to `VerifyError` (never panic on child/transport input per §11 Error Reporting).
6. **Wall-clock timestamps only** (per obs-plan §11 Project-specific bans): no tokio virtual clock for journal timestamps; rmcp session latency must use `std::time::SystemTime`/`Instant` (preserves journal-relative SLO math).

## Patterns to follow
1. **Span instrumentation on critical path** (per obs-plan §4 Scenario 1 must-trace: `scenario.run` → ... → `verify.readback` (MCP call) → ...): wrap the MCP client call in a `#[tracing::instrument]` span with attributes `mcp_method`, `latency_ms` on the `verify.readback*` span variants the gate chunk later consumes.
2. **Typed error wall** (per obs-plan §11 Error Reporting and scope Acceptance intent): rmcp errors (client creation, spawn failure, protocol mismatch, tool-not-found) map to `VerifyError` variants; callers receive typed `Result<…, VerifyError>` (not panics).
3. **Service identity propagation** (per obs-plan §3 Service identity): CLI/Tauri logs must include `service.name` / `service.version` / `deployment.environment` on every line; this chunk's boundary calls inherit those via the parent scenario span's context (the root `scenario.run` span carries the identity fields; child `verify.readback` spans inherit them).

## Anti-patterns to avoid
1. **NEVER initialize OTel SDK** (obs-plan §11 Telemetry Strategy anti-pattern): the rmcp client depends on opentelemetry-proto (PRODUCT), not opentelemetry-sdk. If transitive opentelemetry_sdk is present (dormant, per obs-plan §3 OTel SDK init clarification), do NOT initialize it. Self-obs remains tracing-only.
2. **NEVER use shell to spawn the sidecar** (obs-plan §11 Project-specific bans): TokioChildProcess + fixed path + `.env()` builder (no shell metacharacter exposure).
3. **NEVER leak panics on malformed input** (obs-plan §11 Error Reporting anti-pattern): bounded prost recursion on MCP read-back + any decode failure ⇒ typed error value, not panic (verdict/error wall).

## Contract bindings
- **tests harness binding** (per obs-plan §1 Observability Harness Contract and focus guide §Cross-domain bindings): the MCP read-back client response shape (tool list, tool results) must be agent-readable JSON when logged; tests consume the `verify.readback` span attributes + `VerifyError` variants for error injection (stub child / protocol mismatch scenarios). The binding is read-only on obs side: emit `mcp_method` + `latency_ms` + error context on every call; tests assert those fields are present.
- **obs gate-chunk binding** (Epoch 5 chunk 2 preflight + chunk 4 verdict): this chunk exposes `list_all_tools()` + `call_tool()` + `peer_info()` (negotiated version observable); chunks 2 and 4 consume the typed error wall and span attributes to decide Blocked vs Pass/Fail.

## Acceptance criteria contributions
1. **(obs) MCP transport instrumentation:** `#[tracing::instrument]` spans on `serve_client()` (session init) and `call_tool()` / `query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved` (read-back calls) with attributes `mcp_method` (string), `latency_ms` (wall-clock), `error` (if any). Spawn errors logged as structured `tracing::error!` with `error_kind`, `error_message` (sanitized of host paths per §11 PII Scrubbing).
2. **(obs) Verdict/error wall:** Rmcp transport errors map cleanly to `VerifyError` variants (SpawnFailed, ProtocolMismatch, ToolNotFound, Timeout, DecodeError); callers never receive panics. Logs on error do not leak internal struct names or absolute paths.
3. **(obs) Span closure at phase boundary:** `verify.readback` span closes on MCP response receipt or error; parent `scenario.run` span remains open (closed by report-generation, per obs-plan §4 Critical Paths cleanup).

## Relevant amendment history
1. **2026-06-15-structured-logging-stack** (amendments §1): clarified that self-obs log lines carry base set (`timestamp_ms`, `level`, `target`, `service.name/version/environment`, `run_id`) on every line; this chunk's boundary calls inherit those fields via span context.
2. **2026-06-17-raw-otlp-message-scaffold** (amendments §2): "no-SDK invariant clarified as behavioral" — opentelemetry-proto may be a transitive dep (dormant), but SDK is never initialized. This chunk's rmcp client depends on opentelemetry-proto for Pulse's OTLP types (PRODUCT); self-obs remains tracing-only.

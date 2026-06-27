# Codebase Research — 2026-06-27-mcp-read-back-result-shape-adapter

## Scope
- **Depth:** deep (read-back client + rmcp 1.7 internals + Pulse's sidecar) · **Reads:** conductor-verify (client/preflight/error/spawn/stub) + Pulse mcp-server (jsonrpc/tools/bin) + rmcp 1.7 source · **Code-graph:** 1 impact query + earlier live diagnosis.

## Files inspected
- `crates/conductor-verify/src/client.rs` — `ReadbackClient` over rmcp `RunningService<RoleClient, ClientInfo>`; the 4 wrappers (`query_incident_list` etc.) call `self.service.call_tool(params)` → typed `CallToolResult`. **This typed call is the failure.** `connect`/`connect_transport` (handshake), `negotiated_protocol_version`, `list_tools` work.
- `crates/conductor-verify/src/preflight.rs` — `run_preflight` canary leg (l.126-136) `client.query_incident_list(None)` then `body = serde_json::to_string(&result.content)` + `body.contains(marker)`; the `_ => CanaryOutcome::Failed` (l.135) masks a call **error** as "incident not found in corpus" (l.153).
- `crates/conductor-verify/src/spawn.rs` — hardened spawn: fixed program `andromeda-pulse-mcp` (from PATH), `.env` data-dir after injection-reject. Reusable as-is by a hand-rolled client (it builds a `tokio::process::Command`).
- `crates/conductor-verify/src/bin/stub_pulse_mcp.rs` — the test stub is an **rmcp server** (`Content::text(body)`) → rmcp wraps results in a proper `CallToolResult`. **This is why the bug was hidden** — the stub never modeled Pulse's raw shape.
- `crates/conductor-verify/Cargo.toml` — deps already include `tokio` (`process`, `io-util`, `rt`, `macros`) + `serde_json` + `serde`. **A hand-rolled JSON-RPC stdio client needs NO new dependency.** `stub-server` feature pulls `rmcp/server` + `tokio/io-std`.
- Pulse `andromeda-pulse/crates/mcp-server/src/{jsonrpc.rs,bin/andromeda-pulse-mcp.rs,tools.rs}` — line-delimited JSON-RPC over stdio; methods `initialize` / `tools/list` / `tools/call` / `ping`; `success(id, result: Value)` returns the **raw tool payload** (no `{content:[...]}`), `error(id, code, msg)` for errors. Tool result shapes confirmed (see below).
- rmcp 1.7 `service.rs` + `service/client.rs` (in `D:\dev\rust\cargo\registry`).

## Graph impact (code-graph query — trace at `.andromeda/runs/2026-06-27T19-48-14-phase/tree-query-2026-06-27-mcp-read-back-result-shape-adapter.json`)
Callers of the read-back tool methods / `call_tool`:
- `conductor-verify/src/client.rs:101,109,117,125` — the 4 wrappers (the change locus).
- `conductor-verify/src/preflight.rs:125` — `run_preflight` canary leg (consume raw `Value`; fix the catch-all).
- `conductor-run/src/lib.rs:137` — `execute_scenario` coarse read-back (consume raw `Value`; signature-stable).
- `conductor-verify/tests/readback.rs:77` — the round-trip seam test (update to the raw shape).
**Contained blast radius** — all within conductor-verify + one conductor-run call site + the tests/stub.

## The rmcp 1.7 verdict (the central finding)
rmcp **cannot return Pulse's raw un-enveloped tool result**:
- `service/client.rs:282/297/312/327` — the client typed methods match the deserialized **`ServerResult`** enum (`ServerResult::CallToolResult(r) => Ok(r)`, `_ => Err(ServiceError::UnexpectedResponse)`). rmcp deserializes the JSON-RPC `result` into the typed `ServerResult` BEFORE handing it back; Pulse's `{items,…}` doesn't match the `CallToolResult` variant → `UnexpectedResponse` (the live error).
- `service.rs:442 send_request(req) -> R::PeerResp` — even the low-level call returns the **typed** `ServerResult`, not raw JSON. `ServerResult` (model.rs) has no raw-`Value` tool variant.
- No public transport / raw-send escape on `Peer`/`RunningService` (`peer()` returns `&Peer`; the transport is consumed internally by `serve_inner`). A custom response type needs a custom `ServiceRole` = forking rmcp.
**⇒ Handling Pulse's hand-rolled, non-MCP-compliant `tools/call` requires bypassing rmcp's typed result layer — i.e., a hand-rolled JSON-RPC stdio read-back client.** This reverses arch §Established Decisions *[MCP Read-Back Client]* (rmcp chosen to avoid hand-rolled JSON-RPC) — an arch amendment (wrap's job).

## Pulse wire contract (live-verified)
- Framing: one JSON object per line over stdio; request `{jsonrpc,id,method,params}`, response `{jsonrpc,id,result}` or `{jsonrpc,id,error:{code,message}}`.
- `initialize` → result carries `protocolVersion` (`2024-11-05`) — version negotiation = read this field.
- `tools/list` → result `{tools:[{name,…}]}`.
- `query_incident_list` → `{items:[{incident_id,status,severity,title,opened_at_unix_nano}],total,next_cursor}`.
- `retrieve_report` → `{markdown,degraded_mode}` · `retrieve_telemetry_slice` → `{incident_id,span_refs,fingerprint_refs,timestamps_unix_nano}` · `mark_incident_resolved` → `{resolved,incident_id}`.
- Errors: JSON-RPC `error` (e.g. `"incident corpus unavailable"`).

## Patterns to follow
- **Hardened spawn reuse** (`spawn.rs`): a hand-rolled client reuses `resolve_data_dir` + `build_command` (fixed path + `.env`); no new spawn code (security-plan §Anti-Patterns).
- **Verdict/error wall** (`preflight.rs:163`, `error.rs`): call/transport/parse failures → typed `VerifyError`/`Blocked` with the TRUE precondition; never panic, never mask.
- **Bounded JSON decode** (security): cap response line size / serde recursion; malformed → typed error, not panic.
- **Boundary obs spans** (obs §4): keep `verify.readback*` span names + `mcp_method`/`latency_ms` attributes on the hand-rolled calls.

## Conventions to follow
- `thiserror` `VerifyError` variants per failure class; `redact_value` on any error string entering `ReadyState`.
- `std::time` stamps; bounded low-cardinality span names; self-obs base-line schema on log lines.

## New files to create
- `crates/conductor-verify/src/jsonrpc.rs` (or `rpc.rs`) — a minimal line-delimited JSON-RPC stdio client over `tokio::process` (write request, read line, parse `{result|error}` as `serde_json::Value`); the request-id counter + initialize/version handshake.

## Files to modify
- `crates/conductor-verify/src/client.rs` — `ReadbackClient` backed by the hand-rolled stdio session; the 4 wrappers return raw `serde_json::Value`; `connect`/`negotiated_protocol_version`/`list_tools` reimplemented hand-rolled (drop rmcp `RunningService`).
- `crates/conductor-verify/src/preflight.rs` — canary leg consumes the raw `Value` (parse `items` / fingerprint as needed); fix the `_ => Failed` catch-all to distinguish call-error (→ Blocked w/ true precondition) from empty/absent.
- `crates/conductor-verify/src/error.rs` — variants for the hand-rolled transport/parse errors (replace the rmcp `ServiceError`-boxed variants).
- `crates/conductor-verify/src/bin/stub_pulse_mcp.rs` — rebuild as a hand-rolled JSON-RPC stdio server emitting Pulse's **raw** result shapes (faithful) — no longer rmcp-server.
- `crates/conductor-verify/src/lib.rs` + `Cargo.toml` — drop the `rmcp` dependency + the `stub-server`/rmcp features (arch reversal); `tokio`/`serde_json` already present.
- `crates/conductor-verify/tests/{readback,preflight,preflight_spawn}.rs` — assert the raw shape; a regression that would have caught `UnexpectedResponse`.
- `crates/conductor-run/src/lib.rs:137-141` — coarse read-back consumes the raw `Value` (signature-stable).

## Open questions
1. **Approach sign-off:** rmcp can't return raw results → the fix is a hand-rolled JSON-RPC read-back client, which **removes rmcp** + reverses arch §[MCP Read-Back Client]. → P4 AskUserQuestion (confirm before finalizing; the only alternative is forking rmcp, not recommended).
2. **Drop rmcp dep entirely vs leave it dead** — removing it is cleaner (and drops transitive deps from audit/deny) but touches `Cargo.lock`; recommend removing. Resolve in plan.
3. Bounded read-line size for the stdio client (DoS-bound on child stdout) — pick a sane cap (security §Input Validation).

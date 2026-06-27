# Scope — MCP read-back result-shape adapter

**Marker:** `2026-06-27-mcp-read-back-result-shape-adapter`
**Version:** conductor-0.1.0 · **Epoch:** 10 (Polish & ship)
**Working entry:** _MCP read-back result-shape adapter — adapt conductor-verify's read-back client to Pulse's hand-rolled `tools/call` results (raw tool JSON, NO MCP `{content:[...]}` envelope)…_

## Intent
Make Conductor's MCP read-back actually work against the **live Pulse sidecar**. Discovered 2026-06-27 (implementing Live-Pulse E2E proof against a real Pulse): conductor-verify's read-back client uses rmcp 1.7's **typed `call_tool`**, which requires the MCP `CallToolResult` envelope `{ "content": [ … ], "isError": … }`. Pulse's `andromeda-pulse-mcp` is **hand-rolled JSON-RPC** (`crates/mcp-server/src/jsonrpc.rs` — `success(id, result: Value)`, no content wrapping anywhere) and returns the **raw tool payload** as the JSON-RPC `result` (e.g. `query_incident_list` → `{ "items": [...], "total": N, "next_cursor": null }`). rmcp can't parse that as `CallToolResult` → `ServiceError::UnexpectedResponse` (`VerifyError::Call`) on **every** tool call. So read-back has never worked live; only `initialize` + `tools/list` (which happen to be shape-compatible) succeed, which is why preflight's protocol-version + tool-presence checks pass while every actual tool **call** fails.

This chunk is the **prerequisite** for the (de-promoted) Live-Pulse E2E proof: its canary bridge + 5-family verification are unverifiable until a tool call returns usable data.

## What this chunk builds
1. **Raw-result read-back calls** — in `conductor-verify` (`client.rs`), call the four read-back tools via a request path that returns Pulse's **raw JSON-RPC `result`** parsed as `serde_json::Value`, instead of rmcp's typed `call_tool` that enforces the `CallToolResult` envelope. **Research P3 settled the mechanism (P4 user-confirmed): rmcp cannot surface Pulse's raw result and exposes no transport escape, so the ENTIRE read-back client becomes a hand-rolled line-delimited JSON-RPC stdio client and rmcp is removed; `initialize`/version-negotiation + `list_tools` are hand-rolled too** (partial-keep is infeasible — rmcp's `RunningService` owns the transport).
2. **Faithful test stub** — `stub_pulse_mcp` currently uses an rmcp **server** that wraps tool results in a proper `CallToolResult`, so CI never modeled Pulse's non-compliant raw shape (the reason this was hidden). Make the stub emit Pulse's **raw** `tools/call` result shape (`{items,…}` / `{markdown,degraded_mode}` / `{span_refs,…}` / `{resolved,incident_id}`) so the seam tests exercise the real wire format.
3. **Un-mask the error wall** — `run_preflight`'s canary leg has a `_ => CanaryOutcome::Failed` catch-all that reported a genuine call **error** as the misleading precondition `"canary round-trip failed: incident not found in corpus"`. Distinguish a real call/transport error (a harness/transport precondition → `Blocked` with the *true* reason, or `Result::Err` per the verdict/error wall) from a successful-but-empty / marker-absent result.
4. **Update consumers to the raw shape** — every site that read `CallToolResult` (`run_preflight` reads `result.content`; `execute_scenario`'s coarse read-back in `conductor-run`) switches to the raw `Value`.

## Surfaces & contracts touched
- `conductor-verify/src/client.rs` — the four tool wrappers + the call mechanism (raw request → `Value`); `initialize`/`list_tools` unchanged.
- `conductor-verify/src/preflight.rs` — the canary read-back leg consumes the raw `Value`; the error-masking catch-all is fixed.
- `conductor-verify/src/bin/stub_pulse_mcp.rs` — emit Pulse's raw `tools/call` result shape (the key fidelity fix).
- `conductor-verify/tests/{preflight,preflight_spawn,readback}.rs` — assert against the raw shape; add a regression that would have caught `UnexpectedResponse`.
- `conductor-run/src/lib.rs` — the coarse read-back call site that read `CallToolResult` (minimal, signature-stable).
- `conductor-verify/src/error.rs` — possibly a clearer variant if a raw-call failure needs distinguishing (only if warranted).

## The live wire contract (verified against the running Pulse)
- `query_incident_list` → `{ "items": [ {incident_id, status, severity, title, opened_at_unix_nano} ], "total", "next_cursor" }` (titles are scrubbed/generated, not verbatim).
- `retrieve_report` → `{ "markdown", "degraded_mode" }`.
- `retrieve_telemetry_slice` → `{ "incident_id", "span_refs", "fingerprint_refs", "timestamps_unix_nano" }`.
- `mark_incident_resolved` → `{ "resolved": true, "incident_id" }`.
- Errors come back as JSON-RPC errors (e.g. `"incident corpus unavailable"`); the sidecar wraps NOTHING in MCP content blocks (`jsonrpc.rs` `success(id, result: Value)`).
- Proven run env: pulse-app on `:4317` · `andromeda-pulse-mcp.exe` at `andromeda-pulse/target/debug` · `ANDROMEDA_PULSE_MCP_ENABLED=true` · `ANDROMEDA_PULSE_DATA_DIR=%APPDATA%\andromeda-pulse`.

## Boundaries / non-goals
- **NOT** the canary bridge, faithful emission, or 5-family verification — those are the re-promoted Live-Pulse E2E proof chunk, which builds on this adapter.
- **Replace the rmcp client entirely with a hand-rolled JSON-RPC stdio client** (research P3: rmcp can't return the raw result + can't share its transport, so partial-keep is infeasible); preserve the hardened spawn (`spawn.rs`), the `2024-11-05` pin (now read from the `initialize` result), and the preflight gate's three assertions. rmcp is dropped from conductor-verify — an arch §[MCP Read-Back Client] reversal (wrap amends the decision record).
- Preserve the verdict/error wall (call/transport failures → typed `Blocked`/`Err`, never a panic), the redaction discipline, and the preflight gate's three assertions (version / tools / canary).
- No new crate edge; no scenario/TOML changes; no GUI.

## Acceptance shape (refined in plan.md)
- A live `conductor preflight` reaches the canary leg with a **real** `query_incident_list` result (the call no longer errors `UnexpectedResponse`); the canary's pass/fail then reflects corpus contents, not a parse failure. (Full `ready:true` still needs the canary *emission*, which is the next chunk — but this chunk proves the call returns usable data.)
- Seam tests run against a stub that emits Pulse's **raw** result shape; a regression test pins the raw-shape contract.
- `run_preflight` reports a genuine call error with its true precondition (not "incident not found").
- `cargo nextest run -p conductor-verify` + the workspace gate green; the hermetic CLI Blocked-leg E2E still green; determinism + verdict/error-wall intact.

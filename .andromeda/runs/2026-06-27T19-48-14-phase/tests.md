# tests extract

## Relevance
relevant

## Constraints
1. The rmcp client's tool-call results must parse as Pulse's **raw** JSON-RPC `result` shape (`serde_json::Value`), not MCP `CallToolResult` envelope (per §3 ipc-internal surface "IPC response payload check" + §1 testable conductor-verify "verdict logic + preflight state mapping...against rmcp in-process stub").
2. The `stub_pulse_mcp.rs` must emit Pulse's **raw** `tools/call` result shapes (`{items,total,next_cursor}` for `query_incident_list`, `{markdown,degraded_mode}` for `retrieve_report`, `{incident_id,span_refs,fingerprint_refs,timestamps_unix_nano}` for `retrieve_telemetry_slice`, `{resolved,incident_id}` for `mark_incident_resolved`), not MCP-wrapped `CallToolResult` (per §2 Agent-runnable invariants "mocks must match the real wire contract").
3. The preflight canary leg must distinguish **genuine call/transport errors** (→ `Blocked` with the true precondition reason OR typed `Result::Err`) from **successful-but-empty canary results** (per §3 boot "readiness signal...ready:false ⇒ every dependent scenario reports state=Blocked with the **named precondition string**").
4. Unit tests on `conductor-verify` must assert against the raw shape; integration tests running `stub_pulse_mcp` must exercise the non-wrapped result contract so the shape mismatch is regression-tested (per §4 "conductor-verify: verdict logic + preflight state mapping...against an rmcp in-process stub").
5. The verdict/error wall (call/transport failures → typed `Blocked`/`Err`, no panic) must remain intact; redaction discipline preserved (per §1 Error-handling + §3 Status endpoint "artifacts MUST NOT leak absolute host paths or internal struct names").

## Patterns to follow
1. **Stub-server fidelity matching** — the stub must emit the real wire format Pulse uses (raw JSON-RPC result, not MCP envelope), proven by golden-test regression that would have caught the original `UnexpectedResponse` (per §2 test-pyramid Integration row "rmcp stub stdio" + §3.ipc-internal driver "language-native IPC test client — a stub/mock MCP server").
2. **Raw-request path via rmcp::Peer** — call the four read-back tools over a lower-level rmcp request mechanism that returns `serde_json::Value` without enforcing `CallToolResult` envelope shape; keep `initialize`/`list_tools` on the high-level typed API (per §3.bootstrap "5-command-discipline-wire...rmcp client for the handshake + tool listing").
3. **Per-seam test isolation with table-driven fixtures** — add regression tests to `conductor-verify/tests/` asserting the stub result shape matches Pulse's raw format; use rstest `#[case]` pattern (per §4 "rstest...table-driven...fixtures...per-seam crate-local tests/").

## Anti-patterns to avoid
1. **Do not wrap the stub result in MCP `CallToolResult`** — the bug being fixed is exactly that wrapping. The stub must emit Pulse's raw shape unchanged (per §3 "ipc-internal surface...the sidecar wraps NOTHING in MCP content blocks").
2. **Do not mask call errors with "incident not found"** — distinguish transport/call errors from empty corpus. `VerifyError::Call(UnexpectedResponse)` is a genuine precondition failure, not a benign empty result.

## Contract bindings
**obs ↔ tests harness (verdict/error wall)** — The preflight readiness gate's `Blocked` verdict and its precondition string are part of the Run-report envelope (§3 Status endpoint shape); they flow to the emission journal and `runs.db` row which obs reads. A call error (vs. empty corpus) must map to a distinct precondition so the envelope's `state` field is truthful (per §3 "log-format-bind-with-obs...harness greps the journal for assertions; format break = harness break").

## Acceptance criteria contributions
1. **(tests) `cargo nextest run -p conductor-verify` passes** with new/updated regression tests asserting `stub_pulse_mcp` emits Pulse's raw JSON-RPC result shape (not MCP-wrapped) and regression test proving raw-shape parsing works (would have caught `UnexpectedResponse`).
2. **(tests) Live-preflight reaches the canary leg** — a `conductor preflight` against the live Pulse sidecar performs a real `query_incident_list` tool call that returns the raw shape and parses successfully (no `UnexpectedResponse`); the canary's pass/fail then reflects corpus contents, not a parse error.
3. **(tests) Verdict/error wall — call errors distinguished from empty corpus** — preflight distinguishes between genuine call/transport error (→ `Blocked` with "mcp sidecar unreachable" or similar true precondition) and successful-but-empty corpus (→ `Blocked` with "canary incident not found"), verified by integration tests on the stub and by fixing the error-masking catch-all in `run_preflight`.

## Relevant amendment history
- **2026-06-26-live-counter-channel-stream** — deferred Tauri integration + GUI parity tests to GUI test-harness chunk; noted that run logic IS covered deterministically at unit tier + CLI parity E2E (byte-identical `Blocked` envelope post-extraction). This chunk contributes to that determinism by fixing the read-back wire contract so the envelope (`verdict`/`state`) is correct and the canary test can properly detect corpus contents.

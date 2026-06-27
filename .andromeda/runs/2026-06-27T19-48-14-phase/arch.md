# arch extract

## Relevance
relevant

## Constraints
1. per §Stack and Technologies: rmcp 1.7.0 is the official Rust MCP SDK; version negotiation + typed tool calls are the silent-mismatch guard the contract manifest fixes; hand-rolled JSON-RPC / third-party sdk rejected for exactly this reason
2. per §Established Decisions [MCP Read-Back Client]: rmcp client MUST negotiate down to `2024-11-05` (Pulse's hand-rolled server version, not rmcp default); call-result handling changes ONLY, not handshake / `list_tools` / spawn
3. per §Established Decisions [Read-Back Dependency Posture]: Three-leg preflight (protocol version + tool presence + canary round-trip) gates every dependent scenario; a failed tool call surfaces as `Blocked` with named true precondition, never silent/masked
4. per §Standard Contracts (Run report envelope): `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}`; `Blocked` is a distinct terminal state reached on precondition mismatch/absence/transport-error, never silently downgraded to pass/fail
5. per §Conventions (Error handling): typed `thiserror` enums per seam crate (e.g. `VerifyError`); `Result::Err` reserved for harness faults (transport down, MCP unreachable, protocol mismatch); verification outcomes are `Ok(Verdict)`, not panic/catch
6. per §Cross-cutting Patterns (Verdict/error wall): call/transport failures are harness-layer errors (not verification verdicts); canary's error must be distinguished from empty-result (both block, but blocked_precondition differs)

## Patterns to follow
1. per §Established Decisions [MCP Read-Back Client]: preserve rmcp for `initialize` + `list_tools` + `TokioChildProcess` spawn; only tool-call result handling changes (accept raw `serde_json::Value` instead of rmcp's typed `CallToolResult`)
2. per §Conventions (Error handling) + §Cross-cutting Patterns (Verdict/error wall): a real JSON-RPC parse error / transport failure maps to a typed `VerifyError` variant, then to `Blocked` verdict + true precondition string (not "incident not found" masking)

## Anti-patterns to avoid
1. per §Established Decisions [MCP Read-Back Client]: do NOT hand-roll JSON-RPC as a substitute; do NOT downgrade rmcp itself or switch to third-party SDK (silent-mismatch risk); keep version negotiation + tool-list via rmcp
2. per §Cross-cutting Patterns (Verdict/error wall): do NOT report a real call error as a `Fail` verdict or pass silently; distinguish call-error → `Blocked`/`Err` (harness) from empty-result → `Blocked` (true precondition varies)

## Contract bindings
- **Verify ↔ Run:** conductor-run calls verify's query_incident_list/retrieve_report wrappers in execute_scenario; raw-Value consumers must match run's call sites (conductor-run/src/lib.rs)
- **Verify ↔ Report:** canary outcome (pass/fail/error with true precondition) determines run report's `ready` field + dependent scenarios' `Blocked` state and `blocked_precondition` string
- **Verify ↔ Storage:** blocked scenarios populate runs.db with NULL `latency_ms`/`journal_emitted_at`/etc.; cross-run latency/percentile queries in report seam must exclude these rows (per §Data model conventions)

## Acceptance criteria contributions
1. (arch) MCP tool calls return raw `serde_json::Value` per Pulse's JSON-RPC `tools/call` envelope; no MCP `CallToolResult` wrapping (per §Established Decisions [MCP Read-Back Client])
2. (arch) A genuine call/transport error surfaces as typed `VerifyError` variant → `Blocked` verdict with true precondition (not masked "incident not found")
3. (arch) Stub emits Pulse's raw `tools/call` shape (per §Standard Contracts — readiness gate verifies actual wire format, not rmcp-wrapped stub shape)
4. (arch) No new external crate edge; conductor-verify/conductor-run remain five-seam workspace (per §Inherited Defaults [Module Boundaries])

## Relevant amendment history
1. **2026-06-21-run-report-envelope-serializer** (§Read-Back Dependency Posture + §Probabilistic-Assertion Policy) — ManualCheck definition broadened to include calibration-region checks; Verdict→ReportState default mapping (`CalibrationRegion→ManualCheck`) locks the behavior. This chunk's error-masking fix ensures true MCP call errors surface as `Blocked`, preserving the distinct state semantics.

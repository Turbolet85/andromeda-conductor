
## 2026-10-03-p-075-re-round-on-incident-events — the pinned tool set named, and the incident-events read's tiers
**Section:** §1 Test Scope Summary → ipc-internal (MCP read-back client) · §2 Test Strategy → Deterministic (the `p075_round_live` clause) · §5 Integration Test Strategy → Cross-module patterns covered · §6 E2E Test Strategy → desktop-webview driver row (the driven arm's firing form)
**Change:**
- §1's RAW-result schema assertion was over a four-name list; it is over the pinned required-tool set (`required_tools` = `READBACK_TOOLS`), which the `retrieve_incident_events` read joined 2026-10-03.
- §2's `p075_round_live` leg was "read back through the four registered MCP tools (the resolve through the shipped `probe_resolve_lifecycle`)"; it now reads back through the pinned set, reading the incident's lifecycle events before and after a resolve that runs through `probe_resolve_lifecycle_timed` (the shipped probe delegates to it), with window-relative nanosecond offsets on stdout; its harvest ids are `p075_round_assertion_*` (Pulse `03ec944`) and `p075_reround_assertion_*` (Pulse S2 `cdb6c1e`).
- §5 gains a `retrieve_incident_events` bullet: the stub tier (raw S2 keys in order; empty `events` reads `total: 0`; an unknown id is a typed `VerifyError::JsonRpc`, never `Ok`), the missing-tool preflight arm on the existing precondition (in-process and against the child stub), the operator-gated live leg, the harvest-tier assertion-7 grader with its synthetic arms, and the two inline `ResolveWindow` unit tests.
- §6's bare-invocation sentence was "all four tools `absent`"; it is "every pinned required tool `absent`", its 2026-09-01 date kept.
**Why:** The pinned set grew to five this chunk; naming the set keeps these sites from re-staling, and the new read needed its tier recorded.
**Ref:** .andromeda/runs/2026-10-03T23-44-32-wrap/

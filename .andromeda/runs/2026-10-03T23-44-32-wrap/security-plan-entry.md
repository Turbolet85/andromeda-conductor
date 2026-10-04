
## 2026-10-03-p-075-re-round-on-incident-events — the pinned manifest's fifth tool and the new MCP-child-stdout read
**Section:** §Input Validation → Pinned MCP contract manifest (`contracts/`) · §Input Validation → MCP child stdout (trusted-child boundary)
**Change:**
- The pinned manifest's required-tool name set was four; it is five, adding `retrieve_incident_events`. A sidecar lacking any one blocks on the EXISTING `required tool(s) absent` precondition, so the gate's named preconditions stay five.
- The MCP-child-stdout boundary now also names the `retrieve_incident_events` by-id READ on the same `call_tool` path and controls: its response `{incident_id, events[{event_kind, occurred_unix_nano}], total, truncated}`, and its unknown-id arm, a JSON-RPC error carrying `incident not found` that lands as a typed `VerifyError::JsonRpc`, never `Ok` and never a panic.
**Why:** A Boundary widening, the MCP child stdout admitting a new input class: ratified on the founder's live word, relayed by the overseer 2026-10-03. The validation it needs is the boundary's existing bounded decode plus the typed error wall, both stub-proven.
**Kept:** the corpus-access sentences ("read-back plus the `mark_incident_resolved` lifecycle write") stay as written — the new tool is a read, which "read-back" already covers. The dated 2026-09-04 "4-of-4 tools" `ReadyState` reading stays as measured.
**Ref:** .andromeda/runs/2026-10-03T23-44-32-wrap/

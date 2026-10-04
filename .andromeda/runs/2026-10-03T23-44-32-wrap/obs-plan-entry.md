
## 2026-10-03-p-075-re-round-on-incident-events — the fifth consumed tool and the delegated-timing re-grade at S2
**Section:** §1 Obs Scope Summary → Pulse MCP server row · §4 Span / Trace Coverage → Delegated-timing family · §6 Log Coverage → Boundary-call wrappers (MCP readback)
**Change:**
- §1's Pulse MCP row and §6's MCP-readback must-log enumeration were four tools; both name five, adding `retrieve_incident_events` (a by-id read of an incident's lifecycle events, present at Pulse S2 `cdb6c1e`). §6 states it rides the bounded `verify.readback.call_tool` span with the allowlisted `mcp_tool` field, with no span name or attribute of its own.
- §4 gains a dated re-grade BESIDE the `03ec944` one: at Pulse S2 `cdb6c1e` (2026-10-03, a Linux dev host under WebKitGTK on Wayland) all four delegated bounds PASS under the same rule — P-025 one in-window rise 128.45 ms anchored 0.55 ms (the leg's 930.82 ms fall stamped after `scenario.run` closed, so outside the window), P-027 worst 715.54 ms of 2, P-037 0 ms, P-045 worst 1.0 ms of 118 — held by `p075_reround_assertion_{3..6}_*`.
**Why:** The pinned set grew this chunk, and the fresh round re-graded the delegated bounds on a new Pulse build and a new host platform; the `03ec944` grade stays as its own dated record.
**Kept:** §1's `conductor-verify` row — it states no count, and frames the seam as read-back plus the one write, which stays true of a new read.
**Ref:** .andromeda/runs/2026-10-03T23-44-32-wrap/

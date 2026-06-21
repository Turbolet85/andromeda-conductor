# Scope — MCP read-back client

**Chunk:** MCP read-back client
**Marker:** `2026-06-21-mcp-read-back-client`
**Crate seam:** `conductor-verify` (first chunk to fill the seam)
**Epoch:** 5 — Verification & read-back (chunk 1 of 6)
**P-IDs:** infrastructure — no single P-ID; this is the read-back transport every *auto* P-ID verification depends on.

## What it builds
The foundational MCP read-back **client/transport** layer in `conductor-verify`: an rmcp 1.7.0 client
that spawns the Pulse MCP sidecar over stdio, negotiates the protocol version *down* to Pulse's
hand-rolled `2024-11-05`, and exposes a typed tool-call surface for the four consumed read-back tools.
This is the session foundation only; the preflight gate, the data-dir canary, and verdict logic are
later Epoch-5 chunks that build on top of it.

Concretely:
- **Hardened sidecar spawn** — launch `andromeda-pulse-mcp` from a FIXED hard-coded program path via
  `TokioChildProcess`; propagate `ANDROMEDA_PULSE_DATA_DIR` strictly through the `.env(...)` builder
  after rejecting injection metacharacters; never interpolate into argv or a shell. Resolve the
  platform-default data-dir (`%APPDATA%\andromeda-pulse` on Windows · `$XDG_CONFIG_HOME`/`~/.andromeda-pulse`
  on Linux) when the live Pulse leaves it unset.
- **Session establishment** — `serve_client()` over the child's stdio; the client session is held for
  the suite lifetime (handle returned to callers; not yet wired into a scenario runner).
- **Version negotiation** — negotiate DOWN to `2024-11-05` (never pin a strict newer rmcp default);
  expose the negotiated `ProtocolVersion` via `peer_info()` so the downstream preflight gate can assert it.
- **Typed tool-call surface** — `list_all_tools()` + `call_tool()` wrappers for the four consumed tools
  (`query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved`) at the
  right altitude for the gate/verdict chunks to consume.
- **Typed error wall** — a `VerifyError` thiserror enum; rmcp client / transport / spawn errors fan into
  `VerifyError` variants along the verdict/error wall (transport-down / spawn-failure = harness `Err`;
  protocol / tool issues surface as typed values the *gate* chunk later maps to `Blocked`).

## Boundaries (explicitly OUT — later chunks)
- **Preflight readiness gate** — version assertion + tool-presence + data-dir canary round-trip,
  `Blocked` on mismatch → Epoch 5 chunk 2.
- **OTLP egress liveness check** (loopback `:4317` connectable) → Epoch 5 chunk 3.
- **Verdict + assertion-policy split** (Pass/Fail vs CalibrationRegion) → Epoch 5 chunk 4.
- **Expected-outcome + SLO timing model** → Epoch 5 chunk 5.
- No canary incident emission / round-trip (the gate's job; this chunk only provides the call surface).
- No scenario wiring, no report / `runs.db` writes, no CLI surface.

## Surfaces / contracts touched
- **New dependency:** rmcp 1.7.0 (`client` feature) — first use in the workspace; `Cargo.lock` must stay
  un-drifted and cargo-audit / cargo-deny green.
- **MCP tools consumed:** `query_incident_list`, `retrieve_report` (with `degraded_mode`),
  `retrieve_telemetry_slice`, `mark_incident_resolved` — the four pinned read-back tools.
- **Env:** `ANDROMEDA_PULSE_DATA_DIR` (propagated to the sidecar via `.env`) + platform-default fallback.
- **Security invariants:** subprocess-spawn hardening (fixed path, no shell, metacharacter rejection —
  the rmcp STDIO injection class, CVE-2026-30623); bounded decode on the read-back path; no keychain reads.
- **Verdict/error wall:** `VerifyError` (thiserror) lives in this seam; `Result::Err` reserved for harness faults.

## Acceptance intent (anchor for validation-1)
- The client spawns the sidecar from a fixed path, sets the data-dir ONLY via `.env`, and rejects
  injection metacharacters before spawn.
- Version negotiation lands on `2024-11-05` and the negotiated version is observable to callers.
- The four tools are reachable through a typed surface; rmcp errors become typed `VerifyError` values,
  never panics (verdict/error wall).
- Tests cover what is verifiable without a live Pulse (metacharacter rejection, data-dir default
  resolution, error mapping; an MCP stub child where the test harness supports it).
- Gates green: `Cargo.lock` un-drifted · cargo-audit / cargo-deny clean · clippy `-D` · nextest + doctest.

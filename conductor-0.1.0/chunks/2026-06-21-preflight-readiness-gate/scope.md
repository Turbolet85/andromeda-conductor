# Scope — Preflight readiness gate

**Marker:** 2026-06-21-preflight-readiness-gate
**Chunk:** Epoch 5 (Verification & read-back) — chunk 2 of 6
**Crate:** conductor-verify (builds on chunk 1's `ReadbackClient`)
**Working-route entry:** "Preflight readiness gate — pinned 2024-11-05 + tool presence + data-dir canary, Blocked on mismatch"

## What it builds
The MCP `initialize` preflight readiness gate — run once at suite start — that proves the Pulse read-back
path is trustworthy before any scenario depends on it. Three assertions, each producing a distinct `Blocked`
precondition on failure (never a silent downgrade to pass/fail/manual-check):

1. **Protocol-version pin** — assert the negotiated protocol version (`peer_info()` on the rmcp client)
   equals the manifest-pinned `2024-11-05` (Pulse's hand-rolled server version). Mismatch ⇒ `Blocked`.
2. **Required-tool presence** — assert each of the 4 read-back tools is present (`list_all_tools()`) against
   the pinned contract manifest: `query_incident_list`, `retrieve_report` (with `degraded_mode`),
   `retrieve_telemetry_slice`, `mark_incident_resolved`. Any absent ⇒ `Blocked`.
3. **Data-dir canary round-trip** — emit one known canary incident → assert `query_incident_list` returns it
   from the shared corpus → proves the `ANDROMEDA_PULSE_DATA_DIR` / workspace wiring end-to-end (tool presence
   alone does not prove read-back sees the live Pulse's corpus). Empty / missing canary ⇒ `Blocked`.

## Artifacts
- **Pinned MCP contract manifest** — a file under `contracts/` declaring the expected protocol version + the
  4 required tool names; loaded from the default path or the `CONDUCTOR_CONTRACT_MANIFEST` override
  (canonicalized at the edge); parsed + bounds-checked.
- **`ReadyState` / readiness-result value type** in conductor-verify — the serializable in-memory struct
  matching the architecture's readiness JSON (`ready`, `negotiated_protocol_version`,
  `expected_protocol_version`, `required_tools` map, `data_dir`, `canary_round_trip`, `blocked_precondition`,
  `checked_at`). A typed VALUE per the verdict/error wall — `Blocked` is a `ReportState`, returned `Ok(...)`,
  never `Result::Err`.
- **Preflight gate function(s)** wrapping the chunk-1 `ReadbackClient` (reuse the `connect_transport()`
  injection seam — the gate must be drivable over an injected transport for tests + the CI `ready:true` leg).
- **Live child-spawn integration test + stub-child MCP-server binary** (the deferred follow-up from chunk 1)
  — a real `TokioChildProcess` spawn exercising connect→initialize→preflight→canary against a stub MCP server
  that speaks `2024-11-05`, advertises the 4 tools, and round-trips the canary incident. This is where chunk
  1's deferred live-spawn integration test lands.

## Boundaries
- **IN:** the preflight gate + its 3 assertions; the contract manifest file + loader; the `ReadyState` value +
  its serialization; the `Blocked` precondition strings; the canary round-trip; the live child-spawn
  integration test + stub-child binary.
- **OUT (later chunks):** OTLP egress liveness check (next working-route chunk — `:4317` connectable, refused
  ⇒ harness `Err`); the `Verdict`/`CalibrationRegion` assertion-policy split; the expected-outcome + SLO
  timing model; the full run-report envelope serializer + `runs.db` wiring (Epoch 6 — `ReadyState` need only
  be a serializable value here, not yet persisted).
- **Invariants (must hold):** preflight integrity (never silently downgrade a failed preflight — distinct
  `Blocked` + named precondition); verdict/error wall (transport / MCP errors → typed `Blocked` / harness
  `Err`, never panic); subprocess hardening (fixed-path spawn + `.env` data-dir, established in chunk 1); the
  contract manifest is the single source of the pinned `2024-11-05` + the tool list.

## Open design points (resolve in P3 research / P4 plan)
- **Canary emission seam** — the canary must emit an incident for Pulse to ingest, then read it back. Does
  conductor-verify take a dependency on conductor-emit for the emission, or is emission orchestrated one level
  up (CLI / suite runner) with conductor-verify owning only the read-back assertion? The stub-child
  integration test sidesteps live emission; the live wiring's dependency direction must respect the
  crate-per-seam edges (a forbidden cross-seam edge won't compile).
- **Manifest format** — TOML (consistent with `scenarios/*.toml` + the project's audit/deny-clean `toml`
  crate) vs JSON. Architecture names it the "pinned MCP contract manifest file" under `contracts/` without
  fixing the format.
- **Encrypted-corpus / keychain canary leg (P-049)** — architecture notes the canary also exercises Pulse's
  encrypted `corpus.db` opened via OS keychain; a keychain / read-while-write failure ⇒ failed canary ⇒
  `Blocked`. Confirm whether this leg is in-scope now (likely a `Blocked` precondition string + stub coverage)
  or deferred to live-Pulse E2E (Epoch 10).

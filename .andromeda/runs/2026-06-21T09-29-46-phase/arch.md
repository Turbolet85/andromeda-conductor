# arch extract

## Relevance
Relevant — chunk 2 of Epoch 5 (Verification & read-back) implements the pinned readiness gate contract in conductor-verify crate.

## Constraints
1. Protocol version negotiation must pin exactly `2024-11-05` per architecture §Established Decisions [Read-Back Dependency Posture], never upgrade to rmcp client default.
2. Readiness result must conform to the canonical JSON shape (per architecture §Standard Contracts — `ready`, `negotiated_protocol_version`, `expected_protocol_version`, `required_tools` map, `data_dir`, `canary_round_trip`, `blocked_precondition`, `checked_at`).
3. MCP contract manifest file must be loaded from `contracts/` directory (or `CONDUCTOR_CONTRACT_MANIFEST` override per architecture §Occupied Resources) and declare the pinned protocol version + the four required tool names (`query_incident_list`, `retrieve_report`, `retrieve_telemetry_slice`, `mark_incident_resolved`).
4. Transport errors / MCP errors must fan out to typed `Blocked`/harness `Err` per the verdict/error wall (architecture §Conventions — Error handling); verdicts/report states are values returned `Ok(...)`, never panics.
5. Code lives in `conductor-verify` crate per architecture §Inherited Defaults (Workspace / Core Structure).
6. Canary round-trip must emit an incident via conductor-emit seam (dependency direction per crate-per-seam rule) and assert `query_incident_list` returns it, proving `ANDROMEDA_PULSE_DATA_DIR` wiring end-to-end per architecture §Occupied Resources [Environment variables].
7. Live child-spawn integration test must use `TokioChildProcess` with `.env(ANDROMEDA_PULSE_DATA_DIR, ...)` per architecture §Established Decisions [Read-Back Dependency Posture] and §Occupied Resources [Service / process names].

## Patterns to follow
1. Inject transport seam (`connect_transport()` from chunk 1's `ReadbackClient`) for test-drivability — reuse the same injection pattern that enables stubbed tests + live tests per architecture §Established Decisions (Compiler-enforced module seams).
2. Serialize `ReadyState` with serde (canonical-name round-trip) matching the readiness JSON shape per architecture §Conventions [Serialization (JSON)].
3. Express the pinned contract manifest as a declarative data structure (TOML format per chunk scope's open design point, consistent with `scenarios/*.toml` and architecture §Established Decisions [Scenario Config Format]).
4. Use rmcp 1.7.0's `peer_info()` + `list_all_tools()` + `call_tool()` directly per architecture §Stack and Technologies [MCP read-back client].

## Anti-patterns to avoid
1. Do not silently downgrade a failed preflight to pass/fail/manual-check — emit a distinct `Blocked` state with the named precondition per architecture §Conventions [Error handling] and §Standard Contracts (Readiness gate).
2. Do not use tokio's virtual clock for timestamp serialization in readiness result (`checked_at`) — use `std::time::SystemTime` per architecture §Established Decisions [Timing-Tolerance Model].
3. Do not spawn the MCP sidecar without the `ANDROMEDA_PULSE_DATA_DIR` environment variable propagated per architecture §Occupied Resources [Environment variables].

## Contract bindings
- **emit ↔ verify:** canary emission orchestration — conductor-verify calls conductor-emit to emit the canary incident before read-back assertions (dependency direction defined in this chunk's scope as an open design point; crate-per-seam rule enforces it).
- **report ↔ verify:** `ReadyState` is serialized into the run report + `runs.db` in Epoch 6 (this chunk need only define the value type; persistence deferred).

## Acceptance criteria contributions
1. (arch) Protocol version negotiation asserts `peer_info()` matches manifest's pinned `2024-11-05` — mismatch ⇒ `Blocked` per §Standard Contracts.
2. (arch) Required-tool presence asserted against manifest (4 named tools) — any absent ⇒ `Blocked` with tool name in precondition string.
3. (arch) Canary round-trip emits incident → asserts `query_incident_list` returns it → proves `ANDROMEDA_PULSE_DATA_DIR` wiring per §Occupied Resources.
4. (arch) Code lives in `conductor-verify` crate; `ReadyState` serializable; transport/MCP errors typed as `Blocked`/harness `Err` per §Inherited Defaults (Module Boundaries) + §Conventions (Error handling).
5. (arch) Live child-spawn integration test spawns stub MCP server with `TokioChildProcess` + `.env(ANDROMEDA_PULSE_DATA_DIR, ...)` per §Occupied Resources [Service / process names].

## Relevant amendment history
- **2026-06-15-structured-logging-stack** (§Occupied Resources): registered `CONDUCTOR_SERVICE_NAME` and `CONDUCTOR_ENV` env vars — relevant as readiness gate may need to propagate these to the spawned sidecar or populate `ReadyState` service context.
- **2026-06-18-exception-events-fingerprint-control** (§Infrastructure Patterns): fingerprint primitive pinned to `conductor-emit` (not faults) — relevant because chunk 1's `ReadbackClient` reuse + canary round-trip logic may touch fingerprint state (deferred to full scenario runs in Epoch 6, but canary may benefit from consistency).

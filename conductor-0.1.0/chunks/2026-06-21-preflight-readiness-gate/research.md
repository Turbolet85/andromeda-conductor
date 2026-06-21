# Codebase Research — 2026-06-21-preflight-readiness-gate

## Scope
- **Depth:** moderate · **Reads:** 11 files · **Globs/Greps:** 3 (+ 2 code-graph queries)

## Files inspected
- `crates/conductor-verify/src/lib.rs` (full) — chunk-1 surface: re-exports `ReadbackClient` + the 4 tool-name consts + `VerifyError`. Module doc already names "the preflight readiness gate, the data-dir canary round-trip" as *this* chunk's job building on the handle.
- `crates/conductor-verify/src/client.rs` (full) — `ReadbackClient`: `connect()` (live, fixed-path), **`connect_transport<T>()` (client.rs:58) the injection seam this chunk drives**, `negotiated_protocol_version()` → `peer_info().protocol_version` (client.rs:72), `list_tools()` → `list_all_tools()`, typed `query_incident_list()`/`retrieve_report()`/etc. `client_info()` pins `ProtocolVersion::V_2024_11_05` as the *offered* version (client.rs:42).
- `crates/conductor-verify/src/error.rs` (full) — `VerifyError` (`#[non_exhaustive]`): `DataDirRejected` / `Spawn` / `Initialize(Box<ClientInitializeError>)` / `Call(Box<ServiceError>)`. **The harness-fault wall** — a version diff / absent tool is explicitly NOT an error here, it is data the gate maps to `Blocked`.
- `crates/conductor-verify/src/spawn.rs` (full) — `resolve_data_dir()` (metachar rejection + platform default), `build_command()` (fixed `PULSE_MCP_PROGRAM="andromeda-pulse-mcp"`, data-dir only via `.env`). The live child-spawn test must wrap a **stub** exe, not this fixed program (Pulse's bin is not on the test host's PATH).
- `crates/conductor-verify/tests/readback.rs` (full) — **`StubPulse` pattern**: a `ServerHandler` pinning `V_2024_11_05`, advertising 1 tool, echoing calls, served over `tokio::io::duplex` and driven through `connect_transport`. The gate's tests extend this (advertise all 4 tools; return the canary incident; variant stubs for each Blocked leg).
- `crates/conductor-core/src/report_state.rs` (full) — **`ReportState::Blocked` already exists** (report_state.rs:24) with `label()`="Blocked" + `status_prefix()`="[BLOCKED]"; serde canonical-name. The gate reuses this — it does NOT define a new state enum.
- `crates/conductor-core/src/config_path.rs` (full) — **`resolve_under(base, candidate)`** (config_path.rs:17): the `CONDUCTOR_*` path-handle guard (rejects `..` / absolute / symlink-escape), already documents `CONDUCTOR_CONTRACT_MANIFEST` as a caller. Env-read + base-choice happen "at the cli edge (a later chunk)" — so this chunk's manifest loader takes an **already-resolved path**.
- `crates/conductor-core/src/lib.rs` (full) — re-exports: `resolve_under`, `CoreError`/`Result`, `init_observability`/`mint_run_id`/`now_rfc3339`/`ServiceIdentity`, `ReportState`, `RunRecord`, `Verdict`, scenario model. conductor-core is the dependency root.
- `crates/conductor-core/src/obs.rs` (full) — **`now_rfc3339()`** (obs.rs:122) = the `checked_at` stamp (RFC-3339 Z, `std::time`, NOT virtual clock); `mint_run_id()`; the field-allowlist + **host-path scrub** (`redact_value`) the gate's `data_dir` field must pass through. Bounded `verify.readback*` span family already in use.
- `Cargo.toml` (workspace, full) — members + shared deps. `serde`(derive)/`serde_json`/`toml`/`thiserror`/`tracing` all available as workspace deps; rmcp 1.7.0; `assert_fs`/`assert_cmd`/`rstest` dev-deps.
- `crates/conductor-emit/src/lib.rs` (head) — exposes raw OTLP builders (`error_trace_request`, `exception_trace_request`, …) + `TraceEmitter`/`LogsEmitter` egress. This is what a canary "emit incident" would call **if** the gate took an emit dependency.

## Graph impact (code-graph `crate_edges`)
- **Star topology, zero seam→seam edges** — every `conductor-*` seam depends on `conductor-core` only (`conductor-{cli,report,tauri,timeline}` → `conductor-core`; verify→core is the same shape, just post-dates the DB refresh). **A `conductor-verify` → `conductor-emit` edge for canary emission would be the FIRST seam→seam edge in the workspace** — a deliberate departure from the established star. This is the P4 AskUserQuestion (canary-emission seam).
- **`connect_transport()`** — the one public seam the gate builds on; 1 caller today (the stub test). New callers: the gate entrypoint + the live child-spawn test. Low blast radius.
- **`ReportState::Blocked`** — already consumed by `report_state` tests; the gate is a new producer. No change to the enum.

## Patterns detected
- **Transport-injection seam** (`client.rs:58`): the gate runs over `connect_transport`, so the in-proc duplex stub (CI) and the real `TokioChildProcess` child (live leg) drive identical code. Mirrors the chunk-1 test.
- **Verdict/error wall in practice** (`error.rs`): harness faults are `Err(VerifyError)`; reachable-but-wrong (version/tool/canary) is an `Ok` value. The gate returns `Ok(ReadyState{ ready:false, blocked_precondition })`, never `Err`, for the 3 precondition failures.
- **`#[non_exhaustive]` rmcp constructors** (verification-harness.md session note): build `ClientInfo`/params via `::new()`/`with_*()`, never struct literals (SEP-1319).
- **std::time stamps** (`obs.rs:122`): `checked_at` ← `now_rfc3339()`; never tokio's virtual clock (determinism invariant).
- **Host-path scrub** (`obs.rs` `redact_value` + `.claude/rules/observability.md`): any absolute host path (incl. `data_dir`) must be redacted before it lands in a log/report/db artifact.

## Conventions to follow
- **Reuse `ReportState::Blocked`** (conductor-core) — do not mint a parallel state; `ready:false` ⇒ dependents get `Blocked` with the named precondition.
- **Manifest = TOML** (`toml` workspace dep, consistent with `scenarios/*.toml` + arch §Scenario Config Format) under `contracts/`; serde-deserialized, bounds-checked at load (`CoreError::Config`/`Validation` flavor wall).
- **Manifest loader takes a resolved path** — `resolve_under` (env-read + base choice) is the cli edge's job (Epoch 8); this chunk reads/parses an already-validated path (+ a default `contracts/mcp-contract.toml`).
- **Serde canonical-name `ReadyState`** matching arch §Standard Contracts readiness JSON; `data_dir` field redacted via `redact_value` before serialization (artifact hygiene invariant).
- **Spans stay in the bounded `verify.readback*` family** (`.claude/rules/observability.md`) — e.g. `verify.readback.preflight`/`.canary`; `#[tracing::instrument]` like the chunk-1 methods; `Blocked` exits logged at `info` with a `blocked_precondition` field (allowlisted).
- **Test the public seam + envelope, never private fields** (`.claude/rules/testing.md`); rmcp stub over stdio (in-proc duplex + `TokioChildProcess` test-binary); zero nextest retries.

## New files to create
- `crates/conductor-verify/src/manifest.rs` — `ContractManifest` model (`expected_protocol_version` + `required_tools: [String;4]`-ish) + `load(path)`/`default_path()` + bounds-check; the pinned single-source-of-truth loader.
- `crates/conductor-verify/src/preflight.rs` — `ReadyState` (serde) + `ToolPresence` + the gate entrypoint `run_preflight(&ReadbackClient, &ContractManifest, canary…) -> Result<ReadyState, VerifyError>`; the 3 assertions → `Ok(ReadyState)` with `blocked_precondition` on failure.
- `contracts/mcp-contract.toml` — the pinned manifest: `expected_protocol_version = "2024-11-05"` + the 4 tool names. (`contracts/` does not exist yet — create it.)
- `crates/conductor-verify/tests/preflight.rs` — in-proc stub gate tests: `ready:true` happy path + one stub variant per Blocked leg (wrong version, missing tool, empty canary).
- **Stub-child MCP-server binary** for the live `TokioChildProcess` spawn test (deferred follow-up a) — a small server speaking `2024-11-05` + advertising the 4 tools + returning the canary. **Mechanism is a HOW detail (open question 4)**: recommended a feature-gated `[[bin]] stub-pulse-mcp` discovered in the test via `env!("CARGO_BIN_EXE_stub-pulse-mcp")`, vs an `examples/` entry (compiles with dev-deps but no `CARGO_BIN_EXE`).

## Files to modify
- `crates/conductor-verify/src/lib.rs` — `mod manifest; mod preflight;` + re-export `ReadyState`, `ContractManifest`, the gate entrypoint, `ToolPresence`.
- `crates/conductor-verify/Cargo.toml` — add `serde`(derive) + `toml` deps (serde_json already present); the stub-server `[[bin]]` + its feature wiring (rmcp `server` is already a dev-dep — a `[[bin]]` needs it as a feature-gated normal dep, or use `examples/`); `assert_fs` dev-dep for temp-manifest tests.

## Open questions (resolve at P4 / surface at P5)
1. **Canary-emission seam (→ P4 AskUserQuestion).** Does `conductor-verify` own the canary's *emit* half (a new `verify→emit` seam edge — the first in the workspace), or only the *read-back assertion* (canary identity is a parameter; emission orchestrated upstream by the Epoch-8 CLI / live leg)? CI can't exercise real emit→ingest anyway (the stub returns canned data); the live leg is operator-gated. Leaning: read-back-assertion-only (preserve the star topology), but this is the user's call.
2. **Version-mismatch ⇒ `Blocked`, not `Err`.** rmcp may surface a protocol incompatibility as a `ClientInitializeError` (which chunk 1 maps to `VerifyError::Initialize` = harness `Err`). The gate must map a *version* mismatch to `Ok(Blocked)` per preflight-integrity. Verify rmcp 1.7.0's `ClientInitializeError` variants at implement time; the wrong-version stub test pins this behavior.
3. **`data_dir` redaction.** The arch readiness JSON shows a literal `data_dir` path, but artifact hygiene forbids absolute host paths in artifacts. Apply `redact_value` to the serialized `data_dir` (or store a sanitized form) — confirm this doesn't defeat the canary's "proves the wiring" intent (the *fact* of a match matters, not the raw path).

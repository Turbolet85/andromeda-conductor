# Report — 2026-06-27-mcp-read-back-result-shape-adapter

**Chunk:** MCP read-back result-shape adapter — adapt conductor-verify to Pulse's hand-rolled raw `tools/call` results (raw JSON-RPC → `serde_json::Value`, not rmcp typed `CallToolResult`); faithful stub; un-mask the preflight call-error catch-all.
**Date:** 2026-06-27
**Commits:** (uncommitted — wrap commits this chunk) on `build/conductor-0.1.0`

## Changes (structured — detectors read this)
- **Files:**
  - NEW `crates/conductor-verify/src/jsonrpc.rs` (line-delimited JSON-RPC stdio session over a boxed transport).
  - NEW `crates/conductor-verify/tests/common/mod.rs` (shared in-process hand-rolled JSON-RPC stub, raw shapes).
  - rewrite `crates/conductor-verify/src/client.rs`, `src/error.rs`, `src/bin/stub_pulse_mcp.rs`, `tests/{readback,preflight,preflight_spawn}.rs`.
  - edit `crates/conductor-verify/src/preflight.rs`, `src/spawn.rs`, `src/lib.rs`, `Cargo.toml`, `Cargo.lock`.
  - `crates/conductor-run/src/lib.rs` — UNCHANGED (the `Ok(_)/Err(_)` read-back match was source-compatible with the return-type change).
- **Symbols / APIs (conductor-verify, public):**
  - `ReadbackClient` — `connect(Option<PathBuf>)`, NEW `connect_command(Command)`, `connect_transport<T: AsyncRead+AsyncWrite+…>`; `negotiated_protocol_version() -> Option<&str>` (was `Option<&ProtocolVersion>`); `list_tools() -> Vec<String>` (was `Vec<Tool>`); `call_tool`/4 wrappers `-> serde_json::Value` (was `CallToolResult`), args `Option<Value>` (was `Option<JsonObject>`).
  - `preflight_boot(Command, …)` — was generic `<T: IntoTransport>`; `run_preflight` unchanged signature.
  - `VerifyError` — REMOVED `Initialize`/`Call` (rmcp-boxed); ADDED `Transport`/`Protocol`/`Decode`/`JsonRpc{code,message}`; kept `DataDirRejected`/`Spawn`/`Manifest`.
  - NEW internal `jsonrpc::JsonRpcSession`. The 4 tool-name consts + `READBACK_TOOLS` unchanged.
  - No new IPC method / endpoint / port / socket / env var. No `runs.db`/journal/report schema change.
- **Crates / modules:** conductor-verify gains `mod jsonrpc`; no crate added/removed; no cross-crate edge change.
- **Dependencies:** **REMOVED `rmcp` 1.7.0** from conductor-verify (`[dependencies]` + `[dev-dependencies]`); `Cargo.lock` drops rmcp + its transitive tree. Added `tokio` feature `sync` (Mutex) + `serde_json` to `[dev-dependencies]`. No new external crate (tokio/serde_json already workspace deps). `stub-server` feature now `["tokio/io-std"]` (was `["rmcp/server","tokio/io-std"]`).
- **Schema / config:** none on disk. Wire-level: read-back now parses Pulse's RAW JSON-RPC `result` (`{items,…}` / `{markdown,degraded_mode}` / `{span_refs,fingerprint_refs,…}` / `{resolved,…}`) instead of the MCP `CallToolResult` envelope.
- **Coverage of new surfaces:**
  - `MCP read-back call path (hand-rolled JSON-RPC stdio)` → validation {bounded read-line (soft 16 MiB) + manifest tool/version assert · serde_json recursion-limited} · instrumentation {`verify.readback*` spans kept ✓, `mcp_tool` field} · PII {JsonRpc server message `redact_value`-sanitized before the precondition; Display shows only the code ✓} · tests {unit+integ ✓ — 67 conductor-verify incl. `--features stub-server`} · a11y n/a · tokens n/a.

## Deviations from intent
- **`conductor-run/src/lib.rs` not modified** (plan listed it) — its coarse read-back `match { Ok(_) => …, Err(_) => … }` ignores the value, so the `CallToolResult`→`Value` return change was source-compatible; no edit needed.
- **`spawn.rs` edited** (research said "reuse verbatim") — dropping rmcp removed `ConfigureCommandExt::configure`, so `build_command` switched to plain `tokio::process::Command` (identical behavior; its unit test still passes).
- **Added `tests/common/mod.rs`** (plan's only New file was `jsonrpc.rs`) — needed to share the faithful raw-shape stub across `readback.rs` + `preflight.rs` without duplication (+ `serde_json` dev-dep).
- **`MAX_LINE_BYTES` is a soft post-read bound** (16 MiB), not a hard pre-read cap — `next_line` reads to newline then length-checks; serde_json recursion-limits decode depth; a hard byte-loop cap was disproportionate for a trusted local sidecar (possible future hardening).

## Decisions & corrections
- **ARCH REVERSAL (user-confirmed at /andromeda-phase P4):** conductor-verify **drops rmcp** and speaks hand-rolled line-delimited JSON-RPC. Reverses arch §Established Decisions [MCP Read-Back Client] (rmcp was chosen to avoid hand-rolled JSON-RPC + re-derive version negotiation). **Why:** Pulse's `andromeda-pulse-mcp` is itself hand-rolled JSON-RPC and **non-MCP-compliant for `tools/call`** — it returns the raw tool payload as `result` (no `{content:[…]}`), which rmcp's typed `call_tool` rejects (`UnexpectedResponse`) on every live call. Version negotiation reduces to reading `protocolVersion` from the `initialize` result; the manifest still pins `2024-11-05`; the hardened spawn (fixed path + `.env` + injection-reject) is preserved.
- **Error-masking fix:** `run_preflight`'s `_ => CanaryOutcome::Failed` catch-all reported a genuine call error as "incident not found in corpus." Now a call/transport/JSON-RPC error → distinct `Blocked` precondition ("MCP read-back call failed: …"), vs a successful-but-marker-absent result ("incident not found in corpus").
- **Stub fidelity (root cause of the latent bug):** the old `stub_pulse_mcp` + in-process test stubs were rmcp-*servers* that wrapped results in `CallToolResult`, so CI never modeled Pulse's raw shape. Both stubs are now hand-rolled JSON-RPC emitting the raw shapes — a regression that would have caught `UnexpectedResponse`.
- **Live canary design (for the next chunk, recorded in the route + memory):** Pulse incident titles are scrubbed/generated (not a verbatim marker echo), so canary fidelity should come via `retrieve_telemetry_slice` `fingerprint_refs` + a fingerprint-storm trigger; not built here.

## Outcome
- **Acceptance criteria met.** Read-back tool calls return raw `Value` (no rmcp typing); call errors → typed `Blocked` with the true precondition; stub emits Pulse's raw shape + a regression pins it; version negotiation preserved (manifest `2024-11-05`); rmcp removed, audit/deny green; live preflight reaches a real `query_incident_list`.
- **Gates (all green):** `cargo nextest -p conductor-verify --features stub-server` **67/67** · `cargo nextest --workspace --profile ci` **419/419** · `cargo test --workspace --doc` ok · `cargo clippy --workspace --all-targets -D warnings` clean · `cargo audit` exit 0 (18 allowed warnings, the Tauri/gtk tree) · `cargo deny check` advisories·bans·licenses·sources ok (rmcp dropped from lock).
- **Live boot smoke ✓** (`conductor preflight` vs live Pulse :4317): `negotiated 2024-11-05`, all 4 tools present, `canary_round_trip:"failed"` → `"canary round-trip failed: incident not found in corpus"` — the read-back-result precondition (the call PARSED), **not** `UnexpectedResponse`. `ready:false` is by design (canary emission is the next chunk); exit 1 is the gate's go/no-go.

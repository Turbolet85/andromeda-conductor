# Report — 2026-06-21-mcp-read-back-client

**Chunk:** MCP read-back client — rmcp client over TokioChildProcess stdio, hardened fixed-path sidecar spawn (.env data-dir) + negotiate-down to 2024-11-05 (conductor-verify)
**Date:** 2026-06-21
**Commits:** none yet (commit is wrap P7; prior HEAD = bursty-train)

## Changes (structured — detectors read this)
- **Files:**
  - New: `crates/conductor-verify/src/error.rs` · `src/spawn.rs` · `src/client.rs` · `tests/readback.rs`
  - Modified: `crates/conductor-verify/Cargo.toml` · `src/lib.rs` · `Cargo.lock`
- **Symbols / APIs (new public):**
  - `conductor_verify::ReadbackClient` — `connect(Option<PathBuf>)`, `connect_transport<T,E,A>(T)` (transport-injection seam), `negotiated_protocol_version() -> Option<&ProtocolVersion>`, `list_tools()`, `call_tool(&str, Option<JsonObject>)`, + named wrappers `query_incident_list`/`retrieve_report`/`retrieve_telemetry_slice`/`mark_incident_resolved`
  - `conductor_verify::VerifyError` — thiserror enum, `#[non_exhaustive]`: `DataDirRejected`/`Spawn`/`Initialize`/`Call` (harness-faults only)
  - pub consts: `QUERY_INCIDENT_LIST`/`RETRIEVE_REPORT`/`RETRIEVE_TELEMETRY_SLICE`/`MARK_INCIDENT_RESOLVED`
  - **Ports/sockets:** none opened (Conductor stays a client; no inbound listener). Env consumed + propagated to sidecar via `.env` only: `ANDROMEDA_PULSE_DATA_DIR`. Process spawned: `andromeda-pulse-mcp` (fixed const program, `TokioChildProcess`) — the established sidecar, not a new bind.
- **Crates / modules:** `conductor-verify` — first functional fill (was a one-line stub); new modules `error`/`spawn`/`client`; integration test `tests/readback.rs`.
- **Dependencies (added to `conductor-verify`, all workspace-inherited):** `rmcp` (`client`,`transport-child-process`,`transport-io`) · `tokio` (`process`,`io-util`,`rt`,`macros`) · `thiserror` · `tracing` · `serde_json`. Dev: `rmcp` (`client`,`server`,`transport-io`) · `tokio` · `rstest`. `Cargo.lock`: rmcp 1.7.0 + transitive (240 deps total) newly resolved; un-drifted (`--locked` consistent). **NOTE:** `serde` (derive) NOT added though the plan listed it — unused; dropped for minimal audit surface.
- **Schema / config:** none.
- **Coverage of new surfaces:**
  - `MCP read-back client (rmcp over stdio)` → validation {metacharacter-reject✓ on data-dir; garde n/a — not scenario config, hand-validated at the spawn boundary} · instrumentation {`verify.readback*` tracing spans✓, `#[instrument]` `skip_all`} · PII {redacted✓ — `skip_all` keeps host paths / transport args out of logs} · tests {unit✓ (10 cases) + integ✓ (in-process rmcp duplex stub: negotiate 2024-11-05 + list + call)} · a11y {n/a — headless} · tokens {n/a — no UI}
  - `Sidecar spawn (andromeda-pulse-mcp, fixed path + .env)` → validation {injection-metacharacter reject✓} · instrumentation {covered by `verify.readback.connect` span} · PII {data-dir not logged✓} · tests {unit✓ (`build_command` sets only the data-dir env); live child-spawn integration deferred to preflight/boot chunk per the P4 test-depth decision} · a11y/tokens {n/a}

## Deviations from intent
- **Dropped `serde` dep** (plan step 1 listed serde + serde_json) — only `serde_json` is used (test arg maps); `serde` derive unused → removed for minimal audit surface (security-plan §Dependency Security). In-scope (Cargo.toml).
- **Boxed `VerifyError::Initialize`/`Call`** (`Box<ClientInitializeError>`/`Box<ServiceError>`) — clippy `result_large_err` (unboxed rmcp errors ~500 B bloat every `Result`). Lint-driven; harness-only + source-chaining preserved. (The single fix-loop iteration.)
- **`connect_transport` is `pub`** (plan implied a test seam) — integration tests are a separate crate, and the preflight/boot chunk + CI `ready:true` leg reuse it as the transport-injection point.
- **`transport-io` feature added** (plan named "client + child-process transport") — required for the in-process duplex stub transport; harmless on the client path.
- **rmcp `#[non_exhaustive]` structs** — used `Implementation::new`/`InitializeResult::new`/`CallToolRequestParams::new` builders instead of struct literals (cross-crate non_exhaustive forbids literals). Mechanical API adaptation.
- **`connect()` live child-spawn is compile-verified only** — its runtime exercise is deferred to the preflight/boot chunk (the approved P4 "in-proc stub + unit tests" decision). NOT a gap; route-sequenced (Epoch 5 chunk 2).

## Decisions & corrections
- **P4 AskUserQuestion (test depth):** user chose **"In-proc stub + unit tests"** — session/negotiation/tool-call via an in-process rmcp duplex stub; spawn hardening via pure unit tests; real `TokioChildProcess` child-spawn integration deferred to the preflight/boot chunk.
- **Negotiation is configured, not just observed (durable rmcp-1.7.0 gotcha — curation candidate):** rmcp's client default protocol is `LATEST` (2025-11-25). Pinning to `2024-11-05` requires passing a `ClientInfo` (which `impl`s `ClientHandler`) with `.with_protocol_version(V_2024_11_05)` as the `serve()` service. The default would silently negotiate UP — exactly the mismatch the preflight gate guards against.
- **VerifyError wall discipline:** a protocol-version difference / absent tool is NOT an error variant — it is reported as data (`negotiated_protocol_version()`, `list_tools()`) for the gate chunk to map to `Blocked`. The enum stays harness-fault-only.
- **rmcp 1.7.0 SEP-1319:** param structs are plural + `#[non_exhaustive]` (`CallToolRequestParams`, `InitializeRequestParams` = `ClientInfo`); their `::new`/`with_*` constructors are the only cross-crate build path.

## Outcome
- **Acceptance criteria:** all 8 met (rmcp `client` + seam layout · hardened spawn · negotiate `2024-11-05` observable · `VerifyError` wall + no panics · nextest green with stub session · `verify.readback*` spans, no OTel SDK · audit/deny clean + lock un-drifted · workspace green).
- **Gates (commands run):** `cargo nextest run -p conductor-verify` **15/15** · `cargo test -p conductor-verify --doc` 0 doctests · `cargo clippy -p conductor-verify --all-targets -- -D warnings` clean · `cargo nextest run --workspace --profile ci` **190/190** · `cargo clippy --workspace --all-targets -- -D warnings` clean · `cargo audit` exit 0 (240 deps) · `cargo deny check` advisories/bans/licenses/sources ok · `cargo metadata --locked` consistent.
- **Smoke:** skipped — no boot-path change (pure library; `conductor-verify` not yet wired into a binary/agent-run; the live spawn + `boot` preflight are the next chunk).

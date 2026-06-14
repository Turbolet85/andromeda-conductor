# Quiz II — Technical Layer Results

## Stack Decisions
- Backend Framework: N/A — no web framework. Headless Rust 2024 core; Conductor exposes no HTTP/network service of its own (gRPC client + MCP client + Tauri IPC only).
- Async Runtime: tokio 1.48.x, **current_thread flavor**, core-owned — `#[tokio::main(flavor = "current_thread")]` on the agent-run CLI path; hand-built `Builder::new_current_thread()` runtime owned by the core under Tauri (Tauri's own multi_thread runtime stays the GUI shell's concern). Rationale: zero work-stealing ⇒ emission ordering is a deterministic function of the seed; throughput headroom is irrelevant (load-testing is a non-goal; Pulse prior art saturated at ~10k spans/s).
- OTLP Emission Strategy: **opentelemetry-proto 0.32.0 raw types** (`gen-tonic` + `trace`/`metrics`/`logs` features) — hand-built `ExportTraceServiceRequest`/`ResourceSpans`/`Span`/`Status` shipped via generated `TraceServiceClient`/`LogsServiceClient` over tonic 0.14.6 / tonic-prost 0.14.6 / prost 0.14 (MSRV 1.88.0). The opentelemetry-otlp SDK exporter is rejected: near-deal-breaker for emission primitives (no control over fingerprint identity, severity boundaries, root-vs-child placement) and it spawns its own batch tasks on the runtime. Migration note: tonic 0.14 moved prost codegen to `tonic-prost-build` — budget a build-script adjustment when mining Pulse's prior-art injectors.
- Database: embedded SQLite via **rusqlite 0.38.0** + libsqlite3-sys `bundled` (SQLite 3.51.1 compiled from source, JSON1 included) — synchronous, append-mostly seam, deliberately OFF the async runtime. The brief's stale `rusqlite 0.31 / SQLite ≥3.38` pin is ratified → 0.38.0. sqlx (async-first) and SeaORM (ORM over-scope) rejected.
- ORM / Query Layer: none — raw SQL in the storage seam (right altitude for a ~one-table index).
- Cache Layer: N/A.
- Message Broker: N/A — single-process; inter-module flow is in-process Rust calls / `tokio::sync::mpsc` at most.
- Saga Transport: N/A.
- Mobile Framework: N/A — desktop-only, host-bound (holds :4317, reads local git workspace).
- AI/ML Serving: N/A — Conductor is the test driver; model behavior lives in Pulse (the system under test).
- Push Notifications: N/A as a service — in-app live updates only via Tauri 2 IPC `Channel`; Conductor must NOT emit native OS toasts (those are Pulse behavior it observes via the operator checklist).
- MCP Read-Back Client: **rmcp 1.7.0** (official Rust MCP SDK, 2026-05-13; `client` feature) — `serve_client()` over `TokioChildProcess` (stdio); `RunningService::peer_info()` exposes the negotiated `ProtocolVersion` (dated enum incl. `V_2025_11_25`) + server capabilities for the preflight assertion; typed `list_all_tools()`/`call_tool()`; pluggable `Transport` on the current_thread runtime. Hand-rolled JSON-RPC and third-party rust-mcp-sdk rejected (re-deriving version negotiation = the silent-mismatch risk class; third-party lineage risk).

## Infrastructure Decisions
- Workspace / Core Structure: **Cargo workspace** (cargo 1.85 / Rust 2024) — runtime-agnostic core `lib` + `conductor-cli` (`agent-run`) bin + Tauri bin, all calling the same core. Compiler-enforced headless/CLI/GUI split; mirrors Tauri's "core usable outside Tauri" architecture and Pulse's `crates/…` layout.
- Module Boundary Enforcement: **full crate-per-seam** — `conductor-timeline` · `conductor-emit` · `conductor-faults` · `conductor-verify` · `conductor-report` (+ `conductor-cli`, Tauri bin). `Cargo.toml` dependency edges ARE the architecture: forbidden cross-seam deps won't compile; independent `cargo build -p`/`test -p` per seam; single deployable. Optional audit tooling (cargo-modules, cargo-rail) audits, the compiler enforces.
- Deployment: local `cargo build --release` + `scripts/agent-run.sh` (headless = source of truth, co-located with Pulse); Tauri 2 bundler (v2.10.x) as optional GUI artifact, convenience not release gate. No cloud / container / serverless.
- CI/CD: GitHub Actions, build+test only (`cargo build` / nextest / clippy). Dynamic scenario proof requires live Pulse (`mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED`) — a local operator gate, not a CI gate.

## Convention Decisions
- API Style: no network API of Conductor's own. Three pinned surfaces: outbound OTLP/gRPC to 127.0.0.1:4317 (tonic); inbound MCP read-back (rmcp, version-pinned contract manifest + initialize preflight, "blocked" state on mismatch); internal core↔UI via Tauri 2 commands + `Channel` (streaming live counters). REST/GraphQL/tRPC: N/A.
- Error Handling: **thiserror 2.0.18 typed enums per seam crate + anyhow 1.0.102 at binary edges + a type-level verdict/error wall.** Verification outcomes are VALUES, never Rust errors: `enum Verdict { Pass, Fail, CalibrationRegion }`, `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` — Pulse doing the wrong thing returns `Ok(Verdict::Fail)`; `Result::Err` is reserved strictly for harness failures (config parse, transport down, MCP unreachable). rmcp 1.7.0's client error wraps into `VerifyError` variants fanned out to blocked/fail/harness-error; garde's validation `Report` becomes a `ConfigError` harness-failure class via `#[from]`. `tonic::Status` codes and MCP error responses are first-class verification inputs (an expected MCP error during a blocked-state preflight is expected, not a panic). anyhow only in `conductor-cli` / `#[tauri::command]` edges.
- Validation Library: **serde 1.0.x + garde 0.23.0** — `#[derive(Validate)]` with `range` rules for simple bounds (error fraction ∈ [0,1], non-negative durations, sane ramp factors); struct-level `#[garde(custom = …)]` fns via garde's context mechanism for cross-field invariants (p50≤p95≤p99 ordering, severity-mix sums). Validation co-located with the serde structs in their owning seam crates. validator (predecessor) and serde_valid (JSON-Schema niche obsoleted by rmcp's typed payloads) rejected.
- Module Boundaries: see Infrastructure (crate-per-seam).
- Primary Key Strategy: N/A as a fork — `runs.db` index keyed by run_id/seed/scenario per Quiz I Run-History decision.

## Recommendation Adherence
- Tokio Runtime Flavor: accepted (researched; keystone)
- Workspace / Core-Structure Shape: accepted (researched; keystone; 1 sub-agent retry — first returned summary instead of block)
- OTLP Emission Strategy: accepted (rubber-stamp from Phase 4 research — single viable option)
- SQLite Access Layer: accepted (rubber-stamp from Phase 4 research + Quiz I pin; version bump 0.31→0.38.0 ratified)
- MCP Read-Back Client Idiom: accepted (researched + WebSearch verification of rmcp 1.7.0; keystone)
- Config Validation Library: accepted (researched; keystone)
- Module Boundary Enforcement: accepted (rubber-stamp — second face of workspace decision)
- Error Handling Pattern: accepted (researched; recommendation was an enriched variant — option 1 + type-level verdict wall)
All recommendations accepted: 8/8.

## Defaults Applied
None — all fields explicitly chosen.

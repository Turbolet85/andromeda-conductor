## 1. Architecture Excerpt

### Stack (instrumentation surfaces)

- **Rust 2024 (cargo 1.85)** — Primary implementation language; deterministic single-threaded task scheduling via tokio runtime; Pulse-consistency mandate drives all observability instrumentation in the timeline engine.
- **tokio 1.48.x (`current_thread` flavor)** — Deterministic single-threaded async runtime; zero work-stealing ensures emission ordering is a function of the seed; `tokio::time` gates precise gap/silence/ramp timing for fault injection.
- **opentelemetry-proto 0.32.0** — Raw OTLP message struct generation (`ExportTraceServiceRequest`/`ResourceSpans`/`Span`/`Status`); byte-level fault control for fingerprint identity and severity boundaries; tonic codegen for wire serialization.
- **tonic 0.14.6 + tonic-prost 0.14.6 + prost 0.14** — gRPC transport layer; ships generated `TraceServiceClient`/`LogsServiceClient` to loopback `127.0.0.1:4317`; OpenTelemetry Semantic Conventions vocabulary (`service.name`, span `Status.Code=ERROR`, exception events, log `SeverityNumber`); W3C Trace Context propagates across emulated service edges.
- **rmcp 1.7.0** — Official Rust MCP SDK; version negotiation (`peer_info()` pinned to `2024-11-05`), typed `list_all_tools()`/`call_tool()` over `TokioChildProcess` stdio; silent mismatch prevention on protocol version.
- **rusqlite 0.38.0 + libsqlite3-sys 0.38.0** — Synchronous embedded SQLite index (`runs.db`), deliberately off async runtime; raw SQL (no ORM); JSON1 for fingerprint array indexing; append-mostly run-metadata tracking.
- **serde 1.0.x + garde 0.23.0** — Declarative validation of scenario config structs; range rules (error fractions, durations, ramp factors) + cross-field invariants (p50≤p95≤p99, severity-mix sums).
- **thiserror 2.0.18 + anyhow 1.0.102** — Typed per-seam error enums (`EmitError`, `VerifyError`, `ConfigError`); type-level verdict/error wall separates outcomes (`Verdict`/`ReportState` values) from harness faults (`Result::Err`).
- **Tauri 2 (bundler v2.10.x, latest 2.10.1)** — Optional GUI control panel over the same headless core; `#[tauri::command]` request/response for start/stop/picker actions; `Channel` for streaming live emission counters backend→frontend (in-app only, no native OS toasts).

### Workspace / Modules

- **conductor-core** — Runtime-agnostic engine library; usable outside Tauri; central dependency for all other crates.
- **conductor-timeline** — Deterministic seeded phase scheduler; maps gap/silence/ramp timing to `tokio::time` under `current_thread` runtime.
- **conductor-emit** — OTLP raw-type emission primitives; opentelemetry-proto + tonic message construction; gRPC egress to `127.0.0.1:4317`.
- **conductor-faults** — Fault helpers (ramps, silence, port-occupier, fingerprint generation).
- **conductor-verify** — MCP read-back client (rmcp); preflight gate (protocol version + tool presence + data-dir canary); verdict/blocked logic.
- **conductor-report** — JSONL journal per-run emission stream; Markdown run report; runs.db (rusqlite) storage seam.
- **conductor-cli** — `agent-run` binary entry point; `#[tokio::main(flavor="current_thread")]` CLI bootstrap; anyhow error bridging.
- **conductor-tauri** — Tauri 2 GUI binary; commands + Channel for live updates; owns its own multi_thread runtime separate from core's `current_thread`.

### Standard Contracts

- **MCP preflight readiness gate** (IPC method / version negotiation + tool presence check + data-dir canary) — Asserts negotiated protocol version pinned at `2024-11-05` (Pulse's hand-rolled server version), verifies required tools (`query_incident_list`, `retrieve_report` with `degraded_mode`, `retrieve_telemetry_slice`, `mark_incident_resolved`) against a pinned manifest, round-trips a canary incident through the shared corpus to prove data-dir/workspace wiring; dependent scenarios report `blocked` on version mismatch / tool absence / empty canary.
- **OTLP egress check** (gRPC stream to `127.0.0.1:4317`) — Before emission, timeline engine confirms the gRPC channel is connectable; refused transport surfaces as a harness error (`Result::Err`), not a verification verdict.
- **Run report envelope per scenario** (JSON/Markdown artifact) — Canonical states: `verdict ∈ {Pass, Fail, CalibrationRegion}` (engine verdict) and `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}` (report classification); blocked rows omit measurement fields (all null); timestamp fields use hyphenated run_id (`YYYY-MM-DDTHH-MM-SS-<suffix>` for filesystem safety) and RFC-3339 in payloads (`checked_at` in readiness result, human-facing ISO in report).

### Surfaces

**Product type** — Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO (programmatically via MCP read-back where one exists, via an operator checklist for visual claims).

### Observability Hints

**From Cross-cutting Patterns — Logging / Configuration Management:**
- Config management: Local files + environment variables only; declarative scenario config files (serde + garde, no DSL), pinned MCP contract manifest file, and Pulse-side `ANDROMEDA_PULSE_MCP_ENABLED` + `ANDROMEDA_PULSE_DATA_DIR` env flags; Conductor reserves `CONDUCTOR_*` env namespace (`CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED`); no secrets manager, no cloud config.

**From Cross-cutting Patterns — Determinism and Verdict/Error Wall:**
- Determinism discipline: seeded RNG + `current_thread` runtime guarantee "same scenario + seed ⇒ same stream shape"; wall-clock journal stamps come from `std::time::SystemTime`/`Instant`, never tokio's virtual clock, preserving journal-relative SLO math.
- Verdict/error wall: verification outcomes are typed values (`Verdict { Pass, Fail, CalibrationRegion }` and `ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }`), returned as `Ok(...)`; `Result::Err` reserved for harness faults only; `tonic::Status` codes and MCP error responses are first-class verification inputs, not panics.

**From Inherited Defaults:**
- Logging: OpenTelemetry Semantic Conventions are the shared vocabulary (`service.name`, span `Status.Code=ERROR`, exception span events, log `SeverityNumber`); no dedicated logging library — OTLP is the log surface.
- OTel SDK: opentelemetry-proto 0.32.0 raw types + tonic; no opentelemetry-otlp exporter (rejected for lack of byte-level control); raw message struct construction required for fault injection fidelity.
- Error reporting: thiserror 2.0.18 typed enums per seam; anyhow 1.0.102 at binary edges; no external error-tracking platform (Sentry/Bugsnag/Rollbar).

**Observability Design section absent** — No dedicated Observability Design section in arch. Phase 3 will derive defaults from Cross-cutting Patterns + Inherited Defaults + stack research (raw OTLP emission, seeded determinism, verdict-as-value pattern, journal-relative SLO measurement).

### Project Intent Summary

- **Core functionality:** Scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO.
- **Target users:** Solo developer, personal scale, local dev host; runs next to a real Pulse instance on the dev host.
- **Critical paths hint:** No flows explicitly enumerated in arch; derive from input.md or tests' critical paths in Phase 1 — Pulse scenarios exist as P-ID keyed configs with no narrative flow description in this excerpt.

### CI/CD Platform

- **Platform:** GitHub Actions
- **Pipeline note:** `cargo build` / cargo-nextest / `cargo clippy` on the dev OS target verify the harness compiles and unit/golden tests pass; end-to-end dynamic scenario proof requires a live Pulse (`mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED`) and is an explicit operator/local gate, not a CI gate.

### Obs-Relevant Conventions

- **Run identity keying:** `run_id` uses filesystem-safe hyphen-delimited RFC-3339 stamp (`YYYY-MM-DDTHH-MM-SS-<suffix>`); it is the `runs.db` primary key and the stem of `<run_id>.jsonl` and the run report (colons illegal in Windows filenames).
- **Timestamp encoding:** Payload instant fields use colon-delimited RFC-3339 (`2026-06-12T21:59:37Z`) for `checked_at` in readiness result and human-facing serialization of `journal_emitted_at`/`read_back_observed_at`; `runs.db` columns remain integer-millisecond journal offsets (source of truth for SLO math).
- **Data model columns (SQLite / `runs.db`):** `latency_ms` stored as INTEGER milliseconds (NULL for blocked rows); `journal_emitted_at`/`read_back_observed_at` stored as integer-millisecond journal offsets (never wall-clock); `slo_tier` is closed TEXT enum over exactly `<5s`/`<20s`/`<90s`; fingerprint arrays stored as JSON1 TEXT arrays and indexed via SQLite JSON1.
- **Scenario naming:** Keyed by Pulse capability P-ID (P-001..P-060) — "no scenario without a P-ID"; snake_case Rust identifiers, kebab-case crate names and on-disk Markdown artifact filenames.

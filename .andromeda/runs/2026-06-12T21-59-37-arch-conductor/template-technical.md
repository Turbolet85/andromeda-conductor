## Quiz II Fields

{Fields ordered by impact: high-impact decisions first. Order follows: language/runtime → framework/core structure → OTLP emission → database access → messaging → deployment → API/MCP idiom → validation → module boundaries → error handling.}

### 1. Tokio Runtime Flavor [maps to: Stack and Technologies, Cross-cutting Patterns (determinism)]
The async runtime is pinned (tokio 1.48.x) by Quiz I — but the *flavor* is an open determinism-vs-throughput fork (research Trade-off #2). Same scenario + seed must produce the same stream shape.
- **`current_thread` runtime (single-threaded, determinism-favoring)** — Removes work-stealing nondeterminism in emission ordering; reproducible-under-seed by construction. Recommended default given load-testing is an explicit non-goal.
- **`multi_thread` runtime with a small fixed worker pool** — Adds emission headroom toward :4317 at the cost of ordering reproducibility; only justified if emission saturates one core (not expected under bounded typical/high profiles).
- *[inferred from Quiz I: runtime = tokio 1.48.x — only the flavor is being asked]*

### 2. Workspace / Core-Structure Shape [maps to: Project Intent (growth), Infrastructure Patterns, Module Boundaries]
Quiz I pinned "modular monolith with hard module seams" and "headless-drivable core + thin Tauri shell + agent-run.sh." The fork is whether seams are compiler-enforced and how the headless-core/CLI/GUI split is drawn (research Trade-off #4). *(This field and Field 9 are the two faces of one decision — answer together; this one frames the binary/library split.)*
- **Cargo workspace: runtime-agnostic core `lib` crate + `agent-run` CLI `bin` + Tauri `bin`, all calling the same core** — Makes "headless-drivable, UI-as-thin-shell" literally real; core logic is reused by both entry points. Matches Tauri's own "core usable outside Tauri" architecture and Pulse's `crates/…` layout. Recommended.
- **Single crate with `mod`-level separation + a thin Tauri/CLI feature split** — One crate, GUI vs headless gated by modules/features; cheaper to start but the "thin shell over the same commands" boundary is convention-only, not enforced.

### 3. OTLP Emission Strategy [maps to: Stack and Technologies, Standard Contracts (outbound emission), Established Decisions]
The single most consequential fork in the build (research Trade-off #1) — it determines whether Conductor can perform fault injection at all. The wire protocol (OTLP/gRPC to 127.0.0.1:4317 via tonic) is pinned; this is *how spans/metrics/logs are constructed*.
- **`opentelemetry-proto` 0.32.0 raw types (`gen-tonic` + `trace`/`metrics`/`logs`), hand-built `ExportTraceServiceRequest`/`ResourceSpans`/`Span`/`Status` shipped via the generated `TraceServiceClient`/`LogsServiceClient`** — Byte-level control over root-vs-child error placement, synthesized stacktraces, line-variant-identical fingerprints (amended P-017c), exact `SeverityNumber` at the 17-boundary, and deterministic silence. The path every synthetic generator + Pulse's `load_profiles.rs` prior art uses. Strongly recommended.
- **`opentelemetry-otlp` 0.32.0 SDK exporter** — Built to export SDK-instrumentation-generated spans; gives almost no control over error placement, fingerprint identity, or exact severity numbers. A near-deal-breaker for the spec's emission primitives — listed only to record why it is rejected.

### 4. SQLite Access Layer [maps to: Data Persistence, Established Decisions, Inherited Defaults]
Storage model is pinned by Quiz I (file JSONL/Markdown artifacts as ground truth + thin `runs.db` SQLite index, synchronous, append-mostly). The fork is *which access layer* (research Trade-off #3). A sub-decision rides along: ratify the stale brief pin.
- **rusqlite 0.38.0 + libsqlite3-sys `bundled` (SQLite 3.51.1 from source, JSON1 included)** — Synchronous by design (no async entanglement for a seam Quiz I made sync), self-contained cross-platform build, raw SQL at the right altitude for a one-table index. Matches Quiz I exactly. **Sub-decision: bump the brief's stale `rusqlite 0.31 / SQLite ≥3.38` to current 0.38.0** — a free maintenance/correctness win. Recommended.
- **sqlx 0.8.6** — Async-first (drags a runtime into the deliberately-sync seam) and wants a live DB at build time for compile-time query checks; friction with no payoff for single-writer append. Rejected.
- **SeaORM 2.0 (`sea-ort-sync`)** — Full ActiveRecord ORM; sync mode removes the async requirement but entity/migration machinery is over-scoped for ~one indexed table. Rejected.

### 5. Messaging / Event System [maps to: Standard Contracts, Occupied Resources]
- **[covered by research as N/A]** — Single-process modular monolith on a local dev host; inter-module flow is in-process Rust calls / `tokio::sync::mpsc` at most. No Kafka/NATS/Redis. No field — recorded as an Established Decision "messaging: none (in-process channels only)." The only queue-like surface is the outbound OTLP stream, which is the system under test, not Conductor's own infra.

### 6. Deployment / Build Artifact [maps to: Deployment, Infrastructure Patterns]
Scale intent is personal/solo/local (Quiz I) — "deployment" means local build+run+bundle, not hosting; there is no deploy target or fork of substance. Pinned, recorded as defaults:
- **[inferred from Quiz I]** Primary: `cargo build --release` + `scripts/agent-run.sh` (headless, the source-of-truth run, co-located with Pulse on the dev host).
- **[inferred from Quiz I]** Optional GUI artifact: Tauri 2 bundler (v2.10.x) — convenience surface, not a release gate.
- **[inferred from Quiz I]** CI/CD: GitHub Actions, build+test only (`cargo build`/`nextest`/`clippy`); dynamic scenario proof is a local operator gate, not a CI gate (CI has no live Pulse). No field — no cloud infra to provision.

### 7. MCP Read-Back Client Idiom [maps to: API Style, Standard Contracts (inbound verification), Conventions]
All three interface surfaces have pinned protocols (outbound OTLP/gRPC; inbound MCP read-back; internal Tauri commands + `Channel`). REST/GraphQL/tRPC are N/A. The one open *idiom* fork is the Rust MCP client crate for the inbound verification surface (`query_incident_list` · `retrieve_report` w/ `degraded_mode` · `retrieve_telemetry_slice` · `mark_incident_resolved`); the contract posture (preflight `initialize`, pinned manifest, `blocked` state) is already set by Quiz I.
- **Official `rmcp` Rust MCP SDK client** — Maintained SDK surface for `initialize` negotiation + typed tool calls; less hand-rolled JSON-RPC plumbing to own. Recommended.
- **Hand-rolled JSON-RPC client over the four pinned tools** — Minimal dependency, full control of the wire; more code to maintain for protocol-version negotiation and error mapping into the `blocked`/`fail` distinction.
- *[inferred from Quiz I: protocols (gRPC out / MCP in / Tauri IPC internal) and contract preflight posture are pinned — only the MCP client crate is being asked]*

### 8. Config Validation Library [maps to: Validation Library, Conventions]
Validation is config-shaped, not request-shaped: declarative scenario files (per-phase duration, span rate, error fraction ∈ [0,1], latency p50/p95/p99 targets, severity mix, fingerprint variant) and the pinned MCP contract manifest, parsed via serde then bounds-checked. Framework-native validation is N/A (no web framework).
- **serde 1.0.x + garde 0.23.0** — `#[derive(Validate)]` with range/length/custom rules expresses scenario bounds declaratively beside the serde structs; trait-based composition for nested phase/service specs; no async/web coupling. Modern successor to `validator`. Recommended.
- **serde + validator** — The incumbent garde was rewritten from; still maintained, functionally adequate; choose only for ecosystem familiarity.
- **serde + serde_valid (JSON-Schema-based)** — Niche fit *only* if the MCP contract manifest is expressed as an actual JSON Schema; heavier than needed for plain scenario structs.

### 9. Module Boundary Enforcement [maps to: Module Boundaries, Project Intent (growth), Infrastructure Patterns]
The compile-time hardness of the "hard module seams" (timeline · emission · faults · verify/MCP · report · control panel). *(Two faces of Field 2 — the workspace shape — answered consistently; this field fixes the per-seam crate granularity.)*
- **Cargo workspace, one crate per seam (`conductor-timeline`, `conductor-emit`, `conductor-faults`, `conductor-verify`, `conductor-report`, + `conductor-cli` + Tauri bin)** — `Cargo.toml` dependency edges *are* the architecture; a forbidden cross-seam dep won't compile; independent `cargo build -p`/`test -p` per seam; single deployable. Strongest enforcement, mirrors Pulse's `crates/…` layout. Recommended.
- **Single crate + `mod` + `pub(crate)` visibility** — Seams as modules; cheaper but advisory-by-convention (a `mod` can reach a sibling's `pub(crate)` item) and no per-seam independent test/build.
- *(Optional audit tooling — `cargo-modules`, `cargo-rail` — is not a substitute for compiler enforcement; reach for it only if the workspace grows enough to need dependency-graph visibility.)*

### 10. Error Handling Pattern [maps to: Error Handling, Conventions, Established Decisions]
Conductor must distinguish *its own* failures (config parse, gRPC transport down, MCP unreachable) from *verification outcomes* (Pulse did the wrong thing) — the report's pass / fail / manual-check / known-residual / **blocked** states depend on matching error variants (research Trade-off #5; rides on the workspace decision).
- **thiserror 2.0.18 in seam crates + anyhow 1.0.102 at binary edges** — Typed enums per seam (`EmitError`, `VerifyError`, `ContractMismatch`) let the verification layer `match` on variants ("MCP tool absent" → blocked, "transport refused" → harness error, "wrong incident" → fail); anyhow erases at the `agent-run`/`#[tauri::command]` top level where you just surface a message + exit code. Recommended; pairs with the workspace split.
- **snafu (context-selector) + optionally miette for diagnostics** — snafu suits large systems needing rich per-call-site context; miette renders annotated, source-pointing CLI reports. Heavier than this tool needs given thiserror+anyhow cover the lib/app split — consider miette *only* for fancy operator-facing error output. Not a default.

## Section Coverage Map

| scope-arch Section | Covered by |
|---|---|
| Design Philosophy | Derived from Quiz I (personal/local scale, determinism mandate, headless-drivable intent) + Quiz II fields 1, 9 (determinism & seam enforcement principles) |
| Stack and Technologies | Quiz II fields: 1 (runtime flavor), 3 (OTLP emission), 4 (SQLite layer), 7 (MCP client), 8 (validation) — plus pinned stack from Quiz I (Rust 2024, tonic 0.14.6, Tauri 2.10.x) |
| Established Decisions | Every Quiz II answer (fields 1–4, 7–10) + field 5 (messaging: none) + field 6 (deployment, pinned) |
| Conventions | Quiz II fields: 7 (API/IPC idiom), 8 (config validation), 10 (error-handling schema → report-state mapping) |
| Standard Contracts | Quiz II fields: 3 (outbound OTLP emission contract), 7 (inbound MCP read-back contract) — preflight/manifest posture [covered by Quiz I: Read-Back Channel Dependency Posture]; field 5 (in-process channels) |
| Occupied Resources | Quiz II fields: 3 + 7 (binds outbound :4317 gRPC client / loopback) + defaults from field 2/9 crate names (`conductor-*`); env vars `ANDROMEDA_PULSE_MCP_ENABLED` [from Quiz I], `runs.db` filename [from Quiz I] |
| Infrastructure Patterns | Quiz II fields: 2 (workspace/core-CLI-GUI split), 6 (build + agent-run.sh + CI), 9 (crate-per-seam layout) |
| Cross-cutting Patterns | Quiz II fields: 1 (determinism-under-seed), 10 (error model) + Development Style [covered by Quiz I: agent-driven] |
| Project Intent | [covered by Quiz I: Growth Model = modular monolith; Platform; Core Functionality] |
| Inherited Defaults | Derived from all Quiz I + Quiz II answers (fields 1–10) |
| Existing Scopes | Always "None" — new project |

## Pre-filled Values

- Primary language: Rust 2024 [inferred from: Quiz I Primary Language]
- Async runtime: tokio 1.48.x [inferred from: Quiz I Primary Language "Rust 2024 + tokio"] — only the *flavor* (field 1) is open
- Outbound transport: OTLP over gRPC to 127.0.0.1:4317 via tonic 0.14.6 / tonic-prost 0.14.6 / prost 0.14 [inferred from: Quiz I Core Functionality + research API Style — pinned protocol; note tonic 0.14 moved prost codegen to `tonic-prost-build`]
- Inbound verification protocol: MCP read-back client [inferred from: Quiz I Read-Back Channel Dependency Posture] — only the client crate (field 7) is open
- MCP contract posture: version-pinned manifest + `initialize` preflight gate, `blocked` report state on mismatch [inferred from: Quiz I Read-Back Channel Dependency Posture]
- Internal core↔UI IPC: Tauri 2 commands + `Channel` (streaming live counters) [inferred from: Quiz I Platform — Tauri 2 control panel]
- GUI shell: Tauri 2 (v2.10.x line) [inferred from: Quiz I Platform]
- Storage model: file JSONL journal + Markdown report (ground truth) + thin embedded SQLite `runs.db` index, synchronous/append-mostly [inferred from: Quiz I Run-History & Artifact Persistence] — only the access layer + version (field 4) is open
- Database: embedded SQLite (no server DB, no multi-tenancy) [inferred from: Quiz I Scale Intent "personal/local, no cloud" + Run-History & Artifact Persistence]
- Messaging / event system: none — single-process, in-process Rust calls / `tokio::sync::mpsc` [inferred from: Quiz I Growth Model "one deployable harness" + research Message Queue N/A]
- Deployment: local `cargo build --release` + `scripts/agent-run.sh`; no cloud/container/serverless [inferred from: Quiz I Scale Intent + Development Style agent-driven]
- CI/CD: GitHub Actions build+test only; dynamic verification is a local operator gate [inferred from: Quiz I Scale Intent + research Deployment]
- Development Style: agent-driven [inferred from: Quiz I Development Style]
- Project shape: modular monolith, one deployable harness with hard module seams [inferred from: Quiz I Growth Model]
- Mobile framework: N/A — desktop-only, host-bound (holds :4317, reads local git workspace) [inferred from: Quiz I Platform; research Mobile Framework N/A] — NOT asked despite Quiz II normally binding mobile-to-language, because the platform is explicitly desktop
- AI/ML infrastructure: none for Conductor's own stack — it is the test driver, not an AI consumer; model behavior lives in Pulse (the system under test) [inferred from: Quiz I + research AI/ML N/A]
- Push notifications / real-time: in-app only via Tauri `Channel`; Conductor must NOT emit native OS toasts (those are Pulse behavior it observes) [inferred from: Quiz I + research Push Notification N/A]
- Frontend framework: deferred to design specialist (Tauri webview frontend / CSS / component libraries / visual tokens / typography are NOT a Quiz II decision)
- Auth library / test framework / logging-observability stack / a11y requirements: deferred to specialist skills (security / tests / obs / a11y) — NOT in Quiz II

## Quiz II Scope Note

Quiz II collects ARCHITECTURAL decisions: how to build it at arch level. Each answer becomes an Established Decision in the architecture document. Product decisions (what, for whom) were settled in Quiz I.

Specialist-domain decisions (auth library, test framework, logging/observability, frontend framework / CSS tools / component libraries / visual tokens / typography, a11y) are NOT in Quiz II — they're collected by specialist skills (security / tests / obs / design / a11y) downstream.

Note on this project: several arch-fork domains collapsed to N/A or pinned because Conductor is a headless, single-process, local-only Rust harness with no web server, no broker, no multi-tenancy, and two externally-pinned protocols (OTLP/gRPC out, MCP in). The genuine open forks surfaced as fields are: runtime flavor (1), workspace shape (2), OTLP emission strategy (3 — the most consequential), SQLite access layer (4), MCP client idiom (7), validation library (8), module-boundary granularity (9), and error model (10). Fields 2 and 9 are two faces of one workspace decision and should be answered consistently.

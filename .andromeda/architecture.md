## Design Philosophy

- **Determinism under a seed** — the timeline engine runs on a `current_thread` tokio runtime with zero work-stealing, so the same scenario + seed always produces the same emission stream shape; reproducibility is a property the architecture enforces in the runtime flavor, not a convention bolted on later.
- **Headless-drivable core, thin shells** — all engine logic lives in runtime-agnostic library crates that both `scripts/agent-run.sh` (the source of truth) and the optional Tauri 2 GUI call identically; the CLI/headless path is the release gate and the GUI is convenience only.
- **Compiler-enforced module seams** — the modular monolith is realized as a crate-per-seam Cargo workspace where `Cargo.toml` dependency edges *are* the architecture and a forbidden cross-seam dependency simply will not compile.
- **Outcomes are values, errors are harness faults** — verification verdicts (`Pass`/`Fail`/`CalibrationRegion`) and report states (`Pass`/`Fail`/`ManualCheck`/`KnownResidual`/`Blocked`) are typed return values; `Result::Err` is reserved strictly for Conductor's own failures (config parse, transport down, MCP unreachable), so the report can correctly classify a model-backed SUT's behavior by matching on types rather than catching exceptions.
- **Journal-relative ground truth** — every SLO is measured as `read_back_observed_at − journal_emitted_at` against an on-disk JSONL emission journal, never wall-clock-from-test-start; file artifacts are the agent-parseable ground truth and the embedded SQLite index is a thin convenience layer over them.

## Stack and Technologies

| Layer | Technology | Role |
|-------|-----------|------|
| Language / runtime | Rust 2024 (cargo 1.85, MSRV 1.94.1) | Primary implementation language; Pulse-consistency mandate |
| Async runtime | tokio 1.48.x (`current_thread` flavor) | Deterministic single-threaded scheduler for the timeline engine; core-owned runtime |
| Backend framework | N/A — no web framework | Conductor exposes no HTTP/network service of its own (gRPC client + MCP client + Tauri IPC only) |
| OTLP emission | opentelemetry-proto 0.32.0 (`gen-tonic` + `trace`/`metrics`/`logs`) | Raw hand-built OTLP message structs (`ExportTraceServiceRequest`/`ResourceSpans`/`Span`/`Status`) for byte-level fault control |
| gRPC transport | tonic 0.14.6 + tonic-prost 0.14.6 + prost 0.14 | Ships generated `TraceServiceClient`/`LogsServiceClient` over loopback gRPC |
| MCP read-back client | rmcp 1.7.0 (`client` feature) | Official Rust MCP SDK; `serve_client()` over `TokioChildProcess` stdio; version negotiation + typed tool calls |
| Database | rusqlite 0.38.0 + libsqlite3-sys 0.38.0 (`bundled` → SQLite 3.51.1, JSON1) | Synchronous, append-mostly embedded index (`runs.db`), deliberately off the async runtime |
| ORM / migrations | None — raw SQL in the storage seam | Right altitude for a ~one-table run-metadata index |
| Message broker | N/A | Single-process; inter-module flow is in-process Rust calls / `tokio::sync::mpsc` at most |
| Mobile framework | N/A | Desktop-only, host-bound (holds `:4317`, reads local git workspace) |
| AI/ML serving | N/A | Conductor is the test driver; model behavior lives in Pulse (the system under test) |
| Push / real-time | Tauri 2 IPC `Channel` (in-app only) | Streams live emission counters / target status backend→frontend; no native OS toasts |
| Desktop shell | Tauri 2 (bundler v2.10.x, latest 2.10.1) | Optional GUI control-panel artifact over the same headless core |
| Validation | serde 1.0.x + garde 0.23.0 | `#[derive(Validate)]` range rules + struct-level custom cross-field invariants for scenario config |
| Serialization (JSON) | serde_json 1.0 | Canonical-name (de)serialization for the run-report envelope + per-run JSONL journal (serde companion; report-seam runtime dep) |
| Error handling | thiserror 2.0.18 + anyhow 1.0.102 | Typed per-seam error enums in library crates; type-erased `Result` at binary edges |
| Self-observation | tracing 0.1.44 + tracing-subscriber 0.3.23 | Structured JSON self-observation logs; NOT an OTel SDK (OTLP is the PRODUCT emission); detail in obs-plan §3 |
| Build / packaging | Cargo workspace, `cargo build --release` + Tauri 2 bundler | Local release binary (source of truth) + optional GUI bundle |
| CI/CD | GitHub Actions (`cargo build` / nextest / clippy) | Build + test gating only; dynamic scenario proof is a local operator gate |
| Code quality | clippy + cargo-nextest (optional: cargo-modules, cargo-rail) | Linting, test running, and module-graph auditing |

## Established Decisions

- **[Language / Runtime] Rust 2024 + tokio 1.48.x:** Pinned by brief for Pulse-consistency; tonic/prost require a tokio reactor and the deterministic phase scheduler maps cleanly to `tokio::time` for exact gap/silence/ramp timing.
- **[Async Runtime Flavor] tokio `current_thread`, core-owned:** Zero work-stealing makes emission ordering a deterministic function of the seed; throughput headroom is irrelevant because load-testing is an explicit non-goal (Pulse prior art saturated ~10k spans/s). `#[tokio::main(flavor = "current_thread")]` on the CLI path; hand-built `Builder::new_current_thread()` owned by the core under Tauri so Tauri's own multi_thread runtime stays the GUI shell's concern.
- **[OTLP Emission Strategy] opentelemetry-proto 0.32.0 raw types over the SDK exporter:** The `opentelemetry-otlp` exporter is a near-deal-breaker — it offers no control over fingerprint identity, severity boundaries, or root-vs-child error placement and spawns its own batch tasks on the runtime. Raw message structs give byte-level control, which fault injection requires; this is the single most consequential fork in the build.
- **[Backend Framework] None (no web framework):** Conductor is a gRPC client + MCP client + Tauri IPC host, not a server; it exposes no HTTP/network surface of its own, so Axum/Actix and REST/GraphQL/tRPC are all out of scope.
- **[Database] rusqlite 0.38.0 with `bundled` SQLite, synchronous:** Quiz I pinned file artifacts + a thin SQLite index; rusqlite keeps the storage seam synchronous (no async-runtime entanglement) and `bundled` yields a self-contained cross-platform build with JSON1 for indexing fingerprint arrays. The brief's stale `rusqlite 0.31 / SQLite ≥3.38` pin is ratified to 0.38.0 (a free maintenance/correctness win). sqlx (async-first) and SeaORM (ORM over-scope) were rejected.
- **[ORM] None — raw SQL:** `runs.db` is ~one indexed table of run metadata; an ORM's entity/migration machinery is weight with no payoff for hand-written append + a handful of cross-run SELECTs.
- **[MCP Read-Back Client] rmcp 1.7.0:** Official Rust MCP SDK with version negotiation (`peer_info()` exposing the dated `ProtocolVersion`) and typed `list_all_tools()`/`call_tool()`; hand-rolled JSON-RPC and third-party rust-mcp-sdk were rejected because re-deriving version negotiation is the silent-mismatch risk class the preflight exists to prevent. **Caveat verified against Pulse source:** Pulse's MCP *server* is itself hand-rolled JSON-RPC (not rmcp) speaking the older `2024-11-05` protocol with a minimal `capabilities` object (`crates/mcp-server/src/jsonrpc.rs:7`), so the rmcp *client* MUST negotiate *down* to `2024-11-05` and tolerate minimal capabilities — a strict newer-version client default is precisely the silent mismatch the preflight pins against; the contract manifest fixes the expected version at `2024-11-05`.
- **[Validation Library] serde 1.0.x + garde 0.23.0:** garde (the modern rewrite of `validator`) expresses scenario bounds declaratively next to the serde structs — `range` rules for simple bounds (error fraction ∈ [0,1], non-negative durations, sane ramp factors) and `#[garde(custom = …)]` for cross-field invariants (p50≤p95≤p99 ordering, severity-mix sums) — with no async or web-framework coupling. validator (predecessor) and serde_valid (JSON-Schema niche) were rejected.
- **[Error Handling] thiserror 2.0.18 in seam crates + anyhow 1.0.102 at edges, plus a type-level verdict/error wall:** The report's states require *matching on error variants* — "MCP tool absent" → `Blocked`, "transport refused" → harness error, "wrong incident" → `Fail` — so verification outcomes are VALUES (`Verdict`/`ReportState`), never Rust errors; `Result::Err` is reserved for harness failures only. `tonic::Status` codes and MCP error responses are first-class verification inputs. anyhow lives only in `conductor-cli` / `#[tauri::command]` edges.
- **[Module Boundaries] Crate-per-seam Cargo workspace:** In Rust, crates are compiler-enforced visibility boundaries; one workspace with a runtime-agnostic `conductor-core` plus the seam crates `conductor-timeline`/`conductor-emit`/`conductor-faults`/`conductor-verify`/`conductor-report` (+ `conductor-cli` bin and the `conductor-tauri` bin) makes a forbidden cross-seam dependency fail to compile, gives independent `cargo build -p`/`test -p` per seam, ships a single deployable, and mirrors Pulse's `crates/…` layout. The compiler enforces; cargo-modules/cargo-rail only audit.
- **[Workspace / Core Structure] Runtime-agnostic core library + CLI bin + Tauri bin:** This is the structural decision that makes "headless-drivable" real — engine logic lives in a core usable outside Tauri, and both `agent-run` and Tauri commands call it, enforcing the headless/CLI/GUI split at compile time.
- **[Real-time Strategy] Tauri 2 IPC `Channel` for in-app live updates only:** Streams live counters / target status backend→frontend without per-message JSON overhead; FCM/APNs/web-push are irrelevant on a local tool, and Conductor must NOT emit native OS toasts because those are Pulse behavior it observes via the operator checklist.
- **[Message Broker] N/A:** Single-process modular monolith; inter-module flow is in-process Rust calls / `tokio::sync::mpsc` channels at most — no Kafka/NATS/Redis. The only queue-like surface is the OTLP gRPC stream to Pulse, which is the SUT, not Conductor's infrastructure.
- **[Deployment] Local `cargo build --release` + `scripts/agent-run.sh`:** Headless is the source of truth, co-located with Pulse (which binds loopback `:4317` only, ruling out remote/containerized execution); the Tauri 2 bundler (v2.10.x) produces the GUI artifact as convenience, not a release gate. No cloud / container / serverless.
- **[CI/CD] GitHub Actions, build + test only:** `cargo build` / nextest / clippy on the dev OS target verify the harness compiles and unit/golden tests pass; end-to-end dynamic scenario proof requires a live Pulse (`mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED`) and is an operator/local gate, not a CI gate, because CI has no Pulse instance.
- **[Timing-Tolerance Model] Hybrid journal-relative deadline + tier-scaled bounded slack:** Latency is always `read_back_observed_at − journal_emitted_at` (never wall-clock-from-test-start); deterministic/hard SLOs get a tolerance band scaled to hardware-profile-aware tiers (<5s / <20s / <90s), while sample-count floors (50-sample latency / 10-span error-rate) and model-interpretive timings route to the calibration-region bucket instead of hard-failing. This reconciles determinism-under-seed with the reality that Pulse runs in real time as a black box.
- **[Read-Back Dependency Posture] Version-pinned MCP contract + preflight gate (protocol `2024-11-05` · tool presence · data-dir canary) + per-scenario explicit degrade:** At suite start, MCP `initialize` asserts the negotiated protocol version — **pinned at `2024-11-05`, Pulse's actual hand-rolled server version, NOT the rmcp client default** — plus presence of each required tool (`query_incident_list` · `retrieve_report` with `degraded_mode` · `retrieve_telemetry_slice` · `mark_incident_resolved`) against a pinned manifest; the gate then **round-trips a canary incident** (emit one known incident → assert `query_incident_list` returns it from the shared corpus) to prove the data-dir/workspace wiring before any scenario trusts read-back. On absence / version-mismatch / empty-canary, dependent auto scenarios report as **blocked** (a distinct state, never silently downgraded to pass/fail/manual-check) with the named precondition. The two remaining report states are triggered as follows: **ManualCheck** is the terminal state for visual/operator-checklist claims that have no programmatic MCP read-back (the operator-checklist path) and so cannot be auto-verified; **KnownResidual** is the state for a check whose deviation matches a pre-accepted residual (e.g. a `retrieve_report` result returned under `degraded_mode`, or a fingerprint flagged as an accepted known-residual), distinguishing an accepted residual from a real `Fail` in the run report.
- **[Probabilistic-Assertion Policy] Two-state split (hard-fail / calibration-region):** Deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing) are hard pass/fail; model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting) are calibration-region + report-for-human and never hard-failed on exact values — because exact-match assertions cannot fairly judge a model-backed detector.
- **[Run-History Persistence] File artifacts + embedded SQLite index:** Per-run JSONL emission journal + Markdown run report stay on disk as agent-parseable ground truth; a thin `runs.db` indexes run_id · seed · scenario · P-IDs · verdict · fingerprints · timestamps for cross-run/seed-to-seed queries and the P-036 fingerprint-recurrence ("Previously seen") check, kept synchronous and append-mostly off the async runtime.

## Conventions

**Interface surfaces (three pinned, no network API of Conductor's own):**
- **Outbound emission:** OTLP over gRPC to `127.0.0.1:4317` via tonic 0.14.6 / tonic-prost 0.14.6; OTLP wire format built from opentelemetry-proto raw types. OpenTelemetry Semantic Conventions are the shared vocabulary (`service.name`, span `Status.Code=ERROR`, exception span events with `exception.type`/`message`/`stacktrace`, log `SeverityNumber` at boundaries); W3C Trace Context propagates across emulated service edges.
- **Inbound verification:** MCP read-back via rmcp 1.7.0 over `TokioChildProcess` stdio; a version-pinned contract manifest + `initialize` preflight gate, with the **blocked** report state on mismatch.
- **Internal core↔UI:** Tauri 2 `#[tauri::command]` request/response for start/stop/picker actions; `Channel` for streaming live counters. REST / GraphQL / tRPC are N/A.

**Error handling:** Typed `thiserror` enums per seam crate (e.g. `EmitError`, `VerifyError`, `ContractMismatch`, `ConfigError`); `anyhow` only at `conductor-cli` / `#[tauri::command]` edges. A type-level verdict/error wall separates verification outcomes from harness failures: `enum Verdict { Pass, Fail, CalibrationRegion }` and `enum ReportState { Pass, Fail, ManualCheck, KnownResidual, Blocked }` are returned as `Ok(...)`; `Result::Err` means a harness fault (config parse, transport down, MCP unreachable). rmcp client errors fan out into `VerifyError` variants (blocked/fail/harness-error); garde's validation `Report` becomes a `ConfigError` harness-failure class via `#[from]`. `tonic::Status` codes and MCP error responses are first-class verification inputs, not panics.

**Naming patterns:** snake_case Rust source files and identifiers (per Rust convention); crates named `conductor-<seam>` (kebab-case crate names, snake_case module paths); scenarios keyed by Pulse capability P-ID (P-001..P-060) — "no scenario without a P-ID". On-disk artifacts use kebab-case Markdown filenames (e.g. `coverage-matrix.md`, run report) and per-run JSONL journals.

**Config conventions:** Declarative scenario config (no scenario DSL) deserialized with serde and bounds-checked with garde at load; validation co-located with the serde structs in their owning seam crates. Durations are non-negative; error fractions ∈ [0,1]; ramp factors validated sane; latency targets enforce p50≤p95≤p99 ordering and severity-mix sums.

**Data model conventions (SQLite / `runs.db`):** Raw SQL, no migration framework; primary index keyed by `run_id` (with `seed`/`scenario`) per the Run-History decision — no synthetic UUID/serial PK is introduced. Timestamps are stored as journal-relative values whose source of truth is the JSONL journal; wall-clock stamps for the journal come from `std::time::SystemTime`/`Instant` (never tokio's virtual clock, which would break journal-relative SLO math). Column types are fixed on first write (no migration framework to coerce later): `latency_ms` is INTEGER milliseconds (NULL for blocked rows); `journal_emitted_at`/`read_back_observed_at` are stored as the same integer-millisecond journal offsets the SLO math consumes, not ISO strings; `slo_tier` is a closed TEXT enum over exactly `<5s`/`<20s`/`<90s`. Fingerprint arrays are stored as a JSON1 TEXT array of fingerprint strings and indexed via SQLite JSON1.

## Standard Contracts

Conductor exposes no HTTP service, so it has no `/health` or `/ready` HTTP endpoints. The equivalent contracts every scenario/module must follow are the **MCP preflight readiness gate** and the **run report artifact envelope** — these are the shared shapes scopes depend on.

**Readiness gate (MCP `initialize` preflight + data-dir canary, run once at suite start).** Asserts the negotiated protocol version (pinned at `2024-11-05` — Pulse's hand-rolled server version, `jsonrpc.rs:7`) and required-tool presence against the pinned contract manifest, then **round-trips a canary incident** through the shared corpus (emit one known incident → assert `query_incident_list` returns it) to prove the data-dir/workspace wiring end-to-end — tool presence alone does not prove read-back sees the live Pulse's corpus. Conceptual readiness result shape (in-memory / serialized into the run report):

```json
{
  "ready": false,
  "negotiated_protocol_version": "2024-11-05",
  "expected_protocol_version": "2024-11-05",
  "required_tools": {
    "query_incident_list":     "present",
    "retrieve_report":         "present",
    "retrieve_telemetry_slice":"absent",
    "mark_incident_resolved":  "present"
  },
  "data_dir": "<ANDROMEDA_PULSE_DATA_DIR — must equal the live Pulse's>",
  "canary_round_trip": "ok",
  "blocked_precondition": "mcp-server cargo feature + ANDROMEDA_PULSE_MCP_ENABLED + ANDROMEDA_PULSE_DATA_DIR == live Pulse's data-dir",
  "checked_at": "2026-06-12T21:59:37Z"
}
```

When `ready` is false, every dependent auto scenario is emitted into the report with `state: "blocked"` and the named precondition — never pass/fail/manual-check. A blocked row populates only the identity fields (`run_id`, `seed`, `scenario`, `p_ids`, `slo_tier`); `verdict`, `journal_emitted_at`, `read_back_observed_at`, `latency_ms`, and `fingerprints` are emitted as JSON `null` (and stored NULL in `runs.db`) because the scenario was never measured. Cross-run latency/percentile and P-036 fingerprint queries MUST exclude NULL-measurement rows so blocked scenarios never contribute phantom values. The canary also exercises the encrypted-corpus path: Pulse's `corpus.db` is encrypted at rest (P-049) and the sidecar opens it via the OS keychain (`OsKeychainBackend("com.andromeda.pulse")`, same OS user as the live Pulse) as a second connection while Pulse holds it open for writing — a keychain failure or read-while-write fault surfaces as a failed canary ⇒ **blocked**, never a false pass (host prior art: Pulse `crates/mcp-server/tests/sidecar_subprocess.rs`).

**Liveness equivalent (OTLP egress check).** Before emission, the timeline engine confirms the gRPC channel to `127.0.0.1:4317` is connectable; a refused transport surfaces as a harness error (`Result::Err`), not a verification verdict.

**Run report envelope (per scenario check).** Every check classifies into the canonical states; the Markdown report + `runs.db` row share this shape:

```json
{
  "run_id": "2026-06-12T21-59-37-...",
  "seed": 424242,
  "scenario": "error-baseline-spike",
  "p_ids": ["P-009", "P-010"],
  "verdict": "Pass",
  "state": "Pass",
  "journal_emitted_at": "...",
  "read_back_observed_at": "...",
  "latency_ms": 1840,
  "slo_tier": "<5s",
  "fingerprints": ["..."]
}
```

`verdict ∈ {Pass, Fail, CalibrationRegion}`; `state ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked}`.

**Timestamp formats.** `run_id` uses a filesystem-safe hyphen-delimited stamp (`YYYY-MM-DDTHH-MM-SS-<suffix>`) because it is the `runs.db` primary key and the stem of `<run_id>.jsonl` / the run report — colons are illegal in filenames on the Windows dev host. In-payload instant fields use colon-delimited RFC-3339 (`2026-06-12T21:59:37Z`): `checked_at` in the readiness result, and the human-facing run-report serialization of `journal_emitted_at`/`read_back_observed_at` (whose `runs.db` columns remain the integer-millisecond journal offsets declared in the Data model conventions, not ISO strings). The encodings denote the same instant; only `run_id` is constrained to the hyphen form.

**Live update channel.** Backend→frontend streaming uses the Tauri 2 IPC `Channel` (live emission counters + target status); there is no SSE/WebSocket/polling HTTP surface.

## Occupied Resources

**Ports:**
- `127.0.0.1:4317` — OTLP/gRPC **egress target** (Pulse's loopback ingest). Conductor is the client; the port is owned by Pulse, but Conductor must be co-located with it. The port-occupier fault scenario also binds `:4317` to exercise Pulse's reaction.
- `:4318` (OTLP/HTTP) — not used; gRPC-only loopback is the pinned path.
- No inbound listener of Conductor's own (no API port, no DB port — SQLite is embedded/in-process, no frontend dev-server port — the Tauri webview is bundled).

**Interface routes / surfaces (no HTTP routes):**
- MCP tools consumed: `query_incident_list`, `retrieve_report` (with `degraded_mode`), `retrieve_telemetry_slice`, `mark_incident_resolved`.
- Tauri commands (internal IPC): start/stop, scenario/suite picker, run-report view, operator-pause prompt; one `Channel` for live counters.
- Route prefixes: none — Conductor exposes no HTTP surface; the four consumed MCP tool names and the Tauri command names above are the complete route-equivalent set, and the `conductor-*` crate-name prefix is the reserved namespace.

**Service / process names:**
- `conductor-cli` (the `agent-run` binary) — headless source of truth.
- Conductor Tauri app (GUI bin) — optional control panel.
- Launched child process: Pulse MCP server (`andromeda-pulse-mcp`) via `TokioChildProcess` (stdio), gated on its `mcp-server` feature, **spawned with `ANDROMEDA_PULSE_DATA_DIR` set to the live Pulse's data-dir** so it reads the same corpus (the established `.env(...)` spawn pattern in Pulse's `tests/sidecar_subprocess.rs`).

**Crate names (workspace members):** `conductor-core` (the runtime-agnostic engine library every other crate depends on), `conductor-timeline`, `conductor-emit`, `conductor-faults`, `conductor-verify`, `conductor-report`, `conductor-cli`, plus the `conductor-tauri` bin.

**On-disk artifacts / database:**
- `runs.db` — embedded SQLite index (run_id · seed · scenario · P-IDs · verdict · fingerprints · timestamps).
- Per-run JSONL emission journal + per-run Markdown run report.
- `coverage-matrix.md` — all 60 P-IDs classified.
- Pinned MCP contract manifest file.

**Environment variables:** Conductor reserves the `CONDUCTOR_*` env-var namespace for configuration handles over artifacts and concepts this document already defines:
- `CONDUCTOR_RUNS_DIR` — overrides the default `runs/` artifact directory (JSONL journals, Markdown reports, runs.db).
- `CONDUCTOR_SCENARIOS_DIR` — overrides the default `scenarios/` config directory.
- `CONDUCTOR_CONTRACT_MANIFEST` — overrides the default pinned MCP contract manifest path under `contracts/`.
- `CONDUCTOR_SEED` — seed override for headless runs (equivalent CLI flag on conductor-cli takes precedence; env supports scripts/agent-run.sh parameterization).
- `ANDROMEDA_PULSE_MCP_ENABLED` — Pulse-side flag that gates the live MCP read-back path (operator/local gate); asserted **not** set by Conductor itself.
- `ANDROMEDA_PULSE_DATA_DIR` — the live Pulse-under-test's data directory. **Conductor MUST propagate it to the spawned `andromeda-pulse-mcp` sidecar** so the sidecar reads the SAME `{data_dir}/corpus/corpus.db` the live Pulse writes; the read-back tools filter incidents by `workspace_root = data_dir` (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:74` + `tools.rs:336`), so a mismatch makes every `query_incident_list` come back empty (false pass-as-empty / perpetual blocked). If the live Pulse leaves it unset, Conductor resolves and passes the identical platform default (`%APPDATA%\andromeda-pulse` on Windows · `$XDG_CONFIG_HOME`/`~/.andromeda-pulse` on Linux). The preflight canary verifies the wiring.
- Pulse-side `mcp-server` cargo feature must be built — a precondition Conductor asserts and reports as the `blocked` precondition string.
- No `DATABASE_URL` (SQLite path is a local file), no secrets/cloud env vars (local-only tool).

## Infrastructure Patterns

**Build system:** Cargo (cargo 1.85, Rust 2024 edition, MSRV 1.94.1). Lint with `cargo clippy`; test with `cargo test` / cargo-nextest (consistent with Pulse's static layer). `bundled` SQLite compiles SQLite C from source (longer cold builds, fully self-contained). tonic 0.14 moved prost codegen to `tonic-prost-build` — build scripts use that crate (budget a small adjustment when mining Pulse's prior-art injectors that predate the split). Optional `cargo-modules` / `cargo-rail` audit the dependency graph.

**Deployment model:** Local-only — `cargo build --release` produces the `conductor-cli` binary run beside Pulse via `scripts/agent-run.sh`; the Tauri 2 bundler (v2.10.x) produces an optional ~3 MB GUI installer. No Docker / Compose / Kubernetes / serverless — Pulse binds loopback `:4317`, so execution is necessarily co-located on the dev host.

**Directory structure (crate-per-seam Cargo workspace):**

```
conductor/
├─ Cargo.toml                 # workspace manifest (members + shared deps)
├─ Cargo.lock
├─ rust-toolchain.toml        # pin Rust 2024 / MSRV 1.94.1
├─ scripts/
│  └─ agent-run.sh            # headless source-of-truth entrypoint
├─ crates/
│  ├─ conductor-core/         # runtime-agnostic engine library (usable outside Tauri)
│  ├─ conductor-timeline/     # deterministic seeded phase scheduler (tokio::time)
│  ├─ conductor-emit/         # OTLP raw-type emission primitives (opentelemetry-proto + tonic)
│  ├─ conductor-faults/       # fault helpers (ramps, silence, port-occupier, fingerprints)
│  ├─ conductor-verify/       # MCP read-back client (rmcp), preflight gate, verdict logic
│  ├─ conductor-report/       # JSONL journal + Markdown report + runs.db (rusqlite) storage seam
│  ├─ conductor-cli/          # `agent-run` bin (#[tokio::main(flavor="current_thread")] + anyhow)
│  └─ conductor-tauri/        # Tauri 2 GUI bin (commands + Channel; owns its own runtime)
├─ scenarios/                 # declarative scenario config (serde + garde), one per P-ID
├─ contracts/                 # pinned MCP contract manifest
├─ runs/                      # per-run artifacts: <run_id>.jsonl journal + <run_id>.md report (run_id-stemmed, never overwritten)
│  └─ runs.db                 # embedded SQLite cross-run index
├─ coverage-matrix.md         # all 60 P-IDs classified (definition of done)
└─ .github/workflows/         # GitHub Actions: build + test + clippy
```

**CI/CD approach:** GitHub Actions on the dev OS target runs `cargo build`, `cargo nextest`/`cargo test` (incl. golden tests), and `cargo clippy` — build + test gating only. Dynamic end-to-end scenario proof (error-baseline-spike, fingerprint-storm, restart-suppression) requires a live Pulse with the `mcp-server` feature + `ANDROMEDA_PULSE_MCP_ENABLED` and is an explicit operator/local gate, not a CI gate.

## Cross-cutting Patterns

- **Config management:** Local files + environment variables only — declarative scenario config files (serde + garde, no DSL), a pinned MCP contract manifest file, and the Pulse-side `ANDROMEDA_PULSE_MCP_ENABLED` + `ANDROMEDA_PULSE_DATA_DIR` env flags (the latter propagated to the spawned sidecar so it reads the live Pulse's corpus). Conductor's own configuration handles live under the reserved `CONDUCTOR_*` env namespace (`CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_CONTRACT_MANIFEST`, `CONDUCTOR_SEED`); precedence is files over env defaults, and CLI flags over env. No secrets manager and no cloud config (local-only tool, no credentials).
- **Development Style:** **agent-driven** — the headless `scripts/agent-run.sh` path is the source of truth and the release gate; the GUI is a thin shell over the same core commands. (This signal is read downstream by tests/obs/setup-project specialists.)
- **Determinism discipline:** seeded RNG + `current_thread` runtime guarantee "same scenario + seed ⇒ same stream shape"; wall-clock journal stamps come from `std::time::SystemTime`/`Instant`, never tokio's virtual clock, preserving journal-relative SLO math.
- **Verdict/error wall:** the cross-cutting rule that verification outcomes are typed values and `Result::Err` is reserved for harness faults applies in every seam crate, so the report can classify outcomes by `match` rather than by catching errors.
- **Trust boundary:** Conductor runs entirely on the loopback dev host with no inbound listener of its own — it is a gRPC/MCP *client*, not a server. Its outbound surfaces are OTLP/gRPC egress to `127.0.0.1:4317` (`:4318` unused) and an MCP read-back session to a Pulse server it launches over `TokioChildProcess` stdio (trusting that child's stdout as the read-back source). The single deliberate exception is the port-occupier fault scenario, which intentionally *binds* `:4317` to exercise Pulse's reaction — the only case where Conductor opens a port. There are no secrets, no `DATABASE_URL`, and no cloud credentials (the SQLite path is a local file).
- **Scope law:** "no scenario without a P-ID" and the stated non-goals (not a load-tester, no Pulse process management, no UI automation) are guards against feature creep; the complete `coverage-matrix.md` (all 60 P-IDs, zero gaps) is the definition of done.

## Project Intent

- **Product type:** Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO (programmatically via MCP read-back where one exists, via an operator checklist for visual claims).
- **Scale intent:** Personal — solo developer, local dev host, no cloud, no multi-tenancy; runs next to a real Pulse instance on the dev host.
- **Growth model:** Modular monolith — one deployable harness with hard, compiler-enforced module seams (timeline engine · emission primitives · fault helpers · verification/MCP read-back · run report · control panel).
- **How new functionality is added:** New scenarios are added as declarative config keyed to a Pulse P-ID (no scenario without a P-ID); new module seams are added as new workspace crates with explicit `Cargo.toml` dependency edges. Downstream scopes are added via `/andromeda-scope-arch`, inheriting the conventions in this document.
- **Template patterns:** N/A — single harness, not a fleet of services.

## Inherited Defaults

- **Language / runtime:** Rust 2024 (cargo 1.85, MSRV 1.94.1) + tokio 1.48.x `current_thread`.
- **Backend framework:** None — headless Rust core, no HTTP/network service (gRPC client + MCP client + Tauri IPC).
- **Database:** rusqlite 0.38.0 + `bundled` SQLite 3.51.1 (JSON1), synchronous raw SQL, no ORM/migrations.
- **Interface style:** OTLP/gRPC egress to `127.0.0.1:4317` (tonic 0.14.6); MCP read-back via rmcp 1.7.0 against Pulse's hand-rolled `2024-11-05` server, version-pinned manifest + preflight canary (`blocked` on mismatch/empty), sidecar spawned with the live Pulse's `ANDROMEDA_PULSE_DATA_DIR`; internal Tauri 2 commands + `Channel`. No REST/GraphQL/tRPC.
- **Validation:** serde 1.0.x + garde 0.23.0, co-located with structs in owning seam crates.
- **Error handling:** thiserror 2.0.18 typed enums in seam crates + anyhow 1.0.102 at binary edges; verdicts/report states are values, `Err` is harness-only.
- **Module boundaries:** Crate-per-seam Cargo workspace; forbidden cross-seam deps won't compile.
- **Deployment:** Local `cargo build --release` + `scripts/agent-run.sh` (source of truth); optional Tauri 2 (v2.10.x) GUI bundle. No cloud/container/serverless.
- **CI/CD:** GitHub Actions build + test (cargo build / nextest / clippy); dynamic proof is a local operator gate.
- **Development Style:** agent-driven.

## Existing Scopes

None — new project. Scopes will be added via /andromeda-scope-arch.

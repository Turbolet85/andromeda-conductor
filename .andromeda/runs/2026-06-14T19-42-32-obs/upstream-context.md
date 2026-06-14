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


## 2. Security Plan Excerpt

### Security Tier

- **Tier:** Minimal (0)
- **Justification:** Single-developer, local-only, no-cloud, no-multi-tenancy tool with no user accounts (auth "none"), where the only persisted data is self-generated synthetic test telemetry and run-metadata in an embedded SQLite index and on-disk artifacts (no PII/payment/health/credentials owned by Conductor), and there is zero network exposure (loopback gRPC/MCP client with no inbound listener of its own).

### Logging-Sensitive Vectors

(No logging-sensitive vectors enumerated — Phase 3 will apply default-not-log discipline to High/Critical data classifications.)

### Anti-Patterns Rejected

- **Account enumeration in auth flows** — rejected because: single-user local tool (no user accounts); if an auth/recovery surface is ever added, this ban becomes active immediately.
- **Weak recovery questions** — rejected because: not applicable to Minimal local tool; forward-guardrail if authentication is ever introduced.
- **Skipping rate limiting on auth endpoints** — rejected because: no authenticated surface exists today; if Conductor ever grows an authenticated control surface beyond local IPC, rate limiting is mandatory.
- **Interactive login in headless path** — rejected because: headless `scripts/agent-run.sh` is agent-driven and must remain non-interactive (auth prompt would break the release gate).
- **Path handles without canonicalize + bounds-check** — rejected because: `CONDUCTOR_*` env path handles sit outside garde struct validation and must be validated at the `conductor-cli` edge before any read/write.
- **Interpolating operator-supplied values into argv or shell** — rejected because: `ANDROMEDA_PULSE_DATA_DIR` must be passed strictly via `.env(...)` builder; argument-injection metacharacters must be rejected before spawn (CVE-2026-30623).
- **String concatenation for SQL queries** — rejected because: must use rusqlite bound parameters for `run_id`/`seed`/fingerprint writes to `runs.db`, even though content is self-generated synthetic data.
- **Skipping garde validation on config deserialization** — rejected because: garde's `range` + `#[garde(custom)]` cross-field rules enforce trust boundary (error fraction ∈ [0,1], p50≤p95≤p99); unvalidated `serde` deserialize bypasses this.
- **Unbounded protobuf decoding from child stdout** — rejected because: must enforce bounded recursion on read-back path; empty canary round-trip must surface as `blocked` state, never false pass.
- **Deprecated crypto algorithms** — rejected because: if any fingerprint/hash is added to `runs.db` or journals, must use modern hash algorithms (MD5, SHA-1, DES, RC4, ECB mode forbidden).
- **Copying or decrypting Pulse's encrypted corpus.db** — rejected because: owned and encrypted by Pulse; Conductor only round-trips canary incident through MCP read-back.
- **Persisting secrets to unencrypted runs.db or journals** — rejected because: these store only self-generated synthetic telemetry; treat as world-readable local files.
- **Disabling TLS verification or downgrading to plaintext** — rejected because: today loopback-only, but any promotion beyond loopback must not silently skip transport verification.
- **Logging secrets, tokens, or Authorization headers** — rejected because: Conductor owns no secrets; this ban keeps it that way if future integration introduces them.
- **Exposing stack traces, absolute paths, or internal struct names to operator** — rejected because: sanitize at `anyhow` edge and in run-report artifacts.
- **Leaking absolute host paths in run-report artifacts** — rejected because: `<run_id>.md`, `runs.db` rows, JSONL journals must carry verdict/state/identity fields only; no canonicalized `CONDUCTOR_*` directories or internal seam-crate names.
- **Using virtual clock for journal/report timestamps** — rejected because: must use `std::time::SystemTime`/`Instant`; wrong clock source corrupts ground-truth artifact and breaks journal-relative SLO math.
- **Spawning MCP sidecar via shell with operator-supplied input** — rejected because: fixed program path + `.env(...)` only; no config-into-argv (rmcp STDIO design flaw).
- **Using Tauri shell-open plugin with scenario-derived strings unscoped** — rejected because: if GUI ever opens paths/URLs, must set explicit allow-scope; unscoped plugin enables RCE via dangerous protocols (CVE-2025-31477).
- **Embedding remote-origin iframes in bundled Tauri webview** — rejected because: iframes bypass origin checks for IPC even in isolation mode, combined with origin-confusion CVE (GHSA-57fm-592m-34r7, CVE-2026-42184).
- **Shipping Tauri commands without deny-by-default capabilities file** — rejected because: must author minimal capabilities allowing only actual commands (start/stop, picker, run-report, operator-pause) plus live-counter `Channel`.
- **Pinning rmcp client to strict newer protocol default** — rejected because: must negotiate DOWN to `2024-11-05` (Pulse's hand-rolled server version); strict-newer default is silent-mismatch class the preflight exists to prevent.
- **Following symlinks during crate extraction on pre-1.94.1 toolchain** — rejected because: must bump `rust-toolchain.toml` to ≥ 1.94.1 to clear tar-rs symlink-chmod flaw (CVE-2026-33056 / RUSTSEC-2026-0033).
- **Running release build or merging without cargo-audit green** — rejected because: dependency/supply-chain audit is the Minimal-tier residual-risk control for `bundled`-SQLite-from-C + OTLP/gRPC/MCP tree.
- **Allowing Cargo.lock to drift or go uncommitted** — rejected because: makes `cargo-audit`/`cargo-deny` scan non-deterministic; lets `bundled` SQLite C version float past advisory tracking.
- **Adding scenario without Pulse P-ID or introducing inbound listener** — rejected because: both violate architecture scope law / trust boundary and widen attack surface beyond loopback-client model.
- **Letting malformed child/transport input panic** — rejected because: `tonic::Status` codes and MCP error responses are first-class typed verification inputs; panic on read-back path corrupts run classification.
- **Silently downgrading failed preflight to pass/fail/manual-check** — rejected because: must surface as distinct `blocked` state with named precondition string, never false pass.
- **Adding unsafe code across OTLP/gRPC/SQLite FFI boundary without review** — rejected because: `bundled`-SQLite C and raw-protobuf/tonic codegen are the FFI surface the language cannot check; must keep `libsqlite3-sys` current.
- **Skipping required dependency bumps** — rejected because: `tauri` ≥ 2.10.3 (origin-confusion CVE-2026-42184) and toolchain ≥ 1.94.1 (tar-rs) are required over current pins.

### Data Classifications

- **config** (Sensitivity: Low) — obs handling: OK to log structured; appears in: `scenarios/` directory, deserialized into seam crates.
- **test-telemetry / run-metadata** (Sensitivity: Low) — obs handling: OK to log structured; appears in: `runs/` directory (`<run_id>.jsonl` journals + `<run_id>.md` reports) and `runs.db` index; content is self-generated synthetic OTLP fault data, not user-derived.
- **credential** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A for Conductor (OS keychain access is Pulse-side, exercised via spawned sidecar; Conductor owns no secrets).
- **PII** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A (no user accounts, no user-supplied content; only synthetic incident/fingerprint data Conductor emits).
- **payment** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A (no payment/billing SDK, no transactions).
- **health** (Sensitivity: Critical) — obs handling: never-log; appears in: N/A (no medical data).


## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (Windows/macOS/Linux Tauri 2 bundled webview) — Single-station console running React 19 + Tailwind CSS v4.1 + shadcn/ui; browser OTel SDK + web-vitals for frontend telemetry
- **cli** (Windows/macOS/Linux terminal, headless `conductor-cli`) — Line-oriented source-of-truth via clap + owo-colors + indicatif; stdout-only — no frontend telemetry, ANSI 256-color with TTY-gating for piped output

### Loading / Error / Empty State Patterns

- **Operator-pause go/no-go hold** — visibility: modal dialog (`AlertDialog` on desktop-webview; `inquire` prompt on CLI) — telemetry hook: span around hold-point with frozen-count transition duration + count-tint color change (green → amber over 150ms ease-out) as state-transition marker
- **Run-in-progress state** — visibility: inline in run-report view area (prose: "Run in progress"; titlebar count/matrix live via Tauri `Channel`) — telemetry hook: span from run start to report completion; no partial verdict shown during run
- **Run-report empty state** — visibility: inline prose ("No run yet — pick a scenario/suite to begin") — telemetry hook: counter for state-occurrence (zero-result runs)
- **Coverage-matrix empty state** — visibility: inline prose ("No scenarios loaded") — telemetry hook: counter for scenario-load failure

### User-Facing Error Surfaces

- **Run-report view: `Fail` verdict surface** (location: per-P-ID row in run-report + coverage matrix) — appears for: verdict = `Fail` (test failure, SLO exceeded); recovery affordance: retry scenario/suite — feedback widget candidate: yes (Sentry user-feedback widget on desktop-webview; CLI: sanitized error to stderr with hint-text for operator)
- **Operator-pause hold prompt** (location: modal dialog on desktop-webview; above `inquire` prompt on CLI) — appears for: pending go/no-go decision before committed timeline step; recovery affordance: Proceed (resume run) / Abort (stop run) — feedback widget candidate: no (user action expected, not an error state)
- **CLI error output** (location: stderr, sanitized) — appears for: clap parse failures, internal errors (`--debug`/`-v` only); recovery affordance: hint-text with fix — feedback widget candidate: no (machine-parseable, not user-facing UI)
- **`ManualCheck` checklist** (location: run-report card, per-P-ID row) — appears for: report-state = `ManualCheck` (operator-driven observation awaiting confirmation); recovery affordance: operator ticks y/n checkboxes to resolve items (no programmatic read-back) — feedback widget candidate: no (operator-initiated, not error)


## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** Run console (idle), Run console (live), Run console (HOLD), Run report (terminal)
- **cli:** `conductor run`, `conductor suite`, `conductor report`, `scripts/agent-run.sh` (headless)

### Error Boundary Placement

(No explicit error boundary placement in layouts — Phase 3 will recommend defaults per layout category, typically section-level for dashboards / page-root for forms.)


## 5. Test Plan Excerpt

### Tests Tier

- **Tier:** Minimal (0)
- **Justification:** The passed `security_tier` is Minimal (0) (single-developer, local-only, no-cloud, no-auth, zero network exposure, only self-generated synthetic data) and the product is effectively a single primary surface (the cli release gate) with no persistent USER data — the lone store (`runs.db` + JSONL) holds only synthetic telemetry classified `low`. These place it below Standard. However, this is Minimal at the upper boundary: 8 workspace crates plus several capability surfaces (>5 entities), 7 critical paths, and a Creator-Brief mandate for "production-grade verification rigor" with a determinism hard-bar and an assertion-policy split — so the Minimal baseline is augmented with the Section 5 coverage triggers (security-vector negative-tests, deterministic-replay property/golden tests, MCP contract test, cross-surface parity, bounded fault-injection) without escalating to Comprehensive (which the project disqualifies itself from: NOT a load-tester, no multi-target/distributed, local-only, mobile N/A).

### Test Harness Contract Summary

- **5-command names:** boot, run, status, cleanup, logs
- **Status JSON shape:**
```json
{
  "run_id": "string", "seed": 0, "scenario": "string", "p_ids": ["P-001"],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": 0, "slo_tier": "string",
  "journal_emitted_at": "ISO-8601", "read_back_observed_at": "ISO-8601",
  "fingerprints": []
}
```
- **Log format JSON schema:**
```jsonl
{
  "journal_emitted_at": "ISO-8601 from std::time::SystemTime",
  "run_id": "string",
  "seed": 0,
  "scenario": "string",
  "p_ids": ["P-001"],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": 0,
  "slo_tier": "string",
  "fingerprints": []
}
```
(One JSON object per line; no absolute host paths, no internal struct names; agent-parseable with jq / serde_json.)

### Critical Paths (must-trace)

- **Headless deterministic scenario run with MCP read-back verification:** after `conductor run error-baseline-spike --seed <s>`, exit code 0 AND `runs.db` row has `verdict=Pass` + `state=Pass` with `latency_ms` within `slo_tier` AND the per-run JSONL journal is written AND MCP read-back (`query_incident_list`/`retrieve_report`) confirms Pulse's reaction; same seed ⇒ same stream shape on re-run.
- **Fingerprint-storm scenario:** `conductor run fingerprint-storm` exits 0 with per-P-ID `verdict`/`state` rows recorded; fingerprints field populated in the envelope; read-back confirms Pulse fingerprint reaction within SLO.
- **Restart-suppression incl. one bypass case:** `conductor run restart-suppression` produces a hard pass/fail on suppression logic AND the bypass case reports its expected distinct outcome; journal + `runs.db` row assert the deterministic stream.
- **Severity-lifecycle full pass observing auto-resolve + resolution summary:** one full `severity-lifecycle` pass observes Pulse auto-resolve and a resolution summary via MCP read-back; lifecycle timing asserted as hard pass/fail, severity *choice* asserted as CalibrationRegion (report-for-human, not hard-failed).
- **Known-residual classification path:** `conductor run` for P-032 produces `state=KnownResidual` (NOT `Fail`); a `degraded_mode` read-back result maps to `KnownResidual`; run report distinguishes it from pass/fail/manual-check.
- **Coverage-matrix completeness gate:** the generated `coverage-matrix.md` enumerates all 60 P-IDs with zero unclassified entries; a missing P-XXX fails the gate ("A P-XXX missing from the matrix is a defect").
- **Both-surface parity:** a scenario launched via the Tauri start command yields the same `runs.db` envelope (verdict/state/seed) as the headless `conductor run` for the same scenario+seed; emission journal written per run from both paths.

### Coverage Triggers Summary

- **security-vector-coverage / negative-test (Vector 1)** — obs implication: "negative-test asserting `std::fs::canonicalize` + type/existence check rejects path-traversal `CONDUCTOR_*` handles BEFORE any `runs.db`/journal write"
- **security-vector-coverage / property-test (Vector 2)** — obs implication: "negative-tests asserting garde rejects out-of-range config (error fraction outside [0,1], negative durations, p50>p95>p99 ordering violations, severity-mix sum violations) at load"
- **security-vector-coverage / negative-test (Vector 4)** — obs implication: "negative-tests asserting (a) `ANDROMEDA_PULSE_DATA_DIR` with injection metacharacters is rejected; (b) sidecar program path is fixed/hard-coded; (c) empty/malformed canary round-trip becomes `blocked` (never false pass-as-empty); (d) bounded prost recursion does not panic on malformed child stdout"
- **security-vector-coverage / negative-test (Vector 3)** — obs implication: "negative-test/contract assertion that Tauri capabilities file is deny-by-default; no `shell-open` plugin and no remote-origin iframe; dependency-pin assertion (`tauri` ≥ 2.10.3)"
- **security-vector-coverage / negative-test (SQL injection)** — obs implication: "negative/static assertion that all `runs.db` access uses rusqlite bound parameters (no string-built SQL)"
- **contract-test** — obs implication: "preflight gate negotiates DOWN to `2024-11-05` (not strict-newer) and required-tool set matches pinned `contracts/` manifest; mismatch ⇒ `blocked`"
- **property-test / determinism discipline** — obs implication: "property/golden test asserting a fixed scenario+seed reproduces an identical emission-journal stream shape across runs"
- **cross-surface-coordination** — obs implication: "Tauri-launched run and headless `conductor run` produce identical envelope verdict/state for the same scenario+seed"
- **chaos-test / fault-injection** — obs implication: "fault-injection scenario tests asserting Pulse's reaction to silence/ramp/port-occupier faults within SLO — bounded 'typical/high' profiles only (P-060); explicitly NOT saturation"
- **supply-chain audit** — obs implication: "`cargo-audit` + `cargo-deny` green over bundled-SQLite-from-C + OTLP/gRPC/MCP tree; committed `Cargo.lock`; toolchain pinned ≥ 1.94.1"

### Quality Gates Summary

- **Zero-flakiness statement:** "flaky tests are NOT tolerated. If a test flakes once, it gets quarantined immediately and fixed (root cause — not retry-once budget). Do NOT set cargo-nextest `retries` > 0. Agent-driven dev cannot distinguish flake from real bug; retry policies mask actual failures. Determinism is enforced upstream (tokio `start_paused` virtual clock, seeded `conductor-timeline`, proptest `proptest-regressions/` persistence) so flakes indicate a real determinism break."
- **Coverage thresholds:** Minimal tier: ≥ 60% line, ≥ 50% branch, ≥ 70% function (enforced by cargo-llvm-cov 0.8.7 `--fail-under-lines 60`)


## 6. Creator Brief Excerpt

_Source: `.andromeda/input.md` (creator brief) + the run-invocation **obs scope corrective** supplied to `/andromeda-obs` (creator's verbatim obs steering for this run). Scope-corrective items are tagged `[scope-corrective]`._

### Must-Work Scenarios

These are the flows the creator names as critical — must-trace priorities for obs Phase 1 (in addition to tests' critical paths). All "observation" here is **Conductor observing Pulse**, never Conductor observing itself.

- **Emission journal as ground truth (the self-record):** "Wall-clock-stamped **emission journal** per run = ground truth of what was sent when (the left side of every SLO check)." `[scope-corrective]` reinforces: "The per-run `<run_id>.jsonl` emission journal ('the left side of every SLO check') is the ground-truth artifact, NOT 'observability telemetry.'"
- **MCP read-back as the observability surface:** "**MCP read-back client** over Pulse's tested tool surface — `query_incident_list` · `retrieve_report` (markdown + degraded_mode flag) · `retrieve_telemetry_slice` · `mark_incident_resolved`." `[scope-corrective]`: "The 'observability' that matters is MCP read-back of Pulse's reaction … the verification layer, not self-instrumentation."
- **Preflight canary before trusting read-back:** "preflight must canary-round-trip a known incident before any scenario trusts read-back."
- **End-to-end proof set (verbatim):** "at minimum `error-baseline-spike`, `fingerprint-storm`, `restart-suppression` (incl. one bypass case), `pii-scrub`, and `connection-lifecycle` produce verified expected outcomes (MCP read-back where applicable), and one full `severity-lifecycle` pass observes auto-resolve + resolution summary."
- **Determinism of the stream:** "Deterministic under a seed: same scenario + seed ⇒ same stream shape."
- **Both surfaces emit a journal:** "Every catalog scenario runs from the control panel AND headless; deterministic under a fixed seed; emission journal written per run."

### Rigor Hints

- **Tier intent (verbatim, run-invocation):** `[scope-corrective]` "Net (Minimal tier): keep it light. Self-observation = structured tracing logs (the emission journal + sanitized stderr) + at most a few in-process counters surfaced to the existing Tauri live-counter Channel."
- **Scale / deployment:** "The Pulse developer (solo, local). Runs next to a real Pulse instance on the dev host." Local, single-user, no cloud.
- **Determinism is a hard quality bar:** `[scope-corrective]` "same-seed ⇒ same stream is a hard quality bar"; an exporter that "spawns batch/background tasks on the runtime" would break "the `current_thread` determinism the arch deliberately chose."
- **SLOs are Pulse's, measured journal-relative:** `[scope-corrective]` "Every latency/SLO is `read_back_observed_at − journal_emitted_at` against Pulse's behavior. Conductor has no production SLO of its own (it is not a service, and not a load-tester — saturation is explicitly out of scope)." Pulse's per-tier SLO timing: "<5s / <20s / <90s, hardware-profile-aware."
- **Assertion-policy split:** "deterministic claims (hard signals, baseline math, suppression/bypass logic, lifecycle timing) = hard pass/fail; model-interpretive claims (severity choice, hypothesis quality, P-008 root-vs-deep weighting) = calibration-region checks + report-for-human, never hard-failed on exact values."
- **Not a load-tester:** "NOT a load-tester: bounded 'typical / high' load profiles only (P-060 SLO checks); 50k+ spans/sec saturation regimes are explicitly out."
- **Control surface, not a dashboard:** "Control panel (minimal UI) … A control surface, not a dashboard."

### Obs Anti-Patterns (creator's explicit asks)

The OTel/OTLP relationship is **inverted** — these are hard bans for this run.

- **The OTel/OTLP in the stack is the PRODUCT, not the obs mechanism:** `[scope-corrective]` "`opentelemetry-proto` is used to hand-build the fault telemetry Conductor emits AT Pulse (controlled errors / fingerprints / severities). That is Conductor's job — injecting synthetic OTLP into the system-under-test — NOT Conductor instrumenting itself."
- **NO OTel SDK / exporter for self-observation:** `[scope-corrective]` "Do NOT wire `opentelemetry` + `opentelemetry_sdk` + an OTLP exporter to observe Conductor itself." Why: (a) an exporter spawns batch/background tasks, breaking `current_thread` determinism; (b) self-telemetry would pollute the very OTLP stream Conductor injects at Pulse; (c) full OTel self-obs is massive over-scope for a local, single-user, Minimal-tier tool. "(This mirrors Pulse's own hard invariant: 'NO OTel SDK in the self-observation runtime.')"
- **NO metrics backend, NO trace exporter, NO collector, NO self-instrumentation spans:** `[scope-corrective]` "keep it light … NO metrics backend, NO trace exporter, NO collector, NO OTel SDK, NO self-instrumentation spans."
- **Emission-journal format is owned upstream — obs DERIVES, does not re-author:** `[scope-corrective]` "Its shape is already DEFINED by test-plan §3 (structured JSONL via `tracing-subscriber` json; wall-clock stamps from `std::time::SystemTime`/`Instant`, NEVER tokio's virtual clock — that corrupts journal-relative SLO math). obs DERIVES from that, it does not re-author it."
- **The ONE thing obs owns downstream — the redaction layer:** `[scope-corrective]` "the field-allowlist / redaction layer (no absolute host paths, no internal struct names in artifacts)."
- **No recursion — Conductor observes Pulse, not itself:** `[scope-corrective]` "Conductor does not observe itself observing Pulse."
- **PII scrubbing is about the synthetic corpus Conductor EMITS, not user data:** `[scope-corrective]` "PII scrubbing concerns the synthetic corpus Conductor emits, not user data (it owns none)." (The P-047 seven-category corpus — emails, JWT, bearer, API keys, credit cards, SSN, secret-like key=value — is emitted to verify Pulse scrubs it.)
- **No dashboards-only / human-review obs:** "A control surface, not a dashboard." The agent-readable surface is the journal + sanitized stderr, not a Grafana/Kibana UI.
- **No scenario DSL:** "NO scenario DSL — declarative config files + the built-in catalog; new behavior = new P-XXX first."



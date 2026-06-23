# arch extract

## Relevance
Relevant — this chunk opens Epoch 8 with the CLI surface (conductor run/suite/report verbs) + bootstrap, consuming already-built engine seams; the architecture domain covers workspace placement, inherited runtime/async/module-boundary decisions, standard-contract envelope shape, and cross-cutting determinism discipline.

## Constraints
1. Runtime flavor is locked to `tokio::main(flavor = "current_thread")` with zero work-stealing (arch §Established Decisions [Async Runtime Flavor]) — determinism invariant, non-negotiable on the CLI path.
2. The CLI binary sits in `crates/conductor-cli` within the crate-per-seam Cargo workspace; cross-seam library→library deps are forbidden by `Cargo.toml` edges, but the CLI bin may depend on all library seams (arch §Established Decisions [Module Boundaries]).
3. The run/suite verbs produce the canonical run-report envelope: `run_id` · `seed` · `scenario` · `p_ids` · `verdict` ∈ {Pass, Fail, CalibrationRegion} · `state` ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked} · timestamps (RFC-3339 text + `latency_ms` integer) · `fingerprints` array (per arch §Standard Contracts [Run report envelope]).
4. CLI will wire the already-built seams (`conductor-timeline` · `conductor-emit` · `conductor-verify` · `conductor-report`); no new engine logic or types introduced (arch §Established Decisions [OTLP Emission Strategy] / [MCP Read-Back Client] are locked; §Standard Contracts [Readiness gate] preflight assertion is consumed, not written).
5. Environment variables reserved under `CONDUCTOR_*` namespace: existing `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, `CONDUCTOR_SEED` are already in arch §Occupied Resources; CLI flags take precedence (arch §Cross-cutting Patterns [Config management]).
6. anyhow lives only at binary edges (`conductor-cli` + Tauri commands); seam crates use typed thiserror enums (arch §Conventions [Error handling]).

## Patterns to follow
1. **anyhow at edges:** arch §Conventions calls out `thiserror 2.0.18 typed enums in seam crates + anyhow 1.0.102 at binary edges`; the CLI `main.rs` catches seam errors into anyhow, displays sanitized (no host paths / struct names).
2. **Verdict/error wall:** outcomes (`Verdict`/`ReportState`) are returned as `Ok(...)` values; `Err` means harness failure (config parse, transport down, MCP unreachable); the report matches on types, not exception-catches (arch §Established Decisions [Outcomes are values]).
3. **Self-observation first:** init `conductor_core::init_observability("conductor", …)` before any scenario logic (arch §Stack / obs-plan §3 integration point).
4. **Seeded determinism:** the CLI consumes `--seed` (overrides `CONDUCTOR_SEED` env and the scenario's own seed) and passes it to the timeline engine; the engine's `seed_from_u64(scenario.seed)` pins `ChaCha8Rng` (arch §Established Decisions [Determinism RNG]).

## Anti-patterns to avoid
1. No multi-threaded tokio runtime flavor on the CLI path; `current_thread` is non-negotiable (arch §Design Philosophy [Determinism under a seed] + §Established Decisions).
2. Do not emit new endpoint/IPC method/port outside the locked surfaces: CLI is a gRPC/MCP client + artifact reader/writer only (arch §Cross-cutting Patterns [Trust boundary]).
3. Do not use `Result::Err` for verification verdicts; only harness faults (config, transport, MCP unavailable) map to `Err`; model behavior goes through the `Verdict`/`ReportState` envelope (arch §Conventions [Error handling] + §Established Decisions [Outcomes are values]).

## Contract bindings
Timeline engine ↔ CLI bootstrap (`current_thread` tokio runtime owned by CLI, passed to timeline `Runner`); Emit seam ↔ CLI egress liveness check (gRPC channel to `:4317` before any emission); Verify seam + Report seam ↔ CLI run/suite/report handlers (consume preflight gate verdict, serialize envelope); Config validation (garde) ↔ CLI scenario resolution (`Scenario::from_toml_str` called before timeline start).

## Acceptance criteria contributions
1. (arch) CLI binary runs on `#[tokio::main(flavor = "current_thread")]` with zero work-stealing — determinism invariant verified by absence of `tokio::spawn` / work-stealing configuration (arch §Established Decisions [Async Runtime Flavor]).
2. (arch) `conductor-cli` crate dependencies are fixed at wrap; new cross-seam deps to `conductor-timeline`, `conductor-emit`, `conductor-verify`, `conductor-report` conform to workspace boundary rules; no library→library seam violations (arch §Established Decisions [Module Boundaries]).
3. (arch) Run-report envelope produced by `run`/`suite` verbs conforms to §Standard Contracts shape: `run_id` (hyphen-delimited filesystem-safe stamp) · `verdict` ∈ {Pass, Fail, CalibrationRegion} · `state` ∈ {Pass, Fail, ManualCheck, KnownResidual, Blocked} · `latency_ms` INTEGER (null for Blocked rows) · `journal_emitted_at`/`read_back_observed_at` as TEXT RFC-3339 in serialized form.
4. (arch) Environment-variable edge: `CONDUCTOR_RUNS_DIR` / `CONDUCTOR_SCENARIOS_DIR` / `CONDUCTOR_SEED` are canonicalized with `std::fs::canonicalize` + bounds-checked before any read/write (arch §Occupied Resources + security §Input Validation).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** `CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV` registered in §Occupied Resources (Environment variables); `init_observability` reads both — arch env-var inventory extended by 2 handles (§Cross-cutting [Config management] updated correspondingly).
- **2026-06-21-run-report-envelope-serializer:** ManualCheck definition broadened to include auto-measured calibration-region checks; `Verdict::default_report_state` mapping recorded (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`); `verdict`/`state` kept independent; run-report lamp chosen verdict-first (arch §Probabilistic-Assertion Policy §Read-Back Dependency Posture updated — relevant to verdict routing logic in run/suite handlers).
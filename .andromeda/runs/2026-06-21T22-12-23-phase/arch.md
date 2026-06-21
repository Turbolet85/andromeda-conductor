# arch extract

## Relevance
relevant — connection-lifecycle scenarios (P-001..P-004) are core Epoch-7 catalog entries requiring workspace setup, config TOML authoring, and scenario-model extension for timing expectations and SLO tiers.

## Constraints
- Code lives in `conductor-core` (scenario model) and `scenarios/` (TOML config files) per §Inherited Defaults crate-per-seam workspace structure (arch §Inherited Defaults)
- Scenario config must deserialize via serde + garde validation per §Established Decisions [Scenario Config Format] using the toml 0.9 crate (arch §Stack row "Scenario config (TOML)")
- SLO tiers must be chosen from the closed set `<5s` / `<20s` / `<90s` per §Data model conventions (arch §Conventions)
- ExpectedOutcome model and timing-window assertions must type-match the §Standard Contracts run-report envelope shape (`verdict` / `latency_ms` / `slo_tier`) (arch §Standard Contracts)
- Determinism preserved via seeded `ChaCha8Rng` — same scenario+seed must produce identical phase stream shape (arch §Established Decisions [Determinism RNG])
- Port-occupier fault reuses the existing `conductor-faults` primitive; no new inbound listener beyond `:4317` (arch §Occupied Resources — ports)
- Verdict/error wall: outcomes are typed values (`Verdict` enum), never `Result::Err`; `Err` reserved for harness faults only (arch §Established Decisions [Error Handling] + §Cross-cutting Patterns [Verdict/error wall])

## Patterns to follow
- Declarative TOML scenario config keyed to Pulse P-IDs (precedent: `error-baseline-spike.toml` P-009/P-010 shape with `name · p_ids · seed · slo_tier · jitter_ms · [[phases]]`) (arch §Conventions [Config conventions])
- Timeline engine with deterministic `current_thread` tokio runtime + seeded RNG gap jitter (arch §Established Decisions [Async Runtime Flavor] + [Determinism RNG])
- Per-scenario run report row carrying `run_id · seed · scenario · p_ids · verdict · latency_ms · slo_tier · fingerprints` (arch §Standard Contracts [Run report envelope])
- Headless-drivable via `scripts/agent-run.sh`; verify scenarios with `start_paused` determinism fixture tests mirroring `error-baseline-spike` (arch §Established Decisions [Workspace / Core Structure])

## Anti-patterns to avoid
- No web framework, no HTTP service, no cloud config — OTLP/gRPC egress and MCP read-back only (arch §Established Decisions [Backend Framework] + [No web framework])
- No cross-seam dependency outside the workspace-enforced edges (e.g., `conductor-core` may depend on `conductor-faults` for the port-occupier, but not vice-versa) (arch §Established Decisions [Module Boundaries])
- No verdicts routed through exceptions; connection-state must remain orthogonal to error-verdict (P-004 guard) (arch §Established Decisions [Outcomes are values, errors are harness faults])

## Contract bindings
- **Workspace / crate naming** ↔ all domains (code lives in workspace-enforced crate structure)
- **Scenario TOML surface** ↔ tests (E2E fixture round-trip; nextest verification)
- **Standard Contracts envelope** ↔ obs (JSON self-obs logs of Verdict/ReportState classification) + security (validation harness pre-flight)
- **Port-occupier `:4317`** ↔ ops (the single deliberate inbound bind, exercising Pulse's receiver-failed reaction; released on cleanup)

## Acceptance criteria contributions
- (arch) Scenario TOML deserializes + garde-validates via `Scenario::from_toml_str` producing a valid `PhaseTimeline` (fixture test mirrors `error-baseline-spike` precedent).
- (arch) Connection-lifecycle scenarios for P-001..P-004 live in `scenarios/` with no new workspace crates beyond existing `conductor-core` + `conductor-faults` (arch §Occupied Resources [Crate names]).
- (arch) Same scenario+seed ⇒ identical phase stream shape via seeded `ChaCha8Rng` determinism; proven under `start_paused` (arch §Cross-cutting Patterns [Determinism discipline]).
- (arch) Port-occupier reuses existing `conductor-faults` fault; no new inbound listener (arch §Occupied Resources [Ports]).
- (arch) Verdicts typed as `enum Verdict { Pass, Fail, CalibrationRegion }` returned as `Ok(...)`, never `Err` (arch §Established Decisions [Outcomes are values, errors are harness faults]).

## Relevant amendment history
- **2026-06-15-config-validation-surface**: garde pinned 0.22.1 (was 0.23.0); cross-field invariants (e.g., p50≤p95≤p99 ordering) use garde's `Context` pattern, not container-level custom — applies to any scenario-model extension validating timing bounds (arch-amendment §[Validation Library])
- **2026-06-16-scenario-config-model**: toml 0.9 registered in §Stack + §Established Decisions; declarative TOML chosen over JSON for hand-author ergonomics across 60 per-P-ID scenario files — direct precedent for these 4 connection-lifecycle scenarios (arch-amendment §[Scenario Config Format])
- **2026-06-21-run-report-envelope-serializer**: ManualCheck widened to include model-interpretive (calibration-region) checks; default `Verdict → ReportState` mapping recorded (`Pass→Pass` / `Fail→Fail` / `CalibrationRegion→ManualCheck`) with verdict-first lamp precedence — affects how auto-measured timing windows (Receiving ≤1s, ReceiverFailed ≤2s) route to `Pass`/`Fail` vs. `ManualCheck` (arch-amendment §[Probabilistic-Assertion Policy])
- **2026-06-21-runs-db-index**: `journal_emitted_at` / `read_back_observed_at` columns stored as TEXT RFC-3339 (the JSONL wire form); `latency_ms` is the separate INTEGER millisecond value for SLO math — applies to how timing windows are stored and queried across runs (arch-amendment §[Timestamp formats])
# arch extract

## Relevance
relevant

## Constraints

1. Standard Contract envelope compliance — per §Standard Contracts §Run report envelope, each run record classifies into canonical verdict/state pairs (Pass/Fail/CalibrationRegion → Pass/Fail/ManualCheck/KnownResidual/Blocked) per §Probabilistic-Assertion Policy
2. MCP preflight readiness gate scope — per §Standard Contracts §Readiness gate, initialize negotiates protocol `2024-11-05` (Pulse's actual server version), asserts 4 required tools (query_incident_list, retrieve_report w/ degraded_mode, retrieve_telemetry_slice, mark_incident_resolved), and round-trips a data-dir canary proving corpus visibility; version/tool/canary failures surface as Blocked, never silent downgrade
3. Verdicts/report states are values, never exceptions — per §Design Philosophy §Outcomes are values, errors are harness faults, live transport/MCP/canary problems yield typed Verdict or ReportState or Result::Err (harness faults only); the verdict/error wall applies across all seams touched (conductor-verify, conductor-run, conductor-report)
4. Workspace crate placement — per §Established Decisions §Module Boundaries, code lands in designated seam crates (conductor-verify for MCP client/preflight, conductor-run for composition root, conductor-report for JSONL/runs.db/Markdown, conductor-cli for verb surface); forbidden cross-seam deps won't compile
5. Journal-relative SLO timing — per §Established Decisions §Timing-Tolerance Model, latency is `read_back_observed_at − journal_emitted_at` (journal RFC-3339 TEXT, derived latency_ms INTEGER) within tier-scaled tolerance; no wall-clock-from-test-start; same scenario+seed ⇒ same emission stream shape (determinism under seed)
6. Occupied resources: MCP wiring + data-dir propagation — per §Occupied Resources §Environment variables, respect ANDROMEDA_PULSE_MCP_ENABLED, propagate ANDROMEDA_PULSE_DATA_DIR to spawned andromeda-pulse-mcp sidecar via `.env(...)` (hardened pattern), respect CONDUCTOR_AGENT_MODE trigger + CONDUCTOR_PREFLIGHT_TIMEOUT (preflight gate gating), honor CONDUCTOR_RUNS_DIR artifact placement

## Patterns to follow

1. Blocked-row NULL discipline — per §Standard Contracts §Run report envelope, when ready=false, dependent scenarios emit state "blocked" with only identity fields populated (run_id, seed, scenario, p_ids, slo_tier); verdict, journal_emitted_at, read_back_observed_at, latency_ms, fingerprints are JSON null in report and SQL NULL in runs.db; cross-run latency queries exclude NULLs to prevent phantom values
2. Canary round-trip proof path — per §Standard Contracts, emit a known incident → poll `query_incident_list` (bounded wait) → assert content fidelity → flip ready to true; the canary also exercises the encrypted-corpus path (OS keychain for Pulse's corpus.db, same OS user as live Pulse)
3. MCP client preflight negotiation-down — per §Established Decisions §MCP Read-Back Client, rmcp client asserts negotiated_protocol_version == expected_protocol_version (both `2024-11-05`, pinned in manifest), not rmcp's default; version pin prevents silent mismatches
4. Deterministic failure classification — per §Design Philosophy / §Established Decisions §Verdict/error wall, scenario outcomes yield Verdict (Pass/Fail/CalibrationRegion), which default-maps to ReportState (Pass/Fail/ManualCheck); calibration-region claims never hard-fail (route to ManualCheck instead, operator-decision class)
5. Run artifact naming + tracing isolation — per §Occupied Resources §On-disk artifacts, per-run JSONL journal + Markdown report use hyphen-delimited run_id stem (YYYY-MM-DDTHH-MM-SS-<suffix>, Windows filename-safe); agent-mode self-obs streams to logs/agent-latest.jsonl (separate from per-run emission journal); Tauri runs stream to logs/conductor-tauri.jsonl; artifact paths move with CONDUCTOR_RUNS_DIR

## Anti-patterns to avoid

1. Silent MCP downgrades — per §Read-Back Dependency Posture, version mismatch / missing required tool / empty canary all yield explicit Blocked state; never tolerate a mismatch or report false pass on empty corpus
2. Wall-clock latency measurements — per §Established Decisions §Determinism, never consume time::SystemTime measured after scenario end; all SLO math sources journal_emitted_at (RFC-3339 from JSONL envelope) and read_back_observed_at (RFC-3339 from MCP retrieve timestamp), precomputed latency_ms (integer-ms, the only value the SLO math consumes)
3. Hard failures on model-interpretive calibration claims — per §Probabilistic-Assertion Policy §Two-state split, deterministic claims (baseline math, suppression logic, lifecycle timing) are hard pass/fail; model-interpretive severity/hypothesis claims route to CalibrationRegion → ManualCheck, never hard-fail on exact values vs. live Pulse

## Contract bindings

- **Standard Contract envelope + Verdict→ReportState mapping** ↔ security (validation of verdict/state enum values per design), tests (E2E golden-run fixtures reference canonical states), obs (error logging routes by ReportState + verdict; obs-plan §3 redacts fingerprints)
- **MCP preflight gate + canary round-trip** ↔ obs (readiness result serialized into run report header for observability; preflight timing spans under conductor setup root)
- **Two-build discipline (headless + GUI)** ↔ tests (build profiles for agent-mode vs. GUI; headless is source of truth), obs (dual ObsSink — agent-latest.jsonl vs. conductor-tauri.jsonl per entrypoint)
- **Verdict/error-wall cross-cutting** ↔ all seams (conductor-verify/conductor-run/conductor-report must return typed Verdict/ReportState; only Result::Err at harness boundaries)

## Acceptance criteria contributions

1. (arch) `conductor preflight` reaches `ready: true` against a live Pulse with passing canary round-trip (emit incident → retrieve via query_incident_list → assert content fidelity), flipping dependent scenarios from Blocked to pass/fail/manual-check — Blocked state extinction proof
2. (arch) Five-family E2E runs (error-baseline-spike / fingerprint-storm / restart-suppression / pii-scrub / connection-lifecycle) land non-Blocked verdict-first report states per scenario's claim class (deterministic → Pass/Fail; model-interpretive → ManualCheck); verdict/state pairs conform to §Probabilistic-Assertion Policy default mapping
3. (arch) Evidence artifacts (JSONL journal + runs.db rows + Markdown report) written per run with RFC-3339 timestamps, integer-ms latency_ms, and zero host-path or struct-name leakage (determinism/journal-relative timing invariants preserved)
4. (arch) Determinism and verdict/error-wall invariants intact — same scenario + seed reproducible; zero unlogged panics; transport/MCP/canary faults yield typed outcomes, never exceptions

## Relevant amendment history

- **2026-06-26-live-counter-channel-stream** — registered conductor-run as 9th workspace member (composition root: preflight + execute_scenario + persist + drive_run, shared by both bins); this chunk depends on conductor-run's orchestration layer
- **2026-06-24-sanitized-stderr-agent-mode-logging** — registered CONDUCTOR_AGENT_MODE env var + logs/agent-latest.jsonl artifact (self-obs JSON stream in agent mode, separate from emission journal, sibling of runs dir); this chunk runs in `--agent-mode` or env-triggered agent mode
- **2026-06-23-5-command-agent-run-harness** — registered CONDUCTOR_PREFLIGHT_TIMEOUT (preflight readiness-gate timeout, default 30s, read by agent-run boot wrapper); this chunk's preflight verb gates on this timeout
- **2026-06-21-run-report-envelope-serializer** — ManualCheck widened to include auto-measured calibration-region checks; recorded Verdict→ReportState default mapping (Pass→Pass / Fail→Fail / CalibrationRegion→ManualCheck via `Verdict::default_report_state`), verdict-first lamp precedence; this chunk consumes the default mapping for scenario outcome classification
- **2026-06-21-runs-db-index** — SQLite versions (libsqlite3-sys 0.36.0 bundling 3.50.4) + timestamp-format clarified (journal_emitted_at/read_back_observed_at stored TEXT RFC-3339; latency_ms separate INTEGER column); this chunk writes runs.db rows with the corrected schema
- **2026-06-18-exception-events-fingerprint-control** — fingerprint primitive (`fingerprint()` + exception-event builder) placed in conductor-emit per seam ownership; may apply to fingerprint-storm scenario within this chunk
- **2026-06-16-scenario-config-model** — toml 0.9 registered for declarative scenario config (serde + garde validation); this chunk refines scenarios/*.toml expected/SLO blocks per live Pulse behavior
- **2026-06-15-structured-logging-stack** — CONDUCTOR_SERVICE_NAME / CONDUCTOR_ENV registered for self-obs (init_observability reads both); agent mode setup depends on these
- **2026-06-15-config-validation-surface** — garde 0.22.1 pinned; scenario config validation (field-level custom + context pattern for cross-field invariants)

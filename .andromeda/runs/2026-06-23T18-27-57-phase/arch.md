# arch extract

## Relevance
Relevant — the chunk delivers the headless harness source of truth that architecture prescribes.

## Constraints
1. Harness shells out to `conductor-cli` (run/suite/report verbs) per design §Headless-drivable core, thin shells
2. Five-command discipline (boot/run/status/cleanup/logs) is the agent-parseable entrypoint per design §Headless-drivable core
3. MCP preflight gate (protocol `2024-11-05` · required tools · data-dir canary) is mandatory in `boot` per §Established Decisions [Read-Back Dependency Posture]
4. OTLP `:4317` egress liveness check in `boot` per §Occupied Resources (Ports) and §Established Decisions [Liveness equivalent]
5. Both `.sh` (bash/POSIX) and `.ps1` (PowerShell) must reach feature parity per §Infrastructure Patterns (Deployment model)
6. No scenario emission from harness; scenarios are `conductor-cli` domain per design §Headless-drivable core
7. Harness reads `runs.db` + `runs/<run_id>.jsonl` for status/logs per §Standard Contracts (Run report envelope) and §Occupied Resources (On-disk artifacts)

## Patterns to follow
1. **Readiness gate pattern** — MCP initialize + version assertion `2024-11-05` + required-tool manifest check + canary round-trip per §Standard Contracts (Readiness gate); surface `ready: true/false` + named precondition on mismatch, never silent downgrade to pass/fail
2. **Agent-parseable output discipline** — deterministic exit codes (0/non-0) + machine-readable ASCII status prefixes (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`), never color-alone per scope.md §Output discipline
3. **Idempotent cleanup pattern** — release `:4317` port-occupier if held, prune ephemeral artifacts safely-repeatable per scope.md §cleanup
4. **run_id threading** — every harness action (boot/run/status/logs) carries `run_id` for self-observation correlation per scope.md §Self-observation

## Anti-patterns to avoid
1. Re-implement engine logic in shell — harness is orchestration + artifact-read only per scope.md §Boundaries
2. Silent Blocked downgrade — if preflight fails, surface distinct Blocked state + precondition, never silently pass/fail per §Standard Contracts (Readiness gate)
3. New CLI flags/surfaces beyond the five-command set — scope law prevents feature creep per scope.md §Boundaries

## Contract bindings
- **CLI verbs** ↔ conductor-cli (run/suite/report per Epoch 8 §Established Decisions); harness invokes these as subprocesses and consumes exit-code + stdout contract
- **Readiness gate** ↔ verify seam (MCP preflight: protocol 2024-11-05, tool manifest, canary via rmcp 1.7.0 client per §Read-Back Dependency Posture)
- **OTLP egress** ↔ emit seam (`:4317` liveness check per §Occupied Resources)
- **Run artifacts** ↔ report seam (reads `runs.db` + JSONL journal; honors CONDUCTOR_RUNS_DIR env handle per §Occupied Resources)
- **Self-observation** ↔ obs plan (tracing 0.1.44 JSON logging per obs-plan §3; every action tagged with run_id per scope.md §Self-observation)

## Acceptance criteria contributions
- (arch) Five subcommands (boot/run/status/cleanup/logs) exist and reach feature parity between `scripts/agent-run.sh` (bash) and `scripts/agent-run.ps1` (PowerShell) per §Infrastructure Patterns (Deployment model)
- (arch) `boot` reports readiness (or Blocked + precondition) without emitting scenarios per §Standard Contracts (Readiness gate) and scope.md §boot
- (arch) `run` gates on `cargo nextest` then drives scenarios; exit 0 on success per scope.md §run
- (arch) Harness reads artifacts (runs.db + JSONL) without live Pulse for status/logs per §Standard Contracts (Run report envelope) and scope.md §status/logs
- (arch) cleanup is idempotent and repeat-safe per scope.md §cleanup

## Relevant amendment history
- 2026-06-23-conductor-run-suite-report-verbs — clap 4 registered in §Stack (the `run`/`suite`/`report` verb surface depends on clap 4 `derive` for argument parsing; routine spec→sound-impl alignment)

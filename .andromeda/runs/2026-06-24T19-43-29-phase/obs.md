# obs extract

## Relevance
Relevant — this chunk implements the final CLI surface machinery for agent-mode and sanitized stderr that the obs plan specifies as part of the harness contract and CI integration gates.

## Constraints
- Per obs-plan §3: `--agent-mode` flag must set `CONDUCTOR_AGENT_MODE=1` internally so env var is visible to subscriber/sink wiring and bootstrap scripts.
- Per obs-plan §3: dual-sink CLI behavior (stderr pretty-print in dev mode, file `logs/agent-latest.jsonl` in agent mode) — no JSON-to-stderr in agent mode (determinism + pipe safety).
- Per obs-plan §6: redaction remains at **processor (subscriber-layer)** stage, never sink-only, so upstream failures cannot leak host paths via stderr.
- Per obs-plan §3: log file location is `logs/agent-latest.jsonl` relative to `CONDUCTOR_RUNS_DIR`.
- Per security-plan §Error Handling: sanitized stderr at anyhow edge — no absolute host paths / internal struct names / stack traces; stack traces only under `--debug`/`-v`.
- Per obs-plan §2: agent-readable invariants — every signal must be machine-parseable, no human-only gating (scope.md confirms self-obs stream is **distinct** from emission journal).
- Per scope §Boundaries: zero core/timeline/emit/verify/report model change; `--agent-mode` overrides the `IsTerminal` gate from the operator-pause resolver (2026-06-23 prerequisite).

## Patterns to follow
- Span naming convention: `{module}.{operation}` per obs-plan §2/§4 (e.g., `scenario.run`, `emit.batch`); maintain bounded cardinality set (obs-amendment §3: no per-user/per-path names).
- Self-obs base-field set per obs-amendment §1: `timestamp_ms` (epoch millis from `std::time`), `level`, `target`, service-identity fields (`service.name`/`service.version`/`deployment.environment`), `run_id` on every line — envelope fields only on scenario-result events.
- anyhow error edge: convert panics to `anyhow::Error` + emit structured error log with context (obs-plan §2: zero-unlogged-panics gate); redaction at processor layer via field-allowlist + `Display`-not-`Debug` per obs-amendment §2.

## Anti-patterns to avoid
- Do not create a separate redaction policy or add redaction at sink-only stage (obs-plan §3/§6: must remain processor-layer).
- Do not include absolute file paths (host drive letters, `/home`, `/Users`, `%APPDATA%`, `~/.cargo`, backtrace paths) in stderr or JSON output; per obs-amendment §2 the allowlisted `target` module path is preserved, internal struct names kept out by field-allowlist + Display edge.
- Do not colorize stderr when piped or under `NO_COLOR` / `TERM=dumb` (design-system §cli / layout-templates §263).
- Do not conflate the self-obs stream (`logs/agent-latest.jsonl`, base-field set) with the emission journal (`runs/<run_id>.jsonl`, Run-report envelope) or reuse one schema for both (scope.md §Boundaries; obs-amendment §1).

## Contract bindings
- **tests §3 (harness contract):** `assert_cmd` captures cli stderr from `.get_output().stderr`; self-obs stream is asserted as a **distinct artifact** from the emission journal (test-plan-amendments §7); obs-plan §3 specifies the two record shapes and both are visible to test assertions.
- **design-system §cli "Error output" + layout-templates §263/§268:** error format (`error:` / `hint:`) + stdout/stderr separation + NO_COLOR/TERM=dumb + no colorize-when-piped requirement.
- **scripts/agent-run.{sh,ps1}:** the headless harness invokes cli with `--agent-mode` (or env); confirm boot/run/logs verbs surface the sink; `CONDUCTOR_AGENT_MODE=1` is the release-gate never-block proof.

## Acceptance criteria contributions
- (obs) `--agent-mode` flag present on cli; sets `CONDUCTOR_AGENT_MODE=1`; routes self-obs JSON to `logs/agent-latest.jsonl` (no pretty stderr); forces Headless resolver (never blocks on TTY) per scope §Acceptance intent.
- (obs) Harness-fault error renders as sanitized `error:`/`hint:` on stderr with zero host-path/struct-name/stack-trace leakage; `--debug`/`-v` surfaces trace only (obs-plan §Error Capture).
- (obs) Self-obs sink and emission journal remain distinct files with distinct schemas (base-field set vs Run-report envelope) per obs-amendment §1.
- (obs) Path scrubbing applied at processor (subscriber-layer) stage, never sink-only; no absolute paths / internal struct names in `logs/agent-latest.jsonl` (obs-plan §6 CI conformance).

## Relevant amendment history
- **2026-06-15-structured-logging-stack:** §3 schema clarified into two shapes — self-obs base-field set (`timestamp_ms`/`level`/`target`/service-identity/`run_id`) on every line, Run-report envelope (`journal_emitted_at` + scenario fields) only on scenario-result events; custom `tracing-subscriber` layer required (stock fmt().json() insufficient). This chunk wires the dual sink at the cli bootstrap seam using the base-field layer.
- **2026-06-15-log-error-boundary-redaction:** §6 reconciled redaction model — value scrub masks absolute host-FILE paths → `<redacted>` (NOT `::`-token redaction); internal struct names kept out by field-allowlist + `Display`-not-`Debug` at anyhow edge; allowlisted `target` preserved. This chunk applies the anyhow-edge sanitization to cli stderr at the error-output boundary and ensures processor-layer redaction blocks leaks before sink.
- **2026-06-16-emission-journal-writer:** Run-report envelope adds `read_back_observed_at`; schema now 11 fields. This chunk does NOT modify the journal writer — only the self-obs sink — so the envelope change stays scoped to that epoch; cli logs remain the base-field set.
- **2026-06-17-raw-otlp-message-scaffold:** "no OTel SDK" invariant clarified as behavioral (never init/use, not mere absence); transitive SDK crates are dormant. CLI self-obs is tracing JSON only; no SDK init in cli bootstrap.

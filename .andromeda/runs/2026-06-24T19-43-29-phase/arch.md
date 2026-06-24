# arch extract

## Relevance
Relevant — the chunk adds CLI surface discipline and env-var handling to an existing surface.

## Constraints
1. Code lives in `conductor-cli` crate (arch §Inherited Defaults: modular monolith, crate-per-seam); `--agent-mode` is a clap 4 flag added to the verb surface per §Stack (clap 4 derive).
2. Self-obs redaction stays at processor (subscriber-layer), never sink-only, so upstream failures cannot leak paths (obs-plan §606, noted in scope).
3. Error handling follows the verdict/error wall: `Result::Err` is harness-only; anyhow at binary edges; sanitization at anyhow boundary per security-plan §Error Handling §334 (no paths/struct names/traces to stderr).
4. New env var `CONDUCTOR_AGENT_MODE` must reserve the `CONDUCTOR_*` namespace per §Occupied Resources; `CONDUCTOR_RUNS_DIR` already governs the `logs/agent-latest.jsonl` path placement.
5. stdout remains data-only (artifact paths, results tables); all errors/messages route to stderr (design-system §352, scope §47).
6. `CliResolver::select()` logic (2026-06-23 amendment: isatty-gated) must be overridden by `--agent-mode` to force Headless behavior (never blocks, release-gate invariant).

## Patterns to follow
1. Dual-sink self-obs wiring (precedent: 2026-06-15 structured-logging-stack — tracing-subscriber JSON sink selected by `CONDUCTOR_AGENT_MODE`); scope maps this to obs-plan §3 lines 82/205/490.
2. Isatty-gated behavior override (precedent: 2026-06-23 isatty-gated-operator-pause amendment — `IsTerminal` gate; agent-mode forces Headless regardless).
3. Sanitized error format: `error: <short>` + contextual detail + `hint: <fix>` shape with zero host-path/internal-name leakage; only `--debug`/`-v` surfaces stack traces (precedent: security-plan §Error Handling §334; design-system §cli).

## Anti-patterns to avoid
1. Do NOT conflate `logs/agent-latest.jsonl` (self-obs tracing stream) with `runs/<run_id>.jsonl` (SLO ground-truth emission journal) — separate schemas, separate artifacts (scope §boundary 1; test-plan-amendments §7).
2. Do NOT add new redaction policy — apply the existing field-allowlist + anyhow-edge sanitization; redaction is at the processor layer (scope §boundary 4; obs-plan §606).
3. Do NOT colorize stderr when piped; honor `NO_COLOR`/`TERM=dumb` (design-system §352; scope §55).

## Contract bindings
- **obs ↔ logging:** self-obs JSON sink selection keyed by `CONDUCTOR_AGENT_MODE` env (obs-plan §3, §202); dual-sink wiring in tracing-subscriber; service-identity + `run_id` base-field set inherited from 2026-06-15.
- **security ↔ error edge:** anyhow-edge sanitization (no paths/names/traces to stderr; `--debug` only) per security-plan §Error Handling §334.
- **tests ↔ cli stderr:** `assert_cmd` harness captures stderr (test-plan §3); self-obs stream asserted as distinct artifact from emission journal (test-plan-amendments §7).
- **scripts/agent-run.{sh,ps1}:** consumes `--agent-mode` flag and/or `CONDUCTOR_AGENT_MODE=1` env on the release-gate path; boots without blocking (never-block proof, scope §59).

## Acceptance criteria contributions
1. (arch) `--agent-mode` present on CLI; sets `CONDUCTOR_AGENT_MODE=1` internally; reserved namespace `CONDUCTOR_*` honored per §Occupied Resources.
2. (arch) Self-obs sink selection by mode (stderr JSON dev / file JSON agent-mode) wired at subscriber initialization; redaction at processor layer (obs-plan §3, §606).
3. (arch) Error output sanitized at anyhow edge: zero host-path / struct-name / stack-trace leakage to stderr; `--debug`/`-v` only path to traces (security-plan §334).
4. (arch) `CliResolver::select()` override: `--agent-mode` forces Headless resolver regardless of `isatty()` (release-gate never-blocks invariant; precedent 2026-06-23 amendment).
5. (arch) stdout/stderr separation: data-only stdout (artifact paths, tables); all errors/human messages on stderr (design-system §352).

## Relevant amendment history
- **2026-06-23-conductor-run-suite-report-verbs:** clap 4 registered in §Stack as the CLI verb parser; this chunk adds a global flag to the clap derive surface.
- **2026-06-23-line-oriented-output-rendering:** owo-colors/indicatif/comfy-table registered for terminal rendering; this chunk uses owo-colors to strip ANSI when piped (design-system §352).
- **2026-06-23-isatty-gated-operator-pause:** inquire 0.9 + `IsTerminal` gate registered; this chunk overrides that gate via `--agent-mode` to force Headless (release-gate invariant, scope §21).
- **2026-06-15-structured-logging-stack:** tracing/tracing-subscriber + `CONDUCTOR_SERVICE_NAME`/`CONDUCTOR_ENV` env vars registered; this chunk adds `CONDUCTOR_AGENT_MODE` to the reserved namespace and wires the dual-sink selection by that flag (obs-plan §3).

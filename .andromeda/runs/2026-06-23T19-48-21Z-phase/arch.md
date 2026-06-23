# arch extract

## Relevance
Partial — touches workspace crate (conductor-cli) and inherited CLI/render patterns, but is primarily a presentation layer (out-of-core-arch scope).

## Constraints
1. CLI verb surface routed through `conductor-cli` per workspace boundary (arch §Inherited Defaults, §Workspace / Core Structure)
2. Render layer reads existing types only — no verdict/report model change (zero-model-change pattern per §Established Decisions)
3. Status lines MUST pair ASCII prefix with color, never color-alone (a11y contract; arch §Design Philosophy §Headless-drivable core)
4. CLI output suppresses color in non-tty / `NO_COLOR` via owo-colors auto-detection (arch §Stack and Technologies, Desktop frontend ruled out for this phase)
5. Coverage matrix renders all 60 P-IDs matching `coverage-matrix.md` definition-of-done (arch §Project Intent, §Cross-cutting Patterns)
6. Lamp rendering mirrors existing Markdown state→lamp mapping from `conductor-report` (verdict-first precedence per architecture-amendments.md §2026-06-21-run-report-envelope-serializer)

## Patterns to follow
1. Three new presentation-layer crates land behind a thin render seam in `conductor-cli` (owo-colors / indicatif / comfy-table) per modular-monolith crate-per-seam pattern (arch §Module Boundaries, §Established Decisions)
2. Structured types (`Verdict`, `ReportState`, `Lamp`, `RunRecord`, `ReadyState`) are read-only imports from `conductor-core` / `conductor-report` / `conductor-verify` (no duplication or re-derived classification logic)
3. New dev-stack deps (owo-colors / indicatif / comfy-table) enter `Cargo.lock` and MUST pass `cargo audit` + `cargo deny` per security baseline (arch §Stack and Technologies, §Infrastructure Patterns)

## Anti-patterns to avoid
1. Do NOT introduce color-only status encoding (violates a11y contract — ASCII prefix ALWAYS present)
2. Do NOT add new verdict/report state variants or emission/verification logic (render layer reads only, consistent with zero-model-change pattern)
3. Do NOT create a second state→lamp classification path (reuse Markdown renderer's existing verdict-first precedence, never invent a separate mapping)

## Contract bindings
obs ↔ report (lamp render mirrors canonical state mapping) · tests ↔ golden (cli output golden tests validate render fidelity) · a11y (color-never-alone rule)

## Acceptance criteria contributions
1. (arch) Status lines render with ASCII prefix `[PASS]` / `[FAIL]` / `[HOLD]` / `[BLOCKED]` ALWAYS present; color stripped (non-tty / `NO_COLOR`) remains unambiguous per conductor-cli render seam.
2. (arch) Coverage table renders all 60 P-IDs from `conductor-core` types with zero gaps (definition of done per arch §Project Intent).
3. (arch) `run` / `suite` / `report` / `preflight` output routes through existing `conductor-cli` verb surface; no new verb added in Epoch 8 (arch §Workflow / §Conventions).
4. (arch) New owo-colors / indicatif / comfy-table deps pass `cargo audit` + `cargo deny` clean; `Cargo.lock` committed un-drifted (arch §Infrastructure Patterns).

## Relevant amendment history
- (2026-06-23-conductor-run-suite-report-verbs) clap 4 (`derive`) registered in §Stack for the `run`/`suite`/`report` verb surface; this chunk wires their output rendering (clap verbs are the router, this chunk builds the presented output)
- (2026-06-23-5-command-agent-run-harness) CONDUCTOR_PREFLIGHT_TIMEOUT registered for `conductor preflight` verb gate; `preflight` render (readiness result as lines) lands in this chunk's output paths
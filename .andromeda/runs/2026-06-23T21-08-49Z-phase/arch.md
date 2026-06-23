# arch extract

## Relevance
Relevant — isatty-gated operator-pause adds a new CLI resolver to the core's deferred hold/resume mechanism; workspace crate placement (conductor-cli), resolved dependency (inquire), and terminal-rendering surfaces are all within the architecture domain.

## Constraints
1. **Workspace crate placement** — code lives in `conductor-cli` (the headless source-of-truth binary per arch §Inherited Defaults / §Established Decisions [Workspace / Core Structure]).
2. **No model change** — the interactive resolver implements the existing `conductor_core::PauseResolver` trait (read-only on `HoldPoint`, `Decision`, `HoldResolution`, `HeadlessResolver`); zero new verdict/report-state variants per arch §Established Decisions [Outcomes are values, errors are harness faults].
3. **Interactive/headless resolver selection gated on `IsTerminal`** — per arch §Inherited Defaults (conductor-cli's isatty gate reuses the same tty-detection primitive ch3's `stdout_color()` already uses, establishing tty-gate precedence in the CLI seam).
4. **New dependency (`inquire`)** — must be audit/deny-clean, committed in `Cargo.lock`, and integrated into workspace `[workspace.dependencies]` per arch §Infrastructure Patterns [Build system].
5. **Terminal-rendering layer** — reuses the established `indicatif` progress + `Lamp`→line mapping (ch3's render seam); the `[HOLD]` ASCII prefix is always rendered (status never color-alone, per arch §Cross-cutting Patterns / design-system.md).
6. **Determinism preserved** — same scenario + seed with auto-resolution (non-TTY path) ⇒ identical emission-stream shape; operator pause is wall-clock-only, outside the seeded virtual clock per arch §Design Philosophy [Determinism under a seed] + §Cross-cutting Patterns [Determinism discipline].
7. **Error handling discipline** — the resolver returns `Decision`; hold await may emit bounded `tracing` events (NOT from virtual clock per arch §Conventions [Error handling]), redacting raw prompts per obs-plan §redaction edge.

## Patterns to follow
1. **Crate-per-seam CLI seam** — the interactive resolver is a new module in `conductor-cli` (e.g., `pause.rs` or a resolver alongside `render.rs`), not a new crate; the isatty gate is wired at the run/suite execution point where `HoldPoint` is surfaced (per scope integration question; the hold await-site is an open P4 planning decision).
2. **Tty-gate precedent** — ch3's `stdout_color()` already establishes `IsTerminal::is_terminal()` on stdin/stdout as the canonical tty selector; reuse the same for the interactive/headless resolver dispatch.
3. **Lamp-first rendering** — reuse ch3's `Lamp`→line mapping with `[HOLD]` ASCII prefix + optional color overlay; verdict-first lamp precedence preserves the amber-hold visual hierarchy per arch §Probabilistic-Assertion Policy.
4. **Inquire confirm prompt** — the terminal UI uses the `inquire` crate's confirm dialog, mapping Go/NoGo to `Decision::{Go, NoGo}`; `allow_no_go == false` renders a proceed-only acknowledgment.

## Anti-patterns to avoid
1. **Do NOT introduce a second hold model or decision set** — implement the one `PauseResolver` trait the core defined; no custom hold/resume logic outside the core.
2. **Do NOT emit status via color alone** — `[HOLD]` ASCII prefix is always rendered (non-TTY and color-stripped paths must remain unambiguous).
3. **Do NOT let operator pause leak into seeded timing** — the resolver's prompt await is wall-clock-only; `elapsed_ms` remains a deterministic cumulative sum of seeded gaps (no virtual-clock reads in the hold path).

## Contract bindings
- **core ↔ CLI resolver** — conductor-cli implements the `PauseResolver` trait (`resolve_hold(resolver, hold) → HoldResolution`); the trait and types (`HoldPoint`, `Decision`) are read-only, defined in conductor-core per Epoch 5. **No cross-binding** to security/design/a11y/obs beyond the existing constraints (redaction, status encoding, determinism).
- **CLI render seam ↔ terminal output** — reuses `indicatif` + `owo-colors` + `comfy-table` (registered in arch §Stack §line-oriented-output-rendering amendment); `Lamp`→line + `[HOLD]` prefix logic pre-existing from ch3.

## Acceptance criteria contributions
1. **(arch) Code lives in `conductor-cli` per workspace boundary rules** (arch §Inherited Defaults / §Established Decisions [Module Boundaries]).
2. **(arch) Interactive resolver implements `conductor_core::PauseResolver` trait; zero new core types or variants** (arch §Established Decisions [Outcomes are values], scope §Boundaries "Not the core hold mechanism").
3. **(arch) Non-TTY path auto-resolves via `HeadlessResolver::proceed()` and never blocks** (headless never-block discipline per arch §Design Philosophy [Headless-drivable core, thin shells], scope §Surfaces [second bullet]).
4. **(arch) New `inquire` dependency audit/deny-clean, `Cargo.lock` committed, integrated into `[workspace.dependencies]`** (arch §Infrastructure Patterns [Build system], scope §Surfaces [New dependency]).
5. **(arch) `[HOLD]` ASCII prefix always rendered; status never color-alone** (arch §Cross-cutting Patterns, design-system.md a11y constraint).

## Relevant amendment history
- **2026-06-23-line-oriented-output-rendering** — terminal-rendering stack (owo-colors 4 + indicatif 0.17 + comfy-table 7) registered in §Stack; the `[HOLD]` prefix + `Lamp`→line mapping for this chunk reuses the ch3 render seam (`conductor-cli` presentation layer, tty-gated status-line color, run/suite progress spinner) now locked in architecture.
- **2026-06-23-5-command-agent-run-harness** — `CONDUCTOR_PREFLIGHT_TIMEOUT` registered (the `agent-run boot` entrypoint uses this; isatty gate's non-TTY detection should respect any existing `--agent-mode` signal if present, per scope §Boundaries ch5 reference).
- **2026-06-15-structured-logging-stack** — `CONDUCTOR_SERVICE_NAME` + `CONDUCTOR_ENV` registered (obs §3 reads these; the hold await may emit bounded tracing events per scope §Surfaces, redaction edge applies).

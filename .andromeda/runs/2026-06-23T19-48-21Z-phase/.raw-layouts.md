# layouts extract

## Relevance
partial — this chunk renders CLI line-oriented output for the coverage matrix and verdict lines; desktop-webview surfaces deferred to Epoch 9.

## Constraints
- per layout-templates §Surface: cli — Primary screens: render four CLI verbs (`run` / `suite` / `report` / `preflight`) with colored status headers, live `indicatif` progress, and per-P-ID verdict lines paired with ASCII prefixes `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` (layout-templates §cli Component — Header / banner, §Component — Primary content block 1 & 2)
- per layout-templates §cli Component — Primary content block 1: render coverage-matrix / SLO table as `comfy-table` with exactly **6 columns** (P-ID, scenario, state, slo_tier, latency_ms, fingerprints), dynamically detected terminal width, header dimmed, width never hardcoded (layout-templates §Primary screens — `conductor suite` output structure)
- per layout-templates §cli Component — Primary content block 2: render per-P-ID verdict lines with status glyphs (`✓`/`✗`/`⊙`/`•`/`?`/`~`) and status-color ANSI mappings (`count-nominal` → ANSI 114 / `count-hold` → ANSI 179 / `status-fail` → ANSI 203 / `count-blocked` → ANSI 60 / `color-id-cyan` → ANSI 117 / journal-text → ANSI 146) paired with bracket prefixes, never color-alone (layout-templates §Component — Primary content block 2)
- per layout-templates §cli IA notes — Pipe discipline: raw artifact data on stdout, human messages on stderr; ANSI auto-stripped when piped; `NO_COLOR` / `TERM=dumb` honored; emoji never used in machine-parseable output — ASCII prefixes only (layout-templates §Component — Footer / terminator + error output)
- per scope.md §What it builds: owo-colors status lines are ALWAYS tty-gated and color is paired with ASCII prefix; indicatif spinner shows live per-scenario progress (non-interactive); comfy-table renders all 60 P-IDs zero gaps with mode + lamp columns (scope.md §Boundaries out-of-scope: interactive operator-pause prompt is ch4, sanitized stderr is ch5)
- per scope.md §Acceptance intent: status lines render with ASCII prefix ALWAYS present; piped/non-tty output contains no raw escape sequences; new deps audit/deny clean; `Cargo.lock` committed un-drifted (scope.md §Boundaries out-of-scope)

## Patterns to follow
- **Status prefix invariant** — every verdict/state line opens with `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` + status glyph (`✓`/`✗`/`⊙`/`•`), color optional overlay over ASCII, never sole encoder (layout-templates §Component — Primary content block 2)
- **Terminal-width-detected tables** — `comfy-table` 6-column layout with dynamic width detection, never hardcoded px/char limits; responsive to terminal resize (layout-templates §cli Primary screens — `conductor suite`)
- **Multi-surface token mapping** — design tokens by name map to ANSI codes: desktop CSS vars ↔ cli ANSI (`count-nominal` ↔ ANSI 114, `count-hold` ↔ ANSI 179, `status-fail` ↔ ANSI 203, `count-blocked` ↔ ANSI 60, `color-id-cyan` ↔ ANSI 117), adapted not forked (layout-templates §Multi-surface coordination)
- **Headless non-interactive invariant** — output gating via `isatty` check; piped/agent paths strip color automatically (owo-colors `if_supports_color`), never block on prompt (scope.md §Boundaries out-of-scope: interactive prompt is ch4, but non-interactive output must be piped-clean) (layout-templates §IA notes — Headless invariant)

## Anti-patterns to avoid
- Never use color as the sole encoder of state — every status must be paired with an ASCII prefix / glyph / label (layout-templates §IA notes — Pipe discipline, §cli Component — Primary content block 2)
- Never hardcode terminal width or wrap verdict lines at arbitrary char limits; detect width dynamically and adapt table layout (layout-templates §cli Primary screens — coverage table)
- Never emit raw escape codes in piped / non-tty output; rely on owo-colors `if_supports_color` auto-detection to strip color when `!isatty(1)` (scope.md §Acceptance intent)

## Contract bindings
**a11y** ↔ layouts: "never color-alone" rule binds to a11y §No color dependency SC 1.4.1 (color is never the sole encoder of information; ASCII prefix + status glyph present on all verdict lines) (scope.md §a11y)
**design-system** ↔ layouts: palette color-to-ANSI mapping (design tokens by name ↔ ANSI 256 codes); per layout-templates §Multi-surface coordination (`count-nominal` ANSI 114, etc.) (scope.md §Design / layout)
**conductor-core/report** ↔ layouts: Verdict/ReportState/Lamp types + coverage-matrix shape read (no model change, only rendering); mirror existing Markdown-report row shape and lamp precedence (Pass > CalibrationRegion > Fail > Manual > Residual > Blocked) (scope.md §Reads, no model change)

## Acceptance criteria contributions
- (layouts) Status lines render with ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` always present; piped (non-tty) output contains no escape sequences (owo-colors auto-detection per layout-templates §cli IA notes)
- (layouts) Coverage table renders all 60 P-IDs (P-001..P-060) zero gaps, 6 columns, terminal-width-detected (comfy-table per layout-templates §cli Primary screens — `conductor suite` output structure)
- (layouts) Live progress feedback appears during `conductor run` / `conductor suite` (indicatif spinner per layout-templates §cli Component — Hero / signature output §indicatif heartbeat)
- (layouts) Design tokens map to ANSI codes per layout-templates §Multi-surface coordination mapping (count-nominal ANSI 114, count-hold ANSI 179, status-fail ANSI 203, count-blocked ANSI 60, color-id-cyan ANSI 117, journal-text ANSI 146)

## Relevant amendment history
**2026-06-23-5-command-agent-run-harness** — added `conductor preflight [--json]` verb (readiness gate / `agent-run boot` entrypoint) + `run --unit/--integration/--e2e` stage flags to layout-templates §cli Primary screens command list. This chunk implements the `preflight` verb's output rendering (colored readiness line with `[OK]`/`[BLOCKED]` prefix + protocol/tool-count detail) per layout-templates §cli Component — Header / banner pattern; the stage flags affect `run` verb filtering, not layout (scope.md §What it builds: preflight render lands here as a new dev-stack verb surface).
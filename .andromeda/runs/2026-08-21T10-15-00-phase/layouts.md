# layouts extract

## Relevance
Partial — the chunk is data-model/plumbing, but it names `conductor-report/src/report.rs:147` and `conductor-cli/src/render.rs:185` (latency display), which are the cells layout-templates owns: the cli results/SLO table + verdict lines and the webview coverage-matrix row / run-report card.

## Constraints
- The cli **results / SLO table is fixed at exactly 6 columns** (P-ID · scenario · state · slo_tier · latency_ms · fingerprints), terminal-width-detected; the `conductor coverage` matrix is a **separate 4-column** table and the two must not be conflated (per layout-templates §cli Component — Primary content block 1).
- Output structure is a **parsed contract**: "adding a `comfy-table` column without a `--format` flag would be a breaking change" (per layout-templates §cli IA notes — Command model). If per-check latency wants its own column, that is a breaking-change decision, not a render tweak.
- The **slo_tier cell renders exactly one of `<5s` / `<20s` / `<90s`** — the closed set — on both surfaces (per layout-templates §cli Component — Primary content block 2; §desktop Component — Primary content block 1). A sub-5s budget declared *beneath* a tier must not widen that cell's value set.
- **Non-measured rows render `—` / null in the measurement columns, never a red error** — `Blocked` ≠ red (never measured), `KnownResidual` never red, `ManualCheck` has no machine verdict (per layout-templates §cli Component — Primary content block 1; §desktop Component — Primary content block 2). Whether the code already emits `—` rather than `0`/null-as-zero for an unmeasured check is research's question.
- **`latency_ms` is a Data-role, right-aligned cell in `color-id-cyan` ↔ ANSI 117** on both surfaces (per layout-templates §desktop Component — Primary content block 1; §cli Component — Primary content block 1).
- **Row grain is one row per P-ID** — a dense single-row-per-P-ID list, explicitly NOT a KPI-card grid (per layout-templates §desktop Component — Primary content block 1). If per-check latencies acquire their own rows, that is a layout-grain change requiring an amendment, not a silent render change.
- **Three-surface parity, adapted not forked** — Markdown artifact · cli · webview render the same shape, same tokens by name (per layout-templates §cli Primary screens; §IA notes — Multi-surface coordination).

## Patterns to follow
- **Indented detail line under a row** is the existing carrier for a per-row extra fact — the `Blocked` precondition string, the `KnownResidual` "expected until {named fix}" note, the `ManualCheck` observe line (per layout-templates §cli Component — Primary content block 2; §desktop Component — Primary content block 2). If Open-fork 2 (latency attributable to a *named* check) needs surfacing, this is the precedent shape rather than a new column.
- **Run-level qualifier riding outside the lamp column** — `[ENVIRONMENT-SUSPECT]`, ANSI 246, ASCII bracket always printed, omitted entirely when not applicable, alters no per-check state (per layout-templates §cli Component — Primary content block 2). The precedent for surfacing a timing fact without minting a state.
- **Mode-cell treatment precedent** — a cell value gets a per-surface adapted recessive tint (`--status-residual` ↔ ANSI 246 ↔ Markdown emphasis) with the label *always* rendered so it survives `NO_COLOR` / piping (per layout-templates §desktop Component — Primary content block 1).
- **Derived numbers are never baked** — roll-up figures are manifest-derived, plain text, no new token/ANSI, omitted at zero (per layout-templates §cli Primary screens, `conductor coverage`).
- **Sample cells use the placeholder form** `<slo_tier>` rather than a pinned literal in wireframes (per layout-templates §Wireframe — Run console HOLD; §cli Component — Primary content block 2).

## Anti-patterns to avoid
- **No seventh lamp / sixth `ReportState`** for a budget breach or a slow check — the lamp set is closed at six (per layout-templates §cli Component — Primary content block 2).
- **Never render a missing latency as `0ms`, as red, or as an error**; and `CalibrationRegion` is `[HOLD]` amber, never silently a `Fail` (per layout-templates §cli Component — Primary content block 1).
- **Never re-pin a fresh literal** into a wireframe/sample tier or latency cell — de-literalize instead (per layout-templates §Decisions Log — Notes, and the four prior de-literalization amendments below).

## Contract bindings
- **layouts ↔ envelope contract** (`conductor-core/src/run_record.rs` + `architecture.md` §Standard Contracts): the table/verdict-line cells are the display half of the serialized shape; a per-check `latency_ms` shape change lands in both, and the redaction allowlist (`redact.rs:60-61`) must admit any new field name the render surfaces.
- **layouts ↔ frontend/webview**: the coverage-matrix latency cell is fed by one Tauri `Channel` / the deny-by-default command set — no polling, no URL surface (per layout-templates §IA notes — Live channel, Security guardrails).
- **layouts ↔ a11y**: layouts *states* "status never color-alone" and "verdict changes announced"; a11y derives the attributes (per layout-templates §Decisions Log — Notes).
- **layouts ↔ design**: token names only (`color-id-cyan` ↔ ANSI 117, `status-residual` ↔ ANSI 246); raw hex lives in design-system (per layout-templates §IA notes — Multi-surface coordination).

## Acceptance criteria contributions
- (layouts) The cli results/SLO table still renders exactly 6 columns after the change; any added column is recorded as a breaking output-contract change gated on a `--format` decision (per layout-templates §cli Component — Primary content block 1 + §cli IA notes — Command model).
- (layouts) A check with no measured latency renders `—` / null in the latency cell on cli, the webview matrix/report row, and the Markdown artifact — never `0ms`, never `status-fail`/ANSI 203 (per layout-templates §cli Component — Primary content block 1; §desktop Component — Primary content block 2).
- (layouts) The slo_tier cell still renders only `<5s` / `<20s` / `<90s`; a declared sub-5s budget does not appear as a fourth value in that cell (per layout-templates §desktop Component — Primary content block 1; §cli Component — Primary content block 2).
- (layouts) `latency_ms` keeps its Data role, `color-id-cyan` ↔ ANSI 117 mapping and right alignment on all three surfaces, adapted not forked (per layout-templates §desktop Component — Primary content block 1; §IA notes — Multi-surface coordination).

## Relevant amendment history
- **2026-08-18-error-baseline-spike-live-proof** — P-009 sample tier `<5s` → `<90s` across five sites (both desktop wireframes + two cli output structures + cli block 2). Why: the scenario re-declared its tier; sample cells disagreed with shipped values. Directly the cells this chunk's tier/latency work would touch.
- **2026-08-18-restart-suppression-live-proof** — the same five sites *de-literalized* to the `<slo_tier>` placeholder rather than re-pinned, and the prose now says the cell renders the scenario's TOML-declared value from the closed set. Why: the re-staling class; establishes the placeholder form any new budget cell should follow.
- **2026-08-19-connection-lifecycle-live-proof** — idle-console P-001/P-002/P-003 tier cells de-literalized on the same precedent.
- **2026-08-16-fingerprint-storm-live-proof** — coverage roll-up caption de-literalized after a baked count went stale three times; a **playbook rule for the derived-count class** was appended at that wrap. Why it matters here: any latency/budget figure added to a caption or sample must be placeholder/derived, never a literal.
- **2026-08-21-severity-lifecycle-live-proof** (immediately prior chunk) — swept every mislabeled sample-row P-ID against `conductor_core::coverage_matrix` (`restart-suppression` P-014 → P-015 at 5 sites, `port-occupier` P-022 → P-003 at 4 sites, etc.). Labels only; no wireframe, state, token or lamp changed. Relevant because those are the exact sample rows whose latency/tier cells this chunk may re-render.

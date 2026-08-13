# layouts extract

## Relevance
Partial — the goldens/replay work is outside layouts, but the load-envelope CARRY (a)/(b) changes *when* a documented run-level cli/report surface element (the `[ENVIRONMENT-SUSPECT]` caption) renders on all three surfaces.

## Constraints
- The load-envelope caption is a **run-level qualifier, not a check**: printed once per run, above the verdict lines it qualifies, riding OUTSIDE the lamp column, altering no per-check state, and **omitted entirely** when the run is in-envelope or pinned exempt (per layout-templates §Surface: cli — Output structure `conductor run`). Re-scoping to emitting-phase gaps + retiring the exemptions may change *which* runs caption — never the caption's placement or shape.
- The per-P-ID lamp/bracket set is **closed at six** plus that one run-level non-lamp qualifier; an envelope breach is never `[FAIL]` and never a sixth `ReportState` (per layout-templates §Surface: cli — Component: Primary content block 2, verdict / report-state lines).
- The ASCII bracket label is **always printed** so the signal survives `NO_COLOR` / piping / screen readers; the caption reuses ANSI 246 (Residual-mute) with **no new token/ANSI entry** (per layout-templates §Surface: cli — Output structure + §cli IA notes, Pipe discipline).
- cli output structure is a **parsed contract** — downstream agents read it, so changing a printed line's shape or adding a table column without a `--format` flag is a breaking change; the results/SLO table stays 6 columns and the coverage table 4 (per layout-templates §Surface: cli — IA notes, Command model + §cli Component: Primary content block 1).
- The same element renders **adapted, not forked** across cli stdout, the `<run_id>.md` artifact, and the webview run report (per layout-templates §Surface: cli — IA notes, Multi-surface coordination · §Surface: desktop-webview — IA notes, Multi-surface coordination).
- The headless agent path (`scripts/agent-run.sh` 5-command harness, `run --unit/--integration/--e2e`) is **never gated on an interactive prompt** and its artifacts stay ANSI-clean — the gate this chunk's goldens run under (per layout-templates §Surface: cli — Primary screens + IA notes, Headless invariant).

## Patterns to follow
- **The `not-Conductor's` Mode-cell precedent** the caption already follows: a state that *qualifies* rather than reports gets a recessive `status-residual` ↔ ANSI 246 treatment, its label always rendered, and never becomes a new lamp state (layout-templates §Component — Primary content block 1, Mode cell).
- **Single-source render point**: the caption is produced by one renderer and called once per command — `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs:91` (`envelope_caption`), invoked at `D:\dev\projects\conductor\crates\conductor-cli\src\commands\run.rs:27` and `D:\dev\projects\conductor\crates\conductor-cli\src\commands\suite.rs:38`; the webview consumes the same standing via `D:\dev\projects\conductor\crates\conductor-tauri\src\commands.rs` → `D:\dev\projects\conductor\crates\conductor-tauri\ui\src\components\RunReport.tsx`. Change classification behind that seam, not per surface.
- **Absent, not placeholder**: an in-envelope run prints *nothing* rather than a "no breach" line — matching the surface rule that non-result states are prose or omission, never a filler row (layout-templates §Surface: cli — Output structure · §desktop-webview Component: Primary content block 2, distinct non-result states).
- **Surface-printed numbers are engine-derived, never literals** (the manifest-derived roll-up discipline) — if the re-scope surfaces any count, it comes from the engine, not a hardcoded value (layout-templates §Surface: cli — Primary screens, `conductor coverage [--write]`).

## Anti-patterns to avoid
- Do NOT paint an envelope breach with `status-fail` / `count-blocked` (or `[FAIL]` / `[BLOCKED]`) — out-of-envelope is remit/qualification, not an outcome (layout-templates §Component — Primary content block 1, Mode-cell rationale · §cli Component: Primary content block 2).
- Do NOT add a seventh lamp, a sixth `ReportState`, or a new bracket label to express the emitting-phase re-scope (layout-templates §Surface: cli — Component: Primary content block 2).
- Do NOT widen either `comfy-table` shape (or conflate the 6-column results/SLO table with the 4-column coverage table) to expose per-phase gap data (layout-templates §Surface: cli — Component: Primary content block 1).

## Contract bindings
- **layouts ↔ tests/goldens:** caption presence/absence is asserted as *surface text* in `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs` (`envelope_caption_*` tests) and in `D:\dev\projects\conductor\crates\conductor-run\src\lib.rs` (report asserts `[ENVIRONMENT-SUSPECT]` present and `[FAIL]` absent) — retiring the two `[[exempt]]` entries must move those assertions with it, not silently flip printed output.
- **layouts ↔ design:** the caption's recessive tint is the existing `status-residual` ↔ ANSI 246 mapping; no new token is created by this chunk.
- **layouts ↔ a11y:** none this chunk — no new focusable elements, no dialog, no focus-order change.

## Acceptance criteria contributions
- (layouts) After the emitting-phase re-scope, the `[ENVIRONMENT-SUSPECT]` caption still prints at most once per run, above the verdict lines, outside the lamp column, and is omitted entirely for an in-envelope run (per layout-templates §Surface: cli — Output structure `conductor run`).
- (layouts) No new bracket label, lamp state, or `ReportState` is introduced: the per-P-ID set stays six plus the single run-level qualifier (per layout-templates §Surface: cli — Component: Primary content block 2).
- (layouts) `conductor run` / `conductor suite` stdout shape is unchanged — 6-column results/SLO table, ASCII prefixes intact, caption label legible under `NO_COLOR` and when piped (per layout-templates §Surface: cli — Component: Primary content block 1 · §cli IA notes, Pipe discipline).
- (layouts) Any standing change from retiring `activity-floor` / `incident-auto-resolution` renders identically on cli stdout, the `<run_id>.md` artifact, and the webview run report — adapted, never forked (per layout-templates §Surface: cli — IA notes, Multi-surface coordination).

## Relevant amendment history
- **2026-08-09-sut-load-envelope** (§Surface: cli — Output structure `conductor run` · Component: verdict / report-state lines) — directly this chunk's area: registered the run-level `[ENVIRONMENT-SUSPECT]` caption in the `conductor run` wireframe (ANSI 246, no new entry, ASCII label always printed, omitted when in-envelope or exempt) and clarified that the six verdict labels are the closed per-P-ID set while stdout additionally carries that non-lamp qualifier. Why it matters here: this chunk changes the *condition* that fires that caption, so the layout rules it fixed (placement, closed lamp set, omission) are the invariants to hold.
- **2026-08-09-out-of-scope-classification-treatment** (§Component — Primary content block 1) — established the recessive `status-residual` ↔ ANSI 246 ↔ Markdown-emphasis treatment for a state that is *remit, not outcome*, explicitly not `status-fail`/`count-blocked`; the envelope caption reuses that precedent, so no new treatment should be invented for the re-scope.
- **2026-08-09/-08-10 unbacked-qualifier count amendments** (11 → 10 → 9, §cli Primary screens) — precedent that surface-printed numbers are engine-derived and that a chunk moving one owes this doc an amendment (detector D-layout-derived-count). Applies only if the re-scope moves a printed count; the exempt-entry retirement itself prints none.

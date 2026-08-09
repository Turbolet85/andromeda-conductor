# layouts extract

## Relevance
Partial — core-crate classification work, but it lands on two rendered surfaces (desktop coverage matrix, cli coverage/SLO table) and requires a fix *inside* layout-templates.md itself.

## Constraints
1. Coverage-matrix row anatomy is fixed — status-lamp glyph · scenario label · P-ID · slo_tier · latency_ms, dense single-row-per-P-ID, virtual-scroll (per layout-templates.md §Component — Primary content block 1 (coverage matrix)). A fourth `CoverageMode` adds no row structure in this chunk.
2. The cli coverage/SLO table is **exactly 6 columns** with terminal-detected width, and its output structure is a parse contract — "adding a `comfy-table` column without a `--format` flag would be a breaking change" (per §Surface: cli — Component — Primary content block 1 + §cli IA notes → Command model).
3. The lamp taxonomy is exactly 6 states keyed to verdict/report-state, never to coverage mode (per §Component — Primary content block 2). The new mode gets no lamp, and no non-verdict state is ever rendered red / downgraded to `Fail` (same rule the plan extends from `Blocked` to `KnownResidual`).
4. Row-set and range labels must name the manifest's accepted set, never a hardcoded P-ID span (per §Wireframe — Run console (idle) header strip, "COVERAGE (manifest set) 82 loaded"; §Wireframe row note "one row per manifest capability").
5. Both surfaces adapt the same names, not fork — desktop CSS vars ↔ cli ANSI (per §IA notes — Multi-surface coordination). Anything mode-visible must land on both surfaces or neither.
6. Every cli state cell pairs its color with an ASCII bracket prefix so it survives `NO_COLOR` / piping / screen readers (per §Surface: cli signature block + §cli IA notes — Pipe discipline).
7. No responsive breakpoints exist on either surface (per §Decisions Log — Responsive breakpoints: N/A; cli is terminal-width-detected) — widening 60→82 is a virtual-scroll list-length change only, not a layout event.

## Patterns to follow
1. Header-strip count phrasing "(manifest set) N loaded · N measured" in Data role / `color-id-cyan` (per §Wireframe — Run console (idle)) — the already-corrected form; extend the same manifest-relative phrasing to the component prose.
2. `conductor coverage [--write]` already renders `coverage_matrix()` as P-ID / title / category / **mode** (per §Surface: cli — Primary screens) — the fourth variant flows through that existing cell; no new surface needed.
3. Footer roll-up tallies *report states*, each token paired with its label and tinted in its own status color (per §Component — Footer (status strip)) — coverage-mode counts are not part of that tally; keep them out this chunk.
4. Non-verdict states each get a distinct non-red glyph (hollow ring / neutral checkbox / muted dashed dot) rather than a color variant (per §Component — Primary content block 2) — the precedent the follow-on _Out-of-scope classification treatment_ chunk should inherit.

## Anti-patterns to avoid
1. No KPI-card grid and no per-mode summary card on the matrix — an explicit Rejected Default (per §Component — Primary content block 1).
2. Do not add a 7th cli table column, and do not reintroduce a hardcoded `P-001..P-060` span label anywhere (per §cli content block 1 + §cli IA notes — Command model).
3. No flash / blink / pulse / new motion for the new mode — expression budget 0.3, verdict changes resolve motionless (per §Decisions Log — Motion trigger placement).

## Contract bindings
- **UI mode union ↔ core serde wire spelling:** `D:\dev\projects\conductor\crates\conductor-tauri\ui\src\components\CoverageMatrix.tsx:6` holds `export type CoverageMode = 'auto' | 'drive+observe' | 'static-only'` mirroring `coverage.rs`, and drives both the Mode column and a per-mode count (`by(m)`). The chunk's fourth wire spelling binds **here**. `ui\src\lamp.ts` exhaustively unions `Lamp`/`Verdict`/`ReportState` only — it must NOT gain the new mode (§Component — Primary content block 2).
- **Doc-vs-plan reconciliation:** the scope's own doc list is incomplete — layout-templates.md carries the stale 60 at `:121` (named) **and** `:177` (`conductor coverage` = "render the static 60-P-ID coverage matrix", not named in scope).
- **a11y:** status-never-color-alone and change announcements are stated as behavioral requirements in my plan; the concrete ARIA/live-region attributes derive downstream (per §Notes).
- **design:** token/role names only — no new color token for the new mode is authored here (that treatment is the next working entry).
- **tests/harness:** cli stdout is parsed by the headless `agent-run.sh` path (§cli IA notes — Headless invariant / Pipe discipline); column count and bracket prefixes are the gate-facing contract.

## Acceptance criteria contributions
1. (layouts) §Component — Primary content block 1 no longer reads "the full 60-row wall (P-001..P-060)"; it names the manifest's accepted set, matching the header strip already corrected at the 2026-08-08 amendment (layout-templates §Component — Primary content block 1).
2. (layouts) The second stale hardcode at §Surface: cli — Primary screens (`conductor coverage` "static 60-P-ID coverage matrix", `:177`) is reconciled in the same pass; the `:222` "step 60/60 / 55 Pass" suite caption is illustrative output and may stay unless it is refreshed to the manifest set (layout-templates §Surface: cli — Primary screens).
3. (layouts) After the change the cli coverage table still renders exactly 6 columns, every state cell still carries its `[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]` prefix, and neither surface gains a new column, lamp glyph, or card (layout-templates §cli content block 1, §desktop content block 1).
4. (layouts) No surface renders a not-Conductor's row as `Blocked` / `Fail` / red, nor signals it by color alone (layout-templates §Component — Primary content block 2).

## Relevant amendment history
- **2026-08-08-sut-capability-manifest — De-hardcoded cli coverage range labels** (§desktop wireframes coverage header strip · §cli `conductor suite` header + Primary screens): replaced `P-001..P-060` range labels and the "all 60 rows" note with the manifest's accepted set, because "layout labels must not name a span wider or narrower than the rendered row set". This is the direct predecessor of this chunk's doc gap — that sweep updated the header strip (`:37`) but not the component prose (`:121`) or the `conductor coverage` verb line (`:177`), which is exactly the self-contradiction the scope now reconciles.
- **2026-06-23-line-oriented-output-rendering — `conductor coverage` verb registered** (§cli Primary screens): the verb line was authored as "render the static 60-P-ID coverage matrix (`coverage_matrix()` — P-ID / title / category / mode)" — the origin of the `:177` hardcode, and the reason the mode already has a documented rendering home on the cli surface.
- Precedent worth honoring: the **2026-06-24-frameless-window-shell** amendment established the spec-illustration → shipped-implementation reconcile rule (the doc tracks the shipped value) — applicable if the widened row set makes any wireframe illustration counts drift.

# layouts extract

## Relevance
Partial — the chunk authors scenario data + a core pin, but its third deliverable (the `(N unbacked)` qualifier reconciled across Markdown · cli · webview) and the `DriveObserve` → operator-checklist routing both land on layout-owned render surfaces.

## Constraints
- The `(N unbacked)` parenthetical **qualifies the auto term only and is never a fifth summand** — the four per-mode counts must still sum to the full row count in the roll-up caption (per layout-templates.md §Surface: cli — Primary screens, `conductor coverage [--write]`).
- The roll-up caption's numbers are **manifest/core-derived, never literals**, and the qualifier is **omitted entirely at zero** — so shrinking the pin 11 → 10 must flow from the source, not from an edited string (per layout-templates.md §Surface: cli — Primary screens, `conductor coverage [--write]`).
- **The same roll-up shape renders identically on all three surfaces** (Markdown artifact · cli · webview header strip); the webview receives the count as the coverage matrix's `unbacked` prop sourced from the `unbacked_auto` command, **never mirrored in TypeScript** (per layout-templates.md §Surface: cli — Primary screens; §desktop-webview IA notes "Multi-surface coordination").
- The qualifier is **plain text — no new token, no ANSI code, no colour** (per layout-templates.md §Surface: cli — Primary screens, `conductor coverage [--write]`).
- `conductor coverage` is the **4-column** table (P-ID · Title · Category · Mode) with no verdict/state column and therefore no bracket prefix; it must not be conflated with the 6-column results/SLO table (per layout-templates.md §Surface: cli — Component: Primary content block 1 (results / SLO table)).
- `ManualCheck` is a **distinct non-verdict report-state, never downgraded to Fail and never red** — neutral `status-manual` checkbox glyph on webview, `?` + `[MANUAL]` + ANSI 146 on cli, always paired with its text (per layout-templates.md §desktop-webview Component: Primary content block 2; §cli Component: Primary content block 2).
- The **Mode cell is text in exactly one of four values** and never color-only; the header-strip tally counts all four (per layout-templates.md §desktop-webview Component: Primary content block 1 — Mode).

## Patterns to follow
- **Coverage roll-up caption shape** — in-scope denominator beside the full row count, per-mode counts in parentheses with the `(N unbacked)` qualifier nested inside the auto term (layout-templates.md §cli Primary screens). Existing renderers: `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs`, `D:\dev\projects\conductor\crates\conductor-report\src\coverage.rs`, `D:\dev\projects\conductor\crates\conductor-tauri\ui\src\components\CoverageMatrix.tsx`.
- **`unbacked` flows as a prop from a backend command**, not recomputed frontend-side — `unbacked_auto` in `D:\dev\projects\conductor\crates\conductor-tauri\src\commands.rs` → `App.tsx` → `CoverageMatrix.tsx` (layout-templates.md §desktop-webview IA notes).
- **Operator-checklist render for `DriveObserve` P-IDs** — induced state on the left, expected observation as an operator-ticked yes/no on the right; Space toggles; the footer roll-up surfaces the unticked count (layout-templates.md §desktop-webview Component: Operator-checklist). Existing surface: `D:\dev\projects\conductor\crates\conductor-tauri\ui\src\components\OperatorChecklistView.tsx`; cli mirror is the `?` + `[MANUAL]` line with an indented `observe:` detail (layout-templates.md §cli Output structure — `conductor run`).
- **Dense single-row-per-P-ID coverage list, virtual-scrolled over the manifest's accepted set** — new P-IDs above P-060 add rows, never new card shapes (layout-templates.md §desktop-webview Component: Primary content block 1).
- **Status paired with an ASCII bracket prefix on cli** so every new scenario's state survives `NO_COLOR` / piping (layout-templates.md §cli surface preamble; §cli IA notes "Pipe discipline").

## Anti-patterns to avoid
- Do **not** hardcode the unbacked count (or any roll-up number) as a literal in any of the three renderers — the de-hardcode rule that already fired twice on this caption (layout-templates.md §cli Primary screens; amendments 2026-08-08 / 2026-08-09-current-sut-coverage-classification).
- Do **not** give the two `DriveObserve` scenarios a machine verdict treatment — no green/amber/red lamp, no `[PASS]`/`[FAIL]` prefix for a ManualCheck row (layout-templates.md §desktop-webview Component: Operator-checklist; §cli Component: Primary content block 2).
- Do **not** add a column, a lamp state, or a colour to carry the qualifier — the caption is plain text and adding a `comfy-table` column without a `--format` flag is a breaking change to downstream parsers (layout-templates.md §cli IA notes "Command model").

## Contract bindings
- **layouts ↔ obs** — the "derived qualifier, not a fifth summand" denominator semantics are obs-plan §4; the layout caption is the render of that contract, so the sum invariant is checked on both sides.
- **layouts ↔ architecture/core** — `conductor_core::UNBACKED_AUTO` (`D:\dev\projects\conductor\crates\conductor-core\src\drift.rs`) is the single source; all three caption renderers read it through `unbacked_auto` / the coverage builder rather than each holding a count.
- **layouts ↔ a11y** — the ManualCheck checkbox glyph and the Mode cell must remain text-paired (never colour-only), which a11y derives into the concrete attributes (layout-templates.md §Notes).

## Acceptance criteria contributions
- (layouts) The coverage roll-up caption reads `(10 unbacked)` on **all three** surfaces — Markdown artifact, cli `conductor coverage`, webview coverage-matrix header strip — with no surface forked or stale (layout-templates §cli Primary screens · §desktop-webview Component: Primary content block 1).
- (layouts) The four per-mode counts in the caption still sum to the full row count after the pin shrinks; `(N unbacked)` is nested in the auto term and is not summed (layout-templates §cli Primary screens).
- (layouts) The two `DriveObserve` P-IDs (P-067, P-072) render on the operator-checklist path — neutral `status-manual` checkbox glyph + `?`/`[MANUAL]` on cli, an induced-state/expected-observation row on webview — and never as a machine verdict (layout-templates §desktop-webview Component: Operator-checklist · §cli Component: Primary content block 2).
- (layouts) `conductor coverage` stays a 4-column table with no verdict/state column and no bracket prefix after the new rows land (layout-templates §cli Component: Primary content block 1).

## Relevant amendment history
- **2026-08-09-interpretation-correctness-posture** — the immediately-prior chunk added the unbacked qualifier to the roll-up caption (§cli Primary screens), fixing the literal at `43 auto (11 unbacked)` and recording the rules this chunk must honour: qualifies auto only, never a fifth summand, plain text, omitted at zero, reaches the webview via the `unbacked` prop from the `unbacked_auto` command and is never mirrored in TypeScript. This chunk is the first consumer of that shape — it changes the number, not the shape.
- **2026-08-09-out-of-scope-classification-treatment** — documented the `conductor coverage` **4-column** shape + the roll-up caption itself, and split the cli block into results/SLO (6-col) vs coverage (4-col). Relevant because this chunk touches the caption and must not re-conflate the two tables.
- **2026-08-09-current-sut-coverage-classification** — de-hardcoded the last literal-60 sites and established the rule that a caption's tally and denominator move together (a baked count is a stale-derived-fact regardless of whether it sits in prose or an illustrative sample). Directly governs this chunk's 11 → 10 edit.
- **2026-08-08-sut-capability-manifest** — de-hardcoded the `P-001..P-060` range labels to "the manifest's accepted set"; this chunk adds the first P-IDs above P-060, so that amendment is what keeps the labels correct without further layout edits.

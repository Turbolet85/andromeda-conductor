# layouts extract

## Relevance
Partial — the posture decision itself is out of my domain, but wherever it lands "load-bearing" it lands on the three run-report surfaces (webview report card · cli · Markdown artifact) and possibly the P-033 coverage-matrix row, all of which are templated.

## Constraints
- The caveat has no new surface: the frameless window IS the surface, four states not four routes, no router/deep-links; a green-run qualifier renders inside the existing **RUN REPORT** card region (per layout-templates §Surface: desktop-webview → Primary screens + §IA notes → Global model).
- If the posture attaches to a P-ID row, it uses the existing `KnownResidual` render — muted dashed-ring lamp at `radius-full`, always paired with its state text, plus a "residual: … until {named fix}" note in Body/`text-tertiary` — and is **never** downgraded to `Fail` (per §Component — Primary content block 2 → Distinct non-result states; §cli Component — Primary content block 2 `~ [RESIDUAL]`).
- Never `status-fail` and never `count-blocked` for a pre-accepted/deferred gap: `count-blocked` shares its hex with `text-muted`, so a plain dim reads as `Blocked` (per §Component — Primary content block 1 → Mode cell out-of-scope treatment).
- One fact, adapted per surface, never forked: webview CSS var ↔ cli ANSI ↔ Markdown emphasis (Markdown has no color channel), with the text label **always** rendered so it survives `NO_COLOR` / piping / monochrome (per §IA notes → Multi-surface coordination; §Surface: cli → signature/ASCII-prefix requirement).
- Status is never color-only and never becomes a seventh lamp state — classification/qualifier tints ride a cell-level modifier, the lamp column is untouched (per §Component — Primary content block 1 → Mode cell).
- Any new counted state must join the footer / caption roll-up with its own label + status-color token, and the per-state counts must still sum to the row denominator (per §Component — Footer (status strip); §Surface: cli → `conductor suite` summary caption).
- No new modal: the shadcn `AlertDialog` operator-pause is the only modal; `alert()`/`confirm()`/`prompt()` are never used — a caveat is prose in the report, not a dialog (per §IA notes → Global model; §Component — Hero / signature section).
- cli output structure is a parsed contract: adding a `comfy-table` column without a `--format` flag is a breaking change, and the 4-column `coverage` table must not be conflated with the 6-column results/SLO table (per §cli IA notes → Command model; §cli Component — Primary content block 1).

## Patterns to follow
- **P-032 `recent_commits` residual line** is the exact precedent for a named, pre-accepted gap that travels with an otherwise-green run: lamp + state text on the P-ID line, named-fix note on the continuation line, mirrored `~ [RESIDUAL]` in cli (§Component — Primary content block 2; §cli Component — Primary content block 2).
- **Roll-up caption shape rendered identically on all three surfaces** (Markdown artifact · cli · webview header strip), every number manifest-derived, never a literal (§Surface: cli → Primary screens, `conductor coverage`).
- **Recessive `status-residual` ↔ ANSI 246 ↔ Markdown emphasis** treatment already established for the out-of-scope Mode value — reuse it rather than invent a tint (§Component — Primary content block 1 → Mode cell).
- **Footer status strip as the home for incomplete/qualified state** — the unticked-`ManualCheck` count precedent shows how a "this run is not the whole story" signal is surfaced without adding chrome (§Component — Footer (status strip)).
- **Distinct non-result states never collapse into Fail** — the standing rule that governs Blocked / ManualCheck / KnownResidual applies equally to an interpretation-coverage caveat (§Component — Primary content block 2).

## Anti-patterns to avoid
- No KPI-card grid, banner, or new dashboard panel for the caveat — dense rows and the existing report card only; "a control surface, not a dashboard" (§Component — Primary content block 1 Rejected Default; §Component — Footer).
- No motion to draw attention — no flash/pulse/blink/glow; state resolves motionless over `motion-micro` (§Surface: desktop-webview → Expression level; §Decisions Log → Motion trigger placement).
- No hardcoded span/denominator labels in any wireframe or caption — labels name the manifest's accepted set, and a tally and its denominator move together (§Surface: cli → `conductor suite` caption; §Component — Primary content block 1).

## Contract bindings
- **layouts ↔ design:** token names only here (`status-residual`, `text-tertiary`, `count-nominal`); hex / ANSI values are design's (§Notes).
- **layouts ↔ a11y:** "status never color-alone", accessible names, and the announce-on-state-change requirement are stated here as behavior; a11y derives live-region attributes and conformance (§Notes; §Component — Primary content block 2).
- **layouts ↔ cli output contract / agent consumers:** ASCII bracket prefix pairing, stdout-data / stderr-messages split, column stability — a caveat printed to stdout must stay parseable (§cli IA notes → Pipe discipline, Command model).
- **layouts ↔ report artifact (`conductor-report` Markdown):** the `<run_id>.md` artifact carries the same roll-up/residual shape as cli and webview, adapted not forked (§Surface: cli → Primary screens, `conductor report`; §IA notes → Multi-surface coordination). Implementation touchpoints for the tri-surface mirror already exist at `D:\dev\projects\conductor\crates\conductor-core\src\report_state.rs`, `D:\dev\projects\conductor\crates\conductor-core\src\lamp.rs`, `D:\dev\projects\conductor\crates\conductor-cli\src\render.rs`, `D:\dev\projects\conductor\crates\conductor-report\src\report.rs`, `D:\dev\projects\conductor\crates\conductor-tauri\ui\src\lamp.ts`.

## Acceptance criteria contributions
- (layouts) The caveat renders inside the existing RUN REPORT card region of the run-report state and the cli results/terminator block — no new route, no new surface, no new modal (layout-templates §Surface: desktop-webview Primary screens + §IA notes).
- (layouts) If the posture attaches to a P-ID row (e.g. P-033), it renders as the `KnownResidual` muted dashed-ring lamp + always-paired state text + named-owner/named-fix note in `text-tertiary`, and never in `status-fail` or `count-blocked` (layout-templates §Component — Primary content block 2; §Component — Primary content block 1 Mode cell).
- (layouts) The same fact is adapted, not forked, across webview / cli / Markdown (CSS var ↔ ANSI 246 ↔ emphasis), with its label always printed so it survives `NO_COLOR`, piping, and monochrome (layout-templates §IA notes → Multi-surface coordination).
- (layouts) Any new counted state appears in the footer / caption roll-up with its own label and status-color token, and the per-state counts still sum to the stated denominator (layout-templates §Component — Footer (status strip); §cli suite summary caption).

## Relevant amendment history
- **2026-08-09-out-of-scope-classification-treatment** (§Component block 1 Mode cell · §cli Primary screens · §cli block 1) — added the recessive out-of-scope treatment (`status-residual` ↔ ANSI 246 ↔ Markdown emphasis, explicitly not `status-fail`/`count-blocked`, label always rendered, never a seventh lamp), documented the cli `coverage` **4-column** shape + its new roll-up caption, and split results/SLO (6-col) from coverage (4-col). Directly governs how any P-033 classification qualifier or new roll-up token must be rendered here — reuse the treatment, do not mint a new one.
- **2026-08-09-current-sut-coverage-classification** (§Component block 1 · §cli Primary screens · §cli suite caption) — introduced the Mode cell (four text values, never color-only, tallied by the header strip) and de-hardcoded the last literal-60 sites; established by explicit operator decision that a tally and its denominator must move together or the sample is left internally inconsistent. Relevant because this chunk touches the P-033 `Auto` classification's consistency with its (absent) verifying scenario.
- **2026-08-08-sut-capability-manifest** (§webview coverage header strip · §cli suite header) — layout labels must not name a span wider or narrower than the rendered, manifest-sourced row set.
- Standing cascade precedent from those entries: layout-templates has no specialist-summary doc; coverage-classification changes cascade to `.claude/docs/design-summary.md`.

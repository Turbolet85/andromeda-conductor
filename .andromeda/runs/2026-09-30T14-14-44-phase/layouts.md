# layouts extract

## Relevance
partial — the chunk creates or moves no surface, region or component; it changes the `sr*` leg's input path and regrades screen-reader focus rows (S0-09, E0-05, E0-09, R0-02..R0-04, E0-02..E0-06) whose EXPECTED sequence is the desktop-webview focus order and state prose this plan owns. Which layout element each row ID targets is research's question (the plan names only E0-01, S0-16, R0-01, S2-08 by row).

## Constraints
- The desktop-webview is ONE surface in four run states (`idle` · `live` · `hold` · `aborted`, idle-with-report a sub-state of `idle`), with no router, routes or breakpoints; a regraded focus row expects a state of that one window, never a page change (per layout-templates §Surface: desktop-webview → Primary screens; §IA notes).
- The coverage matrix region has exactly ONE tab stop — the current row (roving focus; ArrowUp/ArrowDown ±1 clamped, Home/End) — and its scroll container is not itself a tab stop; a Tab-driven focus sequence therefore crosses the matrix in ONE step, and an OS-level Tab count planned for a focus row must be derived from that, not from the row count (per layout-templates §Component — Primary content block 1).
- The console key map is MODIFIER-only (Ctrl+Enter start/proceed, Ctrl+. stop, Escape abort a hold), bound by one in-webview `window` keydown listener that stands down while a hold is raised; single letters stay free for the picker's type-ahead and the screen reader's browse mode, and OS shortcuts (Ctrl+W, Alt+F4) are never bound — an OS-level key path that reaches browse-class rows (the E0-09 class) relies on this single-letter freedom (per layout-templates §Component — Primary navigation).
- Focus is visible on every interactive control (`color-focus` ring) and the keyboard-current matrix row carries `aria-current="true"` plus a `border-emphasis` edge, so focus is never color-only; the focus position is communicated to assistive tech — the behavior the silent focus rows grade (per layout-templates §Component — Primary navigation; §Component — Primary content block 1 → States).
- The hold dialog traps focus, dismisses on Escape (Radix → No-Go), and restores focus EXPLICITLY via `restoreFocusTo` in `onCloseAutoFocus` because it has no Radix Trigger; the invoking Start control stays focusable (`aria-disabled`, never native `disabled`) (per layout-templates §Component — Hero / signature section).
- Three distinct App.tsx-owned catalog states carry three distinct shipped strings — empty-catalog `No scenarios found.` (E0-01), filter-miss `No scenarios match.` (S0-16), and the load-error render whose visible copy sits OUTSIDE the `role="alert"` region with a visually-hidden re-assertion inserted once on first `focusin` (R0-01); the run-report empty state is `No run yet` — a regraded `empty`/`error`-subject row expects the string its state owns, never a sibling's (per layout-templates §Component — Primary content block 1 → States; §Component — Primary content block 2).
- The footer status strip is DESIGNED, NOT SHIPPED — the release DOM's landmark set is `banner` + `main` + two `region`s, no `contentinfo` — so no regraded row may expect a footer landmark or its roll-up (per layout-templates §Component — Footer (status strip)).

## Patterns to follow
- The titlebar phase line renders one of four fixed state labels (`Conductor · idle` / `· live` / `· HOLD — operator pause` / `· aborted`), never the segment name — the stable, low-cardinality utterance to key a heard-text comparison on (per layout-templates §Component — Header).
- Record layout facts "as measured {date}" beside the designed intent where they differ (the shipped-vs-designed annotations throughout §Surface: desktop-webview) — a regrade that measures a divergence cites the row and keeps design intent and shipped state separate.
- The key-map hint line is Data role, not focusable, no live region — so it is expected to be absent from a Tab-driven focus sequence and from announced live text (per layout-templates §Wireframe — Run console (idle); §Component — Primary navigation).

## Anti-patterns to avoid
- Expecting a per-row tab stop across the coverage matrix, or a focusable scroll container — the region is one roving stop (per layout-templates §Component — Primary content block 1).
- Driving the leg with single-letter or OS-reserved keys (Ctrl+W, Alt+F4) as console commands — single letters belong to type-ahead and browse mode, and OS shortcuts close the window (per layout-templates §Component — Primary navigation).
- Grading a row against the unshipped footer or the unbuilt run-report checklist render as if they existed (per layout-templates §Component — Footer (status strip); §Component — Operator-checklist).

## Contract bindings
- layouts focus order ↔ a11y §Focus Order (SC 2.4.3): the regraded focus rows' expected order is this plan's region order (titlebar → control row → matrix single stop → report card); the SR record's expected utterances belong to a11y-plan §3 and the leg's harness.
- layouts hold-dialog pattern ↔ a11y modal focus trap (Escape dismiss + explicit focus restoration).
- layouts empty/error state strings ↔ the SR pass rows E0-01 / S0-16 / R0-01 (the plan cites them as measured); whether the regraded E0-02..E0-06 / R0-02..R0-04 rows target these same states is research's question.

## Acceptance criteria contributions
- (layouts) Every regraded focus-row expectation is taken from the shipped desktop-webview surface — no expected `contentinfo` landmark and no footer roll-up utterance (per layout-templates §Component — Footer (status strip)).
- (layouts) Any Tab-count or focus sequence the OS-input leg sends crosses the coverage matrix as exactly ONE tab stop (per layout-templates §Component — Primary content block 1).
- (layouts) The OS-level key path sends no single-letter console command and no OS-reserved shortcut (Ctrl+W / Alt+F4); console actions go through the modifier map only (per layout-templates §Component — Primary navigation).
- (layouts) An `empty`- or `error`-subject row that grades state prose expects the string its own state owns (`No scenarios found.` / `No scenarios match.` / the load-error re-assertion / `No run yet`), never a sibling state's (per layout-templates §Component — Primary content block 1 → States; §Component — Primary content block 2).

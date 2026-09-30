# layouts extract

## Relevance
partial — the chunk touches the desktop-webview surface's coverage matrix, run-report card, control row and HOLD dialog. Their keyboard and focus-visible behaviour is layout-owned intent. The WCAG/ARIA assertions themselves belong to a11y. The cli surface is out of scope.

## Constraints
- The surface is ONE frameless window in different run states (`idle` · `live` · `hold` · `aborted`; idle-with-report is a sub-state of `idle`). There are no routes and no breakpoints. Keyboard navigation moves through run state and in-window regions, never between pages. Per layout-templates §Primary screens and §IA notes.
- layout-templates §Component — Primary navigation requires keybindings to be first-class, exposed through the Radix primitives, and requires start / stop / proceed / abort to each be bound to a shortcut, with OS shortcuts (close-window / quit) still working. It also calls for a keybinding hint row presented for discoverability, "the way k9s pins its hint frame". Whether any shortcut handler or hint row exists in the code is research's question. The scope's take-up survey suggests none does.
- The same section requires the `color-focus` focus ring to be visible on EVERY interactive control. It also requires that keyboard selection moves the visible focus. This is the layout-side statement behind the SC 2.4.7 claim. Per layout-templates §Component — Primary navigation, restated in §Decisions Log → Notes.
- The coverage matrix is a dense single-row-per-P-ID list with FOUR cells in this order: P-ID · Capability · Mode · Status. `slo_tier`/`latency_ms` are not among the cells. §Primary content block 1 requires a selected row to carry a `border-emphasis` LEFT edge and a hover to carry a `color-raised-1` lift. Row selection must be a marking that exists, not a colour alone. Whether rows are focusable or selectable at all is research's question. Per layout-templates §Component — Primary content block 1.
- §Primary content block 1 states "Virtual-scroll for the full wall". The idle wireframe annotates "(virtual-scroll, one row per manifest capability)". The spec asserts this as TARGET state. Whether the shipped matrix is virtualized is research's question; the scope reports it as absent. If it is absent, a claim about "off-screen virtual-scrolled rows" has no layout referent. Per layout-templates §Component — Primary content block 1 and §Wireframe — Run console (idle).
- The run-report card is its own region beneath the matrix. It holds per-P-ID verdict lines (lamp + verdict text + mono identifiers), with the load-envelope banner ABOVE them and outside the lamp column. The spec assigns no selected-row marking to report lines. Only the matrix section defines one. Per layout-templates §Component — Primary content block 2 and §Wireframe — Idle with a report.
- In HOLD, the console controls are inert behind the focus-trapped go/no-go dialog. Proceed and Abort live only inside that dialog (Abort, then Proceed, in the actions row), and the dialog carries a visible `color-focus` ring. So the proceed/abort shortcuts have a layout home only while a hold is live. Per layout-templates §Wireframe — Run console (HOLD) and §Component — Hero / signature section.

## Patterns to follow
- The picker's active item is the model: a `border-emphasis` edge so that focus/selection is "never color-only". Extend the same marking to a selected matrix row rather than inventing a new token. Per layout-templates §Component — Primary navigation and §Component — Primary content block 1 (States).
- Every interactive control carries the `color-focus` ring. A new focusable element added for row navigation joins that convention. Per layout-templates §Component — Primary navigation.
- The keyboard-first register is k9s / lazygit: one shortcut per run action, plus a pinned hint frame. Per layout-templates §Component — Primary navigation.
- Dialog focus restoration is contract-borne (`restoreFocusTo` + `onCloseAutoFocus`). The invoking control stays focusable because Start uses `aria-disabled`, never the native `disabled` attribute. A shortcut that fires Start/Stop must not break that contract. Per layout-templates §Component — Hero / signature section.
- Where the spec's anatomy diverges from what renders, the layout doc corrects itself to the shipped shape, with a measured date. The matrix row (six cells → four) and the footer ("DESIGNED, NOT SHIPPED") are the precedents. A re-stated §5 keyboard claim follows the same shipped-truth discipline. Per layout-templates §Component — Primary content block 1 and §Component — Footer (status strip).

## Anti-patterns to avoid
- Never convey selection or focus by tint alone. A selected row needs the `border-emphasis` edge, and every control needs the visible ring. Per layout-templates §Component — Primary navigation and §Component — Primary content block 1.
- Do not introduce routes, breakpoints or browser-style back/forward as a way to "navigate" rows or reports. The window is the whole surface. Per layout-templates §Component — Primary navigation and §IA notes.
- A shortcut must never land on a control that is inert behind the HOLD dialog. While held, the only live actions are the dialog's Proceed/Abort. Per layout-templates §Wireframe — Run console (HOLD).

## Contract bindings
- Focus order ↔ a11y §5 / SC 2.4.3. The layout requires keyboard selection to move visible focus and that focus position to reach assistive tech. a11y derives the semantics (`aria-selected`/`aria-current`) and owns the assertions and the `claim-ownership.ts` rows. Per layout-templates §Component — Primary navigation.
- Focus ring ↔ design §Color Palette (`--color-focus`, `--border-emphasis`) ↔ a11y SC 2.4.7. The layout names the tokens, design owns their values, and a11y asserts that the ring renders. Per layout-templates §Decisions Log → Notes.
- Modal ↔ a11y modal focus trap. The HOLD dialog's trap, Escape and explicit focus restore bind the proceed/abort shortcut claim to the driven arm's live hold. Per layout-templates §Component — Hero / signature section.

## Acceptance criteria contributions
- (layouts) A keyboard-selected coverage-matrix row renders a `border-emphasis` left edge in the matrix region, beneath the control row. The marking is present, not tint-only. Per layout-templates §Component — Primary content block 1.
- (layouts) The `color-focus` ring is visible on the element that holds focus, for every interactive control the routine arm reaches: titlebar controls, control row, matrix scroll region and report scroll region. Per layout-templates §Component — Primary navigation.
- (layouts) Each of start / stop / proceed / abort is bound to a shortcut. Proceed and abort resolve only inside the HOLD dialog. Otherwise, the spec is re-stated to what ships, recording the shipped truth with its measured date. Per layout-templates §Component — Primary navigation and §Wireframe — Run console (HOLD).
- (layouts) Any matrix claim that depends on virtual scrolling is either backed by a virtualized list or re-stated. The spec's "Virtual-scroll" line must match what renders. Per layout-templates §Component — Primary content block 1.

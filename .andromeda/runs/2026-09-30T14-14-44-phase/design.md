# design extract

## Relevance
partial — the chunk is a harness/evidence chunk (input path + regrade of SR focus rows), not a rendering change; design touches it only through the shipped strings, focus model and status-label discipline the SR rows grade against, and through any UI fix a regrade might surface.

## Constraints
- The expected-announcement side of every regraded row is the SHIPPED prose the plan names, and the three empty/miss states are distinct strings that must not be conflated: `No scenarios found.` (empty catalog, SR row E0-01), `No scenarios match.` (picker filter miss, from a persistently-mounted announced region, SR row S0-16), `No coverage data.` (coverage matrix) and `No run yet` (run-report) (per design-system §Surface: desktop-webview → Component Patterns #3, #5, #6). Whether the code still renders those exact strings is research's question.
- The coverage matrix is ONE tab stop with a roving current row (ArrowUp/ArrowDown, Home/End, `aria-current`, off-screen rows scrolled into view). A focus row that expects per-row Tab stops inside the matrix would be grading against a model the plan does not specify (per design-system §Component Patterns #3).
- Keyboard input is a modifier map: Ctrl+Enter start/proceed, Ctrl+. stop, Escape abort. Single letters stay free for picker type-ahead and the screen reader's browse mode, and OS shortcuts (Ctrl+W / Alt+F4) belong to the OS (per design-system §Navigation Pattern; §Component Patterns #5). Any OS-level key sequence the leg sends must stay inside this map plus Tab and arrow navigation, and must not send a platform shortcut into the window.
- The focus ring is an untransitioned `2px solid var(--color-focus)` `outline` under `:focus-visible`, and no `box-shadow` ships (per design-system §Depth Strategy; §Motion "This project's values"). The input-path change must leave that focus mechanism as it is.
- Status is never color-alone. Every lamp pairs with a text label, and the CLI with its ASCII bracket prefix; `Blocked`, `ManualCheck` and `KnownResidual` never collapse into `Fail` (per design-system §Color Palette → Semantic Colors note; §Iconography "Rule"; §Surface: cli → Tokens). This binds any verdict text the regrade prints or records.
- If a regrade exposes a UI defect whose fix touches styling, the fix binds only `:root` tokens by `var(--…)` name (colors, `--space-*`, `--radius-*`, `--motion-micro` / `--ease-quiet`), never a raw hex, px or ms value (per design-system §Surface: desktop-webview → Tokens; §Self-Validation Protocol #4 Token Test).

## Patterns to follow
- Announced state flips go through a PERSISTENTLY mounted live region: a region that mounts together with its text announces nothing (per design-system §Component Patterns #5, measured at S0-16). The hold flip uses `aria-live="assertive"` (per §Component Patterns #1).
- Every icon-only titlebar or lamp control carries an `aria-label`, and shortcut-bearing controls declare `aria-keyshortcuts` (per design-system §Iconography; §Component Patterns #5). Those are the names a focus row expects to hear.
- The hint line is not focusable and is not a live region (per design-system §Component Patterns #5). A regraded focus row must not expect it to be announced on Tab.

## Anti-patterns to avoid
- Adding motion, a pulse or a transition to make focus "more announceable". Expression 0.3 bans pulse, blink and glow, and all transitions drop under `prefers-reduced-motion: reduce` (per design-system §Motion → Hard limits).
- Hover-only affordances, or a single-letter or OS-shortcut binding, introduced to make a row reachable (per design-system §Anti-Patterns → Per-Surface Bans, desktop-webview; §Navigation Pattern).

## Contract bindings
- design ↔ a11y: the SR rows' expected utterances (focus class and browse class) are the design plan's shipped strings, names and roles; a11y-plan §3 owns the grading and the input-path binding. Token contrast binds to a11y SC 1.4.3, the not-color-alone rule to SC 1.4.1, and the reduced-motion override to SC 2.3.3.
- design ↔ layouts: focus ORDER across the console is a layout-plan concern, not this plan's; design fixes only the matrix's single-tab-stop roving model and the modifier keymap.

## Acceptance criteria contributions
- (design) A focus row graded `announced-as-expected` names the shipped string or accessible name the design plan records for that surface, and the three empty/miss strings are never interchanged (per design-system §Component Patterns #3, #5, #6).
- (design) The OS-level key path the leg adopts sends only Tab, arrows, Home/End, Space, Escape and the declared Ctrl-modifier shortcuts, never a single letter or an OS window shortcut (per design-system §Navigation Pattern).
- (design) If a UI fix lands, `git diff ff4f571 -- crates/conductor-tauri/ui/src` adds no raw hex, px or ms literal outside the `:root` token block, and no `box-shadow` or `transition` on the focus ring (per design-system §Surface: desktop-webview → Tokens; §Depth Strategy).
- (design) Any verdict text the regrade records or prints carries its label or bracket prefix, never color alone (per design-system §Surface: cli → Tokens).

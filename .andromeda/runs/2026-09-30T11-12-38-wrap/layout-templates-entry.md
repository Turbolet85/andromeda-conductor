
## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — the shipped key map and hint line; the coverage matrix's roving stop
**Section:** §Wireframe — Run console (idle) · §Component — Primary navigation · §Component — Primary content block 1 (anatomy · States)
**Change:**
- Primary navigation: keybindings are a MODIFIER map (was "exposed through the Radix primitives … mirroring the k9s / lazygit register" with a k9s-style hint frame): Ctrl+Enter start (and proceed in the hold dialog), Ctrl+. stop, Escape abort; one in-webview `window` listener that stands down during a hold; `aria-keyshortcuts` on each control; single letters, Ctrl+W, Alt+F4 unbound; ONE shipped hint line `Ctrl+Enter start · proceed   Ctrl+. stop   Esc abort` (Data, `text-tertiary`, not focusable, no live region).
- Idle wireframe: the hint line is drawn under the control row; Start/Stop annotated with their `aria-keyshortcuts`; the matrix tail reads "every manifest row rendered; one roving tab stop" (was "virtual-scroll").
- Primary content block 1: every row renders (was "Virtual-scroll for the full wall") inside a scroll container that is not a tab stop; ONE tab stop — the current row (ArrowUp/ArrowDown ±1 clamped, Home/End, focus by any means makes it current); States: the keyboard-current row = `aria-current` + a 2px `border-emphasis` edge on its P-ID cell (transparent reserve on every cell), distinct from the `color-focus` outline ring (was "selected row").
**Why:** the chunk shipped the map and the roving focus (P4 fork, overseer: modifier map, one tab stop per region; the k9s register amended at wrap).
**Kept:** the picker's `border-emphasis` active-item edge; the dialog's `motion-micro` fade entrance; the 58/59-column width mismatch between the titlebar and the body rows (pre-existing).
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/

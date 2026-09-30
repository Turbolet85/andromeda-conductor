
## 2026-09-30-unasserted-keyboard-and-focus-visible-claims-closed — the modifier key map, the ring as an outline, every coverage row rendered
**Section:** §Depth Strategy · §Motion (micro-interactions · token table) · §Component Patterns #3 (coverage matrix) · #5 (picker + start/stop) · §Navigation Pattern
**Change:**
- Key map (was "first-class shortcuts mirroring k9s/lazygit" and "exposed via Radix primitives"): Ctrl+Enter start (and proceed in the hold dialog), Ctrl+. stop, Escape abort, from one in-webview keydown listener + the dialog's own handler; each declared via `aria-keyshortcuts`; a `text-tertiary` hint line beside Start/Stop; never single letters, never a platform shortcut.
- Focus ring (was a `0 0 0 2px var(--color-focus)` `box-shadow` inset with a `--motion-micro` fade-in): a `2px solid var(--color-focus)` `outline` under `:focus-visible`, NO transition; `motion-micro` covers hover + the count tint only.
- Coverage matrix: every row renders (was "Virtual-scroll for the full wall"), ONE roving tab stop (the current row, Arrow/Home/End); the keyboard-current row = `aria-current` + a 2px `--border-emphasis` edge on its P-ID cell (transparent reserve on every cell), distinct from the ring (was "selected row").
**Why:** P4 fork (overseer, founder-delegated): modifier map, single letters left to the picker's type-ahead and NVDA browse mode; the k9s register amended here. The ring reconciles the spec to what all seven `:focus-visible` rules always drew, now the asserted mechanism (SC 2.4.7 spec).
**Kept:** the k9s colour-world lineage in §Color Palette's rationale and the Accent row (a palette origin, not a key register); the dialog's 150 ms `--motion-micro` fade.
**Ref:** .andromeda/runs/2026-09-30T11-12-38-wrap/

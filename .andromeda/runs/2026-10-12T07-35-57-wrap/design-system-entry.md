
## 2026-10-12-panel-shaped-types-retired — the catalogue listing added; table widths follow the code
**Section:** §Surface: cli → Component Patterns 3 · §Spacing · §Platform-Specific Notes (cli) · §Per-Surface Bans (cli)
**Change:**
- Component Patterns 3 names a third, static table: `conductor scenarios` renders 3 columns (Scenario, P-IDs in ANSI 117 behind the stdout TTY gate, SLO). It has no verdict or state column, so no bracket prefix. It is the one table sized to the detected terminal width and ends in a plain roll-up line (`1 scenario` / `N scenarios`); an empty catalogue prints `No scenarios found.`.
- The results and coverage tables' columns are sized to their content: never wrapped, never hardcoded, no terminal-width detection (was "width detected dynamically from the terminal"). §Spacing, the cli platform note and the cli ban now size a table from its content, or from the terminal width where its surface says so (was "detect terminal width dynamically").
**Why:** the catalogue listing is this chunk's new verb. The width wording was a mismatch older than this chunk, found at this wrap: the results and coverage tables use comfy-table's `Disabled` arrangement. The masters were brought to the code on the operator's own word at this wrap.
**Kept:** the window-era hex palette rows ("frameless titlebar", "Picker dropdown", "active picker item", the go/no-go dialog) are not this chunk's; they are owned by `No-survivor sweep`.
**Ref:** .andromeda/runs/2026-10-12T07-35-57-wrap/

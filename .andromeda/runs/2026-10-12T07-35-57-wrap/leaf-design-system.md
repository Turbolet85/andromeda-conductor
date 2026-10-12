group: design-system
read: .claude/docs/design-summary.md · 34 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
read: .claude/rules/frontend.md · 27 · «- 2026-09-17: **In a webview spec, an expectation that inter»
read: CLAUDE.md · 137 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/design-summary.md:25   class: omission
  leaf-text: «The coverage table is 4 columns with a roll-up caption;»
  source: .andromeda/design-system.md:173 «Size `comfy-table` columns from their content (the results and coverage tables) or from the detected terminal width (the `conductor scenarios` listing alone)»
  replace-with: «The coverage table is 4 columns with a roll-up caption, sized to its content like the results table (only the `conductor scenarios` catalogue, a third 3-column table, sizes to the detected terminal width; never hardcode widths or wrap);»

- leaf: CLAUDE.md:42   class: omission
  leaf-text: «pair every Verdict/ReportState with its ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]` (cli)»
  source: .andromeda/design-system.md:150 «the closed per-P-ID set `[PASS]` / `[HOLD]` / `[FAIL]` / `[MANUAL]` / `[RESIDUAL]` / `[BLOCKED]`»
  replace-with: «pair every Verdict/ReportState with its ASCII prefix `[PASS]`/`[FAIL]`/`[HOLD]`/`[MANUAL]`/`[RESIDUAL]`/`[BLOCKED]` (cli)»

- leaf: CLAUDE.md:59   class: retired
  leaf-text: «| Design tokens / palette / typography | `.andromeda/design-system.md` §Color Palette / §Typography |»
  source: .andromeda/design-system.md:79 «Retired 2026-10-11 with the window (its faces, type scale and font loading are last at `321dc8f`). The cli sets no face»
  replace-with: «| Design tokens / palette (the cli's ANSI map) | `.andromeda/design-system.md` §Color Palette / §Surface: cli |»

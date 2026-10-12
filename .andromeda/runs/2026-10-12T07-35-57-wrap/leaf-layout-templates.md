group: layout-templates
read: .claude/docs/design-summary.md · 34 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
read: .claude/rules/frontend.md · 27 · «- 2026-09-17: **In a webview spec, an expectation that inter»
read: CLAUDE.md · 137 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/design-summary.md:25   class: omission
  leaf-text: «The coverage table is 4 columns with a roll-up caption; its `(N unbacked)` qualifies the auto term, never a fifth summand, omitted at zero.»
  source: .andromeda/layout-templates.md:120 «**The scenario catalogue is a third shape** — `conductor scenarios` renders **3 columns** (Scenario · P-IDs · SLO), no state column and no bracket prefix, the one table sized to the terminal width»
  replace-with: «Three table shapes, never conflated: the `run`/`suite` results table is 6 columns sized to their content (no terminal-width detection); `conductor coverage` is 4 columns with a roll-up caption, its `(N unbacked)` qualifying the auto term, never a fifth summand, omitted at zero; `conductor scenarios` is 3 columns (Scenario · P-IDs · SLO), no bracket prefix, the one table sized to the terminal width, ending in its `1 scenario` / `N scenarios` roll-up line.»

- leaf: .claude/docs/design-summary.md:12   class: omission
  leaf-text: «## Key tokens (the cli's ANSI map binds the palette by name)»
  source: .andromeda/layout-templates.md:150 «the ANSI codes bind the design-system palette by token name — `count-nominal` → ANSI 114, `count-hold` → ANSI 179, `status-fail` → ANSI 203, `count-blocked` → ANSI 60, `color-id-cyan` → ANSI 117, journal-text → ANSI 146.»
  replace-with: «## Key tokens (the cli's ANSI map binds the palette by name; journal-text → ANSI 146 carries `[MANUAL]` and dimmed metadata)»

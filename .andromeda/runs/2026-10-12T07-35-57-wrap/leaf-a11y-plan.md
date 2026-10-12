group: a11y-plan
read: .claude/docs/a11y-summary.md · 24 · «**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rule `»
read: .claude/rules/a11y.md · 22 · «- 2026-09-30: **An NVDA started while another window holds t»
read: CLAUDE.md · 137 · «<!-- USER:session-learnings end -->»

- leaf: .claude/docs/a11y-summary.md:6   class: retired
  leaf-text: «**Tier:** Minimal (0). **No assertable surface remains**»
  source: .andromeda/a11y-plan.md:26 «Retired 2026-10-11 with the window (intent R1): no assertable surface remains. `conductor-cli`, the one surface, is not-assertable — no DOM or widget tree;»
  replace-with: «**Tier:** none stated — §1, which held it (last `Minimal (0)`), retired with the window; its full text is at `321dc8f`. **No assertable surface remains**»

- leaf: .claude/rules/a11y.md:13   class: tree
  leaf-text: «Conductor has no interface, so no assertable a11y surface remains.»
  source: .andromeda/a11y-plan.md:12 «**Conductor has no interface.** Its window was retired 2026-10-11 (`2026-10-11-conductors-window-retired`, intent R1), and with it the one assertable a11y surface;»
  replace-with: «Conductor has no interface — the `conductor-tauri` crate and its `ui/` webview left the tree, so no file these paths name exists — and no assertable a11y surface remains.»

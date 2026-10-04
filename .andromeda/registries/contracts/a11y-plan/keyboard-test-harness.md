### Keyboard test harness

- **Sequences per ARIA pattern** (from design excerpt's keyboard contracts):
  - Button (titlebar / start-stop / Proceed-Abort): Enter / Space invokes default action
  - Dialog (`alertdialog` operator-pause): Tab/Shift+Tab cycle within trap across in-trap checklist rows then actions; Space toggles a focused checklist row; Escape closes per convention; Enter/Space activates Proceed/Abort
  - Combobox (scenario/suite picker): Arrow Down/Up navigates options; Enter selects; Escape closes; type-ahead filtering
  - Checkbox (operator-checklist row): Space toggles a focused row
  - Coverage matrix (table rows): roving row focus through one tab stop — ArrowUp/ArrowDown ±1 (clamped), Home/End; off-screen rows reached by focus scroll-into-view (every row renders, no virtualization)
  - First-class shortcuts: Ctrl+Enter start · proceed, Ctrl+. stop, Escape abort — declared via `aria-keyshortcuts` + a visible hint line (layout excerpt's Focus Management Anchors)
- **Tooling:** **WebdriverIO `browser.keys(['Tab' | 'Shift+Tab' | 'ArrowDown' | 'Enter' | 'Space' | 'Escape'])`** over the `@crabnebula/tauri-driver` session (the same one running `@axe-core/webdriverio`); results emit as structured PASS/FAIL JSON folded into the obs Section 6 envelope. CLI keyboard verification is manual-only and out of harness scope (not-assertable surface).

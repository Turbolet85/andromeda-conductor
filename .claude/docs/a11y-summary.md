# Accessibility Summary — Conductor

_Distilled from `.andromeda/a11y-plan.md`. setup-project Phase 3. wrap-session does not modify._

## A11y tier
**Tier:** Minimal (0).
**WCAG target:** WCAG 2.1 AA baseline — SC 2.1.1 Keyboard + SC 1.4.3 Contrast + SC 2.4.3 Focus Order, augmented by token-driven SC 1.4.11 (non-text contrast) / SC 1.4.1 (not-color-alone) / SC 2.3.3 (reduced-motion) + pattern-inherent SC 4.1.2 / 4.1.3 / 2.1.2 / 1.3.1 / 2.4.7 / 3.3.2. No WCAG 2.2 AAA (no escalation trigger; `wcag22aa` stays unset).

## Harness contract (§3) — bound to obs via violation JSON
Enforcement: `.claude/rules/a11y.md`.
- **Automated tools (desktop-webview):** axe-core 4.12.0 via `@axe-core/webdriverio` injected into the tests' `@crabnebula/tauri-driver` + WebdriverIO session + Lighthouse 13.0.3 + colorjs.io 0.6.1. ONE webview-automation stack (Linux + xvfb) — no second puppeteer/CDP stack.
- **cli / non-UI:** N/A — not-assertable (no DOM); ASCII status prefixes are output-stream discipline, not ARIA.
- **Structured violation JSON:** folds into the obs §6 envelope (axe rule-id + WCAG-SC tag + selector ride in `fingerprints[]`); no host paths / struct names in selectors.
- **Gating:** operator/local gate (needs live Pulse), not a CI gate — rides the tests' `wdio run` job, surfaced via the `logs` command.

## Critical paths (must-be-accessible)
- **Scenario/suite pick → start** — combobox/listbox + button + textbox.
- **Operator-pause go/no-go (HOLD)** — `alertdialog` focus trap (Escape-escapable) + `aria-live="assertive"` phase-line flip.
- **Run-report view** — list/listitem coverage matrix + `status`/`aria-live` lamps + real-prose empty/in-progress states.
- **Operator-checklist (`ManualCheck`)** — checkbox-per-item + unticked-count roll-up.

## Bootstrap phases (§3)
1. **a11y-tooling-install** — axe-core + `@axe-core/webdriverio` + Lighthouse + colorjs.io (reuse tests' tauri-driver/WebdriverIO).
2. **focus-management** — shadcn `AlertDialog` over Radix (reuse, NO install).
3. **aria-component** — shadcn/ui over Radix (reuse, NO install).
4. **contrast-verification-harness** — colorjs.io token-pair ratios (SC 1.4.3 / 1.4.11).
5. **screen-reader-test-spec** — NVDA / VoiceOver / Orca per-state manual pass specs (supplemental).
6. **a11y-ci-gate-wire** — `wdio run` a11y step via the 5-command `logs`, operator/local-gated.
7. **violation-json-emission-wire** — JSON aligned to obs §6, service-tagged.

## Universal anti-patterns
- Never convey state by color alone — six lamps each carry text label + glyph. The six-label assertion keys on the LAMP labels: a token-tinted non-lamp element (the coverage out-of-scope Mode cell reusing `--status-residual`) is a classification, not a seventh state — its own label is its signal.
- Never claim WCAG conformance without machine-verifiable evidence (axe/colorjs.io/Lighthouse JSON).
- Never ARIA on non-semantic HTML; never keyboard traps; never `outline:none` without a `:focus-visible` replacement.
- Never stand up a second browser-automation stack for a11y; manual SR is supplemental only.

## Critical decisions
- **Reuse the tests' tauri-driver/WebdriverIO** session (ONE webview stack — Linux+xvfb in CI, and headfully on the Windows WebView2 host, measured 2026-09-01) — not a parallel CDP attach.
- **Radix/shadcn provide focus trap + Escape + ARIA roles** (no `focus-trap-react`/`react-aria` install) — but NOT focus restoration for the Channel-opened operator-pause dialog: with no Radix `Trigger`, focus lands on `<body>`, so the dialog restores explicitly via `onCloseAutoFocus` + a `restoreFocusTo` accessor and the invoker stays focusable (`aria-disabled`, never native `disabled`). Measured 2026-09-01.
- **Reduced-motion emulation** is the one platform-dependent caveat (WebKitGTK fallback to OS/GTK level); macOS WKWebView stays manual-pass-only.

---

**Full plan:** `.andromeda/a11y-plan.md`. Path-scoped rules: `.claude/rules/a11y.md`.

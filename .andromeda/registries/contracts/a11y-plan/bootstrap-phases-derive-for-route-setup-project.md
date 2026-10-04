### Bootstrap phases (derive for route / setup-project)

_[ALL tiers — explicit derivation hint for downstream consumers
per D26 chain. route reads this to plan bootstrap phase ordering;
setup-project reads this to materialize phase scaffolding.]_

The downstream skills derive the following bootstrap phases from
the contract above. Listed for explicitness — route may reorder /
combine, setup-project may add stack-specific intermediate steps.

- **a11y-tooling-install:** `npm install --save-dev axe-core@4.12.0 @axe-core/webdriverio lighthouse@13.0.3` — and **reuse the tests' already-installed `webdriverio` + `@crabnebula/tauri-driver`** (tests §6 devDeps; no second browser-automation stack). Configure axe `withTags(['wcag2a','wcag2aa','wcag21aa'])` per the WCAG criteria mapping (Minimal — no `wcag22aa`). (puppeteer-core / `@axe-core/puppeteer` / pa11y dropped — they implied a parallel CDP/Windows-only stack.)
- **focus-management-library-install:** no separate focus library needed — focus trap + Escape-escape are provided by **shadcn AlertDialog over Radix Primitives** (already in the stack per design excerpt), but RESTORATION is not — the hold opens from a Tauri `Channel`, so the dialog has no Radix `Trigger` to return to and Radix restored to `<body>` (as measured 2026-09-01). Restoration is a few lines of app code (`onCloseAutoFocus` + a `restoreFocusTo` accessor), still not a library install; the harness asserts that app contract via the WebdriverIO session (no `focus-trap-react` / `react-aria` install).
- **aria-component-library-install:** **shadcn/ui over Radix Primitives** is the ARIA component library (already in the stack) — picker = shadcn `Command`/`Select`, dialog = shadcn `AlertDialog`; no additional ARIA library install.
- **contrast-verification-harness-setup:** `npm install --save-dev colorjs.io@0.6.1` and wire the token-name checker that resolves the named pairs (`--text-primary`/`--color-base` … `--color-focus`/`--color-base`) from the compiled stylesheet and emits PASS/FAIL JSON per pair.
- **screen-reader-test-spec-setup:** the per-state pass spec + the agent-driven NVDA leg per the Screen reader test pattern (must-announce list above) — realized 2026-09-02 (`2026-09-02-screen-reader-manual-spec`): `nvda-pass-spec.md`, the `sr` / `sr-empty` / `sr-error` wdio suites over the one stack, `CONDUCTOR_NVDA` as the host-tool handle (unset ⇒ skip at exit 0), the operator reviewing the graded record; VoiceOver / Orca declared-only on this host; supplemental, not CI-gated.
- **a11y-ci-gate-wire:** wire the `wdio run` a11y step (axe via tauri-driver/WebdriverIO) into tests' 5-command discipline emitting through `logs`, REALIZED 2026-09-07 — the routine arm rides `agent-run run --e2e` and is wired into CI as job `a11y` on `windows-2022` (moved 2026-09-17 from `windows-2025`; conformance gate + artifact upload, both measured executing at run 35208593666); the driven and screen-reader arms stay operator/local. The step is not Linux-bound — the measured SET is Linux+`xvfb`, the Windows WebView2 dev host, and the hosted Windows image at a coherent 131.0.2903.86 driver/runtime pair; only macOS lacks a WebDriver: it runs on Linux+`xvfb` and, measured 2026-09-01, headfully on the Windows WebView2 host under `CONDUCTOR_MSEDGEDRIVER` (unset ⇒ skip at exit 0); only macOS lacks a WebDriver.
- **violation-json-emission-wire:** emit structured violation JSON aligned to obs Section 6 log format (binding), folding axe rule-id + WCAG-SC tag + selector into `fingerprints[]` per the Structured violation JSON schema above.

route uses this list to plan phase ordering (typically:
a11y-tooling-install → focus-management-library-install →
aria-component-library-install → contrast-verification-harness-setup
→ screen-reader-test-spec-setup → a11y-ci-gate-wire →
violation-json-emission-wire). setup-project uses this list to
materialize each phase's bootstrap script + dependency list +
verification command.

---

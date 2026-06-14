## A11y Testing Tool Core

### axe-core

- **Version:** v4.12.0
- **Last release:** 2026-06-01
- **Status:** actively maintained (Deque Labs `dequelabs/axe-core`; releases every few weeks)
- **Agent-runnable:** yes — `axe.run()` returns a structured `AxeResults` object whose `violations[]` array serializes directly to JSON (each entry: `id`, `impact`, `tags`, `description`, `help`, `helpUrl`, `nodes[]` with `target` selectors + `failureSummary`). This is the canonical "axe-core JSON" the obs Section 6 violation schema wraps. Configuration: inject the standalone `axe.min.js` into the WebView2 page over CDP and call `axe.run(document, { runOnly: { type: 'tag', values: ['wcag2a','wcag2aa','wcag21aa','wcag22aa'] } })`; capture the returned JSON.
- **WCAG SCs covered:** scopes via tags `wcag2a / wcag2aa / wcag21a / wcag21aa / wcag22aa` (~50 rules). Directly covers the Minimal baseline + token-driven SCs in a11y-scope Section 3: **SC 1.4.3 Contrast (Minimum)** (`color-contrast` rule), **SC 1.4.11 Non-text Contrast** (`color-contrast-enhanced` / non-text rules), **SC 1.4.1 Use of Color** (partial — pairs with custom not-color-alone assertion), **SC 4.1.2 Name/Role/Value** (`button-name`, `aria-*`, `label` rules — icon-only `aria-label` on titlebar/start-stop/Proceed-Abort), **SC 4.1.3 Status Messages** (`aria-live` / role=status on verdict lamp + live counters), **SC 1.3.1 Info and Relationships** (`list`, `listitem`, `aria-required-children` for the coverage matrix), **SC 2.4.6 Headings/Labels**, **SC 1.1.1**. v4.12.0 adds a rule requiring an accessible name for `role="tab"`. WCAG 2.2 rules ship but are off by default — must be enabled via the `wcag22aa` tag.
- **Fits because:** sole automated engine for the single assertable surface (a11y-scope Section 2 desktop-webview; "axe-core ... against the webview DOM"). Drives the visual-discrimination trigger (Section 5) color-contrast + name/role assertions and all four must-be-accessible paths (Section 4 — picker combobox names, alertdialog roles, lamp `aria-live`, checklist `aria-checked`). It is the JSON-producing variant; the axe DevTools browser extension (UI-only) is explicitly NOT recommended.
- **Key detail:** axe-core is a library, not a runner — it needs a host (puppeteer/CDP) to inject into the Tauri WebView2 DOM. WCAG 2.2 SCs (e.g., SC 2.4.11 Focus Not Obscured) require explicitly adding the `wcag22aa` tag; default config omits them.
- **Source:** https://github.com/dequelabs/axe-core/releases , https://www.npmjs.com/package/axe-core

### Lighthouse (`lighthouse` npm CLI — accessibility category)

- **Version:** v13.0.3 (bundles `axe-core ^4.11.4`)
- **Last release:** 2026-02-11 (v13.0.3); v12.8.0 was 2025-07-11
- **Status:** actively maintained (GoogleChrome/lighthouse)
- **Agent-runnable:** yes — produces Lighthouse a11y category JSON. Configuration: `lighthouse <url> --only-categories=accessibility --output=json --output-path=a11y.json --port=9222` pointed at the WebView2 CDP endpoint (`--remote-debugging-port=9222`). The JSON `audits` block carries per-audit `score` + failing `details.items[]` with node selectors; `categories.accessibility.score` is the gate-able rollup.
- **WCAG SCs covered:** runs 57 curated a11y audits as a subset of axe-core's ruleset — covers the same WCAG 2.0/2.1 A+AA SC families as axe-core but a narrower rule set (notably **SC 1.4.3 Contrast**, **SC 4.1.2 Name/Role/Value**, **SC 1.3.1 Info and Relationships**, **SC 2.4.6 Headings/Labels**, **SC 1.1.1 Non-text Content**). Does NOT run the full axe ruleset (no enhanced/2.2 rules) — secondary to axe-core for SC depth.
- **Fits because:** a11y-scope Section 2 names "Lighthouse a11y category (... `lighthouse` CLI against the webview)" as in-reach for desktop-webview. Provides a single category-score gate convenient for the CI/operator gate (Section 3 CI integration) alongside the richer axe-core JSON.
- **Key detail:** because Lighthouse runs only an axe subset, it is a coarse complementary gate — axe-core (full run via puppeteer) remains the SC-coverage source of truth. Lighthouse must attach to the CDP port, so it inherits the Windows-only WebView2 constraint (below).
- **Source:** https://github.com/GoogleChrome/lighthouse/releases , https://www.npmjs.com/package/lighthouse

### pa11y / pa11y-ci

- **Version:** `pa11y` v9.1.1; `pa11y-ci` v4.1.1
- **Last release:** pa11y 2026-02-26 (v9.1.1); pa11y-ci 2026-05-12 (v4.1.1)
- **Status:** actively maintained (`pa11y` org; previous major supported 6 months for critical/security fixes)
- **Agent-runnable:** yes — pa11y ships a built-in JSON reporter (`pa11y --reporter json <url>` or `pa11y-ci --json`), no separate install. Output is an array of issues (`code`, `type` error/warning/notice, `message`, `selector`, `context`, `runner`). Configuration: point at the WebView2 CDP target; select the `axe` runner (`runners: ['axe']`) so issue `code`s carry axe rule IDs / WCAG tags.
- **WCAG SCs covered:** standard-bounded — default runner htmlcs (HTML CodeSniffer) maps to **WCAG2AA** with per-issue WCAG technique codes (e.g., `WCAG2AA.Principle1.Guideline1_4.1_4_3` → **SC 1.4.3**; `...1_3_1` → **SC 1.3.1**; `...4_1_2` → **SC 4.1.2**); supports `WCAG2A` and (htmlcs-only) `WCAG2AAA`. The `axe` runner additionally maps to axe rule tags. So pa11y emits explicit per-SC technique codes, satisfying the SC-mapping rule.
- **Fits because:** secondary CI-centric runner for the desktop-webview surface; a11y-scope Section 2 notes "pa11y (`pa11y` / `pa11y-ci`) is applicable in principle against the rendered DOM but requires CDP attach to the Tauri webview." Its explicit `WCAG2AA.<SC>` codes make it the most directly SC-tagged JSON for downstream Phase 3 SC mapping.
- **Key detail:** pa11y is built around URL/CDP attach; against Tauri it requires the same WebView2 `--remote-debugging-port` endpoint as axe/Lighthouse (no localhost HTTP server exists — Conductor exposes no HTTP surface). htmlcs and axe runners disagree on some rules; pick one runner per gate for determinism (aligns with tests' zero-flakiness rule).
- **Source:** https://github.com/pa11y/pa11y , https://www.npmjs.com/package/pa11y-ci

## Per-Surface A11y Testing Library

_Surface coverage: desktop-webview (assertable). cli surface is flagged **not-assertable** in a11y-scope Section 1 (line-oriented terminal, no DOM/native widget tree, no axe/Lighthouse/pa11y reach) — skipped per research-targets "skip surfaces flagged not-assertable." No web/SSR/mobile/desktop-native/API surface exists in the Stack._

### @axe-core/puppeteer

- **Version:** v4.11.3 (part of the `dequelabs/axe-core-npm` monorepo; version tracks axe-core major.minor)
- **Last release:** 2026-05-20 (v4.11.3)
- **Status:** actively maintained
- **Agent-runnable:** yes — `new AxePuppeteer(page).analyze()` returns the axe-core `AxeResults` JSON (same `violations[]` schema as axe-core core). Configuration: `puppeteer.connect({ browserURL: 'http://localhost:9222' })` to attach to the running Tauri **WebView2** CDP endpoint, get the page, then `await new AxePuppeteer(page).withTags(['wcag2a','wcag2aa','wcag21aa']).analyze()` and write JSON.
- **WCAG SCs covered:** inherits axe-core's full SC coverage (see axe-core block) — **SC 1.4.3, SC 1.4.11, SC 1.4.1 (partial), SC 4.1.2, SC 4.1.3, SC 1.3.1, SC 2.4.6, SC 1.1.1**, scoped by tag.
- **Fits because:** a11y-scope Section 2/3 names `@axe-core/puppeteer` as THE candidate driver to attach axe-core over DevTools/CDP to the embedded Tauri webview (no localhost URL — CDP attach). Puppeteer drives the same DOM for all four must-be-accessible paths. Chosen over `@axe-core/playwright` because the Tauri-CDP attach story is Windows/WebView2 + Chromium-protocol, which puppeteer's `connect({browserURL})` targets directly (Playwright's webkit channel does not help the macOS/Linux webviews either — see constraint).
- **Key detail (binding constraint):** Tauri CDP is **Windows-only**. WebView2 (Windows) exposes CDP via `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=9222`; macOS (WKWebView) and Linux (WebKitGTK) expose only Safari/WebKitGTK inspectors with **no CDP** → no axe/puppeteer attach there. Net: the automated a11y harness runs against the Windows WebView2 build only; macOS/Linux fall back to manual NVDA-equivalent passes. This matches a11y-scope Section 3 CI integration ("operator/local gate, not a CI gate, unless Phase 3 adds a headless webview-CDP step") and the Architecture note that CI runs "on the dev OS target." Phase 3 must scope the CDP a11y step to the Windows runner.
- **Source:** https://www.npmjs.com/package/@axe-core/puppeteer , https://github.com/dequelabs/axe-core-npm/releases , https://github.com/Haprog/tauri-cdp

### chrome-remote-interface (CDP client — low-level axe injection fallback)

- **Version:** v0.34.0
- **Last release:** 2026-02-09
- **Status:** actively maintained
- **Agent-runnable:** yes — connect to the WebView2 CDP port, `Page.enable()`, inject `axe.min.js` via `Runtime.evaluate({ expression: <axe source> })`, then `Runtime.evaluate({ expression: 'axe.run()', awaitPromise: true, returnByValue: true })` and read `result.value` as axe-core JSON. Configuration: `CDP({ port: 9222 })`.
- **WCAG SCs covered:** inherits axe-core SC coverage (it is a transport for axe-core JSON) — **SC 1.4.3, SC 1.4.11, SC 4.1.2, SC 4.1.3, SC 1.3.1**, tag-scoped.
- **Fits because:** lighter-weight alternative to puppeteer for attaching axe-core to the Tauri WebView2 DOM when a full puppeteer/Playwright dependency is unwanted in a Rust-first repo (a11y-scope Section 2: "axe-core/Lighthouse must attach over CDP to the embedded webview rather than a localhost URL"). Same Windows-only CDP constraint applies.
- **Key detail:** pure CDP client (no browser-launch assumptions) — ideal because the Tauri app is already running and only needs an `attach`, not a launch. Same WebView2-only limitation as `@axe-core/puppeteer`.
- **Source:** https://github.com/cyrus-and/chrome-remote-interface , https://www.npmjs.com/package/chrome-remote-interface

## Keyboard Test Harness Pattern

### Puppeteer keyboard API over CDP (`page.keyboard` / `Input.dispatchKeyEvent`)

- **Version:** v24.10.1 (`puppeteer-core`, current 2026 line) driving the same `@axe-core/puppeteer` CDP attach above; no separate package
- **Last release:** 2026-06-05
- **Status:** actively maintained
- **Agent-runnable:** yes — assertions are scripted; results emit as structured PASS/FAIL JSON aligned to the obs Section 6 violation envelope (custom assertion script comparing `document.activeElement` selector after each key). `page.keyboard.press('Tab' | 'Shift+Tab' | 'ArrowDown' | 'Enter' | 'Space' | 'Escape')`; read focused element via `page.evaluate(() => document.activeElement?.getAttribute('data-testid'))`.
- **WCAG SCs covered:** **SC 2.1.1 Keyboard** (Enter/Space invoke default action on button / Proceed / Abort), **SC 2.4.3 Focus Order** (scripted Tab sequence asserts picker→start→matrix order), **SC 2.1.2 No Keyboard Trap** (Tab+Shift+Tab cycle inside the operator-pause `alertdialog` trap stays escapable via Escape), **SC 2.4.7 Focus Visible** (asserts `--color-focus` ring on the active element; pairs with contrast tool for the 3:1 ring).
- **Fits because:** directly realizes a11y-scope Section 3 "Keyboard test harness: scripted key sequences against the desktop-webview" and the keyboard contracts in every must-be-accessible path (Section 4): dialog Tab/Shift+Tab + Enter/Space + Escape; picker Arrow-navigate + Enter-select + type-ahead; checklist Space-toggles-a-row; coverage-matrix keyboard-first row navigation; first-class start/stop/proceed/abort shortcuts.
- **Key detail:** no Playwright/Cypress in the Stack to reuse (a11y-scope Section 3: "No existing E2E driver to reuse"), so keyboard scripting piggybacks on the same puppeteer-CDP session used for axe-core rather than a dedicated E2E framework. Same Windows-WebView2-only CDP constraint. `cypress-real-events` / `cy.tab()` are NOT applicable (no Cypress in stack). CLI keyboard verification is manual-only and out of harness scope (not-assertable surface).
- **Source:** https://github.com/microsoft/playwright/issues/35375 , https://www.npmjs.com/package/@axe-core/puppeteer

## Contrast Verification Tool

### axe-core `color-contrast` rule (runtime, rendered-DOM)

- **Version:** v4.12.0 (`color-contrast` rule default-on since axe-core 3.x; `color-contrast-enhanced` for AAA)
- **Last release:** 2026-06-01
- **Status:** actively maintained
- **Agent-runnable:** yes — contrast failures appear in the standard axe `violations[]` JSON with `id: "color-contrast"`, each node's `failureSummary` carrying the computed ratio and required ratio. Configuration: included in any tagged `axe.run()`; no extra setup.
- **WCAG SCs covered:** **SC 1.4.3 Contrast (Minimum)** (4.5:1 normal / 3:1 large via `color-contrast`), **SC 1.4.11 Non-text Contrast** (3:1 UI components/graphics — status lamps + focus ring), **SC 1.4.6 Contrast (Enhanced)** (7:1 via `color-contrast-enhanced`, only if AAA enabled — not in Minimal baseline).
- **Fits because:** drives the visual-discrimination trigger (a11y-scope Section 5) and the contrast half of Section 4 path 3 (six verdict/state lamps at 3:1). Computes ratios on the *rendered* DOM, so it validates the resolved values of the design tokens after Tailwind compiles them.
- **Key detail:** axe measures *rendered* color, not token *names* — so it confirms the computed pair but cannot by itself assert "token `--text-primary` on `--color-base` meets 4.5:1" by name. The design-token binding (a11y-scope Section 3, `[NO_RAW_DESIGN_VALUES]`) needs the name-level checker below to complement it.
- **Source:** https://github.com/dequelabs/axe-core , https://www.deque.com/axe/core-documentation/api-documentation/

### colorjs.io (token-name-bound ratio assertion)

- **Version:** v0.6.1
- **Last release:** 2026-01-15
- **Status:** actively maintained (color-js/color.js — by the CSS Color spec editors)
- **Agent-runnable:** yes — `new Color(fg).contrast(bg, "WCAG21")` returns the numeric ratio; a thin custom script reads the named token pairs from a11y-scope Section 3, resolves each token's value (from the compiled CSS custom properties), computes the ratio, and emits machine-readable `{ token_fg, token_bg, ratio, required, pass }` JSON folded into the obs Section 6 envelope. Configuration: `import Color from "colorjs.io"`.
- **WCAG SCs covered:** **SC 1.4.3 Contrast (Minimum)** (`contrastWCAG21` for 4.5:1/3:1), **SC 1.4.11 Non-text Contrast** (apply the 3:1 threshold to `--count-nominal`/`--color-base` and `--color-focus`/`--color-base`), **SC 1.4.6 Contrast (Enhanced)** (7:1 if asserted). Also exposes APCA for future-proofing (not WCAG-normative).
- **Fits because:** satisfies the **binding design-token contract** (a11y-scope Section 3 contrast harness: "reads token NAMES verbatim ... emits machine-verifiable ratio assertions"). Asserts the exact named pairs: `--text-primary`/`--color-base`, `--text-secondary`/`--color-raised-1`, `--text-tertiary`/`--color-base`, `--text-muted`/`--color-inset`, `--color-id-cyan`/`--color-base`, `--count-nominal`/`--color-base` (3:1), `--count-hold`/`--color-base`, `--status-fail`/`--color-base`, `--color-focus`/`--color-base` (3:1 focus), plus `--border-emphasis` selection-vs-focus distinction.
- **Key detail:** chosen over the popular `wcag-contrast` package, which is **stale (last publish 2019-11-05)** and fails the 2025-2026 recency rule. colorjs.io's native `WCAG21` contrast algorithm makes the named-token assertion a few lines and is spec-authoritative. Raw token values stay in design-system.md; the script resolves them at runtime from the compiled stylesheet, preserving `[NO_RAW_DESIGN_VALUES]`.
- **Source:** https://www.npmjs.com/package/colorjs.io , https://colorjs.io/docs/contrast

### chroma-js (alternative ratio computer)

- **Version:** v3.2.0
- **Last release:** 2025-11-28
- **Status:** actively maintained
- **Agent-runnable:** yes — `chroma.contrast(fg, bg)` returns the WCAG ratio; same token-name-bound custom-script pattern emitting JSON. Configuration: `import chroma from "chroma-js"`.
- **WCAG SCs covered:** **SC 1.4.3 Contrast (Minimum)**, **SC 1.4.11 Non-text Contrast** (apply 3:1 threshold in the assertion wrapper).
- **Fits because:** drop-in alternative to colorjs.io for the same token-name contrast binding (a11y-scope Section 3) if the team prefers chroma-js's broader color-manipulation API. Either one (not both) satisfies the contract.
- **Key detail:** `chroma.contrast()` returns the raw ratio only (no built-in WCAG pass/fail or large-text branch) — the wrapper script must apply the 4.5:1 / 3:1 thresholds. Prefer colorjs.io when you want the algorithm name (`WCAG21`) explicit in code.
- **Source:** https://github.com/gka/chroma.js , https://www.npmjs.com/package/chroma-js

## CI Integration Pattern

### GitHub Actions — axe-core/pa11y JSON artifact gate (operator/local-gated for live runs)

- **Version:** N/A (convention pattern, not a versioned package)
- **Last release:** N/A (convention pattern)
- **Status:** actively maintained platform; current best practice
- **Agent-runnable:** yes — the a11y job writes axe-core JSON / pa11y JSON / Lighthouse a11y JSON to disk and `actions/upload-artifact` uploads them; the step fails the job (non-zero exit) when `violations.length > 0`, surfacing per-PR new-violation diffs. JSON artifacts are machine-consumable by downstream agents and align to the obs Section 6 envelope.
- **WCAG SCs covered:** inherits whatever the underlying tool covers per run (axe-core ~50 SCs incl. **SC 1.4.3 / 1.4.11 / 4.1.2 / 4.1.3 / 1.3.1**); the CI layer adds no SCs, it gates + persists them.
- **Fits because:** a11y-scope Section 3 CI integration — GitHub Actions is the Architecture CI/CD platform and currently runs `cargo build` + nextest + clippy ("build + test gating only"). The webview-CDP a11y step lands as an **operator/local gate, not a CI gate**, because (a) dynamic scenario proof needs a live Pulse and (b) the CDP attach is Windows-WebView2-only. Reuses tests' binding 5-command harness surface (boot/run/status/cleanup/logs) — Phase 3 specifies which command emits a11y JSON. No `npm test` lane exists in the Rust-first stack, so the a11y JSON job is an added Node step gated to the Windows runner.
- **Key detail:** because there is no existing browser E2E driver to piggyback on (tests' E2E-driver-reuse contract has no driver yet — a11y-scope Section 3), Phase 3 must either (1) add a minimal Node+puppeteer step that launches the Tauri app with `--remote-debugging-port=9222`, attaches, runs axe, uploads JSON; or (2) keep it as a local operator gate. The single-OS CI ("on the dev OS target") means the matrix is one runner — set it to `windows-latest` for the CDP path.
- **Source:** https://dev.to/agentkit/how-to-set-up-automated-accessibility-testing-in-github-actions-copy-paste-config-2eib , https://accessibility.civicactions.com/posts/automated-accessibility-testing-leveraging-github-actions-and-pa11y-ci-with-axe

## Service Identity Convention

### Inherited from obs Service Identity (no WebSearch — per research-targets)

- **Version:** N/A (convention pattern, not a versioned package)
- **Last release:** N/A (convention pattern)
- **Status:** binding contract from upstream-context Section 6 (Obs Plan Excerpt) → Service Identity
- **Agent-runnable:** yes — a11y violation JSON artifacts are tagged with the same OpenTelemetry resource attributes as obs emissions so violation records cross-correlate with obs traces. The structured violation JSON (obs Section 6 schema) carries these as resource fields.
- **WCAG SCs covered:** N/A (cross-correlation metadata convention, not an SC-bearing tool) — tags the SC-bearing axe-core/pa11y/Lighthouse violation artifacts (which collectively carry SC 1.4.3 / SC 2.1.1 / SC 2.4.3 baseline) with service identity for trace correlation.
- **Fits because:** a11y-scope Section 2 "Service identity tagging" binds a11y artifacts to `service.name = "conductor-tauri"` (Tauri GUI) or `"conductor-ui"` (browser frontend), overrideable via `$CONDUCTOR_SERVICE_NAME`; `service.version = env!("CARGO_PKG_VERSION")`; `deployment.environment = CONDUCTOR_ENV` (fallback `"local"`). The desktop-webview is the only assertable surface, so a11y artifacts tag `conductor-tauri` / `conductor-ui`.
- **Key detail:** binding direction is fixed — a11y inherits obs's identity scheme (obs fixed it; a11y aligns). Artifacts MUST NOT leak absolute host paths or internal seam-crate struct names (inherited from tests' Status JSON constraint, a11y-scope Section 3).
- **Source:** https://opentelemetry.io/docs/specs/semconv/resource/ (OpenTelemetry resource semantic conventions — `service.name` / `service.version` / `deployment.environment`, the obs-inherited tagging basis); upstream binding: upstream-context.md Section 6 (Obs Plan Excerpt → Service Identity), cross-validated with a11y-scope Section 2.

## Motion / Reduced-Motion Library

_[trigger-driven; pulled in by **motion-sensitive** trigger from a11y-scope Sec 5; not standard for Minimal tier but required for trigger coverage]_

### Tailwind CSS v4.1 `motion-reduce:` / `motion-safe:` variants + global `prefers-reduced-motion` rule

- **Version:** v4.1.18
- **Last release:** 2025-12-11
- **Status:** actively maintained
- **Agent-runnable:** yes — the *enforcement* is CSS (`motion-reduce:` variant wraps a `@media (prefers-reduced-motion: reduce)` query; `@theme` defines transitions without a plugin), and the *verification* is machine-driven (below). The reduced-motion rule's effect is asserted by toggling the media feature over CDP and diffing computed `animation`/`transition` styles to PASS/FAIL JSON.
- **WCAG SCs covered:** **SC 2.3.3 Animation from Interactions** (AAA — animations dropped under `prefers-reduced-motion: reduce`), supporting **SC 2.3.1 Three Flashes** (no flashing; design already holds the `Fail` lamp motionless).
- **Fits because:** the motion-sensitive trigger (a11y-scope Section 5) is **token-driven** — design's global reduced-motion rule "drops all animations/transitions (binding with a11y SC 2.3.3, including the held-count tint transition)." Tailwind v4's `motion-reduce:` variant + `@theme`-defined `--motion-micro` / `--ease-quiet` are exactly the native mechanism that implements that rule (no framer-motion in the Stack, so the framer-motion `useReducedMotion` hook from research-targets is N/A here).
- **Key detail:** the assertion target is: under `prefers-reduced-motion: reduce`, the count tint (green→amber→slate-violet), focus-ring fade-in, status-light color transition, and operator-pause dialog fade are all dropped (a11y-scope Section 5 required assertion). This is CSS-native, not JS-library-driven, which is why the recommendation is the Tailwind variant rather than a runtime motion package.
- **Source:** https://tailwindcss.com/docs/animation , https://github.com/tailwindlabs/tailwindcss/discussions/12864

### Reduced-motion verification: CDP media emulation (`Emulation.setEmulatedMedia` / Playwright `reducedMotion`)

- **Version:** N/A (convention pattern, not a versioned package)
- **Last release:** N/A (convention pattern)
- **Status:** actively maintained
- **Agent-runnable:** yes — over the WebView2 CDP session, `Emulation.setEmulatedMedia({ features: [{ name: 'prefers-reduced-motion', value: 'reduce' }] })`, then `Runtime.evaluate` to read computed `getComputedStyle(el).transitionDuration` / `animationName` and assert they collapse to none/0s; emit `{ element, prop, value, reduced_motion_respected }` JSON. (Playwright equivalent: `contextOptions: { reducedMotion: 'reduce' }` or `page.emulateMedia({ reducedMotion: 'reduce' })`.)
- **WCAG SCs covered:** **SC 2.3.3 Animation from Interactions** (verifies the reduced-motion drop at runtime).
- **Fits because:** provides the *machine-verifiable* half of the motion-sensitive trigger (a11y-scope Section 5 required assertion) — proves the Tailwind `motion-reduce` rule actually fires, rather than trusting the CSS visually. Runs in the same WebView2-CDP session as axe-core (Windows-only constraint applies).
- **Key detail:** known Playwright bug — emulated `reducedMotion` may reset after a page navigation (Playwright issue #31328); for Conductor this is low-risk because the desktop-webview has **no routes** (four in-place run-STATES, not navigations — a11y-scope Section 2), so a single emulation set persists across the state transitions under test.
- **Source:** https://learn.microsoft.com/en-gb/microsoft-edge/devtools-guide-chromium/accessibility/reduced-motion-simulation , https://github.com/microsoft/playwright/issues/3320

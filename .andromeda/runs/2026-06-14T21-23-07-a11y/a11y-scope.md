## 1. A11y Scope

What entities in this project need a11y assertions:

- **Entity:** Operator-pause go/no-go dialog (Proceed / Abort)
  - **Source:** design excerpt's User-Facing Error Surfaces + ARIA-Relevant Component Patterns ("Operator-pause go/no-go dialog"); layout excerpt's Focus Management Anchors (run-console-HOLD)
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** Verdict / report-state lamp (six states: `Pass` / `CalibrationRegion` / `Fail` / `Blocked` / `Manual` / `Residual`)
  - **Source:** design excerpt's User-Facing Error Surfaces (`Fail` lamp) + ARIA-Relevant Component Patterns ("Verdict / report-state lamp")
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** Coverage matrix (single-row-per-P-ID list)
  - **Source:** design excerpt's ARIA-Relevant Component Patterns ("Coverage matrix") + Loading/Error/Empty State Patterns (Coverage-matrix empty state)
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** Operator-checklist (`ManualCheck` render — checkbox-per-item)
  - **Source:** design excerpt's User-Facing Error Surfaces ("Operator-checklist") + ARIA-Relevant Component Patterns ("Operator-checklist"); creator brief excerpt's Must-Work Scenarios (Run report surface)
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** Scenario / suite picker + start/stop (combobox/listbox over shadcn `Command`/`Select`)
  - **Source:** design excerpt's ARIA-Relevant Component Patterns ("Scenario / suite picker + start/stop"); arch excerpt's Project Intent Summary critical paths hint (scenario/suite picker action, start/stop)
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** Button (titlebar min/close, start/stop, Proceed/Abort — icon-only controls)
  - **Source:** design excerpt's ARIA-Relevant Component Patterns ("Button"); layout excerpt's Focus Management Anchors (desktop-webview window controls)
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** Form input (scenario / seed fields — textbox)
  - **Source:** design excerpt's ARIA-Relevant Component Patterns ("Form input")
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** Run-report view (run-states: idle / live / HOLD / report-terminal; live emission counters + target status streaming display)
  - **Source:** layout excerpt's Layout Types per Surface (desktop-webview four run-states); arch excerpt's Project Intent Summary critical paths hint (run-report view, live emission counters via Tauri `Channel`); design excerpt's Loading/Error/Empty State Patterns (run-report empty / in-progress states, paused-count hold-point)
  - **Assertability:** assertable
  - **Reason if not fully assertable:** N/A

- **Entity:** conductor-cli (`agent-run` headless binary / `conductor-run`, `conductor-suite`, `conductor-report`)
  - **Source:** arch excerpt's Stack ("conductor-cli `agent-run` headless binary") + Surfaces (headless CLI/`agent-run`); design excerpt's Surfaces (cli); layout excerpt's Layout Types per Surface (cli)
  - **Assertability:** not-assertable
  - **Reason if not fully assertable:** CLI line-oriented terminal surface — no DOM / native widget tree; a11y tools (axe / Lighthouse / pa11y) have no applicability per arch excerpt's Stack. Manual keyboard/terminal discipline + ASCII status prefixes / NO_COLOR text-paired output is fallback only. Excluded from Sections 2 and 4.

Cross-check against security plan excerpt's A11y Compliance Triggers: "No a11y compliance triggers in security plan" — no regulated-surface → WCAG SC mandate to bind. No boundary-only zones: arch excerpt's Surfaces confirms "No web/HTTP surface — Conductor exposes no HTTP/network service of its own" and Tauri capabilities are deny-by-default with "no remote-origin iframe affecting a11y selectors" (tests excerpt's Coverage Triggers, Vector 3) — so no third-party iframe / CAPTCHA / vendor-widget boundary zone exists.

## 2. A11y Surfaces & Assistive Tech Reach

- **Surface:** desktop-webview (Tauri 2 bundled webview / React 19 + Tailwind v4.1 + shadcn/ui over Radix Primitives; frameless window, no responsive breakpoints)
  - **Source:** arch excerpt's Stack (Tauri 2 desktop shell) + Surfaces (Tauri 2 desktop webview GUI); design excerpt's Surfaces (desktop-webview)
  - **Automated tool reach:** axe-core via DevTools/CDP protocol against the embedded webview (`@axe-core/puppeteer` / `@axe-core/playwright` against the webview DOM) + Lighthouse a11y category (Chrome DevTools / `lighthouse` CLI against the webview); per design excerpt's Surfaces "axe-core + Lighthouse + screen reader (NVDA / VoiceOver) against the webview DOM." Native menubar / system dialogs are out of scope by design (arch excerpt's Stack: "no native OS toasts by design, so no OS-level notification a11y surface"). pa11y (`pa11y` / `pa11y-ci`) is applicable in principle against the rendered DOM but requires CDP attach to the Tauri webview.
  - **Manual verification (supplemental):** NVDA (Windows) / VoiceOver (macOS) / Orca (Linux) — desktop-bound Windows/macOS/Linux target per design excerpt's Surfaces; supplemental to automated, never sole. No TalkBack/mobile SR (arch excerpt's Surfaces: "No mobile surface").
  - **ARIA roles inventory:** interactive component roles — `alertdialog` (operator-pause), `button` (titlebar / start-stop / Proceed-Abort), `combobox`/`listbox` (scenario/suite picker), `textbox` (scenario/seed form inputs), `checkbox` + group/region (operator-checklist); live region roles — `status` / `aria-live` (verdict/report-state lamp, live emission counters, HOLD phase-line `aria-live="assertive"`); list/listitem (or table semantics) for the coverage matrix; landmark roles NOT explicitly assigned in layouts — per layout excerpt's Heading Hierarchy Anchors, Heading role on phase line, Label role on footer status strip, frameless titlebar `data-tauri-drag-region`; main / banner / contentinfo equivalents to be derived Phase 3. (All ARIA from design excerpt's ARIA-Relevant Component Patterns + layout excerpt's Heading Hierarchy Anchors.)
  - **Service identity tagging:** `service.name` = `"conductor-tauri"` (Tauri GUI) or `"conductor-ui"` (browser frontend), overrideable via `$CONDUCTOR_SERVICE_NAME`; `deployment.environment` = `CONDUCTOR_ENV` (fallback `"local"`) — per obs excerpt's Service Identity. A11y violation artifacts tag with these resource attributes for cross-correlation.
  - **Notes:** Tauri webview a11y tree is exposed via DevTools/CDP, not a standard browser endpoint — axe-core/Lighthouse must attach over CDP to the embedded webview rather than a localhost URL (arch excerpt's Stack). Single frameless window with four run-STATES (idle / live / HOLD / report-terminal), not four routes — no router, no breakpoints (layout excerpt's Layout Types) → no route-change focus surface, all state transitions are in-place. Virtual-scrolled coverage matrix: off-screen rows must remain reachable/announceable (design excerpt's ARIA-Relevant Component Patterns). VoiceOver macOS has no automated SR test — manual pass required.

(cli surface omitted here — flagged not-assertable in Section 1: no DOM / native widget tree, no automated a11y tool reach per arch excerpt's Stack and design excerpt's Surfaces.)

## 3. A11y Assertion Harness Specification

- **A11y testing tool pick:** Deferred to Phase 2 research / Phase 3 plan. Per-surface tool reach (Section 2): axe-core via DevTools/CDP (`@axe-core/puppeteer` / `@axe-core/playwright` against the Tauri webview) + Lighthouse a11y category as the candidate set for the single assertable surface (desktop-webview). Note (arch excerpt's Stack/Testing frameworks): the Rust stack is cargo-nextest / cargo test / golden tests with "no Playwright/Cypress/Selenium or equivalent browser E2E driver exists in the stack for a11y harness reuse" — so the tests' E2E-driver-reuse binding contract has no existing browser driver to piggyback on; Phase 3 must reconcile this (a11y aligns to upstream tests harness if/when one is added).
- **WCAG criteria mapping:** Minimal tier (see Section 6) → baseline only: **SC 2.1.1 Keyboard** + **SC 1.4.3 Contrast (Minimum)** + **SC 2.4.3 Focus Order**. Design-token-backed non-text minima also in baseline reach from design excerpt's A11y-Relevant Design Tokens: SC 1.4.11 Non-text Contrast 3:1 (`--count-nominal`, `--color-focus` tokens annotated to serve it) and SC 1.4.1 Use of Color / not-color-alone (every state token carries a text-label + glyph supplement). SC 2.3.3 Animation from Interactions is bound by design's global reduced-motion rule. No AA/AAA escalation triggers present (Section 5 = None) — tier label stays Minimal, no trigger-pulled AAA SCs added.
- **Structured violation JSON schema:** Binding contract — a11y violation events align to obs excerpt's Log Format JSON Schema (obs Section 6), reproduced verbatim:
  ```jsonl
  {
    "journal_emitted_at": "ISO-8601 from std::time::SystemTime",
    "run_id": "YYYY-MM-DDTHH-MM-SS-<suffix> (filesystem-safe hyphen-delimited)",
    "seed": "u64",
    "scenario": "string",
    "p_ids": ["P-001", "P-002", ...],
    "verdict": "Pass | Fail | CalibrationRegion",
    "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
    "latency_ms": "integer or null (null for blocked rows)",
    "slo_tier": "<5s | <20s | <90s",
    "fingerprints": ["fingerprint1", "fingerprint2", ...] or empty array
  }
  ```
  A11y emissions align to this obs schema (a11y narrows; obs fixed the schema). Per tests excerpt's Status JSON shape constraint, artifacts MUST NOT leak absolute host paths or internal seam-crate struct names — a11y violation artifacts inherit that constraint. Phase 3 will define how WCAG-violation fields map onto / extend this envelope without diverging from the obs contract.
- **Focus management test harness:** No existing E2E driver to reuse (arch excerpt's Stack: no Playwright/Cypress/Selenium). Candidate reach is axe-core/CDP-driver Tab tracking against the Tauri webview; focus anchors to assert from layout excerpt's Focus Management Anchors: modal focus trap on run-console-HOLD (shadcn AlertDialog / Radix, dimmed/inert console, visible `--color-focus` ring on trapped region, Proceed/Abort in-trap targets), focus restoration on dialog close (Radix default, implied), keyboard-first global focus ring on every interactive control with `--border-emphasis` on active picker item so focus is never color-only. Obs has no focus spans yet (obs excerpt's Focus-Relevant Span Coverage: "No focus-relevant spans"; Phase 3 may recommend focus.shift / focus.trap.enter / focus.trap.exit / focus.restore).
- **Keyboard test harness:** Scripted key sequences against the desktop-webview only. From design excerpt's ARIA-Relevant Component Patterns keyboard contracts: dialog Tab/Shift+Tab cycle within trap + Enter/Space activate Proceed/Abort + Escape per convention; button Enter/Space; picker Arrow-navigate + Enter-select + Escape-close + type-ahead; checklist Space toggles a row; coverage-matrix keyboard-first row navigation; first-class start/stop/proceed/abort shortcuts (layout excerpt's Focus Management Anchors). CLI keyboard verification is manual only and out of harness scope (not-assertable surface).
- **Screen reader test pattern:** Manual, supplemental (never sole). NVDA (Windows) / VoiceOver (macOS) / Orca (Linux) pass against the webview DOM per design excerpt's Surfaces. Must-announce items from design excerpt: empty-state real prose ("No scenarios loaded", "No run yet"), in-progress "Run in progress" not announced as Fail, HOLD phase-line via `aria-live="assertive"`, frozen count value (does not blank/keep running), verdict lamp `aria-live` status announcement, operator-checklist unticked-count roll-up. No automated SR tool for the stack.
- **Contrast verification harness:** Programmatic contrast assertion reading design excerpt's A11y-Relevant Design Tokens (binding — Phase 3 reproduces token names verbatim) — emits machine-verifiable ratio assertions per WCAG SC 1.4.3 (AA 4.5:1 normal / 3:1 large) and SC 1.4.11 (3:1 non-text). Token pairs to assert (foreground / background, verbatim from design excerpt): `--text-primary`/`--color-base` (SC 1.4.3 4.5:1 normal-text), `--text-secondary`/`--color-raised-1`, `--text-tertiary`/`--color-base`, `--text-muted`/`--color-inset`, `--color-id-cyan`/`--color-base`, `--count-nominal`/`--color-base` (SC 1.4.11 3:1 non-text status), `--count-hold`/`--color-base`, `--status-fail`/`--color-base`, `--color-focus`/`--color-base` (SC 1.4.11 3:1 non-text focus indicator). Focus-ring tokens `--color-focus` + `--border-emphasis` (selection vs focus distinction). Candidate: axe-core color-contrast rule against rendered DOM + custom assertion reading the named token pairs. Raw values stay in design-system.md (`[NO_RAW_DESIGN_VALUES]`); names are the cross-skill anchor.
- **CI integration:** GitHub Actions (arch excerpt's CI/CD Platform) currently runs `cargo build` + cargo-nextest/`cargo test` (incl. golden) + `cargo clippy` — "build + test gating only; dynamic end-to-end scenario proof requires a live Pulse and is an operator/local gate, not a CI gate." The desktop-webview a11y check therefore lands as an operator/local gate, not a CI gate, unless Phase 3 adds a headless webview-CDP step. Tests' 5-command harness contract names (boot / run / status / cleanup / logs — tests excerpt's Test Harness Contract) is the binding command surface a11y integration reuses; Phase 3 specifies which command emits a11y results. No `npm test` lane exists in the Rust-first stack (arch excerpt's Stack).

## 4. Critical Paths (must-be-accessible)

(cli / headless `conductor run` paths and tests' security-vector / build-gate coverage triggers are excluded — not-assertable, no rendered-UI surface. The only accessible surface is the Tauri 2 desktop-webview control panel, per creator brief excerpt's Must-Work Scenarios.)

- **Path:** Scenario/suite pick → start run (operator launches a scenario or suite from the control panel)
  - **Surfaces involved:** desktop-webview
  - **Required ARIA roles:** `combobox`/`listbox` (picker, `aria-expanded` / `aria-activedescendant`), `button` (start/stop, `aria-label` on icon-only), `textbox` (seed field, associated label)
  - **Required focus order:** focus reaches picker input (visible `--color-focus` ring) → Arrow-navigate options with type-ahead → Enter selects → focus to start control → activate; active picker item carries `--border-emphasis` so focus is never color-only
  - **Required WCAG SC coverage per tier (Minimal):** SC 2.1.1 Keyboard / SC 2.4.3 Focus Order / SC 1.4.3 Contrast (Minimum) (baseline); plus SC 4.1.2 Name/Role/Value (icon-only `aria-label`) and SC 1.4.1 Use of Color (disabled options not color-alone) inherent to the named patterns
  - **Source:** creator brief excerpt's Must-Work Scenarios ("Control panel (minimal UI) — scenario/suite picker, start/stop…"); arch excerpt's Project Intent critical paths hint; design excerpt's ARIA-Relevant Component Patterns (picker)

- **Path:** Operator-pause go/no-go (HOLD) → Proceed or Abort
  - **Surfaces involved:** desktop-webview
  - **Required ARIA roles:** `alertdialog` (`aria-modal`, focus trap), `button` (Proceed / Abort), live region `aria-live="assertive"` on the phase-line HOLD flip
  - **Required focus order:** focus moves into the dialog trap on open (over dimmed/inert console) → Tab/Shift+Tab cycle within trap across Proceed/Abort → Enter/Space activates → Escape per convention → focus restored on close (Radix default); Proceed disabled/in-flight during async committed step (announced via aria-disabled)
  - **Required WCAG SC coverage per tier (Minimal):** SC 2.1.1 Keyboard / SC 2.4.3 Focus Order / SC 1.4.3 Contrast (baseline); plus SC 2.1.2 No Keyboard Trap (trap must be escapable), SC 4.1.2 Name/Role/Value, and SC 2.3.3 Animation from Interactions (dialog fade + count tint dropped under reduced-motion) inherent to the named patterns
  - **Source:** creator brief excerpt's Must-Work Scenarios ("operator pauses … pause with an explicit instruction and resume on confirmation"); design excerpt's User-Facing Error Surfaces + ARIA-Relevant Component Patterns (operator-pause dialog); layout excerpt's Focus Management Anchors (run-console-HOLD)

- **Path:** View run report — verdict/state outcomes + emission journal cross-reference (idle → live → report-terminal states)
  - **Surfaces involved:** desktop-webview
  - **Required ARIA roles:** list/listitem (or table) for the P-ID coverage matrix (`aria-selected` / `aria-current` + `--border-emphasis` on selected row), `status` / `aria-live` for verdict lamp + live emission counters; real-prose empty/in-progress states
  - **Required focus order:** keyboard-first row navigation across the coverage matrix; off-screen virtual-scrolled rows remain reachable/announceable; live count updates announced via `aria-live` without stealing focus; empty/in-progress prose announced (not as a Fail)
  - **Required WCAG SC coverage per tier (Minimal):** SC 2.1.1 Keyboard / SC 2.4.3 Focus Order / SC 1.4.3 Contrast (baseline); plus SC 1.4.1 Use of Color (six verdict/state treatments each text-paired, not color-alone), SC 1.4.11 Non-text Contrast (status lamps 3:1), and SC 4.1.3 Status Messages (aria-live verdict/count) inherent to the named patterns
  - **Source:** creator brief excerpt's Must-Work Scenarios ("Run report surface" + live counters/target status); design excerpt's ARIA-Relevant Component Patterns (lamp, coverage matrix) + Loading/Error/Empty State Patterns; arch excerpt's Project Intent critical paths hint (run-report view, live emission counters via Tauri `Channel`)

- **Path:** Operator-checklist (`ManualCheck`) — tick yes/no items for claims with no programmatic read-back
  - **Surfaces involved:** desktop-webview
  - **Required ARIA roles:** `checkbox` per item within a group/region container (`aria-checked` per row), unticked-count roll-up announced; each item exposes induced-state + expected-observation text; `ManualCheck` lamp links to this checklist
  - **Required focus order:** keyboard-first; Space toggles a focused row; unticked-count footer roll-up reachable so an incomplete pass is not mistaken for a finished run
  - **Required WCAG SC coverage per tier (Minimal):** SC 2.1.1 Keyboard / SC 2.4.3 Focus Order / SC 1.4.3 Contrast (baseline); plus SC 1.4.1 Use of Color (neutral-lavender checkbox glyph paired with text), SC 4.1.2 Name/Role/Value, and SC 4.1.3 Status Messages (unticked-count roll-up) inherent to the named patterns
  - **Source:** creator brief excerpt's Must-Work Scenarios ("manual-check items render as an operator checklist … with induced state + expected observation"); design excerpt's User-Facing Error Surfaces + ARIA-Relevant Component Patterns (operator-checklist)

(Note: tests excerpt's Critical Paths are predominantly headless `conductor run` / coverage-matrix-gate / both-surface-parity flows with no rendered-UI surface — not-assertable, excluded. Both-surface-parity's a11y-relevant half (the GUI-launched run rendering the same accessible envelope) is covered within the three desktop-webview paths above; tests excerpt's Coverage Trigger "Cross-surface coordination" names axe-core/pa11y on the desktop-webview confirming this.)

## 5. A11y Triggers

- **Trigger type:** visual-discrimination
  - **Source:** design excerpt's A11y-Relevant Design Tokens — State color tokens (six color-coded states: `--count-nominal` success, `--count-hold` warning, `--status-fail` error, `--count-blocked` info/blocked, `--status-manual` info/manual, `--status-residual` info/residual) + Verdict/report-state lamp with "six distinct visual+text treatments" (design excerpt's ARIA-Relevant Component Patterns)
  - **Required assertion:** axe-core color-contrast rule across the named foreground/background token pairs in Section 3 + machine-verifiable not-color-alone assertion that every state carries its text label (`Pass`/`HOLD`/`Fail`/`Blocked`/`Manual`/`Residual`) + glyph supplement (SC 1.4.1 Use of Color, SC 1.4.3 Contrast, SC 1.4.11 Non-text Contrast). This is a design-token-driven assertion, not a creator/compliance directive, so it does NOT escalate tier above Minimal.

- **Trigger type:** motion-sensitive (token-driven only; no creator vestibular ask)
  - **Source:** design excerpt's A11y-Relevant Design Tokens — Motion/transition tokens (`--motion-micro`, `--ease-quiet`) + global reduced-motion-preference rule "drops all animations/transitions (binding with a11y SC 2.3.3, including the held-count tint transition)"; design's Paused-count hold-point ("all accompanying color transitions are dropped under reduced-motion preference")
  - **Required assertion:** verify that under `prefers-reduced-motion`, the count tint (green → amber → slate-violet), focus-ring fade-in, status-light color transition, and operator-pause dialog fade are all dropped (SC 2.3.3 Animation from Interactions). Creator brief names no motion-sickness/vestibular requirement (creator brief excerpt's A11y Anti-Patterns = N/A), so this is token-binding only and does not escalate tier.

Cross-input check against expected triggers:
- WCAG compliance regime → wcag-mapping: **none** — security plan excerpt's A11y Compliance Triggers = "No a11y compliance triggers."
- Hardened tier → regulated-compliance: **none** — security_tier = Minimal (passed variable + security plan excerpt's Security Tier).
- Vestibular ask → motion-sensitive: only the token-driven half present (no creator vestibular language); recorded above as token-binding.
- I18n / RTL → multi-language: **none** — no language/RTL tokens in design excerpt; arch excerpt's Project Intent target users = solo developer, no international population.
- Explicit a11y scenario → screen-reader-priority / keyboard-only / cognitive: **none explicit** — creator brief excerpt's Rigor Hints: "No explicit WCAG target named," no blind/low-vision/cognitive user population; arch excerpt's Project Intent: "No a11y-priority user signals present." (Keyboard-first design exists structurally but is not a creator priority ask.)
- Multi-platform touch → target-size: **none** — arch excerpt's Surfaces: "No mobile surface — Desktop-only, host-bound"; design excerpt's Target size tokens: no dedicated tap-target tokens (desktop pointer + keyboard, no touch surface).
- Many state colors → visual-discrimination: **yes** (recorded above).

## 6. A11y Tier

**Tier: Minimal (0)**

**Justification:** One assertable UI surface (desktop-webview Tauri control panel; the cli surface is not-assertable and the project has no web/HTTP, mobile, or native-toolkit surface per arch excerpt's Surfaces) with four must-be-accessible paths (scenario pick/start, operator-pause go/no-go, run-report view, operator-checklist) — at the lower band of the Minimal definition's 2-5 path range. Security tier is Minimal (passed variable, corroborated by security plan excerpt: single-developer local-only no-cloud no-auth utility), and both tests tier (tests excerpt's Tests Tier: Minimal at the upper boundary) and obs tier (obs excerpt's Obs Tier: Minimal) are Minimal — a11y matches its upstream tiers with no ±1 divergence. There are zero WCAG compliance triggers (security plan excerpt's A11y Compliance Triggers) and no creator-set WCAG mandate (creator brief excerpt's Rigor Hints: "no creator-set WCAG mandate to honor"; target user is a single solo developer-operator with no a11y-priority population). The only triggers detected are token-driven (visual-discrimination from six state-color tokens, motion-sensitive from reduced-motion binding) which mandate specific assertions but do not escalate the tier. Net WCAG reach: SC 2.1.1 Keyboard + SC 1.4.3 Contrast (Minimum) + SC 2.4.3 Focus Order baseline, augmented by the design-token-bound not-color-alone (SC 1.4.1), non-text-contrast (SC 1.4.11), and reduced-motion (SC 2.3.3) assertions the design system already commits to — all agent-runnable via axe-core/Lighthouse against the webview DOM, with manual NVDA/VoiceOver/Orca supplemental only.

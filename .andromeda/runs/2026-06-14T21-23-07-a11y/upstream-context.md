# Upstream Context — Conductor A11y (distilled)

_Phase 0 mapper-reduce composite. Seven a11y-relevant excerpts distilled from architecture.md, security-plan.md, design-system.md, layout-templates.md, test-plan.md, obs-plan.md, and the creator brief (input.md). Phases 1–7 read THIS file instead of the full upstreams. Design + layouts excerpts scrubbed of raw token values per `[NO_RAW_DESIGN_VALUES]` — token NAMES are the cross-skill anchor; values stay in design-system.md._

---

## 1. Architecture Excerpt

### Stack (a11y reach)
- **Tauri 2 (desktop shell, bundler v2.10.x)** — optional GUI control-panel artifact rendering over a system webview; as a web-tech desktop surface its a11y testing reaches axe-core via the DevTools/CDP protocol against the embedded webview, and ARIA patterns derive from the control-panel's HTML/DOM (start/stop, scenario picker, run-report view, operator-pause prompt).
- **Tauri 2 IPC `Channel` (push / real-time)** — streams live emission counters / target status backend→frontend into the webview UI; drives the live-region / status-update a11y pattern (no native OS toasts by design, so no OS-level notification a11y surface).
- **conductor-cli (`agent-run` headless binary)** — the headless/CLI source-of-truth release-gate path with no GUI; a11y reach here is manual keyboard/terminal discipline only (no axe/Lighthouse/pa11y applicability).
- **Testing frameworks (cargo-nextest / cargo test, golden tests)** — Rust unit/golden test runners only; no Playwright/Cypress/Selenium or equivalent browser E2E driver exists in the stack for a11y harness reuse.

### Surfaces
**Product type:** Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO (programmatically via MCP read-back where one exists, via an operator checklist for visual claims).

(Surfaces present: Tauri 2 desktop webview GUI [optional, convenience] + headless CLI/`agent-run` [source of truth, release gate]. No mobile surface — "Desktop-only, host-bound." No web/HTTP surface — Conductor exposes no HTTP/network service of its own.)

### Project Intent Summary
- **Core functionality:** "Desktop control-panel app (Tauri 2) over a headless-drivable Rust core — a scenario-driven OTLP fault-injection and verification harness that drives a live Pulse instance through its 60 claimed capabilities (P-001..P-060) on a deterministic seeded timeline and verifies each reaction within its SLO".
- **Target users:** "Personal — solo developer, local dev host, no cloud, no multi-tenancy; runs next to a real Pulse instance on the dev host." (No a11y-priority user signals present — no elderly / low-vision / cognitive-disability / international / assistive-tech user populations named; single solo-developer operator.)
- **Critical paths hint:** Operator/GUI-facing flows enumerated in arch (Tauri commands / interface surfaces):
  - start/stop a scenario or suite run
  - scenario/suite picker action
  - run-report view
  - operator-pause prompt (operator-checklist path for visual/ManualCheck claims)
  - live emission counters / target status streaming display (Tauri `Channel`)

### CI/CD Platform
- **Platform:** GitHub Actions
- **Pipeline note:** Runs `cargo build` + cargo-nextest/`cargo test` (incl. golden tests) + `cargo clippy` on the dev OS target — build + test gating only; dynamic end-to-end scenario proof requires a live Pulse and is an operator/local gate, not a CI gate.

### A11y-Relevant Conventions
(No a11y-specific conventions in arch — Phase 3 will derive defaults from design + WCAG.)

---

## 2. Security Plan Excerpt

### Security Tier
- **Tier:** Minimal
- **Justification:** "Every signal points to a minimal-tier local utility — a single-developer, local-only, no-cloud, no-multi-tenancy tool with no user accounts (auth 'none'), where the only persisted data is self-generated synthetic test telemetry and run-metadata in an embedded SQLite index (`runs.db`) plus on-disk JSONL/Markdown artifacts (no PII/payment/health/credentials owned by Conductor), and there is zero network exposure."

### Anti-Patterns Rejected (a11y-relevant)
(No a11y-relevant anti-patterns in security plan — Phase 3 will apply default a11y discipline to all auth / verification UI.)

### A11y Compliance Triggers
(No a11y compliance triggers in security plan — Phase 1 will derive a11y tier from creator brief + project intent + surface count.)

---

## 3. Design System Excerpt

### Surfaces

- **desktop-webview** (Tauri 2 bundled webview / React SPA) — A single-station, desktop-bound "mission-control console" (Windows / macOS / Linux) built on React 19 + Tailwind CSS v4.1 + shadcn/ui (Radix Primitives) inside a Tauri 2 frameless window; no responsive breakpoints. a11y testing tool reach: axe-core + Lighthouse + screen reader (NVDA / VoiceOver) against the webview DOM.
- **cli** (headless terminal application) — A line-oriented `conductor-cli` (clap + owo-colors + indicatif + comfy-table + inquire) that is the source-of-truth + release gate; ANSI 256-color, TTY-gated, pipe-friendly, no full-TUI. a11y testing tool reach: manual keyboard discipline + no automated tools (relies on ASCII status prefixes, NO_COLOR support, and screen-reader-friendly text-paired output rather than ARIA).

### Loading / Error / Empty State Patterns

- **In-flight / disabled button state (operator-pause Proceed)** — visibility: inline disabled control (no spinner) — a11y impact: button enters disabled in-flight state during the async committed step to prevent double-commit; disabled state must not rely on color alone and must be programmatically conveyed (aria-disabled / disabled attribute) so it is announced.
- **Coverage-matrix empty state** — visibility: page/region inline prose ("No scenarios loaded"), not a gray placeholder — a11y impact: real text content (not a decorative skeleton) so screen readers announce the empty condition.
- **Run-report empty state (no run started)** — visibility: inline prose ("No run yet — pick a scenario/suite to begin") — a11y impact: distinct, readable "no result yet" message; must never be conveyed as a failure/error state.
- **Run-report in-progress state (run not finished)** — visibility: inline prose ("Run in progress") — a11y impact: a distinct "no result yet" status that must not be downgraded to or announced as a Fail; live progress lives in the titlebar count and matrix with aria-live updates.
- **Paused-count hold-point (signature state transition)** — visibility: in-place frozen count + phase-line swap to "HOLD — operator pause" — a11y impact: the phase-line flip is announced via `aria-live="assertive"`; the count value freezes (does not blank or keep running); all accompanying color transitions are dropped under reduced-motion preference.
- **CLI live counter / spinner** — visibility: inline `indicatif` spinner/progress with colored phase-line prefix that STOPS (not hides) at the hold — a11y impact: status is always paired with an ASCII text prefix; ANSI stripped when piped / under NO_COLOR / TERM=dumb so non-visual consumers get text.

### User-Facing Error Surfaces

- **Operator-pause go/no-go dialog (Proceed / Abort)** (modal) — appears for: gating every committed timeline step (including Abort path); a11y requirements: focus is trapped inside the dialog with a visible focus ring; ARIA role `alertdialog`; both Proceed and Abort recovery actions are keyboard reachable: yes.
- **Verdict / report-state lamp — `Fail`** (in-page, on the P-ID matrix row) (in-page) — appears for: a machine `Verdict::Fail`; a11y requirements: resolves in place and motionless (no flashing/pulsing); paired with a text label (`Fail`) so it is not color-only; announced via `aria-live`; ARIA role: status/live-region announcement (not an alert/dialog — held muted per "no alarm").
- **CLI error output** (dedicated stderr stream) (in-page / terminal) — appears for: runtime errors; a11y requirements: text-only `error: <short>` + contextual detail + `hint: <fix>`, never colorized when piped, sanitized (no stack traces except under `--debug`/`-v`); recovery affordance (hint text) is plain readable text.
- **Operator-checklist (`ManualCheck` render)** (in-page card rows) — appears for: claims with no programmatic read-back (operator must confirm a past visual observation, e.g., "halo shifted toward burgundy? ☐"); a11y requirements: keyboard-first (Space toggles a row); each item is a yes/no the operator ticks; an unticked-count footer roll-up prevents an incomplete pass being mistaken for a finished run; not color-only (neutral-lavender checkbox glyph paired with text).

### A11y-Relevant Design Tokens

#### Color tokens (foreground / background pairs)

- **`--text-primary` / `--color-base`** — context: phase-line headline / primary labels on the console base surface (body/heading text — serves SC 1.4.3 4.5:1 normal-text minimum).
- **`--text-secondary` / `--color-raised-1`** — context: run-report body prose and descriptions on the raised card surface.
- **`--text-tertiary` / `--color-base`** — context: metadata, timestamps, captions, and empty-state prose (small/muted text).
- **`--text-muted` / `--color-inset`** — context: disabled labels and placeholders in input fields (non-color-alone disabled signal still required).
- **`--color-id-cyan` / `--color-base`** — context: the reserved monospace status-tier text (count / P-IDs / run_id / SLO timings / fingerprints) on the console surface.
- **`--count-nominal` / `--color-base`** — context: nominal-green status text / `Pass` verdict text on the console surface (non-text status indicator — serves SC 1.4.11 3:1 minimum).
- **`--count-hold` / `--color-base`** — context: hold-amber `CalibrationRegion` / HOLD status text on the console surface.
- **`--status-fail` / `--color-base`** — context: `Fail` verdict text / lamp on the console surface.
- **`--color-focus` / `--color-base`** — context: focus indicator ring against the console surface (non-text focus indicator — serves SC 1.4.11 3:1 minimum).

#### Focus ring tokens

- **`--color-focus`:** focus indicator color token (the single focus-ring color; rendered as an inset ring, composition specified in design).
- **`--border-emphasis`:** active/selected-item border-emphasis token used to mark the focused/selected coverage-matrix row and active picker item (selection indicator distinct from the focus ring color).

#### Target size tokens

(No dedicated minimum-target-size tokens in design-system — interactive sizing is expressed through the icon size grid and spacing/radius scale rather than named tap-target tokens. The icon-size grid token and `--space-sm` container-padding token bound titlebar controls, status lamps, and picker/dialog actions; Phase 3 will derive explicit target-size assertions from WCAG defaults — serves SC 2.5.5 44×44 and SC 2.5.8 24×24.)

#### Motion / transition tokens

- **`--motion-micro`:** micro-interaction / state-transition duration token (control hover-background lift, focus-ring fade-in, in-place status-light color transition, and the count tint green → amber → slate-violet at the hold).
- **`--ease-quiet`:** primary easing-curve token (quiet, no overshoot/spring; value in design).
- Panel/dialog transition: the operator-pause dialog fade uses an inline transition duration (no separate named token in the `@theme` block).
- Reduced-motion override: no dedicated override *token*; a global reduced-motion-preference rule drops all animations/transitions (binding with a11y SC 2.3.3, including the held-count tint transition).

#### State color tokens (error / warning / success / info)

- **`--count-nominal`:** state — success (`Pass` / on-timeline); not-color-alone supplement: text label (`Pass`) + filled status-lamp dot glyph.
- **`--count-hold`:** state — warning (`CalibrationRegion` / operator-pause HOLD); not-color-alone supplement: text label (`HOLD` / `CalibrationRegion`) + amber filled lamp + phase-line text swap.
- **`--status-fail`:** state — error (`Fail`); not-color-alone supplement: text label (`Fail`) + filled lamp; held motionless (no flash).
- **`--count-blocked`:** state — info/blocked (`ReportState::Blocked`, never measured); not-color-alone supplement: text label (`Blocked`) + hollow-ring glyph + named precondition string.
- **`--status-manual`:** state — info/manual (`ReportState::ManualCheck`, awaiting operator); not-color-alone supplement: text label (`Manual`) + checkbox glyph (☐ / ☑), deliberately outside the verdict triad.
- **`--status-residual`:** state — info/residual (`ReportState::KnownResidual`, pre-accepted gap); not-color-alone supplement: text label (`Residual`) + dashed-ring dot glyph + "expected until {fix}" note.

#### Typography tokens (readability)

- **`--font-mono`:** monospace status/code font token (count, P-IDs, run_id, SLO timings, fingerprints, JSONL journal block; font-stack value in design).
- **`--font-sans`:** body/UI prose and display-heading font token (phase line, labels, run-report prose; font-stack value in design).

(Typography roles, weights, sizes, line-heights, and tracking for Display / Heading / Body / Label / Code / Data are defined as table rows in design-system, not as named CSS custom properties; only the two font-family tokens above are emitted as `@theme` tokens. Line-height / letter-spacing / word-spacing readability values stay in design — serves SC 1.4.8 Visual Presentation (line-height / letter-spacing / word-spacing minima) and SC 1.4.12 Text Spacing.)

### ARIA-Relevant Component Patterns

- **Operator-pause go/no-go dialog** — ARIA role: alertdialog; key states / props: aria-modal (focus trap), visible focus ring, Proceed disabled/in-flight state during async step; keyboard contract: Tab/Shift+Tab cycle within trap, Enter/Space activate Proceed or Abort, Escape per dialog convention.
- **Button (titlebar min/close, start/stop, Proceed/Abort)** — ARIA role: button; key states / props: `aria-label` on every icon-only control, disabled/aria-disabled for the in-flight and unavailable states; keyboard contract: Enter / Space activate.
- **Verdict / report-state lamp** — ARIA role: status (live-region announcement), or img/text with accessible name; key states / props: paired text label always present (not color-only), `aria-live` announcement on state change; six distinct visual+text treatments (`Pass` / `CalibrationRegion` / `Fail` / `Blocked` / `Manual` / `Residual`); keyboard contract: non-interactive indicator (no key contract) except the `ManualCheck` lamp which links to the operator-checklist.
- **Coverage matrix (single-row-per-P-ID list)** — ARIA role: list/listitem (or table semantics); key states / props: selected row marked (aria-selected / aria-current) with `--border-emphasis`, hover lift; empty state is real prose; virtual-scrolled — off-screen rows must remain reachable/announceable; keyboard contract: keyboard-first row navigation.
- **Operator-checklist (`ManualCheck`)** — ARIA role: checkbox per item (group/region container); key states / props: aria-checked toggled per row, unticked-count roll-up announced, each item exposes the induced state + expected-observation text; keyboard contract: Space toggles a row.
- **Scenario / suite picker + start/stop** — ARIA role: combobox/listbox (shadcn/ui `Command`/`Select` over Radix); key states / props: aria-expanded / aria-activedescendant for the dropdown, disabled options conveyed without color alone, `--color-focus` ring on focused input; keyboard contract: Arrow keys navigate options, Enter selects, Escape closes, type-ahead filtering, first-class start/stop shortcuts.
- **Form input (scenario / seed fields)** — ARIA role: textbox; key states / props: associated label, `--color-focus` ring on focus, error/hint association via aria-describedby; keyboard contract: standard text-entry.

---

## 4. Layout Templates Excerpt

### Layout Types per Surface

- **desktop-webview:** run-console-idle, run-console-live, run-console-HOLD, run-report-terminal (four run-states of one frameless window, not four routes — no router, no breakpoints)
- **cli:** conductor-run, conductor-suite, conductor-report (plus headless `scripts/agent-run.sh` path; line-oriented, ratatui omitted)

### Error Boundary Placement

(No explicit error boundary placement in layouts — Phase 3 will recommend defaults per layout category, typically section-level for dashboards / page-root for forms with focus-on-error pattern.)

Note: cli surface documents sanitized error output to stderr (`error: <short>` + `hint: <fix>`, no stack traces unless `--debug`), but this is output-stream discipline, not a component error-boundary placement.

### Focus Management Anchors

- **run-console-HOLD (operator-pause go/no-go dialog):** Modal focus trap — shadcn AlertDialog (Radix) traps focus within the dialog region over a dimmed/inert console; visible `color-focus` ring on the trapped region; Proceed/Abort are the in-trap interactive targets.
- **run-console-HOLD (dialog):** Focus restoration implied on close (standard Radix AlertDialog behavior) — not explicitly stated in layout text.
- **desktop-webview (global navigation):** Keyboard-first — focus ring (`color-focus`) visible on every interactive control; active picker item carries `border-emphasis` edge so focus is never color-only; keybindings (start/stop/proceed/abort) are first-class via Radix primitives; selecting a control by keyboard moves visible focus and that position is communicated to assistive tech.
- **desktop-webview (window controls):** Icon-only minimize/close carry derived accessible names.
- **cli (operator-pause):** `inquire` confirm gated behind `isatty`; headless/non-TTY path skips the prompt entirely (never blocks) — no focus-trap concept on cli.
- (Initial-focus-on-mount, skip links, and route-change focus are NOT documented — single-station console has no routes; Phase 3 will derive WCAG SC 2.4.3 + SC 3.2.1 defaults for initial focus and route/state-change focus.)

### Heading Hierarchy Anchors

(No explicit h1/h2/h3 numeric hierarchy in layouts — typography is referenced by role name only.)

Role-level and landmark-equivalent signals present:

- **run-console-idle / -live / -HOLD / -report (desktop-webview):** phase line carries the **Heading** role (named operational segment); window top is a frameless titlebar (`data-tauri-drag-region`); footer is a single-line status strip (**Label** role); landmark roles are NOT explicitly assigned — Phase 3 to map main / banner / contentinfo equivalents.
- **run-console-HOLD (go/no-go dialog):** shadcn AlertDialog provides **alertdialog** semantics; dialog header (Display role) carries frozen-count + step index; body is **Body** role.
- **cli surfaces:** non-DOM (line-oriented stdout) — heading hierarchy / landmark roles N/A; section title rendered as bold colored header text, always paired with ASCII bracket prefixes (`[PASS]`/`[HOLD]`/`[FAIL]`/`[BLOCKED]`) for NO_COLOR/screen-reader survival.
- (Sequential h1/h2/h3 nesting and main/navigation/contentinfo landmark assignment not specified — Phase 3 will recommend WCAG SC 1.3.1 + SC 2.4.6 defaults: single h1 per surface, sequential h2/h3 nesting, main + contentinfo landmarks for the desktop-webview window.)

---

## 5. Test Plan Excerpt

### Tests Tier
- **Tier:** Minimal
- **Justification:** "this is Minimal at the upper boundary: 8 workspace crates plus several capability surfaces (>5 entities), 7 critical paths, and a Creator-Brief mandate for 'production-grade verification rigor' with a determinism hard-bar and an assertion-policy split — so the Minimal baseline is augmented with the Section 5 coverage triggers ... without escalating to Comprehensive."

### Test Harness Contract Summary
- **5-command names:** boot (preflight readiness gate — no daemon to start), run, status, cleanup, logs
- **Status JSON shape:**
```json
{
  "run_id": "string", "seed": 0, "scenario": "string", "p_ids": ["P-001"],
  "verdict": "Pass | Fail | CalibrationRegion",
  "state": "Pass | Fail | ManualCheck | KnownResidual | Blocked",
  "latency_ms": 0, "slo_tier": "string",
  "journal_emitted_at": "ISO-8601", "read_back_observed_at": "ISO-8601",
  "fingerprints": []
}
```
(No HTTP/IPC status endpoint exists — no inbound listener. This is the Run-report envelope serialized identically into the Markdown report, the `runs.db` row, and each JSONL journal line. Artifacts MUST NOT leak absolute host paths or internal seam-crate struct names.)

### Critical Paths (must-be-accessible)
- **Headless deterministic scenario run with MCP read-back (`error-baseline-spike`):** After `conductor run error-baseline-spike --seed <s>`, exit 0 AND `runs.db` row has `verdict=Pass` + `state=Pass` with `latency_ms` within `slo_tier` AND the per-run JSONL journal is written AND MCP read-back confirms Pulse's reaction; same seed ⇒ same stream shape on re-run.
- **Fingerprint-storm (fault-injection burst + read-back):** `conductor run fingerprint-storm` exits 0 with per-P-ID `verdict`/`state` rows recorded; `fingerprints` field populated in the envelope; read-back confirms Pulse fingerprint reaction within SLO.
- **Restart-suppression incl. one bypass case:** `conductor run restart-suppression` produces a hard pass/fail on suppression logic AND the bypass case reports its expected distinct outcome; journal + `runs.db` row assert the deterministic stream.
- **Severity-lifecycle full pass (auto-resolve + resolution summary):** One full `severity-lifecycle` pass observes Pulse auto-resolve and a resolution summary via MCP read-back; lifecycle timing asserted as hard pass/fail, severity choice asserted as CalibrationRegion (report-for-human, not hard-failed).
- **Known-residual classification path (P-032):** `conductor run` for P-032 produces `state=KnownResidual` (NOT `Fail`); a `degraded_mode` read-back result maps to `KnownResidual`; run report distinguishes it from pass/fail/manual-check.
- **Coverage-matrix completeness gate:** The generated `coverage-matrix.md` enumerates all 60 P-IDs with zero unclassified entries; a missing P-XXX fails the gate ("A P-XXX missing from the matrix is a defect"); this is the "0.1.0 done" definition.
- **Both-surface parity (control panel AND headless):** A scenario launched via the Tauri start command yields the same `runs.db` envelope (verdict/state/seed) as the headless `conductor run` for the same scenario+seed; emission journal written per run from both paths.

### Coverage Triggers Summary
- **Security-vector CLI input (Vector 1)** (compliance / negative-test) — a11y implication: scripted verification that path-traversal `CONDUCTOR_*` handles are rejected before any artifact write; no a11y-surface-specific check (CLI-layer).
- **Config-file parsing garde validation (Vector 2)** (compliance / property-test) — a11y implication: ensure config-validation error states surface through accessible error output (no color-only signaling of invalid-config rejection).
- **MCP STDIO injection / preflight downgrade (Vector 4)** (compliance / negative-test) — a11y implication: verify `blocked` preflight state is announced via text/`aria-live`, never a silent or color-only downgrade.
- **Tauri capabilities deny-by-default (Vector 3)** (compliance / negative-test) — a11y implication: confirm only the deny-by-default command set + live-counter Channel reach the webview surface under test (no remote-origin iframe affecting a11y selectors).
- **SQL bound-parameter `runs.db` access** (compliance / negative-test) — a11y implication: none (storage-layer, no accessible surface).
- **Pinned MCP contract manifest** (compliance / contract-test) — a11y implication: verify a contract-mismatch `blocked` outcome renders with a text-paired status label, not color alone.
- **Determinism replay (`conductor-timeline`)** (cognitive / property-test) — a11y implication: deterministic stream shape supports reliable scripted keyboard/selector verification across re-runs (stable status labels).
- **Cross-surface coordination (control panel + headless parity)** (multi-platform / cross-surface) — a11y implication: axe-core / pa11y run on the desktop-webview surface confirming the GUI-launched run renders the same accessible envelope as headless.
- **Bounded fault-injection (P-060 "typical/high")** (chaos / fault-injection) — a11y implication: focus retention and `aria-live` HOLD/verdict announcements survive silence/ramp/port-occupier fault states (bounded, not saturation).
- **Supply-chain audit (`cargo-audit` + `cargo-deny`)** (compliance / supply-chain) — a11y implication: none (build-gate, no accessible surface).

### Quality Gates Summary
- **Zero-flakiness statement:** "flaky tests are NOT tolerated. If a test flakes once, it gets quarantined immediately and fixed (root cause — not retry-once budget). Do NOT set cargo-nextest `retries` > 0."
- **Coverage thresholds:** Minimal tier (selected) — line ≥ 60%, branch ≥ 50%, function ≥ 70%; enforced via cargo-llvm-cov `--fail-under-lines 60` (line gate is the binding CI gate; branch coverage is informational on the stable toolchain).

---

## 6. Obs Plan Excerpt

### Obs Tier
- **Tier:** Minimal
- **Justification:** "8 workspace crates + 2 surfaces (CLI headless + Tauri desktop-webview GUI) places Conductor at the lower end of the module-count spectrum."

### Log Format JSON Schema (binding)
Verbatim from obs Section 6 (Log Coverage) — binding contract from tests excerpt §5:

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

### Service Identity
- **service.name:** Hardcoded `"conductor"` (CLI) or `"conductor-tauri"` (Tauri GUI) or `"conductor-ui"` (browser frontend); overrideable via `$CONDUCTOR_SERVICE_NAME` env var at runtime.
- **service.version:** Compile-time `env!("CARGO_PKG_VERSION")` from root `Cargo.toml`.
- **deployment.environment:** Runtime `std::env::var("CONDUCTOR_ENV").unwrap_or_else(|_| "local".into())` (fallback default `"local"`).

### Sentry User-Feedback Widget (if applicable)
- **Widget pick:** N/A — no user-feedback widget in obs plan. Obs plan has no Section 7 (Error Capture & Reporting); error capture is `std::panic::set_hook()` + `tracing::error!(...)` + `anyhow` binary-edge bridging only. External error-reporting platforms (Sentry / Bugsnag / Rollbar) are explicitly rejected per Inherited Defaults ("no external error-tracking platform").
- **Trigger surface:** N/A
- **Default state:** N/A

### Focus-Relevant Span Coverage (filtered)
(No focus-relevant spans in obs plan Section 4 — a11y Phase 3 will recommend focus tracing spans that obs may add later: focus.shift / focus.trap.enter / focus.trap.exit / focus.restore.)

Section 4 enumerates only backend/scenario spans (`scenario.run`, `timeline.execute*`, `emit.batch`, `verify.readback*`, `report.generate`, `db.insert_run`, `db.query_all_p_ids`, `tauri.command.start_scenario`, `fault.silence` / `fault.ramp` / `fault.port_occupier`); none correlate with focus management, keyboard navigation, modal open/close, or route changes.

---

## 7. Creator Brief Excerpt

### Must-Work Scenarios
UI-facing flows the creator names as critical (become "must-be-accessible" priorities for Phase 1 alongside tests' critical paths). The only accessible (rendered-UI) surface is the Tauri 2 desktop-webview control panel; everything else is headless CLI / journal artifacts.

- **Control panel (minimal UI)** — verbatim: "Control panel (minimal UI) — scenario/suite picker, start/stop, live emission counters + target status, the run report view, operator-pause prompts. A control surface, not a dashboard."
- **Run report surface** — verbatim: "run report: per scenario — pass / fail / manual-check / known-residual ... with the emission journal cross-referenced; manual-check items render as an operator checklist ('halo shifted toward burgundy?', 'no OS toast appeared?') with induced state + expected observation".
- **Operator pauses** — verbatim: "operator pauses: scenarios needing actions Conductor must not perform itself (restart Pulse mid-baseline ...; config edits ...; acknowledge an incident ...; model on/off ...) pause with an explicit instruction and resume on confirmation."
- **Both-surface parity (control panel AND headless)** — verbatim: "Every catalog scenario runs from the control panel AND headless; deterministic under a fixed seed; emission journal written per run." and "Run report distinguishes pass / fail / manual-check / known-residual; manual checklist renders with induced-state context".

### Rigor Hints
No explicit WCAG target named by the creator. The brief frames the UI as deliberately minimal and operator-facing; the target user is the solo developer-operator. Signals (verbatim):

- "**Control panel (minimal UI)** ... A control surface, not a dashboard."
- "**Target user:** The Pulse developer (solo, local). Runs next to a real Pulse instance on the dev host."
- "Development Style: **agent-driven** (headless-drivable core + `scripts/agent-run.sh`; the UI is a thin shell over the same commands)."

Interpretation signal (not a creator directive): no informal-hygiene / WCAG 2.1 AA / WCAG 2.2 AAA + cognitive language appears anywhere in the brief — Phase 1 derives the a11y tier from project intent + single surface + solo-operator audience + Minimal security/tests/obs tiers, with no creator-set WCAG mandate to honor.

### A11y Anti-Patterns (creator's explicit asks)
N/A — the creator brief names no a11y anti-patterns (no "no inaccessible captcha", no "no aria-* without semantic HTML first", no "no manual review as ONLY method", or similar). One adjacent statement exists but is NOT a Conductor-a11y anti-pattern: "NO UI automation of Pulse (Playwright/axe live in Pulse's own suites) — visual claims are operator checklist items." — this scopes out automating the *system-under-test's* (Pulse's) UI and routes Pulse's visual claims to an operator checklist; it does not bear on the accessibility of Conductor's *own* control-panel surface, which remains in a11y scope. The operator-checklist mechanism verifies Pulse behavior, so it is not a "manual-review-as-only-method" a11y verification path for Conductor's UI.

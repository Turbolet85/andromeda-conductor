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

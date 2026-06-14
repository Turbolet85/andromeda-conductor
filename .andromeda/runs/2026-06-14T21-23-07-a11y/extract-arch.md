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

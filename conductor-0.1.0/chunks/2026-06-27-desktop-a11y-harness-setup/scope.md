# Scope — Desktop a11y harness setup

**Marker:** `2026-06-27-desktop-a11y-harness-setup`
**Version:** conductor-0.1.0 · **Epoch 9 (Desktop control panel) · ch9/10**
**Surface title:** Desktop a11y harness setup — axe/Lighthouse/colorjs.io over tauri-driver, shadcn/Radix ARIA binding, token-pair contrast

## What it builds
Stand up the **desktop accessibility + GUI-integration test harness** for the optional Tauri 2 webview — the
tooling and scaffolding that makes the GUI's accessibility assertable — and make it the home for the
GUI-integration tests deferred from three prior Epoch-9 chunks. This is the **SETUP** chunk: it erects the
harness and lands the deferred `tauri::test` mock-runtime + tauri-driver tests; it does **not** author the
exhaustive per-path axe/contrast/keyboard verification sweep (that is the very next chunk, *Desktop a11y
verification*).

### A. The a11y / GUI test harness (new tooling)
- A **tauri-driver** WebDriver harness able to drive the bundled webview for automated checks.
- **axe-core** integration for automated WCAG rule scanning of the rendered UI.
- A **Lighthouse** a11y-category runner, per a11y-plan §3.
- **colorjs.io**-based **token-pair contrast** checking over the design tokens (`ui/src/styles/tokens.css`
  `:root` pairs) — verifying foreground/background pairs meet WCAG AA, reinforcing the "status is never
  color-alone" + contrast discipline.
- shadcn/Radix component ARIA bindings made assertable through the harness.
- npm **devDependencies** added under `crates/conductor-tauri/ui/` (axe / lighthouse / colorjs.io /
  tauri-driver glue) — `npm audit` clean + `package-lock.json` committed (frontend supply-chain gate); any
  Rust-side test deps keep `cargo audit` + `cargo deny` green with committed un-drifted `Cargo.lock`.

### B. The deferred GUI-integration tests (absorbed CARRYs)
The harness is the home for three batches of `tauri::test` mock-runtime + tauri-driver tests deferred because
the background-thread `Channel` stream isn't deterministically assertable in-process (zero-retry bar) and the
prior chunks shipped build-gated only:

1. **From `2026-06-26-live-counter-channel-stream` (Epoch-9 ch4):** `tauri::test` mock-runtime command /
   `Channel`-frame tests for `start_run` / `stop_run`; the hermetic **cross-surface-parity** leg
   (test-plan **Path 7**: Tauri mock-runtime vs CLI subprocess → identical `runs.db` envelope for the same
   seed). Run logic itself is already unit-covered in `conductor-run::drive_run` + the `cli_smoke` parity E2E.
2. **From `2026-06-27-run-report-operator-checklist-views` (Epoch-9 ch7):** `tauri::test` for the read-only
   `run_report` command; tauri-driver axe/contrast/keyboard on `RunReport` (verdict lines; Blocked `—`/null
   **never rendered red**) + `OperatorChecklistView` (checkbox Space-toggle; unticked-count `role=status`).
3. **From `2026-06-27-operator-pause-go-no-go-dialog` (Epoch-9 ch8):** `tauri::test` mock-runtime for
   `resolve_operator_hold` (a stored `oneshot` sender → the delivered `Decision`) + the hold
   `Channel<HoldPrompt>` frame; tauri-driver axe/keyboard on the wired `OperatorPauseDialog` (alertdialog
   role, focus-trap, Escape→NoGo, focus-restore, Proceed/Abort label+color never color-alone, reduced-motion
   fade-drop). The bridge CORE (`HoldGate::arm`/`deliver` decision-capture + abort-default) is already
   unit-covered; only the `Channel`/webview leg lands here.

## Boundaries (NOT in this chunk)
- **Full a11y verification sweep** — axe-violations-zero + contrast-pass + keyboard-trap/focus-order across the
  four accessible paths + the NVDA/VoiceOver manual spec are the NEXT chunk (*Desktop a11y verification*). This
  chunk sets the harness up and proves it runs (smoke-level), landing the deferred integration tests.
- **CI a11y gate** — wiring axe/contrast/keyboard PASS/FAIL into the obs envelope as a CI gate is Epoch 10
  (*A11y CI gate + violation JSON*).
- **No engine/seam model change** — frontend + test-harness + `tauri::test` only; no change to
  `conductor-core`/timeline/emit/faults/verify/report/run engine logic. The mock-runtime / driver tests
  exercise EXISTING commands (`start_run`/`stop_run`/`run_report`/`resolve_operator_hold`) and components, not
  new ones.
- **Live Pulse** — every scenario remains Blocked without a live Pulse; the integration tests assert the
  Blocked-path envelope + UI rendering, not live-verified runs (Epoch 10).

## Surfaces / contracts touched
- `crates/conductor-tauri/ui/` — npm devDeps (axe / lighthouse / colorjs.io / tauri-driver glue),
  `package.json` + `package-lock.json`, a possible test-runner config; `src/styles/tokens.css` is the
  contrast-check input (read-only).
- `crates/conductor-tauri/` — Rust `tauri::test` mock-runtime tests (`[dev-dependencies]` + `#[cfg(test)]`)
  exercising `start_run`/`stop_run`/`run_report`/`resolve_operator_hold` + the two `Channel`s (live-counter +
  `HoldPrompt`).
- test-plan **Path 7** cross-surface parity (Tauri mock-runtime ↔ CLI subprocess, identical `runs.db`
  envelope) — authoritative source test-plan §3 / §Paths.
- **a11y-plan §3** (a11y harness) — authoritative tooling/threshold source (axe rule-set, contrast ratios,
  Lighthouse category).
- Supply-chain gates: `npm audit` (frontend), `cargo audit` + `cargo deny` (Rust test deps), committed
  lockfiles.
- `scripts/agent-run.{sh,ps1}` `run` verb / CI Rust job already build `conductor-tauri` against a fresh
  `ui/dist`; whether the a11y/driver leg joins the headless gate or stays a local-only script is a plan
  decision (see Known risk).

## Known risk (resolve in plan — P4)
The host has **no display** (the handoff repeatedly notes Epoch-9 views were build/type-verified but never
visually smoke-tested). A `tauri-driver` + axe sweep over a *real* webview typically needs a display or a
headless-webview path. The `tauri::test` **mock-runtime** leg (commands + `Channel` frames + Path-7 parity)
is display-free and CI-able; the **tauri-driver** (real-webview axe/contrast/keyboard) leg may be
local-operator-gated or deferred to the verification chunk. The plan must draw this line explicitly and
**never silently skip** a leg — a host-display blocker is surfaced with its reason.

## Acceptance intent (val-1 anchor)
- The a11y/GUI harness tooling is installed + configured (tauri-driver + axe + Lighthouse + colorjs.io
  contrast) and runs at least a smoke check (or its display-gated legs are explicitly, justifiably scoped).
- The three CARRY batches of deferred `tauri::test` (+ tauri-driver where the host allows) tests are landed
  and pass — anything blocked by the no-display host is surfaced with reason, never silently skipped.
- The test-plan **Path 7** hermetic cross-surface-parity leg exists (Tauri mock-runtime ↔ CLI subprocess,
  identical `runs.db` envelope for one seed).
- All gates green: `tsc` · `npm run build` · `npm audit` 0 · `cargo nextest -p conductor-tauri` (+ workspace) ·
  clippy `-D warnings` · `cargo audit` + `cargo deny` · committed un-drifted `Cargo.lock` + `package-lock.json`.
- **Zero engine/seam model change**; the boundary with the verification chunk is respected.

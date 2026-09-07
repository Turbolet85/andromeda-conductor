# Desktop a11y / GUI test harness

The accessibility sweep for the Tauri webview — axe-core + colorjs.io token-pair contrast +
WebdriverIO keyboard/focus over `@crabnebula/tauri-driver` (a11y-plan §3.5, a11y.md §Testing).

## DRIVER-GATED — not display-gated, and not Linux-only

The gate is the availability of a native WebDriver, not a display. Measured 2026-09-01/02: the same
stack drives a real WebView2 session **headfully on the Windows host**, and runs under Linux + xvfb.
**macOS alone lacks a WebDriver** (no WKWebView driver), so it stays manual-pass-only.

The three suite families over that ONE stack differ in what they need:

- **routine** (`npm run a11y`, the `--e2e` arm) — **no live Pulse.** It seeds its own committed fixture
  through the production writer, so a clean tree has a subject. This is the arm wired as a CI job.
  Two of its specs context-skip without a raised hold (the operator-pause dialog and the
  operator-checklist rows); a context-skip is never a pass (test-plan §6).
- **driven** (`npm run a11y:driven`) — needs a live, preflight-ready Pulse and its full firing form;
  operator-local, never a CI gate (test-plan §11).
- **screen-reader** (`npm run a11y:sr` / `a11y:sr-empty` / `a11y:sr-error`) — Windows + NVDA host only,
  with a portable NVDA named by `CONDUCTOR_NVDA` (unset ⇒ skip at exit 0); operator-local.

Both host-tool handles skip their own leg at exit 0 when unset — `CONDUCTOR_MSEDGEDRIVER` for the
native driver, `CONDUCTOR_NVDA` for the screen reader. **`CONDUCTOR_A11Y_STRICT` inverts that for CI**:
with it declared, an unresolvable handle exits NON-ZERO instead, because a gate that can pass by
skipping is banned (a11y-plan §11 Anti-Patterns → CI).

Surfaced, never silently skipped (test-plan §11). The screen-reader pass spec lives at
`test/a11y/screen-reader/nvda-pass-spec.md`; the recorded pass sits under
`conductor-0.2.0/chunks/2026-09-02-screen-reader-manual-spec/evidence/`.

## Run

```bash
cargo build --release -p conductor-tauri --features tauri/custom-protocol   # the bundle tauri-driver attaches to
cd crates/conductor-tauri/ui
npm run a11y                          # routine arm — no live Pulse; needs CONDUCTOR_MSEDGEDRIVER
npm run typecheck:e2e                 # typecheck the specs anywhere (no driver needed)
```

The `--features tauri/custom-protocol` is load-bearing: tauri's `build.rs` computes `dev = !custom_protocol`,
so a bare `--release` binary loads `devUrl` (`localhost:5173`) instead of the embedded bundle.

Note that `typecheck:e2e` does NOT prove the config loads — a `wdio.conf.ts` member is proven only by
EXECUTING the leg that loads it (test-plan §4).

The deterministic, display-free GUI-integration tests (mock-runtime command contracts + the Path-7
cross-surface envelope parity) live in `crates/conductor-tauri/src/{commands,pause}.rs` (`cargo nextest
-p conductor-tauri`).

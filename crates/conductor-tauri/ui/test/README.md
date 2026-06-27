# Desktop a11y / GUI test harness

The accessibility sweep for the Tauri webview — axe-core + colorjs.io token-pair contrast +
WebdriverIO keyboard/focus over `@crabnebula/tauri-driver` (a11y-plan §3.5, a11y.md §Testing).

## DISPLAY-GATED — does not run on every host

This leg runs **only on Linux + xvfb against a live Pulse**. It is **not** part of the always-on gate
(`scripts/agent-run.sh run` / the CI Rust job) and does **not** run on the Windows dev host (no display):

- Every scenario is `Blocked` without a live Pulse, so the operator-pause dialog never fires here.
- `tauri-driver` drives the platform WebView (WebKitGTK) under xvfb; there is no WKWebView/Windows path
  (test-plan §6).

Surfaced, never silently skipped (test-plan §11). The always-on a11y CI gate (violations → obs envelope)
is the Epoch-10 route chunk *A11y CI gate + violation JSON*; the exhaustive per-path verification matrix +
NVDA/VoiceOver manual spec are the next chunk, *Desktop a11y verification*.

## Run

```bash
cargo build --release                 # the bundle tauri-driver attaches to
cd crates/conductor-tauri/ui
npm run a11y                          # Linux + xvfb + live Pulse only
npm run typecheck:e2e                 # typecheck the specs anywhere (no display needed)
```

The deterministic, display-free GUI-integration tests (mock-runtime command contracts + the Path-7
cross-surface envelope parity) live in `crates/conductor-tauri/src/{commands,pause}.rs` (`cargo nextest
-p conductor-tauri`).

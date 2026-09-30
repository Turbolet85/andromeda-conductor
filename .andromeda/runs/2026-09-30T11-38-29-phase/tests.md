# tests extract

## Relevance
partial — the chunk's work is the operator-local screen-reader E2E family (§6 desktop-webview row, third family) plus a cause-isolation experiment; no CI stage, no `agent-run` verb and (unless the remedy lands in Rust) no nextest surface is in play.

## Constraints
- The `sr*` suites (`npm run a11y:sr` / `a11y:sr-empty` / `a11y:sr-error` → `screen-reader.e2e.ts`) are an operator-local leg on a Windows + NVDA host only: never a CI stage, never an `agent-run` verb, never a CI network call. NVDA's own speech log is the driver and the operator's role is REVIEW of the graded record (per test-plan §6 desktop-webview row, "third family"; §11 CI; §11 Universal).
- test-plan §6 fixes the SR leg's firing form: `CONDUCTOR_NVDA` gated with skip-at-exit-0; NVDA started BEFORE tauri-driver and ready on `NVDA initialized` plus a quiet settle; the leg-owned `nvda-config/nvda.ini` copied per session; `activate-window.ps1` (fixed argv) for OS foreground; per-suite runs/scenarios dirs chosen at the one spawn site; the live `sr` subject needs `CONDUCTOR_SCENARIOS_DIR=runs/sr-leg/scenarios`; stop form `nvda -q` → `tasklist` census → `tauriDriver.kill()`. A remedy in "the harness's launch or foreground path" changes this documented form, so the form stays the reference the change is measured against (per test-plan §6 desktop-webview row). Whether the shipped harness still matches this form is research's question.
- Browse-mode rows (E0-09's class) are recorded findings pending OS-level key injection — never passes, never a manual arm, never a hard fail; the `focus` / `live` rows are the agent-driven ones (per test-plan §1 Untestable zones; §6 desktop-webview row).
- The WebView2 × msedgedriver working set is a set of PAIRS, never either literal alone; the measured passing pairs are 152.0.4191.53 × 151.0.4129.101 and 152.0.4191.x × 152.0.4191.53, and refreshing the driver after a runtime update is the operator's host task BEFORE the next leg, never inside a wrap (per test-plan §6 desktop-webview row; §9 Matrix builds). Whether the pass record carries the driver version beside the runtime is research's question (scope reports the 2026-09-04 record has no driver field).
- TS harness members (`wdio.conf.ts`, `screen-reader.e2e.ts`, `nvdaExe` / `startNvda` / `stopNvda` / `SR_SUITES` / the per-suite spawn env) are proven only by EXECUTING an npm leg that loads them, never by the `tsc --noEmit` / `vite build` gate; no JS/TS unit runner is adopted (per test-plan §4 conductor-tauri/ui; §12).
- The SR suites run on real wall clock (30 s readiness window, 1.5 s quiet settle bounded at 8 s, 50 ms utterance-to-stamp window, foreground activation step, ≤15 s census poll) and are carried ONLY as the §9/§11 operator-local-gate exceptions, never as the deterministic tier (per test-plan §2 Test Strategy).
- Any Rust delta (e.g. a spawn or Tauri-side accessibility change) stays under the binding gates: nextest `ci` profile, `clippy -D warnings`, `--fail-under-lines 60`, zero retries (per test-plan §10 Quality Gates; §9 Build failure conditions).

## Patterns to follow
- Single-variable variation against a control where the real path is known to pass — the elevation cause was established this way (elevated vs non-elevated on the known-good host, differing on one attribute), major skew was retired by direct variation with a control, and a correlation without a measured mechanism is recorded as "mechanism recorded, not established" (per test-plan §9 Matrix builds; §6 desktop-webview row).
- Name an unmeasured candidate AS unmeasured (the runtime PATCH "unmeasured as a cause and named here as such") rather than promoting a fitting hypothesis to cause (per test-plan §6 desktop-webview row).
- Grade the pass record from the session's speech log against its action timeline, per subject, with the routine arm re-seeding its fixture dir clean per session so no session's journal leaks into another's subject (per test-plan §6 desktop-webview row).
- A harness-level wait that reproduces a documented precondition is part of the leg's firing form, not a synchronisation sleep (per test-plan §11 E2E).

## Anti-patterns to avoid
- NEVER synchronise inside a test with `sleep(N)` — wait on an explicit signal (speech-log utterance, element role / `aria-live`, focus state); and NEVER add retries to make a silent row pass (per test-plan §11 E2E; §11 Quality; §10 Zero-flakiness budget).
- NEVER add a manual smoke step or "human reviews screenshots" — a row the agent arm cannot reach is a recorded finding with its reason, and visual-pixel assertion is banned (per test-plan §11 Universal).
- NEVER open network beyond loopback — only the harness-lifetime WebDriver pair `:4444`/`:4445`, distinct from `:4317`; the live `sr` subject is local-gate-only (per test-plan §11 Universal; §2 Test Strategy).

## Contract bindings
- tests §6 SR family ↔ a11y-plan (the SR row spec `nvda-pass-spec.md`, its row classes `focus` / `live` / `browse`, and the outcome vocabulary the regrade grades into) — a11y owns what each row expects; tests owns how the leg fires and records.
- tests §1 `CONDUCTOR_NVDA` skip arm ↔ security-plan (handle guarded at the wdio edge; speech log untrusted, host paths scrubbed to `<host-path>`; harness-spawn forms stay seven) — any new launch/foreground mechanism crosses this.
- tests §2 / §11 Universal loopback set ↔ architecture §Occupied Resources (`4444`/`4445` driver ports; `:4317` for the live `sr` subject).

## Acceptance criteria contributions
- The fresh `nvda-pass.json` under this chunk's `evidence/` is produced by EXECUTING the `sr*` npm legs, and every TS harness member the chunk modifies is loaded by at least one executed leg (plus `--e2e` still green if a shared member such as `wdio.conf.ts` changed) — the build gate is not accepted as proof (per test-plan §4 conductor-tauri/ui; §6 desktop-webview row).
- The cause claim cites a discriminating measurement: one variable varied, a control run where the real path passes, and the runtime × driver recorded as a PAIR for every compared run (per test-plan §6 desktop-webview row; §9 Matrix builds).
- Browse-class rows (E0-09) remain `not-run-here` findings in the regrade — never counted as passes and never failing the leg (per test-plan §1 Untestable zones).
- If any Rust code changes: `cargo nextest run --workspace --profile ci` and `cargo clippy --workspace --all-targets -- -D warnings` green, with no nextest `retries` (per test-plan §10 Quality Gates; §11 CI).

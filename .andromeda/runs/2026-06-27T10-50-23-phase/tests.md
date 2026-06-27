# tests extract

## Relevance
Relevant — this chunk implements the GUI-integration test harness + E2E tauri-driver framework + deferred tauri::test integration tests for the desktop-webview surface.

## Constraints
1. §5 Integration — tauri::test mock-runtime command/Channel assertion lands here per 2026-06-26 amendment; GUI `start_run` background-thread Channel frames are thread-timing-dependent, so hermetic mock-runtime assertion is the CI-runnable leg (run logic itself unit-covered in conductor-run + CLI parity E2E).
2. §6 E2E — desktop-webview driver is @crabnebula/tauri-driver 2.0.9 via WebdriverIO headless on Linux `xvfb` ONLY; no WKWebView macOS support; webview is secondary to the CLI release gate.
3. §3 Test Harness Contract (ipc-internal Tauri surface) — commands (`start_run`, `stop_run`, `run_report`, `resolve_operator_hold`) + `Channel` frames must maintain JSON schema contract; mock-runtime assertion (CI) + real webview render (operator/local leg if display available).
4. §6 E2E selector discipline — role/text/aria-live ONLY (no xpath, no hashed CSS classes); "HOLD — operator pause" signature heading + `role="alertdialog"` + text-paired status labels mandatory per Color-Only a11y rule.
5. §9 CI Integration — npm audit 0 gate + committed `package-lock.json` (new frontend supply-chain control); cargo audit + cargo deny green for Rust deps; tauri-driver E2E runs `ubuntu-latest + xvfb` matrix only, not macOS/Windows webview runners.
6. §11 Anti-Patterns (universal) — NEVER skip display-gated legs silently (tauri-driver blockers surfaced with reason); NEVER merge without npm audit 0 + cargo audit/deny green + committed lockfiles.

## Patterns to follow
1. Per §5 Integration — tauri::test mock-runtime: `mock_builder()` + `mock_context(noop_assets())` + `get_ipc_response()` for command shape + fresh in-memory `runs.db` per test via `assert_fs::TempDir`, confirming identical envelope (`verdict`/`state`/`seed`) post-cycle.
2. Per §6 E2E Path 7 (Both-surface parity) — Tauri mock-runtime + CLI subprocess both write to same `CONDUCTOR_RUNS_DIR` → identical `runs.db` envelope for fixed seed; insta golden on journal redacting non-deterministic fields.
3. Per §6 E2E selector pattern — brand-anchor text (`[PASS]`/`[FAIL]`/`[HOLD]`/`[BLOCKED]`, P-ID tokens) + role/aria-live (`role="alertdialog"`, `aria-live` HOLD, `role=status` on checklist); avoid Tailwind hashes.
4. Per §8 Mocking — tauri::test mock-runtime (no webview, no Channel bridging) is CI leg; live render via tauri-driver on local gate or deferred; RUN LOGIC covered at conductor-run unit + conductor-cli E2E parity.

## Anti-patterns to avoid
1. **Never silently skip the tauri-driver webview leg** (§11 Universal + chunk scope Known risk) — if host lacks display, surface blocker with reason; do NOT hide it.
2. **Never use xpath / hashed-CSS-class selectors** (§6 E2E, §11 E2E) — text/role/data-testid only; Color-Only a11y rule makes text/role reliable.
3. **Never treat Blocked / ManualCheck / KnownResidual / CalibrationRegion as non-zero exit** (§11 E2E) — those are reported envelope states, not hard failures.

## Contract bindings
- **5-command discipline** (§3 Test Harness Contract) binds to obs §3 — chunk CONSUMES the harness (boot/run/status/cleanup/logs); does NOT modify commands.
- **Tauri-backend self-obs sink** `logs/conductor-tauri.jsonl` (§3 Log format + 2026-06-24 amendment) binds to obs §3 — distinct self-obs stream, separate from per-run emission journal `runs/<run_id>.jsonl`.
- **Run-report envelope** (§3 Status endpoint shape) binds to obs §Status — reads `runs.db` row + JSONL journal; schema unchanged (verdict/state/latency_ms/slo_tier/fingerprints).
- **Cross-surface-parity Path 7** (§6 E2E Both-surface parity) binds to core — Tauri mock-runtime ↔ CLI subprocess identical envelope for same scenario+seed.

## Acceptance criteria contributions
1. (tests) a11y/GUI harness tooling installed + configured (tauri-driver + axe-core + Lighthouse + colorjs.io) with smoke check; npm audit 0 + `package-lock.json` committed; cargo audit + cargo deny green.
2. (tests) Three deferred CARRY batches of tauri::test mock-runtime pass: `start_run`/`stop_run` + `run_report` + `resolve_operator_hold`; all assert against Run-report envelope schema.
3. (tests) Path 7 hermetic cross-surface-parity leg exists: Tauri mock-runtime `start_run` ↔ CLI subprocess `conductor run` yield identical `runs.db` envelope for one fixed seed; emission journal written from both.
4. (tests) Display-gated tauri-driver webview legs (axe/contrast/keyboard on RunReport + OperatorPauseDialog) either run on local operator gate OR explicitly scoped with reason; never silently skipped.

## Relevant amendment history
- **2026-06-26-live-counter-channel-stream** (§5 Integration deferred list) — deferred `tauri::test` mock-runtime command/Channel assertion + GUI parity leg to THIS chunk because GUI `start_run` background-thread Channel is thread-timing-dependent (would flake under zero-retry). **Resolution:** run logic covered at conductor-run unit tier + CLI parity E2E (headless leg of Path 7 holds).
- **2026-06-24-frameless-window-shell** (§3 Log format) — added Tauri-backend self-obs sink `logs/conductor-tauri.jsonl` when chunk made it LIVE; obs-plan §3 already documented both sinks. Routine §3↔§3 reconcile.
- **2026-06-15-design-token-typography-bundle** (§4 Unit) — frontend tests build-gated only; E2E deferred to Epoch 9 because no JS/TS unit runner adopted (GUI convenience-only). This chunk lands the E2E.
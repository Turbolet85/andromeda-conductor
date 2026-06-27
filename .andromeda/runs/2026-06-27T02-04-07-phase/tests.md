# tests extract

## Relevance
Partial — chunk adds Tauri command + GUI dialog wiring; integration/E2E tests deferred to ch9 per amendment 2026-06-26; this chunk is build/type-gated only.

## Constraints
- Per test-plan.md §1: `conductor-tauri` is "convenience only"; CLI is the release gate — GUI does NOT drive acceptance
- Per test-plan.md §3: The 5-command discipline is inherited; no new harness commands/endpoints introduced by this chunk
- Per test-plan.md §3: The status endpoint shape + log-format binding (to obs) are inherited; any Tauri backend self-obs feeds `logs/conductor-tauri.jsonl` per amendment 2026-06-24
- Per test-plan.md §4 (amended 2026-06-15): Frontend (ui/) bundle is build-gated only (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke); unit tests deferred to Epoch 9
- Per test-plan.md §10 Minimal tier: If new Rust command code is added, it must achieve ≥60% line coverage via `cargo llvm-cov`
- Per amendment 2026-06-26: Tauri command/Channel integration testing + GUI E2E parity deferred to the GUI test-harness ch9; this chunk is deterministic unit + build-gating only

## Patterns to follow
- Build-gate pattern per §4 (amendment 2026-06-15): React 19 + Tailwind bundle gated by `tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke; NO runtime/integration testing in-scope
- If new Rust `#[tauri::command]` logic: unit test pattern from §4 — rstest fixtures + deterministic seeded data (if needed) + test-isolation per `cargo nextest run -p conductor-tauri`; test the bridge LIBRARY fn (the TauriResolver decision-capture), not the thread-timing Channel stream
- E2E selector pattern (deferred to ch9 per §6): `role="alertdialog"` on operator-pause dialog, `aria-live` on HOLD/verdict announcements, signature heading text "HOLD — operator pause", no xpath/CSS-hash selectors
- Tauri mock-runtime pattern (deferred to ch9 per §5): `tauri::test::mock_builder()` + `mock_context(noop_assets())` + `get_ipc_response()` for command testing

## Anti-patterns to avoid
- Per §11 E2E (applies when ch9 lands): NO xpath selectors / brittle CSS-hash selectors on the operator-pause dialog; NO `sleep(N)` for synchronization; rely on role/text/`aria-live` signals + explicit state polling
- Per §11 Mocking (applies if integration testing is added): NEVER monkey-patch Tauri runtime; use the `test` feature + `mock_builder()` pattern only
- Scope safety: DO NOT escalate to tauri-driver / webview E2E testing until ch9; build-gating is the binding gate for this chunk

## Contract bindings
- **core::pause seam** → inherited (CARRY: `HoldPoint`, `Decision`, `PauseResolver` trait — no contract changes)
- **obs log-format** → inherited (amendment 2026-06-24: Tauri backend logs to `logs/conductor-tauri.jsonl` per obs-plan §3 dual-sink list; test-plan §3 owns the per-line schema; no new schema additions)
- **cross-surface parity** → inherited (per §6 Critical Path 7: CLI vs. GUI envelope determinism, deferred to ch9 E2E; run-logic unit coverage at conductor-cli + conductor-run tier NOW)

## Acceptance criteria contributions
- "(tests) `cargo build -p conductor-tauri` succeeds without `-D warnings` lint failures" — build/type-gate
- "(tests) `tsc --noEmit` + `vite build` on React/Tailwind bundle succeeds; `npm audit` green" — build-gate per amendment 2026-06-15
- "(tests) If new Rust command/resolver code: `cargo nextest run -p conductor-tauri` passes; the in-process decision-capture (oneshot bridge) is unit-tested synchronously, not the Channel stream" — unit gate (deferred scope per amendment 2026-06-26)
- "(deferred to ch9) GUI integration test: tauri::test mock-runtime command-shape assertion + `runs.db` row check (per §5 Tauri-IPC boundary)"
- "(deferred to ch9) E2E parity: Tauri-launched scenario + CLI scenario produce identical envelope verdict/state/seed (per §6 Critical Path 7)"

## Relevant amendment history
- **2026-06-15-design-token-typography-bundle** (§4 Unit Test Strategy): frontend (ui/) tests build-gated only (`tsc --noEmit` + `vite build` + `npm audit` + `vite preview` render smoke); frontend coverage via tauri-driver E2E deferred to Epoch 9. [Why: foundation chunk defers downstream-sequenced concern per playbook.]
- **2026-06-26-live-counter-channel-stream** (§5 Integration Test Strategy): Tauri-integration + GUI-parity tests deferred to the Epoch-9 GUI test-harness ch9; run logic covered now at conductor-run unit tier + conductor-cli E2E parity (headless Blocked envelope); integration/E2E legs land with sequenced tauri-driver harness. [Why: GUI `start_run` background-thread Channel streaming is thread-timing-dependent, would flake against zero-retry bar; unit+CLI coverage sufficient for this chunk's acceptance.]
- **2026-06-24-frameless-window-shell** (§3 Test Harness Contract / Log format note): Tauri backend self-obs sink `logs/conductor-tauri.jsonl` registered in dual-sink table; test-plan §3 (log-schema owner) now lists both cli + tauri sinks; schema unchanged (same per-line base fields). [Why: Tauri self-obs sink now LIVE; distinct from still-carried dual-envelope shape reconcile.]

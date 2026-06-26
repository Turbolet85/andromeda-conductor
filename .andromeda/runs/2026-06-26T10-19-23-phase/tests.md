# tests extract

## Relevance
Relevant — chunk adds Tauri IPC (Channel) + run-execution infrastructure for the control panel surface, enabling integration and E2E tests for Tauri commands/Channel and cross-surface parity per test-plan §6 Path 7.

## Constraints
1. Per test-plan §3, Tauri command/Channel must implement the Run-report envelope contract (run_id, seed, scenario, p_ids, verdict, state, latency_ms, slo_tier, journal_emitted_at, read_back_observed_at, fingerprints).
2. Per test-plan §5 (Integration), Tauri command-shape + runs.db row asserted via tauri::test mock-runtime (`get_ipc_response()`), not webview stub.
3. Per test-plan §6 (E2E), cross-surface parity (Path 7) requires Tauri-launched run and headless CLI run to produce identical runs.db envelope (verdict/state/seed) for the same scenario+seed.
4. Per test-plan §4, conductor-tauri/ui frontend is build-gated (tsc, vite, npm audit); webview E2E coverage deferred to later Epoch 9 chunk.
5. Per test-plan §10, line coverage ≥ 60% minimum; new conductor-core/conductor-run (pipeline extraction) code must not drop coverage.
6. Per test-plan §3, the extracted pipeline library must expose preflight + execute_scenario + coarse_emit semantics such that both conductor-cli and conductor-tauri produce byte-identical behavior under the same scenario+seed (CLI ↔ Tauri parity invariant).
7. Per test-plan §8, Tauri testing uses tauri 2.10.3 `test` feature mock-runtime, not monkey-patching.

## Patterns to follow
1. Per test-plan §5, Tauri integration tests: `tauri::test::mock_builder()` + `mock_context(noop_assets())` + `get_ipc_response()` for command-shape assertion + `runs.db` row check after start→stop cycle.
2. Per test-plan §6 Path 7, cross-surface parity: assert_cmd CLI subprocess + tauri::test mock-runtime run, both writing to shared `assert_fs::TempDir` CONDUCTOR_RUNS_DIR, asserting identical envelope via bound-parameter `runs.db` SELECT.
3. Per test-plan §7, rstest `#[fixture]` for seeded `conductor-timeline` generator; determinism asserted via same-seed re-run producing identical stream shape (insta golden, redacting run_id/timestamps).

## Anti-patterns to avoid
1. Per test-plan §11 Integration, NEVER mock runs.db — use rusqlite `open_in_memory()` (unit) or `assert_fs::TempDir` file DB (cross-process).
2. Per test-plan §11 E2E, NEVER treat `blocked`/`ManualCheck`/`KnownResidual` states as non-zero process exits; only hard `Fail`.
3. Per test-plan §11 Universal, NEVER add a scenario without a Pulse P-ID (this chunk adds no scenario, but extracted pipeline must not enable unchecked scenario creation).

## Contract bindings
- **Tauri IPC ↔ Test Harness (§3):** start_run/stop_run in conductor-tauri/src/commands.rs must produce Run-report envelope (runs.db row + JSONL journal) per §3 Status endpoint shape.
- **Pipeline extraction ↔ CLI/Tauri parity:** Extracted library (conductor-core or conductor-run) must expose same pipeline semantics (preflight / execute_scenario / coarse_emit) so both bins yield byte-identical behavior under same scenario+seed.
- **Self-obs ↔ obs-plan §3:** Tauri backend sink logs/conductor-tauri.jsonl carries per-line base fields (timestamp_ms, level, target, service.name/version/environment, run_id), distinct from per-run emission journal (runs/<run_id>.jsonl, SLO ground truth).

## Acceptance criteria contributions
1. (tests) `cargo nextest run -p conductor-tauri` passes new integration tests for start_run/stop_run commands + Channel IPC (tauri::test mock-runtime).
2. (tests) Cross-surface parity: Tauri mock-runtime start + CLI subprocess run for the same scenario+seed produce identical runs.db envelope (verdict/state/latency_ms).
3. (tests) Coverage: new code in conductor-core/conductor-run (pipeline extraction) and conductor-tauri (Channel binding) achieves ≥ 60% line coverage.
4. (tests) Sanitization: run_id, journal_emitted_at, read_back_observed_at, absolute host paths absent from JSONL journal (asserted via negative test or insta golden redaction).

## Relevant amendment history
1. **2026-06-15-design-token-typography-bundle (§4):** conductor-tauri/ui frontend is build-gated (tsc, vite, npm audit); webview E2E coverage deferred to later Epoch 9 chunk — explains why new frontend code replacing DEV cycler is not unit-tested; only Rust backend integration (Tauri commands/Channel) in scope per this chunk's test responsibilities.
2. **2026-06-24-frameless-window-shell (§3):** Tauri backend self-obs sink logs/conductor-tauri.jsonl registered as distinct artifact (per-line base fields incl. run_id, service-identity), separate from emission journal (runs/<run_id>.jsonl) — clarifies log-format binding now extends to Tauri sink alongside cli/agent-latest.jsonl.
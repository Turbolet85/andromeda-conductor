
## 2026-10-12-panel-shaped-types-retired — the webview fixtures, seed-dir class and Tauri sink retired; the parity arm named; the stop's limits recorded
**Section:** §1 Coverage triggers (Vector 1) · §1 Untestable zones · §3 → Log format · §5 Cross-surface parity · §7 Test Data & Fixtures
**Change:**
- §7: the committed fixture family is `crates/conductor-core/tests/fixtures/` plus `scripts/fixtures/mutation-gate/` (was also `crates/conductor-run/tests/fixtures/`). The round-trip set drops `lamps_fixture.rs` and `envelope_fixture.rs`. A reader- or sweep-side change leaving a fixture inert fails (was "a persist-side change").
- §1 Vector 1: `CONDUCTOR_E2E_SEED_DIR`'s class and its seeder NO-OP coverage left the trigger with its one reader.
- §3 Log format: the self-obs stream is the cli's (stderr, or `logs/agent-latest.jsonl` under `--agent-mode`). Was a Tauri backend sink with no writer, held by the Kept clause of `2026-10-11-conductors-window-retired — the window's test surface retired; Critical Path 7 narrowed to in-process ↔ headless`, now discharged.
- §5: the in-process arm `conductor_run::{preflight, drive_run}` is named as the composition `conductor suite` drives in production.
- §1 Untestable zones: `conductor suite`'s Ctrl-C stop at the signal. The `#[cfg(unix)]` first-interrupt test runs on the Linux dev host only until `Linux-only base CI`, and the second Ctrl-C's `std::process::exit(130)` stays unwitnessed with no owner. The outcome stays unit-covered (`suite_exit` ×2, `drive_run` ×3).
**Why:** the two webview fixtures, the seed-dir reader and the window log sink left the tree at this chunk. Both stop limits were accepted by the operator's word at phase review.
**Ref:** .andromeda/runs/2026-10-12T07-35-57-wrap/

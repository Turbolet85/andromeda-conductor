
## 2026-10-12-panel-shaped-types-retired — the Tauri backend sink retired; the READ boundary's spanless caller; the parity arm named
**Section:** §3 Logging stack and Log file location (body + keys) · §6 Sink configuration · §6 Boundary-call wrappers (DB READ) · §1 Instrumentation scope (`runs.db`) and Correlation · §1 Telemetry triggers (cross-surface parity) · §4 conductor-report row · §4 In-process ↔ headless parity · `obs-plan-contracts.toml` labels
**Change:**
- The Tauri backend sink rows retired from §3, §6 and the Logging stack and Log file location keys. Their `Sink (Tauri backend)` and `Tauri backend` labels left the index. Was "no writer since 2026-10-11", held by the Kept clause of `2026-10-11-conductors-window-retired — the window's surfaces, identities, ticks and a11y record retired`, now discharged.
- The DB READ boundary (`read_envelope` over `RunsDb::get_envelope`) logs inside the caller's span where the caller holds one. `conductor report`, its first production caller, holds none (its crate declares no `tracing` dependency), so there the line is matched by `run_id` alone; still no `db.*` read span. §1's `runs.db` row and §4's conductor-report row say the same (was an unqualified "inside the caller's span").
- The parity arm `conductor_run::{preflight, drive_run}` is named as the composition `conductor suite` drives, in §4, the parity path, §1 Correlation and the Telemetry trigger.
**Why:** the window log sink left with this chunk. `conductor report` became `read_envelope`'s first production caller, and `drive_run` gained `conductor suite` as its first production caller.
**Ref:** .andromeda/runs/2026-10-12T07-35-57-wrap/

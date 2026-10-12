
## 2026-10-12-panel-shaped-types-retired — the window log sink and seed-dir handle retired; the suite driver, the checklist bound and the catalogue table restated
**Section:** §Occupied Resources (on-disk artifacts · environment variables · crate names) · §Infrastructure Patterns → Directory structure · §Conventions (config) · §Stack and Technologies (Terminal output rendering)
**Change:**
- The `logs/conductor-tauri.jsonl` artifact row retired (was "no writer since 2026-10-11 … the sink goes at `Panel-shaped types retired`").
- The `CONDUCTOR_E2E_SEED_DIR` row retired: its one reader, `envelope_fixture.rs`, left with this chunk and no code reads or sets it.
- `conductor-run` runs "the suite driver `drive_run`": the composition `conductor suite` drives, with a per-scenario progress callback, a stop flag polled before each scenario and set by the suite's Ctrl-C handler, and one persist that keeps every finished record (was "the live-counter run driver"). The keyed directory tree says the same (was "live-counter drive_run").
- `MAX_CHECKLIST_TEXT = 200` is a length bound on untrusted scenario-authored text the CLI's interactive hold prints, each half on its own indented `induced:` / `observe:` line. `HoldPoint` projects the items to the CLI's interactive hold; the headless path prints none (was "a dialog row's display bound", "projects them to its shell").
- comfy-table's uses add the `conductor scenarios` catalogue table beside the results and capability-coverage tables.
**Why:** the panel's channel payload, picker guard, webview fixtures, seed-dir reader and log sink left the tree, and four functions gained a command-line entry. The window log sink was held for this chunk by the plan.
**Ref:** .andromeda/runs/2026-10-12T07-35-57-wrap/

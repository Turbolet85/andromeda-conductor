
## 2026-10-11-conductors-window-retired — the window's surfaces, identities, ticks and a11y record retired
**Section:** §1 (tier justification; the `conductor-tauri` entity; the desktop-webview and ipc-internal surface rows; service identity; correlation; heartbeat; critical paths; telemetry triggers) · §3 (Sinks; Heartbeat ticks) · §4 (per-surface span table; the parity scenario) · §6 (log levels; sinks) · §9 (the a11y artifact row) · §10 (the clippy failure condition) · §11 (bounded span-name set; PII scrubbing; project-specific bans) · §3 → Service identity · Logging stack · Log file location · Heartbeat ticks · Correlation · OTel SDK init · Log format JSON schema · Bootstrap phases
**Change:**
- Retired: the `conductor-tauri` entity and the desktop-webview / ipc-internal surfaces; the `conductor-tauri` / `conductor-ui` identities (`service.name` is `"conductor"`); the Tauri frontend sinks; the Tauri heartbeat ticks; the IPC correlation row; the `conductor-tauri` log level; `tauri.command.*` from the bounded span-name set; the §9 a11y artifact row; §10's "the `--e2e` arm runs no clippy line" clause.
- The parity path and scenario read in-process ↔ headless (`conductor_run::{preflight, drive_run}` beside `conductor run`); the multi-platform and panic triggers name the CLI alone.
- The Log format JSON schema's extension point is no longer exercised: the a11y gate's violation record left with the `a11y` job (it was, from 2026-09-07).
- The a11y leg's scrub "is a second line of defence" now reads "was, until that leg retired"; the applied `CREATE_NO_WINDOW` flag has no standing evidence leg.
**Why:** the window, its IPC and its CI job left the tree (intent R1).
**Kept:** the Tauri backend sink rows (§3, §6, Logging stack, Log file location) stay, marked no writer since 2026-10-11 — the plan holds the window log sink until `Panel-shaped types retired`, which owns it.
**Ref:** .andromeda/runs/2026-10-11T08-28-34-wrap/

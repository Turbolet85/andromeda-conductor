# arch extract

## Relevance
Relevant — implements the Tauri/GUI branch of the `conductor_core::PauseResolver` seam; wires an established dialog + command surface over the established tokio async runtime and Tauri IPC layer.

## Constraints
1. TauriResolver code lives in `conductor-tauri` workspace crate per module-boundary compiler enforcement (arch §Occupied Resources crate names).
2. Operator-pause `#[tauri::command]` conforms to the established internal core↔UI Tauri-command request/response surface, no new ACL permissions (arch §Standard Contracts — Tauri commands for start/stop/picker/run-report-view; scope: no new capabilities).
3. Hold-request signal backend→frontend MUST use the existing `Channel` streaming mechanism or extend it as a new `RunStage`/event type per architecture §Real-time Strategy (in-app live updates only; `Channel` for live counters / target status backend→frontend without per-message JSON overhead).
4. TauriResolver `resolve(&HoldPoint) -> impl Future<Output=Decision>` MUST NOT block Tauri's GUI thread — background run task awaits a `tokio::sync::oneshot` (arch §Design Philosophy headless-drivable core, thin shells; §Async Runtime Flavor core-owned `current_thread` means the run task + resolver live off the GUI reactor).
5. Error handling at the command edge: `anyhow` Result envelope; `Decision` (Go/NoGo) is a value, not an error variant (arch §Conventions error handling — typed verdict/error wall).
6. TauriResolver + Decision + HoldPoint types REUSE from `conductor_core::pause`; OperatorPauseDialog.tsx REUSES from component-primitives-library (scope: wiring only, no rebuilds).
7. No new `CONDUCTOR_*` env var, no new port/socket binding, no new on-disk artifact (conductor-tauri.jsonl §Occupied Resources already registered; scope: GUI resolver, not live-run emission).

## Patterns to follow
1. Tauri command pattern: `#[tauri::command]` fn `resolve_operator_hold(decision: Decision)` returning `Result<(), String>` (no-op resolution — the awaiting oneshot captures the decision via the channel closure, not a return value).
2. Backend→frontend hold-request: emit HoldPoint as a RunStage / Channel message (precedent: live-counter Channel established in ch6/ch7; extend with `RunEvent::OperatorHoldRequest(HoldPoint)` or reuse existing envelope).
3. Oneshot-based resolver: TauriResolver holds a `tokio::sync::oneshot::Sender<Decision>`, `resolve()` yields it to the channel callback, the command invocation sends the operator's Decision down the channel; awaiting the Receiver is the blocking-free pattern.
4. React component integration: listen for `RunEvent::OperatorHoldRequest(hold)`, open dialog with `hold.prompt` + `allow_no_go`, invoke command on Proceed/Abort, close on decision-received (no polling, event-driven).

## Anti-patterns to avoid
1. Do not block tokio's `current_thread` runtime with a synchronous wait or a `spin_loop` for operator input — the run task must be off-the-GUI thread and the async boundary must be clean (oneshot, not parking_lot::Mutex polling).
2. Do not add new Tauri ACL capabilities (scope boundary: no `core:window:*` or other permission-required Tauri API beyond the established command/window context; confirm `tauri.conf.json` capabilities block is unchanged).
3. Do not duplicate the HoldPoint model, Decision enum, or OperatorPauseDialog scaffold — they are established contracts (core §pause, component-primitives-library) that this chunk wires, never rebuilds.

## Contract bindings
- **conductor_core::pause** ↔ TauriResolver impl: the pause seam's PauseResolver trait is satisfied by the Tauri impl; Decision / HoldPoint types flow through the command→oneshot→resolver boundary.
- **OperatorPauseDialog.tsx** ↔ frontend event loop: the component's controlled open/onOpenChange/onProceed/onAbort slots are driven by incoming hold-requests and operator input over the command surface.
- **Live-counter Channel** ↔ hold-request signal: the channel streaming backend→frontend (established §Real-time Strategy) is extended or re-used to surface the pending HoldPoint (P4 clarifies the mechanism: new RunStage variant vs. dedicated channel).
- **conductor-run (lib crate)** ↔ drive_run pipeline: scope defers run-pipeline hold-emission (e.g., where in the scenario timeline a hold is injected) to Epoch-10; this chunk proves the resolver bridge against a constructed/seeded hold in tests, does not wire hold points into the live scenario loop.

## Acceptance criteria contributions
1. (arch) TauriResolver lives in `conductor-tauri`, imports `conductor_core::pause::{PauseResolver, Decision, HoldPoint}`, and implements the seam (compiler-enforced by trait impl location + dep edges).
2. (arch) New `#[tauri::command]` operator-pause command is reachable from the webview without new ACL permissions (existing Tauri app context allows command invocation; confirm `tauri.conf.json` capabilities list is unchanged from the shipped state).
3. (arch) Hold-request signal backend→frontend uses the established Channel or a new RunStage variant on the same channel (no new IPC mechanism; Standard Contracts §Conventions internal core↔UI surface is Tauri commands + Channel only).
4. (arch) TauriResolver::resolve(&HoldPoint) returns an `impl Future` (e.g., async fn + tokio::sync::oneshot), never blocks the Tauri GUI thread, and decision is captured by a command callback (arch §Async Runtime Flavor core-owned current_thread).
5. (arch) Error on command boundary is `anyhow::Result<()>`; Decision type is an `Ok(value)`, never a `Result` (arch §Conventions verdict/error wall).

## Relevant amendment history
- **2026-06-26-live-counter-channel-stream**: registered `conductor-run` library crate (9th workspace member, run composition root above seams below both bins, shared by conductor-cli + conductor-tauri); live-counter Channel mechanism already pinned in §Real-time Strategy (hold-request signal will extend or reuse this).
- **2026-06-24-frameless-window-shell**: registered `logs/conductor-tauri.jsonl` artifact (Tauri backend self-obs via ObsSink::File); registered `@tauri-apps/api` (window/IPC client) in Desktop frontend stack; documented tauri-build `generate_context!` build-order coupling (ui/dist must be built before conductor-tauri cargo compile).
- **2026-06-23-isatty-gated-operator-pause**: registered inquire 0.9 in §Stack (CLI interactive PauseResolver for headless path); this chunk implements the parallel GUI PauseResolver (two resolver impls: CLI prompt + Tauri dialog, both satisfy the conductor_core::pause seam trait).
- **2026-06-15-design-token-typography-bundle**: registered React 19.x + Vite 8.0.16 + Tailwind v4.1 (Oxide) frontend stack + `crates/conductor-tauri/ui/` npm subtree (not Cargo member); OperatorPauseDialog.tsx lives in this frontend layer.
- **Earlier amendments** (2026-06-14 → 2026-06-16): Tauri 2 v2.10.x registered, tokio 1.48.x `current_thread` runtime flavor locked, async/await + error-handling patterns (thiserror in seams + anyhow at edges) established, verdicts-as-values design pinned.

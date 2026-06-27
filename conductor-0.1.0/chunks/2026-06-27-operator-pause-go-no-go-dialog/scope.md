# Scope — Operator-pause go/no-go dialog

**Marker:** 2026-06-27-operator-pause-go-no-go-dialog
**Version:** conductor-0.1.0 · Epoch 9 (Desktop control panel) · ch8/10
**Primary crate:** `conductor-tauri` (UI + one command; possible thin `conductor-run`/core touch — see Boundary question)

## What this builds
The **desktop go/no-go surface** for an operator pause. When the run reaches a `HoldPoint` — a *non-Conductor action* Conductor deliberately will not perform itself (a Pulse restart for P-015, a manual observation of a visual `ManualCheck` claim, any go/no-go gate before a committed timeline step) — the GUI raises the **already-shipped** `OperatorPauseDialog` (the Radix AlertDialog scaffold from component-primitives-library), the operator answers **Proceed/Abort**, and that `Decision` (`Go`/`NoGo`) flows back to the awaiting run, which records a `HoldResolution` and resumes or stops.

This is the **Tauri implementation of the `conductor_core::PauseResolver` seam** — the third shell after the `HeadlessResolver` (agent path, never blocks) and the CLI `inquire` prompt (Epoch 8). The seam already exists; this chunk supplies the GUI resolver + its backend↔webview bridge + the frontend dialog wiring.

## Surfaces / contracts touched
- **`conductor_core::pause`** — REUSE, never rebuild: `HoldPoint { scenario, p_id, step, prompt, allow_no_go }`, `Decision { Go, NoGo }` (+ `.label()` "Go"/"No-Go"), `HoldResolution`, the `PauseResolver` trait (`resolve(&self, &HoldPoint) -> impl Future<Output = Decision>`), `resolve_hold`.
- **`OperatorPauseDialog.tsx`** — REUSE, never rebuild: controlled `open`/`onOpenChange` + `title`/`body`/`onProceed`/`onAbort`/`allowNoGo`. Already supplies the `alertdialog` role, focus-trap, Escape, focus-restore (Radix).
- **A new `#[tauri::command]`** — *the* operator-pause command (singular, per the CARRY): the frontend posts the operator's `Decision` back to wake the resolver awaiting in the background run thread.
- **A hold-request signal backend→frontend** — surface the pending `HoldPoint` to the webview so the dialog can open (over the existing run `Channel` as a new `RunStage`/event, or a dedicated hold Channel — P4 decides the mechanism).
- **A Tauri `PauseResolver` impl** (new, e.g. `TauriResolver`) bridging the background run thread ↔ webview: `resolve(hold)` emits the hold to the frontend and `await`s the operator's decision (a `tokio::sync::oneshot`), never blocking Tauri's GUI thread.
- **Titlebar HOLD run-state** — REUSE the paused-count freeze/tint signature (paused-count-hold-point-signature): the count freezes at the hold value while the dialog is open; the dialog complements that existing visual cue (motion-is-the-event), not replaces it.
- **App.tsx run loop** — listen for the hold signal, open the dialog with the hold's prompt, invoke the resolve command on Proceed/Abort.

## Boundaries / non-goals
- Does **NOT** rebuild the dialog scaffold or the core hold model (both ship).
- Does **NOT** add the live-Pulse holds for real scenarios (P-015 restart cue, P-025/026/027 visual `ManualCheck` observation) — those land with the Epoch-10 live-Pulse runs (today every scenario resolves `Blocked`, so there is no live non-Conductor action to gate yet).
- **RESOLVED at P4 (AskUserQuestion — scope boundary): Full bridge.** The hold seam already exists in `execute_scenario` (the empty-`expected` operator-checklist hold), but `drive_run` hardcodes `HeadlessResolver::proceed()` so the GUI can't inject a resolver. ch8 **generalizes `drive_run` to accept a `PauseResolver`** (low blast radius — only the Tauri caller + 2 unit tests; the CLI calls `execute_scenario` directly and is untouched) so the bridge is end-to-end: the live-Pulse operator-checklist holds (P-025/026/027/P-032) drive the GUI dialog in Epoch-10 with zero further wiring. (The alternative — purely conductor-tauri, defer the drive_run injection — was declined.)
- Never color-alone / reduced-motion / keyboard-trap-free are inherited from the shipped scaffold + `a11y.md`; **GUI integration + axe/keyboard tests defer** to the Epoch-9 **Desktop a11y harness** chunk (ch9) per the carried route note — this chunk is build/type-gated only.
- No new core/plugin Tauri permission (app `#[tauri::command]`s are not ACL-gated — 2026-06-26 rule); confirm the deny-by-default capability set is unchanged.

## Folded annotations (from the working-route entry)
- **CARRY:** the `OperatorPauseDialog` scaffold (Radix AlertDialog — controlled `open`/`onOpenChange` + title/body/Proceed/Abort slots + `allowNoGo`, focus-trap/Escape/restore) **SHIPS** from component-primitives-library — ch8 **WIRES** it (drive `open`/`onProceed`/`onAbort` from a `HoldPoint` + the operator-pause command), doesn't rebuild it.

# Scope — Live-counter Channel stream

**Marker:** 2026-06-26-live-counter-channel-stream
**Epoch:** 9 — Desktop control panel (ch4/10)
**Crate(s):** conductor-tauri (primary) · conductor-core / new conductor-run (pipeline extraction) · conductor-cli (resolver generalization + parity)
**P-ID:** N/A — GUI/infra control surface, no new scenario (the scope law's "no scenario without a P-ID" governs scenarios, not the live-update plumbing)

## What it builds

The first LIVE backend→frontend data path in the GUI: a Tauri 2 IPC `Channel` streaming live emission
counters + target status off a REAL run execution — replacing ch3's option-A control scaffold (validate-only
`start_run`) and ch2's DEV-only run-state cycler with genuine run-driven titlebar state.

Three intertwined deliverables (the named one + two folded CARRYs):

1. **Tauri `Channel` live stream (the named deliverable).** A backend→frontend `Channel<T>` carrying live
   emission counters (spans/logs/exceptions emitted) + target status (OTLP-egress / preflight state), per the
   architecture Real-time Strategy decision: Channel for in-app live updates only — no per-message JSON
   overhead, no SSE/WebSocket/polling, and NO native OS toasts (those are Pulse behavior Conductor observes).
   The frontend consumes the stream to drive the EXISTING titlebar `runState` heartbeat + counter readout.

2. **Run-execution + pipeline-to-library extraction (CARRY from ch3 scenario-suite-picker-start-stop's
   option-A scaffold).** Make `start_run` drive a REAL run rather than only validating + transitioning state:
   - Extract `conductor-cli`'s bin-local `pipeline.rs` (`preflight` / `execute_scenario` / `coarse_emit`) into
     a LIBRARY reachable by both bins — `conductor-core` or a NEW `conductor-run` crate (a P4 decision).
   - Generalize the pipeline's `&CliResolver` dependency to a generic bound `<R: PauseResolver>` (the
     `PauseResolver` abstraction from the operator-pause-orchestration chunk), so Tauri supplies its own
     resolver — NOT a trait object: `PauseResolver::resolve` returns `impl Future`, so the trait is not
     object-safe; this matches the existing generic `resolve_hold<R: PauseResolver>` (P3 research correction to
     the working-route's "trait object" wording).
   - Stand up a core-owned `current_thread` tokio runtime under Tauri (Tauri's shell runtime is multi_thread;
     the engine's determinism requires `current_thread` per the architecture runtime-flavor decision —
     `Builder::new_current_thread()` owned by the core so Tauri's multi_thread stays the GUI shell's concern).
   - `start_run` then drives a real `RunRecord` → **Blocked** without a live Pulse (preflight gate) →
     persisted to `runs.db` + the JSONL journal, exactly as the CLI does (CLI↔Tauri parity = test-plan Path 7).

3. **DEV-cycler retire (CARRY, ch2→ch3).** Remove the DEV-only run-state cycler in `ui/src/App.tsx`
   (`useState<RunState>` + the `import.meta.env.DEV` block) and drive the existing `Titlebar runState` prop
   (its contract + idle/live/hold/aborted heartbeat/freeze/tint rendering shipped in ch2
   paused-count-hold-point-signature) from the live Channel. Reconcile with ch3's direct start/stop →
   `setRunState` wiring (start/stop now flow through real execution + the Channel, not a local DEV setter).

## Boundaries (what it does NOT do)

- **No engine/seam MODEL change** — Verdict/ReportState, the scenario model, the verdict/error wall, the SLO
  math are untouched. The pipeline EXTRACTION moves code between crates (a structural dependency-edge change)
  but does not change the model; headless `agent-run` behaviour is byte-identical.
- **Headless `agent-run` / CLI path stays green** — the extraction must leave the CLI behaving identically;
  parity is the test, not a regression surface.
- **Still no live Pulse** — runs resolve **Blocked** (the preflight canary fails w/o Pulse). This chunk proves
  the EXECUTION wiring + persistence + live counters, NOT a Pass/Fail verdict. Live-Pulse E2E is Epoch 10.
- **No live interactive operator-pause leg** — the Tauri resolver stays the never-block/headless-equivalent
  default; the interactive go/no-go dialog is a later Epoch-9 chunk and the live pause bridge is Epoch 10.
- **Channel is in-app only** — no native OS notifications, no new inbound listener (scope law); the only OTLP
  egress remains the product fault stream to Pulse `:4317`.
- **Determinism preserved** — the core-owned `current_thread` runtime keeps "same scenario+seed ⇒ same stream
  shape" under the Tauri shell.

## Surfaces / contracts touched

- `conductor-tauri/src/commands.rs` — `start_run` drives real execution + opens the Channel; `stop_run` aborts it.
- `conductor-tauri/src/main.rs` — manage the core-owned `current_thread` runtime + wire the Channel.
- `conductor-tauri/Cargo.toml` — new dependency edge on the extracted pipeline library.
- **Pipeline library extraction** — `conductor-cli/src/pipeline.rs` → `conductor-core` or a new `conductor-run`
  crate; `conductor-cli` becomes a consumer of the library (the parity anchor).
- `PauseResolver` generic-bound generalization (`execute_scenario<R: PauseResolver>`, not a trait object — the
  trait is not object-safe) — conductor-core (resolver abstraction) + conductor-cli (`CliResolver` impl,
  unchanged) + conductor-tauri (passes core's `HeadlessResolver`).
- The Channel payload type — a new serde type (live counters + target status), shared backend→frontend,
  canonical-name serialized.
- `ui/src/App.tsx` + the titlebar component — DEV cycler removed; `runState` + live counters driven by the Channel.
- `crates/conductor-tauri/capabilities/*.json` — NO change needed (P3-confirmed): app-command `Channel` args
  are not ACL-gated; the deny-by-default window allowlist stays the existing 3 entries (ch3 security learning).

## Carried annotations folded in (from the working-route entry)
- **CARRY (ch3):** real run EXECUTION + pipeline-to-library extraction + `&CliResolver`→trait object +
  core-owned `current_thread` runtime under Tauri + real `RunRecord` persistence (CLI↔Tauri parity, test-plan Path 7).
- **CARRY (ch2→ch3):** retire the DEV run-state cycler; drive the existing `Titlebar runState` prop from the
  Channel; reconcile ch3's direct start/stop → `setRunState`.

## Decisions (resolved at P4 — user-ratified)
- **Pipeline library home → NEW `conductor-run` crate.** `conductor-core` is architecturally IMPOSSIBLE (the
  pipeline is the composition root importing timeline/emit/verify/report — all of which depend on core; placing
  it in core inverts every seam edge and won't compile). P3 research finding.
- **`start_run` execution model → background core-owned `current_thread` runtime + live-streaming `Channel` +
  real `stop_run` abort** (option A). The Channel streams run lifecycle + target-status; per-emission counters
  stay 0 on the Blocked path (Epoch-10 adds the pipeline progress hook).
- **Resolver mechanism → generic bound `<R: PauseResolver>`** (not a trait object — the trait returns `impl
  Future`, so it is not object-safe; matches `resolve_hold<R>`).

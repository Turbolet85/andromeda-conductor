# Scope — Operator-pause orchestration

**Marker:** `2026-06-21-operator-pause-orchestration`
**Epoch:** 5 — Verification & read-back (chunk 6 of 6 — the last of the epoch)
**Primary seam:** `conductor-core` (the runtime-agnostic hold model + resolver abstraction) with a likely
integration point in `conductor-timeline` or `conductor-verify` — see Open seam question.
**Working-route intent:** _Operator-pause orchestration — go/no-go holds + resume-on-confirm for non-Conductor
actions._

## What this chunk builds

The runtime-agnostic **hold/resume orchestration** that lets a scenario timeline *pause* at a point where a
**non-Conductor action** is required — an action Conductor deliberately will NOT perform itself (a Pulse
restart for P-015 restart-detection, a manual observation of a visual/desktop claim that terminates in
`ManualCheck`, any operator go/no-go gate before a committed timeline step) — and then *resume* once the
operator confirms. This is the core mechanism the two thin shells later surface: the CLI's isatty-gated
`inquire` prompt (Epoch 8) and the Tauri AlertDialog go/no-go dialog + paused-count titlebar signature
(Epoch 9) are deferred consumers of what this chunk defines.

This chunk supplies, at minimum:

1. **Hold-point model** — a typed description of a single operator hold: which scenario / P-ID / step it
   gates, the human-readable prompt (the non-Conductor action to perform/observe), and whether a `no-go`
   outcome is permitted. A typed VALUE (serde where it must round-trip through an artifact or IPC), no host
   paths or internal struct names leaking through the redaction edge.

2. **Go/no-go decision + resume-on-confirm** — the closed outcome set of a hold (`go` ⇒ resume the timeline
   from the hold point; `no-go` ⇒ the operator-declined outcome) plus the async orchestration that *awaits*
   the resolution and resumes deterministically. The resolution is a VALUE, never a `Result::Err` (the
   verdict/error wall): a `no-go` is an operator decision, not a harness fault.

3. **Resolver abstraction + the headless-safe default** — a trait (or equivalent seam) that decouples *where
   the hold is awaited* from *how it is answered*, so the same core orchestration is driven identically by
   headless and interactive shells. The one resolver this chunk ships is the **headless / agent-mode** default
   that **never blocks** (the "headless never blocks" CLI discipline): in non-interactive runs a hold
   auto-resolves to a defined, deterministic outcome and is recorded for the report rather than waiting on a
   human. Interactive resolvers (CLI `inquire`, Tauri dialog) are explicitly out of scope here.

4. **Determinism preservation** — the hold introduces a *wall-clock* gap only; it MUST NOT perturb the seeded
   emission-stream shape (same scenario + seed ⇒ same stream shape). Tests resolve holds via an immediate
   auto-resolving stub under `tokio::time` `start_paused`; the human pause is outside the seeded virtual
   clock. Any timing the report records stays journal-relative from `std::time` (the pause is not folded into
   the deterministic schedule).

## Boundaries (NOT in this chunk)

- **Not** the CLI interactive prompt — `isatty`-gated `inquire` confirm + paused-count spinner mirror is
  **Epoch 8** (CLI surface). This builds the core mechanism + the headless never-block resolver, not the TTY UI.
- **Not** the desktop go/no-go UI — the Tauri AlertDialog gating each committed step + the paused-count
  hold-point titlebar signature (freeze/tint/resume) are **Epoch 9** (desktop control panel).
- **Not** UI automation and **not** Pulse process management — both are stated project non-goals. Conductor
  *asks the operator* to perform the non-Conductor action; it never performs it.
- **Not** the run-report serializer / `runs.db` / Markdown report (**Epoch 6**) — a hold outcome may need to
  surface in the eventual report (e.g. a `ManualCheck` confirmation), but this chunk computes/records the
  outcome value, it does not persist the report envelope.
- **Not** new MCP read-back or OTLP emission — pure orchestration over the existing timeline/verification
  seams; CI-testable with stubs, no live-Pulse leg.

## Surfaces / contracts touched

- `conductor-core` (likely primary) — the hold-point model + go/no-go decision type + the resolver
  trait/abstraction as shared vocabulary, so both bins drive the same core (the "headless-drivable core, thin
  shells" + star-topology invariant: seam crates import only `conductor_core`).
- `conductor-timeline` and/or `conductor-verify` (integration point) — where a hold is actually *awaited* and
  the timeline resumes; the seam split is the open question below.
- Deferred consumers (defined elsewhere, NOT built here): the CLI resolver (Epoch 8), the Tauri resolver +
  live `Channel` paused-count signal (Epoch 9), the report surfacing of a hold outcome (Epoch 6).
- Verdict/error wall: a hold resolution is `Ok(...)` value; `Result::Err`/a `*Error` variant is reserved for a
  genuine harness fault (e.g. a malformed hold-point at load, if one is deserialized).

## Open seam question (resolve in planning, P4)

Where does the orchestration *await* live, and how thin is the core abstraction?
- The hold-point **model + go/no-go type + resolver trait** bias toward `conductor-core` (runtime-agnostic
  shared vocabulary both bins import — mirrors "config/types in core, logic in the seam").
- The *await + resume* integration could live in `conductor-timeline` (it owns the `tokio::time` phase
  scheduler the hold pauses) or in `conductor-verify` (holds are gated on verification go/no-go and the
  `ManualCheck` operator-checklist path this epoch owns). The async-resolver shape (a trait object vs an
  `mpsc`/oneshot channel vs an `async fn` callback) is the concrete design choice — to be confirmed against
  the architecture (real-time `Channel`/IPC notes) + the test/obs extracts before finalizing.

## Acceptance anchor

Same scenario + seed + an auto-resolving (headless) resolver ⇒ identical emission-stream shape and identical
recorded hold outcome as a run with no holds at all (determinism preserved; the pause is wall-clock-only). A
hold awaited under the headless default **never blocks** — it auto-resolves to its defined outcome. A `no-go`
is returned as a typed value, never a panic or `Result::Err`. No host-path / internal-struct-name leak through
the redaction edge into any recorded prompt or outcome.

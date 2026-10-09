# Obs validation — route draft

## No suggestions
Draft covers obs domain. All three obs suggestions from the earlier passes are in the draft (`Own-log gates on the one form`, `Own log over a run of hours` ahead of `Living background`, the own-log gates in `Linux-only base CI`'s kept-list). The third pass's changes (reworded `Discrimination at description time`, the split-out `Short regression runs: silence subjects`, reworded `Regression set driven against a live engine`) add no own-log, gate or scrubbing surface.

- **Bootstrap items present:**
  - OTel SDK init → none expected: obs-plan §3 key `otel-sdk-init` and §11 ban a self-observation SDK, and no draft entry introduces one.
  - Logging stack / sink (`logger-stack-install`) → shipped in 0.1.0 (`/home/turbolet/dev/projects/conductor/.andromeda/master-route.md:15`, `:58`, both `complete`), so no install entry is owed. Its 0.4.0 changes each have an entry: the window sink leaves in "Panel-shaped types retired" (Epoch 1); the one-writer truncating sink is replaced in "Own log over a run of hours" (Epoch 7); "Rotation: N/A" is replaced in "Disk bound over ten hours" (Epoch 7).
  - Service identity (`service-identity-wire`) → shipped with the same 0.1.0 record; the `conductor-tauri` / `conductor-ui` identities leave with "Conductor's window retired" and "Panel-shaped types retired" (Epoch 1).
  - Log format schema (`log-format-schema-emit`) → the §6 envelope is re-subjected by "Run record for the one form" (Epoch 3); its tier fields leave in "Short-run tiers retired" (Epoch 5).
  - Correlation (`run-id-correlation-wire`) → the IPC envelope half leaves with the window (Epoch 1); `run_id` on every own-log line stays and is held by the own-log conformance gate. No W3C propagation entry is expected (§3 key `correlation-no-distributed-tracing`).
  - Heartbeat → chunk "Own log over a run of hours" (Epoch 7), replacing §3 key `heartbeat-ticks` "CLI: N/A".
  - Error capture → no external platform by plan (§11 Error Reporting); the panic hook shipped in 0.1.0, so no entry is expected.
  - Scrubbing (`pii-scrubbing-wire`) → shipped in 0.1.0 (`master-route.md:16`, `complete`); extended for what 0.4.0 adds by "Credential hygiene" (Epoch 6), "Reaction log on the timeline" (Epoch 3, content boundary) and "Agent's answer taken in" (Epoch 8, scrubbed before recording).
  - Obs CI gates (`obs-ci-gate-wire`, §9) → shipped (`master-route.md:69`, `complete`); carried to Linux by "Linux-only base CI" (Epoch 1) and given a one-form producer by "Own-log gates on the one form" (Epoch 3).

- **Sequencing deps satisfied:**
  - Log init before production code → confirmed; the stack predates every 0.4.0 entry.
  - Own-log gates keep a subject → confirmed. Today's producer is a single-list run forced Blocked through the shared-machine precondition (`/home/turbolet/dev/projects/conductor/.github/workflows/ci.yml:185-193`). The one-form producer lands at the end of Epoch 3, before Epoch 5's "Single-list scenario form retired" and "Spawned sidecar and shared-machine gate retired" remove it.
  - Heartbeat before long-running entries → confirmed. "Own log over a run of hours" precedes "Living background", "Four event families" and every later hours-long entry; everything before it is a short or trivial run.
  - Scrubbing before the first held credential → confirmed. "Credential hygiene" precedes "Engine address and identity given to a run" and "Two-host path reachable", the first entry given a token and door credential. Epochs 1–5 read an own-host, owner-only door (`/home/turbolet/dev/projects/andromeda-pulse/andromeda-pulse-0.4.0/working-route.md:25`), so no credential is held earlier.
  - Trace propagation before cross-surface entries → not applicable; the only cross-surface path (§4 both-surface parity) retires in Epoch 1.

- **Coverage:**
  - §4 must-trace paths → confirmed. Path 1 is carried by Epoch 2's schedule and run description plus Epoch 3's gate, reaction log, per-event grading and run record. Path 2 → "Short regression runs: identity and capture". Path 3 → "Short regression runs: silence subjects". Path 4 → "Short regression runs: quiet and lifecycle". Path 5 retires with "Local-model grading retired" and "Model and manual vocabulary retired". Path 6 → "Accepted capability set re-based" and "Capability backing gate on the one form". Path 7 retires with "Panel-shaped types retired".
  - Telemetry triggers (§1) → confirmed. The perf-budget fields are replaced by "Per-event grading" and retired by "Short-run tiers retired". Of the chaos spans, `fault.port_occupier` leaves in "Port fault and shared-machine scenarios retired", and silence/ramp ride the kept emission shapes in "Schedule on one timeline". Multi-platform exporter compatibility narrows with "Linux-only base CI". The redaction trigger is covered under Scrubbing above.
  - SLO invariant (§10 zero unlogged panics) → confirmed. The gate is kept in Epoch 1, re-fed in Epoch 3, and closed by "Version close — … full gate green on Linux" (Epoch 8). Minimal tier carries no error budget.
  - Run-record conformance gate (§9/§10) → confirmed. Its assertions are re-based by "Run record for the one form". Its CI journal comes from the same producer step that "Own-log gates on the one form" re-subjects (`ci.yml:185-234`; the gate reads every journal under `CONDUCTOR_RUNS_DIR`, `/home/turbolet/dev/projects/conductor/crates/conductor-run/tests/journal_conformance.rs:217`), so it keeps a subject past Epoch 5 without a line of its own.
  - §11 bans → confirmed. No entry adds a self-observation exporter, SDK, metrics backend or trace context. "Engine memory and database growth read through the door" reads the engine's own state as a product reading, not Conductor's self-observation.

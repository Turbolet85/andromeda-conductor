# Scope — Fault-application spans

**Marker:** `2026-08-16-fault-application-spans`
**Version:** conductor-0.2.0 · **Epoch 3** — Live proof: the five families
**Working entry:** _Fault-application spans — silence, ramp and port-occupier phases observable beneath the
timeline span with duration and journal offset_

## What this chunk builds

Self-observation spans for the three fault-application kinds, so a scenario run's fault phases are visible
in `logs/agent-latest.jsonl` beneath the run's existing span tree rather than being inferable only from the
emission journal's gaps.

The obs-plan already specifies the shape exactly (`obs-plan.md:379-393`, and the bounded span-name set at
`:587`) — this chunk is the implementation of a written contract, not a design:

| Span | Attributes |
|---|---|
| `fault.silence` | `fault_type = "silence"` · `fault_duration_ms` (int) · `fault_start_offset_ms` (journal offset) |
| `fault.ramp` | `fault_type = "ramp"` · `fault_duration_ms` · `fault_start_offset_ms` · `ramp_factor` (float) |
| `fault.port_occupier` | `fault_type = "port_occupier"` · `fault_duration_ms` · `fault_start_offset_ms` · `port` (int) |

Each span wraps its fault-application window and closes on fault release; all three sit **beneath
`timeline.execute`** in the span tree (obs-plan `:393`).

## Boundaries

- **Self-observation only.** These are `tracing` spans rendered as JSON by `JsonObsLayer` — never OTLP,
  never an OTel SDK. The only OTLP is the product fault stream to Pulse `:4317`.
- **No new fault behavior.** The chunk observes what the fault phases already do; it does not change
  emission shape, phase timing, seeded determinism, or the port-occupier's bind/hold/release lifecycle.
- **No new span names beyond the three.** The bounded set at `obs-plan.md:587` already reserves exactly
  `fault.silence` / `fault.ramp` / `fault.port_occupier`.
- **Journal-relative offsets, `std::time` stamps.** `fault_start_offset_ms` is a journal offset, so its
  basis is the same `std::time` clock the journal uses — never tokio's virtual clock.
- Not a live-Pulse leg: this is observable from a headless run with no SUT (the a11y/GUI surfaces are
  untouched).

## Surfaces and contracts touched

- `crates/conductor-faults/` — the fault helpers (`EmissionGap` · `AbruptSilence` · `PortOccupier` ·
  `BurstyTrain`); today the crate emits **zero** `tracing` calls and has no `tracing` dependency.
- The run/dispatch path where fault phases actually execute during a scenario — `conductor-run`
  (`dispatch.rs` / `lib.rs`) beneath `conductor-timeline`'s `timeline.execute` span (`scheduler.rs:72`).
- `crates/conductor-core/src/redact.rs` — the tracing field allowlist. A span attribute not on the
  allowlist is **dropped silently** by `JsonObsLayer` (`obs.rs:327-363`), so the five new attribute names
  are part of the deliverable, not an afterthought.
- `logs/agent-latest.jsonl` — the agent-mode self-obs artifact these spans must appear in, and the CI
  obs-conformance gate that reads it.

## Scope premises, closed at P3 (see `research.md`)

- **VERIFIED — the three fault kinds are not one code path today.** Silence is declarative phase data
  (`EmissionSpec { occurrences: 0 }`, `phase_spec.rs:60-62`), ramp is
  `EmissionShape::Ramp { from_rate, to_rate, windows }` realized per emission at `dispatch.rs:139-145`,
  and the port-occupier is a `conductor-faults::PortOccupier` RAII binder. Three distinct sites, not one
  helper.
- **VERIFIED and sharpened — `conductor-faults` is an island, not merely un-depended-on by
  `conductor-run`.** The code-graph returns **0 crate edges** in either direction and **0 references** to
  `PortOccupier` / `AbruptSilence` / `EmissionGap` / `BurstyTrain` from outside the crate; all 29
  `PortOccupier` refs are its own `src/` + `tests/`. `phase_spec.rs:60-62` states the design intent
  outright: `occurrences: 0` is how a quiet phase is expressed "without reaching for a fault helper". So
  instrumenting the helpers alone would put spans on a path **no scenario run reaches**.
- **VERIFIED — `ramp_factor` has no existing source.** obs-plan §4 names a float `0.0-1.0`; the shipped
  shape carries three integers. A derivation must be defined and stated, not guessed (P4 fork).
- **[premise-corrected: the scheduler has no per-phase hook at all — `run_timeline_with` calls `on_emit`
  once per declared occurrence and a phase with `emissions == 0` calls nothing (`scheduler.rs:97-107`,
  `:122-125`)]** — the original bullet asked only whether `fault_duration_ms` is knowable at span open.
  It is (the jittered gap is computed at `scheduler.rs:96` before the phase's first sleep), but the
  binding constraint is upstream of that: **there is currently no place in the run path where a fault
  phase's window can be wrapped.** `Phase` carries `name`/`gap`/`emissions` only and `convert.rs:22-27`
  discards the shape, so the scheduler is shape-blind by design. Opening these spans requires a
  deliberate seam decision (P4 fork), not just an attribute computation.
- **[premise-corrected: `fault.port_occupier` has no run-path application to wrap, and the work of
  creating one is already owned elsewhere]** — `scenarios/receiver-failed-port-conflict.toml:8` names an
  "Epoch-8 driver" that the graph proves does not exist, and matrix `v2-15` assigns
  "the `:4317` port-occupier drives the receiver-failed state and releases its bind on cleanup" to the
  **connection-lifecycle live proof** entry later in Epoch 3. Wiring it here would be the new fault
  behavior this scope excludes.
- **VERIFIED — this chunk links no capability.** No `verification-matrix.json` entry becomes fully
  verifiable; the nearest (`v2-15`, `dynamic-external`) needs a live-Pulse family leg, which this
  headless instrumentation chunk is not. Expect 0 capabilities linked at P5.

## Folded annotations from the working entry

- **CARRY (from `2026-08-13-dispatcher-determinism-goldens`) — ride-along, explicitly _not_ a work item.**
  Align one stale doc comment at `crates/conductor-timeline/src/scheduler.rs:69`: it states "Total elapsed
  time per phase is exactly the jittered gap either way", true of the reported `PhaseTransition` stream but
  **not** of clock consumption — `paced_slices` divides a gap into sub-millisecond slices and `tokio::time`
  rounds each sleep up to its 1ms tick, so emissions overshoot the window by up to one tick each and the
  excess carries forward (measured: phase 0 boundary 12010ms, last emission 12012ms across 6 emissions).
  The true bound is already asserted by
  `crates/conductor-timeline/tests/pacing.rs::each_phase_last_emission_sits_on_its_boundary_within_timer_resolution`.
  All seven masters grep clean of the claim and wrap does not edit source — hence it rides the next
  timeline-touching chunk, which is this one.

- **PREREQ — re-check `cargo audit` (22nd pin; the BASIS CHANGED at the previous chunk).** Standing
  deferral since `2026-08-08-sut-capability-manifest`, ratified by the operator at the
  `2026-08-10-workspace-key-divergence-probe` wrap (re-pins silently, no further ratification HALT). Fault
  class: **advisory-DATABASE** — byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1,
  re-measured last chunk on cargo-audit 0.22.2 (the latest published), so **no released tool version can
  read the DB and there is no floor to raise**. What changed: `Cargo.lock` MOVED — `blake3` 1.8.6 plus
  `arrayref` / `arrayvec` / `constant_time_eq` / `cpufeatures` landed as normal dependencies of
  `conductor-emit` — so the deferral can no longer rest on "no dependency delta" and the tree is no longer
  static. **`cargo deny check advisories bans licenses sources` is now the SOLE coverage for the added
  packages and must be VERIFIED green, never assumed** (security-plan §Dependency Security admission
  condition; playbook rule appended 2026-08-16). Do NOT raise a floor, do NOT add a `deny.toml` ignore, do
  NOT edit CI. Close the deferral the moment it parses. Full rationale:
  `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.

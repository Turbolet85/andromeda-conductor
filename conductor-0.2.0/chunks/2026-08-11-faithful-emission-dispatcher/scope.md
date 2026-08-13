# Scope — Faithful emission dispatcher

**Marker:** `2026-08-11-faithful-emission-dispatcher`
**Version:** conductor-0.2.0 · **Epoch 2 — Live-path enablement**
**Working entry:** _Faithful emission dispatcher — per-phase telemetry shaping over the emit primitives, replacing one-signal-per-phase_
**Matrix cap (candidate):** `v2-08` — Faithful per-phase emission dispatcher

---

## What this chunk builds

A per-phase **emission dispatcher** that replaces `conductor-run::coarse_emit` — today one OTLP
signal per phase boundary — with a path that emits each phase's **declared shape** over the
`conductor-emit` primitives that already exist and are already tested.

The emit seam is not the gap. `conductor-emit` already exposes ten primitive families
(`rate_trace_request`/`RateCurve` · `latency_trace_request`/`LatencyProfile` ·
`exception_trace_request`/`ExceptionSpec`/`FingerprintVariant` · `severity_logs_request`/`Severity` ·
`pii_trace_request`/`pii_logs_request`/`PiiCorpus` · `error_trace_request`/`ErrorPlacement` ·
`service_topology_request`/`ServiceTopology` · `trace_request`), each landed and unit-proven in
Epoch 1–2 chunks. **The gap is that nothing calls them from the run path**: `coarse_emit`
(`crates/conductor-run/src/lib.rs:340`) reaches for exactly two — `trace_request` and a hardcoded
`severity_logs_request(Severity::new(9))` — and picks between them on `EmissionSpec.signal` alone.

This chunk builds the dispatch layer between the phase model and those primitives, so a scenario's
declared shape reaches the wire.

## The shape problem this chunk must resolve first

The scenario model does not currently carry the shape the dispatcher would dispatch on.

- `EmissionSpec` (`crates/conductor-core/src/phase_spec.rs:45`) has **exactly one field**:
  `signal: Signal`, a closed three-variant enum (`Traces`/`Metrics`/`Logs`). There is no rate, no
  occurrence count, no error fraction, no severity mix, and no exception/fingerprint shape.
- Across the **35** committed scenario TOMLs there are **3 emission declarations total** — all of
  them `emission = { signal = "logs" }` (`high-severity-log-capture.toml:18,:23` ·
  `pii-scrub.toml:35`). Every other phase in the catalog defaults to `Signal::Traces`.
- The shape the scenarios actually intend is encoded in **phase names**: `storm-same-fp-6x`,
  `storm-same-fp-12x`, `suppressed-15s-burst`, `sustained-60s`, `bypass-relative-12x-4pct`. This is
  a deliberate, documented convention — `fingerprint-storm.toml` states it outright: _"EmissionSpec
  carries no occurrence-count field, so the Epoch-8 driver realizes the per-phase count + variant
  mix from the phase names (the restart-suppression precedent)."_

So "the phase's declared emission spec" names something that is **half-declared in TOML and
half-encoded in phase names**. Reconciling that is inside this chunk, not before it. `[inferred]`
The route entry says "per-phase telemetry shaping"; it does not say which side of that split the
declaration should end up on. **This is the chunk's primary design fork and is carried to P4 as an
explicit operator question** (extend the declarative model · realize from the authored phase-name
convention · hybrid).

**Resolved at P4 (operator):** extend the declarative model — `EmissionSpec` gains an occurrence
count and a discriminated per-family shape, validated by garde, and the catalog migrates so phase
names become labels rather than the carrier. Two further forks were resolved with it: the dispatcher
emits **during** the timeline walk (not after it), and the chunk covers **all** families, so it
claims `v2-08` in full.

## Boundaries

**In scope**
- The dispatcher itself: phase → primitive selection + invocation, over the existing
  `conductor-emit` public API, replacing `coarse_emit`'s two-primitive switch.
- Whatever model or convention change the fork above resolves to, including its garde validation
  and any scenario-TOML migration it implies. `[inferred]`
- Extensibility to the deferred families named by the cap — the dispatcher must have a place for
  fingerprint-storm / error-baseline-spike / restart-suppression / pii-scrub / connection-lifecycle
  shapes without a redesign, since Epoch 3's five live-proof chunks each depend on this seam.
- Emission counting sufficient to satisfy the `emission_count` obligation folded in below.

**Out of scope**
- **Determinism goldens** — the immediately following working entry (_Dispatcher determinism
  goldens_) owns freezing the stream shape under a seed. This chunk must not weaken determinism,
  but proving it by golden is the next chunk's job. `[inferred]`
- **Read-back / verification** — `execute_scenario`'s read-back and `evaluate_check`→`classify`
  path is untouched here; real per-check extraction is a later working entry (`v2-09`).
- **Live-Pulse proof** — no live leg. The five family live-proofs are Epoch 3; a live green
  preflight is its own later entry (`v2-10`).
- **New emit primitives** — the primitives exist; this chunk consumes them. Any gap discovered
  between a scenario's intent and the available primitives is reported, not silently filled.
- **Scenario catalog semantics** — P-ID assignments, expected checks and SLO tiers stay as
  authored; only emission shape is in play.

## Surfaces and contracts touched

- `crates/conductor-run/src/lib.rs` — `coarse_emit` is the replaced call site; `execute_scenario`
  is its only caller and its public signature must survive unchanged (`v2-09` depends on it).
- `crates/conductor-emit/` — consumed as-is via its public API (`lib.rs:23-32`); no new primitives
  expected.
- `crates/conductor-timeline/` (`scheduler.rs` · `phase.rs` · `convert.rs`) — **added at P5 after
  validation-1 (intent-incomplete, not a defect).** The scope as first written did not list this
  crate, because it assumed the dispatcher could shape emission entirely from `conductor-run`.
  Research falsified that: `run_timeline` sleeps through every gap and *returns* — closing
  `timeline.execute` — before `coarse_emit` is ever called, so emissions reach the wire in a burst
  after the scenario's timing is spent, and `emit.batch` is a sibling of `timeline.execute` rather
  than the child obs-plan §4 CP1 describes. The operator resolved the fork at P4 in favour of
  emitting **during** the walk, which requires a caller-supplied per-boundary hook on the scheduler
  and a declared occurrence count on `Phase`. `conductor-timeline` must still gain **no**
  `conductor-emit` edge — the hook stays generic.
- `crates/conductor-core/src/phase_spec.rs` — `EmissionSpec`/`Signal`, if the fork extends the
  model; garde validation lives with it. `[inferred]`
- `scenarios/*.toml` — 35 files; touched only if the fork moves shape into declarations. `[inferred]`
- **Determinism invariant** — same scenario + seed ⇒ same stream shape. The dispatcher is now the
  main shaper of that stream, so any randomness it introduces must come from the seeded
  `ChaCha8Rng`, never from a clock or an unordered iteration.
- **Load envelope** — `contracts/pulse-load-envelope.toml`. Its rate terms are currently recorded
  *declared-not-derivable* **because the scenario model carries no rate or occurrence-count field**
  (architecture §Occupied Resources). If the fork adds one, that justification stops holding and the
  envelope's assertion surface changes. Surface this consequence; do not silently widen
  `check_load_envelope`. `[inferred]`
- **Self-observation** — `timeline.execute` / `emit.batch` spans (obs-plan §4 CP1).

## Folded-in annotations from the working entry

**CARRY (from `2026-08-10-scenario-run-root-span-tree`) — the post-open span attribute trap.**
`JsonObsLayer` records span attributes on the `on_new_span` record ONLY and implements no
`on_record`, so `tracing::field::Empty` plus a later `Span::current().record(...)` vanishes with no
error — the same silent-field-loss class the previous chunk fixed one layer down. It bites here
specifically: obs-plan §4 CP1 (`obs-plan.md:298`) requires `timeline.execute` to carry
`phase_count` **and** `emission_count`; the shipped span carries only `phase_count`
(`scheduler.rs:34`), and `emission_count` is knowable only AFTER the phase has emitted — which is
exactly what this chunk produces. Resolve deliberately: either implement `on_record` (deciding
whether a late value amends the `new` record or emits its own line) or compute the count before
opening the span. **Do not discover this by watching a field disappear.**

**PREREQ (from `2026-08-10-scenario-run-root-span-tree`) — the `cargo audit` re-check, 10th
consecutive.** Deferred since `2026-08-08-sut-capability-manifest`; **RATIFIED** by the operator at
the 2026-08-10 wrap under the L5 age trigger, so it re-pins **silently** from here with no further
ratification HALT. Last re-proven on cargo-audit **0.22.2** (the latest published): byte-identical
`duplicate advisory ID: RUSTSEC-2026-0244`, true exit 1 — an advisory-**DATABASE** fault with
nothing to raise a floor to. The deferral's standing basis is **"audit SURFACE unchanged (no new
`[[package]]`) + `cargo deny check` verified green"**, not "zero dependency delta". Remedy is the
**bounded wait alone**: re-run it, record the result, and verify `cargo deny check` ran green as the
overlapping signal. Do NOT raise the floor, do NOT add a `deny.toml` ignore, do NOT edit CI. Close
the deferral the moment it parses. (`playbook.md` external-decay · `.claude/rules/security.md`
2026-08-09/-08-10 · security-plan §Dependency Security.)

**Note for this chunk specifically:** if the dispatcher's resolution pulls a new dependency, the
"audit surface unchanged" basis no longer holds literally and the deferral's basis must be
re-stated at wrap rather than re-pinned verbatim. `[inferred]`

## Acceptance anchor (val-1 reference)

The chunk is aligned with its intent when, for each scenario family, the emitted stream matches the
phase's declared emission shape — rate, error fraction, severity mix, exception/fingerprint shape —
rather than a single signal per phase, with `execute_scenario`'s public signature unchanged and the
seeded determinism invariant intact.

# Scope — 2026-08-19-connection-lifecycle-live-proof

**Working-route intent (verbatim):** connection-lifecycle live proof — Listening/Receiving/Idle/Stalled
walk plus the :4317 port-occupier leg (P-001..P-004)

**Capability claim target:** v2-15 "connection-lifecycle family live-proven" (`dynamic-external`; current
outcome-level acceptance: the Listening/Receiving/Idle/Stalled walk produces each expected state transition
within its SLO, and the :4317 port-occupier drives the receiver-failed state and releases its bind on
cleanup). Claimable ONLY in this chunk — the CARRY below is why.

## What this chunk builds

1. **The port-occupier driver (CARRY from 2026-08-16-fault-application-spans, folded in).** There is no
   port-occupier driver anywhere — this chunk WRITES one, it does not merely invoke one. The code-graph
   measured `conductor-faults` as an island: 0 crate edges in either direction and 0 references to
   `PortOccupier`/`AbruptSilence`/`EmissionGap`/`BurstyTrain` from outside the crate, so the "Epoch-8
   driver" named at `scenarios/receiver-failed-port-conflict.toml:8` does not exist, and that scenario's
   `port-held` phase currently emits one plain span while nothing binds `:4317`. Folded consequences:
   - The run→faults crate edge does not exist either; adding it is an explicit `Cargo.toml` dependency
     edge under the crate-per-seam rule — never a bin-local shim.
   - The `fault.port_occupier` span already ships (RAII at `occupy`/`release`, carrying `fault_type` +
     `port`, proven by `crates/conductor-faults/tests/port_occupier.rs`); obs-plan §4 records its
     `timeline.execute` parentage as CONDITIONAL because nothing runs it beneath the timeline. Wiring the
     driver is what makes that parentage real; the §4 wording becomes unconditional in the same wrap (a
     wrap-owned amendment — recorded here so wrap sees it, never a phase edit).

2. **The live proof (P-001..P-004).** Drive the family against a live deterministic-L4 Pulse on fresh data
   dirs (operator convention: leg dirs under `%TEMP%/pulse-legs/`): the Listening/Receiving/Idle/Stalled
   state walk plus the orthogonal :4317 port-occupier receiver-failed leg, with journal + runs.db evidence
   and the run-report envelope intact.
   - [determined at P3] Connection state reaches NO MCP read-back surface: the FSM lives in
     `andromeda-pulse crates/ingest/src/connection.rs` and surfaces only via the TauRPC
     `connection.current_state` resolver, the `pulse://stream/connection-state` broadcast, and tracing
     lines; no `mcp-server` tool carries it, and `Observation.text` (what `Contains` grades against)
     composes incident-list + report text only. The family's four `Contains` checks
     ("Stalled"/"Receiving"/"ReceiverFailed"/"Idle"+"error") are therefore structurally ungradeable →
     the declare-only + harvest route (the fingerprint-storm/error-baseline/pii-scrub precedent), with
     the live claims graded over Pulse's own `connection.state.transition` /
     `app.boot.otlp.grpc.bind` / `connection.tick` lines. The v2-15 acceptance is concretized (never
     weakened) at P4/P5 under the operator preview.
   - [verified at P3] The four TOMLs need re-basing: three lack `[phases.emission]` tables entirely
     (P-004's "high error rate" exists only in comments — the phase emits the default 1 plain span);
     all four retire their read-back checks; tiers re-calibrate from measured leg latency
     (latency ≈ scenario duration, the v2-11/v2-12 precedent).

3. **PREREQ — the 31st consecutive `cargo audit` re-check (compact ratified form).** Standing deferral
   since `2026-08-08-sut-capability-manifest`, ratified at the `2026-08-10-workspace-key-divergence-probe`
   wrap, re-pins silently. Re-verify the basis: `cargo audit` true exit 1 byte-identical
   (`duplicate advisory ID: RUSTSEC-2026-0244`, probed standalone) + the overlap VERIFIED, never
   assumed (`cargo deny check advisories bans licenses sources` true exit 0). Remedy is the bounded
   wait alone — no floor raise, no `deny.toml` ignore, no CI edit; close the moment it parses.
   [premise-corrected at P3: the annotation's "ZERO dependency delta / Cargo.toml untouched" clause
   described the PRIOR chunk's footing and cannot re-pin as-is — THIS chunk adds the run→faults
   `Cargo.toml` edge (a `Cargo.lock` member-dep row, zero new external crates), so the 31st pin
   restates its basis on the changed footing with deny verified green over the NEW lock — the
   2026-08-16 admitting-a-dependency-under-red-audit precedent (security extract §Relevant amendment
   history).] Full rationale:
   `conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.

## Boundaries
- Conductor-side only. Pulse gaps discovered live travel via the overseer's Pulse-visit intake queue,
  never this route's tail (operator directive, pii-scrub wrap).
- Scope law holds: the :4317 port-occupier remains the SOLE deliberate inbound bind, RAII
  bind→hold→release on cleanup; no listener widening, loopback only.
- Determinism holds: the driver runs beneath the seeded `current_thread` timeline (same scenario+seed ⇒
  same stream shape); journal stamps from `std::time`, never tokio's virtual clock.
- Verdict/error wall holds: a refused occupier bind is already the typed `FaultError::Bind` value
  (`port_occupier.rs:47`), never a panic. The driver-level policy for a failed occupy on a
  fault-declared phase (typed warn + continue vs harness Err) is P4's one open plan decision;
  research leans warn + continue (an operator-timing slip must not discard a live run's artifacts,
  and the missing `fault.port_occupier` span + absent harvest witnesses keep non-application visible).
- No scenario without a P-ID; the accepted set stays manifest-driven (P-001..P-004 are accepted).

## Surfaces expected to move (confirmed at P3; enumeration in research.md §Files to modify)
- `crates/conductor-core/src/phase_spec.rs` — `PhaseSpec.fault: Option<FaultSpec>` (serde default,
  garde dive) + the closed `FaultSpec` kind model; NO dispatcher/EmissionShape change — the timeline
  hook (`run_timeline_observed`) is already generic over the phase guard, so the occupier rides the
  existing `on_phase` closure as a guard member [premise-corrected: the "dispatcher lacks a hook"
  reading was wrong — the hook exists and is guard-generic; the missing piece is the phase-level
  fault DECLARATION plus the conductor-run guard struct].
- The 13 `PhaseSpec` literal-constructor sites across 6 files (graph-enumerated) — `fault: None`.
- `crates/conductor-run/Cargo.toml` (+ `Cargo.lock` member-dep row; zero new external crates) and
  `crates/conductor-run/src/lib.rs` (the `PhaseGuard` + occupy-at-phase-open driver).
- All FOUR family TOMLs: `receiver-lifecycle-state` · `receiver-failed-port-conflict` (re-based to
  the 3-phase operator choreography) · `last-span-ago-tracking` · `orthogonal-health-domains`.
- `crates/conductor-core/src/scenario.rs` — catalog-guard re-bases (family declare-only guard +
  expected-count/suite guards naming these four).
- NEW `crates/conductor-run/tests/connection_harvest.rs` — the harvest test (pii_harvest mold).
- `conductor-0.2.0/verification-matrix.json` — the v2-15 claim at P5.

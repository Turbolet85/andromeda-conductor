# P-025 hue-shift measurement contract

**A request to Pulse.** P-025's ≤2 000 ms hue-shift latency budget is delegated to Conductor to grade, and
Conductor cannot grade it: the observable Pulse emits today does not measure the quantity the budget bounds.
This document states what Pulse would need to emit for the bound to become measurable — the observable, its
resolution, its window, and the comparison that would constitute a hard grade — in a form that can be
implemented without a follow-up question to Conductor.

    sut_version  = "v0.3.0"
    captured_at  = "2026-09-13"
    pinned_at    = "Pulse HEAD 83d4060"
    provenance   = "Conductor MEASUREMENT, not a transcribed SUT record — the 2026-08-21 and 2026-09-07
                    live legs, with every cited coordinate re-read at the pinned HEAD on 2026-09-13"

Read the provenance line as a bound on trust: the measurements are Conductor's own and are reproducible from
its committed captures, while every Pulse coordinate cited below is a reading of the SUT at the pinned HEAD
and will need re-checking if that moves. Nothing here asserts a property of Pulse that Conductor cannot
observe from outside it.

## The observable

Pulse already emits `metric.constellation.hue_update_ms`, carrying `duration_ms` and `severity_tier`. The ask
is **not a new leaf** — it is a corrected `duration_ms` on this one, computed over the window in §The window.
If Pulse prefers a new target rather than changing the meaning of an existing one, the contract is equally
satisfied by a sibling leaf carrying the same two fields, provided the old one is retired or documented as
superseded so a reader cannot grade the wrong one.

Two properties of the emission are part of the ask, not incidental:

- **State the field name literally.** Field names in the delegated-timing family are not uniform —
  `duration_ms` on `metric.constellation.hue_update_ms`, `metric.constellation.discovery_ms` and
  `metric.findings.counter_refresh_ms`, but `value` on `metric.report.render_ms`. Conductor reads each leaf
  from its own exact name, so whichever field carries the duration must be stated rather than assumed.
- **Any added field must be admitted to that leaf's allowlist entry.** Pulse runs a default-deny field
  allowlist on its own tracing lines; a field the allowlist does not name arrives redacted on the wire and is
  unreadable even though it was emitted. The measured precedents are `triage.cue.tick` and
  `triage.incident.auto_resolve.tick`, whose counters render as the redaction placeholder for exactly this
  reason. The leaf's current admitted set is `duration_ms` and `severity_tier`, asserted by Pulse's own test
  at `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs`, whose header names the emit sites as
  the authority for those lists.

## The resolution

**Millisecond resolution on both instants**, with the emitted duration carrying at least millisecond
precision.

The present effective resolution is not milliseconds — it is the 15 s lifecycle tick. The value the canvas
computes is offset from `ServiceRegistryEntry.last_seen_unix_nano`, which in steady state is written only by
the lifecycle heartbeat (`DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL`, `crates/triage/src/lifecycle/mod.rs`), so the
instrument's granularity is 7.5× the budget it is asked to grade. A budget of 2 000 ms cannot be graded by an
instrument whose quantisation step is 15 000 ms; that ratio, rather than any individual reading, is why the
present observable is unusable.

## The window

The duration is the interval between two instants, both internal to Pulse:

- **start — `tier_effective_at`:** the instant the service's maximum priority tier last changed VALUE.
- **end:** the paint of the new hue on the constellation dot — the instant the canvas effect already
  observes, when it detects that a dot's tier differs from the tier it last recorded.

**The start instant needs to be exposed, not invented.** Pulse computes the tier at
`pulse-app/src/services_router.rs`, where `list_with_states` filters `incident_registry.list_active()` to the
service's own scope and takes `.max_by_key(tier_rank)`. That site holds the full `Incident` records at the
moment it decides the tier, so both timestamps the rule below needs are already in hand there. What the canvas
receives — `ServiceListItem` — carries no timestamp for `priority_tier` at all, which is why the existing
effect reached for `last_seen_unix_nano`: it is the only per-service instant the item offers.

**The rule, in both directions.** The service's maximum tier can change in exactly two ways, and
`tier_effective_at` is defined for each:

1. **The maximum RISES** — an incident opens at a tier above the current maximum. `tier_effective_at` is that
   incident's `opened_at_unix_nano` (`crates/triage/src/contract.rs`).
2. **The maximum FALLS** — every incident holding the current maximum leaves the active set, by resolution or
   acknowledgement. `tier_effective_at` is the `transitioned_at_unix_nano` of the last one to leave
   (`crates/triage/src/incident/broadcast.rs`).

**Those two cases are exhaustive, and here is why.** An incident's `priority_tier` is immutable after opening,
so there is no third case in which a tier changes in place. Measured at the pinned HEAD: every write of
`priority_tier` across `crates/` and `pulse-app/src/` is a construction site or a DTO projection — eight in
total — and the single assignment among them is the one at `services_router.rs` that sets the DERIVED
`ServiceListItem` field. No persistence UPDATE touches the column either; the incident update statements in
`crates/corpus/src/contract.rs` set `status`, `updated_unix_nano`, `resolved_unix_nano`, `payload` and
`read_unix_nano` only. The maximum therefore moves only when the active SET moves, which is what closes the
rule at two cases.

**The subject is a value Pulse produces.** This is load-bearing rather than stylistic. Conductor's own
`latency_ms`, and the per-check `budget_ms` / `effective_deadline_ms` that bound it, measure
`read_back_observed_at − journal_emitted_at` — Conductor's journal-relative span over a scenario's whole
emission window. They are excluded subjects here: binding a Pulse-internal pipeline latency to them would
grade Conductor's own emission timing and call it Pulse's hue latency.

## The hard grade

`duration_ms ≤ 2000` for every sample attributable to the driven service, graded **hard**.

- **A breach is a hard `Fail`.** A delegated timing bound is a deterministic claim, so it takes the hard
  pass/fail disposition rather than being routed to `CalibrationRegion`, which is reserved for
  model-interpretive outcomes reported for a human.
- **The grade is an operator-gated live claim, never a CI gate.** It is graded at the harvest tier, over
  Pulse's own emitted line captured from a live leg — not through any Conductor-side check. It must not be
  satisfiable by the CI stub, which proves MCP wiring only and can never stand in for Pulse's reaction.
- **Until the observable lands, the delegated budget's verification is `Blocked`**, carrying this contract as
  its named precondition — never `Fail`, which would assert a measurement that was taken, and never
  `KnownResidual`, which would assert a measured-but-accepted gap. Nothing has measured the hue-shift
  latency. The scenario's own run row is a separate matter and is unaffected: `halo-hue-encoding` is
  declare-only and reaches `KnownResidual` by the ordinary degraded read-back path.

## Why the present observable cannot carry the bound

Three properties of the current fire site each defeat the measurement independently, so a replacement has to
escape all three rather than any one. All three were re-read at the pinned HEAD.

1. **Staleness, not latency.** The canvas computes the emitted duration as the difference between the render
   time and the service's `last_seen_unix_nano` (`pulse-app/ui/src/widget/ConstellationCanvas.tsx`). That is
   how stale the service's telemetry was when its hue changed — not how long the hue change took. The two
   quantities coincide only when the service was seen essentially at the moment of the flip.
2. **Slowest-wins aggregation.** The effect loops every dot whose tier changed in the render pass and emits
   the MAXIMUM staleness across them, tagging it with that dot's `severity_tier`. A same-pass change on an
   unrelated service therefore reports that service's number under this one's tier. The leaf carries no
   service identifier — the allowlist is aggregate-only — so a consumer cannot separate them after the fact.
3. **Tick quantization.** `last_seen_unix_nano` has no ingest-path writer. In steady state it is written only
   by the lifecycle tick's refresh, itself gated on the service's quiet duration being zero, where that quiet
   duration is computed in integer-truncated seconds
   (`crates/triage/src/lifecycle/registry.rs`, `crates/triage/src/baseline/activity_floor.rs`). The emitted
   value is therefore the offset from the last refreshing tick, distributed uniformly over the 15 s interval
   and independent of how fast the service is emitting.

**What that costs, measured.** On 2026-08-21 two independent legs read 35 581 ms and 36 705 ms against the
2 000 ms budget. The natural reading — a slow pipeline — is wrong, and the 2026-09-07 re-drive settles it: the
scenario emitted 360 dispatches at 2 per second so the service never went quiet through its tier flip, and its
one in-window sample still read 14 525.9 ms, matching its offset to the preceding tick (14 527 ms) to 1.1 ms.
All seven samples in that capture fit `duration = offset_to_preceding_tick + k × 15 000` with k ∈ {0, 1, 2}
within 2 ms. Meeting 2 000 ms through this instrument requires the flip to land in the first 2 s of a 15 s
window — roughly a 13 % coincidence, at any dispatch rate. The second reading is recorded raw as
36 704.983642578125 in Conductor's committed fixture
(`crates/conductor-run/tests/delegated_timing_harvest.rs`); it is rounded to 36 705 ms everywhere else,
including here.

## Why the two recorded fix candidates are insufficient

Conductor's own artifacts have named two SUT-side changes as lifting this bound: having the service registry
copy the activity floor's `last_observed_unix_nanos`, or having the fire site read span arrival directly.
Both are real and implementable — `last_observed_unix_nanos` exists and is stamped on each observation
(`crates/triage/src/baseline/activity_floor.rs`) — and **neither satisfies this contract.**

Both address term 1 only. With `last_seen_unix_nano` stamped from ingest, a service emitting at 2 per second
would report roughly 500 ms, the 2 000 ms budget would start passing, and the emitted quantity would still be
*the age of the newest span at paint time* rather than the hue-shift latency. That is the worst available
outcome: a green grade over the wrong quantity, indistinguishable from a real one at the point of reading, and
with the two aggregation and attribution defects of terms 2 and 3 still in place beneath it.

The contract is satisfied by supplying `tier_effective_at` per §The window, not by making the existing
staleness number smaller.

## What a complete implementation looks like

For the avoidance of a follow-up question, an implementation satisfying this contract does all of:

1. Exposes the tier's effective instant on the payload the constellation canvas already receives, per the
   two-case rule in §The window.
2. Computes the emitted duration as the paint instant minus that value, at millisecond resolution.
3. Names the carrying field explicitly and admits it to the leaf's allowlist entry.
4. Emits per changed service rather than a slowest-wins maximum across the pass, or else carries a service
   identifier so a consumer can attribute the sample.

Item 4 is what closes term 2. Items 1 and 2 together close terms 1 and 3, because a value anchored to a
backend-stamped tier instant never reads the lifecycle tick at all.

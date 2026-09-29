# P-025 hue-shift measurement contract

**A request to Pulse, now satisfied, and the rule Conductor grades it by.** P-025's ≤2 000 ms hue-shift
latency budget is delegated to Conductor to grade. When this document was written (2026-09-13, Pulse HEAD
`83d4060`) Conductor could not grade it: the observable Pulse emitted then did not measure the quantity the
budget bounds. The document stated what Pulse would need to emit for the bound to become measurable — the
observable, its resolution, its window, and the comparison that would constitute a hard grade — in a form
that could be implemented without a follow-up question to Conductor. Pulse's P-025 change `e98d838` did that.
The document now records the shipped observable, keeps the retired instrument's measurements as history, and
states the grading rule in §The grading rule before any leg is driven against it.

    sut_version  = "v0.3.0"
    captured_at  = "2026-09-29"
    pinned_at    = "Pulse HEAD 226554a"
    provenance   = "MIXED, per clause. Every Pulse coordinate below is a TRANSCRIBED SUT RECORD — a reading
                    of the SUT's own committed source at the pinned HEAD (which carries the P-025 fix
                    e98d838), read by `git show`, never the working tree. The 2026-08-21 readings are
                    CONDUCTOR MEASUREMENTS against Pulse HEAD f0c38f5 and the 2026-09-07 readings are
                    CONDUCTOR MEASUREMENTS against Pulse HEAD 83d4060, each reproducible from Conductor's
                    committed captures. The graded leg's reading, once measured, is a CONDUCTOR MEASUREMENT
                    against 226554a, recorded in its chunk evidence."
                   [re-pinned 2026-09-29 (2026-09-29-hue-shift-budget-graded-hard, before the drive): the
                    prior pin was Pulse HEAD 83d4060, captured 2026-09-13, with provenance "Conductor
                    MEASUREMENT, not a transcribed SUT record". Every coordinate was re-read at 226554a; six
                    claims no longer held as written and carry dated corrections below.]

Read the provenance line as a bound on trust: the measurements are Conductor's own and are reproducible from
its committed captures, while every Pulse coordinate cited below is a reading of the SUT at the pinned HEAD
and will need re-checking if that moves. Nothing here asserts a property of Pulse that Conductor cannot
observe from outside it.

**What now carries the bound.** A backend-stamped tier instant (`ServiceListItem.tier_effective_at_unix_nano`)
read against the webview's `Date.now()`, both on one host wall clock; one sample per changed service; and only
for changes the canvas witnessed. The duration is the paint instant minus the instant the service's maximum
incident tier last changed (`pulse-app/ui/src/widget/constellation-types.ts:245-276`,
`crates/triage/src/incident/tier_effective.rs:23-63`).

## The observable

Pulse already emits `metric.constellation.hue_update_ms`, carrying `duration_ms` and `severity_tier`. The ask
is **not a new leaf** — it is a corrected `duration_ms` on this one, computed over the window in §The window.
If Pulse prefers a new target rather than changing the meaning of an existing one, the contract is equally
satisfied by a sibling leaf carrying the same two fields, provided the old one is retired or documented as
superseded so a reader cannot grade the wrong one.

[satisfied at `e98d838`, verified at `226554a`: the leaf keeps its name and both fields, and only the value's
meaning changed. Emit site `crates/ui-bridge/src/telemetry.rs:278-282`; allowlist entry
`pulse-app/src/observability.rs:989-992`. The ceiling is now also a clamp: `canvas/frame-metrics.ts:28-40`
sets a value over 60 000 to 60 000, and `telemetry.rs:75,84` rejects one over 60 000.]

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
  [corrected 2026-09-29 (f): the precedent sentence is history. Both leaves are FULL at `226554a`
  (`observability.rs:1373-1385`, `:2398-2404`) and already were at `83d4060`; the redacted renderings were
  measured on 2026-08-18 and 2026-08-21 against an older HEAD. The mechanism the sentence illustrates is
  unchanged. No field was added to the hue leaf, so no admission was needed; the allowlist test still holds
  at `226554a`.]

## The resolution

**Millisecond resolution on both instants**, with the emitted duration carrying at least millisecond
precision.

At the retired instrument (Pulse HEAD `83d4060`) the effective resolution was not milliseconds — it was the
15 s lifecycle tick. The value the canvas computed was offset from `ServiceRegistryEntry.last_seen_unix_nano`,
which in steady state is written only by the lifecycle heartbeat (`DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL`,
`crates/triage/src/lifecycle/mod.rs`), so the instrument's granularity was 7.5× the budget it was asked to
grade. A budget of 2 000 ms cannot be graded by an instrument whose quantisation step is 15 000 ms; that
ratio, rather than any individual reading, is why that observable was unusable.

[satisfied at `e98d838`, verified at `226554a`: the duration is backend nanoseconds ÷ 1e6 against the
webview's `Date.now()` milliseconds on the same host wall clock (`constellation-types.ts:266,271`). No tick
lies on the hue path. The heartbeat constant itself holds (`lifecycle/mod.rs:52`, wired at
`pulse-app/src/main.rs:1489`), and so does the tick-only writer of `last_seen_unix_nano`
(`lifecycle/registry.rs:334,340`), but neither is read by the hue sample any more.]

## The window

The duration is the interval between two instants, both internal to Pulse:

- **start — `tier_effective_at`:** the instant the service's maximum priority tier last changed VALUE.
- **end:** the paint of the new hue on the constellation dot — the instant the canvas effect already
  observes, when it detects that a dot's tier differs from the tier it last recorded.

**The start instant needs to be exposed, not invented.** At `83d4060` Pulse computed the tier at
`pulse-app/src/services_router.rs`, where `list_with_states` filtered `incident_registry.list_active()` to the
service's own scope and took `.max_by_key(tier_rank)`. That site held the full `Incident` records at the
moment it decided the tier, so both timestamps the rule below needs were already in hand there. What the
canvas received — `ServiceListItem` — carried no timestamp for `priority_tier` at all, which is why the
retired effect reached for `last_seen_unix_nano`: it was the only per-service instant the item offered.

[corrected 2026-09-29 (c): at `226554a` `ServiceListItem` carries `tier_effective_at_unix_nano: Option<i64>`
(`crates/triage/src/lifecycle/registry.rs:54-69`), derived by the pure replay
`triage::contract::tier_effective_at` (`tier_effective.rs:23-63`).]
[corrected 2026-09-29 (d): "that site held both timestamps" was true of the RISE only at `83d4060` —
`list_active()` excludes resolved records, so the fall's instant was not in hand there. It is true at
`226554a`: one `list_for_workspace` snapshot, Resolved rows included (`crates/triage/src/incident/registry.rs:76-80`),
feeds both the tier (filtered `status != Resolved`) and `tier_effective_at_unix_nano`
(`services_router.rs:95-111`). The set the tier is taken over is unchanged.]

**The rule, in both directions.** The service's maximum tier can change in exactly two ways, and
`tier_effective_at` is defined for each:

1. **The maximum RISES** — an incident opens at a tier above the current maximum. `tier_effective_at` is that
   incident's `opened_at_unix_nano` (`crates/triage/src/contract.rs:418`; the replay pushes it at
   `tier_effective.rs:36`).
2. **The maximum FALLS** — every incident holding the current maximum is resolved. `tier_effective_at` is the
   `resolved_at_unix_nano` of the last one to be resolved (`tier_effective.rs:30-35`).

[corrected 2026-09-29 (a) — the CARRY's first premise, verified at `226554a`: the 2026-09-13 text named the
broadcast's `transitioned_at_unix_nano` (`crates/triage/src/incident/broadcast.rs`) as the fall instant. The
field exists (`broadcast.rs:41`), but the persist-cycle reconciler resolves an incident with no broadcast at
all (`persistence.rs:250-283`), and the broadcast has no production subscriber, so the shipped fall reads
`resolved_at_unix_nano`.]
[corrected 2026-09-29 (b) — the CARRY's second premise, verified at `226554a`: the 2026-09-13 text had the
maximum fall "by resolution or acknowledgement". Acknowledgement does NOT end the tier: an Acknowledged
incident stays in the active set (`incident/registry.rs:219-227`, `services_router.rs:104`), and Pulse pins
it with `acknowledgement_keeps_the_tier` (`tier_effective.rs:167-171`). Acknowledgement is inert.]

**Those two cases are exhaustive, and here is why.** An incident's `priority_tier` is immutable after opening,
so there is no third case in which a tier changes in place. Every write of `priority_tier` across `crates/`
and `pulse-app/src/` is a construction site or a DTO projection, and the single assignment among them is the
one at `services_router.rs` that sets the DERIVED `ServiceListItem` field. No persistence UPDATE touches the
column either; the incident update statements in `crates/corpus/src/contract.rs` set `status`,
`updated_unix_nano`, `resolved_unix_nano`, `payload` and `read_unix_nano` only. The maximum therefore moves
only when the active SET moves, which is what closes the rule at two cases.

[corrected 2026-09-29 (e): the 2026-09-13 text counted "eight in total", which does not reproduce. Re-derived
at `226554a`, and the same set at `83d4060`: 11 non-test writes of a `priority_tier` field (AttentionCue ×4,
DigestCueRef ×2, Incident ×1, DTO ×2, ServiceListItem ×1, the assignment ×1), 5 of them on the incident
chain. The substance holds: the only assignment is the DERIVED item (`services_router.rs:101`), the production
`Incident` is built at `inference_runtime.rs:847/868`, the dedupe path only calls `observe_reemission`, and
the corpus holds no `priority_tier` column (`crates/corpus/src/schema.rs:58-67`; UPDATEs at
`contract.rs:652`, `:682`).]

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
  [expired 2026-09-29: the observable landed at `e98d838`. This clause governed the interval before it; the
  grade is now taken by §The grading rule.]

## The grading rule (stated before the drive)

Stated on 2026-09-29, before the graded leg fires. Its sha256 is recorded into the leg's evidence before the
drive, so the grade cannot be fitted to the reading. The harvest applies it in
`crates/conductor-run/tests/delegated_timing_harvest.rs`.

- **Subject:** every `metric.constellation.hue_update_ms` line whose `timestamp` falls in the leg window.
  - **The window opens at phase-2 start.** That is the `timestamp_ms` of the leg's ONE `timeline.execute` `new`
    line in the frozen Conductor self-obs, plus 30 000 (the `healthy-baseline` phase's `gap_ms`,
    `scenarios/halo-hue-encoding.toml`). This reproduces the prior leg's pinned start exactly:
    1788767011678 + 30000 = 1788767041678. Counting `emit.batch` dispatches does not reproduce it: phase 1's
    60 dispatches spill past the 30 s boundary under jitter, and the 61st lands at +31 126 ms.
  - **The window closes** at that capture's `scenario.run` `close` `timestamp_ms`.
- **Attribution is temporal.** The leaf carries no service identifier. The preflight canary's rise precedes
  `scenario.run`, because preflight readiness requires its incident, and its dot is hidden 60 s after its
  storm and a hidden dot is never sampled.
- **Grade.** If the worst in-window `duration_ms` is ≤ 2 000 (inclusive), the grade is a PASS. Any in-window
  sample over 2 000 is a hard FAIL. A FAIL is a Pulse finding, relayed to Pulse's route for its P-075, and the
  leg is never re-driven to obtain a pass.
- **No in-window sample is UNGRADED,** never met. It is a precondition miss: for example, the widget was not
  mounted before the flip, or the wrong binary was driven. Only an UNGRADED outcome or a Conductor-side fault
  may be re-fired, and only once, with the first attempt kept.
- **Either verdict meets Conductor's gradeability claim (`v3-08`).** Operator ruling, founder-delegated,
  2026-09-29.
- **Mechanism corroboration.** Take each in-window sample whose `severity_tier` is not `none`. Its
  `timestamp − duration_ms` must lie within 1 000 ms of the nearest preceding
  `interpretation.incident.created` line with `created=true`, which is the contracted rise instant
  (`opened_at`) as Pulse's backend witnesses it. A `created=false` line (a dedupe) is skipped. The tolerance is
  the one Pulse's own leg used (`xtask/src/hue_shift.rs:41`, `ANCHOR_TOLERANCE_MS = 1000`).
- **The FALL is witnessed by Pulse's own `smoke:hue-shift` at `e98d838`,** with fall anchor errors of 25 ms and
  28 ms and `duration_ms` 510 and 578 (Pulse `evidence/green-leg.md`). The Conductor leg grades the rise and any
  in-window mid-storm resolve / re-open pair. Its end-of-storm fall lands while the dot is hidden.

## Why the present observable cannot carry the bound

[retired 2026-09-29: "present" in this heading means the observable at Pulse HEAD `83d4060`. The section is
the record of that RETIRED instrument; the terms below are measurements of what it emitted then, and none
of them lies on the hue path at `226554a` — see "What now carries the bound", above §The observable.]

Three properties of the retired fire site each defeated the measurement independently, so a replacement had
to escape all three rather than any one. All three were re-read at `83d4060` on 2026-09-13.

1. **Staleness, not latency.** The canvas computed the emitted duration as the difference between the render
   time and the service's `last_seen_unix_nano` (`pulse-app/ui/src/widget/ConstellationCanvas.tsx`). That is
   how stale the service's telemetry was when its hue changed — not how long the hue change took. The two
   quantities coincide only when the service was seen essentially at the moment of the flip.
2. **Slowest-wins aggregation.** The effect looped every dot whose tier changed in the render pass and emitted
   the MAXIMUM staleness across them, tagging it with that dot's `severity_tier`. A same-pass change on an
   unrelated service therefore reported that service's number under this one's tier. The leaf carries no
   service identifier — the allowlist is aggregate-only — so a consumer could not separate them after the
   fact.
3. **Tick quantization.** `last_seen_unix_nano` has no ingest-path writer. In steady state it is written only
   by the lifecycle tick's refresh, itself gated on the service's quiet duration being zero, where that quiet
   duration is computed in integer-truncated seconds (`crates/triage/src/lifecycle/registry.rs`,
   `crates/triage/src/baseline/activity_floor.rs`). The emitted value was therefore the offset from the last
   refreshing tick, distributed uniformly over the 15 s interval and independent of how fast the service was
   emitting.

[at `226554a`: the effect is `ConstellationCanvas.tsx:119-136`, which calls `hueShiftSamples(...)` and makes
one IPC call per sample (`:133-135`), and reads no `last_seen`. Term 1 is gone, because the start instant is
`tier_effective_at`. Term 2 is gone, because Pulse emits one sample per changed service
(`constellation-types.ts:253-273`). Term 3 is gone from the hue path; the refresh line moved to
`lifecycle/registry.rs:340` and still governs `last_seen` alone.]

**What that cost, measured.** On 2026-08-21 two independent legs read 35 581 ms and 36 705 ms against the
2 000 ms budget. The natural reading — a slow pipeline — was wrong, and the 2026-09-07 re-drive settled it: the
scenario emitted 360 dispatches at 2 per second so the service never went quiet through its tier flip, and its
one in-window sample still read 14 525.9 ms, matching its offset to the preceding tick (14 527 ms) to 1.1 ms.
All seven samples in that capture fit `duration = offset_to_preceding_tick + k × 15 000` with k ∈ {0, 1, 2}
within 2 ms. Meeting 2 000 ms through that instrument required the flip to land in the first 2 s of a 15 s
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

[2026-09-29: Pulse's shipped change `e98d838` is neither candidate. It supplies `tier_effective_at`, and
`last_seen_unix_nano` is off the hue path at `226554a`.]

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

[satisfied at `e98d838`, verified at `226554a`. Item 1: `tier_effective_at_unix_nano` on `ServiceListItem`.
Item 2: `duration_ms = max(0, paint − tier_effective_at / 1e6)` (`constellation-types.ts:271`). Item 3: the
field stays `duration_ms`, which is already admitted. Item 4: one sample per changed service, and only for a
change the canvas witnessed: a change whose effective instant precedes the canvas mount is skipped
(`constellation-types.ts:267`). The sample carries the NEW tier as `severity_tier`, `none` on a fall to no
incident.]

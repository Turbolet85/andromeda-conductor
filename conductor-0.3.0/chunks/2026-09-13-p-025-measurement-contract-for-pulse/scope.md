# Scope — P-025 measurement contract for Pulse

**Marker:** `2026-09-13-p-025-measurement-contract-for-pulse`
**Working entry (verbatim, `conductor-0.3.0/working-route.md:19`):**
> P-025 measurement contract for Pulse — which Pulse-emitted observable, at what resolution, over what
> window, and what constitutes a hard grade

**Capability targeted:** `v3-07` — *P-025 measurement contract stated for Pulse* (`method: by-construction`).

---

## What this chunk builds

A **stated measurement contract** for P-025's delegated hue-shift latency budget: a committed artifact
that tells Pulse exactly what to emit so that the ≤2 s bound becomes measurable at all. It is a
specification Conductor *publishes*, not a measurement Conductor *takes* — the taking is `v3-08`, a later
chunk that is BLOCKED until Pulse implements this.

The working entry names the four required elements, and they are the contract's spine:

1. **WHICH observable** Pulse emits — the leaf/target, its fields, and where it lands.
2. **WHAT resolution** that observable must have — the bound is 2 000 ms, so the instrument's granularity
   has to be far below it.
3. **OVER WHAT WINDOW** it is measured — what the start and end instants are, stated as events in Pulse's
   own pipeline rather than as wall-clock prose.
4. **WHAT COMPARISON constitutes a hard grade** — the exact predicate Conductor would assert, such that a
   green is attainment and not coincidence.

Two qualifiers from `v3-07`'s acceptance bound the writing, and both are substantive rather than stylistic:

- **Implementable without a follow-up question.** The contract has to be complete on its face — a Pulse
  engineer reading only it must know what to change. This is what makes the chunk more than a complaint.
- **Addressed to a value PULSE produces**, never to one Conductor already records for its own emission
  window. This excludes `latency_ms` / `budget_ms` / `effective_deadline_ms`, which bound
  `read_back_observed_at − journal_emitted_at` — Conductor's journal-relative span over the whole scenario
  emission window. Binding the hue budget to those is the category error the obs-plan already names.

## Why the existing instrument cannot carry the bound

This is the measured ground the contract is written from — all of it already committed, none of it
re-litigated here.

- **The disproof.** `metric.constellation.hue_update_ms` measured **35 581 ms** and **36 705 ms** against a
  2 000 ms budget on two independent legs (2026-08-21), pinned by
  `delegated_timing_harvest.rs::p025_hue_update_is_recorded_over_budget_never_asserted_as_a_pass`.
- **The re-drive that identified the cause.** On 2026-09-07 the scenario drove **360 dispatches at 2/s** so
  the subject service never went quiet; its one in-window sample still read **14 525.9 ms**, matching its
  offset to the preceding lifecycle tick (14 527 ms) to **1.1 ms**. All 7 samples in the capture fit
  `duration_ms = offset_to_preceding_tick + k × 15 000`, k ∈ {0,1,2}, within 2 ms
  (`conductor-0.2.0/chunks/2026-09-06-halo-hue-budget-re-driven/evidence/hue-verdict.md`).
- **The mechanism.** `ServiceRegistryEntry.last_seen_unix_nano` has **no ingest-path writer**. Steady-state
  it is written only by `registry.tick_all` (`registry.rs:336`), which stamps the tick's `now` and only when
  `current_quiet_duration_seconds == 0` (integer-truncated, `activity_floor.rs:142-149`), on the 15 s
  `DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL` (`lifecycle/mod.rs:52`). So the leaf reports
  `t_sample − t_last_refreshing_tick` — **U(0, 15 s)** at a randomly-timed flip and **independent of the
  dispatch rate**. A sub-2 s reading is a ~13 % tick coincidence, never attainment.
- **Two further fire-site terms** the contract must also route around (`ConstellationCanvas.tsx:117-155`):
  **slowest-wins** (the effect loops every dot whose tier changed in the render pass and emits the MAXIMUM
  staleness, tagging it with that dot's tier) and a **60 s ceiling** (`visibleDots` drops any service quiet
  longer than `LIVE_RECENCY_WINDOW_NANOS`, `constellation-types.ts:27`).
- **Attribution is temporal, not field-based.** The leaf's allowlist is exactly `duration_ms` +
  `severity_tier` ("Aggregate-only") — no service identifier — and `severity_tier` cannot discriminate
  either, since a dot's tier is incident-derived.
- **The duration is computed in the frontend canvas** and merely logged by the backend, with its start point
  tied to a tick-refreshed timestamp (`v3-07`'s `observed_gap`, measured at Pulse HEAD `83d4060`).

**Consequence, and the reason this chunk exists:** the true hue-shift latency is at present measured by
nothing. The 35 s figure is the reading of an instrument that cannot see the quantity.

**Pulse HEAD re-checked at promotion (2026-09-13): still `83d4060`** — the exact revision every fact above
is bounded at, on branch `chore/migrate-pulse-to-v3`. The mechanism has not been lifted, so the contract is
still owed. **VERIFIED at P3** — every cited Pulse coordinate was re-read against the SUT working copy and
reads exactly as cited (`registry.rs:336`, `activity_floor.rs:142-149`, `lifecycle/mod.rs:52`,
`constellation-types.ts:27`, the canvas effect, and the leaf's `duration_ms` + `severity_tier` allowlist
asserted by Pulse's own test). See `research.md` §Patterns detected.

## In scope

- **The contract artifact itself** — authored whole, carrying the four elements plus the provenance and
  scope bounds that make it auditable.
  `[inferred]` — its FORM and LOCATION are not stated by the working entry. The natural home is
  `contracts/`, beside the three committed SUT-facing manifests, but note the class difference: those three
  are Rust-read at a fixed path and bounds-checked at load, whereas this one is addressed OUTWARD and has no
  Conductor reader. Whether it ships as prose (`.md`) or as a parseable manifest (`.toml`), and whether
  anything in-tree asserts on it, is a P4 decision.
- **The required-observable specification** — naming what the value must measure (the interval between two
  named Pulse-internal events), not merely that it should be "accurate". **VERIFIED and sharpened at P3:**
  the missing value is a START INSTANT. `ServiceListItem` carries no timestamp for `priority_tier` (the
  registry holds no incident data; the `services.list_with_states` resolver enriches the tier by joining the
  incident registry), so the canvas cannot know when the tier became effective and `last_seen` is the only
  per-service instant it is given. The ask is for Pulse to supply the instant the tier became effective on
  the item the canvas already receives — not for a different formula.
- **The sufficiency argument** — an explicit statement of why the proposed observable escapes all three
  fire-site terms above, so the contract cannot be satisfied by an instrument with the same defect in a new
  name. **VERIFIED as necessary at P3** — all three terms reproduce at HEAD, so the argument has three named
  obligations to discharge rather than one general claim to make.
- **The hard-grade predicate** — written so Conductor's harvest tier can assert it directly, and so that
  `v3-08` has an unambiguous target to grade against when the observable lands. **VERIFIED at P3, with one
  alternative positively FORECLOSED:** `Scenario::check_checklist` (`crates/conductor-core/src/scenario.rs:226`)
  raises `CoreError::Config` when a scenario declares checklist items beside expected checks, and
  `halo-hue-encoding.toml` declares one `[[checklist]]` item — so an `[[expected]]` check on that scenario
  fails at load. The harvest tier is the only available home while the checklist stands.
- **Naming the SUT-side change candidates already measured** — the registry copying the activity floor's
  `last_observed_unix_nanos`, or the fire site reading span arrival.
  **[premise-corrected: both candidates fix the STALENESS term only — they would make the number small
  without making it mean hue-shift latency.]** `ActivityFloor.last_observed_unix_nanos` exists and is
  ingest-stamped (`activity_floor.rs:84`, written `:137`), so either change is implementable; but with
  `last_seen` ingest-stamped, a service emitting at 2/s reports ~500 ms and the 2 000 ms budget starts
  PASSING while the quantity remains *the age of the newest span at paint time*. A passing grade over the
  wrong quantity is the exact defect this chunk exists to retire, so the contract cites both candidates as
  **insufficient**, and must never present either as satisfying it.
- **Correcting the stale module doc** at `crates/conductor-run/tests/delegated_timing_harvest.rs:30-36`
  — doc comment only. **Added at P5 (val-1 intent-incomplete, operator-ratified):** P3 found the passage
  names a test that no longer exists and states the cause as staleness, which obs-plan corrected to tick
  quantization on 2026-09-06. Writing a contract about that mechanism while the tree states the superseded
  cause in the file a reader would open next is the argument for taking it in. The two mechanism-pin tests
  and `bounds()` stay untouched — this widens the chunk by a doc comment, not by a behaviour change.

## Out of scope (by design)

- **Implementing the Pulse-side change.** It is Pulse intake, not Conductor work — stated in the scenario
  header, the obs-plan §Delegated-timing family passage, and the 2026-09-06 verdict. This chunk writes the
  ask; it does not reach into the SUT.
- **Re-driving the scenario live or grading the budget hard.** That is `v3-08`, explicitly BLOCKED on Pulse
  emitting the contracted observable, and its matrix note forbids converting it into a narrower claim to
  make the ledger look finished.
- **Changing the mechanism pin.** `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency`
  and its sibling disproof test stay exactly as they are — they record what the current instrument measures,
  and that record stays true whatever Pulse does next.
- **Re-opening the other three delegated budgets.** P-027 / P-037 / P-045 grade hard at real measured values
  under `v2-20` and are untouched here.
- **Weakening the ≤2 s bound.** The bound is Pulse's own delegated claim; this chunk makes it measurable,
  never easier.
- **Any change to `scenarios/halo-hue-encoding.toml`'s driven shape.** Its 360-dispatch 2/s stream is
  calibrated to the current mechanism and is what a future re-drive will reuse. **VERIFIED at P3** — nothing
  in the contract's content requires re-shaping it. Whether the header gains a comment pointing at the
  contract is P4's call; a comment is not a shape change and does not disturb the serde/garde load path.

## Folded annotations

**None.** The working entry carries no annotation at an annotation position — no `PREREQ:`, no `CARRY:`,
no `BLOCKED-ON:`, no `CONTEXT:`, no `BLOCKING:`. Verified by scanning the whole working-route for those
introducers (0 hits across the file) and by reading the entry line in full (142 characters, title plus
scope hint, nothing after it). No external-decay pin stands anywhere in the route to fold either — the
advisory-database deferral that once rode every chunk was CLOSED 2026-09-05.

## Touchpoints (provisional — P3 confirms)

| Surface | Expected involvement |
|---|---|
| `contracts/` | the new contract artifact's likely home `[inferred]` |
| `conductor-0.3.0/verification-matrix.json` | `v3-07` claimed at P5, `ref` set by /implement |
| `scenarios/halo-hue-encoding.toml` | possibly a header pointer to the contract `[inferred]` |
| `.andromeda/obs-plan.md` §Delegated-timing family | the passage that owns the unmeasurable finding — a wrap amendment surface, not a phase edit |
| `crates/conductor-run/tests/delegated_timing_harvest.rs` | READ for the pinned mechanism — plus a stale module-doc reference P3 found at `:36`, which P4 disposes of explicitly (correct here, or record out of scope) |
| SUT working copy (Pulse HEAD `83d4060`) | READ-ONLY — every cited fire-site coordinate re-verified |

## Premise closure (P3) — CLOSED 2026-09-13

Six of the seven `[inferred]` bullets are resolved, each marked in place above: five **VERIFIED** (the Pulse
coordinates, the required-observable specification, the sufficiency argument, the hard-grade predicate's
home, the scenario's unchanged shape) and one **premise-corrected** (the two recorded SUT-side fix
candidates fix staleness only, and would make the budget pass over the wrong quantity).

**One remains open by design and is P4's to decide: the contract's FORM and LOCATION.** Research narrowed it
without settling it — the artifact has no reader, and test-plan's answer to whether a reader-less committed
artifact owes an in-tree gate is *no standing duty* (the 2026-09-13 operator dismissal), so a prose document
is admissible and a parseable manifest would take on the whole §Input Validation regime for no consumer.

Two findings arrived that scope did not anticipate: the missing value is a **start instant** rather than a
better formula, and an `[[expected]]` grade on this scenario is **foreclosed** by `check_checklist`. Both are
folded into the bullets above; full derivation in `research.md`.

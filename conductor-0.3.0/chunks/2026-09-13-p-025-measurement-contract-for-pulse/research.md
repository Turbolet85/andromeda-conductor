# Codebase Research — 2026-09-13-p-025-measurement-contract-for-pulse

## Scope
- **Depth:** moderate–deep (cross-repo: Conductor's grading seam + the SUT's emit path) · **Reads:** 12 · **Globs/Greps:** 9
- **Harness rules consulted:** none — no live leg in this chunk. The chunk authors a document; the live
  re-drive is `v3-08`, explicitly out of scope. No `expect`-atom or firing-form gathering is owed.
- **Code-graph:** rust plane, `db_state: fresh`, 2 queries, 7 rows each
  (`.andromeda/runs/2026-09-13T21-55-00-phase/tree-query-2026-09-13-p-025-measurement-contract-for-pulse.json`).

## Files inspected

**Conductor**
- `crates/conductor-core/src/scenario.rs` (:225-237, `check_checklist`) — the load-path rule that decides
  whether a future hard grade can ride an `[[expected]]` on this scenario. It cannot; see below.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` (:1-80 module doc + `bounds()`; :455-520
  the P-025 pins) — the four delegated bounds as a typed table, and the two mechanism-pin tests.
- `scenarios/halo-hue-encoding.toml` (full) — declares `[[checklist]]`, zero `[[expected]]`, 360 dispatches
  at 2/s across two phases, `slo_tier = "<90s"`.
- `contracts/pulse-run-contract.toml` (full) — the only in-tree precedent for an artifact whose terms are
  about the SUT rather than about Conductor.

**SUT working copy (Pulse), HEAD `83d4060`, read-only**
- `crates/ui-bridge/src/telemetry.rs` (:272-285) — the hue observable's emit site.
- `crates/triage/src/lifecycle/registry.rs` (:43-60 `ServiceListItem`; :180-345 every `last_seen` writer).
- `crates/triage/src/lifecycle/mod.rs` (:52) — the heartbeat constant.
- `crates/triage/src/baseline/activity_floor.rs` (:84, :120-149) — the ingest-stamped instant and the
  quiet-seconds computation.
- `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (:115-155) — the effect that computes the duration.
- `pulse-app/ui/src/widget/constellation-types.ts` (:27, :30) — the 60 s recency window.
- `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs` (:1-40) — Pulse's own assertion of the
  leaf's allowlisted field set.

## Graph impact (from the code-graph query)
- **`conductor-core`** — 7 inbound crate edges (`cli` · `faults` · `report` · `run` · `tauri` · `timeline` ·
  `verify`; `conductor-emit` absent, consistent with the edge removed last chunk). Relevant only as the
  blast radius a *reader* would acquire: this chunk adds no reader, so the measured radius is not taken on.
- **`check_checklist`** — defined at `crates/conductor-core/src/scenario.rs:226` (graph `def_line` 225,
  0-indexed). Its rule, not its call count, is what binds here.

## Patterns detected

- **The hue observable is an IPC resolver, not a backend measurement** (`ui-bridge/src/telemetry.rs:272-285`):
  `record_constellation_hue_latency` receives `ConstellationHueLatencyInput { duration_ms, severity_tier }`
  from the frontend, runs `validate_duration_ms`, and emits `tracing::info!` at the leaf. The backend
  originates no part of the quantity — it validates and logs a number the canvas computed.
- **The canvas computes staleness, and slowest-wins is literal** (`ConstellationCanvas.tsx:115-155`): the
  effect fires only when at least one dot's tier CHANGED, and for each changed dot computes
  `elapsedMs = nowMs - item.last_seen_unix_nano / 1_000_000`, keeping the MAXIMUM across the pass together
  with that dot's tier. Both fire-site terms the scenario header names are confirmed verbatim at HEAD.
- **`last_seen_unix_nano` has five writers and none is on the ingest path**
  (`lifecycle/registry.rs`): entry creation inside `tick_all` (:330), the gated tick refresh (:335-336,
  `if snapshot.current_quiet_duration_seconds == 0`), `set_manual_override` (:211),
  `set_state_on_restart` (:245, :257) and `set_state_on_corpus_restore` (:288). Span arrival writes none of
  them. The claim as recorded is exact.
- **The quiet-seconds gate truncates to whole seconds** (`activity_floor.rs:142-149`):
  `(elapsed_nanos.max(0) / 1_000_000_000) as u64`, so "seen under 1 s before the tick" is the literal
  admission rule for the refresh.
- **An ingest-stamped instant already exists** — `ActivityFloor.last_observed_unix_nanos`
  (`activity_floor.rs:84`), assigned `now_nanos` on each observation (:137). The fix candidate Conductor's
  artifacts name is therefore live and real, not hypothetical.
- **The leaf's allowlist is exactly `["duration_ms", "severity_tier"]`**, asserted by Pulse's own test
  (`pulse-app/tests/unit_observability_allowlist_delegated_timing.rs:27-40`), whose header names the emit
  sites as "the authority for these field lists". Any field the contract asks for must be added there too,
  or it arrives `"<redacted>"` — the obs extract's constraint, confirmed against the SUT's own guard.

## The decisive finding — the missing value is a START INSTANT, not a better formula

`ServiceListItem` (`lifecycle/registry.rs:54-66`) carries `service` · `state` · `last_seen_unix_nano` ·
`manual_override` · `priority_tier`. **`priority_tier` carries no timestamp**, and its doc comment says the
registry holds no incident data at all — `list_all` always emits `None` and the `services.list_with_states`
resolver enriches it by joining the incident registry on `scope_id`.

So the canvas, at the moment it observes a tier change, has no way to know *when* that tier became
effective. `last_seen` is the only per-service instant on the item, which is why the effect reached for it.
The observable is not a miscalculation — it is the best available proxy given what the backend hands over.

This settles what the contract must ask for, and it is a narrower ask than "measure it differently":
**Pulse must supply the instant at which the priority tier became effective**, as a backend-owned field on
the item the canvas already receives. With that, the duration becomes `paint_time − tier_effective_at`, a
real pipeline latency; without it, no frontend formula can express one.

### Addendum (P5 review) — the instant is EXPOSED, not minted, and the rule has exactly two cases

Operator-supplied coordinates, each re-verified against the SUT working copy before use. The tier is
computed at `pulse-app/src/services_router.rs:97-103`: `list_with_states` filters
`incident_registry.list_active()` to `scope == CueScope::Service && scope_id == item.service` and takes
`.max_by_key(tier_rank)`. **That site already holds the full `Incident` records**, so the timestamps are in
hand where the tier is decided — `opened_at_unix_nano` (`crates/triage/src/contract.rs:418`) and
`transitioned_at_unix_nano` on `IncidentLifecycleEvent` (`crates/triage/src/incident/broadcast.rs:41`). All
three coordinates read exactly as dictated. The ask is therefore to **expose** an instant Pulse already has,
not to mint one — a materially smaller change than the plan first assumed.

**The escalation arm does not exist.** `IncidentLifecycleEvent` carries `from_state` / `to_state` of type
`IncidentStatus`, so `transitioned_at_unix_nano` timestamps a STATUS change, not a tier change — and an
incident's `priority_tier` is immutable after opening. Basis:
`grep -rnE '\.priority_tier\s*=|priority_tier:\s*[a-z]' --include=*.rs crates/ pulse-app/src/` returns eight
sites at HEAD `83d4060` — `cadence/coordinator.rs:587`, `triage/contract.rs:628`, `cue/emitter.rs:1282`,
`digest/assembler.rs:265`, `pattern/storm.rs:321`, `incidents_router.rs:59`, `training_export.rs:130`, and
`services_router.rs:97` — every one a construction site or a DTO projection, the single assignment being the
derived `ServiceListItem` field. No persistence UPDATE touches the column
(`crates/corpus/src/contract.rs:652` and `:682` set `status` / `updated_unix_nano` / `resolved_unix_nano` /
`payload` / `read_unix_nano` only).

So the service's max tier changes only when the active SET changes, which closes the rule at two cases:
the max **rises** when an incident opens above it (`opened_at_unix_nano`), and **falls** when the last
incident holding it leaves the active set (`transitioned_at_unix_nano`). The review's open question — which
instant applies when a tier changes by escalation rather than by opening — is answered by dissolution:
escalation is not a state this SUT can reach.

## Scope premise closure

All seven `[inferred]` bullets re-read against the findings.

- **Pulse coordinate currency** — **VERIFIED.** HEAD is `83d4060`; `registry.rs:336`,
  `activity_floor.rs:142-149`, `lifecycle/mod.rs:52`, `constellation-types.ts:27` and the canvas effect all
  read exactly as cited. Tag dropped.
- **Contract form/location** — **still open for P4, but narrowed.** The three SUT-facing manifests are
  Rust-read at fixed paths; this artifact has no reader and test-plan's answer to "must a reader-less
  committed artifact carry a test" is *no standing duty* (the 2026-09-13 dismissal). A prose document is
  therefore admissible without a gate; a parseable manifest would pull in the whole §Input Validation regime
  for no present consumer.
- **Required-observable specification** — **VERIFIED and sharpened** by the start-instant finding above.
- **Sufficiency argument** — **VERIFIED as necessary.** All three fire-site terms reproduce at HEAD, so the
  argument has three named obligations to discharge rather than a general claim to make.
- **Hard-grade predicate home** — **VERIFIED, and one option positively FORECLOSED.**
  `Scenario::check_checklist` (`scenario.rs:226-237`) returns `CoreError::Config` when a scenario declares
  checklist items beside expected checks. `halo-hue-encoding.toml` declares one `[[checklist]]` item, so
  adding an `[[expected]]` check to it fails at load. The harvest tier is not merely the conventional home —
  it is the only one available while the checklist stands.
- **SUT-side change candidates** — **[premise-corrected: copying `last_observed_unix_nanos` into the
  registry, or stamping `last_seen` from span arrival, would make the number SMALL without making it
  MEAN hue-shift latency.]** Both recorded candidates fix the staleness term only. With `last_seen`
  ingest-stamped, a service emitting at 2/s reports ~500 ms — inside the 2 000 ms budget — so the budget
  would start passing while still measuring *the age of the newest span at paint time*. That is a passing
  grade over the wrong quantity, which is precisely the defect this chunk exists to retire. The contract may
  cite the candidates as *insufficient*, and must not present either as satisfying it.
- **No change to the scenario's driven shape** — **VERIFIED.** The 360-dispatch 2/s stream is calibrated to
  keep every 15 s tick refreshing `last_seen`, and nothing in the contract's content requires re-shaping it.
  Whether the header gains a pointer to the contract is a P4 decision; a comment is not a shape change and
  would not disturb the garde/serde load path.

## Two debts found in a file this chunk reads

Both sit in `crates/conductor-run/tests/delegated_timing_harvest.rs`, which scope lists as READ-not-modified.

1. **`:36` names a test that no longer exists and states the retired cause.** The module doc pins the P-025
   disproof to `p025_hue_update_is_measured_over_budget_by_a_staleness_mechanism`; the file's actual tests
   are `p025_hue_update_is_recorded_over_budget_never_asserted_as_a_pass` (:500) and
   `p025_the_re_driven_leg_measures_tick_quantization_not_update_latency` (:661). The same passage says
   "the observable measures staleness rather than update latency" — the cause obs-plan corrected to TICK
   QUANTIZATION on 2026-09-06. The rename landed; the module doc did not follow it.
2. **`bounds()` pins its provenance to SUT HEAD `f0c38f5`** while every mechanism fact in play is bounded at
   `83d4060`. Not wrong — it dates the budget table, not the mechanism — but a reader comparing the two
   pins has no way to tell that from the file.

Neither is this chunk's defect and neither blocks it. Raised for P4 to dispose of explicitly: a four-line
doc correction inside the chunk's own subject matter, or a recorded out-of-scope note.

## New files to create
- The contract artifact (path and extension are P4's decision; `contracts/` is the indicated home).

## Files to modify
- `conductor-0.3.0/verification-matrix.json` — `v3-07` claimed at P5 (`chunk`), `ref` written by /implement.
- `scenarios/halo-hue-encoding.toml` — a header pointer to the contract, if P4 takes it. Comment-only;
  no key, so the serde/garde load path is untouched.
- `crates/conductor-run/tests/delegated_timing_harvest.rs` — **only if** P4 adopts the doc correction above.

No caller threading applies: the chunk changes no signature, adds no parameter and registers no command, so
the graph's caller set is empty by construction rather than unqueried.

## Open questions
- Does the contract ship as prose or as a parseable manifest? → blocks: **plan-decision**. Research has
  narrowed it (no reader exists, no gate duty attaches, the manifest regime would be taken on for no
  consumer) but the choice is P4's.
- Does the harvest test's stale module doc get corrected here, or recorded as out of scope? → blocks:
  **plan-decision** — it decides whether the chunk's modify-set includes a `crates/` file at all.

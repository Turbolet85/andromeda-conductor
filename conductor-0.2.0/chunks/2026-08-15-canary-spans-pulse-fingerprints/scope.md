# Scope — Canary spans that Pulse fingerprints

**Marker:** `2026-08-15-canary-spans-pulse-fingerprints`
**Epoch:** 3 — Live proof: the five families
**Working-route intent:** *the buffer enumerates the canary's span events and its fingerprint observer
runs, as it already does for `inject_demo`.*

---

## What this chunk builds

The preflight canary's OTLP spans currently arrive at Pulse, are counted at ingest, and then
**enumerate to zero** in Pulse's buffer appender — so no span event is ever collected, the fingerprint
observer is never invoked, no fingerprint is computed, no storm is detected, no incident forms, and
preflight never reaches `ready:true`. This chunk finds **what differs between the canary's spans and
`inject_demo`'s** at that boundary, lands the Conductor-side correction, and re-runs the live leg.

The deliverable is the canary emitting spans that Pulse's appender actually walks — measured, not argued.
Concretely the chunk must, in order:

1. **Settle the `rows_ingested` unit** (the flagged tension — it can demote the entire lead set before
   any work is spent on it).
2. **Discriminate the two leads by measurement**, not by plausibility.
3. **Land the Conductor-side change** in the canary emit path.
4. **Re-run the live leg** against a real Pulse and record what it measures.
5. **Claim `v2-10` only if that leg actually reaches `ready:true`** — never on an argument that the
   known obstacles are gone.

## Boundaries

- **Conductor-side change only.** Pulse's repo (`D:/dev/projects/andromeda-pulse`, parked at HEAD
  `d090314`) is **READ-ONLY** for this chunk — read it freely for ground truth, never edit it, never
  commit there.
- **Not a fingerprint-derivation change.** Fingerprinting sits downstream of this gap and is proven
  working for `inject_demo` (Pulse tracked `c33df842`); the observer is never *invoked* here, so nothing
  about the derivation is implicated.
- **Not a scenario-catalog change.** `scenarios/fingerprint-storm.toml` stays as authored (two-phase
  6 → Suggested, 12 → Autonomous); no scenario TOML is edited.
  **[widened at P5 val-1 — intent-incomplete]** The original wording added "only the **preflight canary**
  path is in scope", which research falsified as a boundary: the defective builder (`trace_request` →
  `ok_span`) is SHARED, backing the scenario emission dispatcher at `dispatch.rs:82` and `:95` as well as
  the canary warm-up. Every scenario phase emitting through it has been sending the same
  `(trace_id, span_id)` into a table keyed on that pair. Fixing one builder is compile-forced to visit both
  call sites, and leaving the dispatcher colliding would knowingly preserve a proven production-path defect.
  The **emission-path** fix is therefore in scope; the scenario catalog remains out.
- **Not a re-litigation of `CANARY_STORM_COUNT = 12`.** That fix landed at
  `2026-08-15-canary-storm-autonomous-band` and its live leg measured it correct — necessary, just not
  sufficient. The count stays 12 unless research shows the count itself participates in this gap.
- **No Pulse-side fix is in scope.** If research proves the gap is genuinely Pulse-side and no
  Conductor-side emission change can close it, the chunk's honest exit is the **recorded measurement
  plus an un-claimed `v2-10`** — never a Pulse edit, and never a claim resting on the fix being
  "obviously" available elsewhere.

## Surfaces and contracts touched

| Surface | Role in this chunk |
|---|---|
| `crates/conductor-run/src/lib.rs` | the canary storm emit path + `CANARY_STORM_COUNT` |
| `crates/conductor-emit/` | OTLP raw-type construction — the span/id/event builders the canary does or does not use |
| `crates/conductor-run/tests/canary_wire.rs` | the wire assertion that today passes against Conductor's OWN collector |
| `crates/conductor-verify/` (preflight gate) | consumes the canary round-trip result; touched only if the canary's shape changes what the gate asserts |
| `.andromeda/architecture.md` §Occupied Resources | carries the "second gap" paragraph — an **expected amendment at this chunk's wrap** once the cause is measured |

## Operator-verified Pulse coordinates

Supplied by the overseer, verified against Pulse source at parked HEAD `d090314`.
**Cite as ground truth; re-confirm cheaply in P3** (one read each, per the standing rule that a dictated
cross-project citation is verified when the SUT's repo is on disk).

1. **Trio counter semantics** — `crates/buffer/src/appender.rs:392-404` + `crates/buffer/src/state.rs:82-91`:
   `span_events_seen` = `trace_ids.len()` **collected by** the span-event enumeration loop;
   `fingerprints_computed` = the `Some()` fingerprints among them; `observer_invocations` increments once
   per computed fingerprint dispatched. **Therefore the measured `0/0/0` means the enumeration collected
   ZERO entries from every canary batch** — not "the observer ran and saw nothing".
2. **The enumeration region** — `crates/buffer/src/appender.rs:340-356`:
   `for rs in resource_spans → for ss in rs.scope_spans → for span in ss.spans →`
   **GUARD** `if span.trace_id.is_empty() || span.span_id.is_empty() { continue; }` →
   `for event in span.events →` extract `exception.type` / `.message` / `.stacktrace` from
   `event.attributes`. The guard **silently drops** spans with empty ids — a verified mechanism by which
   arrived-and-counted spans enumerate to zero.

## Premises closed in P3 (research.md §Scope premise closure)

**All three opened premises are now closed, and a fourth cause — named by no prior document — was found.**

- `[premise-corrected: a row is a parent-span row appended to the spans table, not an export request —
  append_record_batch_to_table returns record_batch.num_rows(), pinned by consumer.rs:542-543]`
  **TENSION — the `rows_ingested` unit.** The directive's conditional ("if a row is an export request, the
  gap moves upstream and both leads are demoted") does not fire on its stated antecedent — **yet both leads
  are demoted anyway, on independent evidence below.**
- `[premise-corrected: FALSIFIED — both canary builders emit non-empty correct-length ids, and span_count:15
  is itself proof they passed Pulse's id-length invariant]` **LEAD A — empty ids.**
  `exception_trace_request` seeds 16/8-byte ids per occurrence (`exception.rs:134-135`); `ok_span` uses
  constant-but-well-formed `[1;16]`/`[1;8]` (`message.rs:151`). Pulse's `validate_resource_spans` rejects a
  wrong-length id for the **whole export**, and `record_spans` runs only after that check and after
  `try_send` (`grpc.rs:144-150`).
- `[premise-corrected: substantially weakened — inject_demo uses the identical nesting]` **LEAD B —
  composition / nesting shape.** The surviving producer differences are span-identity **uniqueness** (the
  real one) and batching cardinality, not nesting.
- **THE CAUSE FOUND — span-identity collision against a primary key.** Pulse's `spans` table is
  `PRIMARY KEY (trace_id, span_id)` (`schema.rs:38`) and Conductor's warm-up builder emits a **constant**
  identity on every call, so every warm-up span of every run shares one key: the first append succeeds, each
  subsequent one violates the PK and is logged-and-skipped by `run_consumer`. This is exactly the observed
  producer-dependence — `inject_demo`'s ids are per-sequence and never collide.
- **Trio semantics refined** — a written trio value means what the directive states, but `record_feed_counts`
  sits after the `is_empty` early return, so a **`0` is the never-written state** and cannot alone
  discriminate three distinct upstream causes. `ok_span` carries no events, so the one warm-up span that did
  append cannot contribute to `span_events_seen` either way.

**Residual open question (implementation-scope):** the PK collision accounts for the warm-up, but the
storm's 12 spans carry distinct seeded ids and should append. The leg must capture Pulse's existing
`duckdb.append` `reject_reason` — never captured on a canary leg before — which names any remaining cause
directly instead of inferring it from unwritten counters.

## Carried forward

**CARRY (from `2026-08-15-canary-storm-autonomous-band`)** — the count fix landed and was **necessary but
NOT sufficient**; this entry inherits the **preflight-green blocker** role and its first diagnostic step is
already answered. That chunk raised `CANARY_STORM_COUNT` to 12 and its live leg proved the storm reaches
Pulse intact (`span_count: 15` = 3 warm-up + 12 storm; sidecar dispatching; all four tools present;
workspace key published), yet `incidents: 0` with no storm detected at any tier. The `buffer.tick` trio
discriminates it and rules out the fingerprint stage entirely: across 15 identical ticks
`span_events_seen: 0` · `observer_invocations: 0` · `fingerprints_computed: 0`, with `rows_ingested: 1`
against `span_count: 15`. It is **producer-dependent** — `inject_demo` batches 2 ops × 3 spans per 500 ms
and keeps going; the canary sends 12 single-span unary exports and stops.
Evidence: `conductor-0.2.0/chunks/2026-08-15-canary-storm-autonomous-band/report.md` ·
`andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/evidence/premise-check.md`.

**Standing telemetry-reading rules** (carried, still binding when re-testing):
- `tracked_fingerprints_count` is a **60 s-windowed gauge over DISTINCT fingerprints**, sampled at a 15 s
  tick after eviction — a healthy 12-occurrence *identical*-fingerprint storm reads **1, never 12**, and a
  late sample reads 0 on a healthy path.
- `storms_detected_total` increments on **both** the Suggested and Autonomous branches — it discriminates
  storm **PRESENCE, never TIER**.
- `severity_hint` on `triage.pattern.storm.detected` is what names the tier.

**PREREQ — re-check `cargo audit`** (standing deferral since `2026-08-08-sut-capability-manifest`;
operator-ratified at the `2026-08-10-workspace-key-divergence-probe` wrap; this chunk carries the **20th
pin**, re-deferred by `2026-08-15-canary-storm-autonomous-band` which was itself zero-delta). Basis: an
advisory-**DATABASE** fault — byte-identical `duplicate advisory ID: RUSTSEC-2026-0244`, with no floor to
raise (0.22.2 is the latest published). Overlap `cargo deny check` must be verified true exit 0 across all
four classes, never assumed. Full rationale:
`conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.
**Close the deferral the moment it parses.**

## Verification linkage

`v2-10` ("Live `ready:true` preflight") returned to the pool unclaimed at the previous chunk with the full
measurement in its `notes`, acceptance unchanged and unweakened. This entry inherits the blocker role;
**`v2-10` becomes claimable again only on a leg that actually reaches `ready:true`** — the P5 matrix link
is conditional on research showing that leg is provable by THIS chunk, and the claim never rests on the
argument that the known obstacles are gone.

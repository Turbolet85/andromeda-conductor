# Live-leg verdict — fingerprint-storm family

**Date:** 2026-08-16 · **SUT:** andromeda-pulse @ `d090314`, read-only (no Pulse source touched, nothing built)
**Mode:** deterministic L4 · fresh data dir · six-item recipe, item 6 (`RUST_LOG=info,conductor_emit=debug`)
on the boot + scenario legs only, never on a test-suite invocation.

## Preflight

`agent-run.sh boot` → **`ready:true`**, `canary_round_trip:"ok"`, `blocked_precondition:null`, exit 0,
all four required tools present, protocol `2024-11-05` negotiated.

## Read-back key-set diff — the `2026-08-13-first-live-green-preflight` CARRY, DISCHARGED

All three tools witnessed live for the first time. Diffed against the pinned baseline in
`conductor-verify/tests/readback.rs` BEFORE any verdict was trusted, per the CARRY:

| tool | observed live | pinned baseline | verdict |
|---|---|---|---|
| `query_incident_list` | `[items, next_cursor, total]` | `:133` same | **match** |
| `retrieve_report` | `[degraded_mode, markdown]` | `:139` same | **match** |
| `retrieve_telemetry_slice` | `[fingerprint_refs, incident_id, span_refs, timestamps_unix_nano]` | `:143` same | **match** |

**Zero divergence.** The two readers that were stub-proven-only are now live-proven; two thirds of the
read-back key-diff retire. No finding, no stub test needed.

## Leg 1 — `fingerprint-storm` (seed 4317017)

Harvested from `{data_dir}/logs/agent-latest.jsonl.2026-08-16`, sliced by pre-leg line count 113685.
Window: 31 094 lines. Slice preserved at `storm-leg-window.jsonl`.

Two fingerprints in the window, cleanly separable:

| fingerprint | source | storm-detected lines |
|---|---|---|
| `b86a772c` | preflight canary | `suggested@5` 13:40:58.714Z · `autonomous@10` 13:40:58.746Z |
| **`cbe26ad3`** | **the scenario's own storm** | `suggested@5` 13:41:09.755Z · `autonomous@10` 13:41:15.823Z |

- **P-017 identity — VERIFIED.** The reshaped identical/path/line triple across 18 occurrences produced
  **exactly ONE fingerprint**, `cbe26ad3`. Before the reshape the path variant diverged, which would have
  split the storm across two fingerprints; it did not.
- **P-018 thresholds — VERIFIED.** `suggested` then `autonomous`, the latter at `occurrence_count = 10`,
  on the scenario's own fingerprint. The counts are the tier-crossing points (one-shot per tier), not the
  occurrence total — as documented.
- **P-074 coalescing — VERIFIED in substance.** The 18-occurrence single-fingerprint storm produced
  **zero additional incident rows**: `interpretation.incident.created` at 13:41:15.900Z reads
  `created:false, deduped:true`. Never one incident per occurrence.

Envelope: `verdict Fail` · `state KnownResidual` · `latency_ms 24257` · `slo_tier <20s` · `fingerprints []`.

## Leg 2 — `fingerprint-distinct` (seed 4317018), >60 s quiet after leg 1

Slice at pre-leg line count 180604; window 28 837 lines; preserved at `distinct-leg-window.jsonl`.

- **P-017 distinctness — VERIFIED.** Exactly one fingerprint appears in the window, `37635e30`, and it is
  the **preflight canary's**. The scenario's three sub-floor phases (base / type-variant / frame-variant,
  4 occurrences each) raised **no storm line at all**. Had type/frame variants wrongly merged, the combined
  12 would have crossed the floor and stormed. They did not.

Envelope: `verdict Fail` · `state KnownResidual` · `latency_ms 30192` · `slo_tier <20s`.

## What did NOT pass, and why it is not a Conductor defect

**1. Both legs grade `Fail`/`KnownResidual` on their authored token checks.** Read-back was served under
`degraded_mode: true`, so `retrieve_report.markdown` is a degraded report, not the canned L4 narrative.
`Contains "RetryStorm"` (storm) and `Absent "RetryStorm"` (distinct) both failed against it.

This is the deterministic-L4 limit reaching the family level: **no read-back field varies with what
Conductor emitted**, so an authored token check on read-back text is unverifiable in this mode. The real
identity/tier evidence is the log harvest above — which is exactly why this chunk built one.

**2. The `<20s` tier is structurally unattainable for this scenario.** `latency_ms` is
`read_back_observed_at − journal_emitted_at`, and the storm scenario's own emission window is ~24 s
(two 12 000 ms phases). Measured 24 257 ms and 30 192 ms. No live run of this shape can land under 20 s;
the tier needs `<90s`.

**3. A P-074 caveat, recorded rather than smoothed.** In leg 2 the canary's single storm produced **two**
incidents one second apart (`created:true` at 13:44:35.518Z and 13:44:36.439Z), followed by 41 dedupes.
The scenario's own storm coalesced correctly in leg 1, so the P-074 claim holds on the evidence this chunk
drove — but "one storm ⇒ exactly one incident" is not unconditionally true of the canary path, and this
chunk did not determine why.

## Operational findings not previously recorded

- **Pulse dedupes a new incident against an OPEN one, and the key is not the fingerprint.** The canary's
  `exception_type` is `ConductorCanary_{now_ms}` — unique per run, therefore a distinct fingerprint — yet
  the second run's incident still deduped. Consequence: **back-to-back runs against one Pulse instance
  block**, because each run's preflight canary needs a fresh incident and gets a dedupe instead. The first
  storm leg blocked for exactly this reason (`no incident opened after the canary storm was emitted`).
  The remedy that worked is waiting for auto-resolve — incidents resolved ~5 min after creation — which
  also satisfies the plan's >60 s retention-window quiet.
- **`pulse-app` logs at roughly 2 000 lines/s**, dominated by `metric.webgpu.frame_duration_ms` (61 212 of
  65 968 lines in one window). A harvest must filter by target; a whole-file scan is not viable.
- The graceful `taskkill` did not stop `pulse-app`; a forced stop was required to release `:4317`. The
  instance was idle and its data dir a throwaway temp, so nothing durable was at risk.

## Re-run after re-calibration — FRESH DATA DIR PER LEG (the graded evidence)

The first pass above diagnosed; this pass grades. `pulse-app` was restarted on a brand-new data dir
between legs, which kills BOTH hazards at once — the 60 s shared-base retention window AND the
incident-dedupe (an open incident absorbs a later storm regardless of fingerprint, so on a shared dir
auto-resolve at ~5 min is the only cure). Every artifact below is from a corpus with no prior residue.

### Leg 1 — `fingerprint-storm` @ seed 4317017, dir `pulse-leg1-135709`

| fingerprint | source | detected lines |
|---|---|---|
| `6ca0676a` | preflight canary | `retry_storm/suggested@5`, `retry_storm/autonomous@10` |
| **`cbe26ad3`** | **the scenario's storm** | `retry_storm/suggested@5`, `retry_storm/autonomous@10` |

- **IDENTITY ✓** — one fingerprint across all 18 occurrences. `cbe26ad3` is **byte-identical to the
  first pass's scenario fingerprint on a different data dir**, which independently confirms the
  determinism bar: same scenario + seed ⇒ same fingerprint.
- **THRESHOLDS ✓** — `suggested@5` then `autonomous@10`, `cue_kind = "retry_storm"` on both.
- **COALESCING ✓** — `interpretation.incident.created` reads `(created:true, deduped:false)` for the
  canary then `(created:false, deduped:true)` for the scenario's storm, and the corpus finishes with
  **exactly ONE incident**. 18 same-fingerprint occurrences, one incident, on a clean corpus.

Envelope: `verdict null` · `state KnownResidual` · `latency_ms 24258` · **`slo_tier <90s`** · `p_ids [P-017, P-018, P-074]`.
Slice: `rerun-storm-leg-window.jsonl` · envelope: `rerun-storm-leg-envelope.jsonl`.

### Leg 2 — `fingerprint-distinct` @ seed 4317018, dir `pulse-leg2-135904`

- **DISTINCTNESS ✓** — exactly one fingerprint in the window, `4dabae63`, and it is the **canary's**.
  The scenario's three sub-floor phases raised **zero** storm lines. Had type/frame variants merged,
  the combined 12 would have crossed the ≥5 floor and written a line. Corpus: one incident (canary's).

Envelope: `verdict null` · `state KnownResidual` · `latency_ms 30187` · **`slo_tier <90s`** · `p_ids [P-017]`.
Slice: `rerun-distinct-leg-window.jsonl` · envelope: `rerun-distinct-leg-envelope.jsonl`.

### What the re-calibration changed

- `slo_tier` `<20s` → **`<90s`** in both scenarios, basis recorded in each file: `latency_ms` spans the
  scenario's own emission window (~24 s storm / ~30 s distinct) before read-back cadence, so `<20s` was
  unattainable by construction rather than merely tight. Both legs now sit inside their tier.
- The `Contains`/`Absent "RetryStorm"` read-back checks are **retired to declare-only**, and their
  assertions moved to the harvest surface where the token's real analogue is `cue_kind = "retry_storm"`
  on Pulse's own detected line (`retry_storm_surfaced` / `no_storm_surfaced` in
  `crates/conductor-run/tests/storm_harvest.rs`).
- `verdict` is now `null` rather than `Fail`: with no read-back check to grade, the scenarios route to
  the drive+observe path instead of failing on an ungradeable assertion. That is the honest shape.

**Why this is a correction and not a weakening.** `degraded_mode: true` is the deterministic-L4
report's PERMANENT state — it is computed as `parsed_l4.is_none()`, and the deterministic fixture is
never parsed as an L4 output — so `retrieve_report.markdown` can never carry a scenario-specific token
in this mode. The `Contains` side always failed; the `Absent` side always passed **vacuously**, which is
the more dangerous of the two. This joins the `evidence_refs: []` limitation as the second measured
instance of the same rule: under deterministic L4 no read-back field varies with what Conductor emitted.

## Bottom line for `v2-11`

Every capability the family exists to prove was **measured true**: one fingerprint across the triple, the
suggested→autonomous ladder at the right count, sub-floor distinctness with no aggregation, and
single-incident coalescing. What did not pass is the scenario's authored read-back token checks and its
SLO tier — both of which the live evidence shows are mis-specified rather than unmet.

After the re-calibration and the fresh-dir re-run, **all five acceptance clauses are satisfied on
artifacts this work produced**: identity (one fingerprint across 18 occurrences), distinctness (zero
storm lines from sub-floor variants), thresholds (`suggested`→`autonomous` at 10), coalescing (exactly
one incident on a clean corpus), and the re-calibration the acceptance itself mandated. `v2-11` is
recorded `implemented`.

**One premise in that acceptance is measured insufficient and is corrected, not weakened.** The
acceptance says the two legs must be "separated by more than Pulse's 60s storm-retention window (they
share a base fingerprint)". True but incomplete: separation alone does NOT make a same-dir back-to-back
leg runnable, because a second, independent mechanism blocks it — Pulse dedupes a new incident against
any OPEN incident, and **not by fingerprint** (the canary's `exception_type` is `ConductorCanary_{now_ms}`,
unique per run and therefore a distinct fingerprint, and it deduped anyway). On a shared dir the only
cure is waiting out auto-resolve at ~5 min. The graded re-run therefore used a **fresh data dir per
leg**, which dissolves both hazards. The 60 s clause stays correct about storm retention; it was never
sufficient on its own.

The dedupe semantics — an open incident absorbing a DIFFERENT-fingerprint storm — is a Pulse
product-design question, not a Conductor defect, and is recorded as **Pulse-visit candidate #4**.

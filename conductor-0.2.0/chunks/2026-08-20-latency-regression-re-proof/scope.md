# Scope — 2026-08-20-latency-regression-re-proof

**Working-route intent (verbatim):** latency-regression re-proof — a regression short relative to
accumulated history, read back on a surface that survives the window (P-011, P-012)

## What this chunk builds

1. **A re-shaped latency-regression scenario whose spike Pulse can actually detect.** The shipped 90/90
   shape is measured undetectable — the long-window t-digest p99 ABSORBS a ramp contributing ~half the
   window — so the re-shape must make the regression SHORT relative to accumulated history: a
   longer/denser baseline, a sharper shorter spike, or both. No threshold multiplier can fix a
   denominator the anomaly itself feeds. [premise-corrected: P3 pinned the actual mechanism — the LONG
   baseline read is the union of a 60s-rotating t-digest pair (current+previous, rotation live-confirmed
   from leg B4's `baseline tracker tick` lines), so "accumulated history" spans at most ~the last
   60–120s; DENSITY of that window is the lever, and the spike must stay under ~1% of the union's
   samples while the short pair (15s rotation, current-only read) carries it.]
2. **A read-back/grading design that does not depend on the canary incident surviving the run.** The
   canary incident auto-resolves across a benign window (30s resolver ticks, 120s idle window), so the
   scenario grades purely at the harvest tier with the row's non-Blocked clause re-thought.
   [premise-corrected: the CARRY's other option — the scenario raising its own cue-raising traffic —
   is DEAD on this host: this family's cues top out at Suggested tier, and Tier-2 cadence cycles are
   skipped wholesale on cpu-primary (andromeda-pulse cadence/coordinator.rs:154-172, already settled at
   v2-12 concretization), so no scenario-raised incident can exist. The harvest-tier route is forced,
   realized as a scoped row-state mapping change in `conductor-run` (see Boundaries).]
3. **Live fresh-dir leg(s) against a live deterministic-L4 Pulse** measuring the re-shaped scenario:
   the `latency_regression` cue observed firing (ZERO cue lines was the whole prior-leg verdict), the
   witness offsets recorded, leg-verdict evidence written under `evidence/`.
4. **The latency predicates in `crates/conductor-run/tests/baseline_harvest.rs` pinned VERBATIM from
   this chunk's leg.** They ship live-unexercised with source-derived fixtures; the entry mandates
   re-pinning from THIS entry's leg.
5. **`check_load_envelope` re-run over any re-shaped phases.** The re-shape must pass the envelope's
   asserted sustained terms unaided — the `[[exempt]]` ledger is empty and is held to exact-set
   equality, so it can only shrink; a denser/longer baseline must stay inside
   `max_sustained_storm_ms` / `max_sustained_rate_spans_per_s` per emitting phase. (Verified feasible:
   every candidate phase sits far inside 600s / 10000 spans-per-s.)
6. **PREREQ absorbed: the 32nd consecutive `cargo audit` re-check** (protocol below; close the
   deferral the moment it parses).

## Measured starting points (CARRY from 2026-08-18-error-baseline-spike-live-proof)

Two obstacles, both measured, neither speculative — evidence:
`conductor-0.2.0/chunks/2026-08-18-error-baseline-spike-live-proof/evidence/leg-verdict.md` §Leg B.

1. **t-digest window absorption.** The long-window t-digest p99 absorbs a ramp contributing ~half the
   window: threshold ≈ long·2.5 ≈ 6000ms vs short ≈ 2400ms; ZERO `latency_regression` cue lines across
   the whole leg. Detectability needs the regression short relative to accumulated history.
2. **Canary incident auto-resolve.** The canary incident auto-resolves across a benign window (30s
   resolver ticks) — any read-back keyed on that incident surviving a long latency-only scenario is
   structurally unsound.

The harvest parser + predicates already ship (`crates/conductor-run/tests/baseline_harvest.rs`); the
latency predicates are live-unexercised and their fixtures source-derived.

## Boundaries (NOT this chunk)

- **v2-19 owns per-check latency / sub-5s representation** — explicitly excluded by the working entry;
  this chunk owns ONLY the P-011/P-012 regression-detection re-proof.
- **No Pulse-side changes.** Detectability comes from Conductor's scenario shape against the SUT at
  HEAD; the SUT's cue constants are read, never edited.
- **No new external crates.** The re-shape is declarative TOML phase data over the shipped emit
  primitives (`EmissionShape::Latency` → `latency_trace_request`'s stratified inverse-CDF sampler,
  verified able to realize any density/sharpness inside `MAX_OCCURRENCES = 10_000`) — PLUS one scoped
  Conductor code change the CARRY's ratified option requires: the declare-only row-state mapping in
  `crates/conductor-run/src/lib.rs` (post-green-preflight empty read-back → the pre-accepted
  auto-resolve residual instead of Blocked). [premise-corrected: the original "TOML-only" reading was
  too narrow — the "non-Blocked clause re-thought" is a code change by construction.]

## Surfaces / contracts touched

- `scenarios/latency-regression.toml` — phase re-shape (durations, densities, targets). SLO tier
  stays `<90s` (verified: no tier change — whole-run latency exceeds every tier by construction
  either way; the v2-12 narrowing precedent covers it).
- `crates/conductor-run/src/lib.rs` — the declare-only empty-read-back row-state mapping (see
  Boundaries) + unit tests beside the existing `state_for` tests.
- `crates/conductor-run/tests/baseline_harvest.rs` — latency predicates + verbatim leg pins.
- `contracts/pulse-load-envelope.toml` — gate re-run only; no term edit (verified: candidate shapes
  sit far inside both asserted terms; `[[exempt]]` stays empty).
- Catalog guards: verified unaffected — `conductor-core/src/scenario.rs:441-467` asserts
  load+validate, p_ids, and declare-only (all preserved by the re-shape); `cli_smoke.rs:206` is
  name-only.
- Live-leg harness per `.claude/rules/verification-harness.md` (boot-before-run dedupe, repo-relative
  `CONDUCTOR_*` handles, fresh `%TEMP%` data dirs, self-obs choreography cues).
- Evidence: `conductor-0.2.0/chunks/2026-08-20-latency-regression-re-proof/evidence/leg-verdict.md`.
- Verification matrix: [premise-corrected: NO matrix cap is claimable — the routed-forward latency
  half lives inside v2-12 (already `verified`; its notes route the half to this route entry), no
  unclaimed cap mentions P-011/P-012, and v2-19 is excluded by the entry itself. This chunk links
  nothing; its completion is recorded by the master `complete` flip, with a v2-12 `notes` addendum at
  wrap recording the forward pointer resolved.]

## PREREQ — 32nd consecutive `cargo audit` re-check (ratified protocol, folded verbatim)

Standing deferral since `2026-08-08-sut-capability-manifest`; ratified at the
`2026-08-10-workspace-key-divergence-probe` wrap, re-pins silently; basis RE-DERIVED at
`2026-08-19-connection-lifecycle-live-proof`: advisory-DATABASE fault — `cargo audit` probed
standalone true exit 1 byte-identical (`duplicate advisory ID: RUSTSEC-2026-0244`); the
zero-dep-delta footing ENDED that chunk — lock delta = one member-dep line (the run→faults edge),
zero new external crates; overlap VERIFIED, not assumed: `cargo deny check advisories bans licenses
sources` observed true exit 0 over the NEW lock that chunk. Remedy stays the bounded wait alone — no
floor raise, no `deny.toml` ignore, no CI edit; close the moment it parses. Full rationale:
`conductor-0.2.0/chunks/2026-08-14-canary-fingerprint-feed-capture/report.md`.

## Open premises — CLOSED at P3 (research.md carries the full mechanism)

- Pulse's `latency_regression` mechanism: pinned at HEAD `efabe8e` — `cue/evaluate.rs:85-140` over
  `baseline/mod.rs:483-510` snapshots; long = `TDigestPair::percentile` (union current+previous,
  60s rotation via `run_persist_cycle`, live-confirmed), short = `percentile_current_only` (15s
  rotation at the 1Hz emit cycle); floor `MIN_LATENCY_SAMPLES = 50` and confidence `samples/100`
  both read the LONG pair's `samples_current()`; threshold = `max(long_p99, 100ms) × 2.5`;
  suggested needs magnitude ≥ 3.0 + confidence ≥ 0.7; bypass at `latency_short > 1000ms`.
- Harvest surface: `triage.cue.emit` fields verified against `emitter.rs:194-205`; latency cues
  carry `scope: "operation"` (the shipped fixture says "service" — re-pin verbatim).
- Emit primitive reach: verified (stratified sampler realizes tight profiles; density via
  occurrences × samples over gap).
- Matrix cap identity: resolved above (none claimable).

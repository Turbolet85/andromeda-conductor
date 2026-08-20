# Codebase Research — 2026-08-20-latency-regression-re-proof

## Scope
- **Depth:** deep · **Reads:** 16 files (9 Conductor + 7 andromeda-pulse at HEAD `efabe8e`) · **Globs/Greps:** ~14 · **Graph queries:** 2

## Files inspected
- `scenarios/latency-regression.toml` (full) — the shipped 90/90 shape: baseline 90s (`occurrences 3 × samples 60`, p99 800) + ramp 90s (3×60, p99 2400 = 3×), seed 4317011, `slo_tier "<90s"`, DECLARE-ONLY (zero `[[expected]]`), header already names the harvest target (`triage.cue.emit`, kind `latency_regression`, magnitude ≥ 3.0, confidence ≥ 0.7, priority suggested).
- `contracts/pulse-load-envelope.toml` (full) — asserted per-emitting-phase terms `max_sustained_rate_spans_per_s = 10000` · `max_sustained_storm_ms = 600000`; `max_scenario_duration_ms = 600000` recorded-not-asserted (read only by the run contract's preflight-budget check, `run_contract.rs:186-195` — the PREFLIGHT budget, not scenario length); `[[exempt]]` EMPTY, exact-set equality both directions.
- `crates/conductor-run/tests/baseline_harvest.rs` (full, 341) — shipped predicates `cue_fired` / `convergence_witness` (mag ≥ 3.0, conf ≥ 0.7) / `suggested_tier` / `absolute_bypass_witness`; parser keyed on `target == "triage.cue.emit"` reading `fields.{kind,priority,scope,magnitude,absolute_value,persistence_seconds,confidence,suppression_bypassed}`; harvest = `{data_dir}/logs/agent-latest.jsonl.<date>` sliced by pre-leg line count; latency test uses the SOURCE-DERIVED `cue_line()` builder (scope `"service"`) — the verbatim-pin slot for this chunk's leg. Error-half leg-A pins live here and must not be disturbed.
- `crates/conductor-run/src/lib.rs:320-470` — `execute_scenario`: preflight-gate → egress probe → `emitted_ms` stamp → timeline+dispatch → `observe(client)`; `ReadBackOutcome::EmptyCorpus | CallFailed` → **`RunRecord::blocked` for ALL scenarios** (`lib.rs:371-385`, the leg-B Blocked); declare-only branch (`expected.is_empty()`) → operator hold → `manual_record` (ManualCheck; `state_for` flips to KnownResidual when `observation.degraded`, `lib.rs:445`). The row-state mapping change lands here; `state_for` unit tests at `lib.rs:735-806` are the pattern to extend.
- `crates/conductor-run/src/dispatch.rs:115-124` — `EmissionShape::Latency { operation, p50_ms, p95_ms, p99_ms, samples }` → one `latency_trace_request` batch per occurrence (occurrences paced evenly across the gap by the timeline).
- `crates/conductor-emit/src/latency.rs` (24-148) — `LatencyProfile` (p50≤p95≤p99), stratified inverse-CDF sampler (`sample_durations_nanos`: stratum i draws in [i/n,(i+1)/n) through the piecewise-linear quantile) — tight realization at any n; OK-status root spans named by `operation`, seeded ids.
- `crates/conductor-core/src/phase_spec.rs:95-130, 240-275` — `MAX_OCCURRENCES = 10_000` bounds both `occurrences` and latency `samples`; latency shape validation = ordering + samples 1..=10000; occurrences paced evenly (comment at :101).
- `crates/conductor-core/src/scenario.rs:441-467` — the two catalog guards naming this scenario: `statistical_anomaly_fixtures_load_and_validate` (load + p_ids P-011/P-012) and `statistical_anomaly_fixtures_are_declare_only_at_the_harvest_tier` (expected empty). Both survive any re-shape that keeps p_ids + declare-only. `cli_smoke.rs:204-211` lists the name only.
- `conductor-0.2.0/chunks/2026-08-18-error-baseline-spike-live-proof/evidence/leg-verdict.md` (full) — leg B's two measured obstacles (quoted in scope); leg A's row/KnownResidual precedent; the harness `SEED` fix note.
- `conductor-0.2.0/verification-matrix.json` — v2-12 (verified) carries the routed-forward latency clause + full premise-correction narrative; **no unclaimed cap mentions P-011/P-012/latency-regression** (grep over all 32); v2-19 explicitly excluded by the working entry.
- **andromeda-pulse (SUT, HEAD `efabe8e`):**
  - `crates/triage/src/cue/evaluate.rs:9, 81-140` — the latency loop: per-operation snapshot; skip if `snapshot.samples < min_latency_samples`; `latency_short` = short read; `baseline = max(long_p99, base_latency_ms)`; fire when `latency_short ≥ baseline × latency_multiplier`; `magnitude = latency_short / baseline`; `confidence = min(1, samples/100)` (`CONFIDENCE_SATURATION_SAMPLES = 100.0`); `persistence_seconds = snapshot.samples` (a count, not seconds); `absolute_value = latency_short`; `scope_id = operation_name` → cue `scope: "operation"`.
  - `crates/triage/src/cue/thresholds.rs:65-125` — `DEFAULT_LATENCY_MULTIPLIER = 2.5` · `DEFAULT_BASE_LATENCY_MS = 100.0` · `DEFAULT_TICK_INTERVAL = 1s` · `DEFAULT_LATENCY_PERCENTILE = 0.99` · `MIN_LATENCY_SAMPLES = 50` · `DEFAULT_MAGNITUDE_BYPASS_MULTIPLIER = 10.0` · `DEFAULT_ABSOLUTE_BYPASS_LATENCY_MS = 1000.0`.
  - `crates/triage/src/baseline/tdigest_pair.rs` — `TDigestPair` = current + previous, `swap_on_tick(now, window)` rotates (previous ← current, current ← fresh; FIRST tick after creation swaps immediately since `last_swap == 0`); `percentile(q)` = **union of current+previous**; `percentile_current_only(q)` = current only; `samples_current()` = count since last rotation.
  - `crates/triage/src/baseline/mod.rs:101, 320-360, 483-510, 570-595` — TWO pairs per operation (`latency_tdigest` long, `latency_tdigest_short` short); both fed per span (`insert` at :324-326); LONG rotates in `run_persist_cycle` with `DEFAULT_PERSIST_INTERVAL_NANOS = 60s` (:577, :95); SHORT rotates in the 1Hz emit cycle with `DEFAULT_SHORT_SWAP_INTERVAL_NANOS = 15s` (:355, :101); snapshot: long = union read, short = current-only read, **`samples` = the LONG pair's `samples_current()`** (:497) — the floor AND confidence count resets at every 60s rotation.
  - `crates/triage/src/cue/classify.rs:14-60` — Suggested = mag ≥ 3.0 ∧ conf ≥ 0.7; Autonomous = mag ≥ 5.0 ∧ conf ≥ 0.9 ∧ persistence ≥ 30; latency absolute bypass arm = `absolute_value > 1000ms`.
  - `crates/triage/src/cue/emitter.rs:20-90, 180-215` — 1Hz cycle: swap short pairs → evaluate → emit each cue as a `triage.cue.emit` INFO line (re-emitted every tick while the condition holds; leg A measured 95 lines).
  - `pulse-app/src/main.rs:1303` + `pulse-app/src/incident_observer.rs:24` + `crates/triage/src/incident/persistence.rs:41` — the persist loop RUNS live at 60s (confirmed: 5 `baseline tracker tick` lines in retained leg B4's Pulse log at `%TEMP%/pulse-legs/2026-08-20T0107-b4`); auto-resolve tick 30s, idle window `DEFAULT_INCIDENT_AUTO_RESOLVE_WINDOW_SECS = 120` keyed on `updated_at` (`registry.rs:304`).

## Graph impact (adoption trace: `tree-query-2026-08-20-latency-regression-re-proof.json`)
- **`LoadEnvelope` / `check_load_envelope`** — 119 caller rows: `conductor-cli` paths (`paths.rs:10-44`), `conductor-core::lib`/`load_envelope.rs` internals + catalog tests, `conductor-run` tests. No signature changes planned → read-only relevance; the catalog gate re-runs untouched over the new TOML.
- **`%Latency%` symbols** — 47 rows: `EmissionShape::Latency` (conductor-core), `LatencyProfile`/`LatencyOp`/`latency_trace_request` (conductor-emit), the dispatch arm (conductor-run), harvest tests. The re-shape is data-only over these; the row-state change touches none of them (internal to `execute_scenario`/`manual_record`, no signature change → no caller threading).

## The detectability mechanism (the chunk's core finding)

The CARRY's "accumulated history" is NOT a 5-minute window (the TOML header's "5-min window" is stale
prose): the long baseline = union(current, previous) of a t-digest pair rotating every **~60s on
pulse-app's persist tick** (anchor: pulse-app boot, quantized at 60s; per-pair epoch: first persist
tick after the operation's first sample). Consequences:

1. **Only the last ~60–120s of samples form the baseline denominator.** A "longer" baseline beyond
   ~2 rotations buys nothing; DENSITY of that window is the lever.
2. **The floor + confidence ride the long pair's CURRENT count** (resets each rotation): at spike
   start the count = φ·R_b (φ = seconds since the last rotation, φ ∈ [0,60) effectively
   uncontrollable because the rotation clock is anchored to pulse-app boot, and preflight duration
   varies). φ·R_b ≥ 50 needs φ ≥ 2.5s at R_b = 20/s.
3. **Contamination bound:** the union's p99 stays ≈ baseline only while spike samples < ~1% of the
   union count. With previous = 60·R_b and current = φ·R_b + t·R_s, a spike at R_s = 0.25–1/s keeps
   f < 1% for its first ~20–60s. Past ~1%, q=0.99 lands inside the spike population and magnitude
   collapses toward 1 (the leg-B absorption, now with exact mechanics).
4. **An in-spike rotation** (at 60−φ) moves the mixed current into previous and empties the count —
   after a second rotation the union is spike-dominated. The detectable episode is therefore bounded
   by roughly the first in-spike rotation + a few seconds.
5. **φ-robustness by construction:** a single spike dies when φ < ~2.5s (floor never clears before
   contamination). TWO spike pulses offset by ≡30 (mod 60) seconds of timeline guarantee the two φ
   draws differ by 30s, so at most one can land in the narrow dead zone — the recipe stops being a
   lottery. (The `baseline tracker tick` lines in the harvest let φ be measured post-hoc either way.)
6. **The short side is easy:** the short pair rotates every 15s and its read is current-only with NO
   floor — a handful of spike samples put short_p99 at the spike value. A NARROW spike profile
   (p50≈p95≈p99, e.g. 3000/3100/3200 vs baseline 100/400/800) makes every sample ≥ 3.75× the
   baseline p99, so magnitude ≥ 3.0 holds even if the union drifts partway.
7. **Clean side-effects:** latency spans are OK-status → no error cues; silence family is
   bootstrap-gated (3600s) → unreachable; suggested-tier latency cues form NO incident on this host
   (Tier-2 cadence skipped on cpu-primary — settled at v2-12) → the canary incident is the only
   incident, and it auto-resolves (120s idle / 30s ticks) during any shape this chunk can use.

## Patterns detected
- **Harvest-tier declare-only grading** (`baseline_harvest.rs` + 4 sibling harvest files): row lands
  non-Blocked KnownResidual; live claims grade as unit-tier predicates over verbatim-pinned Pulse
  lines. The row's non-Blocked clause currently FAILS for this family (leg B: empty active list →
  `RunRecord::blocked`, `lib.rs:371-385`) — the mapping change is the re-think the CARRY ratifies.
- **Verbatim leg pinning with recorded-not-asserted arms** (restart precedent; `storm_harvest.rs` →
  `connection_harvest.rs` lineage).
- **`state_for` override pattern** (`lib.rs:440-455`): degradation is a property of the SUT's
  response, applied wherever observed; verdict stays independent. The auto-resolve residual extends
  exactly this shape.
- **Additive `RUST_LOG=info,{crate}=debug`** if a debug witness is ever needed (2026-08-14 amendment);
  cue lines are INFO — no opt-in needed.
- **Service-identity split** (`conductor` scenario vs `conductor-canary` preflight) keeps the
  spike's per-operation history clean of canary traffic.

## Conventions to follow
- Verdict/error wall: the new mapping returns `Ok(RunRecord)` states, never `Err` (`lib.rs:352-368` precedent).
- Never a false pass-as-empty (security-plan): only the DECLARE-ONLY path may convert EmptyCorpus
  (nothing grades against the observation there); scenarios with `[[expected]]` checks keep Blocked;
  `CallFailed` keeps Blocked everywhere. Attribution guard: conversion only after a green preflight
  in the same process (the canary PROVED the corpus reachable this run).
- Journal stamps from `std::time` (`now_ms`/`now_rfc3339` already in the path).
- No new `CONDUCTOR_*` env vars, no new crates, no envelope/exempt edits.
- TOML seed honored by the harness when `SEED` unset (the 2026-08-18 §3 fix) — the leg runs the
  TOML-declared seed.

## New files to create
- `conductor-0.2.0/chunks/2026-08-20-latency-regression-re-proof/evidence/leg-verdict.md` (at implement, from the live leg).

## Files to modify
- `scenarios/latency-regression.toml` — the re-shaped phase list (dense ~60–120s-window baseline at
  ~20/s; two narrow-profile spike pulses at ~0.25–1/s, ~30–40s each, timeline-offset ≡30 (mod 60);
  header prose re-based to the measured 60s-rotation mechanism; declare-only + seed + tier + p_ids
  unchanged).
- `crates/conductor-run/src/lib.rs` — `execute_scenario`'s `EmptyCorpus` arm: declare-only scenarios
  proceed to the manual/KnownResidual record carrying the auto-resolve residual (empty observation,
  no fingerprints, latency stamped); checks-bearing scenarios and `CallFailed` unchanged (Blocked).
  Unit tests beside `state_for`'s (`lib.rs:735+`).
- `crates/conductor-run/tests/baseline_harvest.rs` — the latency predicates re-pinned VERBATIM from
  this chunk's leg (captured `latency_regression` line: expect `scope:"operation"`, magnitude ~3–4,
  confidence 1.0, `suppression_bypassed:true` via the 1000ms absolute arm); leg-A error pins untouched.

## Open questions
- none — the two design forks the CARRY left open are both closed by source: incident-route dead
  (Tier-2 skipped on cpu-primary), and the φ-lottery is closed by the two-pulse shape. The exact
  recipe numbers (rates, pulse lengths, profile values) are P4 synthesis over the §mechanism bounds.

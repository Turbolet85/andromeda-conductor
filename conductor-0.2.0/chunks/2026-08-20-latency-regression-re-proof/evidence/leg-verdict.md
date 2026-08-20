# Leg verdict — latency-regression re-proof

**Run:** `2026-08-20T16-52-49-662` (seed 4317011 — the TOML's, `SEED` unset), fresh data dir
`%TEMP%/pulse-legs/2026-08-20T1855-lat-b`. Wall window 16:52:49 → 16:59:49 UTC (420s: preflight
then 373.8s of emission).
**SUT:** andromeda-pulse HEAD `efabe8e` (unchanged — the TIME-axis obligation discharged), the
ratified binary pair (`pulse-app.exe` 2026-08-17 23:04 · `andromeda-pulse-mcp.exe` 2026-08-17 22:27).
**Recipe:** the six-item set — `ANDROMEDA_PULSE_DATA_DIR` (fresh, Windows form, pulse-app cwd = data
dir) · `ANDROMEDA_PULSE_MCP_ENABLED=1` · `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` · sidecar via `PATH`
(debug dir) · NO `SEED` · NO `RUST_LOG`. Conductor invoked DIRECTLY
(`target/debug/conductor.exe run latency-regression --agent-mode`) — never `boot` first, whose canary
would leave an OPEN incident the run's own canary dedupes against.

All quoted values are from the harvested Pulse self-obs slice (131,050 lines, pre-leg count 25) and
Conductor's own journal. No absolute host path appears in any quoted line.

## The result — P-011/P-012 PROVEN at the harvest tier

**31 `triage.cue.emit` lines with `kind: "latency_regression"`; priority histogram 27 suggested ·
4 curious.** The shipped 90s/90s ramp produced ZERO across its whole leg (2026-08-18 §Leg B); the
re-shape produces them on both pulses.

| Predicted (from the SUT's own constants) | Measured |
|---|---|
| cue fires (existence proves the ≥50-sample floor — the evaluator skips below-floor operations) | **31 lines** |
| `priority "suggested"` (Autonomous needs ≥5x + conf ≥0.9) | **27 of 31**; magnitude 3.50–3.68, never ≥5 |
| magnitude ≥ 3.0 (short p99 ≈3000–3073 over a ≈805–847 baseline) | **3.57 / 3.67** on the pinned pair |
| confidence ≥ 0.7 (`samples/100`, capped) | **1.0** |
| `suppression_bypassed` via the absolute arm (>1000ms) | **true**, `absolute_value` 3024 / 3073 |
| `scope "operation"` (latency is operation-scoped; the error path is service-scoped) | **operation** |
| first cue lands ~13–17s into a pulse (short pair must rotate spike-clean) | **16:56:48 = 13s into spike-a** |

**Both pulses fired**, which is the design's whole point: pulse A at 16:56:48–16:57:21 and pulse B at
16:59:18–16:59:48. Two lines are pinned VERBATIM in
`crates/conductor-run/tests/baseline_harvest.rs` (`captured_latency_pulse_a_line` /
`captured_latency_pulse_b_line`).

## Why the re-shape works (the mechanism, measured)

`persistence_seconds` is a SAMPLE COUNT (the long t-digest pair's current-window count), and it
**reset 2456 → 68 between 16:56:48 and 16:57:19** — direct evidence the long pair rotates on its
~60s cycle, resetting the floor and the confidence denominator mid-leg. That rotation phase is
anchored to pulse-app's own clock and is not controllable from a scenario, so a single pulse is a
coin flip: land just after a rotation and the ≥50-sample floor cannot clear; land just before one
and the window closes mid-pulse. **The two pulses are 150s apart (≡30s modulo the 60s cycle), so
their rotation phases always differ by half a cycle and at most one can fall in a dead zone.** The
leg measured exactly that: pulse A opened *curious* at magnitude 2.895 while the short window was
still baseline-diluted, then the rotation at ~16:57:18 dropped the count to 68 and the very next
cues went suggested at 3.57.

The other half is contamination: the anomaly must not become the history it is judged against. 24
spike spans against a 50-span/s baseline is ~0.3% of the union, so the long p99 held at ≈805–847
(magnitude 3.57 = 3024/847) instead of climbing to absorb the spike — the exact failure the 90/90
ramp suffered, where ~half the window's samples sat at the regressed profile.

## The row

```json
{"journal_emitted_at":"2026-08-20T16:53:35Z","read_back_observed_at":"2026-08-20T16:59:49Z",
 "run_id":"2026-08-20T16-52-49-662","seed":4317011,"scenario":"latency-regression",
 "p_ids":["P-011","P-012"],"verdict":null,"state":"KnownResidual","latency_ms":373784,
 "slo_tier":"<90s","fingerprints":["det-span-…","det-template-0007","det-fingerprint-…"]}
```

Non-Blocked `KnownResidual` with `verdict: null` — the declare-only path — `latency_ms` recorded
journal-relative, `slo_tier <90s`, seed the TOML's (the 2026-08-18 SEED-forcing retirement holds),
fingerprints the constant payload-invariant `det-*` triple. Exit 0, rendered `[RESIDUAL]`.

**One honest limit on the row.** It reached `KnownResidual` by the DEGRADED path, not by this
chunk's new auto-resolve route: `query_incident_list` returned `result_count: 1` at read-back — the
canary incident SURVIVED the 373s window, because the canary's own error-rate EWMA kept emitting
`error_rate_spike` cues (375 of them) that refreshed the incident's `updated_at` and held off the
120s idle auto-resolve. So the empty-active-list arm added in `conductor-run` is **unexercised
live** on this leg; it is covered at the unit tier
(`a_declare_only_empty_read_back_routes_to_the_auto_resolve_residual` and its two negative twins)
and remains the correct guard for the case 2026-08-18 §Leg B measured, but this leg is not evidence
that it fires. Recording it rather than letting the green row imply more than it proved.

## Corrections to the plan's premises

- **`baseline tracker tick` never appears in this leg's harvest (0 lines)** although
  `run_persist_cycle` is what carries the long-pair rotation in the SUT source. The rotation
  demonstrably happens anyway (the 2456 → 68 reset), so the tick line is not a usable rotation
  witness on this host — `persistence_seconds` is. A mid-leg reading of that absence as "the long
  pair never rotates" was wrong and is corrected here.
- **Ingestion is clean at this density**: every `duckdb.append` for `spans` recorded
  `rows_appended: 50` against 50-span batches — zero PK-collision drops, the per-occurrence
  `emission_seed` doing its job at 180+110 dispatches.
- **No false positives**: zero latency cues during either baseline phase; the dense baseline does
  not read as a regression against its own history.

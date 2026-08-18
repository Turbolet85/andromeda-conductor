# Leg Verdict — 2026-08-18-error-baseline-spike-live-proof

Two operator-gated live legs against `pulse-app` at SUT HEAD `efabe8e` (deterministic L4; FRESH data
dir per leg with a restart between; six-item recipe; sidecar + app both built at that HEAD). All
quoted values are from the two harvested `agent-latest.jsonl.2026-08-18` slices (Pulse side, sliced
by pre-leg line counts 1769 / 2119) and Conductor's own `logs/agent-latest.jsonl` — no absolute host
path appears in any quoted line.

## Leg A — error-baseline-spike (run_id `2026-08-18T18-47-32-786`, seed 424242) — PROVEN

- Preflight `ready:true`; canary storm `suggested@5` → `autonomous@10` (18:48:18); incident
  `created:true, deduped:false` at 18:48:18.178.
- Scenario window 18:48:19 → 18:50:50; journal-relative `latency_ms 151124`.
- **`error_rate_spike` cues: 95 `triage.cue.emit` lines.** Early window `curious` (confidence
  0.14–0.15, magnitude ~8.5 while the long EWMA was still converging from zero); late window
  **`suggested` with confidence 1.0 and magnitude 3.09–3.19** — the baseline genuinely converged
  (long EWMA ≈ 0.088 against the authored 10%), and every line carried
  `suppression_bypassed: true` (absolute 0.27–0.33 > the 0.05 P-057 arm). Priority histogram:
  59 curious · 36 suggested · 0 autonomous (magnitude < 5 once converged — Tier-1 unreachable, as
  researched). The last suggested line (18:50:31) is pinned VERBATIM in
  `crates/conductor-run/tests/baseline_harvest.rs`.
- **Row:** `state KnownResidual` · `verdict null` (declare-only path) · `slo_tier <90s` ·
  `fingerprints` = the 3 `det-*` refs — the de-vacuumed deterministic-L4 `evidence_refs`, measured
  live on a Conductor envelope for the first time.
- **Key-set witness vs the pinned baseline — ZERO divergence** (first non-empty-corpus reach of the
  per-check readers): `query_incident_list [items, next_cursor, total]` ·
  `retrieve_report [degraded_mode, markdown]` ·
  `retrieve_telemetry_slice [fingerprint_refs, incident_id, span_refs, timestamps_unix_nano]`.
- **Transcription equality PROVEN LIVE:** Conductor computed
  `bf2c0bf81e6c18189b4e48ca2cf46112` (the new `emit_canary` witness); Pulse's storm line carried
  `fingerprint_hex "bf2c0bf8"` — prefix-8 EQUAL. The verbatim pair is pinned as
  `the_leg_a_capture_proves_the_canary_prefix_equality`. The token-leading transcription
  (2026-08-17) is thereby confirmed against a live Pulse, discharging the CARRY with a match.
- Recorded observation (not a blocker): a SECOND incident `created:true` at 18:49:56
  (`priority_tier autonomous`, `severity error` — both L4-fixture-mapped fields, not cue tiers)
  never appeared in `query_incident_list` (count stayed 1) — consistent with a workspace-key split
  between the app-detected root and the data dir for non-OTLP-attributed digests; a third outcome
  line read `created:false, deduped:true`.

## Leg B — latency-regression (run_id `2026-08-18T18-53-50-494`) — BLOCKED, two measured obstacles

- Preflight `ready:true` (canary incident created 18:54:35); both phases emitted; the scenario
  read-back at 18:57:36 found `query_incident_list` EMPTY → the row is honestly
  `Blocked` ("read-back yielded no gradable observation"), identity fields only, measurements null.
- **Obstacle 1 — the canary incident auto-resolves under a benign window.**
  `triage.incident.auto_resolve.tick` fires every 30s; the canary incident idled through the 180s
  latency window (nothing re-raises its identity — latency spans raise no error cues) and left
  `list_active` before read-back. Leg A survived exactly this because its own error traffic
  re-raised the same incident identity (the observed `deduped` re-emission). Basis: created at
  18:54:35 + present at the canary poll + absent at 18:57:36 + the ticking resolver; the tick's
  `resolved_count` values are redacted in Pulse's log, so this is a strongly-supported inference,
  not a counter reading.
- **Obstacle 2 — the latency cue cannot fire under the shipped shape.** ZERO `latency_regression`
  cue lines across the whole leg. Mechanism: the long-window t-digest p99 absorbs the ramp — with
  90s/90s halves, the 180 ramp samples at p99≈2400 dominate the mixed long window, so the
  threshold (long·2.5) rises toward ~6000 while the short window sits at ~2400 and can never cross
  it. Detection requires the regression to be SHORT relative to accumulated history; the shipped
  recipe saturates it. The research model's "long ≈ 800" was wrong for a percentile over a mixed
  window (an EWMA-vs-window-percentile category error).
- Harness fidelity note: `agent-run.sh` forced `SEED=424242` onto the leg (the TOML declares
  4317011). No bearing on either obstacle — the seed drives emission identity/jitter, not cue
  math — recorded because the row carries the forced seed.

## Consequence

The concretized v2-12 acceptance is satisfied for the ERROR half (leg A: candidate + convergence +
floors + suggested tier + bypass witness + KnownResidual row + equality + key-set parity) and
DISPROVED for the LATENCY half by two structural obstacles that hold under the plan's own
"timing untouched" constraint. Surfaced for wrap's escalation per the matrix contract §Acceptance
lifecycle — nothing was authored, the matrix entry stays `planned`/claimed. The ungradeable-token
settlement itself was confirmed exactly as researched: the report served degraded with fixture
constants, `span_refs` stayed 0, and the retired checks could never have graded either leg.

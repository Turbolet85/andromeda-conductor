# Live-leg verdict — P-025 hue budget re-driven

**Suite:** `run --live`, 2026-09-07. **SUT:** Pulse HEAD `83d4060`, release build, launched
2026-09-07T07:39:36Z on a fresh data dir with deterministic L4 + MCP.
**Boot-wide posture:** `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS=86399`, witnessed by the SUT's own
`triage.baseline.bootstrap_window.override` line at 07:39:37.120Z (`reason="env_override"`,
`resolved_seconds=86399`, `default_seconds=3600`) — emitted **exactly once**, which is the boot-wide
property (`Thresholds::from_env()` resolves once at `main.rs:472`), not a per-leg one.
**Pre-leg baseline:** 50 243 lines, captured after readiness and before the first dispatch.

## Leg outcomes — every leg at its predicted state

| Leg | Scenario | State | verdict | latency_ms | tier | run_id |
|---|---|---|---|---|---|---|
| H | `halo-hue-encoding` | `ManualCheck` | null | 183 768 | `<90s` | `2026-09-07T07-42-45-582` |
| B1 | `degraded-mode-report` | `ManualCheck` | null | 6 020 | `<20s` | `2026-09-07T07-46-35-991` |
| B2 | `degraded-mode-report` | `Blocked` | null | — | `<20s` | `2026-09-07T07-47-28-669` |
| A | `auto-resolve-idle-window` | `KnownResidual` | null | 200 041 | `<90s` | `2026-09-07T07-52-14-133` |

Driven a11y arm: **1 passing** in 3 m 52 s on WebView2 152.0.4191.66. No `[ENVIRONMENT-SUSPECT]`
caption on any leg — the re-shaped phases stayed inside the load envelope unaided.

**CARRY 3.3 discharged — and immediately qualified by the wrap's own re-run.** Leg A reached
`KnownResidual` here, the `ReadBack::AutoResolved` arm that did NOT reproduce on 2026-09-06.

**The light-gate re-run (09:07Z, same tree, same posture, same `pulse-app` process) graded it
`ManualCheck`.** That is the honest headline for this CARRY, so both runs are recorded:

| Run | leg A window | only `created:true` in window | age at read-back | outcome |
|---|---|---|---|---|
| 1 (07:52Z) | 200 s | 07:52:59 (**+45 s**) | ~155 s — cleared 120 s idle + 30 s tick | **`KnownResidual`** |
| 2 (09:07Z) | 200 s | 09:09:37 (**+137 s**) | ~62 s — inside the idle window | **`ManualCheck`** |

**The posture is doing its job in BOTH runs.** `triage.baseline.service_went_silent.evaluate` reports
`services_in_bootstrap: 2` · `services_ready: 0` · **`silence_cues_emitted: 0`**, and the only cues
near either window are `error_rate_spike`/`suggested`. So cause (a) — silence cues refilling the
active set — is genuinely removed by the stretched boot-wide window, at a point (Pulse up ~1 h 28 m)
where the DEFAULT hour would long since have expired.

**What the posture does not touch is cause (b)**, and cause (b) is what decided both runs: the arm
fires only when the window's last incident forms early enough to clear 150 s before read-back.
A wider `gap_ms` does not fix it — it moves read-back later but leaves proportionally more room for a
late incident. **So the stretched posture makes the arm REACHABLE, never RELIABLE, and this leg
remains not run-stable under it.** That correction was applied to `architecture` §69, `test-plan` §9,
`verification-harness.md` and the scenario header before this chunk was committed.

## The measurement — P-025

Leg H drove **360 dispatches at 2/s** across two phases (`emission_count: 360`, `timeline.execute`
spanning 183.8 s), so the subject service was emitting CONTINUOUSLY through its tier flip. This is
what the 2026-08-21 legs could not claim, and it is what makes the result decisive.

The capture held **7** `metric.constellation.hue_update_ms` samples after the pre-leg baseline.
Exactly **one** falls inside leg H's window (phase-2 start `1788767041678` → `scenario.run` close
`1788767195445`):

| | timestamp | duration_ms | severity_tier | offset to preceding tick | \|diff\| |
|---|---|---|---|---|---|
| **in-window** | 07:44:07.658Z | **14 525.947** | autonomous | 14 527 | **1.1 ms** |

**14 525.9 ms against a 2 000 ms budget — on a leg where the service never stopped emitting.**

### The mechanism, confirmed across every sample

Lifecycle tick spacing measured **n=76, min 14 986 ms, median 15 000 ms, max 15 013 ms** — the 15 s
heartbeat. And every one of the 7 samples satisfies

```
duration_ms  =  offset_to_preceding_tick  +  k × 15 000      (k ∈ {0, 1, 2}, residual ≤ 2 ms)
```

| duration_ms | offset | diff | k |
|---|---|---|---|
| 38 515.77 | 8 532 | 29 983.8 | **2** |
| **14 525.95** | **14 527** | −1.1 | **0** |
| 15 521.59 | 520 | 15 001.6 | **1** |
| 518.72 | 520 | −1.3 | 0 |
| 526.01 | 527 | −1.0 | 0 |
| 6 532.11 | 6 534 | −1.9 | 0 |
| 529.11 | 531 | −1.9 | 0 |

`k` is how many ticks back the service was last SEEN: `k = 0` while it is emitting (every tick
refreshes `last_seen`), `k > 0` once it goes quiet and the refresh stops. The two `k > 0` rows are
the canary (quiet since preflight) and a post-leg `tier=none` transition after the scenario ended —
so they corroborate the mechanism rather than contradicting it.

### What this settles

The observable reports `t_sample − t_last_refreshing_tick`, never an update latency. Meeting 2 000 ms
requires the tier flip to land inside the first 2 s of a 15 s window — a ~13 % coincidence,
independent of dispatch rate. **The ≤2 s bound is unmeasurable through this leaf** while
`ServiceRegistryEntry.last_seen_unix_nano` has no ingest-path writer (`registry.rs:336`, gated on
`current_quiet_duration_seconds == 0`; `activity_floor.rs:142-149`;
`DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL = 15 s` at `lifecycle/mod.rs:52`).

No arm asserts a pass. The measurement is recorded and the mechanism is pinned by
`delegated_timing_harvest.rs::p025_the_re_driven_leg_measures_tick_quantization_not_update_latency`
over the verbatim lines above.

**SUT-side fix (Pulse intake, not Conductor's):** have the registry copy the activity floor's
`last_observed_unix_nanos`, or have the fire site read span arrival directly. Either makes the bound
measurable; neither is in this chunk's scope.

## Hygiene

All four committed captures scan clean for absolute host paths — zero hits for the word-anchored
drive-letter form, `%APPDATA%`, `/Users/`, `/home/`, `.cargo`, `.rustup`, and zero for
`pulse-legs`/`AppData`. Zero `<redacted>` markers, so the streams are clean **by construction**
rather than by the scrubber having fired.

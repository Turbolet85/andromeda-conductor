# Leg verdict — 2026-08-21-severity-lifecycle-live-proof

Five fresh-data-dir legs against a live deterministic-L4 Pulse at SUT HEAD `efabe8e`, 2026-08-21
08:59–09:23Z. Every leg: `conductor run <scenario> --agent-mode`, exit **0**, row persisted.
Evidence per leg under `leg-{a..e}/` — Conductor's own `agent-latest.jsonl` + `conductor.stderr`
(sidecar stream) + `run.jsonl` (the envelope) + `pulse-events.jsonl` (Pulse's own lines, filtered
from the raw capture to the six load-bearing targets; leg A's raw slice was 138,065 lines / 48 MB,
of which 128,820 were `metric.webgpu.frame_duration_ms`).

## The rows — all five landed as declare-only predicted

| leg | scenario | verdict | state | latency_ms | slo_tier |
|---|---|---|---|---|---|
| A | incident-auto-resolution | `null` | KnownResidual | 365827 | `<90s` |
| B | severity-tier-autonomous | `null` | KnownResidual | 121307 | `<5s` |
| C | severity-tier-suggested | `null` | KnownResidual | 82923 | `<20s` |
| D | severity-tier-curious | `null` | KnownResidual | 26334 | `<90s` |
| E | ack-cooldown | `null` | KnownResidual | 370001 | `<20s` |

CLI rendered `[RESIDUAL] <scenario>` — inside the closed six-label set, never a new lamp.

## P-022 auto-resolve — PROVEN, and at the predicted instant

Leg A, on Pulse's own ledger (`leg-a/pulse-events.jsonl`):

```
09:00:27.965  storm.detected   fp=12dcd67b n=10 severity_hint=autonomous
09:00:28.046  incident.created created=true deduped=false priority_tier=autonomous
09:00:28.188  list_active      item_count=2
09:02:15.675  list_active      item_count=1     <- the canary's incident (created 09:00:07.026)
09:02:45.671  list_active      item_count=0     <- the scenario's
09:06:09.625  storm.detected   fp=12dcd67b n=10 severity_hint=autonomous
09:06:09.695  incident.created created=true deduped=false
```

Both resolutions land at exactly `created + 120s → next 30s observer tick`:
canary `09:00:07.026 → 09:02:15.675` (128.6s), scenario `09:00:28.046 → 09:02:45.671` (**137.6s**),
both inside the 150s bound and neither merely "somewhere inside" it. `triage.incident.persist`
reports `incident_count: 0` at 09:03:15 — the corpus agrees.

**New-not-reopen** is the second, independent proof: the retrigger carried the SAME fingerprint
(`12dcd67b`, pinned in both storm lines) and still produced `created=true, deduped=false`. Pulse
dedups against the ACTIVE set only, so had the incident still been open the line would read
`created=false, deduped=true`.

**Ingestion was lossless:** cumulative `span_count` ended at **189** = 15 canary (3 warm-up + 12
storm) + 12 trigger + 150 dilution + 12 retrigger. Zero PK-collision drops.

## Two witnesses the plan named turned out not to carry the claim

1. **`triage.incident.auto_resolve.tick` counters are `"<redacted>"` on the wire** —
   `resolved_count`, `evaluated_count` and `duration_ms` all render as the literal string, because
   Pulse's observer field allowlist does not admit them. The tick line proves the observer RAN and
   can never prove it RESOLVED anything. The auto-resolve witness is instead
   `incidents.list_active.request.item_count`, which is unredacted. (The 2026-08-18 rendering-clause
   lesson firing again, on a different field.)
2. **The corpus-backed read-back and the in-app ledger DIVERGED.** At leg A's read-back (09:06:13)
   the sidecar's `query_incident_list` returned **2** incidents while `incidents.list_active`
   reported **1** at 09:06:10. They agreed earlier in the leg (both 1 at 09:00:07) and diverged only
   after the two auto-resolutions, so at least one resolved incident's corpus row still reads active
   to the sidecar. Cause not established from logs alone (identifying it would need a direct
   `corpus.db` read, which is out of bounds) — recorded as SUT intake, and as an honest limit on
   read-back absence assertions.

Consequence for the concretized v2-16 acceptance: its OUTCOME clauses hold and are proven more
strongly than written, but two named MECHANISM sub-clauses ("hard pass/fail from the harvested
`auto_resolve.tick`", and "the final active-only list … no longer carries the resolved one") are
contradicted by measurement. Surfaced for wrap, never silently edited.

## P-059 resolution summary — the premise correction is now MEASURED

**Zero** lines naming `resolution_summary` / `ResolutionSummary` across all five legs. This confirms
live what research established at source: the summary attaches only on `DigestKind::ResolutionSummary`
or an L4 output whose `is_resolution_summary` is true, nothing in the SUT constructs that digest kind,
and the deterministic fixture pins the flag false. The mechanism exists SUT-side and is unreachable in
the mode every verifiable run uses.

## P-019/P-020/P-060 tier ladder — three bands, separated by the authored term

| leg | baseline samples | first cue | magnitude | confidence | band |
|---|---|---|---|---|---|
| B | 90 | persist 91 | 3.330 | 0.91 | suggested |
| B | " | persist 92 | **6.549** | 0.92 | **autonomous** |
| C | 70 | persist 71 | 3.330 | 0.71 | **suggested** |
| D | 20 | persist 21 | 3.330 | 0.21 | **curious** |

The magnitude progression is **identical across all three legs** (3.330 → 6.549 → 9.661 …) because
the spike shape and the EWMA math are the same; the only thing that differed was the baseline sample
count, and the band followed it. `confidence == persistence_seconds / 100` exactly, on every row —
so the baseline phase is literally the tier selector, which is what the three files were re-shaped to
be. Recorded, never hard-failed on the exact value (calibration-region policy).

Attribution note: `triage.cue.emit` carries NO service identity (Pulse's aggregate-only convention
bans `scope_id`/`service_name` from it). Cues are attributed by `persistence_seconds`, which is the
emitting service's sample count. The preflight canary's is pinned at **15** (3 + 12) and, because the
EWMA is sample-driven and the canary stops emitting, its cue FREEZES at exactly
`magnitude 8.511516379456504 / confidence 0.15 / persistence 15` and repeats every second forever —
byte-identical on legs A and B, which is what makes it a reliable discriminator.

## The `AutoResolved` arm fired live for the first time (leg E)

The arm shipped unit-pinned but UNEXERCISED live on 2026-08-20, because the canary's own cues kept
its incident alive. Leg E exercised it. `ack-cooldown` emits no incident-forming stimulus at all
(five default plain spans), so the only incident is the preflight canary's:

```
09:16:01.859  list_active item_count=0
09:16:46.858  list_active item_count=1   <- the canary's incident
09:18:55.300  list_active item_count=0   <- auto-resolved 128.4s later
   … 4 minutes of empty active list …
09:22:57      read-back: query_incident_list -> result_count 0
```

Conductor's own line: `declare-only read-back empty: no active incident outlived the emission
window` (`target: conductor_run`, run `2026-08-21T09-16-01-488`), and the envelope carries
`fingerprints: []` — the arm's signature. The 2026-08-20 leg's honest limit is now closed by
measurement rather than by argument.

Why leg A did NOT take this arm: its retrigger deliberately re-opens an incident 3.5 minutes after
the resolution, so read-back lands on a populated list. Both paths are therefore exercised across the
family — the graded path on A–D, the residual arm on E.

## `CountAtLeast` retirement ground, measured live

`retrieve_telemetry_slice` returned `result_count: 0` on every read-back call: `span_refs` is empty
because Pulse's incident producer writes `evidence_refs.span_ids = Vec::new()`. The evaluator grades
`CountAtLeast` against exactly that sum, so the check observed "0" regardless of what the lifecycle
did — a mis-aimed instrument, retired with its measurement rather than left to fail.
(`fingerprint_refs` is separately non-empty: the constant `det-*` triple, payload-invariant.)

## Hygiene

Committed evidence is 502 KB across five legs, with **zero** matches for host-path patterns
(drive-letter, `/c/Users`, `.cargo`, `.rustup`). Envelopes carry the eleven-field shape;
`journal_emitted_at` / `read_back_observed_at` are RFC-3339 and `latency_ms` is integer milliseconds.

## Leg recipe used (all five)

Fresh `%TEMP%/pulse-legs/sev-{a..e}` per leg · `pulse-app` launched from the Pulse repo (outside this
repo) with `ANDROMEDA_PULSE_L4_DETERMINISTIC=true ANDROMEDA_PULSE_MCP_ENABLED=true
ANDROMEDA_PULSE_DATA_DIR=<leg dir>` · sidecar rebuilt at HEAD and put on `PATH` in msys form · leg
waited on `:4317` accepting (a real readiness signal, not a fixed sleep) · Pulse's
`{data_dir}/logs/agent-latest.jsonl.<date>` sliced by a pre-leg line count.

**Correction to a prior harvest's assumption:** that pre-leg count is NOT zero on a fresh data dir.
`pulse-app` logs its own boot before the receiver opens — 2234 lines on leg A, 2264–2275 on the
others. A harvest that assumed 0 would have mixed boot output into the leg window.

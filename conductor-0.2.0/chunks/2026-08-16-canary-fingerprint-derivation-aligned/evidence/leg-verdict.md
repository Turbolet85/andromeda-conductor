# Live leg verdict — 2026-08-16-canary-fingerprint-derivation-aligned

**Outcome: `ready:true`.** The first green preflight in the project's history, and the first run record
carrying a measured verdict rather than a `Blocked` row.

## Environment

| | |
|---|---|
| Pulse | `pulse-app` + `andromeda-pulse-mcp`, both built at HEAD `d090314` |
| Data dir | `D:/dev/tmp/pulse-leg-0946` — **fresh**, created for this leg |
| Launch cwd | `D:/dev/tmp` (outside both repos — the Specta/TauRPC binding-emission precaution) |
| Mode | `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` — confirmed active in Pulse's own log: `L4 deterministic mode active (ANDROMEDA_PULSE_L4_DETERMINISTIC); canned output, no model` |
| Workspace key | published to `{data_dir}/run/workspace-key` = `\\?\D:\dev\tmp` |
| Conductor env | `ANDROMEDA_PULSE_MCP_ENABLED=true` · `ANDROMEDA_PULSE_DATA_DIR` shared · `RUST_LOG=info,conductor_emit=debug` · sidecar dir on `PATH` |

## Arm 1 — `agent-run.sh boot` (run_id `2026-08-16T09-51-41-829`)

```json
{"ready":true,"negotiated_protocol_version":"2024-11-05","expected_protocol_version":"2024-11-05",
 "required_tools":{"mark_incident_resolved":"present","query_incident_list":"present",
 "retrieve_report":"present","retrieve_telemetry_slice":"present"},
 "data_dir":"<redacted>","canary_round_trip":"ok","blocked_precondition":null,
 "checked_at":"2026-08-16T09:52:26Z"}
```
Exit **0** (the gate contract: exit 0 iff `ready:true`). `data_dir` redacted — host-path-free.

**What this leg uniquely proves.** Everything upstream of the final precondition was already proven by the
2026-08-15 leg; what had never run against a live Pulse is the **re-aimed final precondition** (an incident
opened after the emission stamp). It fired exactly as designed, and the transition is visible on the wire:

| t | `query_incident_list` | gate |
|---|---|---|
| 09:52:26.939 | `result_count: 0` | `NotYet(EmptyCorpus)` — retry |
| 09:52:27.952 | `result_count: 1` | **`Ok`** — an incident opened past the stamp |

Pulse's own incident-persist line stamps `2026-08-16T09:52:27.074Z`, after the storm emitted at
`09:52:26.9` — the freshness comparison is doing real work, not passing on residue.

## Arm 2 — `SCENARIO=span-status-error-detection agent-run.sh run` (run_id `2026-08-16T09-54-12-950`)

Run through a green preflight to a measured verdict — `[RESIDUAL]`, exit 0. The evidence pointer:

```json
{"journal_emitted_at":"2026-08-16T09:54:59Z","read_back_observed_at":"2026-08-16T09:55:01Z",
 "run_id":"2026-08-16T09-54-12-950","seed":424242,"scenario":"span-status-error-detection",
 "p_ids":["P-005"],"verdict":"Pass","state":"KnownResidual","latency_ms":2153,
 "slo_tier":"<5s","fingerprints":[]}
```

Beside the previous row for contrast — every measurement field null under the Blocked-row rule:

```
2026-08-14T16-33-10-333 | fingerprint-storm | verdict null | state Blocked | latency null
2026-08-16T09-54-12-950 | span-status-error-detection | verdict Pass | state KnownResidual | latency 2153
```

`latency_ms: 2153` is the first real journal-relative SLO measurement the harness has produced, inside its
`<5s` tier. `state: KnownResidual` is the pre-accepted `degraded_mode` read-back, not a fault.

## Pulse-side telemetry (harvested post-leg)

Sink: `{data_dir}/logs/agent-latest.jsonl.2026-08-16`, sliced to the leg window by pre-leg line count
(**10,759 → 32,663**; 21,904 lines). The console tee remains 0 bytes — `pulse-app` is a Windows
GUI-subsystem binary and never attaches to a console.

- **27 `duckdb.append` lines, ZERO `reject_reason`** — the ingest→fingerprint feed is healthy, reproducing
  the 2026-08-15 leg exactly.
- **`severity_hint: "autonomous"` ×3 at `occurrence_count: 10`** (plus `suggested` at 5) — the storm clears
  Pulse's Autonomous band.
- Incident formed and persisted.

## Independent confirmation of the chunk's premise correction

`retrieve_telemetry_slice` returned **`result_count: 0`** on the live wire during arm 2, and the run's
envelope carries `fingerprints: []`. That is the falsified premise measured directly: the field the canary
used to assert against is empty on a healthy path, because it is populated from the L4 model's
`evidence_refs`, which the deterministic-L4 fixture pins to `[]`. No derivation could have made the old
assertion pass.

## Honest limits

- **Freshness proves causation-in-time, not payload identity.** A concurrent unrelated incident inside the
  poll window would satisfy the gate. Under deterministic L4 no stronger claim is available — every
  L4-authored field (`title` / `severity` / `fingerprint` / `evidence_refs`) is a fixture constant.
- **The adopted derivation was not verified against a Pulse-computed fingerprint**, because none reaches any
  read-back surface. Its correctness rests on transcription from the SUT's source plus unit tests, not on a
  live comparison.

# Live leg verdict — 2026-08-15-canary-spans-pulse-fingerprints

**Run:** `run_id 2026-08-16T08-17-48-786` · leg window `2026-08-16T08:17:48Z → 08:20:03Z` (UTC)
**Pulse:** `pulse-app` at parked HEAD `d090314`, rebuilt binary, **fresh data dir**, deterministic L4,
`ANDROMEDA_PULSE_MCP_ENABLED=true`, sidecar on PATH. `RUST_LOG=info` (Pulse) ·
`RUST_LOG=info,conductor_emit=debug` (Conductor).
**Artifacts:** `pulse-leg-window.jsonl` (47,784 Pulse lines for the leg) · `boot-leg.out` (Conductor) ·
`pulse-app-leg.log` (console tee — **empty**: `pulse-app` is a Windows GUI-subsystem binary and never
attaches to the console; its real telemetry sink is `{data_dir}/logs/agent-latest.jsonl.<date>`, which is
what `pulse-leg-window.jsonl` is sliced from).

---

## 1. The fix is PROVEN — quantitatively, on Pulse's own telemetry

**`duckdb.append`: 27 lines, ZERO carrying `reject_reason`.** Every append succeeded.

The arithmetic is exact:

| | appends |
|---|---|
| warm-up spans → `spans` (08:17:48, 08:18:03, 08:18:18 — 15 s apart) | 3 |
| storm spans → `spans` | 12 |
| storm spans → `span_events` | 12 |
| **total** | **27** |

`ok_span` carries no events, so the warm-up contributes to `spans` only — the count matches that exactly.

### The `buffer.tick` trio, before and after

| counter | prior chunk (2026-08-15) | **this leg** |
|---|---|---|
| `rows_ingested` | 1 | **15** |
| `span_events_seen` | 0 | **12** |
| `observer_invocations` | 0 | **12** |
| `fingerprints_computed` | 0 | **12** |
| `storms_detected_total` | 0 | **2** |
| `fingerprints_evicted_total` | 0 | **1** |

`observer_invocations: 12` means the fingerprint observer was **invoked once per computed fingerprint** —
the stage that was previously never reached at all.

**The ingest→fingerprint gap is CLOSED** — that much is measured, on Pulse's own telemetry, in this leg.

**On the "second independent unknown", keep the register honest.** What is MEASURED here: with the
constant-identity collision removed, all 15 spans append and all 12 events fingerprint. What is **INFERRED,
not measured**: *why* the prior leg's 12 distinct-id storm spans appended zero rows. The natural reading —
the warm-up's primary-key violation poisoned the shared DuckDB appender/connection for the batches that
followed — is **consistent with both measurements but was never observed**, because the prior leg captured
no `duckdb.append` lines at all (that chunk's report contains zero; capturing them is exactly what this
chunk's plan made mandatory). Re-checking it after the fact is impossible: **that leg's data dir no longer
exists** (searched 2026-08-16 — no `agent-latest.jsonl.2026-08-15` in the default platform dir, the
scratchpad trees, or the Pulse repo). So the two causes behaved as one *in this leg's outcome*; whether they
were mechanically one cause is an inference the evidence supports and does not prove.

`tracked_fingerprints_count: 0` at 08:19:50+ is the **expected healthy reading**, not a defect: it is a
60 s-windowed gauge over DISTINCT fingerprints sampled after eviction, and the storm landed at 08:18:33 —
77 s earlier, outside the window. The window-immune cumulative counters (`storms_detected_total: 2`,
`fingerprints_evicted_total: 1`) are the ones that carry the signal, exactly as the standing rule says.

## 2. The storm reached Pulse's Autonomous band and an incident FORMED

```
triage.pattern.storm.detected  cue_kind=retry_storm  fingerprint_hex=6074a716
    occurrence_count=5   severity_hint="suggested"    window_seconds=30   08:18:33.916Z
    occurrence_count=10  severity_hint="autonomous"   window_seconds=30   08:18:33.945Z
```

Both tiers fired inside one 30 s detection sub-window — the ladder walking as designed, and the first time
the canary has ever cleared the Autonomous threshold in a live leg. An incident followed:
`incidents.get_report returned  item_id=1  degraded_mode=true  section_count=6  markdown_size_bytes=1070`.

### Conductor's wire-shape witness agrees

```
wire shape: spans=1 spans_missing_ids=0 event_names=[]            ← ×3 warm-up
wire shape: spans=1 spans_missing_ids=0 event_names=[exception]
            event_attr_keys=[exception.message,exception.stacktrace,exception.type]   ← ×12 storm
```

`spans_missing_ids=0` on every batch — independently re-confirming that LEAD A (empty ids) was correctly
falsified at P3.

## 3. Preflight still `ready:false` — and the NEXT blocker is now precisely named

```json
{"ready":false,"negotiated_protocol_version":"2024-11-05","expected_protocol_version":"2024-11-05",
 "required_tools":{"mark_incident_resolved":"present","query_incident_list":"present",
                   "retrieve_report":"present","retrieve_telemetry_slice":"present"},
 "data_dir":"<redacted>","canary_round_trip":"failed",
 "blocked_precondition":"canary fingerprint not found in telemetry slice",
 "checked_at":"2026-08-16T08:18:33Z"}
```

Every earlier precondition is cleared — protocol negotiated, all four tools present, workspace key aligned,
an incident found, `retrieve_telemetry_slice` returning
`[fingerprint_refs, incident_id, span_refs, timestamps_unix_nano]`. The gate now fails at the **last** step:
the emitted fingerprint is absent from `fingerprint_refs`.

### Why it is absent — a spec↔reality gap, SURFACED not authored

`architecture.md` §Read-Back Dependency Posture states the canary's fingerprint is **"computed to match
Pulse's derivation"**. Measured against both sources, **it cannot be**. Three independent mismatches:

| | Conductor `fingerprint()` (`conductor-emit/src/exception.rs:115-123`) | Pulse `compute_exception_fingerprint()` (`crates/buffer/src/fingerprint.rs:79-96`) |
|---|---|---|
| algorithm | **FNV-1a**, 64-bit | **blake3** |
| input | `exception_type` + `\n` + each frame's **`function`** | `exception_type` + `\0` + `normalize_stacktrace(stacktrace)` |
| normalization | none | first 3 non-empty lines; strips path tokens, hex addresses, `:line(:col)` |
| width | 8 bytes → `{:016x}`, 16 hex chars | **16 bytes** (blake3 truncated); logged as an 8-char prefix of the first 4 bytes |

The width alone makes equality impossible; the algorithm and input differ on top of that. Pulse computed
`6074a716…` for the canary; Conductor's expected value is a different function of different bytes.

**This is a spec↔reality gap for wrap to resolve — nothing was authored here.** It is also NOT a regression
this chunk introduced: the mismatch has always existed and was simply unreachable, because until now no
canary span ever survived to be fingerprinted at all.

## 4. Standing on `v2-10`

`v2-10` **stays pooled this chunk** — the decline is deterministic and final for this chunk, per its own
matrix `notes`. This leg did not reach `ready:true`, so nothing about the decline changes. The leg is
recorded here as the evidence a later chunk's P5 can claim on once the fingerprint derivation agrees.

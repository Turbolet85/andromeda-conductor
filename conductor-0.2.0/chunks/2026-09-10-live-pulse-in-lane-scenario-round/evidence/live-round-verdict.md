# Live-round verdict — 2026-09-10-live-pulse-in-lane-scenario-round

Three in-lane scenarios driven against a live, operator-launched Pulse. Every value below is read from an
artifact this round produced (`runs.db`, the per-run journals, Pulse's own log) — none is inferred.

## SUT posture (measured from Pulse's own log, never from the launch command)

- **pulse-app** PID 33508, launched 2026-09-10 16:54:20 local by `/d/dev/tools/pulse-a11y.sh`; fresh data dir
  `…/pulse-legs/a11y-20260910-165420`; deterministic L4 on; MCP on.
- Boot witnesses, first occurrence in `{data_dir}/logs/agent-latest.jsonl.2026-09-10`:
  - `app.boot.workspace_key` — "workspace key published for sidecar" — `2026-09-10T14:54:21.247Z`
  - `app.boot.otlp.http.bind` — "OTLP HTTP receiver bound" — `2026-09-10T14:54:21.915Z`
  - `app.boot.otlp.grpc.bind` — "OTLP gRPC receiver bound" — `2026-09-10T14:54:21.915Z`
- Published workspace key: `\\?\D:\dev\projects\additional\andromedaV3` (outside the Conductor repo).
- **Bootstrap window: DEFAULT.** `grep -c 'triage.baseline.bootstrap_window.override'` over that log returns
  **0** — no `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` override was in force, which is correct here because
  none of these three scenarios rides the empty-active-set arm.
- **Zero silence cues were emitted.** A bare grep for `service_went_silent` returns 1177 lines, but reading
  them shows every one is `triage.baseline.service_went_silent.evaluate` — "service-went-silent evaluation
  cycle", the evaluator RUNNING, not a cue emitted. The round ran ~9–18 min after boot, inside the default
  one-hour window. 13 `triage.cue.emit` lines fired in the leg window; those are the legs' own storm/error cues.

## Verdicts — all three NON-BLOCKED

| P-ID | scenario | run_id | verdict | state | printed | latency_ms | slo_tier | run_check rows |
|---|---|---|---|---|---|---|---|---|
| P-067 | `live-only-service-truth` | `2026-09-10T15-03-01-195` | `null` | `ManualCheck` | `[MANUAL] live-only-service-truth` | 18169 | `<20s` | 0 |
| P-072 | `investigate-actions-functional` | `2026-09-10T15-07-24-325` | `null` | `ManualCheck` | `[MANUAL] investigate-actions-functional` | 35120 | `<90s` | 0 |
| P-079 | `constellation-severity-live-wiring` | `2026-09-10T15-11-26-937` | `CalibrationRegion` | `ManualCheck` | `[HOLD] constellation-severity-live-wiring` | 30212 | `<20s` | 1 |

Seeds are the TOML-declared values (4317067 / 4317072 / 4317079) — `SEED` was unset, so the TOML governs.
Each row's `journal_emitted_at` / `read_back_observed_at`: 15:03:47→15:04:05Z · 15:08:10→15:08:45Z ·
15:12:13→15:12:43Z.

**Evidence probes:** `non-blocked rows: 3` (runs.db) · `journal envelopes non-blocked: 3` (journals).

**Per-check record contract holds exactly:** 0 / 0 / 1 `run_check` rows — zero for the two declare-only
scenarios, one for the only checks-bearing scenario.

## Sequencing — the quiet windows worked

Each leg carried a 150 s window (Pulse's 120 s idle + a full 30 s resolver tick) before its invocation, because
Pulse dedupes a new incident against ANY open incident and every `conductor run` fires its own preflight canary.
Measured: leg 1's canary incident formed 15:03:47Z; leg 2's canary fired ~15:08:10Z (≈ 263 s later) and formed a
FRESH incident rather than deduping; likewise leg 3. Had the legs run back-to-back, legs 2 and 3 would each have
landed `Blocked: no incident opened after the canary storm was emitted`. No leg blocked.

`query_incident_list` was observed returning `result_count: 0` then `1` inside leg 1's preflight poll — the
canary forming — and `1` again at read-back, so the canary incident was still open when each leg graded.

## P-079 — two independent structural defects, both recorded, neither this chunk's to fix

Both route to `CalibrationRegion`, so the row is non-blocked either way and `v2-04`'s acceptance is met.

**(1) `CountAtLeast >= 1` is unsatisfiable.** `retrieve_telemetry_slice` returned `result_count: 0` on every
leg. Pulse writes `EvidenceRefs.span_ids: Vec::new()` at its only production incident-construction site
(`pulse-app/src/inference_runtime.rs:871`, HEAD `83d4060`) and nothing mutates it after, so `span_refs` is
always `[]`, `evidence_count` is always 0, and `0 >= 1` is false. An unmet sample floor routes to
`CalibrationRegion` regardless of the declared `Hard` (`conductor-verify/src/slo.rs:97-100`).

*The capability itself was NOT the problem.* P-079's `fingerprints` array carries **two** incidents' evidence
(the `det-*` triple twice plus two distinct computed hashes), so the storm's incident DID form and WAS visible
to read-back beside the canary's — which is exactly what P-079 claims. The declared check simply reads a field
that cannot witness it.

**(2) `slo_tier = "<20s"` is unattainable — NEW, not predicted at P3/P4.** The CheckRecord reads
`latency_ms 30212` against `deadline_ms 20000`. Latency is `read_back_observed_at − journal_emitted_at`, which
spans the whole emission window; this scenario's own phases sum to 30 s (`gap_ms` 3000+15000+12000), so the
measured latency can never fall under its declared 20 s tier. The tier is unreachable by construction, for
reasons entirely inside the committed scenario. (P-067: 18 s of phases under `<20s` — 18169 ms, tight but
attainable. P-072: 35 s under `<90s` — comfortable.)

**Class, not a P-079 defect (for (1)).** Exactly three committed scenarios carry a live `kind = "CountAtLeast"`
key — `constellation-severity-live-wiring.toml:53` · `findings-counter-refresh.toml:49` ·
`pulse-run-contract.toml:58` (a bare token grep returns seven files; four mention it only in retirement
comments). The mechanism is already recorded at `.andromeda/architecture-amendments.md:403`, and
`.andromeda/architecture.md:69` records the same outcome as measured and accepted for the still-shipping
sibling `findings-counter-refresh`. **Owner: a route-entry candidate raised at this chunk's wrap** — now
covering both defects.

## Second independent round — the wrap light gate (2026-09-10T16:10–16:21Z)

The wrap re-ran all three legs verbatim, quiet windows included, against the SAME pulse-app instance
(PID 33508, still up, same data dir). **The round is RUN-STABLE**: identical verdicts, and latencies within
34 ms of the first round.

| P-ID | run_id | verdict | state | latency_ms (round 1 → 2) |
|---|---|---|---|---|
| P-067 | `2026-09-10T16-12-33-438` | `null` | `ManualCheck` | 18169 → **18145** |
| P-072 | `2026-09-10T16-16-16-671` | `null` | `ManualCheck` | 35120 → **35111** |
| P-079 | `2026-09-10T16-20-15-737` | `CalibrationRegion` | `ManualCheck` | 30212 → **30246** |

This matters beyond confirming the gate: test-plan §2 carries live legs as an enumerated exception to the
determinism invariant, and the 2026-09-06 rule warns that a leg over an unchanged tree can grade two ways
when its precondition rides the SUT's own clock. **These three do not** — none has an empty-active-set arm,
each leg's own canary is still open at its read-back, and the 150 s quiet window is what makes the sequence
reproducible rather than order-dependent. Two rounds, one tree, one SUT, same verdicts.

Round-2 envelope lines are committed beside the round-1 ones as `*.run2-lightgate.jsonl`, per the
`2026-09-07-sr-findings-fixed` precedent (which committed its own `run3-lightgate` capture rather than
overwriting).

## Files beside this record

`p-067.jsonl` · `p-072.jsonl` · `p-079.jsonl` — each leg's run-report envelope line verbatim, selected by the
envelope-only `seed` key. P-079's journal carries 2 lines (envelope + CheckRecord), so the "newest ENVELOPE
line, never the file's last line" rule was load-bearing here in practice, not only in theory.

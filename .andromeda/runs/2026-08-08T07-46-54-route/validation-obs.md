# Obs validation — route draft

_Bootstrap items (§3 logging stack, service identity, `run_id` correlation, redaction wire, §9 log-upload + conformance gate) shipped in 0.1.0 and correctly carry no 0.2.0 chunk; §3 heartbeat is N/A/optional at Minimal tier._

---

## Insert
- Between `First live green preflight` and `fingerprint-storm live proof`: **"Fault-application spans — silence, ramp and port-occupier phases observable beneath the timeline span, with duration and journal offset"** (epoch: `Epoch 3`)
  Reason: obs-plan §4 "Fault-injection spans (chaos-instrumentation trigger)" mandates `fault.silence`/`fault.ramp`/`fault.port_occupier` under `timeline.execute`, yet `conductor-faults` carries no instrumentation and no chunk creates it.
- Between `Coverage completeness gate` and `Dependency polish`: **"Run-report envelope conformance gate — every run journal row schema-complete and host-path-free, build failing on violation"** (epoch: `Epoch 6`)
  Reason: obs-plan §9 (per amendment `2026-06-27-obs-ci-conformance-gate`) records the envelope record shape as having its own "not-yet-built" gate — the shipped gate asserts only the §3 self-obs base line, and 0.2.0 is where the envelope first carries live read-back values.

## Reorder
- Move `scenario.run root span tree` before `First live green preflight` (into `Epoch 2`)
  Reason: obs-plan §4 roots all seven must-trace paths at `scenario.run`, so leaving it in Epoch 5 runs five family proofs plus the lifecycle pass with only leaf spans (`timeline.execute`/`emit.batch`/`verify.readback`) and no root correlation — §11 "NEVER skip spans on critical paths (Scenarios 1-7) — agent loses debug ability."

## Rewrite
- `A11y CI gate`: "violation JSON into the obs envelope, service-tagged" → "violation JSON as service-tagged self-obs log lines"
  Reason: obs-plan §9 + amendment `2026-06-27` hold the two record shapes distinct — "obs envelope" names the §6 run-report scenario-result record, which a11y violations do not populate.
- `Per-check latency measurement`: "sub-5s budget representation beneath the closed tier set, measured per check" → "sub-5s budgets beneath the closed `slo_tier` set, per-check `latency_ms` carried in the run-report envelope"
  Reason: obs-plan §5/§10 fix `latency_ms` (wall-clock `read_back_observed_at − journal_emitted_at`) and the closed `slo_tier` enum as the envelope carriers, so a finer per-check budget must extend that record rather than fork a parallel one.

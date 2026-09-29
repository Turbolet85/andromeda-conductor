
## 2026-09-29-diagnostic-quality-cluster-off-the-drift-pin — the real-model posture chain observed live
**Section:** §4 Span / Trace Coverage → the headless deterministic scenario run → Real-model posture
**Change:** was "The one drive (2026-09-23) was blocked at the preflight, so this chain was not observed live under the real-model posture". Now the 2026-09-29 series observed it live: both Stage B preflights reached ready, the graded drive's envelope landed `ManualCheck` with `verdict` null and the eleven keys, and canary-blocked drives' envelopes landed `Blocked`. The three-storm canary adds no span or field; each storm logs the existing `canary fingerprint computed` line.
**Why:** the series superseded the claim that the chain was never observed. Instrumentation is unchanged: the diff adds no span name and no allowlisted field.
**Ref:** .andromeda/runs/2026-09-29T17-50-46-wrap/

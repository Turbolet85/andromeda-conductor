# Obs Plan — Amendments

_Append-only changelog of amendments to `obs-plan.md` (the body holds only current truth; history lives here + in git). Written by /andromeda-wrap-session P2._

## 2026-06-15-structured-logging-stack — two record shapes clarified (self-obs base line vs Run-report envelope)
**Section:** §3 Observability Harness Contract / Log format JSON schema
**Change:** clarified that the §3 schema block is the Run-report envelope (scenario-result record — emission journal + `runs.db`, report seam Epoch 6); the foundational self-obs log line carries a base set — `timestamp_ms` (epoch millis, `std::time`), `level`, `target`, service-identity, `run_id` — on every line, with envelope/result fields only on scenario-result events. Noted the implementation uses a custom `tracing-subscriber` layer (stock `fmt().json()` can't emit constant identity fields flat).
**Why:** the structured-logging-stack chunk implemented the self-obs stream; its line schema differs from the envelope the §3 block showed. Justified divergence (foundational lines can't carry verdict/latency yet) → §3 reconciled. D-obs-stack cleared (tracing JSON, no OTel SDK). D-obs-redaction (escalate) resolved WITH the user as correct sequencing (redaction = `pii-scrubbing-wire`, the next chunk; synthetic-only, no leak) — NO obs body edit; a `playbook.md` rule was added so Foundation deferrals of separately-sequenced concerns no longer escalate. Cascaded to `.claude/rules/observability.md` + `.claude/docs/obs-summary.md`.

# Obs validation — route draft

## Insert
- Between `Structured logging stack` and `Design-token + typography bundle`: **"Panic-capture hook — std::panic::set_hook + structured tracing::error(panic) JSON, anyhow edge bridging"** (epoch: `Foundation`)
  Reason: Per obs-plan §10 SLO Invariants (zero unlogged panics), panic-hook wiring is a startup obligation no current chunk covers.
- Between `Desktop a11y assertion harness` and `Live-Pulse E2E proof`: **"Obs CI artifact gate — agent-latest.jsonl upload, log-schema conformance, zero-unlogged-panics check"** (epoch: `Polish & ship`)
  Reason: Per obs-plan §9 CI Integration, the obs-specific CI stage (log upload + conformance + panic-hook verification) is not covered by the generic Base CI chunk.

## Reorder
- Move `Artifact redaction layer` (Run report & persistence) to Foundation, after `Structured logging stack`
  Reason: Per obs-plan §3 + §6, redaction must wire at the tracing-subscriber processor stage so upstream failures never leak via stderr — at the source, before any emitting chunk.

## Rewrite
- `Structured logging stack`: "tracing + tracing-subscriber JSON sink, service.name/version/env identity" → "tracing + tracing-subscriber JSON sink, service identity as per-line JSON fields (no OTel SDK)"
  Reason: Per obs-plan §3 Service Identity + §6, identity is emitted as flat per-line JSON fields, not OTel resource attributes (self-observation runs no OTel SDK).

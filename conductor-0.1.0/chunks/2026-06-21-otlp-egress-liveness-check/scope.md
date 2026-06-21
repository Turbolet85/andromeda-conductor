# Scope — OTLP egress liveness check

**Marker:** 2026-06-21-otlp-egress-liveness-check
**Version:** conductor-0.1.0 · Epoch 5 (Verification & read-back) · chunk 3 of 6
**Working-route intent:** OTLP egress liveness check — loopback :4317 connectable, refused ⇒ harness Err

## What it builds

A liveness / connectability probe over the OTLP-over-gRPC **egress** channel to `127.0.0.1:4317`
(Pulse's loopback ingest). Before the timeline engine begins emitting, this check confirms the gRPC
channel is connectable. It is the OTLP-egress analogue of an HTTP `/health` — the **"Liveness
equivalent (OTLP egress check)"** named in architecture.md §Standard Contracts — paired with, but
categorically distinct from, the MCP `initialize` preflight readiness gate (Epoch-5 chunk 2).

The probe attempts to establish the gRPC transport connection to `:4317` and resolves to:
- **connectable** → `Ok(())` (the egress path is live; emission may proceed);
- **refused / unreachable** → `Result::Err` — a **Conductor harness fault**, NOT a verification verdict.

## Boundaries

- **Verdict/error wall (the load-bearing invariant):** a refused/unreachable transport is `Result::Err`
  (harness fault), never a `Verdict`/`ReportState`. It is **not** `Blocked` (that is the MCP-preflight
  state), not `Fail`, not `Pass`. This check yields no verdict at all — it gates whether emission is even
  attempted; a refused transport halts the run as a harness error before any scenario is measured.
- **Connect probe only:** establishes / checks the channel. It does NOT export an OTLP message (no
  spans/logs/metrics sent over the wire), does NOT mutate Pulse state, and does NOT run a scenario.
- **Owning seam — conductor-emit:** it owns the tonic 0.14.6 `Channel`/`Endpoint` to `127.0.0.1:4317`
  (established by the raw-otlp-message-scaffold chunk). The star topology (every seam depends on
  `conductor-core` only) forbids `conductor-verify`/`conductor-timeline` from reaching emit's channel,
  so the probe primitive lives in conductor-emit and surfaces refusal as a typed `EmitError`.
- **Out of scope — deferred:** suite-start *orchestration* ("run the liveness probe before the timeline
  emits; abort the run on `Err`") is conductor-cli's bootstrap wiring in Epoch 8. This chunk delivers the
  reusable primitive + its typed-error contract, not the CLI call site. Also out of scope: the P-003
  port-occupier fault (conductor-faults) — a *listening* occupier on :4317 ACCEPTS the connect, so it is a
  different signal from "refused"; the two must not be conflated.
- **No new bind / scope law:** the probe is a client connect to the loopback target only; Conductor opens
  no inbound listener of its own (the `:4317` port-occupier remains the sole deliberate bind, elsewhere).

## Surfaces / contracts it touches

- **architecture.md §Standard Contracts — "Liveness equivalent (OTLP egress check)":** "Before emission,
  the timeline engine confirms the gRPC channel to 127.0.0.1:4317 is connectable; a refused transport
  surfaces as a harness error (`Result::Err`), not a verification verdict." — this chunk realizes that
  contract.
- **conductor-emit** tonic 0.14.6 / tonic-prost gRPC egress channel to `127.0.0.1:4317` (the existing raw
  OTLP scaffold + its `EmitError` typed enum).
- **Verdict/error wall** — `Result::Err` = harness fault only; sanitized at the conductor-cli / `anyhow`
  edge (no host-path / struct-name / stack-trace leak).
- **Wall-clock from std::time** — any connect-timeout bound is real time (`std::time`), never tokio's
  virtual clock; the probe is a connectivity gate, not seeded timeline scheduling.

## Acceptance intent (validated at P5)

1. A probe against a connectable `:4317` returns `Ok`; against a refused / down `:4317` it returns
   `Result::Err` (a typed `EmitError`), **never a verdict value** and never a panic.
2. The refusal error is typed (thiserror) and sanitizable at the binary edge — no absolute host path,
   internal struct name, or stack trace leaks into operator output or run artifacts.
3. The primitive lives in conductor-emit; **no seam→seam dependency edge** is introduced (star topology
   preserved — verified against `Cargo.toml` edges + the code-graph).

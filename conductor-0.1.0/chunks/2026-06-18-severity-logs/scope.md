# Scope — Severity logs

**Marker:** `2026-06-18-severity-logs`
**Version:** conductor-0.1.0 · Epoch 3 (Emission primitives) chunk 4/8
**Primary crate:** `conductor-emit` (builds on the prior chunks' `client.rs` / `message.rs` egress + helpers)
**Pulse P-IDs:** P-007 (log records with controlled `SeverityNumber` across the 17-boundary)

## What it builds
The **log-record** emission primitive on the raw-OTLP scaffold — the logs analogue of the existing trace
path. Conductor must drive Pulse's severity-boundary hard-signal detection (input.md §Capabilities line 59 +
§Coverage line 103, hard-signals P-005..P-008: "SeverityNumber 17-boundary"), which needs two new surfaces:

1. **A raw `ExportLogsServiceRequest` builder** — hand-built `ResourceLogs` / `ScopeLogs` / `LogRecord`
   (opentelemetry-proto `logs` raw types, mirroring the `trace_request` / `error_trace_request` builders),
   each `LogRecord` carrying a **controlled `SeverityNumber`** with its **matching `SeverityText`** plus a
   `body`, `time_unix_nano` / `observed_time_unix_nano`, and the shared `service.name` resource.
2. **A `LogsEmitter`** over `LogsServiceClient<Channel>` — the logs sibling of `TraceEmitter`: same loopback
   gRPC egress to `127.0.0.1:4317`, same `connect` / `from_channel` shape, same verdict/error wall (a refused
   or malformed transport ⇒ typed `EmitError` `Result::Err`; a collector-returned `tonic::Status` is a typed
   input, never a panic).

**The 17-boundary** is the OTel WARN→ERROR cut: `SeverityNumber` 13–16 = WARN, 17–20 = ERROR; Pulse treats
`SeverityNumber ≥ 17` as an error-grade hard signal. The primitive must let a scenario place records on
**either side** of the boundary — a just-below `16/WARN` record and a just-at `17/ERROR` record — so a later
scenario can prove Pulse fires on `≥17` and not on `≤16`. Severity is **spec-controlled** (the caller's input),
NOT seed-derived; the seed governs only record identity/timing bytes (mirroring `error_trace_request`), and
wall-clock stamps come from `std::time` (never tokio's virtual clock).

## Boundaries / out of scope
- NOT the severity **lifecycle** (tiered inputs · auto-resolve · ack-retrigger · per-tier SLO, P-019..P-023) —
  that is Epoch 7 "Severity-lifecycle scenarios". Here: only the per-record `SeverityNumber` primitive (P-007).
- NOT log **body** PII content — the seven-category PII corpus is the later Epoch-3 "PII payload corpus" chunk
  (P-047/P-035/P-048). Bodies here are minimal/benign placeholders.
- NOT latency shaping, multi-service topology, or traffic-rate ramps (sibling later-Epoch-3 chunks).
- NOT metrics emission (no `ExportMetricsServiceRequest`).
- NOT MCP read-back / verification — **emit-side only**. This chunk produces (and, if wired, journals) the
  emitted severity; it does not assert Pulse's reaction.
- NOT yet timeline/journal-wired beyond whatever the sibling emit primitives already are — mirror the altitude
  of `raw-otlp-message-scaffold` / `error-spans` / `exception-events` (standalone emit primitives + loopback
  stub tests; timeline integration is a later concern). Confirm in research (see boundary below).
- Stays inside loopback gRPC egress to `:4317`; opens NO listener (`:4318` unused); determinism-under-seed
  preserved.

## Surfaces / contracts touched
- `conductor-emit` public API: a new logs-request builder fn + a `LogsEmitter` (re-exported from `lib.rs`
  alongside `TraceEmitter` / `trace_request`). Reuse `message.rs` helpers (`service_resource`, `string_kv`,
  `unix_nanos`) where the logs message shares them; the existing trace egress path is unchanged.
- OTel logs semantic convention: the `SeverityNumber` enum + its canonical `SeverityText`, `LogRecord.body`
  (`AnyValue`), and observed/emitted timestamps.
- `opentelemetry-proto` **`logs` feature** + the generated `LogsServiceClient` (the trace path uses only
  `trace`) — a Cargo manifest feature change, expected with **no new crate** (`Cargo.lock` un-drifted).

## Boundaries to resolve in planning
1. **`logs` feature + the deferred `default-features = false` trim.** This chunk must enable
   opentelemetry-proto's `logs` feature. The handoff carries a tracked follow-up — `default-features = false`
   to drop the dormant transitive `opentelemetry_sdk` (obs-plan rule confirms it is a tracked, non-violating
   trim). Folding that trim in here is natural (the feature list is being rewritten anyway) but adds
   `Cargo.lock`-drift / build-surface risk. **Decide in P4 (AskUserQuestion):** fold the trim into this chunk
   vs keep it deferred. Default lean: minimal — enable `logs`, leave the trim deferred unless cheap+clean.
2. **Journal integration altitude.** Do the emitted log records feed the emission journal (`RunRecord`, the
   "ground truth of what was sent" + the left side of every SLO check), or is this a standalone emit primitive
   like its three sibling chunks (not yet timeline/journal-wired)? Resolve from the sibling chunks' actual
   altitude in research; default lean: match the siblings (standalone primitive), defer journal wiring.

## Acceptance intent (val-1 anchor)
- A logs builder produces a raw `ExportLogsServiceRequest` whose `LogRecord`s carry a **controlled
  `SeverityNumber` with its matching `SeverityText`**, and can place records on **either side of the
  17-boundary** (a `16/WARN` and a `17/ERROR` record are constructed and distinguishable).
- A `LogsEmitter` ships them over loopback gRPC to `:4317`, mirroring `TraceEmitter`: a refused/malformed
  transport surfaces as a typed `EmitError` (`Result::Err`); a collector `tonic::Status` is a typed input,
  never a panic (verdict/error wall intact).
- **Determinism:** same seed + same log spec ⇒ identical emitted log shape on a shape-projection that excludes
  wall-clock stamps (severity + body + linkage); severity is spec-controlled, not seed-derived. Both-directions
  where a seed materially participates (per testing.md): same seed ⇒ identical projection, divergent seeds ⇒
  divergent identity bytes.
- Tested via a **loopback gRPC logs-service stub** (the `LogsService` analogue of the existing trace stub on an
  ephemeral `127.0.0.1:0`); the real `:4317` is NEVER bound (reserved for the Epoch-4 port-occupier).
- Gates green: `cargo nextest run --workspace --profile ci` · `cargo clippy --workspace --all-targets -D
  warnings` · `cargo audit` + `cargo deny` · `Cargo.lock` un-drifted.

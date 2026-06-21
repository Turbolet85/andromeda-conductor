# arch extract

## Relevance
relevant — the chunk realizes a locked contract (§Standard Contracts "Liveness equivalent") in the conductor-emit seam with a connectivity probe to the OTLP egress channel.

## Constraints
- (arch §Design Philosophy) Determinism and errors: outcomes are values; `Result::Err` is reserved for Conductor's own harness faults, not verification verdicts, so a refused `:4317` transport must surface as `Err`, never a `Verdict` or `ReportState`.
- (arch §Stack and Technologies) tonic 0.14.6 + tonic-prost 0.14.6 over `opentelemetry-proto 0.32.0` raw types; no web framework / HTTP service — gRPC client only.
- (arch §Established Decisions [OTLP Emission Strategy]) Raw OTLP message structs from `opentelemetry-proto` for byte-level control; this chunk does not emit a message, only establishes the transport.
- (arch §Occupied Resources — Ports) `127.0.0.1:4317` is Pulse's loopback OTLP ingest (owned by Pulse); Conductor is the gRPC client, co-located on the dev host.
- (arch §Established Decisions [Async Runtime Flavor]) tokio `current_thread`, core-owned; a connect-timeout is **not** seeded timeline scheduling — it must measure real time via `std::time`, never tokio's virtual clock.
- (arch §Infrastructure Patterns) Cargo workspace, crate-per-seam: conductor-emit owns the tonic `Channel` / `Endpoint` to `:4317`; typed error enums surface from the owning seam (typed `EmitError`).
- (arch §Cross-cutting Patterns — Verdict/error wall) The verdict/error wall applies universally: refusal is `Err`, not a state or verdict.

## Patterns to follow
- Establish the gRPC `Endpoint`/`Channel` to `127.0.0.1:4317` using tonic 0.14.6's `Endpoint::from_static()` + `.connect()` or equivalent, reusing the scaffold from raw-otlp-message-scaffold (Epoch 4).
- Return a typed `EmitError` variant on connection refusal (thiserror-derived enum in conductor-emit); use `Result<(), EmitError>` as the probe signature.
- Wall-clock timeout bounds must come from `std::time::Duration` / `std::time::Instant` (never tokio's virtual clock or seeded jitter).

## Anti-patterns to avoid
- Do **not** emit any OTLP message (no spans/logs/metrics over the wire); this is a connectivity check only.
- Do **not** classify connection refusal as a verdict value (`Blocked`, `Pass`, `Fail`, `ManualCheck`, `KnownResidual`) or a `Verdict` enum value; it must remain `Result::Err`.
- Do **not** allow a bare panic or untyped error to propagate; all error paths must be typed and sanitizable at the binary edge.

## Contract bindings
- **obs ↔ emit:** self-observation (obs-plan §3) logs the probe result + any connection latency as structured JSON; emit's typed `EmitError` is loggable via tracing.
- **tests harness ↔ emit:** E2E fixture tests must cover connectable and refused `:4317` scenarios (e.g., port-occupier fault running in parallel, or a mock refused endpoint); test golden-output validation confirms no verdicts leak into error paths.

## Acceptance criteria contributions
- (arch) The probe is placed in `conductor-emit` per workspace crate-seam rules (architecture.md §Inherited Defaults: `conductor-emit` owns OTLP client).
- (arch) Connection refusal surfaces as `Result::Err` with a typed `EmitError` variant (never a `Verdict` or `ReportState`), satisfying the verdict/error wall (architecture.md §Design Philosophy).
- (arch) No new env var, port, or IPC route is introduced (`:4317` is already occupied; only a client connect, no inbound bind).
- (arch) Wall-clock timeout uses `std::time` (never tokio's virtual clock), preserving journal-relative SLO math in non-probe paths (architecture.md §Cross-cutting Patterns — Determinism discipline).

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold** — MSRV raised 1.88.0 → 1.94.1 across §Stack, §Infrastructure, §Inherited Defaults; affects this chunk's build compliance.
- **2026-06-18-exception-events-fingerprint-control** — fingerprint primitive (`fingerprint()` + exception-event builder) placed in `conductor-emit`; this chunk reuses the same crate's ownership + export infrastructure.

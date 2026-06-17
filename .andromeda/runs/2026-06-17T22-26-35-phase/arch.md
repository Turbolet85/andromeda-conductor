# arch extract

## Relevance
Partial — foundational emission scaffold underpins all P-005+ scenarios; not specific to individual verifications.

## Constraints
- Per §Stack and Technologies: Wire `conductor-emit` to **opentelemetry-proto 0.32.0** (`gen-tonic` + `trace`, plus `metrics`/`logs` for later chunks), **tonic 0.14.6**, **tonic-prost 0.14.6**, **prost 0.14**, pinned in `[workspace.dependencies]`; all versions audit/deny-clean + `Cargo.lock` committed.
- Per §Established Decisions [OTLP Emission Strategy]: Use **raw opentelemetry-proto message structs** (NOT the opentelemetry-otlp SDK exporter) for byte-level fault-injection control; hand-built `ExportTraceServiceRequest` / `ResourceSpans` / `Span`.
- Per §Established Decisions [Async Runtime Flavor]: Emit on the **core-owned tokio `current_thread` runtime** (determinism-under-seed constraint); no work-stealing.
- Per §Stack and Technologies (Determinism RNG): Use **rand_chacha 0.9 `ChaCha8Rng` + seed_from_u64** for platform-stable per-gap jitter; this is the timeline scheduler's sole non-determinism source.
- Per §Conventions (Interface surfaces): **tonic 0.14.6 / tonic-prost / prost-built messages over gRPC to `127.0.0.1:4317`**; no `:4318` HTTP variant; OpenTelemetry Semantic Conventions as shared vocabulary (`service.name`, span `Status.Code`, exception events, log `SeverityNumber`).
- Per §Conventions (Error handling): **Typed `EmitError` (thiserror) enum for seam**; transport refusal is harness `Result::Err` (verdict/error wall), not a verification verdict; `tonic::Status` codes are first-class typed inputs, never panics.
- Per §Cross-cutting Patterns (Config management): No secrets/cloud config; OTLP egress target `:4317` is owned by Pulse (read-only from Conductor's perspective).

## Patterns to follow
- **Raw OTLP message construction:** Hand-assemble `ExportTraceServiceRequest` → `ResourceSpans` (with `Resource` carrying `service.name` via §Occupied Resources `CONDUCTOR_SERVICE_NAME` override) → `ScopeSpans` → `Span` (trace_id / span_id / name / start+end nanos / `Status`); no SDK batch-task spawning — byte-level control is foundational for fault injection.
- **Emit-path liveness:** Before timeline-engine emission, confirm gRPC channel to `127.0.0.1:4317` is connectable; a refused transport surfaces as `Result::Err`, not a verdict — this establishes the "OTLP egress liveness equivalent" (per §Standard Contracts, alongside the MCP readiness gate).
- **Workspace crate discipline:** `conductor-emit` depends on `conductor-core` (permitted); no cross-seam deps (compiler-enforced). Shared deps live in root `[workspace.dependencies]`.

## Anti-patterns to avoid
- Do NOT use the opentelemetry-otlp SDK exporter (it spawns batch tasks, hides fingerprint/status control, and forbids raw message manipulation — the opposite of fault injection).
- Do NOT emit batches dynamically on tokio work-stealing; the seeded RNG + `current_thread` runtime together guarantee emission-order determinism under the same seed.
- Do NOT add transitive web-framework or message-broker deps (Conductor has no HTTP/network service of its own; gRPC is client-only).

## Contract bindings
**emit ↔ timeline:** The emission primitive integrates into `run_timeline`'s per-transition loop as the gRPC egress point (timeline calls emit for each shaped span). **emit ↔ report:** The journal writer (report seam) already records emissions; this chunk adds the wire egress — sync/order of writes to journal vs. gRPC wire is the seam contract. **emit ↔ faults:** Fault helpers (next epochs) will perturb raw OTLP fields (status code, exception events, fingerprint identity, root-vs-child placement) — this scaffold provides the unperturbed base. **emit ↔ tests:** Transport-error tests use a loopback gRPC stub (determinism discipline: loopback only in tests, never in live Pulse integration).

## Acceptance criteria contributions
- "(arch) `conductor-emit` builds a well-formed `ExportTraceServiceRequest` from opentelemetry-proto 0.32.0 raw types (per §Established Decisions OTLP Emission Strategy)."
- "(arch) gRPC `TraceServiceClient` ships spans to `127.0.0.1:4317` via tonic 0.14.6 (per §Conventions Interface surfaces)."
- "(arch) Transport refusal surfaces as `EmitError` variant (thiserror), a `Result::Err` (verdict/error wall, per §Conventions Error handling + §Cross-cutting Patterns)."
- "(arch) Crate boundary: `conductor-emit` → `conductor-core` only; no cross-seam deps (per §Established Decisions Module Boundaries)."
- "(arch) `Cargo.lock` committed; audit/deny green; tonic 0.14 codegen via `tonic-prost-build` (per §Stack and Technologies + Infrastructure Patterns)."

## Relevant amendment history
- **2026-06-16-seeded-phase-scheduler:** Registered rand_chacha 0.9 / rand_core 0.9 in §Stack Determinism RNG + established decision; seeded RNG (per-gap jitter) is the timeline scheduler's sole non-determinism source — governs jitter shaping in emission timing.
- **2026-06-14-cargo-workspace-scaffold:** MSRV 1.88.0 → 1.94.1 (security-plan §Dependency Security CVE-2026-33056); tonic 0.14.6 requires ≥1.94.1.
# arch extract

## Relevance
Relevant — this chunk extends the OTLP emission primitives with fault-meaningful span status and intra-trace span tree construction.

## Constraints
1. Status-code field must use the raw opentelemetry-proto `Status { code: STATUS_CODE_ERROR, message }` struct (per §Stack and Technologies — OTLP emission strategy), never an exporter wrapper.
2. Span and trace IDs must be deterministic under seed (per §Established Decisions [Determinism RNG] `rand_chacha 0.9` + `seed_from_u64`); the builder must accept randomness as input, not own global entropy.
3. Egress surface unchanged — `127.0.0.1:4317` via existing tonic `TraceServiceClient` (per §Occupied Resources); no new transport or port binding.
4. Code lives in `conductor-emit` (per §Inherited Defaults — crate-per-seam workspace); no forbidden cross-seam dependency to `conductor-timeline` or other seam crates.
5. No new workspace dependency beyond opentelemetry-proto's already-present `Status` / `Span` fields (per §Occupied Resources); any addition requires `Cargo.lock` commit + cargo-audit/deny green.
6. Verdict/error wall preserved: `EmitError` for harness/transport faults; `tonic::Status` as first-class verification input (per §Established Decisions [Error Handling]).

## Patterns to follow
1. Raw OTLP message assembly (Status code construction, parent/child linkage via `parent_span_id` + shared `trace_id`) mirrors the scaffold chunk's hand-built protobuf approach (per §Design Philosophy "determinism under a seed" + §Stack "raw hand-built OTLP message structs").
2. Builder/parameter API exposes the root-vs-child placement selector as a composed surface on top of deterministic ID generation (caller supplies seed or IDs; builder assembles the tree).
3. Export via the existing `ExportTraceServiceRequest` → `ResourceSpans` → `ScopeSpans` → `Span[]` hierarchy; no new wrapper types beyond status/tree helpers.

## Anti-patterns to avoid
1. Clock-based or truly-random ID generation — seeded RNG only; reproducibility is non-negotiable (per §Design Philosophy).
2. Exception span events (P-006, P-017, P-018 scope) — Status field only in this chunk; exception events are a later chunk.
3. Cross-service topology or W3C Trace Context propagation — this chunk is single-service intra-trace parent/child only (multi-service topology is the later "Multi-service topology" chunk per P-027).

## Contract bindings
Emission primitives ↔ Verify / MCP read-back (P-005 hard pass/fail and P-008 calibration-region weighting are Epoch 5/7; this chunk outputs construction only) · Emit ↔ Report (run report JSONL journal captures the emitted traces; no new storage schema expected in this chunk).

## Acceptance criteria contributions
1. (arch) Code lives in `conductor-emit` per workspace boundary rules (arch §Inherited Defaults).
2. (arch) Status code construction uses raw opentelemetry-proto `Status` struct, never an exporter wrapper (arch §Stack and Technologies — OTLP emission strategy).
3. (arch) Span / trace IDs are deterministic under seed via `rand_chacha 0.9` `ChaCha8Rng` + `seed_from_u64`; builder accepts IDs as input (arch §Established Decisions [Determinism RNG]).
4. (arch) No new workspace dependency beyond opentelemetry-proto's existing fields; `Cargo.lock` committed + cargo-audit/deny green (arch §Occupied Resources).

## Relevant amendment history
(none) — no prior amendments to architecture.md touch the error-span emission surface (Epoch 3, chunk 2). The determinism RNG amendment (2026-06-16-seeded-phase-scheduler) and OTLP emission strategy (anchored in original §Established Decisions) are antecedent decisions but not amendments to this specific chunk's area.

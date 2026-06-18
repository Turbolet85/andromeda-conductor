# arch extract

## Relevance
Partial — PII payload corpus is an emission primitives chunk (Epoch 3) within `conductor-emit` scope; workspace placement and standard contracts apply.

## Constraints
1. Code lives in `conductor-emit` crate per the modular monolith crate-per-seam architecture (§Workspace / Core Structure + §Inherited Defaults).
2. OTLP emission surfaces MUST conform to opentelemetry-proto raw types (KeyValue, log body, exception-event fields) over tonic 0.14.6 + tonic-prost, NOT the OTel SDK (§Established Decisions [OTLP Emission Strategy]).
3. Determinism invariant: corpus generation is seeded via `seed_from_u64` + ChaCha8Rng (same seed ⇒ identical corpus shape) per §Design Philosophy (Determinism under a seed) + §Established Decisions [Determinism RNG].
4. Public corpus API re-exported from `conductor-emit/src/lib.rs` following the crate-internal composition pattern (existing `pub(crate)` builders in message.rs, exception.rs, logs.rs surfaced to public scope); no new external crate dependencies (§Inherited Defaults — module boundaries).
5. Transport faults surface as `EmitError` typed enum values (never panic), maintaining the verdict/error wall across emission seams (§Design Philosophy [Outcomes are values] + §Conventions [Error handling]).

## Patterns to follow
1. **Seeded deterministic generation:** ChaCha8Rng initialized with scenario seed → reproducible corpus state; existing precedent in `conductor-timeline` seeded phase scheduler.
2. **Builder composition:** embed PII across spans/logs/exceptions via existing resource/span/exception/log-record builders (message.rs, exception.rs, logs.rs pattern) — reuse TraceEmitter/LogsEmitter infrastructure.
3. **Structural validity for detector recognition:** each synthetic payload matches the shape Pulse's scrubber keys on (email regex / JWT segment count / bearer token prefix / API key format / PAN checksum / SSN dash positions / key=value KV string) — Pulse's P-047 category detector compliance is the proof bar, not abstract format specs.

## Anti-patterns to avoid
1. Real or host-derived PII in the corpus — synthetic generation only, never sensitive production data or host secrets.
2. Crossing the emission/verification boundary — this chunk produces the corpus + embeds it on the wire; scrub verification (P-035/P-047/P-048) and the `pii-scrub` scenario route to Epoch 7 (verify seam, MCP read-back).
3. Direct tokio dependency in a module crate — TraceEmitter/LogsEmitter already own the async context; the corpus module composes them, never directly spawns tokio tasks.

## Contract bindings
**Emit ↔ Verify:** The three signal types (spans/logs/exceptions) carrying PII are observably present on the wire (§Standard Contracts OTLP emission outbound to 127.0.0.1:4317 tonic/gRPC); downstream P-035/P-047/P-048 verification scenarios (Epoch 7, conductor-verify seam) will read these back via MCP and assert scrub structure-preservation — the test harness contract is signal observability, not scrub validation.

## Acceptance criteria contributions
- (arch) PiiCategory enum (or equivalent) defines exactly the seven P-047 categories — emails, JWT, bearer, API keys, credit cards, SSN, secret-like key=value.
- (arch) Corpus generation is seeded reproducible — same `seed_from_u64(u64)` input ⇒ identical category values.
- (arch) Corpus embeds and observably reaches the wire on all three signal types (spans, logs, exceptions) — integration test per signal type looping back to the gRPC channel.
- (arch) Each synthetic payload matches the structural shape its category detector recognizes (format validity for Pulse scrubber compliance, not just schema).
- (arch) Code lives in `conductor-emit` per workspace boundary rules; public API re-exported from `lib.rs`; no new external crates; transport faults surface as `EmitError`.

## Relevant amendment history
**2026-06-18-exception-events-fingerprint-control:** fingerprint primitive + exception-event builder co-located in `conductor-emit` (identical/path/line/type/frame variants), narrowing `conductor-faults` "fingerprint generation" to the storm fault (Epoch-7). Per §Infrastructure Patterns dir-tree: `conductor-emit` owns emission + exception fingerprint primitives; `conductor-faults` owns fault-specific fingerprint storms. This chunk (PII corpus) similarly composes existing emit builders (message.rs, exception.rs, logs.rs); the exception fingerprint primitive is available co-located in the same crate.
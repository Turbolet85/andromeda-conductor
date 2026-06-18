# arch extract

## Relevance
Partial — exception events and fingerprinting fit the emit epoch; fingerprint-computation discipline crosses into faults domain semantics requiring a seam decision.

## Constraints
- Per architecture.md §Established Decisions [OTLP Emission Strategy], raw `opentelemetry-proto` hand-built types give byte-level control required for deterministic exception-event content; use `opentelemetry-proto 0.32.0` + `tonic` 0.14.6 / `tonic-prost` 0.14.6 / `prost` 0.14 for all exception span events.
- Per §Stack and Technologies, the seeded RNG `ChaCha8Rng` (from `rand_chacha 0.9` + `rand_core 0.9`) is the sole source of non-determinism; fingerprint computation must use `ChaCha8Rng::seed_from_u64(scenario.seed)` to ensure "same seed + same exception spec ⇒ identical fingerprint".
- Per §Conventions [Config conventions], scenario config deserialization + bounds-checking with `garde 0.22.1` co-locates validation with serde structs; any exception-event config in scenarios must use `#[derive(Validate)]` + `#[garde(...)]` annotations in the owning seam crate.
- Per §Design Philosophy [Outcomes are values, errors are harness faults], exception-event emission failures (serialization, gRPC transport) are `Result::Err` (harness faults); verification outcomes (fingerprint match / mismatch) are typed return values (`Verdict`).
- Per §Occupied Resources [Environment variables], no new env var beyond the locked `CONDUCTOR_*` namespace; exception-event determinism is driven by scenario config + seed, not env.
- Per §Conventions [Data model conventions], fingerprints are stored as SQLite JSON1 TEXT arrays in `runs.db`; the emission-journal line schema records the expected fingerprint per the standard contract.
- Per §Stack and Technologies [Validation], `serde 1.0.x` + `garde 0.22.1` for exception-event config bounds (type names match Pulse's `exception.type` semantic; stacktrace frames are bounded sane).

## Patterns to follow
- Exception-event builders in `conductor-emit` extend the existing `TraceEmitter` / `span_tree.rs` / `message.rs` API surface — add methods to attach exception span events onto error spans without breaking the existing egress path (OTLP to `127.0.0.1:4317` unchanged).
- Fingerprint computation is deterministic, seeded (shape-projection not wall-clock), and co-located with the exception content it derives from; the output (expected fingerprint string) feeds the emission-journal `fingerprints` array per §Standard Contracts.
- Same discipline as error-span builder: `ChaCha8Rng::seed_from_u64(scenario.seed)` for reproducible line-insensitive fingerprints across runs.

## Anti-patterns to avoid
- Do NOT use `opentelemetry` SDK exporter (it spawns background batch tasks on the tokio runtime, hides fingerprint identity, and offers no line-number control — raw message structs are the lock).
- Do NOT compute fingerprints from wall-clock or tokio virtual-clock state; use seeded determinism only.
- Do NOT introduce new `CONDUCTOR_*` or `ANDROMEDA_*` env vars to control exception-event shape; scenario config is the sole driver.

## Contract bindings
- **Emit → Report:** Exception events populated into the `fingerprints` JSON array of the standard run-report envelope (per §Standard Contracts); expected fingerprints journaled per line schema (tests/obs-owned).
- **Emit → Timeline:** Exception events are attached in response to scenario config + timeline phase (e.g., error-spike scenario at P-006); the timeline scheduler drives when exceptions are emitted (no ad-hoc injection).
- **Emit → Verify (later):** Expected fingerprint from this chunk is the baseline; the verify seam later reads back Pulse's derived fingerprint via MCP `retrieve_report` and asserts the match (P-036 fingerprint-recurrence check, P-017/P-018 path/line variants — out of scope for this chunk, Epoch 7+).

## Acceptance criteria contributions
- (arch) Exception-event builder code lives in `conductor-emit` per workspace boundary rules (arch §Inherited Defaults); exception-event exception-type/message/stacktrace attributes conform to OpenTelemetry Semantic Conventions (OTel spec, not Pulse-specific).
- (arch) Fingerprint-compute function is deterministic under seed; same scenario + seed produces identical expected fingerprint; fingerprints are journaled in the emission-journal `fingerprints` field per line schema.
- (arch) Exception-event emission failures (serialization, gRPC transport) are `Result::Err` (harness); fingerprint / exception-content mismatches are typed `Verdict` values (outcomes).
- (arch) No new env var beyond the locked `CONDUCTOR_*` namespace; exception-event shape is driven by scenario config + seed determinism only.

## Relevant amendment history
- **2026-06-16-seeded-phase-scheduler:** `rand_chacha 0.9` + `rand_core 0.9` registered in §Stack; §Established Decisions locked `ChaCha8Rng` + `seed_from_u64` for platform/version-stable determinism. This chunk must use the same seeding discipline for exception-event fingerprint computation.
- **2026-06-15-config-validation-surface:** `garde 0.22.1` registered (not 0.23.0 — `garde_derive 0.23.0` absent from registry); field-level `#[garde(custom)]` only; cross-field invariants use `Context` pattern. Exception-event config validators co-locate with serde structs; apply this pattern if exception-type/stacktrace cross-field rules emerge.

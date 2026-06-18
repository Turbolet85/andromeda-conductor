# security extract

## Relevance
Partial — exception-event emission and fingerprint primitives fall within input validation + error handling scope; deterministic fingerprint derivation and stacktrace control require bounded I/O discipline and error sanitization.

## Constraints
- Exception-event stacktrace content must be deterministically controlled (same seed + same spec ⇒ identical stacktrace frames + line); exception events carry OTLP exception-semantic attributes (`exception.type` / `exception.message` / `exception.stacktrace`), so stacktrace strings must be bounded (no unbounded recursion) and must not leak absolute host paths or internal struct names to the journal (per security-plan §Error Handling, run-report artifact sanitization).
- The line-insensitive fingerprint primitive computes fingerprints deterministically (same seeded `ChaCha8Rng::seed_from_u64` discipline as the error-span builder, shape-projection not wall-clock) and must remain stable across line-number variation; if it ever becomes a public API surface beyond the emission journal, it requires the same input-validation gate as other seam boundaries (per security-plan §Input Validation, boundary discipline).
- Exception-event data is self-generated synthetic telemetry (part of `runs.db` + JSONL journals, per security-plan §Threat Model Summary §Data classification); stacktrace/fingerprint fields are journaled into the emission journal's `fingerprints` field and run-report `fingerprints` JSON1 TEXT array — ensure no canonicalized path handles or internal seam-crate enum/struct names leak into those fields (per security-plan §Error Handling).
- Fingerprint derivation from exception content is deterministic (seeded RNG, not I/O or wall-clock dependent) and emitted only on the egress path, not read back in this chunk; no network exposure beyond the existing loopback gRPC `127.0.0.1:4317` egress (per security-plan §Threat Model Summary §Attack surface).
- No new external dependency is introduced for exception-event construction or fingerprint compute (use only `serde`, `serde_json`, `opentelemetry` semantic constants, and `rand`/`rand_chacha` already in the workspace); if any new crate is added, confirm `cargo-audit` + `cargo-deny` green (per security-plan §Dependency Security).

## Patterns to follow
- Error-span exception-event builder pattern (extends existing `TraceEmitter` / `span_tree.rs` / `message.rs` API): stacktrace as a structured string array (not recursive JSON nesting) to bound protobuf/prost decode depth; validate field lengths at serialization time.
- Fingerprint compute as a pure, side-effect-free function (shaped by the error-span builder's determinism discipline) seeded from the scenario seed via `ChaCha8Rng::seed_from_u64`; output as a stable hex-encoded string for journaling into the `fingerprints` array.
- Run-report artifact path handling (per security-plan §Error Handling): exception stacktrace frames and derived fingerprints recorded in `<run_id>.jsonl` and `runs.db` rows use relative module/function paths only (no canonicalized `CONDUCTOR_*` dirs, no absolute host paths); the fingerprint itself is an opaque hex string, not a path.

## Anti-patterns to avoid
- Do NOT log or journal absolute file paths (canonicalized `CONDUCTOR_RUNS_DIR`, `CONDUCTOR_SCENARIOS_DIR`, etc.) or internal seam-crate struct/enum names in the exception-event stacktrace or fingerprint fields — artifact sanitization is mandatory (security-plan §Error Handling + Anti-Patterns §Logging).
- Do NOT use wall-clock time, tokio virtual clock, or any non-deterministic I/O in fingerprint derivation — keep the seeded RNG discipline; never call `SystemTime::now()` or `Instant::now()` within the fingerprint-compute path (security-plan §Error Handling; Cross-cutting Patterns: Determinism discipline).
- Do NOT introduce unbounded exception-event stacktrace depth or frame count — bounded arrays only; if stacktrace arrays ever become config/API input (not today), validate with garde `#[garde(length(...))]` (security-plan §Input Validation).

## Contract bindings
- **obs ↔ emit:** Exception-event shape and the emission-journal line schema's `fingerprints` field — obs owns PII scrubbing + cardinality (red-flag any real file paths / usernames in stacktraces); emit owns journaling the computed fingerprint per run. Stored in `runs.db` JSON1 `fingerprints` TEXT array + `<run_id>.jsonl` for later MCP read-back assertion (not this chunk).
- **tests ↔ emit:** Acceptance intent specifies same-seed/same-spec determinism + identical/path/line variant relationships; test harness verifies stacktrace shape + fingerprint stability with golden tests (nextest, no real Pulse).

## Acceptance criteria contributions
- (security) Same seed + same exception spec ⇒ identical exception-event stacktrace content and identical expected fingerprint (determinism verification, no wall-clock or I/O in fingerprint compute).
- (security) Exception-event stacktrace frames and fingerprints in the emission journal + `runs.db` contain no absolute host paths, no internal struct/enum names, no library versions — only module/function paths and sealed fingerprint strings.
- (security) Fingerprint-compute function is side-effect-free and seeded exclusively via `ChaCha8Rng::seed_from_u64` (no tokio virtual clock, no syscalls, no wall-clock dependency).
- (security) `cargo-audit` + `cargo-deny` remain green (no new dependency vulnerabilities introduced).

## Relevant amendment history
- **2026-06-15-structured-logging-stack** — obs identity env-handles (`CONDUCTOR_SERVICE_NAME` / `CONDUCTOR_ENV`) are non-path string labels (no validation needed), distinct from the `CONDUCTOR_*` path handles that canonicalize + bounds-check. Reinforces that observability labels and error artifacts stay separate from path validation (security-plan §Input Validation note added).

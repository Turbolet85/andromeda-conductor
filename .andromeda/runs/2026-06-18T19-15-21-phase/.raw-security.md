# security extract

## Relevance
Relevant — the chunk introduces PII payloads across three OTLP signal types; applies to §Data Protection / §Input Validation / §Logging & Monitoring / §Error Handling boundary rules.

## Constraints
- All seven P-047 PII categories (email, JWT, bearer, API key, credit card/PAN, SSN, secret key=value) embedded in spans/logs/exceptions MUST be structurally valid per their category detector pattern (security plan §Data Protection, corpus determinism per scope.md acceptance intent — "each emitted payload is structurally valid for its category").
- Seeded corpus generation (same seed ⇒ identical corpus) via ChaCha8 or equivalent PRNG to satisfy the determinism invariant; no randomness bleeding between runs (security plan §Input Validation "non-negative durations, sane ramp factors"; scope.md "determinism invariant").
- All PII corpus values are synthetic fixtures — never sourced from or derived from real host/network/secrets data; zero credentials, no `DATABASE_URL`, no cloud env vars (security plan §Secret Management, §Threat Model Summary "no secrets owned by Conductor", §Anti-Patterns §Secrets).
- Emission-side builders (`TraceEmitter`, `LogsEmitter`) self-validating; transport faults surface as typed `EmitError` (Result::Err), never panics — consistent with sibling emit modules (security plan §Error Handling "verdict/error wall keeps `Result::Err` strictly for harness faults"; scope.md "builder is self-validating type").
- Run-report artifacts and any debug logs must NOT leak the seeded corpus values or internal `PiiCategory` enum names into agent-parseable output (`<run_id>.md`, `runs.db`, JSONL journals); sanitize at the `anyhow` edge (security plan §Error Handling "no stack traces, file paths, internal struct names exposed"; scope.md "NOT this chunk" for Pulse scrub verdict, but emission must be audit-clean).
- PII payloads as OTLP wire KeyValue/body/event attributes are self-observation data (synthetic test telemetry, not user-derived); they must not trigger false-positive secret-scanning in pre-commit or CI (security plan §Dependency Security "no rigid allowlist at Minimal tier"; scope.md "synthetic data only" by construction).

## Patterns to follow
- `PiiCategory` enum mirrors the seven P-047 categories exactly — sourced from the input.md coverage matrix (scope.md line 64–65) or parent architecture; no variants added or omitted.
- Seeded-corpus factory function takes `seed: u64`, returns the same structured payloads per invocation; expose to `conductor-emit` public API for deterministic test scenarios.
- Emission builders reuse existing `message.rs` / `exception.rs` / `logs.rs` infrastructure (scope.md "composes existing… primitives"); KeyValue injection pattern consistent with sibling span/log builders.
- Integration test per signal type (spans, logs, exceptions) — verify corpus reaches wire on loopback gRPC client, emit + workspace nextest green (scope.md acceptance intent).

## Anti-patterns to avoid
- NEVER hardcode or pre-calculate PII corpus values — use seeded generation so the same scenario+seed reproduces the corpus deterministically (security plan §Anti-Patterns §Code Patterns, §Input "NEVER deserialize scenario config without garde validation"; scope.md determinism).
- NEVER emit real/host-derived identifiers (hostname, user, process ID, actual email/token) as corpus — only structurally valid synthetic fixtures (security plan §Data Protection "no PII owned by Conductor"; scope.md "never real or host-derived PII").
- NEVER log the emitted PII corpus unredacted to stdout/stderr or `runs.db` — run-report artifacts must remain audit-clean (security plan §Anti-Patterns §Logging "NEVER expose… internal struct/field names to the operator"; scope.md "emission-side only").

## Contract bindings
- **obs → validation-1 (verdict):** PII corpus observability reaches the wire per scope.md determinism + wire contract; downstream pii-scrub scenario (P-035, Epoch 7) reads it back via MCP and asserts Pulse scrubbed it (NOT this chunk).
- **emit module ↔ acceptance gates:** unit + loopback integration tests; nextest / clippy `-D` / llvm-cov / doctest — same suite as sibling emit modules (scope.md "gates green").

## Acceptance criteria contributions
- **(security) All seven P-047 PII categories represented in `PiiCategory` enum** — grep verifies exactly seven variants, matching input.md coverage matrix line 64–65.
- **(security) Corpus seeded + reproducible** — unit test: identical `seed` value produces identical corpus across two invocations (determinism invariant).
- **(security) Synthetic payloads only, no real/host/secret data** — static analysis: no `std::env`, `hostname`, `get_user_home`, credential reads, or secrets in corpus-generation path; scope.md "synthetic data only".
- **(security) Emission reaches wire on all three signal types** — loopback integration test: corpus injected into spans/logs/exceptions via builders; tonic client observes each category on the wire before shutdown.

## Relevant amendment history
- **2026-06-15-structured-logging-stack** — obs identity env-handles (`CONDUCTOR_SERVICE_NAME`, `CONDUCTOR_ENV`) clarified as non-path labels (JSON-escaped log values), no validation boundary. Not directly this chunk, but reinforces that self-obs strings (including PII corpus metadata if any) must be sanitized at the edge (§Input Validation note + Anti-Patterns §Logging).
- **2026-06-15-dependency-audit-gate** — cargo-audit/cargo-deny confirmed green (0.22.1 / 0.19.4 floors), toolchain 1.94.1 done. New dependency tree for PII corpus (likely `rand`/`rand_chacha` for PRNG) must pass cargo-audit/deny in this chunk's build; no new advisories introduced.
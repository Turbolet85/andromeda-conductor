# security extract

## Relevance
Partial — port-occupier chunk introduces a new deliberate inbound bind exception; applies to threat-boundary exception, input validation (bind addr parameterization), and error handling (typed bind-failure conditions).

## Constraints
- Must honor the sole-deliberate-exception discipline: loopback-only bind (`127.0.0.1:4317`), never widened to `0.0.0.0` or routable interface (security-plan §Threat Model Summary §Attack surface — port-occupier single-exception, §Anti-Patterns §Universal "scope law, trust boundary").
- RAII release discipline mandatory: bind MUST be released deterministically on cleanup (Drop/explicit release) so subsequent scenarios' egress reaches the real Pulse on `:4317` (security-plan §Threat Model Summary §Attack surface — "held socket"; scope.md "leaked occupier would silently break the rest of the suite").
- Bind-failure condition MUST be typed (not panic) when port already held — surface as harness fault through the verdict/error wall, never `unwrap` (security-plan §Error Handling, §Anti-Patterns §Universal).
- Cross-platform exclusivity: `SO_REUSEADDR`/`SO_EXCLUSIVEADDRUSE` semantics must NOT permit Pulse co-bind — Windows default-exclusive is the safe default; confirm Linux does not reuse (scope.md "determinism-critical detail").
- No Pulse process management, no OTLP/gRPC protocol speaking, no scenario-config wiring — fault is a standalone primitive exercised in isolation (scope.md "crate-per-seam law": conductor-faults must NOT pull conductor-emit/conductor-timeline).

## Patterns to follow
- Standalone fault helper precedent: matches `conductor-emit` module pattern (`rate.rs`/`latency.rs` style) — `port_occupier.rs` as a `PortOccupier` type re-exported from `lib.rs` with module doc (scope.md "matches conductor-emit module precedent").
- Unit-test isolation: default to ephemeral `:0` port in tests to prove exclusivity + release mechanics with zero flakiness, never colliding with real Pulse or fixed CI port (scope.md "address parameterization + test isolation").
- Typed error/condition for infrastructure faults: conductor-faults errors surface as Result, routed through verdict/error wall as harness faults (scope.md "verdict/error wall: typed conductor-faults error for could not bind").

## Anti-patterns to avoid
- NEVER widen the bind beyond loopback: `0.0.0.0` or routable interface violates the trust-boundary invariant (security-plan §Anti-Patterns §Universal — scope law, trust boundary).
- NEVER leak the occupier socket: an unreleased bind silently breaks subsequent egress to Pulse (scope.md consequence).
- NEVER panic on bind failure: port already held is a typed condition (Result::Err), not a panic (security-plan §Error Handling, §Anti-Patterns §Universal).
- NEVER set socket flags that permit reuse/co-binding: confirm `SO_REUSEADDR` / `SO_EXCLUSIVEADDRUSE` semantics block Pulse's concurrent bind.

## Contract bindings
Verdict/error wall (faults seam) ↔ conductor-core (shared error/types); confirms research needed on `conductor-core` dependency (scope.md open question: "only existing edge is conductor-core, to confirm in research").

## Acceptance criteria contributions
- (security) Loopback-only assertion: `127.0.0.1` hardcoded, never parameterized to a user-supplied address; grep confirms no `0.0.0.0` or routable bind (security-plan §Threat Model Summary §Attack surface exception).
- (security) RAII release: socket Drop implementation or explicit release method proved by unit test that re-binds the same address after occupier drops (scope.md "clean, idempotent release: after drop/release address is re-bindable").
- (security) Cross-platform exclusivity: unit test on ephemeral port confirms second bind to occupied address fails (Windows exclusive, Linux reuse explicitly disabled or confirmed not applicable); no `unwrap` on bind failure.
- (security) Typed bind-failure condition: conductor-faults module exports a typed error variant for bind-EADDRINUSE (e.g. `PortBindFailed`), never panics; verdict/error wall consumes it as `Result::Err(PortBindFailed)` (security-plan §Error Handling).
- (security) Crate-per-seam verified: grep confirms `conductor-faults` does not import conductor-emit / conductor-timeline / conductor-report (scope.md "crate-per-seam law").

## Relevant amendment history
2026-06-15-dependency-audit-gate — confirms toolchain bumped to 1.95.0 / `rust-version = 1.94.1`, clearing tar-rs symlink-chmod CVE-2026-33056 / RUSTSEC-2026-0033 that affects the `cargo build` extraction path (security-plan §Dependency Security §Pinning: "keep current so SQLite patch bumps flow in" + tar-rs CVE). No *port-occupier specific* amendments exist; this chunk is the first to exercise the sole-deliberate-bind exception in substantive code, so amendment history is empty for this domain area.

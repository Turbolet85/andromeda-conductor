# security extract

## Relevance
partial — core determinism contract + loopback-only telemetry emission; minimal config/auth/dependency surface

## Constraints
1. Seeded determinism via `ChaCha8Rng::seed_from_u64` — identical seed ⇒ identical rate-curve output, cross-platform stable (security-plan §Threat Model Summary; loopback-client trust boundary; Epoch-3 determinism discipline).
2. Synthetic data only — the rate-curve primitive emits synthetic OTLP span counts; no real user data, PII, or external input (security-plan §Threat Model Summary § Data classification: test-telemetry).
3. No new dependencies without audit — if any dependency is added counter to the scope's "no new dependency expected," it MUST clear `cargo-audit` ≥0.22 + optional `cargo-deny` ≥0.19 before merge (security-plan §Dependency Security; standing gate per amendment 2026-06-15-dependency-audit-gate).
4. Loopback-only OTLP egress — reuses existing trace/span message builders for `127.0.0.1:4317`; no new network surface introduced (security-plan §Threat Model Summary § Attack surface — OTLP/gRPC egress).
5. No wall-clock timestamp pollution — if the primitive emits journal lines or metadata with timestamps, use `std::time::SystemTime`/`Instant`, never tokio virtual clock (security-plan §Security Anti-Patterns § Logging; Conventions: Determinism discipline).

## Patterns to follow
1. **Seeded per-window determinism** — match the latency-shaping precedent in the same crate: `ChaCha8Rng` via `seed_from_u64`, rate-curve iterations keyed by scenario seed + window index.
2. **Loopback-capture unit testing** — reuse existing OTLP builders; emit shaped span stream to loopback and assert curve properties (ramp monotonicity, breathing oscillation) via loopback fixture (matches `pii.rs`/latency-shaping test style).
3. **Module-level documentation** — re-export from `lib.rs` with module-doc describing the determinism contract and rate-curve semantics (matches `pii.rs` precedent).

## Anti-patterns to avoid
1. NEVER use tokio virtual clock for timestamps — if wall-clock emission is journaled, use `std::time::SystemTime`/`Instant` only (security-plan §Anti-Patterns § Logging; clock-source corruption breaks ground-truth SLO math).
2. NEVER skip `cargo-audit` / `cargo-deny` if any dependency is added — the audit gate is the Minimal-tier residual-risk control for the OTLP/gRPC tree (security-plan §Anti-Patterns § Universal).
3. NEVER introduce config/scenario wiring or guard-band validation — this is a standalone seeded primitive; scenario-config integration deferred (scope: "No scenario-config / garde wiring").

## Contract bindings
- **obs ↔ journal schema:** if per-window rate/count is journaled, schema ownership lives in obs-plan; clock source (`std::time`) matters for SLO-math ground truth.

## Acceptance criteria contributions
- (security) No new dependencies, OR if any dependency added: `cargo audit` + `cargo-deny check advisories` pass (Minimal-tier residual-risk gate per §Dependency Security).
- (security) Seeded determinism verified: identical `CONDUCTOR_SEED` + scenario ⇒ identical rate-curve span counts (unit test assertion).
- (security) Loopback-capture test confirms shaped stream conforms to rate-curve spec (ramp monotonicity within tolerance; breathing oscillation verified).
- (security) No wall-clock timestamps in emitted journal — if timestamps appear, use `std::time::SystemTime` only (ground-truth artifact integrity per §Anti-Patterns § Logging).

## Relevant amendment history
- **2026-06-15-dependency-audit-gate:** cargo-audit + cargo-deny are minimum floors; toolchain ≥1.94.1 confirmed. Applies as a standing CI/pre-merge gate for any new dependencies this chunk introduces (currently expected: none). (security-plan §Dependency Security; standing gate, not chunk-specific.)

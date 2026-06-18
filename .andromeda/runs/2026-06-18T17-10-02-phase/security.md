# security extract

## Relevance
Partial — latency-shaping is an emission primitive (no IPC/HTTP/auth/secrets); input-validation + determinism checks apply.

## Constraints
- Per §Input Validation, seed-based durations must use `ChaCha8Rng` for cross-platform determinism (no system-clock-derived randomness corrupting reproducibility).
- Per §Error Handling, any latency-sampling error must collapse to `anyhow` only at `conductor-emit` module edges; internal `thiserror` enum (e.g., `LatencyError`) stays typed.
- Per §Dependency Security §Universal, `cargo-audit` (≥0.22.1 floor) + recommended `cargo-deny` (≥0.19.4 floor) must run green before release; `Cargo.lock` committed.
- Per §Threat Model Summary §Data classification (test-telemetry type), realized duration sequences in JSONL journals / `runs.db` rows carry no confidentiality requirement — they are self-generated synthetic OTLP fault data, not user-derived.
- Per §Error Handling, run-report artifacts must not leak absolute host paths or internal struct names from `conductor-emit` (keep identity fields only — operation name, seed, profile targets, realized p50/p95/p99).

## Patterns to follow
- Seeded-RNG pattern: `ChaCha8Rng` seeded from the determinism contract (same profile + seed ⇒ identical duration sequence); mirror the existing `conductor-timeline` discipline.
- Raw-OTLP Span timing field contract: shape only `start_time_unix_nano` / `end_time_unix_nano` on the existing `Span` struct; use `std::time::SystemTime` (not tokio virtual clock) for base stamps per journal-relative SLO invariant (§Error Handling Logging ban).
- Percentile-distribution sampling pattern: given p50/p95/p99 targets + sample count ≥50, produce durations whose realized percentiles converge to targets within sampling tolerance (Acceptance intent val-1).

## Anti-patterns to avoid
- NEVER seed the duration RNG from a non-deterministic source (wall-clock, `OsRng`, SystemTime delta); only `ChaCha8Rng(seed=CONDUCTOR_SEED)` ensures reproducibility.
- NEVER expose raw error types or stack traces in JSONL journals / `runs.db` rows — keep verdict/identity fields only (operation, p50/p95/p99 targets, realized percentiles); sanitize at the emit boundary.
- NEVER skip `Cargo.lock` commit or run `cargo-audit` green before release — dependency audit is the Minimal-tier residual-risk control for the OTLP/gRPC/rand tree.

## Contract bindings
- Conductor-core (scenario-config validation): the p50 ≤ p95 ≤ p99 ordering invariant is authoritative in garde layer; latency.rs consumes a validated profile + asserts precondition locally only.
- Conductor-verify (Epoch 5, MCP read-back): latency-regression *orchestration* (90s baseline → 3× p99 90s → 60s persistence check) is NOT this chunk; conductor-verify consumes the realized durations to detect regressions; the ≥50-sample floor appears here only to ensure producer capacity.

## Acceptance criteria contributions
- (security) Determinism verified: identical seed + profile produces identical duration sequence (unit test + reproducibility across platforms via `ChaCha8Rng`).
- (security) Error boundary sanitized: no absolute paths / internal struct names in JSONL journals or `runs.db` rows; verdict/identity fields only.
- (security) `cargo audit` (≥0.22.1 floor) + `cargo deny` (≥0.19.4 floor, recommended) pass green; `Cargo.lock` un-drifted.

## Relevant amendment history
- **2026-06-15-dependency-audit-gate:** cargo-audit/cargo-deny version pins reframed as **minimum floors** (audit tool versions are external CLI, not lock-pinnable; advisory DB runtime-fetched); toolchain ≥1.94.1 confirmed **done** (1.95.0 channel). Impact: this chunk's dependency audit must clear using ≥0.22.1 / ≥0.19.4 and confirm `Cargo.lock` is un-drifted.
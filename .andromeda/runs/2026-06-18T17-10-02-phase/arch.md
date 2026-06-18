# arch extract

## Relevance
partial — chunk's latency-shaping primitive is a standalone Epoch 3 emission surface; architecture governs module placement, cross-cutting patterns, and OTLP surface shape, but not latency-distribution algorithms.

## Constraints
- Per **architecture.md §Established Decisions [OTLP Emission Strategy]**, raw opentelemetry-proto 0.32.0 types (`Span` with `start_time_unix_nano` / `end_time_unix_nano` fields) are the sole interface; no SDK exporter involvement.
- Per **§Design Philosophy (Determinism under a seed)**, target profile + seed ⇒ identical duration sequence — same determinism RNG (rand_chacha 0.9 `ChaCha8Rng` seeded via `seed_from_u64`) already owned by conductor-timeline per **§Established Decisions [Determinism RNG]**.
- Per **§Stack and Technologies**, Rust 2024 + tokio 1.48.x (core-owned, `current_thread` flavor), serde 1.0.x, no web framework, synchronous storage only.
- Per **§Conventions (Config conventions)**, p50 ≤ p95 ≤ p99 ordering is validated at scenario-config load (conductor-core's garde layer) — this primitive *consumes* a pre-ordered profile, may locally assert ordering as precondition.
- Per **§Compiler-enforced module seams**, code lives in `conductor-emit` crate (named in §Occupied Resources / §Inherited Defaults); cross-seam dependencies are compile-forbidden — no direct dep on conductor-timeline (must use its RNG types via workspace.dependencies or sealed seam interface).
- Per **§Infrastructure Patterns (Build system)**, gate with `cargo clippy -D warnings`, `cargo nextest` (zero-retry `ci` profile in `.config/nextest.toml`), `cargo-audit`, and `cargo-deny` (Cargo.lock committed; test versions in test-plan §4).

## Patterns to follow
- **Seeded RNG discipline** (per **§Cross-cutting Patterns · Determinism discipline**): use the workspace-owned rand_chacha 0.9 `ChaCha8Rng` keyed to `scenario.seed` (cf. conductor-timeline precedent); reproducibility is enforced in architecture.
- **Wall-clock vs virtual clock** (per **§Design Philosophy (Journal-relative ground truth)** + **§Cross-cutting Patterns**): span timestamps come from `std::time::SystemTime`/`Instant`, never tokio's virtual clock; journal-relative SLO math depends on this.
- **Emit primitive as library function** (per **§Design Philosophy (Headless-drivable core)** + **§Conventions (three pinned surfaces)**): the latency primitive is a function taking a profile + RNG, returning durations; no I/O side-effects, no tokio task spawn, no direct TraceEmitter call (caller orchestrates emission).

## Anti-patterns to avoid
- **Do NOT split validation across boundaries**: p50 ≤ p95 ≤ p99 ordering + severity-mix sums are conductor-core's garde layer job; this primitive assumes input is pre-validated (may assert locally, but is not the source of truth).
- **Do NOT emit timestamps via tokio::time**: journal-relative SLO math (read_back_observed_at − journal_emitted_at) depends on wall-clock source; tokio's virtual clock is verboten per **§Design Philosophy**.
- **Do NOT introduce new crate deps or ports**: latency primitive must not add env vars beyond the existing `CONDUCTOR_*` namespace or bind any port; any new workspace-level dependency goes through cargo-audit/cargo-deny (gated in CI per §Infrastructure Patterns).

## Contract bindings
- **emit ↔ timeline**: the latency primitive consumes seeded rand_chacha 0.9 `ChaCha8Rng` — an interface provided by conductor-timeline's Epoch 3 chunk (seeded-phase-scheduler, amendment 2026-06-16) for deterministic per-gap jitter; this chunk reuses that same RNG for duration distribution sampling.
- **emit ↔ report**: the `Span` objects the latency primitive shapes (with timed durations) flow to conductor-report's JSONL emission journal and `runs.db` (read_back_observed_at, latency_ms, slo_tier columns — Epoch 6); the journal-relative SLO math depends on wall-clock span timestamps.
- **emit ↔ verify**: conductor-verify (Epoch 5) will judge whether realized percentiles match targets within tolerance + sample-count floor; this primitive only produces durations; SLO logic is downstream.

## Acceptance criteria contributions
- (arch) Code lives in `conductor-emit` per workspace boundary rules (§Inherited Defaults, crate-per-seam layout — amendment 2026-06-18 records the exception-event fingerprint primitive placement precedent).
- (arch) Span `start_time_unix_nano` / `end_time_unix_nano` shaped by the latency primitive conform to raw opentelemetry-proto 0.32.0 types; no SDK exporter introduced.
- (arch) Deterministic under a seed: identical seed + target profile reproduces identical duration sequence across platforms (ChaCha8 seeding per §Stack + §Established Decisions [Determinism RNG]).
- (arch) No new env var beyond existing `CONDUCTOR_*` namespace; no new ports bound (latency primitive is a library function, not an I/O surface).

## Relevant amendment history
- **2026-06-16-seeded-phase-scheduler**: rand_chacha 0.9 + rand_core 0.9 registered in §Stack + §Established Decisions [Determinism RNG] — this chunk's RNG seeding discipline reuses that already-locked interface (ChaCha8Rng, seed_from_u64, cross-platform stability).
- **2026-06-18-exception-events-fingerprint-control**: fingerprint primitive (`fingerprint()` + exception-event builder) placed in conductor-emit, narrowing conductor-faults to fault-specific generation (Epoch-7 fingerprint-storm) — latency primitive joins emit as another Epoch 3 standalone primitive.
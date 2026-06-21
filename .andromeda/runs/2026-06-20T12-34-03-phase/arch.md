# arch extract

## Relevance
relevant — fault helpers primitive (duty-cycle descriptor) for the activity-floor pattern, placed in conductor-faults module per architecture §Occupied Resources (crate names).

## Constraints
- **Crate placement:** code must live in `conductor-faults` per §Occupied Resources (Crate names) — "conductor-faults — fault helpers (ramps, silence, port-occupier, fingerprint-storm)".
- **Module boundary:** concept-named file (matches `gap.rs` / `silence.rs` / `port_occupier.rs`), per §Conventions (Naming patterns); exported via `lib.rs` module + public re-export.
- **Validator contract:** FaultError variant (Result-based validation, never panic), aligning with §Design Philosophy (Outcomes are values, errors are harness faults) and §Conventions (Error handling with typed `thiserror` enums per seam).
- **Determinism:** no runtime seed in the descriptor (fixed duty-cycle values), per §Design Philosophy (Determinism under a seed) — seeded jitter applied later by timeline at §Cross-cutting Patterns.
- **Dependencies:** no new deps; Cargo.lock remains un-drifted, per §Stack and Technologies pinned set.
- **Build gates:** `-p conductor-faults` + workspace nextest, clippy `-D warnings`, doctest, llvm-cov; green required per §Infrastructure Patterns (Build system CI/CD approach).

## Patterns to follow
- **Duty-cycle descriptor pattern:** own exact un-jittered values (like `EmissionGap::new` returns a fixed `gap_ms` duration), exposing `*_ms` accessors and phase-query methods for timeline/PhaseSpec boundary consumption per the amendment history §2026-06-18 (fingerprint primitive placed in conductor-emit; conductor-faults owns descriptors like gap/silence, emit owns primitives).
- **Error typing:** extend `#[non_exhaustive] enum FaultError` with new duty-cycle-validation variants (`Result<Self, FaultError>` constructor), matching the `EmissionGap` constructor shape per §Established Decisions (Error Handling).
- **Crate-doc registration:** extend `lib.rs` crate-doc "Shipped so far" line to 4/4 mirroring the `silence.rs` addition pattern, per §Occupied Resources (crate names documentation).

## Anti-patterns to avoid
- **Panic on invalid bounds:** invalid active/quiet durations must be typed `FaultError`, never `panic!` or `unwrap`, per §Design Philosophy (Outcomes are values, errors are harness faults).
- **Seeded RNG in the descriptor:** the duty cycle must be a fixed, deterministic descriptor; jitter application is timeline's responsibility (Epoch 7) per §Established Decisions (Determinism RNG is `rand_chacha` in the timeline scheduler alone).
- **Cross-seam module dependency:** BurstyTrain implementation must not reach upward into timeline/emit/verify/report seams; only downstream crates (timeline via import) depend on faults.

## Contract bindings
- **timeline (Epoch 7):** BurstyTrain descriptor feeds `PhaseSpec` boundary when timeline wiring consumes `*_ms` accessors and phase-query methods (active vs quiet at offset) to apply seeded jitter and drive the repeating duty cycle; the timeline owns seeded jitter application.
- **scenario config (Epoch 7):** `BurstyTrain` type participates in declarative scenario config wiring (garde validation co-located with struct) when the scenario model adds activity-floor / restart-suppression patterns.
- **obs / fault span (Epoch 7/8):** a `fault.bursty_train` (or similar) tracing span will be registered later in the obs/timeline wiring; currently no observation integration in this epoch.

## Acceptance criteria contributions
- (arch) Code lives in `conductor-faults/src/train.rs` per workspace boundary rules (arch §Occupied Resources).
- (arch) `BurstyTrain` type exported via `lib.rs` re-export + crate-doc extended to 4/4 completion; no new dependency drift (Cargo.lock unchanged).
- (arch) Constructor returns `Result<Self, FaultError>` with typed error variant(s) for out-of-range duty cycles (arch §Conventions — Error handling, §Design Philosophy — Outcomes are values).
- (arch) No seeded RNG or jitter in the descriptor; duty-cycle values are exact and deterministic per scenario input (arch §Established Decisions — Determinism RNG reserved for timeline scheduler alone).

## Relevant amendment history
- **2026-06-14-cargo-workspace-scaffold** — MSRV raised to 1.94.1; applies to build gates (clippy `-D warnings`, rust-toolchain.toml).
- **2026-06-14-cargo-workspace-scaffold** — self-observation stack row added (tracing 0.1.44 + tracing-subscriber 0.3.23); not in scope for this epoch (obs wiring deferred to Epoch 7/8).
- **2026-06-16-test-framework-fixtures-coverage-tooling** — test/coverage toolchain (cargo-nextest zero-retry, clippy, doctest, llvm-cov) registered; applies to build gates validation in this chunk.
- **2026-06-18-exception-events-fingerprint-control** — fingerprint primitive placed in conductor-emit; conductor-faults owns descriptors (gap/silence/port_occupier/bursty_train) not primitives — no change to this chunk's responsibility, but clarifies module boundary (faults is descriptors + helpers, emit is primitives + event builders).
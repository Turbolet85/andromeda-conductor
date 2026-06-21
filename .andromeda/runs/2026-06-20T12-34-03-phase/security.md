# security extract

## Relevance
Partial — fault-helper primitive descriptor (no external input boundary, but input validation at constructor applies if bounds-checked).

## Constraints
1. Constructor validates duty-cycle bounds (active/quiet > 0, sanity ceiling) via `Result<Self, FaultError>` — never panic on out-of-range input (per security-plan §Error Handling, verdict/error wall).
2. Failed validation returns typed `FaultError` enum variant, routed through the error wall as `Result::Err` harness fault (per security-plan §Error Handling, Internal logging).
3. `Cargo.lock` remains committed and un-drifted across the chunk (per security-plan §Dependency Security, Pinning).
4. No new dependencies introduced (per security-plan §Dependency Security, audit-tool premise).

## Patterns to follow
1. Match `EmissionGap::new` constructor signature (`Result<Self, FaultError>` with bounds validation) — exact duty-cycle bounds shape resolved at plan time.
2. Un-jittered owned values mirroring `EmissionGap` (determinism: same inputs ⇒ same `BurstyTrain`).
3. Typed error variants in `FaultError` enum matching the `#[non_exhaustive]` harness established in `silence.rs` / `gap.rs`.

## Anti-patterns to avoid
1. NEVER panic on invalid duty-cycle bounds — use `Result<Self, FaultError>` (per security-plan §Error Handling, verdict/error wall).
2. NEVER introduce jitter into the descriptor primitive (it owns un-jittered values only; jitter applied later in `conductor-timeline` Epoch 7).

## Contract bindings
Fault-helpers harness (error wall, `FaultError` enum); `conductor-timeline` (Epoch 7, duty-cycle consumption + seeded jitter application).

## Acceptance criteria contributions
1. (security) Constructor bounds-checks active/quiet durations and returns `Result<Self, FaultError>` on invalid input (per security-plan §Error Handling, verdict/error wall).
2. (security) No panics on malformed duty-cycle input; all edge cases route through `FaultError` typed value (per security-plan §Error Handling, verdict/error wall; Anti-Patterns § Universal).
3. (security) `cargo-audit` and `cargo deny check` pass (no new advisories, `Cargo.lock` un-drifted) (per security-plan §Dependency Security, CI integration).
4. (security) Crate gates green: `-p conductor-faults` nextest + clippy `-D warnings` + doctest + llvm-cov (per security-plan §Dependency Security, scope integrity).

## Relevant amendment history
- **2026-06-15-dependency-audit-gate** — audit-tool versions reframed as minimum floors (cargo-audit ≥0.22, cargo-deny ≥0.19); toolchain confirmed ≥1.94.1 — relevant to this chunk's `cargo-audit`/`cargo deny` gates.
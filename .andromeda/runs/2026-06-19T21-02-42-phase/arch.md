# arch extract

## Relevance
Partial — the chunk touches the `conductor-faults` crate (workspace boundary) and defines a fault helper under the Epoch 4 determinism discipline, but does not exercise the full OTLP emission or verification pipeline.

## Constraints
1. Code lives in `conductor-faults` per workspace module-boundary rules (arch §Inherited Defaults, crate-per-seam).
2. Typed `FaultError` enum — invalid gap/resume configuration surfaces as a typed value, never panic (arch §Established Decisions [Error Handling] + §Cross-cutting Patterns verdict/error wall).
3. Exact gap duration must be deterministic under seed — same scenario + seed ⇒ identical gap length, reproducible across platforms (arch §Design Philosophy "Determinism under seed"; §Established Decisions [Determinism RNG] ChaCha8Rng + seed_from_u64).
4. Gap duration must clear the ≥20s floor (P-015 restart-detection threshold per input.md:70/107; bounds co-located with the struct).
5. No direct tokio dependency in `conductor-faults` — the helper is self-contained; timeline/emit wiring is Epoch 7/8 (arch §Inherited Defaults "module crates don't depend on tokio").
6. Re-exported from `conductor-faults` public API alongside `PortOccupier` (arch §Occupied Resources, crate-name `conductor-faults` pinned).
7. Faults seam tests + workspace nextest + clippy `-D warnings` + llvm-cov threshold must all pass green (arch §Infrastructure Patterns Build system).

## Patterns to follow
1. Validation rule co-located with the struct via garde declarative bounds (scope.md acceptance intent: "typed FaultError, no panic"); follow the config-validation-surface precedent (arch amendments 2026-06-15, §Established Decisions [Validation Library] garde 0.22.1 field-level custom validators).
2. Per the verdict/error wall, invalid inputs must emit a `Result<T, FaultError>` with a typed variant, not an exception — matching the exception-events-fingerprint-control precedent (arch amendments 2026-06-18, §Cross-cutting Patterns) and the error-handling row in §Conventions.
3. Use `thiserror 2.0.18` for the error enum in `conductor-faults/src/error.rs` (already established; §Stack and Technologies, §Conventions errors).
4. Determinism contract: the gap duration is a function of the seed + scenario inputs only, no wall-clock reads — follow the seeded-phase-scheduler precedent (arch amendments 2026-06-16, per-gap jitter via rand_chacha ChaCha8Rng seed_from_u64).

## Anti-patterns to avoid
1. Do NOT introduce a runtime dependency (tokio, async); the gap/resume structure is a static descriptor (§Inherited Defaults "module crates don't depend on tokio").
2. Do NOT use panics for validation failures — all invalid config states route through typed `FaultError` (arch §Established Decisions, verdict/error wall).
3. Do NOT emit OTLP or call MCP — this chunk ships the structure only; emission and verification wiring are separate Epoch 7/8 chunks (scope.md "Boundaries: Not the live emit/timeline wiring").

## Contract bindings
obs ↔ tests harness — the `fault.silence` span event (scoped to `fault.{silence,ramp,port_occupier}` per obs-plan rule names) is deferred to the driven-under-timeline epoch (scope.md "Downstream consumer ... obs `fault.silence` span"), so this chunk's acceptance criterion is zero-deferral for that contract (no span instrumentation).

## Acceptance criteria contributions
1. (arch) Code lives in `conductor-faults` per workspace boundary rules (arch §Inherited Defaults).
2. (arch) Invalid gap duration (≤20s floor) or malformed ordering ⇒ typed `FaultError` value, no panic (arch §Established Decisions verdict/error wall).
3. (arch) Deterministic exact gap; same seed + inputs ⇒ identical shape reproducible across platforms (arch §Design Philosophy §Established Decisions [Determinism RNG]).
4. (arch) Re-exported from `conductor-faults::lib.rs` public API; gates green (faults seam tests, workspace nextest, clippy `-D warnings`, llvm-cov threshold).

## Relevant amendment history
— 2026-06-18-exception-events-fingerprint-control: fingerprint primitive placed in `conductor-emit` (not faults); `conductor-faults` narrowed to fault-helper composition only. Relevant: confirms faults is a pure helper/structure crate with no emission/fingerprint generation responsibility in this epoch.

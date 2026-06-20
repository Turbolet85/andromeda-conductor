# arch extract

## Relevance
Relevant — P-014 abrupt-silence fault helper belongs in `conductor-faults` seam per crate-per-seam architecture.

## Constraints
- Code lives in `conductor-faults` workspace crate per §Inherited Defaults (Crate names: LOCKED to `conductor-{seam}` naming and the six-seam structure)
- Determinism by construction (no seeded randomness, same scenario ⇒ same stop) per §Established Decisions (Determinism RNG pinned to ChaCha8; abrupt-silence is seed-independent like the gap)
- Any construction failure surfaces as typed `FaultError` value, never panic, per §Cross-cutting Patterns (Verdict/error wall) — or remains infallible, with contrast documented
- Faults module must maintain forbidden cross-seam dependency discipline per §Established Decisions (Module Boundaries: compiler-enforced crate seams)
- New types must be `pub use` re-exported in `conductor-faults` `lib.rs` per §Occupied Resources (Crate public API surface)
- No new `CONDUCTOR_*` or `ANDROMEDA_*` env vars beyond existing namespace per §Occupied Resources (Environment variables — CONDUCTOR_* reserved set is fixed; ANDROMEDA_PULSE_* are Pulse-side only)

## Patterns to follow
- Standalone fault helper primitive structure (no live emit/timeline wiring, matches `PortOccupier`/`EmissionGap` design) per §Cross-cutting Patterns (Config management / local files only)
- Deterministic/seed-independent implementation (like `EmissionGap`, no "different-seeds-diverge" test variant) per scope acceptance-intent (testing rule 2026-06-16/2026-06-18)
- Failure-path handling either typed `FaultError` (if fallible) or documented infallible constructor, no panics per scope boundaries (Verdict/error wall)
- Public re-export alongside sibling helpers (`PortOccupier`, `EmissionGap`) in lib.rs per §Occupied Resources (Public API surface)

## Anti-patterns to avoid
- Panics on invalid construction — use typed `FaultError` or infallible constructor instead
- Seeded randomness or seed parameters — abrupt-silence is terminal/unconditional (unlike gap's floor/ceiling), so no randomness source
- Emitting OTLP, reading MCP, or driving timeline logic — defer to Epochs 7/8 consumer phases

## Contract bindings
- `FaultError` enum (conductor-faults error seam) — extended if helper is fallible; unchanged if infallible
- `conductor-emit` downstream consumer (Epoch 7 activity-floor scenario family P-013/P-014) will compose this helper with timeline/emit wiring
- obs plan §3 (fault.silence span with bounded set `fault.{silence,ramp,port_occupier}`) — observation contract deferred to driven-under-timeline epoch

## Acceptance criteria contributions
- (arch) Code lives in `conductor-faults` per workspace boundary rules (§Inherited Defaults).
- (arch) Constructor surfaces as typed `FaultError` value on failure OR infallible with contrast documented (§Cross-cutting Patterns Verdict/error wall); no panics.
- (arch) No new env vars beyond `CONDUCTOR_*` / `ANDROMEDA_PULSE_*` reserved namespaces (§Occupied Resources).
- (arch) Re-exported from `conductor-faults` `lib.rs` alongside existing helpers; gates green (faults nextest, workspace nextest, clippy `-D warnings`, llvm-cov).

## Relevant amendment history
- **2026-06-18-exception-events-fingerprint-control:** conductor-emit now owns the fingerprint primitive (exception-trace builder + `fingerprint()` fn); conductor-faults is narrowed to fingerprint-STORM fault (Epoch-7). Relevance: confirms faults' module boundary as fault-helpers only; fingerprint composition is emit's domain, deferred to Epoch 7.
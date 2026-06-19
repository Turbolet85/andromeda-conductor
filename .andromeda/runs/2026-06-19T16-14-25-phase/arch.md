# arch extract

## Relevance
Relevant — port-occupier is a dedicated fault-helper primitive in `conductor-faults` seam, touching the sole deliberate trust-boundary exception (inbound `:4317` bind).

## Constraints
- Per §Occupied Resources: Conductor reserves `:4317` as Pulse's OTLP egress target; the port-occupier fault is the **sole documented exception** where Conductor opens an inbound listener (bind→hold→release on loopback).
- Per §Design Philosophy: Compiler-enforced module seams mean `conductor-faults` must NOT pull `conductor-emit` or `conductor-timeline` — the occupier is orthogonal infrastructure orthogonal to emission/scheduling.
- Per §Established Decisions [Async Runtime Flavor]: tokio `current_thread` runtime persists into the fault helper; occupier must not entangle multi-threaded runtimes.
- Per §Stack and Technologies: std/tokio `net` only (no new dependencies expected per scope).
- Per §Conventions (Naming patterns): fault helpers follow snake_case module naming (precedent: `conductor-emit` 's `rate.rs`/`latency.rs`/`pii.rs`).
- Per §Cross-cutting Patterns (Trust boundary): inbound bind is loopback-only (`127.0.0.1` exclusively, never `0.0.0.0`); release on cleanup is mandatory (RAII/`Drop`) to unblock subsequent scenarios' egress to the real Pulse.

## Patterns to follow
- Fault-helper module structure: type definition (e.g., `PortOccupier`) + module-level doc (precedent: `conductor-emit`'s per-module docstrings); standalone, no scenario-config wiring (deferred to Epoch 7).
- Verdict/error wall (fault flavor): typed error condition for "bind failed" (port already held, e.g., live Pulse up) — never panic on bind collision; surface as a typed return value.
- Crate-per-seam edges: occupier depends only on `conductor-core` (shared error/types); re-exported from `conductor-faults::lib.rs`.

## Anti-patterns to avoid
- Never widen the bind to `0.0.0.0` or a routable interface (trust-boundary violation).
- Never leak the socket on cleanup — mandatory RAII-enforced release (a leaked occupier silently breaks all subsequent scenarios' egress).
- Never introduce process management (start/stop/restart Pulse) — out of scope; scenario orchestration is Epoch 7.

## Contract bindings
obs (error/condition logging) ↔ self-observation stack (tracing §Occupied Resources); security-plan (trust-boundary exception flagged in Critical Warnings); scenarios-epoch (Epoch 7 orchestration will use this primitive to realize P-003 ReceiverFailed).

## Acceptance criteria contributions
- (arch) Code lives in `conductor-faults` per workspace-seam boundary (arch §Inherited Defaults, §Occupied Resources crate-names).
- (arch) PortOccupier binds `127.0.0.1:4317` exclusively (loopback-only, trust-boundary invariant from §Cross-cutting Patterns).
- (arch) Bind-failure typed as a condition, not panic (arch §Conventions error-handling, verdict/error wall).
- (arch) No new env vars or crate dependencies beyond std/tokio `net` (arch §Stack and Technologies, §Occupied Resources environment-variables).

## Relevant amendment history
§2026-06-18-exception-events-fingerprint-control — fingerprint primitive placed in `conductor-emit`, NOT `conductor-faults`. The amendment clarifies crate-seam ownership: faults is responsible for fingerprint-STORM fault composition (Epoch 7), not the fingerprint primitive itself. Confirms `conductor-faults` remains orthogonal to emit.

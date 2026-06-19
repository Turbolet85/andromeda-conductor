# Codebase Research — 2026-06-19-port-occupier-fault

## Scope
- **Depth:** moderate · **Reads:** 7 · **Globs/Greps:** 3 · **code-graph queries:** 1
- `conductor-faults` is a **cold-start crate** (scaffolded empty in Epoch 1); rich, directly-applicable
  precedent lives in `conductor-emit` (8 sibling primitives) + `conductor-core` (error pattern).

## Files inspected
- `crates/conductor-faults/Cargo.toml` (full) — deps = **`conductor-core` only**. No `tokio`/`thiserror`/`tracing`. This chunk adds the first real dep.
- `crates/conductor-faults/src/lib.rs` (full) — a single `//!` line (`Fault helpers — ramps, silence, port-occupier, fingerprint generation.`), **no modules**. The seam to populate.
- `crates/conductor-emit/src/lib.rs` (full) — the **module-seam precedent**: private `mod x;` (lib.rs:12-21) + `pub use x::{…}` (lib.rs:23-32) + a crate-level `//!` doc that links the public items + names the verdict/error wall (`EmitError` typed `Result::Err`).
- `crates/conductor-emit/src/rate.rs` (1-60) — the **module precedent**: thorough module `//!` doc citing the arch determinism §; an enum with **validating constructors returning `Option<Self>`** (rate.rs:49-60) that reject invalid params; `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`.
- `crates/conductor-core/src/error.rs` (full) — `CoreError` (thiserror, `#[non_exhaustive]`, `Config`/`Validation`) + `pub type Result<T>`. Confirms the **verdict/error wall** narrative; seam crates carry their **own** error enums (EmitError) rather than forcing everything through `CoreError`.
- `crates/conductor-emit/src/error.rs` (referenced via lib.rs) — `EmitError` is the **seam-local thiserror precedent** to mirror for a `FaultError` (`Transport` is a typed `Result::Err`, the wall).
- `crates/conductor-emit/tests/egress.rs` (full) — the **ephemeral-`:0` + typed-error test precedent**: `tokio::net::TcpListener::bind("127.0.0.1:0")` + `local_addr()` (egress.rs:34-35); the `refused_transport_surfaces_emit_error` test asserts `matches!(result, Err(EmitError::Transport(_)))` (egress.rs:73-77) — never a panic. Its module doc (egress.rs:1-3) already reserves `:4317` "for the Epoch-4 port-occupier fault" — **this chunk**.

## Graph impact (code-graph query → `tree-query-2026-06-19-port-occupier-fault.json`)
- **`conductor-faults` symbols:** exactly one — the crate node (`…conductor-faults 0.1.0 crate/`). No `PortOccupier`, no functions. **Cold-start for the crate.**
- **Dependents of `conductor-faults`:** none (zero `crate_edges` with `to_crate='conductor-faults'`). **Purely additive — zero blast radius**; no caller to update.
- **`conductor-faults` outbound usage edges:** none yet (its `conductor-core` Cargo dep isn't exercised in code).

## Patterns detected
- **Module-per-primitive** (`conductor-emit/src/lib.rs:12-32`): each primitive = private `mod` + `pub use` re-export; crate `//!` doc enumerates the public surface. Mirror for `port_occupier`.
- **Validating constructor → `Option`/`Result`** (`rate.rs:49-60`): reject invalid input at construction, never panic. The occupier's bind-failure is the `Result` analogue.
- **Seam-local thiserror error enum** (`conductor-emit/src/error.rs` = `EmitError`; `core/error.rs` = `CoreError`): typed `Result::Err` for harness faults — the verdict/error wall. A new `FaultError` follows this shape (likely `#[non_exhaustive]`, wrapping `std::io::Error` with the target addr for context).
- **Ephemeral-`:0` bind in tests** (`egress.rs:34-35`): bind `127.0.0.1:0`, read the OS-assigned port via `local_addr()` — the zero-flakiness way to exercise binding without touching `:4317` or a fixed CI port.
- **Typed-error assertion** (`egress.rs:73-77`): `matches!(…, Err(Variant(_)))` proves the condition is a value, not a panic.

## Conventions to follow
- **Plain `#[test]`** in `conductor-faults` (no `rstest` dev-dep — matches the `conductor-emit` convention; the occupier test is fully **synchronous**, so no `#[tokio::test]` either).
- **`std::time`/no-virtual-clock** is N/A here (no timing); **loopback-only** + **release-on-cleanup** are the live invariants (CLAUDE.md Critical Warnings; verification-harness rule: `cleanup` releases the bind and is idempotent).
- **Redaction:** a loopback `SocketAddr` (`127.0.0.1:4317`) is safe to include in an error message — the redaction ban targets host **file** paths, not socket addrs (`conductor-core::redact`).

## New files to create
- `crates/conductor-faults/src/port_occupier.rs` — the `PortOccupier` type (binds on construction, releases on `Drop`) + a `local_addr()` accessor for tests; module `//!` doc.
- `crates/conductor-faults/src/error.rs` — `FaultError` (thiserror, `#[non_exhaustive]`), mirroring `EmitError`; bind-failure variant carrying the addr + `std::io::Error`.
- `crates/conductor-faults/tests/port_occupier.rs` — synchronous integration test: bind `:0` → second bind to the resolved addr fails (exclusivity) → drop → rebind succeeds (clean release); plus loopback-only + typed-error assertions.

## Files to modify
- `crates/conductor-faults/src/lib.rs` — add `mod error; mod port_occupier;` + `pub use` re-exports; expand the crate `//!` doc (Epoch-4 fault seam; the sole-deliberate-bind invariant).
- `crates/conductor-faults/Cargo.toml` — add `thiserror.workspace = true` (for `FaultError`). **No** `tokio` (sync `std::net`), **no** `tracing` (deferred — see open questions). No dev-deps (sync `#[test]`).

## Open questions
1. **`std::net` vs `tokio::net` (recommend `std::net`).** The occupier only binds + holds + drops — it never `accept()`s — so a sync `std::net::TcpListener` denies the port (bind+listen reserves it) with **no tokio dep** and a sync test. `tokio::net` would force a runtime + async test for zero benefit. Resolve in P4 (recommended: `std::net`).
2. **Loopback-hardcode vs addr-param (recommend port-only param).** Reconciles security ("`127.0.0.1` hardcoded, never a user-supplied address") with test isolation ("ephemeral `:0`"): the constructor takes a **`u16` port** (default `4317`) and **always** binds `Ipv4Addr::LOCALHOST` — the IP is non-parameterizable (loopback by construction), the port is overridable so tests pass `0`. Resolve in P4.
3. **Obs instrumentation now vs deferred (recommend deferred).** The obs extract's `fault.port_occupier` span + `warn!`-on-bind-fail apply when the occupier runs **under a timeline/scenario** (Epoch 7); the standalone primitive surfaces the **typed `FaultError`** and need not log — keeping the dep set minimal (thiserror only), matching how `conductor-emit`'s pure primitives (`rate.rs`/`latency.rs`) return values without logging.

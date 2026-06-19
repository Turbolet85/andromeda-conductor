# Scope — Port-occupier fault (P-003)

**Marker:** `2026-06-19-port-occupier-fault`
**Working entry:** Port-occupier fault — sacrificial :4317 listener before Pulse starts (P-003 ReceiverFailed)
**Epoch:** 4 — Fault helpers (chunk 1/4 — **opens Epoch 4**)
**Crate:** `conductor-faults` (first real content — scaffolded empty since Epoch 1; first non-`conductor-emit` chunk since Epoch 2)
**P-IDs:** P-003 (ReceiverFailed)

## What it builds
A `conductor-faults` fault helper — a **sacrificial port occupier** that **binds `127.0.0.1:4317`**
(Pulse's OTLP/gRPC ingest port) and **holds it**, denying the bind to Pulse, then **releases it
deterministically on cleanup**. While the occupier holds the socket, a competing bind to the same
address fails — that exclusivity is the whole point: it is the lever that drives Pulse's **P-003
"ReceiverFailed"** connection-lifecycle state (Pulse's OTLP receiver cannot bind its listen port, so
it surfaces a receiver-failure rather than the healthy Listening/Receiving states P-001/P-002).

This is the **single most architecturally sensitive primitive in the build**: it is the **sole
deliberate inbound bind** in all of Conductor. Everywhere else Conductor is a pure client (gRPC egress
+ MCP-client stdio); the trust-boundary doc names the port-occupier as *the one* documented exception
where Conductor opens a port. So the chunk is as much about the **bind→hold→release discipline** (RAII
release, loopback-only, never widened) as about the bind itself.

It is the first of the four Epoch-4 fault helpers (port-occupier · gap/resume · abrupt-silence ·
bursty-train) and gives `conductor-faults` — scaffolded empty in Epoch 1 — its first real module.

## Boundaries (what it does NOT do)
- **Does NOT manage the Pulse process.** "No Pulse process management" is a stated non-goal. The occupier
  *holds the port*; it does not start, stop, restart, or sequence Pulse. How the held port interleaves
  with a Pulse (re)start to actually realize P-003 is a *scenario-orchestration* concern, wired in
  **Epoch 7** ("Connection-lifecycle scenarios … with orthogonal port-occupier, P-001..P-004") — not here.
- **Does NOT speak OTLP / gRPC.** It is a bare socket occupier, **not** a fake receiver — it denies the
  port, it does not impersonate Pulse's ingest or respond to exports. No tonic server, no protocol.
- **Loopback only.** Binds `127.0.0.1` exclusively — **never** `0.0.0.0` or a routable interface
  (trust-boundary invariant; widening is forbidden).
- **Released on cleanup — mandatory.** The bind is the sole deliberate inbound bind and MUST be released
  (RAII/`Drop` + an explicit release path) so every subsequent scenario's egress reaches the *real*
  Pulse on `:4317`. A leaked occupier would silently break the rest of the suite.
- **No verification, no P-003 verdict here.** This chunk *produces* the fault (occupies the port); the
  programmatic/operator verification of Pulse's ReceiverFailed reaction is a later epoch. Faults are
  values/conditions, not panics — a bind that fails because the port is *already* taken is a typed
  condition surfaced to the caller, never an `unwrap`.
- **No scenario-config / garde wiring.** Consistent with the Epoch-3 primitives, this is a standalone
  fault helper exercised in isolation; scenario-config wiring is deferred to the scenario epoch.
- **`:4318` (OTLP/HTTP) untouched** — gRPC-only loopback is the pinned path.
- **No new *external* dependency** — confirmed at plan time: the occupier uses sync `std::net` (no `tokio`);
  the typed bind-failure adds `thiserror` (already a workspace dependency — used by `conductor-emit`/`conductor-core`,
  so no new supply-chain/advisory surface). `tracing` instrumentation is deferred (Epoch 7).

## Surfaces / contracts it touches
- `conductor-faults`: new module (e.g. `port_occupier.rs`) — a `PortOccupier` type whose constructor
  binds the target socket and whose `Drop`/release frees it; re-exported from `lib.rs` with a module-doc
  (matches the `conductor-emit` module precedent: `rate.rs`/`latency.rs`/`pii.rs`).
- **Trust-boundary contract:** this is THE documented exception to "Conductor opens no inbound listener
  of its own." The scope-law caveat (sole deliberate `:4317` bind, released on cleanup, loopback-only)
  is the invariant the implementation must visibly honor (security-plan §trust-boundary / CLAUDE.md
  Critical Warnings).
- **Verdict/error wall (fault flavor):** a typed `conductor-faults` error/condition for "could not bind"
  (port already held — e.g. a live Pulse is up) rather than a panic.
- **Crate-per-seam law:** `conductor-faults` must NOT pull in `conductor-emit` or `conductor-timeline`
  for this primitive — the occupier is orthogonal infrastructure (it denies a socket; it neither emits
  nor schedules). Its only existing edge is `conductor-core` (shared error/types), to confirm in research.

## Open questions for P3/P4 (flag, don't pre-decide)
- **`std::net::TcpListener` vs `tokio::net::TcpListener`** for the bound socket — and whether `listen()`
  (TcpListener does both) is needed or a bare `bind()` suffices to deny the port.
- **Cross-platform exclusivity:** the dev host is Windows. Reliable *denial* of a second bind depends on
  `SO_REUSEADDR`/`SO_EXCLUSIVEADDRUSE` semantics (Windows default-exclusive vs Linux reuse). The occupier
  must NOT set reuse flags that would let Pulse co-bind. This is the determinism-critical detail.
- **Address parameterization + test isolation:** default `127.0.0.1:4317`, but unit tests likely bind an
  **ephemeral `:0`** port (then assert a second bind to *that* address fails, then assert release) to
  prove the mechanics with **zero flakiness** — never colliding with a real Pulse or a fixed CI port.
  Whether the constructor takes a `SocketAddr` (default `:4317`) is a plan decision.

## Acceptance intent (anchor for validation-1)
- A `PortOccupier` fault helper in `conductor-faults` that **binds `127.0.0.1:4317` (loopback, sacrificial)**
  on construction and **releases it on drop/cleanup**.
- **Exclusivity proven:** while the occupier holds the address, a second bind to the same address fails —
  the property that denies Pulse's receiver bind (P-003) — asserted by a unit test on an ephemeral port.
- **Clean, idempotent release:** after drop/release the address is re-bindable; no leaked socket.
- **Loopback-only:** never binds a routable interface (trust-boundary assertion).
- **Bind-failure is a typed condition,** not a panic, when the port is already taken (verdict/error wall).
- **Standalone:** no Pulse management, no OTLP speaking, no scenario-config wiring (all deferred).
- **All gates green:** `cargo nextest` (faults + workspace), clippy `-D warnings`, llvm-cov, doctest;
  no new dependency / no Cargo.toml drift unless justified.

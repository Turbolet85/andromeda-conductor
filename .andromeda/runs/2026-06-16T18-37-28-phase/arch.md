# arch extract

## Relevance
relevant — the chunk implements the Epoch 2 timeline engine's seeded phase scheduler, the foundational determinism substrate per architecture §Design Philosophy.

## Constraints
1. Per §Design Philosophy: seeded PRNG owns every non-deterministic choice; same scenario + seed ⇒ same stream shape; determinism is an architectural property of the runtime, not bolted on later.
2. Per §Established Decisions [Async Runtime Flavor]: `tokio::current_thread` (zero work-stealing) is compiler-enforced for deterministic ordering; `#[tokio::main(flavor = "current_thread")]` on CLI, hand-built `Builder::new_current_thread()` under Tauri.
3. Per §Established Decisions [Language / Runtime]: Rust 2024 (cargo 1.85, MSRV 1.94.1) + tokio 1.48.x; see amendments §2026-06-14-cargo-workspace-scaffold (MSRV 1.88.0 → 1.94.1).
4. Per §Conventions: crate-per-seam Cargo workspace with forbidden cross-seam deps; `conductor-timeline` library must be callable identically by headless CLI and Tauri (§Established Decisions [Module Boundaries]).
5. Per §Stack and Technologies: seedable RNG dependency is required (exact crate deferred); tokio 1.48.x `time` feature; no OTLP/journal/wall-clock in this chunk.
6. Per §Cross-cutting Patterns: determinism discipline enforces `tokio::time` (virtual clock) for ALL scheduling, never `std::time`; wall-clock journal stamps deferred to Epoch 2 chunk 3.

## Patterns to follow
1. Per §Design Philosophy [Headless-drivable core, thin shells]: the timeline scheduler is a runtime-agnostic library; CLI and Tauri both call it identically — favor generic `async fn` over `#[tokio::main]` wrappers in the library.
2. Per §Established Decisions [Module Boundaries]: `conductor-timeline` is a `conductor-core` consumer (allowed edge); any seedable RNG is a direct dep of `conductor-timeline`, not re-exported through conductor-core.
3. Unit tests using `tokio::time::pause()` for deterministic virtual-clock assertion (Definition of done item 3); no `start_paused` beyond tests.

## Anti-patterns to avoid
1. Ambient entropy or system-clock-derived scheduling (tokio's multi-threaded work-stealing, `std::time`, `SystemTime`, or non-seeded RNG) — violates determinism substrate.
2. Direct OTLP, journal writes, or wall-clock capture in the timeline seam — timing surfaces are minimal; emission/journal are later chunks.
3. Forbidden cross-seam dependency (e.g., direct tonic/opentelemetry-proto/rusqlite imports in `conductor-timeline`) — crate edges are compiler-enforced.

## Contract bindings
**timeline ↔ emit (Epoch 3):** The phase-boundary transitions the scheduler surfaces (e.g., "phase 1 start", "phase 1 end", silence duration) are the contract; the emit seam will subscribe to these events and drive OTLP emission. Format TBD in next Epoch-2 chunk (rich per-phase emission spec).

**timeline ↔ faults (Epoch 4):** Phase sequencing is the substrate faults compose on (ramps, silence, port-occupier apply timing over this schedule); the timeline must surface hook points or event-streaming so faults can register state transitions.

## Acceptance criteria contributions
1. (arch) Code lives in `conductor-timeline` per workspace boundary rules (arch §Inherited Defaults, manifest member list).
2. (arch) Seeded PRNG reproduces deterministic phase sequence/shape across runs (same scenario + seed ⇒ identical advance pattern + relative timing).
3. (arch) Runtime is `current_thread` tokio with zero work-stealing; all scheduling uses `tokio::time`, never `std::time` (verified by unit tests under `tokio::time::pause()`).
4. (arch) No OTLP, journal writes, or scenario-spec declarative config in this chunk; phase-boundary events are surfaced to caller for downstream consumption.

## Relevant amendment history
**2026-06-14-cargo-workspace-scaffold** (§Stack and Technologies · §Infrastructure Patterns Build system): MSRV bumped 1.88.0 → 1.94.1 (tar-rs CVE-2026-33056); this chunk's Cargo.toml must enforce `rust-version = "1.94.1"` via workspace-level declaration per that precedent.

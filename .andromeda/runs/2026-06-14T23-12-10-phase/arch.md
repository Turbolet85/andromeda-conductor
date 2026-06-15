# arch extract

## Relevance
relevant — this chunk is the foundational workspace scaffold that establishes all crate-per-seam module boundaries and the Cargo dependency structure that enforces the architecture.

## Constraints
1. Crate-per-seam Cargo workspace with compiler-enforced module seams per arch §Module Boundaries (six library crates + two binary crates under `crates/` with explicit `Cargo.toml` dependency edges).
2. Rust 2024 edition workspace-wide, MSRV ≥1.94.1 per arch §Stack and Technologies + security-plan bump (tar-rs CVE-2026-33056).
3. `[workspace.dependencies]` centralizes pinned versions per arch §Stack and Technologies (tokio 1.48.x, tonic 0.14.6, opentelemetry-proto 0.32.0, rmcp 1.7.0, rusqlite 0.38.0, garde 0.23.0, thiserror 2.0.18, anyhow 1.0.102, serde 1.0.x, Tauri 2.10.x).
4. Eight crate names `conductor-{core,timeline,emit,faults,verify,report,cli,tauri}` per arch §Occupied Resources (crate names locked).
5. Seam→core dependency edges (five seam libs + two bins depend on `conductor-core`); forbidden cross-seam deps must fail to compile per arch §Design Philosophy (compiler-enforced seams).
6. `Cargo.lock` committed for deterministic builds per arch §Infrastructure Patterns.
7. CLI binary built with `#[tokio::main(flavor = "current_thread")]` per arch §Established Decisions [Async Runtime Flavor] (relevant when cli main is fleshed out; scaffold sets up the bin crate).

## Patterns to follow
1. Crate-per-seam dependency graph + reverse-dependency audit (`cargo-modules` / `cargo-rail`) to verify forbidden cross-seam edges are blocked at compile time per arch §Infrastructure Patterns.
2. `[workspace.dependencies]` as the single source of truth for pinned versions; downstream crates reference `.workspace = true` per arch §Infrastructure Patterns (Build system).
3. Rust 2024 + MSRV pin in `rust-toolchain.toml` per arch §Stack and Technologies.

## Anti-patterns to avoid
1. Do NOT introduce cross-seam dependencies (only `conductor-core` is a common dependency) per arch §Design Philosophy.
2. Do NOT omit `Cargo.lock` or leave it un-drifted per arch §Infrastructure Patterns.
3. Do NOT pin the async runtime in library/seam crates; tokio is core-owned (cli/core only) per arch §Established Decisions [Async Runtime Flavor].

## Contract bindings
- **arch ↔ security**: the `conductor-*` workspace topology + crate names are materialized here; security owns the MSRV ≥1.94.1 + `Cargo.lock`-committed gate over it.

## Acceptance criteria contributions
- (arch) Eight crates with correct names + crate types (six lib, two bin) per §Occupied Resources.
- (arch) `rust-toolchain.toml` pins ≥1.94.1; root `Cargo.toml` sets `edition = "2024"` workspace-wide per §Stack and Technologies.
- (arch) `[workspace.dependencies]` centralizes the pinned versions per §Stack and Technologies.
- (arch) Seam→core edges present; a deliberately-forbidden cross-seam edge fails to compile per §Module Boundaries.
- (arch) `cargo build --workspace` succeeds; `Cargo.lock` committed.

## Relevant amendment history
(none) — fresh project, no prior amendments yet.
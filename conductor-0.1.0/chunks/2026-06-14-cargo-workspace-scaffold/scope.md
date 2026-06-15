# Scope — Cargo workspace scaffold

**Marker:** 2026-06-14-cargo-workspace-scaffold
**Epoch:** 1 — Foundation
**Working entry:** "Cargo workspace scaffold — 8 crate-per-seam members, workspace manifest, rust-toolchain pin ≥1.94.1"

## What this chunk builds
The compiling skeleton of the crate-per-seam Cargo workspace — the structural foundation every later chunk builds on. No feature logic.

- **Root workspace manifest** (`Cargo.toml`) declaring all 8 members + `[workspace.package]` shared `edition = "2024"`, `rust-version`, version, and a `[workspace.dependencies]` table centralizing the pinned versions the architecture fixes (so later chunks reference `{dep}.workspace = true`).
- **`rust-toolchain.toml`** pinning the toolchain to ≥ 1.94.1 (security-plan required bump over arch's MSRV 1.88.0; clears tar-rs CVE-2026-33056), edition-2024-capable channel.
- **8 member crates** under `crates/`, each with its own `Cargo.toml` + minimal placeholder source so the workspace compiles:
  - libraries: `conductor-core`, `conductor-timeline`, `conductor-emit`, `conductor-faults`, `conductor-verify`, `conductor-report`
  - binaries: `conductor-cli` (`agent-run`), `conductor-tauri`
- **Crate-per-seam dependency edges** wired in each `Cargo.toml`: the five seam libs + the two bins depend on `conductor-core`; the bins compose the seams. The `Cargo.toml` edges ARE the architecture — a forbidden cross-seam dependency must fail to compile.
- **`Cargo.lock`** generated + committed (binary workspace → reproducible builds + deterministic `cargo-audit`).

## Boundaries (NOT in this chunk)
- NO shared-type bodies — `conductor-core`'s `Verdict`/`ReportState`/scenario model are the NEXT chunk ("conductor-core shared types"); here `conductor-core` is a minimal compiling lib.
- NO per-seam feature deps beyond what compiles — tokio/tonic/rmcp/rusqlite/Tauri etc. are wired into their crates in their feature chunks; the scaffold may PIN their versions centrally in `[workspace.dependencies]` but need not reference them from every crate yet.
- NO config-validation, logging stack, design tokens, CI, or agent-harness wiring (each is its own Foundation chunk).
- NO application logic, behavior tests, or scenario code.

## Surfaces / contracts touched
- arch §Infrastructure Patterns (Directory structure: `crates/` + the 8 members) + §Occupied Resources (crate names) + §Module Boundaries (crate-per-seam, compiler-enforced seams).
- security-plan §Dependency Security (toolchain ≥1.94.1; `Cargo.lock` committed + un-drifted).
- Establishes the `conductor-*` workspace that every subsequent chunk extends.

## Acceptance hints (refined into criteria in plan.md)
- `cargo build --workspace` succeeds (green skeleton).
- All 8 crates exist with correct crate-type (6 lib, 2 bin) and `conductor-*` names.
- `rust-toolchain.toml` pins ≥1.94.1; root manifest sets `edition = "2024"` workspace-wide.
- Seam→core dependency edges present; a deliberately-forbidden cross-seam edge does not compile.
- `Cargo.lock` committed.

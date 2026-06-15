# Report — 2026-06-14-cargo-workspace-scaffold

**Chunk:** Cargo workspace scaffold — 8 crate-per-seam members + toolchain pin ≥1.94.1
**Date:** 2026-06-14 (UTC)
**Commits:** none yet — this chunk's changes are uncommitted; the chunk commit is created in P7 (the prior commit, `setup-project`, predates this chunk).

## Changes (structured — detectors read this)
- **Files:** `Cargo.toml` (root), `rust-toolchain.toml`, `Cargo.lock`, and per crate `crates/conductor-{core,timeline,emit,faults,verify,report}/{Cargo.toml,src/lib.rs}` + `crates/conductor-{cli,tauri}/{Cargo.toml,src/main.rs}`.
- **Symbols / APIs:** none yet — `conductor-cli` produces the binary **`conductor`** (`[[bin]] name = "conductor"`); all crate sources are placeholders (libs = one-line `//!` crate-doc, no public items; bins = `fn main() {}`). No IPC methods, endpoints, ports, sockets, or env vars implemented this chunk (the `:4317` egress / MCP tools / `CONDUCTOR_*` handles arrive in their feature chunks).
- **Crates / modules:** **added** — `conductor-core`, `conductor-timeline`, `conductor-emit`, `conductor-faults`, `conductor-verify`, `conductor-report` (libs) + `conductor-cli`, `conductor-tauri` (bins). removed — none. changed — none. Edges: every seam + both bins depend on `conductor-core` only (no seam→seam).
- **Dependencies:** `[workspace.dependencies]` declares the architecture-pinned versions — `tokio 1.48`, `opentelemetry-proto 0.32.0`, `tonic 0.14.6`, `tonic-prost 0.14.6`, `prost 0.14`, `rmcp 1.7.0`, `rusqlite 0.38.0 (bundled)`, `serde 1.0`, `garde 0.23.0`, `thiserror 2.0.18`, `anyhow 1.0.102`, `tracing 0.1.44`, `tracing-subscriber 0.3.23`, `tauri 2.10.3`. These are **inert** (no member references them yet) → NOT in `Cargo.lock`; only the `conductor-core` path edges are resolved. No bumps (greenfield).
- **Schema / config:** `rust-toolchain.toml` `channel = "1.95.0"` + `components = [clippy, rustfmt]`; root manifest `resolver = "3"`, `[workspace.package]` `edition = "2024"`, `rust-version = "1.94.1"`, `version = "0.1.0"`. No runtime config keys, migrations, or violation schemas.
- **Coverage of new surfaces:** **none** — pure build-time scaffold. No external-input surface, hot-path operation, UI element, telemetry-emitting op, or logged data this chunk. (validation n/a · instrumentation n/a · PII n/a · tests n/a-this-chunk · a11y n/a · tokens n/a)

## Deviations from intent
- **`rust-toolchain.toml` channel `1.95.0`, not `1.94.1`** — justified: the plan explicitly allowed "a dated stable ≥1.94.1"; `1.95.0` is the installed stable (includes the tar-rs CVE-2026-33056 fix), avoiding a toolchain download. The CVE floor is encoded as `rust-version = "1.94.1"` (MSRV) in `[workspace.package]`, so the security requirement holds.
- **Bins depend on `conductor-core` only** (not on the seam crates) — justified: matches the plan's implementation steps 5–6 ("depending on conductor-core"); seam→bin "compose" edges are added per-seam as each seam gains functionality in its feature chunk (declaring unused seam edges now would be dead weight).
- **`[workspace.dependencies]` declared but inert** — justified: the plan's single-source-of-truth pin strategy; declaring unused workspace deps is valid and intended; they activate when feature chunks reference `{dep}.workspace = true`.
- **Minor:** each `lib.rs` carries a one-line `//!` crate-doc (conventional Rust, durable role from arch — not WHAT-restating); `license` omitted from `[workspace.package]` (crates are unpublished/local — avoids imposing a choice; addable later).

## Decisions & corrections
- **Branch policy (user correction, session-level):** the entire version build runs on a long-lived `build/conductor-<version>` branch (`build/conductor-0.1.0`); `main` is NOT fast-forwarded until the version is fully built and tagged ("task done"). For Andromeda the branching unit is the version, not the task — specializes the global "feature branch per task" rule. (Already saved to memory.)
- **Toolchain pin decision:** pin the build toolchain to the installed `1.95.0` while encoding the security CVE floor as the `1.94.1` MSRV — gives reproducibility + the floor without a download.
- **`conductor` bin name:** the `conductor-cli` crate's binary is named `conductor` (matching the design/cli command surface `conductor run|suite|report` + `agent-run.sh`'s `cargo run -p conductor-cli --bin conductor`).

## Outcome
- **Acceptance criteria:** all 8 met (8 crates correct names+types · toolchain ≥1.94.1 + edition 2024 · `tauri` ≥2.10.3 pinned · centralized `[workspace.dependencies]` · seam→core-only edges · no OTel SDK · `cargo build --workspace` = 0 · `Cargo.lock` generated).
- **Gates green:** `cargo build --workspace` (0.42s) · `cargo test --workspace` (0 tests, all compile) · `cargo tree --workspace | grep -i opentelemetry_sdk` (empty).
- **Smoke:** ✓ both bins boot — `cargo run --bin conductor` and `cargo run -p conductor-tauri` exit 0 (`conductor.exe` + `conductor-tauri.exe` built). Full `agent-run.sh` functional smoke deferred (its `boot=preflight` / `run=nextest` depend on later chunks).
